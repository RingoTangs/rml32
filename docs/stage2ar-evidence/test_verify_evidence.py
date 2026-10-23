"""Corrupt in-memory records or temporary files; never alter original fixtures."""
import copy
import gzip
import json
from pathlib import Path
import tempfile
import unittest

import verify_evidence as v
from render import check_outputs


class EvidenceTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.elf=v.Elf((v.ROOT/'magic_Linux32').read_bytes())
        cls.record=v.load(v.ROOT,'docs/stage2ar-evidence/rea.json.gz')['records'][0]
        cls.manifest=v.describe(cls.record,cls.elf,'rea.json.gz')
        cls.all_records=[r for b in ['rea.json.gz','reused.json.gz'] for r in v.load(v.ROOT,'docs/stage2ar-evidence/'+b)['records']]
        cls.byid={r['evidence_id']:r for r in cls.all_records}
        cls.anchors=v.load(v.ROOT,'docs/stage2ar-evidence/anchors.json')
        cls.findings=v.load(v.ROOT,'docs/stage2ar-evidence/findings.json')
        cls.cases=v.load(v.ROOT,'docs/stage2ar-evidence/expected-cases.json')
        cls.questions=v.load(v.ROOT,'docs/stage2ar-evidence/questions.json')

    def setUp(self):
        self.r=copy.deepcopy(self.record);self.m=copy.deepcopy(self.manifest)

    def records(self,records=None,manifest=None):
        return v.validate_records([self.r] if records is None else records,[self.m] if manifest is None else manifest,self.elf)

    def catalog(self,findings=None,cases=None,questions=None):
        v.validate_findings(self.findings if findings is None else findings,self.cases if cases is None else cases,
                            self.questions if questions is None else questions,self.byid,self.anchors)

    def test_valid_record_preserves_noncontiguous_ranges(self):
        self.records();self.assertEqual(len(self.m['analyzed_ranges']),2)

    def test_duplicate_evidence_is_rejected(self):
        with self.assertRaisesRegex(ValueError,'duplicate Evidence'):self.records([self.r,self.r],[self.m,self.m])

    def test_missing_manifest_record_is_rejected(self):
        with self.assertRaisesRegex(ValueError,'partition'):self.records(manifest=[])

    def test_wrong_artifact_identity_is_rejected(self):
        self.r['subject']['digest']['sha256']='0'*64
        with self.assertRaisesRegex(ValueError,'wrong target'):self.records()

    def test_wrong_architecture_is_rejected(self):
        self.r['subject']['architecture']='x86_64'
        with self.assertRaisesRegex(ValueError,'wrong target'):self.records()

    def test_wrong_provider_is_rejected(self):
        self.r['provider']['version']='unknown'
        with self.assertRaisesRegex(ValueError,'wrong provider'):self.records()

    def test_record_digest_cannot_be_rewritten(self):
        self.m['record_sha256']='0'*64
        with self.assertRaisesRegex(ValueError,'digest differs'):self.records()

    def test_analyzed_body_range_is_checked(self):
        self.m['analyzed_ranges'][0]['end']='0x808bfb5'
        with self.assertRaisesRegex(ValueError,'range/digest'):self.records()

    def test_raw_evidence_cannot_disappear(self):
        self.r['raw_result']=None
        with self.assertRaisesRegex(ValueError,'incomplete evidence'):self.records()

    def test_symbol_extent_cannot_change(self):
        self.m['size']+=1
        with self.assertRaisesRegex(ValueError,'identity/range'):self.records()

    def test_anchor_original_bytes_are_checked(self):
        a=copy.deepcopy(self.anchors);a[0]['bytes_hex']='00'
        with self.assertRaisesRegex(ValueError,'bytes differ'):v.validate_anchors(a,self.byid,self.elf)

    def test_anchor_instruction_must_match_record(self):
        a=copy.deepcopy(self.anchors);a[0]['instruction']='invented'
        with self.assertRaisesRegex(ValueError,'instruction differs'):v.validate_anchors(a,self.byid,self.elf)

    def test_anchor_cannot_leave_symbol(self):
        a=copy.deepcopy(self.anchors);a[0]['address']='0x1'
        with self.assertRaisesRegex(ValueError,'outside function'):v.validate_anchors(a,self.byid,self.elf)

    def test_anchor_cannot_reference_absent_evidence(self):
        a=copy.deepcopy(self.anchors);a[0]['evidence_id']='missing'
        with self.assertRaisesRegex(ValueError,'dangling anchor'):v.validate_anchors(a,self.byid,self.elf)

    def test_duplicate_anchor_id_is_rejected(self):
        with self.assertRaisesRegex(ValueError,'duplicate'):v.validate_anchors([self.anchors[0]]*2,self.byid,self.elf)

    def test_valid_findings_cases_and_question_states(self):
        self.catalog()

    def test_finding_requires_real_evidence(self):
        f=copy.deepcopy(self.findings);f[0]['evidence']=['missing']
        with self.assertRaisesRegex(ValueError,'dangling finding evidence'):self.catalog(findings=f)

    def test_finding_requires_matching_anchor_provenance(self):
        f=copy.deepcopy(self.findings);f[0]['evidence']=[self.anchors[-1]['evidence_id']]
        with self.assertRaisesRegex(ValueError,'provenance mismatch'):self.catalog(findings=f)

    def test_case_cannot_claim_native_execution(self):
        c=copy.deepcopy(self.cases);c[0]['runtime_executed']=True
        with self.assertRaisesRegex(ValueError,'runtime execution'):self.catalog(cases=c)

    def test_case_requires_known_finding(self):
        c=copy.deepcopy(self.cases);c[0]['findings']=['absent']
        with self.assertRaisesRegex(ValueError,'dangling expected'):self.catalog(cases=c)

    def test_design_only_case_cannot_claim_static_result(self):
        c=copy.deepcopy(self.cases);c[0]['findings']=['F21']
        with self.assertRaisesRegex(ValueError,'observational basis'):self.catalog(cases=c)

    def test_unknown_question_cannot_claim_answered(self):
        q=copy.deepcopy(self.questions);q[0]['status']='answered'
        with self.assertRaisesRegex(ValueError,'unresolved question'):self.catalog(questions=q)

    def test_question_inventory_cannot_omit_graph_boundary(self):
        with self.assertRaisesRegex(ValueError,'question inventory'):self.catalog(questions=self.questions[:-1])

    def test_unresolved_cases_cannot_disappear(self):
        c=[x for x in self.cases if x['basis']!='unresolved']
        with self.assertRaisesRegex(ValueError,'unresolved boundary'):self.catalog(cases=c)

    def test_reused_record_must_exist_in_original_full_source(self):
        with tempfile.TemporaryDirectory() as name:
            root=Path(name);raw=gzip.compress(v.canonical(dict(records=[self.r])),mtime=0);(root/'old.json.gz').write_bytes(raw)
            origin=dict(source='old.json.gz',source_sha256=v.digest(raw),record_sha256=v.digest(v.canonical(self.r)),name=self.r['parameters']['procedure'],operation=self.r['operation'],evidence_id=self.r['evidence_id'])
            v.validate_origins([self.r],[origin],root)
            (root/'old.json.gz').write_bytes(gzip.compress(b'{"records":[]}',mtime=0))
            with self.assertRaisesRegex(ValueError,'source changed'):v.validate_origins([self.r],[origin],root)

    def test_path_traversal_and_symlink_escape_are_rejected(self):
        with tempfile.TemporaryDirectory() as name:
            root=Path(name)
            for path in ['../old','/tmp/old','back\\slash']:
                with self.subTest(path=path),self.assertRaisesRegex(ValueError,'unsafe source'):v.safe_path(root,path)
            (root/'escape').symlink_to('/tmp')
            with self.assertRaisesRegex(ValueError,'escapes root'):v.safe_path(root,'escape/old')

    def test_stale_generated_view_is_rejected(self):
        with tempfile.TemporaryDirectory() as name:
            root=Path(name);(root/'view.md').write_text('old')
            with self.assertRaisesRegex(ValueError,'stale generated'):check_outputs({'view.md':'new'},root)


if __name__=='__main__':unittest.main()
