"""Corrupt temporary audit inputs; never alter repository fixtures or reference files."""
import copy
import tempfile
from pathlib import Path
import unittest
import audit_rules as rules


class GapAuditTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        self.root = Path(self.temp.name)
        (self.root/'source.rs').write_text('fn work() {}\n')
        (self.root/'evidence.md').write_text('retained static evidence')
        ref = dict(path='source.rs', line=1, excerpt='fn work() {}')
        common = dict(sources=[ref], evidence=['evidence.md'], tests=['adds'])
        self.gaps = dict(claim='bounded', gaps=[dict(id='G01', title='gap', domain='value',
            status='rejected', stage=2, current_behavior='bounded', unknowns=['native contract'],
            depends_on=[], exit_criteria='recover and test', **copy.deepcopy(common))])
        self.caps = dict(stage2_complete=False, capabilities=[dict(id='C01', status='open',
            gap_ids=['G01'], exit_criteria='resolve gap', **copy.deepcopy(common))])
        self.rows = [dict(sources=[ref], excluded_cases=['missing values'],
            exclusion_links=[dict(text='missing values', gap_ids=['G01'])], assertion_review='human review')]
        self.priorities = dict(selected='P01', candidates=[dict(id=f'P{i:02}', rank=i,
            next_action='evidence', reason='dependency isolation', readiness='needs evidence',
            static_relevance='instruction only', gap_ids=['G01']) for i in range(1, 4)])

    def check(self):
        return rules.validate(self.gaps, self.caps, self.priorities, self.rows, self.root, {'adds'})

    def test_valid_bounded_catalog(self):
        self.assertEqual(self.check()['selected_next'], 'P01')

    def test_duplicate_gap_id(self):
        self.gaps['gaps'] *= 2
        with self.assertRaisesRegex(ValueError, 'duplicate'): self.check()

    def test_dangling_dependency(self):
        self.gaps['gaps'][0]['depends_on'] = ['absent']
        with self.assertRaisesRegex(ValueError, 'dangling dependency'): self.check()

    def test_self_dependency(self):
        self.gaps['gaps'][0]['depends_on'] = ['G01']
        with self.assertRaisesRegex(ValueError, 'cyclic'): self.check()

    def test_multiple_gap_cycle(self):
        g = copy.deepcopy(self.gaps['gaps'][0]); g['id']='G02'; g['depends_on']=['G01']
        self.gaps['gaps'][0]['depends_on']=['G02']; self.gaps['gaps'].append(g)
        with self.assertRaisesRegex(ValueError, 'cyclic'): self.check()

    def test_later_stage_is_not_hidden_stage2_prerequisite(self):
        g = copy.deepcopy(self.gaps['gaps'][0]);g['id']='G02';g['stage']=3
        self.gaps['gaps'].append(g);self.gaps['gaps'][0]['depends_on']=['G02']
        with self.assertRaisesRegex(ValueError, 'later stage'): self.check()

    def test_exclusion_text_must_be_mapped_exactly(self):
        self.rows[0]['excluded_cases'].append('unmapped')
        with self.assertRaisesRegex(ValueError, 'unmapped exclusion'): self.check()

    def test_exclusion_requires_nonempty_gap_references(self):
        self.rows[0]['exclusion_links'][0]['gap_ids']=[]
        with self.assertRaisesRegex(ValueError, 'dangling exclusion'): self.check()

    def test_unknown_exclusion_gap(self):
        self.rows[0]['exclusion_links'][0]['gap_ids']=['absent']
        with self.assertRaisesRegex(ValueError, 'dangling exclusion'): self.check()

    def test_gap_requires_capability_gate(self):
        self.caps['capabilities']=[]
        with self.assertRaisesRegex(ValueError, 'without capability'): self.check()

    def test_stage2_gap_cannot_be_reclassified_outside_gate(self):
        self.caps['capabilities'][0]['status']='outside_stage2'
        with self.assertRaisesRegex(ValueError, 'hidden outside'): self.check()

    def test_later_stage_cannot_be_stage2_gate(self):
        self.gaps['gaps'][0]['stage']=3
        with self.assertRaisesRegex(ValueError, 'later-stage gap'): self.check()

    def test_unknown_test_is_rejected(self):
        self.gaps['gaps'][0]['tests']=['helper']
        with self.assertRaisesRegex(ValueError, 'unknown Rust test'): self.check()

    def test_evidence_must_exist(self):
        self.gaps['gaps'][0]['evidence']=['absent.md']
        with self.assertRaisesRegex(ValueError, 'missing file'): self.check()

    def test_path_cannot_escape_root(self):
        for name in ['../evidence.md', '/tmp/evidence.md']:
            with self.subTest(name=name), self.assertRaisesRegex(ValueError, 'invalid local path'):
                rules.local_file(self.root, name)

    def test_source_line_and_excerpt_are_checked(self):
        for line, excerpt in [(2, 'fn work() {}'), (1, 'fn another() {}')]:
            self.gaps['gaps'][0]['sources']=[dict(path='source.rs', line=line, excerpt=excerpt)]
            with self.subTest(line=line), self.assertRaisesRegex(ValueError, 'stale source'): self.check()

    def test_complete_compatibility_claim_is_rejected(self):
        self.caps['stage2_complete']=True
        with self.assertRaisesRegex(ValueError, 'complete compatibility'): self.check()

    def test_missing_exit_condition(self):
        self.gaps['gaps'][0]['exit_criteria']=''
        with self.assertRaisesRegex(ValueError, 'empty gap field'): self.check()

    def test_ambiguous_or_noncontiguous_priority_is_rejected(self):
        self.priorities['selected']='P02'
        with self.assertRaisesRegex(ValueError, 'ambiguous'): self.check()
        self.priorities['selected']='P01'; self.priorities['candidates'][1]['rank']=1
        with self.assertRaisesRegex(ValueError, 'priority order'): self.check()

    def test_insufficient_evidence_is_not_implementation_ready(self):
        self.priorities['candidates'][0]['next_action']='implement'
        with self.assertRaisesRegex(ValueError, 'readiness'): self.check()

    def test_hash_verification_detects_stale_input(self):
        import hashlib
        before=hashlib.sha256((self.root/'source.rs').read_bytes()).hexdigest()
        rules.verify_hashes(self.root, {'source.rs':before})
        (self.root/'source.rs').write_text('changed')
        with self.assertRaisesRegex(ValueError, 'hash mismatch'):
            rules.verify_hashes(self.root, {'source.rs':before})

    def test_change_ledger_cannot_rewrite_history_or_current_review(self):
        before=[dict(opcode='01',excluded_cases=['old'],related_tests=['adds'],implemented_cases=['bounded'])]
        after=[dict(opcode='01',excluded_cases=['new'],related_tests=['adds'],implemented_cases=['bounded'])]
        valid=[dict(opcode='01',previous_exclusions=['old'],current_exclusions=['new'],
            previous_tests=['adds'],current_tests=['adds'],previous_implemented_cases=['bounded'],
            current_implemented_cases=['bounded'],reason='reviewed')]
        rules.validate_changes(valid,before,after)
        for field in ['previous_exclusions','current_exclusions','previous_tests','current_tests',
                      'previous_implemented_cases','current_implemented_cases']:
            changed=copy.deepcopy(valid);changed[0][field]=['fabricated']
            with self.subTest(field=field),self.assertRaisesRegex(ValueError,'differs'):
                rules.validate_changes(changed,before,after)
        with self.assertRaisesRegex(ValueError,'partition'):
            rules.validate_changes([],before,after)

    def test_test_registry_excludes_unattributed_helpers(self):
        (self.root/'src').mkdir()
        (self.root/'src/lib.rs').write_text('fn helper() {}\n#[test]\nfn actual() { assert!(true); }')
        self.assertEqual(set(rules.test_registry(self.root)), {'actual'})


if __name__ == '__main__':
    unittest.main()
