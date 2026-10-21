"""Small synthetic provenance fixtures; no large retained bundle copies."""
import copy
import gzip
import json
from pathlib import Path
import tempfile
import unittest

import verify_reused as v


class RetainedRecordsTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        self.root = Path(self.temp.name)
        self.record = {'evidence_id': 'ev_test', 'provider': {'id': 'ghidra', 'version': '12.1.4'},
                       'subject': {'digest': {'sha256': v.TARGET}}, 'raw_result': {'full': ['raw']},
                       'parameters': {'procedure': 'vm_eqValue'}, 'operation': 'procedure_pseudo_code',
                       'normalized_result': 'void vm_eqValue(void) {}'}
        self.bundle = {'records': [self.record]}
        self.source = self.root / 'source.json.gz'
        self.source.write_bytes(gzip.compress(v.canonical(self.bundle), mtime=0))
        self.item = {'source': 'source.json.gz', 'source_sha256': v.digest(self.source.read_bytes()),
                     'evidence_id': 'ev_test', 'record_sha256': v.digest(v.canonical(self.record)),
                     'provider': copy.deepcopy(self.record['provider']),
                     'subject': copy.deepcopy(self.record['subject']), 'function': 'vm_eqValue',
                     'operation': 'procedure_pseudo_code', 'address': None}
        self.manifest = {'records': [self.item], 'record_count': 1}

    def check(self):
        return v.validate_records(self.bundle, self.manifest, self.root)

    def test_valid_complete_record(self):
        self.assertTrue(self.check()['provenance_verified'])

    def test_missing_record(self):
        self.bundle['records'] = []
        with self.assertRaises(ValueError): self.check()

    def test_edited_raw_record(self):
        self.record['raw_result']['full'].append('edited')
        with self.assertRaisesRegex(ValueError, 'complete source'): self.check()

    def test_missing_raw_record(self):
        del self.record['raw_result']
        with self.assertRaises(ValueError): self.check()

    def test_metadata_tampering(self):
        for key, value in [('address', '0xdead'), ('provider', {'id': 'other'}),
                           ('subject', {'digest': {'sha256': 'wrong'}}),
                           ('source_sha256', 'bad'), ('record_sha256', 'bad'),
                           ('function', 'wrong'), ('operation', 'wrong')]:
            with self.subTest(key=key):
                old = self.item[key]
                self.item[key] = value
                with self.assertRaises(ValueError): self.check()
                self.item[key] = old

    def test_duplicate_manifest(self):
        self.manifest['records'].append(copy.deepcopy(self.item))
        with self.assertRaisesRegex(ValueError, 'duplicate manifest'): self.check()

    def test_duplicate_bundle(self):
        self.bundle['records'].append(copy.deepcopy(self.record))
        with self.assertRaisesRegex(ValueError, 'duplicate bundle'): self.check()

    def test_escaping_paths(self):
        for path in ['../source.json.gz', str(self.source)]:
            with self.subTest(path=path):
                self.item['source'] = path
                with self.assertRaisesRegex(ValueError, 'unsafe'): self.check()

    def test_symlink_escape(self):
        (self.root / 'escape').symlink_to(self.root.parent)
        self.item['source'] = 'escape/source.json.gz'
        with self.assertRaisesRegex(ValueError, 'escapes'): self.check()

    def test_wrong_count(self):
        self.manifest['record_count'] = 15
        with self.assertRaisesRegex(ValueError, 'count'): self.check()

    def test_source_missing_evidence(self):
        self.source.write_bytes(gzip.compress(v.canonical({'records': []}), mtime=0))
        self.item['source_sha256'] = v.digest(self.source.read_bytes())
        with self.assertRaisesRegex(ValueError, 'source record'): self.check()

    def test_native_identity_checks(self):
        for field, value in [('provider', {'id': 'ghidra', 'version': 'wrong'}),
                             ('subject', {'digest': {'sha256': 'wrong'}})]:
            with self.subTest(field=field):
                original = copy.deepcopy(self.record[field])
                self.record[field] = value
                self.item[field] = copy.deepcopy(value)
                self.source.write_bytes(gzip.compress(v.canonical(self.bundle), mtime=0))
                self.item['source_sha256'] = v.digest(self.source.read_bytes())
                self.item['record_sha256'] = v.digest(v.canonical(self.record))
                with self.assertRaises(ValueError): self.check()
                self.record[field] = original


if __name__ == '__main__':
    unittest.main()
