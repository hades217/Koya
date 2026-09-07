"""Server-only bridge from verified CRM identity to the subscription gateway."""
import json
import os
from urllib.parse import urlparse
from urllib.request import Request, build_opener, HTTPRedirectHandler
from urllib.error import HTTPError, URLError

class BillingError(Exception):
    def __init__(self, status, message):
        self.status, self.message = status, message

class NoRedirect(HTTPRedirectHandler):
    def redirect_request(self, req, fp, code, msg, headers, newurl):
        return None

def account_billing(account_id, action, preview=False):
    origin = os.environ.get('CRM_BILLING_GATEWAY_URL', '').rstrip('/')
    secret = os.environ.get('CRM_BILLING_ADMIN_SECRET', '')
    parsed = urlparse(origin)
    if not origin or not secret:
        raise BillingError(503, '会员支付尚未配置，请联系 Estate Studio。')
    local = parsed.scheme == 'http' and parsed.hostname in ('127.0.0.1', 'localhost')
    if parsed.username or parsed.password or parsed.path or parsed.query or parsed.fragment or not (parsed.scheme == 'https' or local):
        raise BillingError(503, '会员服务地址配置无效。')
    routes = {'membership': ('GET', '/v1/billing/membership'), 'checkout': ('POST', '/v1/billing/checkout'), 'portal': ('POST', '/v1/billing/portal'), 'cancel': ('POST', '/v1/billing/cancel')}
    if action not in routes:
        raise BillingError(404, '会员操作不存在。')
    def call(method, path, data=None, headers=None):
        request = Request(origin + path, data=json.dumps(data).encode() if data is not None else None, method=method, headers={'Content-Type': 'application/json', **(headers or {})})
        try:
            with build_opener(NoRedirect()).open(request, timeout=25) as response:
                return json.load(response)
        except HTTPError as error:
            messages = {409: '订阅状态需要更新，请刷新页面或进入账单管理。', 503: '会员支付尚未配置，请稍后再试。'}
            raise BillingError(error.code if error.code in (409, 503) else 502, messages.get(error.code, '会员服务暂时不可用，请稍后重试。')) from None
        except (URLError, TimeoutError, ValueError):
            raise BillingError(502, '会员服务暂时无法连接，请稍后重试。') from None
    identity = call('POST', '/v1/admin/memberships', {'accountId': account_id, 'expectedMode': 'test' if preview else os.environ.get('CRM_BILLING_MODE', 'test')}, {'X-Gateway-Admin': secret})
    auth = call('POST', '/v1/auth/exchange', {'subscriptionId': identity['subscriptionId'], 'activationCode': identity['activationCode']})
    method, path = routes[action]
    # Neither activation material nor gateway/admin tokens are returned to the browser.
    return call(method, path, {} if method == 'POST' else None, {'Authorization': 'Bearer ' + auth['accessToken'], 'X-Estate-Subscription': identity['subscriptionId']})
