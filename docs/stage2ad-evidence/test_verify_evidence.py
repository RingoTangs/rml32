import gzip
import json
from pathlib import Path
import shutil
import tempfile
import unittest

from verify_evidence import OUT, ROOT, digest, read, source_path, verify


class EvidenceTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        self.here = Path(self.temp.name)
        for name in ('rea.json.gz', 'reused.json.gz', 'function-manifest.json',
                     'findings.json', 'functions.md'):
            shutil.copyfile(OUT / name, self.here / name)

    def change(self, name, mutation, *, refresh_manifest_hash=False):
        path = self.here / name
        value = read(path)
        mutation(value)
        raw = (json.dumps(value, sort_keys=True, indent=2) + '\n').encode()
        path.write_bytes(gzip.compress(raw, mtime=0) if name.endswith('.gz') else raw)
        if refresh_manifest_hash:
            manifest = self.here / 'function-manifest.json'
            values = read(manifest)
            for item in values:
                if item['source'] == 'docs/stage2ad-evidence/rea.json.gz':
                    item['source_sha256'] = digest(path)
            manifest.write_text(json.dumps(values))

    def reject(self, message):
        with self.assertRaisesRegex(ValueError, message):
            verify(ROOT, self.here)

    def test_unchanged_bundle_passes(self):
        result = verify(ROOT, self.here)
        self.assertEqual(result['records'], 20)
        self.assertEqual(result['findings'], 10)

    def test_wrong_provider(self):
        self.change('rea.json.gz', lambda b: b['providers'][0].update(version='other'))
        self.reject('provider')

    def test_wrong_target(self):
        self.change('rea.json.gz', lambda b: b['artifacts'][0]['digest'].update(sha256='0' * 64))
        self.reject('bundle target')

    def test_wrong_record_target(self):
        self.change('rea.json.gz', lambda b: b['records'][0]['subject']['digest'].update(sha256='0' * 64))
        self.reject('record provider or target')

    def test_missing_record(self):
        self.change('rea.json.gz', lambda b: b['records'].pop())
        self.reject('record counts')

    def test_duplicate_record(self):
        self.change('rea.json.gz', lambda b: b['records'].__setitem__(0, b['records'][1]))
        self.reject('duplicate evidence')

    def test_reused_source_hash(self):
        self.change('reused.json.gz', lambda b: b['sources'][0].update(sha256='0' * 64))
        self.reject('original bundle hash')

    def test_reused_record_cannot_be_edited(self):
        self.change('reused.json.gz', lambda b: b['records'][0].update(confidence='inferred'))
        self.reject('edited reused record')

    def test_provenance_must_cover_every_record(self):
        self.change('reused.json.gz', lambda b: b['sources'][0]['evidence_ids'].pop())
        self.reject('missing provenance')

    def test_source_paths_cannot_escape_root(self):
        for relative in ('../outside.json.gz', '/tmp/outside.json.gz'):
            with self.subTest(relative=relative), self.assertRaises(ValueError):
                source_path(ROOT, relative)

    def test_manifest_address_and_limitations(self):
        self.change('function-manifest.json', lambda m: m[0].update(address='0x0'))
        self.reject('manifest address')

    def test_manifest_limitations_are_exact(self):
        self.change('function-manifest.json', lambda m: m[0].update(limitations=[]))
        self.reject('manifest limitations')

    def test_manifest_is_a_bijection(self):
        self.change('function-manifest.json', lambda m: m.__setitem__(0, m[1]))
        self.reject('manifest bijection')

    def test_navigation_search_boundary_is_fixed(self):
        def mutation(bundle):
            next(r for r in bundle['records'] if r['operation'] == 'search_procedures')['parameters']['pattern'] = '.*'
        self.change('rea.json.gz', mutation)
        self.reject('navigation search boundary')

    def test_instruction_site_checked_even_after_manifest_hash_refresh(self):
        def mutation(bundle):
            r = next(r for r in bundle['records'] if r['parameters'].get('procedure') == 'vm_returnFromEfun')
            lines = r['normalized_result']['assembly']
            index = next(i for i, line in enumerate(lines) if line.startswith('0x8093c84:'))
            lines[index] = '0x8093c84: CMP EBX, 0x2'
        self.change('rea.json.gz', mutation, refresh_manifest_hash=True)
        self.reject('instruction site')

    def test_findings_cannot_reference_unknown_evidence(self):
        self.change('findings.json', lambda f: f[0].update(evidence_ids=['ev_unknown']))
        self.reject('unknown finding evidence')

    def test_observations_need_evidence(self):
        self.change('findings.json', lambda f: f[0].update(evidence_ids=[]))
        self.reject('unsubstantiated finding')

    def test_readable_view_covers_all_records(self):
        (self.here / 'functions.md').write_text('incomplete')
        self.reject('readable view evidence coverage')


if __name__ == '__main__':
    unittest.main()
