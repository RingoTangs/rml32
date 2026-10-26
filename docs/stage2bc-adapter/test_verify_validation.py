import gzip,json,unittest
from verify_validation import ROOT,COMMANDS,inventory,validate_rust,validate_commands

def sample():
    logs=json.loads(gzip.decompress((ROOT/'docs/stage2bb-kernel/validation-logs.json.gz').read_bytes()))['commands'];old=next(c for c in logs if c['argv'][:2]==['cargo','test']);i=inventory()
    return old['stdout']+old['stderr']+'\n'+''.join('test '+n+' ... ok\n' for n in i['ordinary'])+'test '+i['ignored_driver']+' ... ignored, explicit driver\n'+f"test result: ok. {len(i['ordinary'])} passed; 0 failed; 1 ignored;\n"
class ReceiptTests(unittest.TestCase):
    def test_rust_superset(self):self.assertEqual(len(validate_rust(sample())),705)
    def test_missing_test(self):
        s=sample().replace('test '+inventory()['ordinary'][0]+' ... ok\n','')
        with self.assertRaises(AssertionError):validate_rust(s)
    def test_duplicate_test(self):
        with self.assertRaises(AssertionError):validate_rust(sample()+'test '+inventory()['ordinary'][0]+' ... ok\n')
    def test_unexpected_skip(self):
        with self.assertRaises(AssertionError):validate_rust(sample()+'test skipped ... ignored\n')
    def test_failure(self):
        with self.assertRaises(AssertionError):validate_rust(sample().replace('0 failed;','1 failed;',1))
    def test_missing_commands(self):
        with self.assertRaises(AssertionError):validate_commands([])
    def test_failed_command(self):
        with self.assertRaises(AssertionError):validate_commands([dict(argv=a,returncode=1,stdout='',stderr='') for a in COMMANDS])
    def test_reordered_commands(self):
        with self.assertRaises(AssertionError):validate_commands([dict(argv=a,returncode=0,stdout='',stderr='') for a in reversed(COMMANDS)])
