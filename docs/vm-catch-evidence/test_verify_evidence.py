"""Synthetic evidence provenance rejection tests; no native target execution."""
import copy
import gzip
import hashlib
import json
from pathlib import Path
import tempfile
import unittest

from verify_evidence import TARGET_SHA, check_records, check_sources


def record():
    return dict(evidence_id='ev_'+'a'*64, provider=dict(id='ghidra'),
                subject=dict(digest=dict(sha256=TARGET_SHA)), authority='shipped-artifact')


class EvidenceTests(unittest.TestCase):
    def test_accepts_matching_record(self):
        r = record()
        self.assertEqual(check_records([r]), {r['evidence_id']: r})

    def test_rejects_wrong_engine_and_target(self):
        for change in ('provider', 'target', 'authority'):
            r = record()
            if change == 'provider':
                r['provider']['id'] = 'ida'
            elif change == 'target':
                r['subject']['digest']['sha256'] = '0'*64
            else:
                r['authority'] = 'analyst-inference'
            with self.subTest(change=change), self.assertRaises(AssertionError):
                check_records([r])

    def test_rejects_duplicate_record(self):
        with self.assertRaisesRegex(AssertionError, 'duplicate'):
            check_records([record(), record()])

    def test_rejects_invalid_evidence_id(self):
        r = record()
        r['evidence_id'] = 'ev_partial'
        with self.assertRaisesRegex(AssertionError, 'invalid'):
            check_records([r])

    def test_source_hash_and_exact_record_equality(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            path = root / 'source.json.gz'
            original = record()
            path.write_bytes(gzip.compress(json.dumps(dict(records=[original])).encode(), mtime=0))
            bundle = dict(records=[copy.deepcopy(original)], sources=[dict(path=path.name,
                sha256=hashlib.sha256(path.read_bytes()).hexdigest(), evidence_ids=[original['evidence_id']])])
            check_sources(bundle, root)
            bundle['records'][0]['unobserved_addition'] = 'invented'
            with self.assertRaisesRegex(AssertionError, 'differs'):
                check_sources(bundle, root)
            bundle['records'][0] = original
            path.write_bytes(path.read_bytes() + b'changed')
            with self.assertRaisesRegex(AssertionError, 'changed evidence source'):
                check_sources(bundle, root)

    def test_rejects_unattributed_record(self):
        with tempfile.TemporaryDirectory() as directory:
            with self.assertRaisesRegex(AssertionError, 'unattributed'):
                check_sources(dict(records=[record()], sources=[]), Path(directory))


if __name__ == '__main__':
    unittest.main()
