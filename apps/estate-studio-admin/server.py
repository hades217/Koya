"""Estate Studio CRM. Python standard library + transactional SQLite; no remote calls on startup."""
import argparse
import base64
import csv
import datetime as dt
import getpass
import hashlib
import hmac
import http.cookies
import io
import json
import os
from pathlib import Path
import re
import secrets
import smtplib
import sqlite3
import struct
import threading
import time
from email.message import EmailMessage
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
from urllib.parse import urlparse, parse_qs
from zoneinfo import ZoneInfo
from gateway import read_gateway
from funnel import analyse
from commissions import CHANNELS, ROLES, percent, calculate
from billing import account_billing, BillingError

ROOT = Path(__file__).resolve().parent
KINDS = ('staff', 'commission_rules', 'contacts', 'companies', 'inquiries', 'tasks', 'opportunities', 'communications', 'quotes', 'payments')
STAGES = ['需求确认', '演示沟通', '报价中', '商务确认', '已成交', '已流失']
INQUIRY_STATES = ['新咨询', '跟进中', '已安排演示', '已完成', '已关闭']

def now():
    return dt.datetime.now(dt.timezone.utc).isoformat(timespec='seconds')

def uid():
    return secrets.token_hex(12)

def digest(value):
    return hashlib.sha256(value.encode()).hexdigest()

def json_dump(value):
    return json.dumps(value, ensure_ascii=False, separators=(',', ':'))

class Problem(Exception):
    def __init__(self, status, message):
        self.status, self.message = status, message

def require(value, message, status=400):
    if not value:
        raise Problem(status, message)

def clean(value, limit=200, required=False):
    require(value is None or isinstance(value, str), '字段必须为文本')
    value = (value or '').strip()
    require(len(value) <= limit, '字段过长（最多 %s 字符）' % limit)
    require(not required or value, '请填写必填字段')
    return value

def email(value, required=True):
    value = clean(value, 254, required).lower()
    require(not value or re.fullmatch(r'[^\s@]+@[^\s@]+\.[^\s@]+', value), '请输入有效的邮箱地址')
    return value

def date_value(value, required=False):
    value = clean(value, 64, required)
    if value:
        try:
            parsed = dt.datetime.fromisoformat(value.replace('Z', '+00:00'))
            require(parsed.tzinfo is not None, '日期必须包含时区')
            return parsed.astimezone(dt.timezone.utc).isoformat(timespec='seconds')
        except ValueError:
            raise Problem(400, '日期格式无效')
    return ''

def amount(value):
    if value in ('', None):
        return None
    require(isinstance(value, (str, int, float)) and not isinstance(value, bool), '金额格式无效')
    from decimal import Decimal, InvalidOperation
    try:
        number = Decimal(str(value))
        require(number.is_finite() and 0 <= number <= 1000000000 and number.as_tuple().exponent >= -2, '金额必须为非负数，最多两位小数')
        return int(number * 100)
    except InvalidOperation:
        raise Problem(400, '金额格式无效')

def password_hash(password, salt):
    return hashlib.pbkdf2_hmac('sha256', password.encode(), bytes.fromhex(salt), 600000).hex()

def totp(secret, tick=None):
    tick = int(time.time() // 30) if tick is None else tick
    raw = hmac.new(base64.b32decode(secret), struct.pack('>Q', tick), hashlib.sha1).digest()
    offset = raw[-1] & 15
    return str((struct.unpack('>I', raw[offset:offset+4])[0] & 0x7fffffff) % 1000000).zfill(6)

class CRM:
    def __init__(self, path, preview=False, origin='http://127.0.0.1:18767'):
        self.path, self.preview, self.origin = Path(path), preview, origin.rstrip('/')
        self.path.parent.mkdir(parents=True, exist_ok=True, mode=0o700)
        self.db = sqlite3.connect(str(self.path), check_same_thread=False)
        os.chmod(self.path, 0o600)
        self.db.row_factory = sqlite3.Row
        self.db.executescript('''
        PRAGMA journal_mode=WAL;
        PRAGMA foreign_keys=ON;
        CREATE TABLE IF NOT EXISTS entities(id TEXT PRIMARY KEY,kind TEXT NOT NULL,data TEXT NOT NULL,version INTEGER NOT NULL DEFAULT 1,created_at TEXT NOT NULL,updated_at TEXT NOT NULL);
        CREATE INDEX IF NOT EXISTS entity_kind_date ON entities(kind,created_at DESC,id);
        CREATE INDEX IF NOT EXISTS entity_email ON entities(kind,json_extract(data,'$.email'));
        CREATE INDEX IF NOT EXISTS entity_contact ON entities(kind,json_extract(data,'$.contact_id'));
        CREATE INDEX IF NOT EXISTS entity_status ON entities(kind,json_extract(data,'$.status'));
        CREATE TABLE IF NOT EXISTS sessions(token TEXT PRIMARY KEY,csrf TEXT NOT NULL,user_id TEXT NOT NULL,role TEXT NOT NULL,created REAL NOT NULL,touched REAL NOT NULL);
        CREATE TABLE IF NOT EXISTS users(id TEXT PRIMARY KEY,email TEXT UNIQUE NOT NULL,status TEXT NOT NULL,registered_at TEXT NOT NULL,last_login_at TEXT NOT NULL);
        CREATE TABLE IF NOT EXISTS magic(token TEXT PRIMARY KEY,email TEXT NOT NULL,expires REAL NOT NULL,used INTEGER NOT NULL DEFAULT 0);
        CREATE TABLE IF NOT EXISTS audit(id TEXT PRIMARY KEY,actor TEXT NOT NULL,action TEXT NOT NULL,target TEXT NOT NULL,details TEXT NOT NULL,created_at TEXT NOT NULL);
        CREATE TABLE IF NOT EXISTS replay(key TEXT PRIMARY KEY,payload TEXT NOT NULL,result TEXT NOT NULL);
        CREATE TABLE IF NOT EXISTS settings(key TEXT PRIMARY KEY,value TEXT NOT NULL);
        CREATE TABLE IF NOT EXISTS limits(key TEXT PRIMARY KEY,count INTEGER NOT NULL,expires REAL NOT NULL);
        ''')
        self.db.commit()
        self.lock = threading.RLock()
        mode = self.setting('database_mode')
        expected_mode = 'preview' if preview else 'production'
        require(not mode or mode == expected_mode, '本地预览和正式模式必须使用不同数据库', 503)
        self.db.execute('INSERT OR IGNORE INTO settings VALUES(?,?)', ('database_mode', json_dump(expected_mode)))
        self.db.commit()
        self.admin = None
        config = self.path.parent / 'admin.json'
        if config.exists():
            self.admin = json.loads(config.read_text())
        if not preview:
            require(self.admin, '请先运行 npm run setup 配置管理员', 503)
            require(self.origin.startswith('https://'), '正式模式必须设置 HTTPS CRM_ORIGIN', 503)

    def audit(self, actor, action, target='', details=None):
        self.db.execute('INSERT INTO audit VALUES(?,?,?,?,?,?)', (uid(), actor, action, target, json_dump(details or {}), now()))

    def insert(self, kind, data):
        identifier, stamp = uid(), now()
        self.db.execute('INSERT INTO entities VALUES(?,?,?,1,?,?)', (identifier, kind, json_dump(data), stamp, stamp))
        return self.get(kind, identifier)

    def unpack(self, row):
        return dict(json.loads(row['data']), id=row['id'], version=row['version'], created_at=row['created_at'], updated_at=row['updated_at'])

    def get(self, kind, identifier):
        row = self.db.execute('SELECT * FROM entities WHERE kind=? AND id=?', (kind, identifier)).fetchone()
        require(row, '记录不存在', 404)
        return self.unpack(row)

    def replace(self, kind, identifier, data, version):
        data = {k: v for k, v in data.items() if k not in ('id', 'version', 'created_at', 'updated_at')}
        result = self.db.execute('UPDATE entities SET data=?,version=version+1,updated_at=? WHERE id=? AND kind=? AND version=?', (json_dump(data), now(), identifier, kind, version))
        require(result.rowcount == 1, '记录已被更新，请刷新核对后再保存。输入已保留。', 409)
        return self.get(kind, identifier)

    def rows(self, kind, key=None, value=None):
        sql, params = 'SELECT * FROM entities WHERE kind=?', [kind]
        if key:
            require(key in ('contact_id', 'opportunity_id', 'email', 'user_id', 'company_id', 'inquiry_id'), '查询字段无效')
            sql += " AND json_extract(data,'$.%s')=?" % key
            params.append(value)
        return [self.unpack(r) for r in self.db.execute(sql + ' ORDER BY created_at DESC,id DESC', params)]

    def setting(self, key, default=''):
        row = self.db.execute('SELECT value FROM settings WHERE key=?', (key,)).fetchone()
        return json.loads(row[0]) if row else default

    def limit(self, key, maximum=10, window=600):
        row = self.db.execute('SELECT * FROM limits WHERE key=?', (key,)).fetchone()
        if row and row['expires'] > time.time():
            require(row['count'] < maximum, '请求过于频繁，请稍后再试', 429)
            self.db.execute('UPDATE limits SET count=count+1 WHERE key=?', (key,))
        else:
            self.db.execute('INSERT OR REPLACE INTO limits VALUES(?,1,?)', (key, time.time()+window))

    def session(self, token):
        row = self.db.execute('SELECT * FROM sessions WHERE token=?', (digest(token),)).fetchone()
        require(row and time.time()-row['touched'] < 1800 and time.time()-row['created'] < 43200, '请登录后继续', 401)
        if row['role'] == 'admin' and self.admin:
            require(row['user_id'] == self.admin['email'], '请使用管理员账号重新登录', 401)
        if row['role'] == 'user':
            user = self.db.execute('SELECT * FROM users WHERE id=?', (row['user_id'],)).fetchone()
            require(user and user['status'] == 'active', '账户已禁用', 401)
        self.db.execute('UPDATE sessions SET touched=? WHERE token=?', (time.time(), digest(token)))
        return dict(row)

    def login(self, user_id, role):
        token, csrf = secrets.token_urlsafe(32), secrets.token_urlsafe(24)
        self.db.execute('INSERT INTO sessions VALUES(?,?,?,?,?,?)', (digest(token), csrf, user_id, role, time.time(), time.time()))
        self.audit(user_id, '登录', role)
        return {'csrf': csrf, 'role': role, 'preview': self.preview}, token

    def contact(self, data, verified_user=None):
        address = email(data.get('email'), False)
        matches = [c for c in self.rows('contacts', 'email', address) if not c.get('merged_into')] if address else []
        if len(matches) == 1 and (not verified_user or matches[0].get('user_id') in ('', None, verified_user)):
            contact = matches[0]
            if verified_user:
                contact.update(user_id=verified_user, verification_state='verified')
                contact = self.replace('contacts', contact['id'], contact, contact['version'])
            return contact
        return self.insert('contacts', dict(name=clean(data.get('name'), 100), email=address, phone=clean(data.get('phone'), 50), company_id='', company_name=clean(data.get('company'), 150), source=data.get('source', '官网咨询'), tags='', priority='中', notes='', user_id=verified_user or '', verification_state='verified' if verified_user else 'unverified', needs_review=bool(matches)))

    def validate(self, kind, data, old=None):
        old = old or {}
        fields = {
            'staff': ['name', 'role', 'status', 'notes'],
            'commission_rules': ['name', 'channel', 'mode', 'status', 'notes'],
            'contacts': ['name', 'email', 'phone', 'wechat', 'job_title', 'company_id', 'company_name', 'source', 'tags', 'priority', 'notes'],
            'companies': ['name', 'website', 'industry', 'notes'],
            'inquiries': ['name', 'email', 'phone', 'company', 'message', 'source', 'status', 'received_at', 'reason', 'demo_at', 'is_spam'],
            'tasks': ['contact_id', 'inquiry_id', 'opportunity_id', 'title', 'type', 'due_at', 'status', 'result', 'reason'],
            'opportunities': ['contact_id', 'company_id', 'name', 'business_type', 'summary', 'stage', 'currency', 'expected_close_at', 'closed_at', 'acceptance_reference', 'reason', 'next_action', 'inquiry_id', 'pipeline', 'reason_category', 'blocker', 'lead_staff_id', 'sales_staff_id', 'lead_channel', 'commission_rule_id'],
            'communications': ['contact_id', 'inquiry_id', 'opportunity_id', 'channel', 'occurred_at', 'summary', 'result'],
            'quotes': ['opportunity_id', 'currency', 'issued_at', 'valid_until', 'send_status', 'reference_url', 'notes'],
            'payments': ['opportunity_id', 'currency', 'paid_at', 'evidence_reference', 'notes']
        }[kind]
        output = dict(old)
        for field in fields:
            if field in data:
                if field == 'is_spam':
                    require(isinstance(data[field], bool), '垃圾标记必须为布尔值')
                output[field] = bool(data[field]) if field == 'is_spam' else clean(data[field], 4000 if field in ('notes', 'message', 'summary', 'result', 'reason', 'acceptance_reference') else 300)
        for field in ('email',):
            if field in output:
                output[field] = email(output[field], False)
        for field in ('due_at', 'received_at', 'demo_at', 'occurred_at', 'expected_close_at', 'closed_at', 'issued_at', 'valid_until', 'paid_at'):
            if field in output:
                output[field] = date_value(output[field])
        for field in ('website', 'reference_url'):
            if output.get(field):
                require(urlparse(output[field]).scheme == 'https' and urlparse(output[field]).netloc, '链接必须使用 HTTPS')
        for field, related in [('contact_id', 'contacts'), ('company_id', 'companies'), ('inquiry_id', 'inquiries'), ('opportunity_id', 'opportunities')]:
            if output.get(field):
                target = self.get(related, output[field])
                require(not target.get('merged_into'), '该客户已经合并，请使用保留的档案')
                if field in ('inquiry_id', 'opportunity_id') and output.get('contact_id'):
                    require(target.get('contact_id') == output['contact_id'], '关联记录必须属于同一客户')
        if kind == 'contacts':
            require(output.get('name') and (output.get('email') or output.get('phone') or output.get('wechat')), '请填写姓名及至少一种联系方式')
            output.setdefault('priority', '中')
            require(output['priority'] in ['高', '中', '低'], '优先级无效')
            if output.get('company_id'):
                output['company_name'] = self.get('companies', output['company_id'])['name']
            if output.get('email'):
                matches = [c for c in self.rows('contacts', 'email', output['email']) if c['id'] != old.get('id') and not c.get('merged_into')]
                require(not matches, '此邮箱已有客户，请打开已有档案；共享邮箱需先人工核对', 409)
        if kind == 'staff':
            require(output.get('name'), '请填写员工或转介绍伙伴姓名')
            require(output.get('role') in ROLES, '请选择员工职责')
            output.setdefault('status', 'active')
            require(output['status'] in ('active', 'inactive'), '员工状态无效')
        if kind == 'commission_rules':
            require(output.get('name') and output.get('channel') in CHANNELS, '请填写规则名称与获客渠道')
            require(output.get('mode') in ('direct', 'pool'), '请选择佣金计算方式')
            output.setdefault('status', 'active')
            require(output['status'] in ('active', 'inactive'), '规则状态无效')
            keys = ('lead_rate', 'sales_rate') if output['mode'] == 'direct' else ('pool_rate', 'lead_share')
            for key in keys:
                try: output[key+'_bps'] = percent(data[key]) if key in data else old[key+'_bps']
                except (ValueError, KeyError) as error: raise Problem(400, str(error) if isinstance(error, ValueError) else '请填写适用的佣金比例')
            if output['mode'] == 'direct':
                require(output['lead_rate_bps']+output['sales_rate_bps'] <= 10000, '获客与成交佣金合计不能超过计佣金额的 100%')
        if kind == 'companies':
            require(output.get('name'), '请填写公司名称')
        if kind in ('tasks', 'opportunities', 'communications'):
            require(output.get('contact_id'), '请选择客户')
        if kind == 'inquiries':
            output.setdefault('status', '新咨询')
            output.setdefault('received_at', now())
            require(output.get('name') and (output.get('email') or output.get('phone')), '请填写姓名和联系方式')
            require(output['status'] in INQUIRY_STATES, '咨询状态无效')
            if output['status'] in ('已完成', '已关闭') or (old and output['status'] != old.get('status') and INQUIRY_STATES.index(output['status']) < INQUIRY_STATES.index(old['status'])):
                require(clean(data.get('reason'), 4000), '请填写处理结果或状态变更原因')
            if output['status'] == '已安排演示':
                require(output.get('demo_at'), '请填写已确认的演示时间')
            if old:
                for field in ('name', 'email', 'phone', 'company', 'message', 'source', 'received_at'):
                    require(output.get(field) == old.get(field), '原始咨询不能覆盖，请添加沟通记录')
        if kind == 'tasks':
            output.setdefault('status', '待完成')
            require(output.get('title') and output.get('due_at'), '请填写任务标题和截止时间')
            require(output['status'] in ('待完成', '已完成', '已取消'), '任务状态无效')
            if output['status'] == '已完成':
                require(output.get('result'), '请填写本次任务的完成结果')
            if output['status'] == '已取消' and old.get('status') != '已取消':
                require(clean(data.get('reason'), 4000), '请填写本次取消原因')
            if old.get('status') in ('已完成', '已取消') and output['status'] == '待完成':
                require(clean(data.get('reason'), 4000), '重新打开任务需要填写原因')
                output['completed_at'] = ''
            if old and output.get('due_at') != old.get('due_at'):
                require(clean(data.get('reason'), 4000), '调整截止时间需要填写原因')
                output['deadline_history'] = old.get('deadline_history', []) + [{'due_at': old['due_at'], 'changed_at': now(), 'reason': data['reason']}]
            if output['status'] == '已完成':
                output['completed_at'] = old.get('completed_at') or now()
        if kind == 'opportunities':
            if old and output.get('contact_id') != old.get('contact_id'):
                require(not any(self.rows(k, 'opportunity_id', old['id']) for k in ('quotes', 'payments', 'tasks', 'communications')), '已有相关记录时不能直接更换客户，请使用客户合并流程')
            output.setdefault('stage', '需求确认')
            require(output.get('name') and output.get('summary'), '请填写机会名称和需求摘要')
            require(output['stage'] in STAGES, '销售阶段无效')
            for field in ('expected_amount', 'agreed_amount'):
                if field in data:
                    output[field + '_minor'] = amount(data[field])
            if old and output['stage'] != old['stage'] and (STAGES.index(output['stage']) < STAGES.index(old['stage']) or old['stage'] in STAGES[-2:]):
                require(clean(data.get('reason'), 4000), '回退或重新打开机会需要填写原因')
            if output['stage'] == '报价中':
                require(old and self.rows('quotes', 'opportunity_id', old['id']), '请先保存机会并添加报价记录')
            if output['stage'] in ('已成交', '已流失'):
                require(output.get('closed_at'), '请填写成交或关闭日期')
            if output['stage'] == '已成交':
                require(output.get('agreed_amount_minor') is not None and output.get('acceptance_reference'), '成交需要约定金额和接受依据')
            if output['stage'] == '已流失':
                require(output.get('reason'), '请填写流失原因')
            output['pipeline'] = output.get('pipeline') or '默认销售'
            require(not old.get('pipeline') or output['pipeline'] == old['pipeline'], '已有销售历史的 Pipeline 不可更改，请新建独立机会')
            changed = not old or old['stage'] != output['stage']
            output['stage_history'] = list(old.get('stage_history', []))
            if changed:
                require(not old or clean(data.get('reason'), 4000), '请记录本次阶段变化的客户反馈或原因')
                require(not old or clean(data.get('reason_category'), 300), '请选择本次阶段变化的原因分类')
                if old and not output['stage_history']:
                    output['history_incomplete'] = True
                output['stage_history'].append({'from': old.get('stage'), 'to': output['stage'], 'at': now(),
                                                'category': clean(data.get('reason_category'), 300),
                                                'reason': clean(data.get('reason'), 4000)})
                if old and old['stage'] in STAGES[-2:] and output['stage'] not in STAGES[-2:]:
                    output['closed_at'] = ''
            attribution = ('lead_staff_id', 'sales_staff_id', 'lead_channel', 'commission_rule_id')
            attribution_changed = any(output.get(k, '') != old.get(k, '') for k in attribution)
            require(not old.get('commission_locked') or not attribution_changed, '已成交的佣金归属和比例已锁定，不能直接改写')
            if output.get('lead_channel'):
                require(output['lead_channel'] in CHANNELS, '获客渠道无效')
            for key in ('lead_staff_id', 'sales_staff_id'):
                if output.get(key):
                    staff = self.get('staff', output[key])
                    if not old or output.get(key) != old.get(key):
                        require(staff['status'] == 'active', '不能分配给已停用员工')
                        require(key != 'sales_staff_id' or staff['role'] in ('Sales', '获客与 Sales'), '成交负责人需具有 Sales 职责')
            if output.get('commission_rule_id') and (attribution_changed or not old.get('commission_snapshot')):
                require(not old.get('commission_locked'), '已成交佣金不可重新配置')
                rule = self.get('commission_rules', output['commission_rule_id'])
                require(rule['status'] == 'active', '该佣金规则已停用')
                require(rule['channel'] == output.get('lead_channel'), '佣金规则与获客渠道不匹配')
                require(output.get('lead_staff_id') and output.get('sales_staff_id'), '计佣需分别指定获客人和成交 Sales，可为同一人')
                output['commission_snapshot'] = {k:v for k,v in rule.items() if k not in ('created_at', 'updated_at', 'notes')}
                output['commission_snapshot'].update(lead_name=self.get('staff', output['lead_staff_id'])['name'], sales_name=self.get('staff', output['sales_staff_id'])['name'], captured_at=now())
            elif not output.get('commission_rule_id'):
                output.pop('commission_snapshot', None)
            if output['stage'] == '已成交' and output.get('commission_snapshot'):
                output['commission_locked'] = True
            output['owner'] = 'Super Admin'
        if kind == 'communications':
            require(output.get('summary') and output.get('channel'), '请填写沟通方式和内容')
            output.setdefault('occurred_at', now())
        if kind in ('quotes', 'payments'):
            require(output.get('opportunity_id'), '请选择销售机会')
            output['amount_minor'] = amount(data.get('amount'))
            require(output['amount_minor'] is not None, '请填写金额')
            opportunity = self.get('opportunities', output['opportunity_id'])
            output['contact_id'] = opportunity['contact_id']
            require(output.get('currency') == opportunity.get('currency'), '记录币种必须与销售机会一致')
            if kind == 'quotes':
                require(output.get('issued_at') and output.get('send_status') in ('草稿', '已发送'), '请填写报价日期和发送状态')
                output['quote_version'] = len(self.rows('quotes', 'opportunity_id', output['opportunity_id'])) + 1
            else:
                require(output.get('paid_at') and output.get('evidence_reference'), '请填写收款日期与凭证引用')
                output.update(source='人工录入', verification_status='未核验')
        if kind in ('opportunities', 'quotes', 'payments'):
            output.setdefault('currency', 'AUD')
            require(output['currency'] in ('AUD', 'USD', 'CNY', 'NZD', 'EUR', 'GBP'), '请选择支持的币种')
            if old and old.get('currency') != output['currency']:
                require(not self.rows('quotes', 'opportunity_id', old['id']) and not self.rows('payments', 'opportunity_id', old['id']), '已有报价或收款时不能改币种')
        return output

    def save(self, kind, data, actor, identifier=None):
        require(kind in KINDS, '模块不存在', 404)
        old = self.get(kind, identifier) if identifier else None
        if old:
            require(kind not in ('communications', 'quotes', 'payments'), '历史记录不可覆盖，请新增记录或更正说明')
            require(not old.get('merged_into'), '该客户已经合并', 409)
            require(data.get('version') == old['version'], '记录已更新，请刷新后核对', 409)
        output = self.validate(kind, data, old)
        if kind == 'inquiries' and not old:
            output['contact_id'] = self.contact(output)['id']
        result = self.replace(kind, identifier, output, data['version']) if old else self.insert(kind, output)
        if kind == 'inquiries' and not old:
            destination = self.setting('notify_email')
            self.insert('notifications', {'inquiry_id': result['id'], 'email': destination, 'status': '待发送' if destination else '未配置', 'attempts': 0})
        self.audit(actor, '更新' if old else '创建', result['id'], {'kind': kind, 'before_status': (old or {}).get('status', (old or {}).get('stage')), 'after_status': output.get('status', output.get('stage')), 'reason': clean(data.get('reason'), 4000), 'fields': sorted(k for k in data if k != 'version')})
        return result

    def list_items(self, kind, query):
        require(kind in KINDS or kind == 'users', '模块不存在', 404)
        try:
            page = max(1, int(query.get('page', '1')))
            size = min(50, max(1, int(query.get('size', '25'))))
        except ValueError:
            raise Problem(400, '分页参数无效')
        if kind == 'users':
            conditions, args = ['1=1'], []
            if query.get('q'):
                conditions.append('(email LIKE ? OR id LIKE ?)')
                args += ['%'+query['q']+'%']*2
            if query.get('status'):
                conditions.append('status=?'); args.append(query['status'])
            for field, op in [('from', '>='), ('to', '<')]:
                if query.get(field):
                    conditions.append('registered_at '+op+' ?'); args.append(date_value(query[field], True))
            where = ' AND '.join(conditions)
            total = self.db.execute('SELECT count(*) FROM users WHERE '+where, args).fetchone()[0]
            items = [dict(r) for r in self.db.execute('SELECT * FROM users WHERE '+where+' ORDER BY registered_at DESC,id LIMIT ? OFFSET ?', args+[size,(page-1)*size])]
            for user in items:
                user['contact_id'] = next((c['id'] for c in self.rows('contacts', 'user_id', user['id']) if not c.get('merged_into')), '')
        else:
            conditions, args = ['kind=?'], [kind]
            if kind == 'contacts':
                conditions.append("coalesce(json_extract(data,'$.merged_into'),'')=''")
            if kind == 'inquiries' and query.get('spam') != '1':
                conditions.append("coalesce(json_extract(data,'$.is_spam'),0)=0")
            if query.get('q'):
                conditions.append("(coalesce(json_extract(data,'$.name'),'') || ' ' || coalesce(json_extract(data,'$.email'),'') || ' ' || coalesce(json_extract(data,'$.company_name'),'') || ' ' || coalesce(json_extract(data,'$.title'),'') || ' ' || id) LIKE ?")
                args.append('%'+query['q']+'%')
            for field in ('contact_id', 'opportunity_id', 'company_id', 'status', 'stage', 'priority', 'source'):
                if query.get(field):
                    conditions.append("json_extract(data,'$.%s')=?" % field); args.append(query[field])
            for field, op in [('from', '>='), ('to', '<')]:
                if query.get(field):
                    conditions.append(("json_extract(data,'$.received_at')" if kind == 'inquiries' else 'created_at')+' '+op+' ?'); args.append(date_value(query[field], True))
            if query.get('registered') == '1':
                conditions.append("coalesce(json_extract(data,'$.user_id'),'') != ''")
            if query.get('due'):
                conditions.append("json_extract(data,'$.status')='待完成' AND json_extract(data,'$.due_at')<=?")
                args.append(date_value(query['due'], True))
            if kind == 'opportunities' and query.get('pipeline'):
                conditions.append("coalesce(nullif(json_extract(data,'$.pipeline'),''),'默认销售')=?"); args.append(query['pipeline'])
            where = ' AND '.join(conditions)
            total = self.db.execute('SELECT count(*) FROM entities WHERE '+where, args).fetchone()[0]
            order = "json_extract(data,'$.due_at') ASC,id" if kind == 'tasks' else 'created_at DESC,id DESC'
            items = [self.unpack(r) for r in self.db.execute('SELECT * FROM entities WHERE '+where+' ORDER BY '+order+' LIMIT ? OFFSET ?', args+[size,(page-1)*size])]
            for item in items:
                if item.get('contact_id'):
                    item['contact_name'] = self.get('contacts', item['contact_id']).get('name') or '未填写'
                if kind == 'contacts':
                    item['open_opportunities'] = self.db.execute("SELECT count(*) FROM entities WHERE kind='opportunities' AND json_extract(data,'$.contact_id')=? AND json_extract(data,'$.stage') NOT IN ('已成交','已流失')", (item['id'],)).fetchone()[0]
                    next_task = self.db.execute("SELECT json_extract(data,'$.due_at') FROM entities WHERE kind='tasks' AND json_extract(data,'$.contact_id')=? AND json_extract(data,'$.status')='待完成' ORDER BY json_extract(data,'$.due_at') LIMIT 1", (item['id'],)).fetchone()
                    item['next_follow_up'] = next_task[0] if next_task else ''
        return {'items': items, 'total': total, 'page': page, 'size': size}

    def dashboard(self, query=None):
        query = query or {}
        count = lambda sql, args=(): self.db.execute(sql, args).fetchone()[0]
        timestamp = now()
        timezone = self.setting('timezone', 'Australia/Brisbane')
        today = dt.datetime.now(ZoneInfo(timezone)).replace(hour=0, minute=0, second=0, microsecond=0)
        end = (today + dt.timedelta(days=1)).astimezone(dt.timezone.utc).isoformat(timespec='seconds')
        days = int(query.get('days', '7'))
        require(days in (1, 7, 30), '请选择今天、近 7 天或近 30 天')
        start = (today-dt.timedelta(days=days-1)).astimezone(dt.timezone.utc).isoformat(timespec='seconds')
        metrics = {
            'contacts': count("SELECT count(*) FROM entities WHERE kind='contacts' AND coalesce(json_extract(data,'$.merged_into'),'')=''"),
            'inquiries': count("SELECT count(*) FROM entities WHERE kind='inquiries' AND json_extract(data,'$.status')='新咨询' AND coalesce(json_extract(data,'$.is_spam'),0)=0"),
            'users': count("SELECT count(*) FROM users"),
            'due': count("SELECT count(*) FROM entities WHERE kind='tasks' AND json_extract(data,'$.status')='待完成' AND json_extract(data,'$.due_at')<=?", (timestamp,)),
            'opportunities': count("SELECT count(*) FROM entities WHERE kind='opportunities' AND json_extract(data,'$.stage') NOT IN ('已成交','已流失')")
        }
        pipeline = []
        for stage in STAGES:
            field = 'agreed_amount_minor' if stage == '已成交' else 'expected_amount_minor'
            rows = self.db.execute("SELECT json_extract(data,'$.currency') currency,count(*) total,sum(CASE WHEN json_extract(data,'$."+field+"') IS NULL THEN 1 ELSE 0 END) unknown,sum(json_extract(data,'$."+field+"')) amount FROM entities WHERE kind='opportunities' AND json_extract(data,'$.stage')=? GROUP BY currency", (stage,)).fetchall()
            pipeline.append({'stage': stage, 'groups': [dict(r) for r in rows]})
        activity = {'inquiries': count("SELECT count(*) FROM entities WHERE kind='inquiries' AND coalesce(json_extract(data,'$.is_spam'),0)=0 AND json_extract(data,'$.received_at')>=? AND json_extract(data,'$.received_at')<?",(start,end)), 'users': count('SELECT count(*) FROM users WHERE registered_at>=? AND registered_at<?',(start,end))}
        return {'metrics': metrics, 'activity': activity, 'period': {'days': days, 'from': start, 'to': end}, 'pipeline': pipeline, 'tasks': self.list_items('tasks', {'due':end})['items'], 'inquiries': self.list_items('inquiries', {'status':'新咨询', 'size':'5'})['items'], 'updated_at':timestamp, 'timezone':timezone, 'today_end':end}

    def replay(self, key, payload, operation):
        key = clean(key, 150, True)
        fingerprint = digest(json_dump(payload))
        row = self.db.execute('SELECT * FROM replay WHERE key=?', (key,)).fetchone()
        if row:
            require(row['payload'] == fingerprint, '同一提交标识不能用于不同内容', 409)
            return json.loads(row['result'])
        result = operation()
        self.db.execute('INSERT INTO replay VALUES(?,?,?)', (key, fingerprint, json_dump(result)))
        return result

    def send_mail(self, address, subject, body, reference=''):
        """Called only after an explicit recipient configuration or signup."""
        require(os.getenv('CRM_SMTP_HOST') and os.getenv('CRM_SMTP_FROM'), '邮件服务未配置', 503)
        msg = EmailMessage()
        msg['Subject'], msg['From'], msg['To'] = subject, os.environ['CRM_SMTP_FROM'], address
        if reference:
            msg['Message-ID'] = '<'+reference+'@estate-studio.local>'
        msg.set_content(body)
        with smtplib.SMTP_SSL(os.environ['CRM_SMTP_HOST'], int(os.getenv('CRM_SMTP_PORT', '465')), timeout=10) as smtp:
            if os.getenv('CRM_SMTP_USER'):
                smtp.login(os.environ['CRM_SMTP_USER'], os.environ['CRM_SMTP_PASSWORD'])
            smtp.send_message(msg)

    def deliver_notifications(self):
        """Delivery is outside the inquiry transaction. Unknown delivery is not auto-retried."""
        while not getattr(self, 'stopping', False):
            item = None
            with self.lock, self.db:
                pending = self.db.execute("SELECT * FROM entities WHERE kind='notifications' AND json_extract(data,'$.status')='待发送' ORDER BY created_at LIMIT 1").fetchone()
                if pending:
                    row = self.unpack(pending)
                    item = self.replace('notifications', row['id'], dict(row, status='发送中', attempts=row.get('attempts', 0)+1), row['version'])
            if item:
                status = '已发送'
                try:
                    if self.preview:
                        status = '本地通知（未发送）'
                    else:
                        self.send_mail(item['email'], 'Estate Studio 新咨询', '咨询编号：'+item['inquiry_id']+'\n在后台查看：'+self.origin+'/admin#inquiries/'+item['inquiry_id'], item['id'])
                except (TimeoutError, OSError):
                    status = '发送结果不确定'
                except Exception:
                    status = '发送失败'
                with self.lock, self.db:
                    latest = self.get('notifications', item['id'])
                    self.replace('notifications', latest['id'], dict(latest, status=status), latest['version'])
                    self.audit('notification-worker', '咨询通知', item['inquiry_id'], {'status': status})
            time.sleep(0.5)

    def dispatch(self, method, path, data, query, token, csrf, ip):
        if path == '/api/config' and method == 'GET':
            return {'preview': self.preview, 'admin_configured': bool(self.admin), 'registration_available': self.preview or bool(os.getenv('CRM_SMTP_HOST'))}, None
        if path == '/api/notify/verify' and method == 'POST':
            self.limit('notify-verify:'+ip, 20)
            token_hash = digest(clean(data.get('token'), 100, True))
            row = self.db.execute("SELECT * FROM entities WHERE kind='notification_verifications' AND json_extract(data,'$.token')=?", (token_hash,)).fetchone()
            require(row, '验证链接无效')
            verification = self.unpack(row)
            require(not verification.get('used') and verification['expires'] > time.time() and self.setting('notify_pending') == verification['id'], '链接已使用、过期或被更新')
            self.replace('notification_verifications', verification['id'], dict(verification, used=True), verification['version'])
            self.db.execute('INSERT OR REPLACE INTO settings VALUES(?,?)', ('notify_email', json_dump(verification['email'])))
            self.audit('email-verification', '验证通知邮箱', verification['id'])
            return {'message': '本地通知邮箱已验证。后续新咨询会产生测试通知，不会发送邮件。' if self.preview else '通知邮箱已验证。后续新咨询将发送到此邮箱。'}, None
        if path == '/api/login' and method == 'POST':
            self.limit('login:'+ip, 8)
            if data.get('local'):
                require(self.preview and not self.admin and ip in ('127.0.0.1', '::1'), '本地入口不可用', 403)
                return self.login('local-owner', 'admin')
            valid = False
            if self.admin:
                code = clean(data.get('code'), 6)
                tick = int(time.time()//30)
                matched = next((t for t in range(tick-1, tick+2) if hmac.compare_digest(totp(self.admin['totp'], t), code)), None)
                valid = hmac.compare_digest(clean(data.get('email'),254).lower(), self.admin['email']) and hmac.compare_digest(password_hash(clean(data.get('password'),1024), self.admin['salt']), self.admin['hash']) and matched is not None and matched > self.setting('last_totp', 0)
                if valid:
                    self.db.execute('INSERT OR REPLACE INTO settings VALUES(?,?)', ('last_totp', json_dump(matched)))
            require(valid, '登录信息或验证码无效', 401)
            return self.login(self.admin['email'], 'admin')
        if path == '/api/public/inquiries' and method == 'POST':
            self.limit('inquiry:'+ip, 20)
            require(data.get('terms') is True, '请确认咨询资料处理说明')
            allowed = {k: data.get(k, '') for k in ('name','email','company','message')}
            allowed['email'] = email(allowed['email'])
            allowed.update(source='官网咨询',status='新咨询',received_at=now())
            payload = {k:v for k,v in allowed.items() if k != 'received_at'}
            def receive():
                inquiry = self.save('inquiries', allowed, 'visitor')
                self.insert('consents', {'inquiry_id': inquiry['id'], 'contact_id': inquiry['contact_id'], 'purpose': '处理本次咨询', 'version': '2026-09-06', 'accepted_at': now(), 'marketing': False})
                return {'id': inquiry['id'], 'received': True}
            result = self.replay('inquiry:'+clean(data.get('key'),100,True), payload, receive)
            return result, None
        if path == '/api/register' and method == 'POST':
            self.limit('register:'+ip, 10)
            address = email(data.get('email'))
            require(data.get('terms') is True, '请确认服务条款与隐私说明')
            require(self.preview or os.getenv('CRM_SMTP_HOST'), '邮件服务未配置，暂时无法注册', 503)
            raw = secrets.token_urlsafe(32)
            self.db.execute('INSERT INTO magic VALUES(?,?,?,0)', (digest(raw), address, time.time()+900))
            link = self.origin + '/register#verify=' + raw
            if self.preview:
                self.insert('outbox', {'email':address, 'link':link, 'expires_at':time.time()+900, 'status':'本地验证邮件，未发送', 'created_at':now()})
            else:
                msg = EmailMessage(); msg['Subject']='Estate Studio 验证邮箱'; msg['From']=os.environ['CRM_SMTP_FROM']; msg['To']=address
                msg.set_content('验证链接（15 分钟有效）：\n'+link+'\n如果不是你发起，请忽略。')
                try:
                    with smtplib.SMTP_SSL(os.environ['CRM_SMTP_HOST'], int(os.getenv('CRM_SMTP_PORT','465')), timeout=10) as smtp:
                        if os.getenv('CRM_SMTP_USER'):
                            smtp.login(os.environ['CRM_SMTP_USER'], os.environ['CRM_SMTP_PASSWORD'])
                        smtp.send_message(msg)
                except Exception:
                    raise Problem(503, '邮件暂时无法发送，请稍后重试')
            return {'message':'请通过邮箱验证链接继续。' if not self.preview else '本地验证邮件已保存，可由管理员在系统设置中查看。'}, None
        if path == '/api/verify' and method == 'POST':
            self.limit('verify:'+ip, 20)
            key = digest(clean(data.get('token'),100,True))
            magic = self.db.execute('SELECT * FROM magic WHERE token=?', (key,)).fetchone()
            require(magic and not magic['used'] and magic['expires'] > time.time(), '验证链接无效、已使用或已过期')
            user = self.db.execute('SELECT * FROM users WHERE email=?', (magic['email'],)).fetchone()
            require(not user or user['status'] == 'active', '该账户暂不可用', 403)
            self.db.execute('UPDATE magic SET used=1 WHERE token=?', (key,))
            if not user:
                identifier = uid()
                self.db.execute('INSERT INTO users VALUES(?,?,?,?,?)',(identifier,magic['email'],'active',now(),now()))
                self.contact({'email':magic['email'],'source':'邮箱注册'},identifier)
                self.insert('consents', {'user_id': identifier, 'purpose': '账户注册', 'version': '2026-09-06', 'accepted_at': now(), 'marketing': False})
                self.audit(identifier,'完成注册','',{'terms_version':'2026-09-06'})
            else:
                identifier = user['id']
                self.db.execute('UPDATE users SET last_login_at=? WHERE id=?',(now(),identifier))
            return self.login(identifier,'user')
        session = self.session(token)
        actor = session['user_id']
        if method != 'GET':
            require(hmac.compare_digest(session['csrf'], csrf), '请求验证失败，请刷新页面', 403)
        if path == '/api/session' and method == 'GET':
            return {'csrf':session['csrf'],'role':session['role'],'preview':self.preview,'identity':actor}, None
        if path == '/api/logout' and method == 'POST':
            self.db.execute('DELETE FROM sessions WHERE token=?',(digest(token),))
            return {'ok':True}, ''
        if path == '/api/account' and method == 'GET':
            require(session['role']=='user','请使用客户账户',403)
            return dict(self.db.execute('SELECT * FROM users WHERE id=?',(actor,)).fetchone()), None
        if path.startswith('/api/account/billing/'):
            require(session['role']=='user','请使用客户账户',403)
            action = path.rsplit('/', 1)[-1]
            require((action == 'membership' and method == 'GET') or (action in ('checkout', 'portal', 'cancel') and method == 'POST'), '会员操作不存在', 404)
            try:
                return account_billing(actor, action, self.preview), None
            except BillingError as error:
                raise Problem(error.status, error.message) from None
        require(session['role']=='admin','没有后台访问权限',403)
        if path == '/api/commissions' and method == 'GET':
            opportunities = self.rows('opportunities')
            return {'staff':self.rows('staff'), 'rules':self.rows('commission_rules'), 'channels':CHANNELS, 'roles':ROLES,
                    'items':[dict(id=o['id'], name=o['name'], stage=o['stage'], lead_staff_id=o.get('lead_staff_id'), sales_staff_id=o.get('sales_staff_id'), lead_channel=o.get('lead_channel'), snapshot=o.get('commission_snapshot'), **calculate(o)) for o in opportunities]}, None
        if path == '/api/funnel' and method == 'GET':
            days = query.get('days', '0')
            require(days in ('0', '30', '90', '365'), '统计周期无效')
            return analyse(self.rows('opportunities'), STAGES, int(days), query.get('pipeline', '')), None
        if path == '/api/dashboard' and method == 'GET':
            return self.dashboard(query), None
        if path == '/api/notifications' and method == 'GET':
            return {'email': self.setting('notify_email'), 'items': self.rows('notifications')[:100]}, None
        if path == '/api/notifications/configure' and method == 'POST':
            address = email(data.get('email'))
            raw = secrets.token_urlsafe(32)
            record = self.insert('notification_verifications', {'email': address, 'token': digest(raw), 'expires': time.time()+900, 'used': False})
            self.db.execute('INSERT OR REPLACE INTO settings VALUES(?,?)', ('notify_pending', json_dump(record['id'])))
            link = self.origin+'/notify#verify='+raw
            if self.preview:
                self.insert('outbox', {'purpose': 'notification', 'email': address, 'link': link, 'expires_at': time.time()+900, 'status': '本地验证邮件，未发送'})
            else:
                try:
                    self.send_mail(address, '验证 Estate Studio 通知邮箱', '请确认接收新咨询通知：\n'+link)
                except Exception:
                    raise Problem(503, '验证邮件发送失败，原通知邮箱保持不变')
            self.audit(actor, '申请更换通知邮箱', record['id'])
            return {'message': '验证链接已生成，请在本地验证邮箱中打开。' if self.preview else '请在新邮箱中完成验证后启用通知。'}, None
        if path == '/api/notifications/retry' and method == 'POST':
            record = self.get('notifications', data.get('id', ''))
            require(record['status'] in ('未配置', '发送失败', '发送结果不确定'), '该通知无需重试', 409)
            address = self.setting('notify_email')
            require(address, '请先验证通知接收邮箱')
            require(record['status'] != '发送结果不确定' or data.get('acknowledge') is True, '上次结果不确定，重试可能重复发送，请确认')
            result = self.replace('notifications', record['id'], dict(record, status='待发送', email=address), record['version'])
            self.audit(actor, '重试咨询通知', record['id'])
            return result, None
        if path == '/api/gateway' and method == 'GET':
            gateway = read_gateway(os.getenv('CRM_GATEWAY_DATA_FILE'))
            mappings = {r['subscription_id']: r for r in self.rows('subscription_links')}
            for subscription in gateway['subscriptions']:
                mapping = mappings.get(subscription['id'])
                subscription['user_id'] = mapping['user_id'] if mapping else None
            return gateway, None
        if path == '/api/gateway/link' and method == 'POST':
            gateway = read_gateway(os.getenv('CRM_GATEWAY_DATA_FILE'))
            identifier = clean(data.get('subscription_id'), 300, True)
            require(any(s['id'] == identifier for s in gateway['subscriptions']), '订阅不在已接入账本中')
            user_id = clean(data.get('user_id'), 100, True)
            require(self.db.execute('SELECT id FROM users WHERE id=?', (user_id,)).fetchone(), '注册用户不存在')
            reason = clean(data.get('reason'), 1000, True)
            require(not any(r['subscription_id'] == identifier for r in self.rows('subscription_links')), '该订阅已有映射，请先人工核对', 409)
            record = self.insert('subscription_links', {'subscription_id': identifier, 'user_id': user_id, 'reason': reason, 'verified_at': now(), 'source': '管理员核验'})
            self.audit(actor, '关联订阅', record['id'], {'user_id': user_id, 'subscription_id': identifier, 'reason': reason})
            return record, None
        if path == '/api/settings':
            if method == 'POST':
                timezone = clean(data.get('timezone'),100,True)
                try: ZoneInfo(timezone)
                except Exception: raise Problem(400,'时区无效')
                self.db.execute('INSERT OR REPLACE INTO settings VALUES(?,?)',('timezone',json_dump(timezone)))
                self.audit(actor,'更新时区','',{'timezone':timezone})
            require(method in ('GET','POST'),'请求方法无效',405)
            return {'timezone':self.setting('timezone','Australia/Brisbane'),'preview':self.preview,'services':[{'name':'咨询接收','status':'已接入','detail':'站内咨询实时保存至 SQLite'},{'name':'邮箱身份验证','status':'本地验证' if self.preview else ('已配置' if os.getenv('CRM_SMTP_HOST') else '未接入'),'detail':'本地邮件不发送' if self.preview else '通过 SMTP 发送验证链接'},{'name':'订阅账本 / 桌面 App','status':read_gateway(os.getenv('CRM_GATEWAY_DATA_FILE'))['status'],'detail':'订阅账本可按配置只读接入；桌面身份与本地项目不可远程控制'},{'name':'运营通知','status':('本地测试' if self.preview else '已配置') if self.setting('notify_email') else '未接入','detail':'通知邮箱需验证；发送失败不影响咨询入库'}]}, None
        if path == '/api/outbox' and method == 'GET':
            require(self.preview,'本地邮箱不可用',404)
            def pending_mail(record):
                key = digest(record['link'].split('#verify=')[-1])
                if record.get('purpose') == 'notification':
                    verification = self.db.execute("SELECT json_extract(data,'$.used') used FROM entities WHERE kind='notification_verifications' AND json_extract(data,'$.token')=?", (key,)).fetchone()
                else:
                    verification = self.db.execute('SELECT used FROM magic WHERE token=?', (key,)).fetchone()
                return record['expires_at'] > time.time() and verification and not verification['used']
            return {'items':[r for r in self.rows('outbox') if pending_mail(r)]}, None
        if path == '/api/audit' and method == 'GET':
            return {'items':[dict(r, details=json.loads(r['details'])) for r in self.db.execute('SELECT * FROM audit ORDER BY created_at DESC,rowid DESC LIMIT 100')]}, None
        if path.startswith('/api/users/') and method == 'POST':
            identifier = path.rsplit('/',1)[-1]
            user = self.db.execute('SELECT * FROM users WHERE id=?',(identifier,)).fetchone()
            require(user,'用户不存在',404)
            action = data.get('action'); require(action in ('disable','restore','revoke'),'操作无效')
            reason = clean(data.get('reason'),1000,True)
            if action != 'restore':
                self.db.execute('DELETE FROM sessions WHERE user_id=?',(identifier,))
            if action != 'revoke':
                self.db.execute('UPDATE users SET status=? WHERE id=?',('disabled' if action=='disable' else 'active',identifier))
            self.audit(actor,action,identifier,{'reason':reason})
            return {'ok':True}, None
        if path == '/api/import/headers' and method == 'POST':
            contents = clean(data.get('csv'), 2000000, True)
            headers = next(csv.reader(io.StringIO(contents.lstrip('\ufeff'))), [])
            require(headers and len(headers) <= 50 and len(set(headers)) == len(headers), 'CSV 表头为空、重复或超过 50 列')
            return {'headers': headers}, None
        if path == '/api/import/preview' and method == 'POST':
            contents = clean(data.get('csv'), 2000000, True)
            reader = csv.DictReader(io.StringIO(contents.lstrip('\ufeff')))
            mapping = data.get('mapping') or {k: k for k in ('name', 'email', 'phone', 'company', 'tags') if reader.fieldnames and k in reader.fieldnames}
            require(isinstance(mapping, dict) and reader.fieldnames and all(v in reader.fieldnames for v in mapping.values() if v), '列映射无效')
            require(mapping.get('name') and (mapping.get('email') or mapping.get('phone')), '请映射姓名，以及邮箱或电话列')
            rows, seen = [], set()
            for i,row in enumerate(reader):
                require(i < 5000,'每批最多 5000 行')
                row = {key: row.get(mapping.get(key)) for key in ('name', 'email', 'phone', 'company', 'tags')}
                try:
                    valid = self.validate('contacts',dict(name=row.get('name'),email=row.get('email'),phone=row.get('phone'),company_name=row.get('company'),tags=row.get('tags'),source='CSV 导入',priority='中'))
                    key = valid.get('email') or valid.get('phone')
                    require(key not in seen,'文件内重复联系方式',409); seen.add(key)
                    rows.append({'line':i+2,'data':valid,'error':''})
                except Problem as error:
                    rows.append({'line':i+2,'data':{},'error':error.message})
            batch = self.insert('imports',{'rows':rows,'status':'preview','actor':actor})
            return {'id':batch['id'],'total':len(rows),'valid':sum(not r['error'] for r in rows),'errors':[{'line':r['line'],'error':r['error']} for r in rows if r['error']]}, None
        if path == '/api/import/commit' and method == 'POST':
            batch = self.get('imports',data.get('id',''))
            require(batch['actor']==actor,'没有权限',403)
            if batch['status']=='complete': return batch['result'], None
            result={'created':0,'skipped':0,'failed':[]}
            for row in batch['rows']:
                if row['error']:
                    result['skipped']+=1; continue
                try:
                    self.save('contacts',row['data'],actor)
                    result['created']+=1
                except Problem as error:
                    result['failed'].append({'line':row['line'],'error':error.message})
            batch.update(status='complete',result=result)
            self.replace('imports',batch['id'],batch,batch['version']); self.audit(actor,'导入客户',batch['id'],result)
            return result, None
        if path == '/api/export' and method == 'POST':
            kind = data.get('kind'); require(kind in KINDS or kind=='users','导出类型无效')
            query = data.get('query',{}); require(isinstance(query,dict),'筛选无效')
            first = self.list_items(kind,dict(query,page='1',size='50'))
            require(first['total']<=10000,'首版每次最多导出 10000 条，请缩小筛选范围')
            items=list(first['items'])
            for page in range(2,(first['total']+49)//50+1):
                items+=self.list_items(kind,dict(query,page=str(page),size='50'))['items']
            fields={'contacts':['id','name','email','phone','company_name','tags','priority','source','created_at'], 'users':['id','email','status','registered_at'], 'inquiries':['id','name','email','phone','company','message','status','source','received_at'], 'opportunities':['id','name','contact_name','stage','expected_amount_minor','currency','created_at']}.get(kind,['id','contact_name','title','status','created_at'])
            if not data.get('pii'): fields=[f for f in fields if f not in ('email','phone','message')]
            stream=io.StringIO(); writer=csv.writer(stream); writer.writerow(fields)
            for item in items:
                writer.writerow([("'"+str(item.get(f,''))) if re.match(r'^[\s]*[=+@-]',str(item.get(f,''))) else item.get(f,'') for f in fields])
            record=self.insert('exports',{'actor':actor,'expires':time.time()+86400,'csv':'\ufeff'+stream.getvalue(),'count':len(items),'kind':kind})
            self.audit(actor,'导出',record['id'],{'kind':kind,'count':len(items),'pii':bool(data.get('pii'))})
            return {'id':record['id'],'count':len(items)}, None
        if path.startswith('/api/exports/') and method=='GET':
            record=self.get('exports',path.rsplit('/',1)[-1]); require(record['actor']==actor and record['expires']>time.time(),'导出文件不可用或已过期',403)
            return {'__csv':record['csv']}, None
        if path == '/api/merge' and method=='POST':
            source=self.get('contacts',data.get('source','')); target=self.get('contacts',data.get('target',''))
            require(source['id']!=target['id'] and not source.get('merged_into') and not target.get('merged_into'),'请选择两个独立客户')
            require(not (source.get('user_id') and target.get('user_id') and source['user_id']!=target['user_id']),'不同注册账户不能直接合并')
            require(data.get('source_version')==source['version'] and data.get('target_version')==target['version'],'档案已更新，请重新核对',409)
            reason=clean(data.get('reason'),1000,True)
            snapshots=[]
            for kind in ('inquiries','tasks','opportunities','communications','quotes','payments'):
                for row in self.rows(kind,'contact_id',source['id']):
                    snapshots.append({'kind':kind,'before':row})
                    self.replace(kind,row['id'],dict(row,contact_id=target['id']),row['version'])
            before_target=dict(target)
            target.update(user_id=target.get('user_id') or source.get('user_id',''))
            if target.get('user_id'):
                target['verification_state'] = 'verified' if target.get('email') == source.get('email') or before_target.get('user_id') else 'unverified'
            self.replace('contacts',target['id'],target,target['version'])
            self.replace('contacts',source['id'],dict(source,merged_into=target['id'],user_id=''),source['version'])
            record=self.insert('merges',{'source':source,'target':before_target,'snapshots':snapshots,'reason':reason,'status':'merged'})
            self.audit(actor,'合并客户',target['id'],{'merge_id':record['id'],'source':source['id'],'reason':reason})
            return {'id':record['id'],'target':target['id']},None
        if path == '/api/unmerge' and method=='POST':
            record=self.get('merges',data.get('id','')); require(record['status']=='merged','合并已撤销',409)
            reason=clean(data.get('reason'),1000,True)
            for kind,row in [('contacts',record['source']),('contacts',record['target'])]+[(r['kind'],r['before']) for r in record['snapshots']]:
                current=self.get(kind,row['id']); require(current['version']==row['version']+1,'合并后记录已变化，请人工核对再拆分',409)
            for kind,row in [('contacts',record['source']),('contacts',record['target'])]+[(r['kind'],r['before']) for r in record['snapshots']]:
                self.replace(kind,row['id'],row,row['version']+1)
            self.replace('merges',record['id'],dict(record,status='reversed'),record['version'])
            self.audit(actor,'撤销合并',record['id'],{'reason':reason}); return {'ok':True},None
        parts=path.strip('/').split('/')
        if len(parts)>=2 and parts[0]=='api' and parts[1] in KINDS+('users',):
            kind=parts[1]
            if len(parts)==2:
                if method=='GET':
                    result=self.list_items(kind,query)
                    for item in result['items']:
                        if 'email' in item: item['email']=re.sub(r'(^.).*(@.*$)',r'\1•••\2',item['email'])
                        if item.get('phone'): item['phone']='••••'+item['phone'][-4:]
                    return result,None
                if method=='POST' and kind!='users':
                    key=clean(data.get('_key'),100,True)
                    return self.replay(actor+':'+kind+':'+key,{k:v for k,v in data.items() if k!='_key'},lambda:self.save(kind,data,actor)),None
            if len(parts)==3 and kind!='users':
                if method=='PATCH': return self.save(kind,data,actor,parts[2]),None
                if method=='GET':
                    record=self.get(kind,parts[2]); self.audit(actor,'查看详情',record['id'],{'kind':kind})
                    if kind=='contacts':
                        record['related']={k:self.rows(k,'contact_id',record['id']) for k in ('inquiries','tasks','opportunities','communications')}
                        if record.get('user_id'):
                            user=self.db.execute('SELECT * FROM users WHERE id=?',(record['user_id'],)).fetchone(); record['user']=dict(user) if user else None
                            gateway = read_gateway(os.getenv('CRM_GATEWAY_DATA_FILE'))
                            linked_ids = {r['subscription_id'] for r in self.rows('subscription_links', 'user_id', record['user_id'])}
                            record['subscriptions'] = [s for s in gateway['subscriptions'] if s['id'] in linked_ids]
                            record['gateway_status'] = gateway['status']
                    if kind=='opportunities':
                        record['commission'] = calculate(record)
                        record['related']={k:self.rows(k,'opportunity_id',record['id']) for k in ('quotes','payments','tasks','communications')}
                    return record,None
        raise Problem(404,'接口不存在')

class Handler(BaseHTTPRequestHandler):
    def log_message(self,*args):
        pass  # Avoid storing verification URLs, PII or tokens in HTTP logs.

    def handle_request(self):
        parsed=urlparse(self.path)
        path=parsed.path
        crm=self.server.crm
        host=self.headers.get('Host','')
        allowed=urlparse(crm.origin).netloc
        if host != allowed:
            self.respond(403,{'error':'Host 不受信任'}); return
        if path.startswith('/api/'):
            try:
                data={}
                if self.command not in ('GET','HEAD'):
                    require(self.headers.get('Origin')==crm.origin,'跨站请求已拒绝',403)
                    require(self.headers.get('Content-Type','').split(';')[0]=='application/json','需要 JSON 请求',415)
                    length=int(self.headers.get('Content-Length','0')); require(0<=length<=2500000,'请求过大',413)
                    data=json.loads(self.rfile.read(length) or b'{}'); require(isinstance(data,dict),'请求必须是对象')
                cookies=http.cookies.SimpleCookie(self.headers.get('Cookie',''))
                cookie_name='crm_preview' if crm.preview else '__Host-crm'
                token=cookies[cookie_name].value if cookie_name in cookies else ''
                query={k:v[0] for k,v in parse_qs(parsed.query).items()}
                with crm.lock:
                    try:
                        crm.db.execute('BEGIN IMMEDIATE')
                        result,new_token=crm.dispatch(self.command,path,data,query,token,self.headers.get('X-CSRF-Token',''),self.client_address[0])
                        crm.db.commit()
                    except Exception:
                        # Rate-limit increments must survive rejected auth requests.
                        limits=list(crm.db.execute('SELECT * FROM limits'))
                        crm.db.rollback()
                        for row in limits:
                            crm.db.execute('INSERT OR REPLACE INTO limits VALUES(?,?,?)',tuple(row))
                        crm.db.commit()
                        raise
                headers={}
                if new_token is not None:
                    headers['Set-Cookie']=cookie_name+'='+new_token+'; Path=/; HttpOnly; SameSite=Strict'+('' if crm.preview else '; Secure')+('; Max-Age=0' if not new_token else '; Max-Age=43200')
                if '__csv' in result:
                    self.respond(200,result['__csv'].encode('utf-8'),dict(headers,**{'Content-Type':'text/csv; charset=utf-8','Content-Disposition':'attachment; filename="estate-studio-export.csv"'}))
                else: self.respond(200,result,headers)
            except Problem as error: self.respond(error.status,{'error':error.message})
            except (ValueError,TypeError,KeyError): self.respond(400,{'error':'请求参数无效'})
            except Exception: self.respond(500,{'error':'服务暂时无法完成请求，请重试'})
            return
        if self.command!='GET': self.respond(405,{'error':'请求方法无效'}); return
        files={'/membership':'membership.html','/membership.js':'membership.js','/membership.css':'membership.css','/':'index.html','/admin':'index.html','/register':'index.html','/notify':'index.html','/inquiry':'index.html','/app.js':'app.js','/styles.css':'styles.css','/favicon.svg':'favicon.svg'}
        if path not in files: self.respond(404,{'error':'页面不存在'}); return
        file=ROOT/'public'/files[path]
        types={'.html':'text/html; charset=utf-8','.js':'text/javascript; charset=utf-8','.css':'text/css; charset=utf-8','.svg':'image/svg+xml'}
        self.respond(200,file.read_bytes(),{'Content-Type':types[file.suffix]})

    def respond(self,status,body,headers=None):
        headers=headers or {}
        if not isinstance(body,bytes): body=json_dump(body).encode('utf-8')
        self.send_response(status)
        standard={'Content-Type':'application/json; charset=utf-8','Content-Length':str(len(body)),'Cache-Control':'no-store','X-Content-Type-Options':'nosniff','X-Frame-Options':'DENY','Referrer-Policy':'no-referrer','X-Robots-Tag':'noindex, nofollow','Content-Security-Policy':"default-src 'self'; script-src 'self'; style-src 'self'; img-src 'self'; connect-src 'self'; frame-ancestors 'none'; form-action 'self'; base-uri 'none'"}
        for key,value in dict(standard,**headers).items(): self.send_header(key,value)
        self.end_headers(); self.wfile.write(body)

    do_GET=handle_request
    do_POST=handle_request
    do_PATCH=handle_request

def main():
    parser=argparse.ArgumentParser(); parser.add_argument('--local-preview',action='store_true'); parser.add_argument('--setup-admin',action='store_true'); parser.add_argument('--backup',action='store_true'); parser.add_argument('--port',type=int,default=int(os.getenv('PORT','18767')))
    args=parser.parse_args()
    os.umask(0o077)
    path=Path(os.getenv('CRM_DATA_FILE',str(ROOT/'.data'/('preview.sqlite3' if args.local_preview else 'crm.sqlite3'))))
    if args.setup_admin:
        path.parent.mkdir(parents=True,exist_ok=True,mode=0o700)
        require(not (path.parent/'admin.json').exists(),'管理员已存在；不要覆盖现有身份')
        address=email(input('Admin email: ')); password=getpass.getpass('Password (at least 14 characters): ')
        require(len(password)>=14,'密码至少 14 位'); require(password==getpass.getpass('Repeat password: '),'密码不一致')
        salt=secrets.token_hex(16); secret=base64.b32encode(secrets.token_bytes(20)).decode()
        (path.parent/'admin.json').write_text(json_dump({'email':address,'salt':salt,'hash':password_hash(password,salt),'algorithm':'pbkdf2-sha256-600000','totp':secret}))
        os.chmod(path.parent/'admin.json',0o600)
        print('Add this secret to your authenticator, then store it securely: '+secret)
        return
    if args.backup:
        require(path.exists(),'数据库不存在')
        target=path.parent/('backup-'+dt.datetime.now().strftime('%Y%m%d-%H%M%S')+'.sqlite3')
        with sqlite3.connect(str(path)) as source,sqlite3.connect(str(target)) as dest: source.backup(dest)
        os.chmod(target,0o600); print('Backup created: '+str(target)); return
    origin=os.getenv('CRM_ORIGIN','http://127.0.0.1:'+str(args.port))
    crm=CRM(path,args.local_preview,origin)
    with crm.lock, crm.db:
        for item in crm.rows('notifications'):
            if item['status'] == '发送中':
                crm.replace('notifications', item['id'], dict(item, status='发送结果不确定'), item['version'])
    worker = threading.Thread(target=crm.deliver_notifications, daemon=True)
    worker.start()
    server=ThreadingHTTPServer(('127.0.0.1',args.port),Handler); server.crm=crm
    print('Estate Studio CRM: '+origin+'/admin'+(' (local preview; no external email)' if args.local_preview else ''),flush=True)
    try: server.serve_forever()
    except KeyboardInterrupt: pass
    finally:
        server.server_close()
        crm.stopping = True
        worker.join(timeout=12)
        crm.db.close()

if __name__=='__main__': main()
