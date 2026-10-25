"""Exercise evidence rejection using frozen inputs and temporary copies only."""
import copy
import gzip
import tempfile
import unittest
from pathlib import Path

import verify_evidence as v


class EvidenceTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.elf = v.Elf((v.ROOT / 'magic_Linux32').read_bytes())
        directory = 'docs/stage2ar-evidence/'
        cls.new = v.load(v.ROOT, directory + 'rea.json.gz')['records']
        cls.reused = v.load(v.ROOT, directory + 'reused.json.gz')['records']
        cls.manifest = v.load(v.ROOT, directory + 'function-manifest.json')
        cls.byid = {r['evidence_id']: r for r in cls.new + cls.reused}
        cls.anchors = v.load(v.ROOT, directory + 'anchors.json')
        cls.findings = v.load(v.ROOT, directory + 'findings.json')
        cls.cases = v.load(v.ROOT, directory + 'expected-cases.json')
        cls.questions = v.load(v.ROOT, directory + 'questions.json')

    def setUp(self):
        self.record = copy.deepcopy(self.new[0])
        self.item = v.describe(self.record, self.elf, 'rea.json.gz')

    def check_record(self):
        return v.validate_records([self.record], [self.item], self.elf)

    def catalog(self, findings=None, cases=None, questions=None):
        v.validate_findings(self.findings if findings is None else findings,
                            self.cases if cases is None else cases,
                            self.questions if questions is None else questions,
                            self.byid, self.anchors)

    def mapping(self):
        return {'items': [dict(source_case_id=c['id'], source_basis=c['basis'],
                               source_findings=c['findings'], case_ids=[c['id']],
                               status='unchanged-boundary', limitations=c['boundary'])
                          for c in self.cases]}

    def session(self):
        return dict(before={'open': False}, after={'open': False}, target_execution='none',
                    analysis_requests=[dict(procedure=self.record['parameters']['procedure'],
                                            status='complete', evidence_id=self.record['evidence_id'])])

    def test_valid_frozen_sources_preserve_noncontiguous_body(self):
        self.check_record()
        self.assertGreater(len(self.item['analyzed_ranges']), 1)
        v.validate_bundles(self.new, self.reused, self.manifest, self.elf)
        self.catalog()

    def test_wrong_artifact_is_rejected(self):
        self.record['subject']['digest']['sha256'] = '0' * 64
        with self.assertRaisesRegex(ValueError, 'wrong target'):
            self.check_record()

    def test_runtime_authority_cannot_impersonate_static_evidence(self):
        self.record['authority'] = 'controlled-replay'
        with self.assertRaisesRegex(ValueError, 'observed static artifact'):
            self.check_record()

    def test_missing_manifest_record_is_rejected(self):
        with self.assertRaisesRegex(ValueError, 'partition'):
            v.validate_records([self.record], [], self.elf)

    def test_changed_analyzed_range_is_rejected(self):
        self.item['analyzed_ranges'][0]['end'] = '0x1'
        with self.assertRaisesRegex(ValueError, 'range/digest'):
            self.check_record()

    def test_missing_raw_result_is_rejected(self):
        self.record['raw_result'] = None
        with self.assertRaisesRegex(ValueError, 'incomplete evidence'):
            self.check_record()

    def test_bundle_partition_cannot_relabel_reused_record(self):
        manifest = copy.deepcopy(self.manifest)
        manifest[0]['bundle'] = 'reused.json.gz'
        with self.assertRaisesRegex(ValueError, 'bundle attribution'):
            v.validate_bundles(self.new, self.reused, manifest, self.elf)

    def test_new_and_reused_cannot_overlap(self):
        with self.assertRaisesRegex(ValueError, 'overlap'):
            v.validate_bundles(self.new, self.new, self.manifest, self.elf)

    def test_new_primitive_cannot_be_called_full_dossier(self):
        primitive = next(r for r in self.reused if r['operation'] == 'procedure_assembly')
        with self.assertRaisesRegex(ValueError, 'full dossier'):
            v.validate_bundles([primitive], self.new, self.manifest, self.elf)

    def test_anchor_range_and_original_bytes_are_checked(self):
        for field, value, message in [('address', '0x1', 'outside function'),
                                      ('bytes_hex', '00', 'bytes differ'),
                                      ('evidence_id', 'absent', 'dangling anchor')]:
            anchors = copy.deepcopy(self.anchors)
            anchors[0][field] = value
            with self.subTest(field=field), self.assertRaisesRegex(ValueError, message):
                v.validate_anchors(anchors, self.byid, self.elf)

    def test_dangling_finding_evidence_is_rejected(self):
        findings = copy.deepcopy(self.findings)
        findings[0]['evidence'] = ['absent']
        with self.assertRaisesRegex(ValueError, 'dangling finding evidence'):
            self.catalog(findings=findings)

    def test_static_case_cannot_claim_execution(self):
        cases = copy.deepcopy(self.cases)
        cases[0]['runtime_executed'] = True
        with self.assertRaisesRegex(ValueError, 'runtime execution'):
            self.catalog(cases=cases)

    def test_unknown_only_case_cannot_claim_static_result(self):
        cases = copy.deepcopy(self.cases)
        cases[0]['findings'] = [next(f['id'] for f in self.findings if f['kind'] == 'unknown')]
        with self.assertRaisesRegex(ValueError, 'observational basis'):
            self.catalog(cases=cases)

    def test_unknown_parent_cannot_claim_answered(self):
        questions = copy.deepcopy(self.questions)
        questions[0]['status'] = 'answered'
        with self.assertRaisesRegex(ValueError, 'unresolved question'):
            self.catalog(questions=questions)

    def test_out_of_scope_exit_domains_cannot_be_closed_by_selected_findings(self):
        questions = copy.deepcopy(self.questions)
        questions[4]['status'] = 'answered'
        questions[4]['findings'] = ['F01']
        with self.assertRaisesRegex(ValueError, 'excluded exit/GC domains'):
            self.catalog(questions=questions)

    def test_subquestion_can_answer_a_bounded_observed_contact(self):
        questions = copy.deepcopy(self.questions)
        questions[0]['subquestions'] = [dict(id='Q01.transfer', status='answered', findings=['F03'],
                                            remaining='Other exits remain in the parent question.',
                                            next_action='Preserve bounded transfer semantics.')]
        self.catalog(questions=questions)

    def test_unknown_subquestion_cannot_claim_answered(self):
        questions = copy.deepcopy(self.questions)
        questions[0]['subquestions'] = [dict(id='Q01.unpublished', status='answered', findings=['F20'],
                                            remaining='Final cleanup unknown.', next_action='Trace cleanup.')]
        with self.assertRaisesRegex(ValueError, 'unresolved subquestion'):
            self.catalog(questions=questions)

    def test_unresolved_boundaries_cannot_disappear(self):
        with self.assertRaisesRegex(ValueError, 'unresolved boundary'):
            self.catalog(cases=[c for c in self.cases if c['basis'] != 'unresolved'])

    def test_original_case_mapping_is_complete_and_preserves_basis(self):
        mapping = self.mapping()
        v.validate_case_map(mapping, self.cases, self.cases)
        mapping['items'][0]['source_basis'] = 'invented'
        with self.assertRaisesRegex(ValueError, 'original case basis'):
            v.validate_case_map(mapping, self.cases, self.cases)

    def test_original_case_mapping_cannot_drop_case(self):
        mapping = self.mapping()
        mapping['items'].pop()
        with self.assertRaisesRegex(ValueError, 'case inventory'):
            v.validate_case_map(mapping, self.cases, self.cases)

    def test_case_mapping_cannot_target_missing_case(self):
        mapping = self.mapping()
        mapping['items'][0]['case_ids'] = ['absent']
        with self.assertRaisesRegex(ValueError, 'missing/duplicate target'):
            v.validate_case_map(mapping, self.cases, self.cases)

    def test_reuse_source_tampering_is_rejected(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            raw = gzip.compress(v.canonical({'records': [self.record]}), mtime=0)
            (root / 'old.json.gz').write_bytes(raw)
            origin = dict(source='old.json.gz', source_sha256=v.digest(raw),
                          record_sha256=v.digest(v.canonical(self.record)),
                          evidence_id=self.record['evidence_id'],
                          name=self.record['parameters']['procedure'], operation=self.record['operation'])
            v.validate_origins([self.record], [origin], root)
            (root / 'old.json.gz').write_bytes(gzip.compress(b'{"records":[]}', mtime=0))
            with self.assertRaisesRegex(ValueError, 'source changed'):
                v.validate_origins([self.record], [origin], root)

    def test_reuse_must_name_a_frozen_baseline_artifact(self):
        origin = dict(source='new.json.gz', source_sha256='0' * 64)
        with self.assertRaisesRegex(ValueError, 'not a frozen baseline'):
            v.validate_frozen_origins([], [origin], v.ROOT, {'files': {}})

    def test_reuse_digest_cannot_replace_frozen_baseline_digest(self):
        origin = dict(source='old.json.gz', source_sha256='0' * 64)
        with self.assertRaisesRegex(ValueError, 'not a frozen baseline'):
            v.validate_frozen_origins([], [origin], v.ROOT, {'files': {'old.json.gz': '1' * 64}})

    def test_session_requires_a_closed_target_and_static_scope(self):
        session = self.session()
        v.validate_session(session, [self.record])
        for field, value, message in [('after', {'open': True}, 'lifecycle'),
                                      ('target_execution', 'native', 'execution')]:
            altered = copy.deepcopy(session)
            altered[field] = value
            with self.subTest(field=field), self.assertRaisesRegex(ValueError, message):
                v.validate_session(altered, [self.record])

    def test_new_dossier_requires_completed_request(self):
        session = self.session()
        session['analysis_requests'][0] = dict(procedure=self.record['parameters']['procedure'],
                                             status='transport-limited', reason='Recorded transport notice.')
        with self.assertRaisesRegex(ValueError, 'lacks completed request'):
            v.validate_session(session, [self.record])

    def test_request_cannot_claim_an_unrelated_procedure(self):
        session = self.session()
        session['analysis_requests'][0]['procedure'] = 'not_the_function'
        with self.assertRaisesRegex(ValueError, 'procedure differs'):
            v.validate_session(session, [self.record])


if __name__ == '__main__':
    unittest.main()
