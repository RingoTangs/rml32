"""Synthetic corruptions of copied evidence; originals remain read-only."""
import gzip
import importlib.util
import json
from pathlib import Path
import shutil
import tempfile
import unittest

HERE = Path(__file__).resolve().parent
SPEC = importlib.util.spec_from_file_location('stage2w_verify_evidence', HERE / 'verify_evidence.py')
VERIFY = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(VERIFY)


class EvidenceVerification(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        self.root = Path(self.temp.name)
        self.here = self.root / 'docs/stage2w-evidence'
        self.here.mkdir(parents=True)
        for name in ['reused.json.gz', 'function-manifest.json', 'fault-sites.json']:
            shutil.copyfile(HERE / name, self.here / name)
        for source in VERIFY.read_json(HERE / 'reused.json.gz')['sources']:
            target = self.root / source['path']
            target.parent.mkdir(parents=True, exist_ok=True)
            shutil.copyfile(HERE.parent.parent / source['path'], target)

    def mutate(self, filename, edit):
        path = self.here / filename
        data = VERIFY.read_json(path)
        edit(data)
        raw = json.dumps(data).encode()
        path.write_bytes(gzip.compress(raw, mtime=0) if path.suffix == '.gz' else raw)

    def reject(self):
        with self.assertRaises(ValueError):
            VERIFY.verify_bundle(self.root, self.here)

    def test_real_bundle_passes(self):
        self.assertEqual(VERIFY.verify_bundle(self.root, self.here),
                         {'records': 8, 'source_bundles': 3, 'sites': 7, 'private_sources': 5})

    def test_wrong_provider(self):
        self.mutate('reused.json.gz', lambda d: d['providers'][0].update(id='other'))
        self.reject()

    def test_wrong_target(self):
        self.mutate('reused.json.gz', lambda d: d['artifacts'][0]['digest'].update(sha256='0' * 64))
        self.reject()

    def test_wrong_record_provider(self):
        self.mutate('reused.json.gz', lambda d: d['records'][0]['provider'].update(id='other'))
        self.reject()

    def test_bad_source_hash(self):
        self.mutate('reused.json.gz', lambda d: d['sources'][0].update(sha256='0' * 64))
        self.reject()

    def test_changed_source_bytes(self):
        source = VERIFY.read_json(self.here / 'reused.json.gz')['sources'][0]['path']
        path = self.root / source
        path.write_bytes(path.read_bytes() + b'changed')
        self.reject()

    def test_edited_record(self):
        self.mutate('reused.json.gz', lambda d: d['records'][0].update(normalized_result='changed'))
        self.reject()

    def test_unknown_site_evidence(self):
        self.mutate('fault-sites.json', lambda d: d['sites'][0].update(evidence_ids=['unknown']))
        self.reject()

    def test_wrong_error_address(self):
        self.mutate('fault-sites.json', lambda d: d['sites'][0].update(error_call='0x0808e287'))
        self.reject()

    def test_wrong_branch_address(self):
        self.mutate('fault-sites.json', lambda d: d['sites'][1].update(zero_branch='0x0808e291'))
        self.reject()

    def test_wrong_delegate_address(self):
        self.mutate('fault-sites.json', lambda d: d['sites'][5].update(delegate_call='0x0808ec52'))
        self.reject()

    def test_wrong_operand_types(self):
        self.mutate('fault-sites.json', lambda d: d['sites'][1].update(operand_types=['Int', 'String']))
        self.reject()

    def test_unknown_fault_source(self):
        self.mutate('fault-sites.json', lambda d: d['sites'][1].update(rust_fault_source='AnyArithmetic'))
        self.reject()

    def test_wrong_compound_selector(self):
        self.mutate('fault-sites.json', lambda d: d['sites'][6].update(selector=5))
        self.reject()

    def test_wrong_manifest_address(self):
        self.mutate('function-manifest.json', lambda d: d[0].update(address='0x0808e287'))
        self.reject()

    def test_unknown_manifest_id(self):
        self.mutate('function-manifest.json', lambda d: d[0].update(evidence_id='unknown'))
        self.reject()

    def test_fault_target_mismatch(self):
        self.mutate('fault-sites.json', lambda d: d.update(target_sha256='0' * 64))
        self.reject()

    def test_missing_compound_error(self):
        self.mutate('fault-sites.json', lambda d: d['sites'][5]['error_calls'].pop())
        self.reject()


if __name__ == '__main__':
    unittest.main()
