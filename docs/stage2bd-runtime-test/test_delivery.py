import unittest
from vm_diff import generate,verify as vm
from historical import verify as historical,COMMIT,NESTED
from reference_replay import verify as reference
from verify_runtime import verify,inventory,coverage,FLAGS
class DeliveryTests(unittest.TestCase):
    def test_current_boundary(self):self.assertEqual(verify()['byte_frozen_files'],857)
    def test_actual_vm_inventory(self):self.assertGreaterEqual(len(inventory()['ordinary']),23)
    def test_case_unknowns_unchanged(self):self.assertEqual(coverage(),dict(cases=115,native_unknowns=20))
    def test_timeline_reproducible(self):self.assertEqual(generate(),generate())
    def test_vm_results(self):self.assertEqual(vm()['replay'],dict(traces=10,steps=70,physical_teardowns=10))
    def test_reference_results(self):
        r=reference();self.assertEqual(r['adapter']['replay']['steps'],146);self.assertEqual(r['kernel']['replay']['transitions'],14948)
    def test_historical_layers(self):
        r=historical();self.assertEqual(r['source_commit'],COMMIT);self.assertEqual(r['nested_report']['source_commit'],NESTED);self.assertEqual(r['historical_python_tests'],292)
    def test_closed_gates(self):
        for key in ('runtime_changed','public_api_changed','dependencies_changed','model_changed','new_native_analysis','direct_owner_capture_ready','non_scalar_ready','implementation_ready','stage2_complete','historical_python_tests_run_on_current_source'):self.assertFalse(FLAGS[key])
