"""Corrupt temporary copies; preserve all retained evidence and the executable."""
import gzip
import hashlib
import json
from pathlib import Path
import shutil
import tempfile
import unittest

from verify_evidence import verify_bundle, read_json

HERE = Path(__file__).resolve().parent
ROOT = HERE.parent.parent


class VerifyEvidenceTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        self.root = Path(self.temp.name)
        self.here = self.root / 'docs/vm-range-evidence'
        self.here.mkdir(parents=True)
        for name in ['rea.json.gz', 'reused.json.gz', 'function-manifest.json']:
            shutil.copyfile(HERE / name, self.here / name)
        shutil.copyfile(ROOT / 'magic_Linux32', self.root / 'magic_Linux32')
        action = self.root / 'docs/evidence/actions.csv'
        action.parent.mkdir(parents=True)
        shutil.copyfile(ROOT / 'docs/evidence/actions.csv', action)
        for source in read_json(HERE / 'reused.json.gz')['sources']:
            dest = self.root / source['path']
            dest.parent.mkdir(parents=True, exist_ok=True)
            shutil.copyfile(ROOT / source['path'], dest)

    def save(self, filename, data):
        path = self.here / filename
        if filename.endswith('.gz'):
            with gzip.open(path, 'wt') as stream:
                json.dump(data, stream)
        else:
            path.write_text(json.dumps(data))

    def check(self):
        return verify_bundle(self.root, self.here)

    def reject(self, filename, change, message):
        data = read_json(self.here / filename)
        change(data)
        self.save(filename, data)
        with self.assertRaisesRegex(ValueError, message):
            self.check()

    def test_valid_baseline(self):
        self.assertEqual(self.check()['records'], 13)

    def test_target_bytes(self):
        (self.root / 'magic_Linux32').write_bytes(b'wrong executable')
        with self.assertRaisesRegex(ValueError, 'target bytes'):
            self.check()

    def test_record_target(self):
        self.reject('rea.json.gz', lambda b: b['records'][0]['subject']['digest'].update(sha256='wrong'), 'record target')

    def test_provider(self):
        self.reject('rea.json.gz', lambda b: b['providers'][0].update(version='wrong'), 'bundle provider')

    def test_duplicate_records(self):
        self.reject('rea.json.gz', lambda b: b['records'].__setitem__(1, b['records'][0]), 'duplicate record')

    def test_missing_record(self):
        self.reject('rea.json.gz', lambda b: b['records'].pop(), 'record count')

    def test_source_hash(self):
        self.reject('reused.json.gz', lambda b: b['sources'][0].update(sha256='wrong'), 'source hash')

    def test_source_boundary(self):
        self.reject('reused.json.gz', lambda b: b['sources'][0].update(path='../outside'), 'source outside root')

    def test_missing_provenance(self):
        self.reject('reused.json.gz', lambda b: b['sources'][0]['evidence_ids'].pop(), 'missing provenance')

    def test_edited_retained_record(self):
        self.reject('reused.json.gz', lambda b: b['records'][0]['limitations'].append('fabricated'), 'edited retained record')

    def test_manifest_bijection(self):
        self.reject('function-manifest.json', lambda m: m.__setitem__(1, m[0]), 'manifest bijection')

    def test_manifest_limitations(self):
        self.reject('function-manifest.json', lambda m: m[0].update(limitations=[]), 'manifest limitations')

    def test_manifest_body(self):
        self.reject('function-manifest.json', lambda m: m[0].update(body={}), 'manifest body')

    def test_manifest_provenance(self):
        self.reject('function-manifest.json', lambda m: m[0].update(source_sha256='wrong'), 'manifest provenance')

    def test_opcode_binding(self):
        path = self.root / 'docs/evidence/actions.csv'
        path.write_text(path.read_text().replace('0x45,0x808fd04,vm_queryRange,', '0x45,0x808fd05,vm_queryRange,'))
        with self.assertRaisesRegex(ValueError, 'opcode binding'):
            self.check()

    def test_instruction_site_after_rehash(self):
        bundle = read_json(self.here / 'rea.json.gz')
        query = next(r for r in bundle['records'] if r['parameters']['procedure'] == 'vm_queryRange')
        lines = query['normalized_result']['assembly']
        lines[:] = [line.replace('0x808fed2: CALL 0x080a36e4', '0x808fed2: NOP') for line in lines]
        self.save('rea.json.gz', bundle)
        manifest = read_json(self.here / 'function-manifest.json')
        sha = hashlib.sha256((self.here / 'rea.json.gz').read_bytes()).hexdigest()
        for item in manifest:
            if item['source'].endswith('/vm-range-evidence/rea.json.gz'):
                item['source_sha256'] = sha
        self.save('function-manifest.json', manifest)
        with self.assertRaisesRegex(ValueError, 'instruction site'):
            self.check()


if __name__ == '__main__':
    unittest.main()
