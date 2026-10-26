import unittest
from verify_validation import frozen,FLAGS,validate_commands,COMMANDS
class ReceiptTests(unittest.TestCase):
    def test_frozen_replay(self):self.assertEqual(frozen()['stage'],'2AZ')
    def test_policy(self):
        self.assertTrue(FLAGS['model_changed']);self.assertFalse(FLAGS['runtime_changed'])
        self.assertFalse(FLAGS['implementation_ready']);self.assertFalse(FLAGS['new_native_analysis'])
    def test_missing_commands(self):
        with self.assertRaises(AssertionError):validate_commands([])
    def test_failed_command(self):
        c=[dict(argv=a,returncode=1,stdout='',stderr='') for a in COMMANDS]
        with self.assertRaises(AssertionError):validate_commands(c)
    def test_reordered_commands(self):
        c=[dict(argv=a,returncode=0,stdout='',stderr='') for a in reversed(COMMANDS)]
        with self.assertRaises(AssertionError):validate_commands(c)
