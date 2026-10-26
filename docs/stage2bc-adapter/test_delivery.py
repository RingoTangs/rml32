from copy import deepcopy
import json,unittest
from adapter_diff import generate,verify as adapter,load,HERE
from historical import verify as historical
from reference_replay import verify as reference
from verify_adapter import verify,inventory,coverage,FLAGS
class DeliveryTests(unittest.TestCase):
    def test_current_boundary(self):self.assertEqual(verify()['byte_frozen_files'],833)
    def test_inventory(self):self.assertEqual(len(inventory()['ordinary']),26)
    def test_all_cases_and_unknowns(self):self.assertEqual(coverage(),dict(cases=115,native_unknowns=20))
    def test_historical_location(self):
        r=historical();self.assertEqual(r['historical_python_tests'],267);self.assertIn('not current source',r['execution_location'])
    def test_line_ending_restore_recorded(self):self.assertEqual(len(historical()['checkout_line_ending_restorations']),11)
    def test_current_reference(self):self.assertEqual(reference()['replay']['transitions'],14948)
    def test_adapter_trace_count(self):self.assertEqual(adapter()['replay'],dict(traces=18,steps=146,rejected=2,physical_teardowns=18))
    def test_generator_deterministic(self):self.assertEqual(generate(),generate())
    def test_trace_oracles_separate(self):
        raw,_=generate();self.assertEqual({json.loads(line)['model'] for line in raw.splitlines()},{'2AS','2BA'})
    def test_gates_unchanged(self):
        for key in ('runtime_changed','public_api_changed','dependencies_changed','new_native_analysis','direct_owner_capture_ready','non_scalar_ready','implementation_ready','stage2_complete','historical_python_tests_run_on_current_source'):self.assertFalse(FLAGS[key])
