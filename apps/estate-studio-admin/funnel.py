"""Observed stage transitions; never infer unrecorded visits or causal explanations."""
from collections import Counter
from datetime import datetime, timezone, timedelta


def analyse(records, stages, days=0, pipeline='', timestamp=None):
    timestamp = timestamp or datetime.now(timezone.utc)
    parse = lambda value: datetime.fromisoformat(value.replace('Z', '+00:00'))
    pipelines = sorted({r.get('pipeline') or '默认销售' for r in records})
    records = [r for r in records if (not pipeline or (r.get('pipeline') or '默认销售') == pipeline)
               and (not days or parse(r['created_at']) >= timestamp-timedelta(days=days))]
    rows, reasons, stalled = [], [], []
    for stage in stages[:-2]:
        entered, advanced, lost, durations = set(), set(), set(), []
        for r in records:
            events = r.get('stage_history', [])
            visits = [i for i,e in enumerate(events) if e['to'] == stage]
            if visits: entered.add(r['id'])
            for i in visits:
                if i+1 >= len(events): continue
                e = events[i+1]
                durations.append(max(0, (parse(e['at'])-parse(events[i]['at'])).total_seconds()/86400))
                if e['to'] == '已流失': lost.add(r['id'])
                elif stages.index(e['to']) > stages.index(stage): advanced.add(r['id'])
        rows.append({'stage':stage, 'entered':len(entered), 'advanced':len(advanced), 'lost':len(lost),
                     'conversion':len(advanced)/len(entered) if entered else None,
                     'mean_exit_days':sum(durations)/len(durations) if durations else None})
    for r in records:
        events = r.get('stage_history', [])
        for e in events:
            if not e.get('from'): continue
            result = '流失' if e['to']=='已流失' else '转化' if stages.index(e['to'])>stages.index(e['from']) and e['from'] not in stages[-2:] else '回退 / 重开'
            reasons.append({'id':r['id'],'name':r['name'],'stage':e['from'],'to':e['to'], 'result':result,
                            'category':e.get('category') or '未分类','reason':e.get('reason') or '未记录', 'at':e['at']})
        if r['stage'] not in stages[-2:]:
            age = max(0,(timestamp-parse(events[-1]['at'])).total_seconds()/86400) if events else None
            if age is None or age >= 14 or r.get('blocker'):
                stalled.append({'id':r['id'],'name':r['name'],'stage':r['stage'],'days':age,
                                'blocker':r.get('blocker') or '尚未记录阻碍原因','next_action':r.get('next_action') or '尚未安排下一步'})
    summary = Counter((e['stage'],e['result'],e['category']) for e in reasons)
    won = sum(r['stage']=='已成交' for r in records); lost = sum(r['stage']=='已流失' for r in records)
    return {'pipelines':pipelines,'total':len(records),'won':won,'lost':lost,'open':len(records)-won-lost,
            'win_rate':won/(won+lost) if won+lost else None,
            'missing_history':sum(not r.get('stage_history') or r.get('history_incomplete',False) for r in records),
            'stages':rows,'reasons':sorted(reversed(reasons),key=lambda e:e['at'],reverse=True),
            'reason_groups':[dict(stage=k[0],result=k[1],category=k[2],count=v) for k,v in summary.most_common()],
            'stalled':sorted(stalled,key=lambda r:r['days'] if r['days'] is not None else -1,reverse=True)}
