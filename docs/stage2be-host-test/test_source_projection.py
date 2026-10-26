import subprocess,unittest
from source_projection import production,test_bodies
from host_diff import ROOT,COMMIT
class ProjectionTests(unittest.TestCase):
    def test_actual_production_sources(self):
        for p in ('src/boundary.rs','src/object.rs','src/vm.rs'):
            self.assertEqual(production(subprocess.check_output(['git','show',COMMIT+':'+p],cwd=ROOT).decode()),production((ROOT/p).read_text()))
    def test_test_items_are_removed(self):self.assertEqual(production('fn a(){} #[cfg(test)] fn b(){}'),production('fn a(){}'))
    def test_production_item_is_kept(self):self.assertEqual(production('#[cfg(not(test))] fn a(){}'),production('fn a(){}'))
    def test_unknown_cfg_fails_closed(self):
        with self.assertRaises(AssertionError):production('#[cfg(feature="x")] fn a(){}')
    def test_unapproved_macro_fails_closed(self):
        with self.assertRaises(AssertionError):production('macro_rules! secret { () => { 1 }; }')
    def test_factory_body_change_fails_closed(self):
        with self.assertRaises(AssertionError):production('macro_rules! life_cell { ($owner:expr, $value:expr $(,)?) => { wrong($value) }; }')
    def test_production_change_is_visible(self):self.assertNotEqual(production('fn a(){old()}'),production('fn a(){changed()}'))
    def test_legacy_test_change_is_visible(self):self.assertNotEqual(test_bodies('#[test] fn a(){old()}'),test_bodies('#[test] fn a(){changed()}'))
