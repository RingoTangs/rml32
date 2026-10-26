import unittest
from source_projection import production,tokens,test_bodies
class SourceTests(unittest.TestCase):
    def test_fields_and_statements_removed(self):
        self.assertEqual(production('struct A { #[cfg(test)] x: Option<u32>, y:u32 } fn f(){ #[cfg(test)] probe(); go(); }'),tokens('struct A {y:u32} fn f(){go();}'))
    def test_blocks_and_literals(self):
        self.assertEqual(production('fn f(){#[cfg(test)] if flag {probe("}");} go();}'),tokens('fn f(){go();}'))
    def test_module_removed(self):self.assertEqual(production('#[cfg(test)] mod tests {fn a(){}} fn f(){}'),tokens('fn f(){}'))
    def test_unknown_cfg_rejected(self):
        with self.assertRaises(AssertionError):production('#[cfg(feature="x")] fn a(){}')
    def test_non_test_mutation_remains_visible(self):self.assertNotEqual(production('fn f(){x();}'),production('fn f(){y();}'))
    def test_equivalent_workspace_temporary(self):
        a='prepared.push((index(&c), OwnedPayload::new(SlotPayload::Value(v))?));'
        b='let payload = OwnedPayload::new(SlotPayload::Value(v))?; prepared.push((index(&c), payload));'
        self.assertEqual(production(a),production(b))
    def test_legacy_body_changes_visible(self):
        self.assertNotEqual(test_bodies('#[test] fn a(){x();}'),test_bodies('#[test] fn a(){y();}'))
