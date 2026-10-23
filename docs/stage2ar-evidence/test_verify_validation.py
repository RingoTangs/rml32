"""Receipt failure tests mutate temporary/in-memory state only."""
import copy
from pathlib import Path
import tempfile
import unittest
import verify_validation as v


class ReceiptTests(unittest.TestCase):
    def setUp(self):
        self.names={f'case_{i}' for i in range(558)}
        text='\n'.join(f'test {n} ... ok' for n in sorted(self.names))+'\ntest result: ok. 558 passed; 0 failed;\n'
        self.commands=[dict(argv=c,returncode=0,stdout='',stderr='') for c in v.COMMANDS]
        for c in self.commands:
            if c['argv'][:2]==['cargo','test']:c['stdout']=text
            if 'unittest' in c['argv']:c['stderr']='Ran 37 tests\n\nOK\n'

    def check(self):v.validate_commands(self.commands,self.names,37)

    def test_valid_receipt_command_results(self):self.check()

    def test_missing_command_is_rejected(self):
        self.commands.pop()
        with self.assertRaisesRegex(ValueError,'command inventory'):self.check()

    def test_failed_check_cannot_be_frozen(self):
        self.commands[0]['returncode']=1
        with self.assertRaisesRegex(ValueError,'failed check'):self.check()

    def test_rust_names_must_match_previous_baseline(self):
        self.commands[3]['stdout']=self.commands[3]['stdout'].replace('case_0','invented_0')
        with self.assertRaisesRegex(ValueError,'Rust test inventory'):self.check()

    def test_release_count_must_match(self):
        self.commands[4]['stdout']=self.commands[4]['stdout'].replace('558 passed','557 passed')
        with self.assertRaisesRegex(ValueError,'Rust results'):self.check()

    def test_python_failed_text_is_rejected(self):
        self.commands[2]['stderr']='Ran 37 tests\nFAILED\n'
        with self.assertRaisesRegex(ValueError,'Python results'):self.check()

    def test_delivery_cannot_omit_file(self):
        with self.assertRaisesRegex(ValueError,'delivery inventory'):v.check_inventory({'a':'hash'},{'a','b'})

    def test_prior_entry_identity_cannot_be_rewritten(self):
        baseline=dict(before_entry_sha256={p:'before' for p in v.ENTRY});prior=dict(artifact_sha256=copy.deepcopy(baseline['before_entry_sha256']))
        v.check_entry_baseline(baseline,prior);baseline['before_entry_sha256']['README.md']='new'
        with self.assertRaisesRegex(ValueError,'entry baseline'):v.check_entry_baseline(baseline,prior)

    def test_historical_file_tampering_is_rejected(self):
        with tempfile.TemporaryDirectory() as name:
            root=Path(name);p=root/'old.json';p.write_text('frozen');hashes={'old.json':v.digest(p.read_bytes())}
            v.check_hashes(root,hashes);p.write_text('changed')
            with self.assertRaisesRegex(ValueError,'protected input changed'):v.check_hashes(root,hashes)

    def test_missing_baseline_test_inventory_is_rejected(self):
        self.names.pop()
        with self.assertRaisesRegex(ValueError,'original test inventory'):self.check()


if __name__=='__main__':unittest.main()
