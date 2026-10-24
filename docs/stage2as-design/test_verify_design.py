"""Catalog/provenance negative tests mutate only copied metadata or temporary files."""
import copy
from pathlib import Path
import tempfile
import unittest
import verify_design as v


class DesignTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.decisions=v.load(v.ROOT,'docs/stage2as-design/decisions.json')['decisions']
        cls.apis=v.load(v.ROOT,'docs/stage2as-design/api-migration.json')['items']
        cls.mapping=v.load(v.ROOT,'docs/stage2as-design/case-map.json')['items']
        cls.plan=v.load(v.ROOT,'docs/stage2as-design/implementation-plan.json')
        cls.ar_cases=v.load(v.ROOT,'docs/stage2ar-evidence/expected-cases.json')
        cls.ar_findings={f['id'] for f in v.load(v.ROOT,'docs/stage2ar-evidence/findings.json')}

    def setUp(self):
        self.d=copy.deepcopy(self.decisions);self.a=copy.deepcopy(self.apis)
        self.m=copy.deepcopy(self.mapping);self.p=copy.deepcopy(self.plan)

    def check(self):v.validate(self.d,self.a,self.m,self.p,self.ar_cases,self.ar_findings,v.test_registry(v.ROOT),v.ROOT)

    def test_valid_catalog_preserves_native_unknowns(self):self.check()

    def test_decision_inventory_cannot_shrink(self):
        self.d.pop()
        with self.assertRaisesRegex(ValueError,'decision inventory'):self.check()

    def test_design_cannot_be_relabelled_native_observation(self):
        self.d[0]['authority']='new native observation'
        with self.assertRaisesRegex(ValueError,'decision status'):self.check()

    def test_unknown_ar_finding_is_rejected(self):
        self.d[0]['ar_findings']=['absent']
        with self.assertRaisesRegex(ValueError,'dangling finding'):self.check()

    def test_current_source_excerpt_is_checked(self):
        self.d[0]['sources'][0]['excerpt']='invented source'
        with self.assertRaisesRegex(ValueError,'stale source'):self.check()

    def test_api_inventory_cannot_omit_callback(self):
        self.a.pop()
        with self.assertRaisesRegex(ValueError,'API inventory'):self.check()

    def test_observation_cannot_implicitly_create_owner(self):
        self.a[0]['owner_effect']='new-host-root'
        with self.assertRaisesRegex(ValueError,'implicit owner'):self.check()

    def test_case_map_must_partition_all_51_cases(self):
        self.m.pop()
        with self.assertRaisesRegex(ValueError,'partition'):self.check()

    def test_case_payload_digest_is_checked(self):
        self.m[0]['case_sha256']='0'*64
        with self.assertRaisesRegex(ValueError,'source changed'):self.check()

    def test_case_file_digest_is_checked(self):
        self.m[0]['source_sha256']='0'*64
        with self.assertRaisesRegex(ValueError,'source changed'):self.check()

    def test_native_unknown_cannot_be_promoted_by_model(self):
        row=next(m for m in self.m if m['ar_basis']=='unresolved');row['coverage']='modeled';row['tests']=self.m[0]['tests']
        with self.assertRaisesRegex(ValueError,'unknown hidden'):self.check()

    def test_modeled_case_requires_real_test(self):
        self.m[0]['tests']=[]
        with self.assertRaisesRegex(ValueError,'lacks tests'):self.check()

    def test_helper_name_is_not_a_test(self):
        self.m[0]['tests']=['put']
        with self.assertRaisesRegex(ValueError,'unknown model test'):self.check()

    def test_model_cannot_claim_runtime_implementation(self):
        self.p['runtime_changed']=True
        with self.assertRaisesRegex(ValueError,'implementation claim'):self.check()

    def test_batch_cannot_depend_on_later_batch(self):
        self.p['batches'][0]['depends_on']=['T04']
        with self.assertRaisesRegex(ValueError,'batch dependency'):self.check()

    def test_non_scalar_gate_cannot_be_removed(self):
        self.p['batches'][3]['status']='specified'
        with self.assertRaisesRegex(ValueError,'capture gate'):self.check()

    def test_native_lifecycle_batch_stays_blocked(self):
        self.p['batches'][4]['status']='specified'
        with self.assertRaisesRegex(ValueError,'capture gate'):self.check()

    def test_stale_generated_view_is_rejected(self):
        with tempfile.TemporaryDirectory() as name:
            root=Path(name);(root/'view.md').write_text('old')
            with self.assertRaisesRegex(ValueError,'stale generated'):v.check_outputs({'view.md':'new'},root)

    def test_path_traversal_is_rejected(self):
        for name in ['../escape','/tmp/escape','x\\y']:
            with self.subTest(name=name),self.assertRaisesRegex(ValueError,'unsafe path'):v.safe_path(v.ROOT,name)

    def test_symlink_escape_is_rejected(self):
        with tempfile.TemporaryDirectory() as name:
            root=Path(name);(root/'outside').symlink_to('/tmp')
            with self.assertRaisesRegex(ValueError,'escapes root'):v.safe_path(root,'outside/file')


if __name__=='__main__':unittest.main()
