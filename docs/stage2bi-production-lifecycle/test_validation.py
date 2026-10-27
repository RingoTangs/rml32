import gzip,json,unittest
from unittest.mock import patch
from common import ROOT,HERE,load
from verify_validation import expected_tests,validate_rust,validate_commands,COMMANDS,IGNORED,verify
from model_replay import verify as model
def sample():
    lines=['test '+n+' ... ok'for n in sorted(expected_tests())]
    lines+=['test '+n+' ... ignored'for n in sorted(IGNORED)]
    docs=['src/boundary.rs - boundary::HostValue (line 1)','src/boundary.rs - boundary::HostValue (line 2)','src/lifecycle.rs - lifecycle::LifecycleLimits (line 1)','src/lifecycle.rs - lifecycle::LifecycleBudget (line 2)','src/lifecycle.rs - lifecycle::LifecycleBudget (line 3)']
    lines+=['test '+n+' - compile fail ... ok'for n in docs]
    return '\n'.join(lines+['test result: ok. 856 passed; 0 failed; 6 ignored;'])
class ValidationTests(unittest.TestCase):
    def test_complete_inventory_preserves_all_819_old_names(self):
        actual=set(validate_rust(sample()));old=set(load(ROOT/'docs/stage2bh-validation.json')['current_test_names']);self.assertEqual(len(old),819);self.assertEqual(len(actual-old),32);self.assertTrue(old<=actual)
    def test_missing_old_scenario_rejects(self):
        old=load(ROOT/'docs/stage2bh-validation.json')['current_test_names'][0]
        with self.assertRaises(AssertionError):validate_rust(sample().replace('test '+old+' ... ok',''))
    def test_missing_production_case_rejects(self):
        name=load(HERE/'case-coverage.json')['rust_test_names'][0]
        with self.assertRaises(AssertionError):validate_rust(sample().replace('test '+name+' ... ok',''))
    def test_duplicate_case_rejects(self):
        with self.assertRaises(AssertionError):validate_rust(sample()+'\ntest '+sorted(expected_tests())[0]+' ... ok')
    def test_unexpected_skip_rejects(self):
        with self.assertRaises(AssertionError):validate_rust(sample()+'\ntest unknown ... ignored')
    def test_failed_summary_rejects(self):
        with self.assertRaises(AssertionError):validate_rust(sample().replace('0 failed;','1 failed;'))
    def test_missing_public_api_doctest_rejects(self):
        with self.assertRaises(AssertionError):validate_rust(sample().replace('test src/lifecycle.rs - lifecycle::LifecycleLimits (line 1) - compile fail ... ok',''))
    def test_missing_or_failed_command_rejects(self):
        with self.assertRaises(AssertionError):validate_commands([])
        with self.assertRaises(AssertionError):validate_commands([dict(argv=c,returncode=1,stdout='',stderr='')for c in COMMANDS])
    def test_tampered_quota_model_identity_rejects(self):
        report=load(HERE/'model-replay.json');report['model']['model_cases']=1
        original=load
        with patch('model_replay.load',side_effect=lambda p:report if p==HERE/'model-replay.json'else original(p)):
            with self.assertRaises(AssertionError):model()
    def test_tampered_receipt_log_hash_rejects(self):
        raw=gzip.compress(json.dumps(dict(stage='2BI',commands=[])).encode())
        with patch('verify_validation.load',return_value=dict(logs_sha256='wrong')),patch('pathlib.Path.read_bytes',return_value=raw):
            with self.assertRaisesRegex(AssertionError,'validation logs'):verify()
