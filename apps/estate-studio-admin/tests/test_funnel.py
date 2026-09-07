import unittest
from datetime import datetime, timezone
from funnel import analyse

class FunnelTest(unittest.TestCase):
    def test_cohort_legacy_stall_and_duration(self):
        stages=['需求确认','演示沟通','报价中','商务确认','已成交','已流失']
        records=[dict(id='old',name='legacy',stage='需求确认',created_at='2020-01-01T00:00:00Z'),
                 dict(id='new',name='deal',pipeline='续费',stage='演示沟通',created_at='2026-09-01T00:00:00Z',stage_history=[
                     dict(to='需求确认',at='2026-09-01T00:00:00Z'),dict(to='演示沟通',**{'from':'需求确认'},at='2026-09-03T00:00:00Z',reason='确认需求')])]
        stamp=datetime(2026,9,30,tzinfo=timezone.utc)
        d=analyse(records,stages,timestamp=stamp)
        self.assertEqual(d['missing_history'],1)
        self.assertEqual(d['stages'][0]['mean_exit_days'],2)
        self.assertEqual(d['stalled'][0]['days'],27)
        self.assertIsNone(d['stalled'][1]['days'])
        self.assertEqual(analyse(records,stages,30,'续费',stamp)['total'],1)
        self.assertEqual(analyse(records,stages,30,'默认销售',stamp)['total'],0)
