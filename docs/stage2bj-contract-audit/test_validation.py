import copy,gzip,unittest
from unittest.mock import patch
from common import digest,encoded,load,HERE
import verify_validation as v
from verify_boundary import verify as boundary

def rust_output():
 lines=['test '+n+' ... ok'for n in sorted(v.expected_tests())]
 lines+=['test '+n+' ... ignored'for n in sorted(v.IGNORED)]
 lines+=['test src/boundary.rs - boundary::HostValue (line 1) - compile fail ... ok','test src/boundary.rs - boundary::HostValue (line 2) - compile fail ... ok','test src/lifecycle.rs - lifecycle::LifecycleLimits (line 1) - compile fail ... ok','test src/lifecycle.rs - lifecycle::LifecycleBudget (line 1) - compile fail ... ok','test src/lifecycle.rs - lifecycle::LifecycleBudget (line 2) - compile fail ... ok']
 lines+=['test result: ok. 862 passed; 0 failed; 8 ignored; 0 measured; 0 filtered out;']
 return '\n'.join(lines)

class ValidationTests(unittest.TestCase):
 def test_exact_rust_scenario_inventory_passes(self):self.assertEqual(len(v.validate_rust(rust_output())),857)
 def remove_test(self,name):
  out=rust_output().replace('test '+name+' ... ok\n','')
  with self.assertRaises(AssertionError):v.validate_rust(out)
 def test_missing_old_rust_case_is_rejected(self):self.remove_test(next(n for n in v.expected_tests()if not n.startswith('bounded_')))
 def test_missing_new_rust_case_is_rejected(self):self.remove_test(load(HERE/'case-coverage.json')['rust_test_names'][0])
 def test_duplicate_rust_case_is_rejected(self):
  with self.assertRaises(AssertionError):v.validate_rust(rust_output()+'\ntest '+next(iter(v.expected_tests()))+' ... ok')
 def test_missing_ignored_worker_is_rejected(self):
  with self.assertRaises(AssertionError):v.validate_rust(rust_output().replace('test '+sorted(v.IGNORED)[0]+' ... ignored',''))
 def test_missing_doctest_is_rejected(self):
  with self.assertRaises(AssertionError):v.validate_rust(rust_output().replace('test src/lifecycle.rs - lifecycle::LifecycleLimits (line 1) - compile fail ... ok',''))
 def test_failed_rust_result_is_rejected(self):
  with self.assertRaises(AssertionError):v.validate_rust(rust_output().replace('862 passed; 0 failed','861 passed; 1 failed'))
 def test_missing_command_is_rejected(self):
  with self.assertRaises(AssertionError):v.validate_commands([])
 def test_failed_command_is_rejected(self):
  c=[dict(argv=a,returncode=0,stdout='',stderr='')for a in v.COMMANDS];c[-1]['returncode']=1
  with self.assertRaises(AssertionError):v.validate_commands(c)
 def record(self):
  raw=gzip.compress(encoded(dict(stage='2BJ',target_execution='none',commands=[])),mtime=0)
  expected=dict(stage='2BJ',status='pass',artifact_sha256={'some_source':'actual_digest'})
  return dict(**expected,logs_sha256=digest(raw)),raw,expected
 def test_receipt_roundtrip(self):
  r,raw,expected=self.record()
  with patch.object(v,'build',return_value=expected):v.validate_record(r,raw)
 def test_wrong_log_hash_is_rejected(self):
  r,raw,expected=self.record();r['logs_sha256']='0'*64
  with patch.object(v,'build',return_value=expected),self.assertRaises(AssertionError):v.validate_record(r,raw)
 def test_receipt_scope_change_is_rejected(self):
  r,raw,expected=self.record();r['stage']='2BI'
  with patch.object(v,'build',return_value=expected),self.assertRaises(AssertionError):v.validate_record(r,raw)
 def test_receipt_artifact_change_is_rejected(self):
  r,raw,expected=self.record();r=copy.deepcopy(r);r['artifact_sha256']['some_source']='different_digest'
  with patch.object(v,'build',return_value=expected),self.assertRaises(AssertionError):v.validate_record(r,raw)
 def test_frozen_evidence_and_old_test_bodies_are_preserved(self):
  r=boundary();self.assertEqual(r['byte_frozen_files'],1043);self.assertEqual(r['preserved_test_bodies_in_changed_files'],92)

if __name__=='__main__':unittest.main()
