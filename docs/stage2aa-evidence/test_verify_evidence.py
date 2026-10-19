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
        self.here = self.root / 'docs/stage2aa-evidence'
        self.here.mkdir(parents=True)
        for name in ['rea.json.gz', 'reused.json.gz', 'function-manifest.json', 'navigation.json.gz', 'navigation-manifest.json']:
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
        self.assertEqual(self.check()['records'], 22)

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
        self.reject('function-manifest.json', lambda m: m[1].update(body={}), 'manifest body')

    def test_manifest_provenance(self):
        self.reject('function-manifest.json', lambda m: m[0].update(source_sha256='wrong'), 'manifest provenance')

    def test_opcode_binding(self):
        path = self.root / 'docs/evidence/actions.csv'
        path.write_text(path.read_text().replace('0x35,0x809113c,vm_pushGlobalVar,', '0x35,0x809113d,vm_pushGlobalVar,'))
        with self.assertRaisesRegex(ValueError, 'opcode binding'):
            self.check()

    def mutate_new_and_rehash(self, change):
        bundle = read_json(self.here / 'rea.json.gz')
        change(bundle['records'])
        self.save('rea.json.gz', bundle)
        manifest = read_json(self.here / 'function-manifest.json')
        sha = hashlib.sha256((self.here / 'rea.json.gz').read_bytes()).hexdigest()
        for item in manifest:
            if item['source'] == 'docs/stage2aa-evidence/rea.json.gz':
                item['source_sha256'] = sha
        self.save('function-manifest.json', manifest)

    def test_instruction_site_after_rehash(self):
        def change(records):
            r=next(r for r in records if r['parameters']['procedure']=='vm_pushGlobalVptr')
            r['normalized_result']=r['normalized_result'].replace('INC dword ptr [EBX]','NOP')
        self.mutate_new_and_rehash(change)
        with self.assertRaisesRegex(ValueError,'instruction site'):
            self.check()

    def test_allocator_target_after_rehash(self):
        def change(records):
            r=next(r for r in records if r['parameters']['procedure']=='vm_endNewFile')
            r['normalized_result']['assembly']=[line.replace('0x080ce784','0x080ce785')
                for line in r['normalized_result']['assembly']]
        self.mutate_new_and_rehash(change)
        with self.assertRaisesRegex(ValueError,'instruction site'):
            self.check()

    def test_manifest_address(self):
        self.reject('function-manifest.json',lambda m:m[0].update(address='0x1'),'manifest address')

    def test_navigation_missing(self):
        self.reject('navigation.json.gz',lambda b:b['records'].pop(),'navigation count')

    def test_navigation_edited_manifest(self):
        self.reject('navigation-manifest.json',lambda m:m[0].update(limitations=[]),'navigation manifest exact')

    def test_navigation_edited_both(self):
        b=read_json(self.here/'navigation.json.gz')
        eid=b['records'][0]['evidence_id']
        b['records'][0]['parameters']['address']='0x1'
        self.save('navigation.json.gz',b)
        manifest=read_json(self.here/'navigation-manifest.json')
        next(m for m in manifest if m['evidence_id']==eid)['parameters']['address']='0x1'
        self.save('navigation-manifest.json',manifest)
        with self.assertRaisesRegex(ValueError,'navigation operations'):
            self.check()

    def test_navigation_duplicate(self):
        self.reject('navigation.json.gz',lambda b:b['records'].__setitem__(1,b['records'][0]),'navigation unique ids')


if __name__ == '__main__':
    unittest.main()
