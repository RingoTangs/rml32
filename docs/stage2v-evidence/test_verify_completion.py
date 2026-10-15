"""Synthetic audit-rule tests; no original artifact or business program execution."""
import copy
import importlib.util
from pathlib import Path
import tempfile
import unittest
import sys
sys.dont_write_bytecode=True
spec=importlib.util.spec_from_file_location('audit',Path(__file__).with_name('verify_completion.py'))
audit=importlib.util.module_from_spec(spec)
spec.loader.exec_module(audit)


class AuditTests(unittest.TestCase):
    def setUp(self):
        self.temp=tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        self.root=Path(self.temp.name)
        (self.root/'evidence.md').write_text('synthetic evidence')
        self.rows=[dict(opcode='01',semantic_status='bounded',implemented_cases=['i32 add'],excluded_cases=['string add'],related_tests=['adds'],evidence=['evidence.md'],dependencies=['value model'],exit_criteria='type-pair tests'),dict(opcode='08',semantic_status='unimplemented',implemented_cases=[],excluded_cases=['all runtime semantics'],related_tests=[],evidence=['evidence.md'],dependencies=['lookup'],exit_criteria='implement with tests')]
    def check(self,rows=None):
        audit.validate_review(self.rows if rows is None else rows,{1,8},{1},{'adds'},self.root)
    def test_valid_rows_preserve_bounded_and_unimplemented_distinction(self):
        self.check()
    def test_missing_union_member_is_rejected(self):
        with self.assertRaisesRegex(ValueError,'partition'):self.check(self.rows[:1])
    def test_duplicate_union_member_is_rejected(self):
        with self.assertRaisesRegex(ValueError,'partition'):self.check(self.rows+[copy.deepcopy(self.rows[0])])
    def test_implemented_cannot_claim_complete(self):
        self.rows[0]['semantic_status']='complete'
        with self.assertRaisesRegex(ValueError,'status'):self.check()
    def test_absent_runtime_cannot_claim_support(self):
        self.rows[1]['semantic_status']='bounded'
        with self.assertRaisesRegex(ValueError,'status'):self.check()
    def test_bounded_row_requires_test(self):
        self.rows[0]['related_tests']=[]
        with self.assertRaisesRegex(ValueError,'related tests'):self.check()
    def test_non_test_helper_name_is_rejected(self):
        self.rows[0]['related_tests']=['helper']
        with self.assertRaisesRegex(ValueError,'unknown Rust test'):self.check()
    def test_unimplemented_row_cannot_claim_semantic_test(self):
        self.rows[1]['related_tests']=['adds']
        with self.assertRaisesRegex(ValueError,'cannot claim'):self.check()
    def test_unimplemented_row_cannot_claim_cases(self):
        self.rows[1]['implemented_cases']=['lookup']
        with self.assertRaisesRegex(ValueError,'cannot claim'):self.check()
    def test_exclusions_and_dependencies_and_exit_are_required(self):
        for field in ['excluded_cases','dependencies','exit_criteria']:
            rows=copy.deepcopy(self.rows); rows[0][field]=[]
            with self.subTest(field=field),self.assertRaisesRegex(ValueError,'boundaries'):self.check(rows)
    def test_missing_and_traversing_evidence_is_rejected(self):
        for evidence in ['absent.md','../evidence.md','/tmp/evidence.md']:
            self.rows[0]['evidence']=[evidence]
            with self.subTest(evidence=evidence),self.assertRaisesRegex(ValueError,'invalid evidence'):self.check()
    def test_evidence_is_required(self):
        self.rows[0]['evidence']=[]
        with self.assertRaisesRegex(ValueError,'missing evidence'):self.check()
    def test_supported_definition_not_unrelated_hex_literals(self):
        text='// 0xff\npub const SUPPORTED_OPCODES: &[u8] = &[0x1, 0x08];\n// 0xaa'
        self.assertEqual(audit.supported(text),{1,8})
    def test_duplicate_or_missing_supported_definition_fails(self):
        for text in ['nothing','pub const SUPPORTED_OPCODES: &[u8] = &[0x1, 0x1];']:
            with self.subTest(text=text),self.assertRaises(ValueError):audit.supported(text)
    def test_count_includes_init_hidden_and_ordinary_without_running(self):
        counts,functions=audit.count_programs([dict(init={'nodes':[[0,0,46],[0,0,72]]},functions=[{'code':{'nodes':[[0,0,1],[0,0,1]]}},{'code':None}],hidden=[{'code':{'nodes':[[0,0,8]]}}])])
        self.assertEqual(dict(counts),{46:1,72:1,1:2,8:1});self.assertEqual(functions,2)
    def test_malformed_node_fails(self):
        with self.assertRaisesRegex(ValueError,'malformed'):audit.count_programs([dict(init={'nodes':[[0,1]]},functions=[],hidden=[])])
    def test_only_test_attributed_functions_are_evidence(self):
        self.assertEqual(audit.test_names('fn helper() {}\n#[test]\nfn adds() {}'),{'adds'})
    def test_hash_changes_when_source_changes(self):
        p=self.root/'evidence.md'; old=audit.digest(p);p.write_text('changed');self.assertNotEqual(old,audit.digest(p))


if __name__=='__main__':unittest.main()
