import unittest
from commissions import percent, calculate

class CalculationTest(unittest.TestCase):
    def test_direct_and_pool_and_unknown(self):
        deal=dict(stage='已成交',agreed_amount_minor=1000000,currency='AUD',commission_snapshot=dict(mode='direct',lead_rate_bps=200,sales_rate_bps=500))
        result=calculate(deal)
        self.assertEqual((result['lead_minor'],result['sales_minor']),(20000,50000))
        deal['commission_snapshot']=dict(mode='pool',pool_rate_bps=700,lead_share_bps=3000)
        result=calculate(deal)
        self.assertEqual((result['lead_minor'],result['sales_minor'],result['total_minor']),(21000,49000,70000))
        deal['agreed_amount_minor']=1
        deal['commission_snapshot']=dict(mode='pool',pool_rate_bps=10000,lead_share_bps=5000)
        result=calculate(deal)
        self.assertEqual(result['lead_minor']+result['sales_minor'],1)
        deal['commission_snapshot']=dict(mode='direct',lead_rate_bps=5000,sales_rate_bps=5000)
        self.assertEqual(calculate(deal)['total_minor'],1)
        deal['agreed_amount_minor']=None
        self.assertIsNone(calculate(deal)['total_minor'])
        deal['stage']='已流失'
        self.assertEqual(calculate(deal)['status'],'已流失 · 不计提')
        self.assertEqual(percent('2.35'),235)
        for invalid in ('NaN','Infinity','-1','100.01','0.001',''):
            with self.assertRaises(ValueError):percent(invalid)
