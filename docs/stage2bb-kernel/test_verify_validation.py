import gzip
import json
import unittest
from verify_validation import ROOT,FLAGS,COMMANDS,frozen,inventory,validate_rust,validate_commands

def sample_rust():
    logs=json.loads(gzip.decompress((ROOT/'docs/stage2ba-design/validation-logs.json.gz').read_bytes()))['commands']
    c=next(c for c in logs if c['argv'][:2]==['cargo','test'])
    i=inventory()
    return c['stdout']+c['stderr']+'\n'+''.join('test '+n+' ... ok\n' for n in i['ordinary'])+'test '+i['ignored_driver']+' ... ignored, explicit worker\n'+f"test result: ok. {len(i['ordinary'])} passed; 0 failed; 1 ignored;\n"

class ReceiptTests(unittest.TestCase):
    def test_historical_replay(self):self.assertEqual(frozen()['stage'],'2BA')
    def test_flags(self):
        self.assertTrue(FLAGS['test_only_rust_kernel_added']);self.assertTrue(FLAGS['rust_source_changed'])
        self.assertFalse(FLAGS['runtime_changed']);self.assertFalse(FLAGS['public_api_changed'])
        self.assertFalse(FLAGS['model_changed']);self.assertFalse(FLAGS['implementation_ready'])
    def test_missing_commands_rejected(self):
        with self.assertRaises(AssertionError):validate_commands([])
    def test_failed_commands_rejected(self):
        commands=[dict(argv=a,returncode=1,stdout='',stderr='failed') for a in COMMANDS]
        with self.assertRaises(AssertionError):validate_commands(commands)
    def test_command_reorder_rejected(self):
        commands=[dict(argv=a,returncode=0,stdout='',stderr='') for a in reversed(COMMANDS)]
        with self.assertRaises(AssertionError):validate_commands(commands)
    def test_rust_superset_accepted(self):self.assertEqual(len(validate_rust(sample_rust())),644+len(inventory()['ordinary']))
    def test_missing_kernel_scenario_rejected(self):
        s=sample_rust().replace('test '+inventory()['ordinary'][0]+' ... ok\n','')
        with self.assertRaises(AssertionError):validate_rust(s)
    def test_duplicate_kernel_scenario_rejected(self):
        s=sample_rust()+'test '+inventory()['ordinary'][0]+' ... ok\n'
        with self.assertRaises(AssertionError):validate_rust(s)
    def test_additional_ignored_scenario_rejected(self):
        with self.assertRaises(AssertionError):validate_rust(sample_rust()+'test unintended ... ignored\n')
    def test_failure_summary_rejected(self):
        with self.assertRaises(AssertionError):validate_rust(sample_rust().replace('0 failed;','1 failed;',1))
