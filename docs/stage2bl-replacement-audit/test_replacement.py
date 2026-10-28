import copy,unittest
from common import HERE
from matrix import families,traces,HIGH
from replacement_oracle import Reference,Rejected
from replacement_diff import compare,observations

def fixture(name,q=None):
 f=next(f for f in families()if f['name']==name);r=Reference(q or HIGH)
 for a in f['setup']:r.apply(a)
 r.apply(dict(op='prepare',writes=f['writes'],raw=f.get('raw',False)));return r
class ReplacementTests(unittest.TestCase):
 @classmethod
 def setUpClass(cls):
  cls.t=next(t for t in traces()if t['id']=='object_swap-Q163-W00-commit');cls.rows=[]
  for row in observations(HERE/'core-observations.jsonl.gz'):
   if row['id']==cls.t['id']:cls.rows.append(row)
   elif cls.rows:break
 def tamper(self,f):
  rows=copy.deepcopy(self.rows);f(rows)
  with self.assertRaises(AssertionError):compare([self.t],rows)
 def test_exact_domain(self):self.assertEqual(len(families()),16);self.assertEqual(sum(1 for _ in traces()),238736)
 def test_actions_only_input(self):
  def walk(v):
   if isinstance(v,dict):
    self.assertFalse({'expected','oracle','projection','before','after'}&v.keys())
    for x in v.values():walk(x)
   elif isinstance(v,list):
    for x in v:walk(x)
  for t in traces(True):walk(t)
 def test_ordinary_boundary_before_release(self):
  for cap in (3,4,5):
   r=fixture('object_swap',dict(HIGH,max_ordinary_refs_per_object=cap));before=r.projection()
   if cap==3:
    with self.assertRaises(Rejected):r.apply(dict(op='commit'))
    self.assertEqual(r.projection(),before)
   else:r.apply(dict(op='commit'))
 def test_header_boundary_before_release(self):
  for cap in (3,4,5):
   r=fixture('array_same_header',dict(HIGH,max_owners_per_header=cap))
   if cap==3:
    with self.assertRaises(Rejected):r.apply(dict(op='commit'))
   else:r.apply(dict(op='commit'));self.assertFalse(r.heads['h0']['retired'])
 def test_object_swap_preserves_distinct_identity(self):
  r=fixture('object_swap');r.apply(dict(op='commit'));self.assertEqual(r.slots['d0'],('object','o0'));self.assertEqual(r.slots['d1'],('object','o1'))
 def test_same_header_is_not_retired(self):
  r=fixture('array_same_header');r.apply(dict(op='commit'));self.assertFalse(r.heads['h0']['retired']);self.assertEqual(r.projection()['headers']['h0']['owners'],2)
 def test_last_owner_clears_members(self):
  r=fixture('mapping_last_owner');r.apply(dict(op='commit'));self.assertEqual(r.projection()['slots']['m:h0'],['undefined']);self.assertEqual(r.projection()['objects']['o1']['ordinary'],1)
 def test_external_alias_delays_retirement(self):
  r=fixture('array_external_alias');r.apply(dict(op='commit'));self.assertFalse(r.heads['h0']['retired']);r.apply(dict(op='drop',slot='alias'));self.assertTrue(r.heads['h0']['retired'])
 def test_view_is_not_a_semantic_owner(self):
  r=fixture('array_last_owner');before=r.projection();r.apply(dict(op='view',header='h0'));self.assertEqual(r.projection(),before);r.apply(dict(op='commit'));self.assertTrue(r.heads['h0']['retired']);self.assertIn('h0',r.views)
 def test_shared_dag_releases_child_once(self):
  r=fixture('shared_dag');r.apply(dict(op='commit'));self.assertTrue(r.heads['child']['retired']);self.assertEqual(r.projection()['objects']['o1']['ordinary'],1)
 def test_function_creator_not_duplicated_and_released(self):
  r=fixture('function_same_header');self.assertEqual(r.projection()['objects']['o2']['ordinary'],1);r.apply(dict(op='commit'));self.assertEqual(r.projection()['objects']['o2']['ordinary'],1)
  r=fixture('function_last_owner');r.apply(dict(op='commit'));self.assertEqual(r.projection()['objects']['o2']['ordinary'],0)
 def test_new_member_value_is_cleared_by_final_retirement(self):
  r=fixture('function_retired_member');r.apply(dict(op='commit'));self.assertEqual(r.projection()['slots']['m:h0'],['undefined']);self.assertEqual(r.projection()['objects']['o0']['ordinary'],1)
 def test_stale_source_rejection_is_atomic(self):
  r=fixture('object_swap');r.apply(dict(op='put',dest='s0',value=['int',1]));before=r.projection()
  with self.assertRaises(Rejected):r.apply(dict(op='commit'))
  self.assertEqual(r.projection(),before)
 def test_occupied_raw_destination_is_rejected(self):
  r=fixture('raw_occupied');before=r.projection()
  with self.assertRaises(Rejected):r.apply(dict(op='commit'))
  self.assertEqual(r.projection(),before)
 def test_borrow_blocks_final_member_cleanup(self):
  r=fixture('array_last_owner');before=r.projection()
  with self.assertRaises(Rejected):r.apply(dict(op='commit',borrow='m:h0'))
  self.assertEqual(r.projection(),before)
 def test_borrow_of_non_retiring_member_is_allowed(self):
  r=fixture('array_external_alias');r.apply(dict(op='commit',borrow='m:h0'));self.assertFalse(r.heads['h0']['retired'])
 def test_actual_checkpoint_control(self):self.assertEqual(compare([self.t],self.rows)[0]['rows'],3)
 def test_missing_checkpoint_is_rejected(self):self.tamper(lambda r:r.pop(1))
 def test_corrupted_owner_count_is_rejected(self):self.tamper(lambda r:r[-1]['after']['objects']['o0'].update(ordinary=800))
 def test_wrong_destination_identity_is_rejected(self):self.tamper(lambda r:r[-1]['after']['slots'].update(d0=['object','o1']))
if __name__=='__main__':unittest.main()
