import gzip,json,unittest
from unittest.mock import patch
from host_diff import ROOT
from verify_runtime import inventory
from verify_validation import COMMANDS,validate_rust,validate_commands,verify

def sample():
    logs=json.loads(gzip.decompress((ROOT/'docs/stage2bd-runtime-test/validation-logs.json.gz').read_bytes()))['commands'];old=next(c for c in logs if c['argv'][:2]==['cargo','test']);i=inventory()
    return old['stdout']+old['stderr']+'\n'+''.join('test '+n+' ... ok\n'for n in i['ordinary'])+'test '+i['ignored_driver']+' ... ignored, explicit worker\n'+f"test result: ok. {len(i['ordinary'])} passed; 0 failed; 1 ignored;\n"
class ReceiptTests(unittest.TestCase):
    def test_rust_superset(self):self.assertEqual(len(validate_rust(sample())),728+len(inventory()['ordinary']))
    def test_missing_scenario(self):
        with self.assertRaises(AssertionError):validate_rust(sample().replace('test '+inventory()['ordinary'][0]+' ... ok\n',''))
    def test_duplicate_scenario(self):
        with self.assertRaises(AssertionError):validate_rust(sample()+'test '+inventory()['ordinary'][0]+' ... ok\n')
    def test_unexpected_skip(self):
        with self.assertRaises(AssertionError):validate_rust(sample()+'test unexpected ... ignored\n')
    def test_failure_summary(self):
        with self.assertRaises(AssertionError):validate_rust(sample().replace('0 failed;','1 failed;',1))
    def test_missing_commands(self):
        with self.assertRaises(AssertionError):validate_commands([])
    def test_failed_commands(self):
        with self.assertRaises(AssertionError):validate_commands([dict(argv=a,returncode=1,stdout='',stderr='')for a in COMMANDS])
    def test_receipt_log_tampering_is_rejected_before_rebuild(self):
        with patch('verify_validation.load',return_value=dict(logs_sha256='wrong')),patch('pathlib.Path.read_bytes',return_value=gzip.compress(json.dumps(dict(stage='2BE',target_execution='none',commands=[])).encode())):
            with self.assertRaisesRegex(AssertionError,'validation log identity'):verify()
