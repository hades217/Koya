let csrf = '';
const el = (id) => document.getElementById(id);
async function api(path, method = 'GET') {
  const response = await fetch(path, { method, headers: { 'Content-Type': 'application/json', 'X-CSRF-Token': csrf }, ...(method === 'GET' ? {} : { body: '{}' }) });
  const result = await response.json();
  if (!response.ok) throw Object.assign(new Error(result.error || '暂时无法完成请求'), { status: response.status });
  return result;
}
async function refresh() {
  for (const id of ['subscribe', 'portal', 'cancel', 'login']) el(id).hidden = true;
  try {
    const session = await api('/api/session'); csrf = session.csrf;
    if (session.role !== 'user') throw new Error('请使用客户账户登录后管理会员。');
    el('refresh').hidden = false;
    const membership = await api('/api/account/billing/membership');
    const active = membership.basicFeaturesEnabled;
    el('status').textContent = active ? 'Basic 会员已开通' : ({ past_due: '续费付款未完成', canceled: '订阅已结束', incomplete: '首次付款未完成' }[membership.status] || '尚未开通会员');
    el('detail').textContent = membership.cancelAtPeriodEnd && active ? '已取消自动续费，可用至 ' + new Date(membership.periodEndsAt * 1000).toLocaleDateString('zh-CN') : active ? '当前周期结束：' + new Date(membership.periodEndsAt * 1000).toLocaleDateString('zh-CN') : '付款状态会在 Stripe 确认后更新。';
    el('subscribe').hidden = !['inactive', 'canceled', 'incomplete_expired'].includes(membership.status);
    el('portal').hidden = !membership.hasBillingCustomer;
    el('cancel').hidden = !active || membership.cancelAtPeriodEnd;
  } catch (error) {
    el('status').textContent = error.status === 401 ? '登录后开通会员' : '会员服务暂不可用';
    el('detail').textContent = error.message;
    el('login').hidden = error.status !== 401;
  }
}
async function action(name) {
  if (name === 'cancel' && !window.confirm('取消后不再自动续费，已付款周期内仍可使用。确定取消吗？')) return;
  document.querySelectorAll('button').forEach((button) => { button.disabled = true; });
  try {
    const result = await api('/api/account/billing/' + name, 'POST');
    if (result.url) {
      const url = new URL(result.url);
      if (url.protocol !== 'https:' || !['checkout.stripe.com', 'billing.stripe.com'].includes(url.hostname)) throw new Error('付款跳转地址无效。');
      window.location.assign(url.href);
    } else await refresh();
  } catch (error) { el('detail').textContent = error.message; }
  finally { document.querySelectorAll('button').forEach((button) => { button.disabled = false; }); }
}
el('subscribe').onclick = () => action('checkout');
el('portal').onclick = () => action('portal');
el('cancel').onclick = () => action('cancel');
el('refresh').onclick = refresh;
refresh();
