"""Only temporary/in-memory receipt corruption; no native/runtime mutations."""
import copy
from pathlib import Path
import tempfile
import unittest
import verify_validation as v


class ReceiptTests(unittest.TestCase):
    def setUp(self):
        self.names={f'case_{i}' for i in range(558)}
        rust='\n'.join(f'test {n} ... ok' for n in sorted(self.names))+'\ntest result: ok. 558 passed; 0 failed;\n'
        self.commands=[dict(argv=c,returncode=0,stdout='',stderr='') for c in v.COMMANDS]
        for c in self.commands:
            if c['argv'][:2]==['cargo','test']:c['stdout']=rust
            if 'unittest' in c['argv']:c['stderr']='Ran 86 tests\nOK\n'

    def check(self):v.validate_commands(self.commands,self.names,86)

    def test_valid_receipt_inventory(self):self.check()

    def test_missing_check_cannot_be_frozen(self):
        self.commands.pop()
        with self.assertRaisesRegex(ValueError,'command inventory'):self.check()

    def test_failure_cannot_be_frozen_as_success(self):
        self.commands[0]['returncode']=1
        with self.assertRaisesRegex(ValueError,'failed check'):self.check()

    def test_rust_test_names_must_match_baseline(self):
        self.commands[4]['stdout']=self.commands[4]['stdout'].replace('case_0','forged_0')
        with self.assertRaisesRegex(ValueError,'Rust test inventory'):self.check()

    def test_release_count_cannot_shrink(self):
        self.commands[5]['stdout']=self.commands[5]['stdout'].replace('558 passed','557 passed')
        with self.assertRaisesRegex(ValueError,'Rust results'):self.check()

    def test_python_failed_text_is_rejected(self):
        self.commands[3]['stderr']='Ran 86 tests\nFAILED\n'
        with self.assertRaisesRegex(ValueError,'Python results'):self.check()

    def test_delivery_cannot_omit_model(self):
        with self.assertRaisesRegex(ValueError,'delivery inventory'):v.check_inventory({'a':'hash'},{'a','model.py'})

    def test_original_entry_identity_cannot_be_rewritten(self):
        b=dict(before_entry_sha256={p:'before' for p in v.ENTRY});prior=dict(artifact_sha256=copy.deepcopy(b['before_entry_sha256']))
        v.check_entries(b,prior);b['before_entry_sha256']['README.md']='new'
        with self.assertRaisesRegex(ValueError,'entry baseline'):v.check_entries(b,prior)

    def test_historical_file_tampering_is_rejected(self):
        with tempfile.TemporaryDirectory() as name:
            root=Path(name);p=root/'old.json';p.write_text('before');hashes={'old.json':v.digest(p.read_bytes())}
            v.check_hashes(root,hashes);p.write_text('after')
            with self.assertRaisesRegex(ValueError,'protected input changed'):v.check_hashes(root,hashes)

    def test_missing_original_test_inventory_is_rejected(self):
        self.names.pop()
        with self.assertRaisesRegex(ValueError,'original test inventory'):self.check()


if __name__=='__main__':unittest.main()
