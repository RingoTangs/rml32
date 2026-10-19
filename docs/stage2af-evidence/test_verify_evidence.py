import gzip
import json
from pathlib import Path
import shutil
import tempfile
import unittest

from verify_evidence import OUT, ROOT, digest, read, source_path, verify


class EvidenceTests(unittest.TestCase):
    def setUp(self):
        self.tmp=tempfile.TemporaryDirectory();self.addCleanup(self.tmp.cleanup)
        self.here=Path(self.tmp.name)
        for name in ('rea.json.gz','reused.json.gz','function-manifest.json','findings.json','functions.md'):
            shutil.copyfile(OUT/name,self.here/name)

    def change(self,name,mutation,refresh=False):
        path=self.here/name;value=read(path);mutation(value)
        raw=(json.dumps(value,sort_keys=True,indent=2)+'\n').encode()
        path.write_bytes(gzip.compress(raw,mtime=0) if name.endswith('.gz') else raw)
        if refresh:
            manifest=read(self.here/'function-manifest.json')
            for m in manifest:
                if m['source']=='docs/stage2af-evidence/rea.json.gz':m['source_sha256']=digest(path)
            (self.here/'function-manifest.json').write_text(json.dumps(manifest))

    def reject(self,message):
        with self.assertRaisesRegex(ValueError,message):verify(ROOT,self.here)

    def test_original_bundle_passes(self):
        result=verify(ROOT,self.here)
        self.assertEqual((result['records'],result['instruction_sites'],result['findings']),(21,26,12))

    def test_provider_target_and_record_subject_are_checked(self):
        self.change('rea.json.gz',lambda b:b['providers'][0].update(version='other'))
        self.reject('bundle provider')

    def test_wrong_target(self):
        self.change('rea.json.gz',lambda b:b['artifacts'][0]['digest'].update(sha256='0'*64))
        self.reject('bundle target')

    def test_wrong_record_subject(self):
        self.change('rea.json.gz',lambda b:b['records'][0]['subject']['digest'].update(sha256='0'*64))
        self.reject('record target/provider')

    def test_count_and_duplicate_record_fail(self):
        self.change('rea.json.gz',lambda b:b['records'].__setitem__(0,b['records'][1]))
        self.reject('duplicate evidence')

    def test_missing_record(self):
        self.change('rea.json.gz',lambda b:b['records'].pop())
        self.reject('record counts')

    def test_procedure_inventory_is_fixed(self):
        self.change('rea.json.gz',lambda b:b['records'][0]['parameters'].update(procedure='unreviewed'))
        self.reject('procedure inventory')

    def test_source_hash_and_reused_original_equality(self):
        self.change('reused.json.gz',lambda b:b['sources'][0].update(sha256='0'*64))
        self.reject('source hash')

    def test_reused_record_cannot_be_edited(self):
        self.change('reused.json.gz',lambda b:b['records'][0].update(confidence='inferred'))
        self.reject('edited reused record')

    def test_missing_provenance(self):
        self.change('reused.json.gz',lambda b:b['sources'][0]['evidence_ids'].pop())
        self.reject('missing provenance')

    def test_source_paths_cannot_escape(self):
        for name in ('../out.json.gz','/tmp/out.json.gz'):
            with self.subTest(name=name),self.assertRaises(ValueError):source_path(ROOT,name)

    def test_manifest_is_bijective(self):
        self.change('function-manifest.json',lambda m:m.__setitem__(0,m[1]))
        self.reject('manifest bijection')

    def test_manifest_addresses_and_limitations_are_checked(self):
        self.change('function-manifest.json',lambda m:m[0].update(address='0x0'))
        self.reject('manifest address')

    def test_manifest_limitations_cannot_be_removed(self):
        self.change('function-manifest.json',lambda m:m[0].update(limitations=[]))
        self.reject('manifest limitations')

    def test_selected_instruction_even_after_hash_refresh(self):
        def mutation(b):
            r=next(r for r in b['records'] if r['parameters']['procedure']=='vm_callFunPtr')
            lines=r['normalized_result']['assembly']
            index=next(i for i,s in enumerate(lines) if s.startswith('0x809445f:'))
            lines[index]='0x809445f: INC dword ptr [EBP + 0xc]'
        self.change('rea.json.gz',mutation,refresh=True)
        self.reject('instruction site')

    def test_finding_evidence_and_classification(self):
        self.change('findings.json',lambda f:f[0].update(evidence_ids=['ev_unknown']))
        self.reject('unknown finding evidence')

    def test_observation_requires_evidence(self):
        self.change('findings.json',lambda f:f[0].update(evidence_ids=[]))
        self.reject('unsubstantiated finding')

    def test_readable_view_covers_all_records(self):
        (self.here/'functions.md').write_text('incomplete')
        self.reject('readable evidence coverage')


if __name__=='__main__':unittest.main()
