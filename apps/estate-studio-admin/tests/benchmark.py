"""Bounded local query check; disposable DB, no customer data, no network."""
import json
from pathlib import Path
import statistics
import sys
import tempfile
import time
sys.path.insert(0,str(Path(__file__).resolve().parents[1]))
from server import CRM, now

with tempfile.TemporaryDirectory() as directory:
    crm=CRM(Path(directory)/'benchmark.sqlite3',True)
    stamp=now()
    with crm.db:
        crm.db.executemany('INSERT INTO entities VALUES(?,?,?,1,?,?)',(
            (f'contact-{i}','contacts',json.dumps({'name':f'Benchmark person {i}','email':f'{i}@example.com','priority':'中'}),stamp,stamp)
            for i in range(100000)))
        crm.db.executemany('INSERT INTO entities VALUES(?,?,?,1,?,?)',(
            (f'inquiry-{i}','inquiries',json.dumps({'name':f'Benchmark person {i}','email':f'{i}@example.com','contact_id':f'contact-{i}','status':'新咨询','received_at':stamp}),stamp,stamp)
            for i in range(100000)))
    results={}
    for kind,query in [('contacts',{}),('inquiries',{'status':'新咨询'}),('contacts',{'q':'Benchmark person 99999'})]:
        timings=[]
        for _ in range(7):
            start=time.perf_counter();crm.list_items(kind,query);timings.append((time.perf_counter()-start)*1000)
        label=kind+(' search' if query.get('q') else ' list')
        results[label]={'max_ms':round(max(timings),1),'median_ms':round(statistics.median(timings),1)}
        assert max(timings)<2000,(label,timings)
    print(json.dumps({'fixture':{'contacts':100000,'inquiries':100000},'local_sqlite_queries':results},indent=2))
    crm.db.close()
