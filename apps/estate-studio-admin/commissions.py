"""Commission estimates in minor currency units; never execute payouts."""
from decimal import Decimal, InvalidOperation, ROUND_HALF_UP

CHANNELS = ['Marketing', '线下地推', '运营', '转介绍', 'Sales 自拓', '其他']
ROLES = ['获客', 'Sales', '获客与 Sales', '转介绍伙伴']


def percent(value):
    try:
        number = Decimal(str(value))
        if not number.is_finite() or not 0 <= number <= 100 or number.as_tuple().exponent < -2:
            raise ValueError()
        return int(number*100)
    except (InvalidOperation, ValueError, TypeError):
        raise ValueError('提成比例需为 0–100 的数字，最多两位小数；未知比例请先不配置规则')


def calculate(opportunity):
    snapshot = opportunity.get('commission_snapshot')
    closed = opportunity['stage'] == '已成交'
    base = opportunity.get('agreed_amount_minor' if closed else 'expected_amount_minor')
    result = dict(base_minor=base, lead_minor=None, sales_minor=None, total_minor=None,
                  basis='成交约定金额' if closed else '预计成交金额', status='未配置', currency=opportunity.get('currency', 'AUD'))
    if opportunity['stage'] == '已流失':
        result.update(status='已流失 · 不计提'); return result
    if not snapshot: return result
    result['status'] = '金额未知' if base is None else '成交佣金测算 · 待结算确认' if closed else '预测佣金'
    if base is None: return result
    round_minor = lambda n: int(n.quantize(Decimal('1'), rounding=ROUND_HALF_UP))
    if snapshot['mode'] == 'direct':
        lead = round_minor(Decimal(base)*snapshot['lead_rate_bps']/10000)
        total = round_minor(Decimal(base)*(snapshot['lead_rate_bps']+snapshot['sales_rate_bps'])/10000)
        sales = total-lead
    else:
        total = round_minor(Decimal(base)*snapshot['pool_rate_bps']/10000)
        lead = round_minor(Decimal(total)*snapshot['lead_share_bps']/10000)
        sales = total-lead
    result.update(lead_minor=lead, sales_minor=sales, total_minor=lead+sales)
    return result
