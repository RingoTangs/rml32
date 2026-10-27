import copy,unittest
from callable_diff import generate,cases,check_model,verify as callable_result
from verify_runtime import verify,inventory,coverage,FLAGS
from historical import verify as historical
from reference_replay import verify as reference
class DeliveryTests(unittest.TestCase):
    def test_frozen_boundary(self):self.assertEqual(verify()['byte_frozen_files'],944)
    def test_scenario_inventory(self):self.assertEqual(len(inventory()['ordinary']),34)
    def test_case_prefix_is_not_native_promotion(self):self.assertEqual(coverage(),dict(cases=115,native_unknowns=20,callable_groups=6))
    def test_timeline_is_deterministic(self):self.assertEqual(generate(),generate())
    def test_actual_callable_results(self):self.assertEqual(callable_result()['replay'],dict(traces=14,steps=120,physical_teardowns=14,frozen_model_traces=4))
    def test_frozen_drivers_all_reran(self):
        r=reference();self.assertEqual(r['managed']['replay']['steps'],85);self.assertEqual(r['host']['replay']['steps'],75);self.assertEqual(r['vm']['replay']['steps'],70);self.assertEqual(r['adapter']['replay']['steps'],146);self.assertEqual(r['kernel']['replay']['transitions'],14948)
    def test_historical_authority(self):
        r=historical();self.assertEqual(r['historical_python_tests'],366);self.assertEqual(r['outer_python_tests'],26);self.assertEqual(r['nested_report']['historical_python_tests'],340)
    def test_gates_stay_closed(self):
        for key in ('runtime_changed','public_api_changed','dependencies_changed','model_changed','new_native_analysis','direct_owner_capture_ready','non_scalar_ready','implementation_ready','stage2_complete','historical_python_tests_run_on_current_source'):self.assertFalse(FLAGS[key])
    def test_corrupt_shared_projection_is_rejected(self):
        c=copy.deepcopy(next(c for c in cases()if c['shared_model']));c['steps'][0]['expected']['life']['objects']['0']['execs']=0
        with self.assertRaises(AssertionError):check_model(c)
    def test_corrupt_head_owner_count_is_rejected(self):
        c=copy.deepcopy(next(c for c in cases()if c['shared_model']));c['steps'][1]['expected']['head']['owners']=2
        with self.assertRaises(AssertionError):check_model(c)
