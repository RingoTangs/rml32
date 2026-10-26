import unittest
from host_diff import generate,verify as host
from verify_runtime import verify,inventory,coverage,FLAGS
from historical import verify as historical
from reference_replay import verify as reference
class DeliveryTests(unittest.TestCase):
    def test_frozen_boundary(self):self.assertEqual(verify()['byte_frozen_files'],886)
    def test_scenario_inventory(self):self.assertEqual(len(inventory()['ordinary']),31)
    def test_cases_are_not_native_promotions(self):self.assertEqual(coverage(),dict(cases=115,native_unknowns=20,host_groups=8))
    def test_timeline_is_deterministic(self):self.assertEqual(generate(),generate())
    def test_host_actual_results(self):self.assertEqual(host()['replay'],dict(traces=12,steps=75,physical_teardowns=12))
    def test_all_frozen_drivers_reran(self):
        r=reference();self.assertEqual(r['vm']['replay']['steps'],70);self.assertEqual(r['adapter']['replay']['steps'],146);self.assertEqual(r['kernel']['replay']['transitions'],14948)
    def test_historical_authorities(self):
        r=historical();self.assertEqual(r['historical_python_tests'],316);self.assertEqual(r['outer_python_tests'],24);self.assertEqual(r['nested_report']['historical_python_tests'],292)
    def test_gates_stay_closed(self):
        for key in ('runtime_changed','public_api_changed','dependencies_changed','model_changed','new_native_analysis','direct_owner_capture_ready','non_scalar_ready','implementation_ready','stage2_complete','historical_python_tests_run_on_current_source'):self.assertFalse(FLAGS[key])
