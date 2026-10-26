import unittest
from source_projection import production,tokens,test_bodies
class ProjectionTests(unittest.TestCase):
    def test_test_and_non_test_branches(self):self.assertEqual(production('fn f(){#[cfg(test)] {probe();} #[cfg(not(test))] original();}'),tokens('fn f(){original();}'))
    def test_unknown_cfg_rejects(self):
        with self.assertRaises(AssertionError):production('#[cfg(feature="x")] fn f(){}')
    def test_unknown_macro_rejects(self):
        with self.assertRaises(AssertionError):production('macro_rules! hidden {()=>{foo()};}')
    def test_factory_expands_with_optional_comma(self):
        self.assertEqual(production('life_cell!(self, Value::Int(3),)'),tokens('crate::cell::Cell::new(Value::Int(3))'))
    def test_factory_owner_cannot_have_effects(self):
        with self.assertRaises(AssertionError):production('life_cell!(side_effect(), v)')
    def test_factory_body_change_rejects(self):
        with self.assertRaises(AssertionError):production('macro_rules! life_cell {($owner:expr,$value:expr $(,)?)=>{different($value)};}')
    def test_legacy_body_change_visible(self):self.assertNotEqual(test_bodies('#[test] fn a(){one();}'),test_bodies('#[test] fn a(){two();}'))
    def test_production_change_visible(self):self.assertNotEqual(production('fn a(){one();}'),production('fn a(){two();}'))
