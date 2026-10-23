"""Negative receipt tests use only temporary fixtures and in-memory logs."""
import copy
import hashlib
import tempfile
from pathlib import Path
import unittest

import verify_validation as receipt
from audit_rules import verify_hashes


class ReceiptTests(unittest.TestCase):
    def setUp(self):
        self.names = {f'case_{i}' for i in range(558)}
        rust = '\n'.join(f'test {name} ... ok' for name in sorted(self.names))
        rust += '\ntest result: ok. 558 passed; 0 failed;\n'
        self.commands = [dict(argv=argv, returncode=0, stdout='', stderr='') for argv in receipt.COMMANDS]
        for command in self.commands:
            if command['argv'][:2] == ['cargo', 'test']:
                command['stdout'] = rust
            if 'unittest' in command['argv']:
                command['stderr'] = f'Ran {receipt.PYTHON_TESTS} tests\n\nOK\n'

    def check(self):
        receipt.validate_commands(self.commands, self.names)

    def test_valid_command_log(self):
        self.check()

    def test_missing_command(self):
        self.commands.pop()
        with self.assertRaisesRegex(ValueError, 'command inventory'):
            self.check()

    def test_failed_command_cannot_be_frozen(self):
        self.commands[0]['returncode'] = 1
        with self.assertRaisesRegex(ValueError, 'failed check'):
            self.check()

    def test_count_without_original_test_names_is_rejected(self):
        self.commands[3]['stdout'] = self.commands[3]['stdout'].replace('case_0', 'forged_0')
        with self.assertRaisesRegex(ValueError, 'Rust test inventory'):
            self.check()

    def test_release_count_must_match(self):
        self.commands[4]['stdout'] = self.commands[4]['stdout'].replace('558 passed', '557 passed')
        with self.assertRaisesRegex(ValueError, 'Rust result count'):
            self.check()

    def test_python_failure_text_is_rejected(self):
        self.commands[2]['stderr'] = 'Ran 55 tests\nFAILED\n'
        with self.assertRaisesRegex(ValueError, 'Python results'):
            self.check()

    def test_delivery_omission_is_rejected(self):
        with self.assertRaisesRegex(ValueError, 'delivery inventory'):
            receipt.check_inventory({'a': 'hash'}, {'a', 'b'})

    def test_entry_baseline_cannot_rewrite_previous_receipt(self):
        baseline = dict(before_entry_sha256={p: 'before' for p in receipt.ENTRY})
        previous = dict(artifact_sha256=copy.deepcopy(baseline['before_entry_sha256']))
        receipt.check_entries(baseline, previous)
        baseline['before_entry_sha256']['README.md'] = 'rewritten'
        with self.assertRaisesRegex(ValueError, 'entry baseline'):
            receipt.check_entries(baseline, previous)

    def test_historical_hash_corruption_is_rejected(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            path = root/'old.json'
            path.write_text('historical')
            frozen = {'old.json': hashlib.sha256(path.read_bytes()).hexdigest()}
            verify_hashes(root, frozen)
            path.write_text('corrupted')
            with self.assertRaisesRegex(ValueError, 'hash mismatch'):
                verify_hashes(root, frozen)

    def test_new_corpus_error_cannot_be_accepted(self):
        corpus = dict(status='pass', originals_unchanged=36, known_errors=['dba/pack_data/etc.pak/etc'],
                      checked=dict(packages=12, members=9527, programs=7159, inheritance=9933), target_execution='none')
        receipt.check_corpus(corpus)
        corpus['known_errors'].append('new failure')
        with self.assertRaisesRegex(ValueError, 'corpus boundary'):
            receipt.check_corpus(corpus)


if __name__ == '__main__':
    unittest.main()
