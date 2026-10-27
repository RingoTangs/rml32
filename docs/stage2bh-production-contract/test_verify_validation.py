import gzip,json,unittest
from unittest.mock import patch
from common import ROOT
from verify_validation import validate_rust,validate_commands,COMMANDS,verify

def sample():
    logs=json.loads(gzip.decompress((ROOT/'docs/stage2bg-host-function-test/validation-logs.json.gz').read_bytes()))['commands'];r=next(c for c in logs if c['argv'][:2]==['cargo','test']);return r['stdout']+r['stderr']
class ReceiptTests(unittest.TestCase):
    def test_exact_rust_inventory(self):self.assertEqual(len(validate_rust(sample())),819)
    def test_missing_rust_scenario(self):
        out=sample();name=validate_rust(out)[0]
        with self.assertRaises(AssertionError):validate_rust(out.replace('test '+name+' ... ok\n',''))
    def test_duplicate_rust_scenario(self):
        out=sample();name=validate_rust(out)[0]
        with self.assertRaises(AssertionError):validate_rust(out+'test '+name+' ... ok\n')
    def test_unexpected_skip(self):
        with self.assertRaises(AssertionError):validate_rust(sample()+'test unexpected ... ignored\n')
    def test_failure_summary(self):
        with self.assertRaises(AssertionError):validate_rust(sample().replace('0 failed;','1 failed;',1))
    def test_missing_commands(self):
        with self.assertRaises(AssertionError):validate_commands([])
    def test_failed_commands(self):
        with self.assertRaises(AssertionError):validate_commands([dict(argv=a,returncode=1,stdout='',stderr='')for a in COMMANDS])
    def test_tampered_receipt_logs(self):
        raw=gzip.compress(json.dumps(dict(stage='2BH',target_execution='none',commands=[])).encode())
        with patch('verify_validation.load',return_value=dict(logs_sha256='bad')),patch('pathlib.Path.read_bytes',return_value=raw):
            with self.assertRaisesRegex(AssertionError,'validation logs'):verify()
