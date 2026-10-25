"""Negative checks for frozen history, scenario identity and receipt authority."""
from copy import deepcopy
import gzip
import json
from pathlib import Path
import unittest
from unittest.mock import patch
import verify_validation as verifier


def commands_fixture():
    frozen = verifier.load(verifier.ROOT / 'docs/stage2aw-evidence/validation-logs.json.gz')['commands']
    commands = []
    for argv in verifier.COMMANDS:
        found = next((c for c in frozen if c['argv'] == argv), None)
        if found is not None:
            commands.append(deepcopy(found))
        elif argv[-1].endswith('/verify_evidence.py'):
            commands.append(dict(argv=argv, returncode=0, stderr='',
                                 stdout=json.dumps(verifier.load(verifier.HERE / 'summary.json'))))
        else:
            commands.append(dict(argv=argv, returncode=0, stdout='', stderr='Ran 1 test in 0.001s\n\nOK\n'))
    return commands


class ReceiptTests(unittest.TestCase):
    def test_current_frozen_baseline_passes(self):
        verifier.baseline()
        verifier.frozen_aw()

    def test_changed_runtime_or_dependency_is_rejected(self):
        original = Path.read_bytes
        for relative in ('Cargo.lock', 'src/vm.rs'):
            def changed(path):
                return b'changed' if path == verifier.ROOT / relative else original(path)
            with self.subTest(relative=relative), patch.object(Path, 'read_bytes', changed):
                with self.assertRaisesRegex(AssertionError, 'frozen input changed'):
                    verifier.baseline()

    def test_deleted_manifest_entry_is_rejected(self):
        original = verifier.load
        data = deepcopy(original(verifier.HERE / 'baseline-inputs.json'))
        del data['files']['Cargo.lock']
        with patch.object(verifier, 'load', lambda p: data if p.name == 'baseline-inputs.json' else original(p)):
            with self.assertRaisesRegex(AssertionError, 'file inventory'):
                verifier.baseline()

    def test_historical_document_suffix_cannot_be_rewritten(self):
        original = Path.read_bytes
        def changed(path):
            raw = original(path)
            return raw + b'changed historical suffix' if path == verifier.ROOT / 'README.md' else raw
        with patch.object(Path, 'read_bytes', changed):
            with self.assertRaisesRegex(AssertionError, 'historical document suffix changed'):
                verifier.baseline()

    def test_equal_count_replacement_does_not_hide_lost_scenario(self):
        commands = commands_fixture()
        test = next(c for c in commands if c['argv'][:2] == ['cargo', 'test'])
        verifier.validate_rust(test['stdout'] + test['stderr'])
        name = verifier.rust_names(test['stdout'])[0]
        test['stdout'] = test['stdout'].replace('test ' + name + ' ... ok', 'test invented_replacement ... ok', 1)
        with self.assertRaisesRegex(AssertionError, 'scenario inventory'):
            verifier.validate_commands(commands)

    def test_duplicate_pass_line_is_rejected(self):
        test = next(c for c in commands_fixture() if c['argv'][:2] == ['cargo', 'test'])
        output = test['stdout'] + test['stderr']
        with self.assertRaisesRegex(AssertionError, 'duplicate Rust scenario'):
            verifier.validate_rust(output + '\ntest ' + verifier.rust_names(output)[0] + ' ... ok\n')

    def test_missing_doctest_is_rejected(self):
        test = next(c for c in commands_fixture() if c['argv'][:2] == ['cargo', 'test'])
        output = test['stdout'] + test['stderr']
        output = '\n'.join(line for line in output.splitlines() if not (line.startswith('test ') and 'compile fail' in line))
        with self.assertRaisesRegex(AssertionError, 'doctest scenario'):
            verifier.validate_rust(output)

    def test_missing_command_and_nonzero_exit_are_rejected(self):
        commands = commands_fixture()
        with self.assertRaisesRegex(AssertionError, 'command inventory'):
            verifier.validate_commands(commands[:-1])
        commands[-1]['returncode'] = 1
        with self.assertRaisesRegex(AssertionError, 'command failed'):
            verifier.validate_commands(commands)

    def test_zero_python_tests_and_failed_tests_are_rejected(self):
        commands = commands_fixture()
        target = next(c for c in commands if c['argv'][-1] == 'test_verify_evidence.py')
        for output in ('Ran 0 tests in 0.001s\n\nOK\n', 'Ran 1 test in 0.001s\n\nFAILED (failures=1)\n'):
            target['stderr'] = output
            with self.assertRaisesRegex(AssertionError, 'Python test result'):
                verifier.validate_commands(commands)

    def test_missing_native_evidence_cannot_claim_new_analysis(self):
        commands = commands_fixture()
        evidence = json.loads(commands[0]['stdout'])
        evidence['new_records'] = 0
        commands[0]['stdout'] = json.dumps(evidence)
        with self.assertRaisesRegex(AssertionError, '2AX evidence inventory'):
            verifier.validate_commands(commands)

    def test_changed_reused_corpus_is_rejected(self):
        original = Path.read_bytes
        def changed(path):
            raw = original(path)
            if path == verifier.ROOT / verifier.CORPUS:
                data = json.loads(raw)
                data['scope'] += ' changed'
                return json.dumps(data).encode()
            return raw
        with patch.object(Path, 'read_bytes', changed):
            with self.assertRaisesRegex(AssertionError, 'reused corpus hash'):
                verifier.reused_corpus()

    def test_log_hash_and_receipt_capability_tampering_are_rejected(self):
        commands = commands_fixture()
        expected = verifier.build_record(commands)
        raw = gzip.compress(json.dumps(dict(stage='2AX', target_execution='none', commands=commands)).encode())
        record = dict(expected, logs_sha256=verifier.digest(raw))
        with patch.object(verifier, 'build_record', return_value=expected):
            verifier.validate_record(record, commands, raw)
            with self.assertRaisesRegex(AssertionError, 'validation logs changed'):
                verifier.validate_record(record, commands, raw + b'changed')
            for key in ('runtime_changed', 'direct_owner_capture_ready', 'stage2_complete'):
                changed = deepcopy(record)
                changed[key] = True
                with self.subTest(key=key), self.assertRaisesRegex(AssertionError, 'receipt content'):
                    verifier.validate_record(changed, commands, raw)

    def test_deliverable_digest_change_is_rejected(self):
        commands = commands_fixture()
        expected = verifier.build_record(commands)
        raw = b'captured log fixture'
        record = dict(deepcopy(expected), logs_sha256=verifier.digest(raw))
        record['artifact_sha256']['README.md'] = '0' * 64
        with patch.object(verifier, 'build_record', return_value=expected):
            with self.assertRaisesRegex(AssertionError, 'delivery hashes'):
                verifier.validate_record(record, commands, raw)

    def test_current_evidence_summary_cannot_be_replaced(self):
        commands = commands_fixture()
        evidence = json.loads(commands[0]['stdout'])
        evidence['anchors'] += 1
        commands[0]['stdout'] = json.dumps(evidence)
        with self.assertRaisesRegex(AssertionError, 'evidence log differs from current delivery'):
            verifier.build_record(commands)

    def test_actual_record_preserves_closed_gates_and_corpus_authority(self):
        record = verifier.build_record(commands_fixture())
        self.assertTrue(record['container_capture_ready'])
        for key in ('runtime_changed', 'public_api_changed', 'dependencies_changed',
                    'direct_owner_capture_ready', 'non_scalar_ready', 'stage2_complete'):
            self.assertIs(record[key], False)
        self.assertFalse(record['reused_corpus']['rerun'])
        self.assertEqual(record['target_execution'], 'none')
        self.assertEqual(record['selected_next'], 'T05-lifecycle-contract-evidence')


if __name__ == '__main__':
    unittest.main()
