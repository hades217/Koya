"""Opt-in, read-only adapter for the existing subscription gateway ledger.

Only CRM_GATEWAY_DATA_FILE supplied by the operator is read. Never discover
credentials, copy activation material, or expose provider payloads to the UI.
"""
import json
from pathlib import Path


def read_gateway(path):
    if not path:
        return {'status': '未接入', 'subscriptions': [], 'requests': [], 'updated_at': None}
    try:
        file = Path(path)
        if not file.is_file() or file.stat().st_size > 50_000_000:
            raise ValueError('ledger unavailable')
        raw = json.loads(file.read_text())
        if not isinstance(raw.get('subscriptions'), dict) or not isinstance(raw.get('requests', {}), dict):
            raise ValueError('invalid ledger')
        subscriptions = []
        for key, record in raw['subscriptions'].items():
            if not isinstance(record, dict) or record.get('id') != key:
                raise ValueError('identity mismatch')
            credit_fields = ('includedCredits', 'topUpCredits', 'usedCredits', 'reservedCredits')
            credits = [record.get(field) for field in credit_fields]
            known = all(type(value) is int and value >= 0 for value in credits)
            subscriptions.append({
                'id': key, 'status': record.get('status', 'unavailable'),
                'plan': record.get('plan', 'unavailable'),
                'remaining': max(0, credits[0]+credits[1]-credits[2]-credits[3]) if known else None,
                'cancel_at_period_end': record.get('cancelAtPeriodEnd'),
                'period_ends_at': record.get('periodEndsAt'),
            })
        requests = []
        for key, record in raw.get('requests', {}).items():
            if not isinstance(record, dict):
                raise ValueError('invalid request')
            requests.append({'id': key, 'subscription_id': record.get('subscriptionId'),
                             'status': record.get('status', 'unavailable')})
        return {'status': '已接入（只读）', 'subscriptions': subscriptions,
                'requests': requests, 'updated_at': file.stat().st_mtime}
    except (OSError, ValueError, TypeError):
        return {'status': '暂不可用', 'subscriptions': [], 'requests': [], 'updated_at': None}
