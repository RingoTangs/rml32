import copy,gzip,unittest
from unittest.mock import patch
import verify_validation as v
from common import digest,encoded,load,HERE

def output():
 lines=['test '+n+' ... ok'for n in sorted(v.expected_tests())]+['test '+n+' ... ignored'for n in sorted(v.IGNORED)]
 lines+=['test src/boundary.rs - boundary::HostValue (line 1) - compile fail ... ok','test src/boundary.rs - boundary::HostValue (line 2) - compile fail ... ok','test src/lifecycle.rs - lifecycle::LifecycleLimits (line 1) - compile fail ... ok','test src/lifecycle.rs - lifecycle::LifecycleBudget (line 1) - compile fail ... ok','test src/lifecycle.rs - lifecycle::LifecycleBudget (line 2) - compile fail ... ok','test result: ok. 868 passed; 0 failed; 11 ignored; 0 measured; 0 filtered out;']
 return '\n'.join(lines)
class ValidationTests(unittest.TestCase):
 def test_complete_inventory(self):self.assertEqual(len(v.validate_rust(output())),863)
 def remove(self,name):
  with self.assertRaises(AssertionError):v.validate_rust(output().replace('test '+name+' ... ok',''))
 def test_missing_original_case(self):self.remove_test_case=next(n for n in v.expected_tests()if n not in load(HERE/'case-coverage.json')['new_test_names']);self.remove(self.remove_test_case)
 def test_missing_new_case(self):self.remove(load(HERE/'case-coverage.json')['new_test_names'][0])
 def test_duplicate_case(self):
  with self.assertRaises(AssertionError):v.validate_rust(output()+'\ntest '+next(iter(v.expected_tests()))+' ... ok')
 def test_missing_driver(self):
  with self.assertRaises(AssertionError):v.validate_rust(output().replace('test '+sorted(v.IGNORED)[0]+' ... ignored',''))
 def test_missing_doctest(self):
  with self.assertRaises(AssertionError):v.validate_rust(output().replace('test src/lifecycle.rs - lifecycle::LifecycleLimits (line 1) - compile fail ... ok',''))
 def test_missing_command(self):
  with self.assertRaises(AssertionError):v.validate_commands([])
 def test_failed_command(self):
  c=[dict(argv=a,returncode=0,stdout='',stderr='')for a in v.COMMANDS];c[1]['returncode']=101
  with self.assertRaises(AssertionError):v.validate_commands(c)
 def record(self):
  expected=dict(stage='2BK',artifact_sha256={'source':'actual'});raw=gzip.compress(encoded(dict(stage='2BK',target_execution='none',commands=[])),mtime=0);return dict(**expected,logs_sha256=digest(raw)),raw,expected
 def test_receipt_roundtrip(self):
  r,raw,expected=self.record()
  with patch.object(v,'build',return_value=expected):v.validate_record(r,raw)
 def test_receipt_log_hash(self):
  r,raw,expected=self.record();r['logs_sha256']='bad'
  with patch.object(v,'build',return_value=expected),self.assertRaises(AssertionError):v.validate_record(r,raw)
 def test_receipt_artifact_hash(self):
  r,raw,expected=self.record();r=copy.deepcopy(r);r['artifact_sha256']['source']='modified'
  with patch.object(v,'build',return_value=expected),self.assertRaises(AssertionError):v.validate_record(r,raw)
 def test_receipt_scope(self):
  r,raw,expected=self.record();r['stage']='2BJ'
  with patch.object(v,'build',return_value=expected),self.assertRaises(AssertionError):v.validate_record(r,raw)
if __name__=='__main__':unittest.main()
