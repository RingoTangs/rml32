import copy,unittest
from managed_diff import generate,cases,check_shared_model,verify as managed
from verify_runtime import verify,inventory,coverage,FLAGS
from historical import verify as historical
from reference_replay import verify as reference
class DeliveryTests(unittest.TestCase):
    def test_frozen_boundary(self):self.assertEqual(verify()['byte_frozen_files'],915)
    def test_scenario_inventory(self):self.assertEqual(len(inventory()['ordinary']),26)
    def test_case_prefix_is_not_native_promotion(self):self.assertEqual(coverage(),dict(cases=115,native_unknowns=20,managed_groups=6))
    def test_timeline_is_deterministic(self):self.assertEqual(generate(),generate())
    def test_actual_managed_results(self):self.assertEqual(managed()['replay'],dict(traces=13,steps=85,physical_teardowns=13,frozen_model_traces=3))
    def test_frozen_drivers_all_reran(self):
        r=reference();self.assertEqual(r['host']['replay']['steps'],75);self.assertEqual(r['vm']['replay']['steps'],70);self.assertEqual(r['adapter']['replay']['steps'],146);self.assertEqual(r['kernel']['replay']['transitions'],14948)
    def test_historical_authority(self):
        r=historical();self.assertEqual(r['historical_python_tests'],340);self.assertEqual(r['outer_python_tests'],24);self.assertEqual(r['nested_report']['historical_python_tests'],316)
    def test_gates_stay_closed(self):
        for key in ('runtime_changed','public_api_changed','dependencies_changed','model_changed','new_native_analysis','direct_owner_capture_ready','non_scalar_ready','implementation_ready','stage2_complete','historical_python_tests_run_on_current_source'):self.assertFalse(FLAGS[key])
    def test_corrupt_shared_projection_is_rejected(self):
        c=copy.deepcopy(next(c for c in cases()if c['shared_model']));c['steps'][0]['expected']['life']['objects']['1']['execs']=0
        with self.assertRaises(AssertionError):check_shared_model(c)
    def test_zero_exec_cleanup_is_declared_policy_not_faked_model_frame(self):
        c=next(c for c in cases()if c['name']=='zero-exec-entry-failure-queues');self.assertFalse(c['shared_model']);s=c['steps'][1]['expected'];self.assertEqual(s['managed']['frames'],[]);self.assertEqual(s['life']['objects']['1'],dict(state='queued',refs=1,execs=0,cleaned=False))
