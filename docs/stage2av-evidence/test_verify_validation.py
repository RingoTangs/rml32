"""Mutation checks for history, scenario coverage and contact provenance."""
from copy import deepcopy
from pathlib import Path
import unittest
from unittest.mock import patch
import verify_validation as verifier


class ReceiptTests(unittest.TestCase):
    def test_registered_regression_delta_matches_frozen_source(self):
        verifier.baseline()

    def test_an_unregistered_assertion_change_is_rejected(self):
        current = (verifier.ROOT / 'src/regression.rs').read_text().replace('assert_eq!(', 'assert_ne!(', 1)
        with self.assertRaisesRegex(AssertionError, 'unregistered'):
            verifier.check_regression(verifier.old_sources()['src/regression.rs'], current)

    def test_a_changed_frozen_dependency_is_rejected(self):
        original = Path.read_bytes
        def changed(path):
            return b'changed' if path == verifier.ROOT / 'Cargo.lock' else original(path)
        with patch.object(Path, 'read_bytes', changed):
            with self.assertRaisesRegex(AssertionError, 'frozen input changed: Cargo.lock'):
                verifier.baseline()

    def test_an_omitted_copy_contact_is_rejected(self):
        data = deepcopy(verifier.load(verifier.HERE / 'publication-contacts.json'))
        del data['items'][0]
        with self.assertRaises(AssertionError):
            verifier.contacts(data)

    def test_a_stale_current_source_anchor_is_rejected(self):
        data = deepcopy(verifier.load(verifier.HERE / 'publication-contacts.json'))
        data['items'][0]['after']['line'] += 1
        with self.assertRaisesRegex(AssertionError, 'stale source anchor'):
            verifier.contacts(data)

    def test_equal_counts_cannot_hide_a_lost_baseline_scenario(self):
        old = {verifier.RENAME.get(n, n) for n in verifier.old_names()}
        expected = verifier.source_test_count()
        cases = old | {f'extra::test_{n}' for n in range(expected - len(old))}
        def output(names):
            return '\n'.join(f'test {name} ... ok' for name in names) + f'\ntest result: ok. {expected} passed; 0 failed;\ntest result: ok. 2 passed; 0 failed;\n'
        verifier.validate_rust(output(cases))
        cases.remove(sorted(old)[0])
        cases.add('extra::replacement_hides_loss')
        with self.assertRaisesRegex(AssertionError, 'lost baseline scenario'):
            verifier.validate_rust(output(cases))


if __name__ == '__main__':
    unittest.main()
