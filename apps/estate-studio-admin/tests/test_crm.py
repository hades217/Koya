import importlib.util
import json
import tempfile
import sys
import threading
import unittest
import urllib.request
import urllib.error
import http.cookiejar
import sqlite3
from pathlib import Path
from http.server import ThreadingHTTPServer
from unittest.mock import patch

sys.path.insert(0,str(Path(__file__).resolve().parents[1]))
spec=importlib.util.spec_from_file_location('crm',Path(__file__).resolve().parents[1]/'server.py')
m=importlib.util.module_from_spec(spec);spec.loader.exec_module(m)

class CRMTest(unittest.TestCase):
    def setUp(self):
        self.temp=tempfile.TemporaryDirectory()
        self.crm=m.CRM(Path(self.temp.name)/'test.sqlite3',True)
        self.server=ThreadingHTTPServer(('127.0.0.1',0),m.Handler)
        self.origin='http://127.0.0.1:'+str(self.server.server_port)
        self.crm.origin=self.origin;self.server.crm=self.crm
        self.thread=threading.Thread(target=self.server.serve_forever,daemon=True);self.thread.start()
        self.client=self.new_client();self.csrf=''
        _,data=self.call('login',{'local':True});self.csrf=data['csrf']
    def tearDown(self):
        self.server.shutdown();self.server.server_close();self.thread.join();self.crm.db.close();self.temp.cleanup()
    def new_client(self):
        return urllib.request.build_opener(urllib.request.HTTPCookieProcessor(http.cookiejar.CookieJar()))
    def call(self,path,data=None,method=None,client=None,csrf=None,origin=None):
        if path=='public/inquiries' and data is not None:data=dict({'terms':True},**data)
        headers={'Content-Type':'application/json','Origin':origin or self.origin,'X-CSRF-Token':self.csrf if csrf is None else csrf}
        req=urllib.request.Request(self.origin+'/api/'+path,data=json.dumps(data).encode() if data is not None else None,headers=headers,method=method or ('POST' if data is not None else 'GET'))
        try:
            with (client or self.client).open(req) as res:
                body=res.read();return res.status,json.loads(body) if 'json' in res.headers.get('Content-Type','') else body.decode()
        except urllib.error.HTTPError as error:
            return error.code,json.loads(error.read())
    def create(self,kind,**data):
        status,result=self.call(kind,dict(data,_key=m.uid()));self.assertEqual(status,200,result);return result
    def contact(self,address='buyer@example.com'):
        return self.create('contacts',name='测试客户',email=address,source='测试',priority='高')
    def opportunity(self,contact=None):
        contact=contact or self.contact()
        return self.create('opportunities',contact_id=contact['id'],name='年度合作',summary='营销内容制作',currency='AUD',expected_amount='1200.50',stage='需求确认')
    def change(self,kind,record,**data):
        return self.call(kind+'/'+record['id'],dict(data,version=record['version']),'PATCH')
    def register(self,address='buyer@example.com'):
        customer=self.new_client()
        self.assertEqual(self.call('register',{'email':address,'terms':True},client=customer)[0],200)
        _,out=self.call('outbox');token=next(r['link'].split('#verify=')[1] for r in out['items'] if r['email']==address)
        status,auth=self.call('verify',{'token':token},client=customer);self.assertEqual(status,200,auth)
        return customer,auth,token

    def test_membership_bridge_uses_session_identity_and_csrf(self):
        customer,auth,_=self.register()
        _,account=self.call('account',client=customer)
        with patch.object(m,'account_billing',return_value={'url':'https://checkout.stripe.com/test'}) as bridge:
            code,_=self.call('account/billing/checkout',{'accountId':'attacker'},client=customer,csrf=auth['csrf'])
            self.assertEqual(code,200)
            bridge.assert_called_once_with(account['id'],'checkout',True)
        with patch.object(m,'account_billing') as bridge:
            self.assertEqual(self.call('account/billing/checkout',{},client=customer,csrf='wrong')[0],403)
            self.assertEqual(self.call('account/billing/checkout',{})[0],403)
            self.assertEqual(self.call('account/billing/checkout',client=customer)[0],404)
            bridge.assert_not_called()

    def test_membership_disabled_user_and_missing_configuration(self):
        customer,auth,_=self.register()
        with patch.dict(m.os.environ,{'CRM_BILLING_GATEWAY_URL':'','CRM_BILLING_ADMIN_SECRET':''}):
            self.assertEqual(self.call('account/billing/membership',client=customer)[0],503)
        _,account=self.call('account',client=customer)
        self.call('users/'+account['id'],{'action':'disable','reason':'Test'})
        with patch.object(m,'account_billing') as bridge:
            self.assertEqual(self.call('account/billing/checkout',{},client=customer,csrf=auth['csrf'])[0],401)
            bridge.assert_not_called()

    def test_unauthorized_and_csrf_boundaries(self):
        self.assertEqual(self.call('contacts',client=self.new_client())[0],401)
        self.assertEqual(self.call('contacts',{'name':'x','email':'a@example.com','_key':'x'},csrf='wrong')[0],403)
        self.assertEqual(self.call('login',{'local':True},origin='https://evil.example')[0],403)
        self.assertEqual(self.call('session',client=self.new_client())[0],401)
    def test_inquiry_retry_and_payload_conflict(self):
        payload={'name':'访客','email':'person@example.com','message':'需要演示','key':'request-1'}
        code,a=self.call('public/inquiries',payload,client=self.new_client());self.assertEqual(code,200,a)
        self.assertEqual(self.call('public/inquiries',payload)[1]['id'],a['id'])
        self.assertEqual(self.call('public/inquiries',dict(payload,message='changed'))[0],409)
        code,b=self.call('public/inquiries',dict(payload,key='request-2'));self.assertEqual(code,200)
        self.assertNotEqual(a['id'],b['id']);self.assertEqual(self.call('contacts')[1]['total'],1)
    def test_failed_write_rolls_back(self):
        initial=self.call('contacts')[1]['total']
        code,_=self.call('inquiries',{'name':'访客','email':'invalid','_key':'bad'})
        self.assertEqual(code,400);self.assertEqual(self.call('contacts')[1]['total'],initial)
    def test_unverified_not_registered_and_one_time_verification(self):
        self.call('register',{'email':'test@example.com','terms':True})
        self.assertEqual(self.call('users')[1]['total'],0)
        customer,auth,token=self.register('test2@example.com')
        self.assertEqual(auth['role'],'user');self.assertEqual(self.call('verify',{'token':token},client=customer)[0],400)
        self.assertEqual(self.call('users')[1]['total'],1)
        self.assertEqual(self.call('contacts',client=customer)[0],403)
    def test_consultation_registration_links_stable_contact(self):
        self.call('public/inquiries',{'name':'Buyer','email':'Buyer@Example.COM','message':'Demo','key':'a'})
        original=self.call('contacts')[1]['items'][0]
        self.register()
        _,contact=self.call('contacts/'+original['id']);self.assertTrue(contact['user_id'])
        self.assertEqual(len(contact['related']['inquiries']),1);self.assertEqual(self.call('contacts')[1]['total'],1)
        self.assertEqual(contact['related']['inquiries'][0]['status'],'新咨询')
    def test_account_disable_restore_revoke(self):
        customer,auth,_=self.register();_,account=self.call('account',client=customer)
        self.assertEqual(self.call('users/'+account['id'],{'action':'disable','reason':'测试'})[0],200)
        self.assertEqual(self.call('account',client=customer)[0],401)
        self.call('users/'+account['id'],{'action':'restore','reason':'已核对'})
        self.assertEqual(self.call('account',client=customer)[0],401)
        again,_,_=self.register();self.assertEqual(self.call('account',client=again)[0],200)
        self.call('users/'+account['id'],{'action':'revoke','reason':'安全检查'})
        self.assertEqual(self.call('account',client=again)[0],401)
    def test_cannot_forge_admin_or_user_link(self):
        c=self.create('contacts',name='Forgery',email='fake@example.com',user_id='local-owner',role='admin')
        self.assertFalse(c.get('user_id'));self.assertNotIn('role',c)
        self.assertEqual(self.call('users',{'email':'a@example.com','role':'admin'})[0],404)
    def test_optimistic_lock_and_history(self):
        c=self.contact();status,new=self.change('contacts',c,name='新姓名');self.assertEqual(status,200)
        self.assertEqual(self.change('contacts',c,name='陈旧覆盖')[0],409)
        self.assertEqual(self.call('contacts/'+c['id'])[1]['name'],'新姓名')
    def test_inquiry_immutable_and_close_reason(self):
        i=self.create('inquiries',name='Guest',email='g@example.com',message='original',source='手工录入')
        self.assertEqual(self.change('inquiries',i,message='overwrite')[0],400)
        self.assertEqual(self.change('inquiries',i,status='已关闭')[0],400)
        self.assertEqual(self.change('inquiries',i,status='已安排演示')[0],400)
        code,closed=self.change('inquiries',i,status='已关闭',reason='暂无需求');self.assertEqual(code,200)
        self.assertEqual(self.change('inquiries',closed,status='跟进中',reason='')[0],400)
    def test_task_validation_and_reschedule_history(self):
        c=self.contact();t=self.create('tasks',contact_id=c['id'],title='回访',due_at='2026-09-01T10:00:00+10:00',status='待完成')
        self.assertEqual(self.change('tasks',t,due_at='2026-09-08T10:00:00+10:00')[0],400)
        code,t=self.change('tasks',t,due_at='2026-09-08T10:00:00+10:00',reason='客户改期');self.assertEqual(code,200)
        self.assertEqual(len(t['deadline_history']),1)
        self.assertEqual(self.change('tasks',t,status='已完成')[0],400)
        code,t=self.change('tasks',t,status='已完成',result='电话沟通完成');self.assertEqual(code,200);self.assertTrue(t['completed_at'])
    def test_wrong_contact_relation_rejected(self):
        a=self.contact();b=self.contact('second@example.com');o=self.opportunity(a)
        self.assertEqual(self.call('tasks',{'contact_id':b['id'],'opportunity_id':o['id'],'title':'x','due_at':m.now(),'_key':m.uid()})[0],400)
    def test_commission_attribution_snapshots_and_lock(self):
        lead=self.create('staff',name='获客员工',role='获客')
        sales=self.create('staff',name='Sales 员工',role='Sales')
        rule=self.create('commission_rules',name='地推标准',channel='线下地推',mode='direct',lead_rate='2',sales_rate='5')
        o=self.opportunity()
        self.assertEqual(self.call('commissions',client=self.new_client())[0],401)
        self.assertEqual(self.change('opportunities',o,lead_staff_id=lead['id'],sales_staff_id=lead['id'])[0],400)
        self.assertEqual(self.change('opportunities',o,lead_channel='转介绍',lead_staff_id=lead['id'],sales_staff_id=sales['id'],commission_rule_id=rule['id'])[0],400)
        code,o=self.change('opportunities',o,lead_channel='线下地推',lead_staff_id=lead['id'],sales_staff_id=sales['id'],commission_rule_id=rule['id'])
        self.assertEqual(code,200)
        self.assertEqual(o['commission_snapshot']['lead_rate_bps'],200)
        self.assertEqual(self.change('commission_rules',rule,lead_rate='3')[0],200)
        code,o=self.change('opportunities',o,next_action='安排演示',commission_snapshot={'mode':'forged'})
        self.assertEqual(code,200)
        self.assertEqual(o['commission_snapshot']['lead_rate_bps'],200)
        code,o=self.change('opportunities',o,stage='已成交',agreed_amount='10000',closed_at=m.now(),acceptance_reference='客户邮件',reason='认可方案',reason_category='产品价值')
        self.assertEqual(code,200)
        self.assertTrue(o['commission_locked'])
        item=self.call('commissions')[1]['items'][0]
        self.assertEqual(item['lead_minor'],20000)
        self.assertEqual(item['sales_minor'],50000)
        self.assertEqual(self.change('opportunities',o,commission_rule_id='')[0],400)
        code,o=self.change('opportunities',o,stage='需求确认',reason='重新沟通',reason_category='其他')
        self.assertEqual(code,200)
        self.assertEqual(self.change('opportunities',o,lead_staff_id=sales['id'])[0],400)
        same=self.opportunity(self.crm.get('contacts',o['contact_id']))
        self.assertEqual(self.change('opportunities',same,lead_channel='线下地推',lead_staff_id=sales['id'],sales_staff_id=sales['id'],commission_rule_id=rule['id'])[0],200)
        self.assertEqual(self.call('commission_rules',dict(name='invalid',channel='运营',mode='direct',lead_rate='80',sales_rate='40',_key=m.uid()))[0],400)

    def test_funnel_history_reasons_reopen_and_permissions(self):
        o=self.opportunity()
        self.assertEqual(self.call('funnel',client=self.new_client())[0],401)
        self.assertEqual(self.change('opportunities',o,stage='商务确认')[0],400)
        code,o=self.change('opportunities',o,stage='商务确认',reason='确认需求',reason_category='需求匹配',stage_history=[{'to':'已成交'}])
        self.assertEqual(code,200)
        self.assertEqual(len(o['stage_history']),2)
        code,o=self.change('opportunities',o,stage='已流失',closed_at=m.now(),reason='预算不够',reason_category='价格 / 预算')
        self.assertEqual(code,200)
        d=self.call('funnel')[1]
        self.assertEqual(d['stages'][0]['conversion'],1)
        self.assertEqual(d['stages'][1]['entered'],0)
        self.assertIsNone(d['stages'][1]['conversion'])
        self.assertEqual(d['lost'],1)
        self.assertEqual(d['reasons'][0]['reason'],'预算不够')
        code,o=self.change('opportunities',o,stage='需求确认',reason='新预算已批准',reason_category='时间 / 优先级')
        self.assertEqual(code,200)
        self.assertEqual(o['closed_at'],'')
        self.assertEqual(self.call('funnel')[1]['stages'][0]['entered'],1)
        self.assertIsNone(self.call('funnel')[1]['win_rate'])
        self.assertEqual(self.change('opportunities',o,pipeline='另一个')[0],400)
        self.assertEqual(self.call('funnel?pipeline=missing')[1]['total'],0)
        self.assertEqual(self.call('opportunities?pipeline=missing')[1]['total'],0)
        self.assertEqual(self.call('funnel?days=bad')[0],400)

    def test_opportunity_stage_quote_and_win_contract(self):
        o=self.opportunity();self.assertEqual(o['expected_amount_minor'],120050)
        self.assertEqual(self.change('opportunities',o,stage='报价中')[0],400)
        q=self.create('quotes',opportunity_id=o['id'],amount='1200.50',currency='AUD',issued_at=m.now(),send_status='已发送')
        q2=self.create('quotes',opportunity_id=o['id'],amount='1300',currency='AUD',issued_at=m.now(),send_status='草稿')
        self.assertEqual(q2['quote_version'],2)
        code,o=self.change('opportunities',o,stage='报价中',reason='客户要求正式报价',reason_category='需求匹配');self.assertEqual(code,200)
        self.assertEqual(self.change('opportunities',o,stage='已成交')[0],400)
        code,o=self.change('opportunities',o,stage='已成交',agreed_amount='1200.50',closed_at=m.now(),acceptance_reference='客户邮件确认',reason='客户确认方案价值',reason_category='产品价值');self.assertEqual(code,200)
        self.assertEqual(self.call('payments')[1]['total'],0)
        self.assertEqual(self.change('opportunities',o,stage='需求确认')[0],400)
        self.assertEqual(self.change('quotes',q,amount='1')[0],400)
    def test_payment_not_provider_verified_or_negative(self):
        o=self.opportunity()
        self.assertEqual(self.call('payments',{'opportunity_id':o['id'],'amount':'-1','currency':'AUD','paid_at':m.now(),'evidence_reference':'x','_key':m.uid()})[0],400)
        p=self.create('payments',opportunity_id=o['id'],amount='100',currency='AUD',paid_at=m.now(),evidence_reference='BANK-123',verification_status='verified')
        self.assertEqual(p['verification_status'],'未核验')
        self.assertEqual(self.call('payments',{'opportunity_id':o['id'],'amount':'100','currency':'USD','paid_at':m.now(),'evidence_reference':'x','_key':m.uid()})[0],400)
    def test_import_preview_replay_and_conflicts(self):
        _,preview=self.call('import/preview',{'csv':'name,email\nAlice,a@example.com\nDuplicate,a@example.com\nBad,nope\nBob,b@example.com'})
        self.assertEqual(preview['valid'],2);self.assertEqual(len(preview['errors']),2)
        self.assertEqual(self.call('contacts')[1]['total'],0)
        _,r=self.call('import/commit',{'id':preview['id']});self.assertEqual(r['created'],2)
        self.assertEqual(self.call('import/commit',{'id':preview['id']})[1],r)
        self.assertEqual(self.call('contacts')[1]['total'],2)
    def test_export_pii_formula_and_expiry(self):
        self.create('contacts',name='=HYPERLINK("x")',email='x@example.com')
        _,export=self.call('export',{'kind':'contacts','pii':False,'query':{}})
        _,csv=self.call('exports/'+export['id']);self.assertNotIn('x@example.com',csv);self.assertIn("'=HYPERLINK",csv)
        with self.crm.lock,self.crm.db:
            r=self.crm.get('exports',export['id']);r['expires']=0;self.crm.replace('exports',r['id'],r,r['version'])
        self.assertEqual(self.call('exports/'+export['id'])[0],403)
    def test_merge_and_safe_unmerge(self):
        a=self.contact();b=self.contact('second@example.com');o=self.opportunity(a)
        _,merged=self.call('merge',{'source':a['id'],'target':b['id'],'source_version':a['version'],'target_version':b['version'],'reason':'核对同一人'})
        self.assertEqual(self.call('contacts')[1]['total'],1)
        self.assertEqual(self.call('opportunities/'+o['id'])[1]['contact_id'],b['id'])
        self.assertEqual(self.call('unmerge',{'id':merged['id'],'reason':'误合并'})[0],200)
        self.assertEqual(self.call('contacts')[1]['total'],2)
        self.assertEqual(self.call('opportunities/'+o['id'])[1]['contact_id'],a['id'])
    def test_merge_with_different_users_blocked(self):
        self.register('a@example.com');self.register('b@example.com')
        _,r=self.call('contacts');a,b=r['items']
        self.assertEqual(self.call('merge',{'source':a['id'],'target':b['id'],'source_version':a['version'],'target_version':b['version'],'reason':'bad'})[0],400)
    def test_reopen_database_preserves_entities_audit_and_backup(self):
        c=self.contact()
        target=Path(self.temp.name)/'backup.sqlite3'
        with self.crm.lock,sqlite3.connect(str(target)) as dest:self.crm.db.backup(dest)
        restored=m.CRM(target,True)
        self.assertEqual(restored.get('contacts',c['id'])['email'],'buyer@example.com')
        self.assertTrue(restored.db.execute('SELECT count(*) FROM audit').fetchone()[0]);restored.db.close()
    def test_dashboard_counts_spam_and_unknown_amount(self):
        self.contact();self.create('inquiries',name='Spam',email='s@example.com',is_spam=True)
        _,d=self.call('dashboard');self.assertEqual(d['metrics']['inquiries'],0);self.assertEqual(d['metrics']['users'],0)
        _,s=self.call('settings');self.assertEqual(next(x for x in s['services'] if '订阅' in x['name'])['status'],'未接入')
    def test_pagination_and_search(self):
        with self.crm.lock,self.crm.db:
            for i in range(55):self.crm.insert('contacts',{'name':'Person '+str(i),'email':str(i)+'@example.com'})
        _,d=self.call('contacts?size=25&page=2');self.assertEqual(d['total'],55);self.assertEqual(len(d['items']),25)
        _,d=self.call('contacts?q=Person%2054');self.assertEqual(d['total'],1)
        self.assertIn('•••',d['items'][0]['email'])
    def test_rate_limit_persists_on_failed_logins(self):
        for _ in range(7):self.assertEqual(self.call('login',{'email':'no@example.com','password':'x','code':'000000'})[0],401)
        self.assertEqual(self.call('login',{'local':True})[0],429)
    def test_registration_requires_consent_and_no_public_outbox(self):
        self.assertEqual(self.call('register',{'email':'x@example.com'})[0],400)
        self.assertEqual(self.call('outbox',client=self.new_client())[0],401)
    def test_replayed_admin_create_is_not_duplicate(self):
        payload={'name':'Same','email':'same@example.com','_key':'one'}
        a=self.call('contacts',payload);b=self.call('contacts',payload);self.assertEqual(a,b)
        self.assertEqual(self.call('contacts')[1]['total'],1)
    def test_static_and_host_security(self):
        with urllib.request.urlopen(self.origin+'/admin') as res:
            self.assertIn("script-src 'self'",res.headers['Content-Security-Policy'])
            self.assertNotIn(b'local-owner',res.read())
        req=urllib.request.Request(self.origin+'/api/config',headers={'Host':'evil.example'})
        with self.assertRaises(urllib.error.HTTPError) as cm:urllib.request.urlopen(req)
        self.assertEqual(cm.exception.code,403)


    def test_gateway_adapter_never_exposes_provider_payloads(self):
        ledger=Path(self.temp.name)/'gateway.json'
        ledger.write_text(json.dumps({'subscriptions':{'sub-1':{'id':'sub-1','status':'active','plan':'studio','includedCredits':8,'topUpCredits':0,'usedCredits':2,'reservedCredits':1,'activationCode':'must-not-leak'}},'requests':{'request-1':{'subscriptionId':'sub-1','status':'succeeded','prompt':'private','outputs':['secret-media']}}}))
        with patch.dict('os.environ',{'CRM_GATEWAY_DATA_FILE':str(ledger)}):
            code,result=self.call('gateway');self.assertEqual(code,200)
            self.assertEqual(result['subscriptions'][0]['remaining'],5)
            self.assertNotIn('must-not-leak',json.dumps(result));self.assertNotIn('private',json.dumps(result));self.assertNotIn('secret-media',json.dumps(result))
            self.register();user=self.call('users')[1]['items'][0]
            self.assertEqual(self.call('gateway/link',{'subscription_id':'sub-1','user_id':user['id'],'reason':'测试映射核验'})[0],200)
            self.assertEqual(self.call('gateway/link',{'subscription_id':'sub-1','user_id':user['id'],'reason':'重试'})[0],409)
            self.assertEqual(self.call('gateway')[1]['subscriptions'][0]['user_id'],user['id'])
    def test_gateway_missing_and_incomplete_are_not_zero(self):
        ledger=Path(self.temp.name)/'missing.json'
        self.assertEqual(m.read_gateway(str(ledger))['status'],'暂不可用')
        ledger.write_text(json.dumps({'subscriptions':{'sub':{'id':'sub','status':'active'}}}))
        self.assertIsNone(m.read_gateway(str(ledger))['subscriptions'][0]['remaining'])
    def test_mfa_login_requires_factors_and_rejects_replay(self):
        secret='JBSWY3DPEHPK3PXP';salt='00'*16
        self.crm.admin={'email':'admin@example.com','salt':salt,'hash':m.password_hash('fixture-password-long',salt),'totp':secret}
        self.assertEqual(self.call('login',{'email':'admin@example.com','password':'wrong','code':m.totp(secret)})[0],401)
        payload={'email':'admin@example.com','password':'fixture-password-long','code':m.totp(secret)}
        self.assertEqual(self.call('login',payload)[0],200)
        self.assertEqual(self.call('login',payload)[0],401)
    def test_configured_preview_disables_shortcut_and_old_session(self):
        secret='JBSWY3DPEHPK3PXP';salt='00'*16
        self.assertFalse(self.call('config')[1]['admin_configured'])
        self.crm.admin={'email':'admin@example.com','salt':salt,'hash':m.password_hash('fixture-password-long',salt),'totp':secret}
        config=self.call('config')[1]
        self.assertTrue(config['admin_configured'])
        self.assertEqual(set(config),{'preview','admin_configured','registration_available'})
        self.assertEqual(self.call('login',{'local':True})[0],403)
        self.assertEqual(self.call('session')[0],401)
        self.assertEqual(self.call('contacts')[0],401)
        status,auth=self.call('login',{'email':'admin@example.com','password':'fixture-password-long','code':m.totp(secret)})
        self.assertEqual(status,200)
        self.csrf=auth['csrf']
        self.assertEqual(self.call('session')[1]['identity'],'admin@example.com')
        self.assertEqual(self.call('contacts')[0],200)

    def test_opportunity_reassignment_with_quote_is_blocked(self):
        o=self.opportunity();b=self.contact('other@example.com')
        self.create('quotes',opportunity_id=o['id'],amount='20',currency='AUD',issued_at=m.now(),send_status='草稿')
        self.assertEqual(self.change('opportunities',o,contact_id=b['id'])[0],400)


    def test_notification_verification_and_failure_preserve_inquiry(self):
        self.assertEqual(self.call('notifications/configure',{'email':'owner@example.com'})[0],200)
        self.assertEqual(self.call('notifications')[1]['email'],'')
        out=self.call('outbox')[1]['items'];token=next(r['link'].split('#verify=')[1] for r in out if r.get('purpose')=='notification')
        self.assertEqual(self.call('notify/verify',{'token':token})[0],200)
        self.assertEqual(self.call('notify/verify',{'token':token})[0],400)
        inquiry=self.create('inquiries',name='Visitor',email='v@example.com')
        notices=self.call('notifications')[1]['items'];self.assertEqual(notices[0]['status'],'待发送')
        self.crm.preview=False;self.crm.stopping=False
        with patch.object(self.crm,'send_mail',side_effect=OSError('test timeout')),patch.object(m.time,'sleep',side_effect=lambda _:setattr(self.crm,'stopping',True)):
            self.crm.deliver_notifications()
        self.crm.preview=True
        self.assertEqual(self.call('inquiries/'+inquiry['id'])[0],200)
        record=self.call('notifications')[1]['items'][0];self.assertEqual(record['status'],'发送结果不确定')
        self.assertEqual(self.call('notifications/retry',{'id':record['id']})[0],400)
        self.assertEqual(self.call('notifications/retry',{'id':record['id'],'acknowledge':True})[0],200)
        with self.crm.lock,self.crm.db:
            r=self.crm.get('notifications',record['id']);self.crm.replace('notifications',r['id'],dict(r,status='已发送'),r['version'])
        self.assertEqual(self.call('notifications/retry',{'id':record['id'],'acknowledge':True})[0],409)
    def test_dashboard_period_matches_detail_and_unknown_is_separate(self):
        o=self.opportunity()
        self.assertEqual(self.call('dashboard?days=365')[0],400)
        self.create('inquiries',name='Old',email='old@example.com',received_at='2020-01-01T00:00:00+00:00')
        d=self.call('dashboard?days=7')[1]
        self.assertEqual(d['activity']['inquiries'],0)
        from urllib.parse import urlencode
        q=urlencode({'from':d['period']['from'],'to':d['period']['to']})
        self.assertEqual(self.call('inquiries?'+q)[1]['total'],d['activity']['inquiries'])


    def test_csv_column_mapping_and_consent_are_explicit(self):
        content='姓名,电子邮件,团队\nAlice,mapped@example.com,Studio'
        self.assertEqual(self.call('import/headers',{'csv':content})[1]['headers'],['姓名','电子邮件','团队'])
        code,p=self.call('import/preview',{'csv':content,'mapping':{'name':'姓名','email':'电子邮件','company':'团队'}})
        self.assertEqual(code,200,p);self.assertEqual(p['valid'],1)
        self.assertEqual(self.call('public/inquiries',{'name':'No consent','email':'x@example.com','key':'bad','terms':False})[0],400)
    def test_preview_cannot_reopen_production_mode_database(self):
        with self.crm.lock,self.crm.db:
            self.crm.db.execute("UPDATE settings SET value=? WHERE key='database_mode'",(json.dumps('production'),))
        with self.assertRaises(m.Problem):m.CRM(self.crm.path,True)

if __name__=='__main__':unittest.main()
