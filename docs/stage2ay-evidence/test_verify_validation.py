"""Reject receipt tampering using real frozen logs, never invented pass logs."""
from copy import deepcopy
from pathlib import Path
import unittest
from unittest.mock import patch

import verify_validation as v


class ReceiptTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.logs = v.load(v.ROOT / 'docs/stage2ax-evidence/validation-logs.json.gz')['commands']
        cls.rust = next(c for c in cls.logs if c['argv'] == ['cargo', 'test', '--locked', '--offline'])
        cls.rust_output = cls.rust['stdout'] + cls.rust['stderr']

    def test_actual_frozen_baseline_and_prior_log_replay_pass(self):
        self.assertEqual(len(v.baseline()['files']), 734)
        self.assertEqual(v.frozen_ax()['stage'], '2AX')

    def test_prior_receipt_whole_delivery_verifier_is_never_called(self):
        with patch.object(v._old, 'verify', side_effect=AssertionError('old whole-delivery called')):
            with patch.object(v._old, 'build_record', side_effect=AssertionError('old receipt rebuilt')):
                self.assertEqual(v.frozen_ax()['status'], 'pass')

    def test_frozen_runtime_dependency_model_and_old_receipt_cannot_change(self):
        original = Path.read_bytes
        for relative in ('src/vm.rs', 'Cargo.lock', 'docs/stage2as-design/model.py',
                         'docs/stage2ax-validation.json'):
            def changed(path):
                return b'changed' if path == v.ROOT / relative else original(path)
            with self.subTest(relative=relative), patch.object(Path, 'read_bytes', changed):
                with self.assertRaisesRegex(AssertionError, 'frozen input changed'):
                    v.baseline()

    def test_baseline_manifest_cannot_omit_a_file(self):
        original = v.load
        data = deepcopy(original(v.HERE / 'baseline-inputs.json'))
        del data['files']['Cargo.lock']
        with patch.object(v, 'load', lambda p: data if p == v.HERE / 'baseline-inputs.json' else original(p)):
            with self.assertRaisesRegex(AssertionError, 'file inventory'):
                v.baseline()

    def test_document_historical_suffix_cannot_be_rewritten(self):
        original = Path.read_bytes
        def changed(path):
            raw = original(path)
            return raw + b'changed suffix' if path == v.ROOT / 'README.md' else raw
        with patch.object(Path, 'read_bytes', changed):
            with self.assertRaisesRegex(AssertionError, 'historical document suffix'):
                v.baseline()

    def test_equal_count_scenario_replacement_is_rejected(self):
        v.validate_rust(self.rust_output)
        name = v.rust_names(self.rust_output)[0]
        altered = self.rust_output.replace('test ' + name + ' ... ok',
                                          'test fabricated_replacement ... ok', 1)
        with self.assertRaisesRegex(AssertionError, 'scenario inventory'):
            v.validate_rust(altered)

    def test_duplicate_scenario_and_missing_doctest_are_rejected(self):
        name = v.rust_names(self.rust_output)[0]
        with self.assertRaisesRegex(AssertionError, 'duplicate Rust scenario'):
            v.validate_rust(self.rust_output + '\ntest ' + name + ' ... ok\n')
        altered = '\n'.join(line for line in self.rust_output.splitlines()
                            if not (line.startswith('test ') and 'compile fail' in line))
        with self.assertRaisesRegex(AssertionError, 'doctest scenario'):
            v.validate_rust(altered)

    def test_command_inventory_and_failed_commands_are_rejected(self):
        with self.assertRaisesRegex(AssertionError, 'command inventory'):
            v.validate_commands([])
        # A deliberately failed inventory is a rejection fixture, not fabricated pass output.
        failed = [dict(argv=argv, returncode=1, stdout='', stderr='rejection fixture')
                  for argv in v.COMMANDS]
        with self.assertRaisesRegex(AssertionError, 'command failed'):
            v.validate_commands(failed)

    def test_empty_or_failed_python_runs_cannot_count_as_checks(self):
        frozen = next(c for c in self.logs if c['argv'][-1] == 'test_verify_evidence.py')
        self.assertEqual(v.python_count(frozen['stdout'] + frozen['stderr']), 37)
        for text in ('Ran 0 tests in 0.001s\n\nOK\n',
                     'Ran 2 tests in 0.001s\n\nFAILED (failures=1)\n'):
            with self.subTest(text=text), self.assertRaisesRegex(AssertionError, 'Python test result'):
                v.python_count(text)

    def test_actual_summary_cannot_be_replaced_or_lose_new_evidence(self):
        actual = v.current_evidence()
        altered = deepcopy(actual)
        altered['anchors'] += 1
        with self.assertRaisesRegex(AssertionError, 'evidence log differs from current delivery'):
            v.validate_current_evidence(altered)
        altered = deepcopy(actual)
        altered['new_records'] = 0
        with self.assertRaisesRegex(AssertionError, '2AY evidence inventory'):
            v.validate_evidence_summary(altered)

    def test_frozen_prior_logs_and_reused_corpus_are_hash_bound(self):
        original = Path.read_bytes
        for relative, check, message in [
                ('docs/stage2ax-evidence/validation-logs.json.gz', v.frozen_ax, 'frozen receipt or logs'),
                (v.CORPUS, v.reused_corpus, 'reused corpus hash')]:
            def changed(path):
                raw = original(path)
                return raw + b'changed' if path == v.ROOT / relative else raw
            with self.subTest(relative=relative), patch.object(Path, 'read_bytes', changed):
                with self.assertRaisesRegex(AssertionError, message):
                    check()

    def test_receipt_log_hash_rejects_changed_bytes_before_replay(self):
        raw = (v.ROOT / 'docs/stage2ax-evidence/validation-logs.json.gz').read_bytes()
        with self.assertRaisesRegex(AssertionError, 'validation logs changed'):
            v.validate_record({'logs_sha256': v.digest(raw)}, [], raw + b'changed')

    def test_closed_capability_policy_rejects_boolean_and_type_changes(self):
        policy = v.capabilities()
        v.validate_capabilities(policy)
        for key in policy:
            for value in (not policy[key], int(policy[key])):
                altered = deepcopy(policy)
                altered[key] = value
                with self.subTest(key=key, value=value), self.assertRaisesRegex(AssertionError, 'capability policy'):
                    v.validate_capabilities(altered)
        for key in ('runtime_changed', 'public_api_changed', 'dependencies_changed', 'model_changed',
                    'direct_owner_capture_ready', 'non_scalar_ready', 'stage2_complete'):
            self.assertIs(policy[key], False)

    def test_delivery_hash_tampering_is_rejected(self):
        # Unit-test the comparison against production delivery hashes. No successful
        # command log or receipt is invented or written by this rejection test.
        expected = dict(v.capabilities(), artifact_sha256=v.delivery())
        raw = (v.ROOT / 'docs/stage2ax-evidence/validation-logs.json.gz').read_bytes()
        altered = dict(deepcopy(expected), logs_sha256=v.digest(raw))
        altered['artifact_sha256']['README.md'] = '0' * 64
        with patch.object(v, 'build_record', return_value=expected):
            with self.assertRaisesRegex(AssertionError, 'delivery hashes'):
                v.validate_record(altered, [], raw)

    def test_scope_keeps_corpus_reused_and_108_prior_python_checks(self):
        corpus = v.reused_corpus()
        self.assertIs(corpus['rerun'], False)
        self.assertEqual(corpus['source_stage'], '2AW')
        self.assertEqual(corpus['target_execution'], 'none')
        self.assertEqual(sum(v.FROZEN_PYTHON_COUNTS.values()), 108)
        self.assertTrue(v.SELECTED_NEXT)
        self.assertEqual(v.COMMANDS[1], ['python3', '-B', 'docs/stage2ax-evidence/verify_evidence.py'])
        self.assertNotIn(['python3', '-B', 'docs/stage2ax-evidence/verify_validation.py'], v.COMMANDS)


if __name__ == '__main__':
    unittest.main()
