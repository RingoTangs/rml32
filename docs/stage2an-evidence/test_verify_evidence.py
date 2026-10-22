import copy
import tempfile
import unittest
from pathlib import Path

from verify_evidence import (ROOT, Elf, canonical, digest, load, safe_path, validate_anchors,
                             validate_findings, validate_origins, validate_records, verify)


class EvidenceTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        directory = 'docs/stage2an-evidence/'
        cls.records = load(ROOT, directory + 'rea.json.gz')['records'] + load(ROOT, directory + 'reused.json.gz')['records']
        cls.manifest = load(ROOT, directory + 'function-manifest.json')
        cls.anchors = load(ROOT, directory + 'anchors.json')
        cls.findings = load(ROOT, directory + 'findings.json')
        cls.cases = load(ROOT, directory + 'expected-cases.json')
        cls.origins = load(ROOT, directory + 'reused-manifest.json')
        cls.elf = Elf((ROOT / 'magic_Linux32').read_bytes())

    def test_delivery_and_protected_inputs(self):
        self.assertEqual(verify()['unique_records'], 20)
        # Importing the shared ELF helper must not redirect this stage's scanner
        # to the identically named historical module.
        from verify_validation import artifacts
        self.assertIn('corpus-summary.json', artifacts())

    def test_missing_or_duplicate_record_rejected(self):
        for records in [self.records[:-1], self.records + [self.records[0]]]:
            with self.assertRaises(ValueError):
                validate_records(records, self.manifest, self.elf)

    def test_wrong_target_rejected_even_after_rehash(self):
        records = copy.deepcopy(self.records)
        manifest = copy.deepcopy(self.manifest)
        records[0]['subject']['digest']['sha256'] = '0' * 64
        manifest[0]['record_sha256'] = digest(canonical(records[0]))
        with self.assertRaisesRegex(ValueError, 'target'):
            validate_records(records, manifest, self.elf)

    def test_forged_entry_rejected_even_after_rehash(self):
        records = copy.deepcopy(self.records)
        manifest = copy.deepcopy(self.manifest)
        records[0]['normalized_result']['procedure']['address'] = '0x8093d99'
        manifest[0]['record_sha256'] = digest(canonical(records[0]))
        with self.assertRaisesRegex(ValueError, 'identity'):
            validate_records(records, manifest, self.elf)

    def test_wrong_manifest_address_and_body_hash_rejected(self):
        for field, value in [('address', '0x8093d99'), ('body_sha256', '0' * 64)]:
            manifest = copy.deepcopy(self.manifest)
            manifest[0][field] = value
            with self.subTest(field=field), self.assertRaises(ValueError):
                validate_records(self.records, manifest, self.elf)

    def test_dropped_raw_record_rejected(self):
        records = copy.deepcopy(self.records)
        records[0]['raw_result'] = None
        manifest = copy.deepcopy(self.manifest)
        manifest[0]['record_sha256'] = digest(canonical(records[0]))
        with self.assertRaisesRegex(ValueError, 'missing record'):
            validate_records(records, manifest, self.elf)

    def test_changed_limitations_rejected(self):
        manifest = copy.deepcopy(self.manifest)
        manifest[0]['limitations'] = []
        with self.assertRaisesRegex(ValueError, 'limitations'):
            validate_records(self.records, manifest, self.elf)

    def test_wrong_source_digest_rejected(self):
        origins = copy.deepcopy(self.origins)
        origins[0]['source_sha256'] = '0' * 64
        with self.assertRaisesRegex(ValueError, 'source changed'):
            validate_origins(self.records[3:], origins, ROOT)

    def test_missing_reuse_link_rejected(self):
        with self.assertRaises(ValueError):
            validate_origins(self.records[3:], self.origins[:-1], ROOT)

    def test_anchor_address_instruction_and_machine_bytes(self):
        byid = {r['evidence_id']: r for r in self.records}
        for field, value in [('address', '0x0'), ('instruction', 'fabricated'), ('bytes_hex', '00')]:
            anchors = copy.deepcopy(self.anchors)
            anchors[0][field] = value
            with self.subTest(field=field), self.assertRaises(ValueError):
                validate_anchors(anchors, byid, self.elf)

    def test_unknown_finding_evidence_rejected(self):
        findings = copy.deepcopy(self.findings)
        findings[0]['evidence'] = ['ev_missing']
        with self.assertRaisesRegex(ValueError, 'dangling'):
            validate_findings(findings, self.cases, {r['evidence_id']: r for r in self.records})

    def test_case_requires_existing_finding_and_declared_basis(self):
        for field, value in [('findings', ['absent']), ('basis', 'runtime-observed')]:
            cases = copy.deepcopy(self.cases)
            cases[0][field] = value
            with self.subTest(field=field), self.assertRaises(ValueError):
                validate_findings(self.findings, cases, {r['evidence_id']: r for r in self.records})

    def test_unsafe_source_paths_rejected(self):
        for path in ['', '../magic_Linux32', '/tmp/test', 'docs/../../test', 'docs\\test']:
            with self.subTest(path=path), self.assertRaises(ValueError):
                safe_path(ROOT, path)

    def test_source_symlink_cannot_escape_root(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            (root / 'escape').symlink_to(ROOT / 'magic_Linux32')
            with self.assertRaises(ValueError):
                safe_path(root, 'escape')


if __name__ == '__main__':
    unittest.main()
