import copy,gzip,json,unittest
from common import HERE,load
from matrix import traces
from contract_diff import compare,inputs
from maintenance_oracle import MaintenanceOracle
from baseline_proof import verify as baseline_proof

class ContractTests(unittest.TestCase):
 @classmethod
 def setUpClass(cls):
  cls.cases=traces();cls.rows=[json.loads(x)for x in gzip.decompress((HERE/'contract-observations.jsonl.gz').read_bytes()).splitlines()]
  cls.by_id={t['id']:t for t in cls.cases}
  cls.md=next(r for r in cls.rows if cls.by_id[r['id']]['family']=='F13'and cls.by_id[r['id']]['actions'][r['step']]['op']=='maintain'and r['progress']and r['progress']['metadata_examined']==1)
 def corrupt(self,index,change):
  rows=list(self.rows);rows[index]=copy.deepcopy(rows[index]);change(rows[index])
  with self.assertRaises(AssertionError):compare(self.cases,rows)
 def metadata_corrupt(self,change):
  r=copy.deepcopy(self.md);a=self.by_id[r['id']]['actions'][r['step']];change(r)
  with self.assertRaises(AssertionError):MaintenanceOracle().check(a,r['before'],r['after'],r['progress'])
 def test_matrix_and_actions_only_protocol(self):
  self.assertEqual(len(self.cases),2592);self.assertEqual(len({json.dumps(t['limits'],sort_keys=True)for t in self.cases}),162)
  self.assertEqual(len({t['family']for t in self.cases}),16)
  def walk(v):
   if isinstance(v,dict):
    self.assertFalse({'expected','oracle','projection','progress','before','after'}&v.keys())
    for x in v.values():walk(x)
   elif isinstance(v,list):
    for x in v:walk(x)
  walk([json.loads(x)for x in inputs().splitlines()])
 def test_actual_observations_match_frozen_model(self):
  s=compare(self.cases,self.rows);self.assertEqual((s['steps'],s['semantic_checkpoints'],s['metadata_checkpoints']),(10944,10008,1518));self.assertEqual(s['unclassified_differences'],0)
 def test_missing_checkpoint_is_rejected(self):
  with self.assertRaises(AssertionError):compare(self.cases,self.rows[1:])
 def test_reordered_checkpoint_is_rejected(self):
  rows=list(self.rows);rows[0],rows[1]=rows[1],rows[0]
  with self.assertRaises(AssertionError):compare(self.cases,rows)
 def test_extra_checkpoint_is_rejected(self):
  with self.assertRaises(AssertionError):compare(self.cases,self.rows+[self.rows[-1]])
 def test_wrong_result_kind_is_rejected(self):self.corrupt(0,lambda r:r.update(error_kind='InvalidState'))
 def test_wrong_owner_count_is_rejected(self):
  i=next(i for i,r in enumerate(self.rows)if r['after']['semantic']['objects'])
  self.corrupt(i,lambda r:r['after']['semantic']['objects'][next(iter(r['after']['semantic']['objects']))].update(ordinary=99))
 def test_wrong_lifecycle_state_is_rejected(self):
  i=next(i for i,r in enumerate(self.rows)if r['after']['semantic']['objects'])
  self.corrupt(i,lambda r:r['after']['semantic']['objects'][next(iter(r['after']['semantic']['objects']))].update(state='freed'))
 def test_wrong_lifo_queue_is_rejected(self):
  i=next(i for i,r in enumerate(self.rows)if len(r['after']['semantic']['queue'])==2)
  self.corrupt(i,lambda r:r['after']['semantic']['queue'].reverse())
 def test_discontinuous_before_snapshot_is_rejected(self):
  i=next(i for i,r in enumerate(self.rows)if r['step']>0)
  self.corrupt(i,lambda r:r['before']['raw'].update(epoch=999999))
 def test_wrong_logical_identity_is_rejected(self):
  i=next(i for i,r in enumerate(self.rows)if any(v[0]=='object'for v in r['after']['semantic']['slots'].values()))
  def change(r):
   slots=r['after']['semantic']['slots'];key=next(k for k,v in slots.items()if v[0]=='object');slots[key]=['object','different_object']
  self.corrupt(i,change)
 def test_rejection_mutation_is_rejected(self):
  i=next(i for i,r in enumerate(self.rows)if not r['accepted'])
  self.corrupt(i,lambda r:r['after']['semantic'].update(pending_roots=17))
 def test_maintenance_count_is_independently_checked(self):self.metadata_corrupt(lambda r:r['progress'].update(metadata_examined=99))
 def test_maintenance_stop_is_independently_checked(self):self.metadata_corrupt(lambda r:r['progress'].update(stop='WorkspaceLimit'))
 def test_maintenance_epoch_is_independently_checked(self):self.metadata_corrupt(lambda r:r['after']['raw'].update(epoch=0))
 def test_zero_budget_cannot_advance_cursor(self):
  r=copy.deepcopy(next(r for r in self.rows if r['progress']and r['progress'].get('metadata_examined')==0 and self.by_id[r['id']]['actions'][r['step']].get('budget')==dict(host_roots=0,objects=0,metadata_entries=0)))
  a=self.by_id[r['id']]['actions'][r['step']];r['after']['raw']['scan']['phase']=99
  with self.assertRaises(AssertionError):MaintenanceOracle().check_semantic_phase(a,r['before'],r['after'],r['progress'],r['progress'])
 def test_both_findings_have_actual_baseline_reproductions(self):
  p=baseline_proof();self.assertEqual(p['findings_reproduced'],['D08-R01','D08-R02']);self.assertEqual(p['expected_baseline_failure_commands'],1)
 def test_pre_fix_publication_disagrees_with_frozen_model(self):
  f=load(HERE/'counterexamples.json')['findings'][0]
  with self.assertRaisesRegex(AssertionError,'result expected=InvalidObject'):
   compare([f['trace']],f['pre_fix_observations'])

if __name__=='__main__':unittest.main()
