"""Exercise the actual 2AZ delivery and bounded provenance rejection rules."""
from copy import deepcopy
from pathlib import Path
import struct
import unittest
from unittest.mock import patch

import verify_evidence as v


class EvidenceTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.new = v.load(v.ROOT, v.DIRECTORY + 'rea.json.gz')['records']
        cls.reused = v.load(v.ROOT, v.DIRECTORY + 'reused.json.gz')['records']
        cls.manifest = v.load(v.ROOT, v.DIRECTORY + 'function-manifest.json')
        cls.anchors = v.load(v.ROOT, v.DIRECTORY + 'anchors.json')
        cls.findings = v.load(v.ROOT, v.DIRECTORY + 'findings.json')
        cls.cases = v.load(v.ROOT, v.DIRECTORY + 'expected-cases.json')
        cls.questions = v.load(v.ROOT, v.DIRECTORY + 'questions.json')
        cls.mapping = v.load(v.ROOT, v.DIRECTORY + 'case-map.json')
        cls.original = v.load(v.ROOT, v.PREVIOUS + 'expected-cases.json')
        cls.origins = v.load(v.ROOT, v.DIRECTORY + 'reused-manifest.json')
        cls.baseline = v.load(v.ROOT, v.DIRECTORY + 'baseline-inputs.json')
        cls.byid = {r['evidence_id']: r for r in cls.new + cls.reused}
        cls.elf = v.Elf((v.ROOT / 'magic_Linux32').read_bytes())

    def catalog(self, findings=None, cases=None, questions=None):
        v.validate_findings(self.findings if findings is None else findings,
                            self.cases if cases is None else cases,
                            self.questions if questions is None else questions,
                            self.byid, self.anchors)

    def test_actual_current_delivery_is_complete(self):
        summary = v.verify()
        self.assertEqual(summary['stage'], '2AZ')
        self.assertEqual(summary['new_records'], len(self.new))
        self.assertEqual(summary['reused_records'], len(self.reused))
        self.assertEqual(summary['case_mappings'], 94)
        self.assertEqual(summary['target_execution'], 'none')
        self.assertEqual(summary['question_status']['Q05'], 'partial')
        self.assertEqual(summary['question_status']['Q06'], 'partial')

    def test_actual_dispatch_table_matches_frozen_csv_and_elf(self):
        v.validate_dispatch_table(v.ROOT, self.elf)

    def test_dispatch_entry_mismatch_is_rejected_from_original_bytes(self):
        original = self.elf.locate
        def changed(address, size):
            offset, body = original(address, size)
            if address == 0x083cc904 and size == 28:
                body = bytearray(body)
                struct.pack_into('<I', body, 8, 0x08090ae8)
                body = bytes(body)
            return offset, body
        with patch.object(self.elf, 'locate', changed):
            with self.assertRaisesRegex(ValueError, 'dispatch entry differs from original ELF'):
                v.validate_dispatch_table(v.ROOT, self.elf)

    def test_actual_verifier_requires_this_batch_ledger(self):
        original = v.load
        def missing(root, name):
            if name == v.DIRECTORY + 'anchors.json':
                raise FileNotFoundError(name)
            return original(root, name)
        with patch.object(v, 'load', missing):
            with self.assertRaises(FileNotFoundError):
                v.verify()

    def test_real_generated_view_cannot_be_stale(self):
        original = Path.read_text
        def changed(path, *args, **kwargs):
            text = original(path, *args, **kwargs)
            return text + '\nstale view\n' if path == v.HERE / 'findings.md' else text
        with patch.object(Path, 'read_text', changed):
            with self.assertRaisesRegex(ValueError, 'generated|stale'):
                v.verify()

    def test_all_three_2ay_ledger_prefixes_are_frozen(self):
        for index in range(3):
            findings, cases, anchors = deepcopy(self.findings), deepcopy(self.cases), deepcopy(self.anchors)
            if index == 0:
                anchors[0]['bytes_hex'] = '00'
            elif index == 1:
                findings[0]['claim'] = 'rewritten historical claim'
            else:
                cases[0]['expected'] = 'rewritten historical case'
            with self.subTest(index=index), self.assertRaisesRegex(ValueError, 'historical ledger changed'):
                v.validate_delivery(v.ROOT, self.new, self.reused, findings, cases, anchors)

    def test_a_new_dossier_must_have_a_linked_finding_and_anchor(self):
        findings = deepcopy(self.findings)
        eid = self.new[0]['evidence_id']
        anchor_ids = {a['id'] for a in self.anchors if a['evidence_id'] == eid}
        for finding in findings:
            if eid in finding['evidence']:
                finding['anchors'] = [a for a in finding['anchors'] if a not in anchor_ids]
        with self.assertRaisesRegex(ValueError, 'linked finding and anchor'):
            v.validate_delivery(v.ROOT, self.new, self.reused, findings, self.cases, self.anchors)

    def test_required_gate_document_cannot_be_missing(self):
        original = Path.is_file
        def missing(path):
            return False if path == v.HERE / 'gate-review.md' else original(path)
        with patch.object(Path, 'is_file', missing):
            with self.assertRaisesRegex(ValueError, 'missing delivery document'):
                v.validate_delivery(v.ROOT, self.new, self.reused, self.findings, self.cases, self.anchors)

    def test_current_documents_cannot_cite_an_external_unretained_record(self):
        original = Path.read_text
        def changed(path, *args, **kwargs):
            text = original(path, *args, **kwargs)
            return text + '\nev_' + '0' * 64 if path == v.HERE / 'gate-review.md' else text
        with patch.object(Path, 'read_text', changed):
            with self.assertRaisesRegex(ValueError, 'document cites missing evidence'):
                v.validate_delivery(v.ROOT, self.new, self.reused, self.findings, self.cases, self.anchors)

    def test_all_94_source_cases_and_basis_are_required(self):
        mapping = deepcopy(self.mapping)
        mapping['items'].pop()
        with self.assertRaisesRegex(ValueError, 'case inventory'):
            v.validate_case_map(mapping, self.original, self.cases)
        mapping = deepcopy(self.mapping)
        mapping['items'][0]['source_basis'] = 'invented'
        with self.assertRaisesRegex(ValueError, 'case basis'):
            v.validate_case_map(mapping, self.original, self.cases)

    def test_case_mapping_cannot_target_an_absent_case(self):
        mapping = deepcopy(self.mapping)
        mapping['items'][0]['case_ids'] = ['absent']
        with self.assertRaisesRegex(ValueError, 'missing/duplicate target'):
            v.validate_case_map(mapping, self.original, self.cases)

    def test_dangling_finding_evidence_is_rejected(self):
        findings = deepcopy(self.findings)
        findings[-1]['evidence'] = ['absent']
        with self.assertRaisesRegex(ValueError, 'dangling finding evidence'):
            self.catalog(findings=findings)

    def test_unknown_cannot_be_marked_answered(self):
        findings = deepcopy(self.findings)
        next(f for f in findings if f['kind'] == 'unknown')['status'] = 'answered'
        with self.assertRaisesRegex(ValueError, 'unknown finding claimed answered'):
            self.catalog(findings=findings)

    def test_broad_exit_and_gc_parent_domains_stay_partial(self):
        for index in (4, 5):
            questions = deepcopy(self.questions)
            questions[index]['status'] = 'answered'
            questions[index]['findings'] = ['F01']
            questions[index]['subquestions'] = []
            with self.subTest(index=index), self.assertRaisesRegex(ValueError, 'excluded exit/GC'):
                self.catalog(questions=questions)

    def test_unknown_child_cannot_close_a_bounded_question(self):
        questions = deepcopy(self.questions)
        finding = next(f['id'] for f in self.findings if f['kind'] == 'unknown')
        questions[0]['subquestions'] = [dict(id='Q01.invalid-closure', status='answered',
            findings=[finding], remaining='Unresolved recovery.', next_action='Retain the gate.')]
        with self.assertRaisesRegex(ValueError, 'unresolved subquestion'):
            self.catalog(questions=questions)

    def test_static_cases_cannot_claim_execution(self):
        cases = deepcopy(self.cases)
        cases[-1]['runtime_executed'] = True
        with self.assertRaisesRegex(ValueError, 'runtime execution'):
            self.catalog(cases=cases)

    def test_reuse_source_hash_cannot_escape_the_frozen_baseline(self):
        origins = deepcopy(self.origins)
        origins[0]['source_sha256'] = '0' * 64
        with self.assertRaisesRegex(ValueError, 'not a frozen baseline'):
            v.validate_frozen_origins(self.reused, origins, v.ROOT, self.baseline)

    def test_wrong_target_and_missing_raw_dossier_are_rejected(self):
        for field, value, message in [('subject', None, 'wrong target'),
                                      ('raw_result', None, 'incomplete evidence')]:
            record = deepcopy(self.new[0])
            manifest = v.describe(record, self.elf, 'rea.json.gz')
            if field == 'subject':
                record['subject']['digest']['sha256'] = '0' * 64
            else:
                record[field] = value
            with self.subTest(field=field), self.assertRaisesRegex(ValueError, message):
                v.validate_records([record], [manifest], self.elf)

    def test_new_and_reused_bundles_cannot_overlap(self):
        with self.assertRaisesRegex(ValueError, 'overlap'):
            v.validate_bundles(self.new, self.new, self.manifest, self.elf)

    def test_session_requires_matching_closed_static_requests(self):
        session = v.load(v.ROOT, v.DIRECTORY + 'session.json')
        altered = deepcopy(session)
        altered['analysis_requests'] = []
        with self.assertRaisesRegex(ValueError, 'request ledger'):
            v.validate_session(altered, self.new)
        for key, value, message in [('target_execution', 'native', 'execution'),
                                    ('target_sha256', '0' * 64, 'target differs'),
                                    ('after', {'open': True}, 'lifecycle')]:
            altered = deepcopy(session)
            altered[key] = value
            with self.subTest(key=key), self.assertRaisesRegex(ValueError, message):
                v.validate_session(altered, self.new)


if __name__ == '__main__':
    unittest.main()
