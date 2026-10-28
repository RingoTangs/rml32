import copy,gzip,json,unittest
from common import HERE,load
from matrix import batches,mappings,closes,profiles
from batch_diff import compare,rows
from batch_oracle import BatchOracle,m
from combination_diff import check,check_faults
class AuditTests(unittest.TestCase):
 @classmethod
 def setUpClass(cls):
  cls.trace=next(t for t in batches()if t['id']=='B162-independent-00-commit');cls.observations=[]
  for row in rows(HERE/'batch-observations.jsonl.gz'):
   if row['id']==cls.trace['id']:cls.observations.append(row)
  cls.close=next(t for t in closes()if t['id'].startswith('C162-'))
  cls.close_row=next(json.loads(line)for line in gzip.decompress((HERE/'private-observations.jsonl.gz').read_bytes()).splitlines()if json.loads(line)['id']==cls.close['id'])
  cls.mapping=next(t for t in mappings()if t['id'].startswith('M162-'))
  cls.mapping_row=next(json.loads(line)for line in gzip.decompress((HERE/'private-observations.jsonl.gz').read_bytes()).splitlines()if json.loads(line)['id']==cls.mapping['id'])
 def oracle(self,ordinary=6,owners=8):
  q=dict(profiles()[-1],max_ordinary_refs_per_object=ordinary,max_owners_per_header=owners);w=BatchOracle(q);w.event(dict(op='setup'));return w
 def tamper(self,change):
  r=copy.deepcopy(self.observations);change(r)
  with self.assertRaises(AssertionError):compare([self.trace],r)
 def test_exact_bounded_domain(self):
  self.assertEqual(len(profiles()),163);self.assertEqual(sum(1 for _ in batches()),84108);self.assertEqual(sum(1 for _ in mappings()),3912);self.assertEqual(sum(1 for _ in closes()),7824)
 def test_protocol_has_no_expected_data(self):
  for t in (next(batches()),next(mappings()),next(closes())):
   self.assertFalse({'expected','oracle','before','after','projection'}&t.keys())
 def test_actual_batch_control(self):self.assertEqual(compare([self.trace],self.observations)['steps'],3)
 def test_missing_checkpoint(self):self.tamper(lambda r:r.pop(0))
 def test_reordered_checkpoint(self):
  def f(r):r[0],r[1]=r[1],r[0]
  self.tamper(f)
 def test_extra_checkpoint(self):self.tamper(lambda r:r.append(r[-1]))
 def test_wrong_owner_count(self):self.tamper(lambda r:r[-1]['after'].update(ordinary=900))
 def test_partial_publication_identity(self):self.tamper(lambda r:r[-1]['after']['slots'].update(d1=['undefined']))
 def test_stale_watch_rejects_without_publication(self):
  w=self.oracle();w.event(dict(op='prepare',layout='independent'));w.event(dict(op='interfere',target='s0'));before=w.projection()
  with self.assertRaises(m.Rejected):w.event(dict(op='commit'))
  self.assertEqual(before,w.projection())
 def test_peak_counts_all_destinations_before_release(self):
  w=self.oracle(ordinary=3);w.event(dict(op='prepare',layout='fanout'));before=w.projection()
  with self.assertRaisesRegex(m.Rejected,'quota'):w.event(dict(op='commit'))
  self.assertEqual(before,w.projection())
 def test_raw_moves_need_no_header_owner_increment(self):
  w=self.oracle(owners=1);w.event(dict(op='prepare',layout='moves'));w.event(dict(op='commit'));self.assertEqual(w.projection()['header_owners'],1);self.assertEqual(w.projection()['slots']['s1'],['undefined'])
 def test_conflicting_duplicate_destination_is_rejected(self):
  w=self.oracle();before=w.projection()
  with self.assertRaises(m.Rejected):w.event(dict(op='prepare',layout='same_conflict'))
  self.assertEqual(before,w.projection())
 def test_maintenance_budget_corruption(self):
  row=copy.deepcopy(self.close_row);next(p for p in row['checkpoints']if 'drained'in p)['drained']=9
  with self.assertRaises(AssertionError):check(self.close,row,True)
 def test_premature_control_release(self):
  row=copy.deepcopy(self.close_row);row['checkpoints'][0]['observation']=dict(registry_alive=False,context_alive=False)
  with self.assertRaises(AssertionError):check(self.close,row,True)
 def test_final_control_leak(self):
  row=copy.deepcopy(self.close_row);row['final']['context_alive']=True
  with self.assertRaises(AssertionError):check(self.close,row,True)
 def test_failed_injection_cannot_be_reported_as_success(self):
  t=dict(self.mapping,fail_after=0)
  with self.assertRaises(AssertionError):check_faults([(t,1)],[self.mapping_row])
if __name__=='__main__':unittest.main()
