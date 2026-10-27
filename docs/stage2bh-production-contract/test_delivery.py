import unittest
from unittest.mock import patch
from common import COMMIT
from verify_boundary import verify,baseline,coverage,FLAGS
from historical import verify as historical
from reference_replay import verify as reference
class DeliveryTests(unittest.TestCase):
    def test_entire_source_and_evidence_boundary(self):self.assertEqual(verify()['byte_frozen_files'],978)
    def test_baseline_commit(self):self.assertEqual(baseline()['baseline_commit'],COMMIT)
    def test_modified_frozen_input_rejected(self):
        with patch('verify_boundary.digest',return_value='bad'):
            with self.assertRaises(AssertionError):baseline()
    def test_original_cases_are_not_promoted(self):self.assertEqual(coverage(),dict(cases=115,native_unknowns=20,contract_decisions=10))
    def test_historical_authority(self):
        r=historical();self.assertEqual(r['historical_python_tests'],392);self.assertEqual(r['outer_python_tests'],26);self.assertEqual(r['nested_report']['historical_python_tests'],366)
    def test_all_six_previous_drivers_reran(self):
        r=reference();self.assertEqual(r['callable']['replay']['steps'],120);self.assertEqual(r['managed']['replay']['steps'],85);self.assertEqual(r['host']['replay']['steps'],75);self.assertEqual(r['vm']['replay']['steps'],70);self.assertEqual(r['adapter']['replay']['steps'],146);self.assertEqual(r['kernel']['replay']['transitions'],14948)
    def test_only_design_readiness_is_promoted(self):
        for key in ('runtime_changed','rust_source_changed','public_api_changed','dependencies_changed','existing_models_changed','new_native_analysis','direct_owner_capture_ready','non_scalar_ready','implementation_ready','stage2_complete','historical_python_tests_run_on_current_source'):self.assertFalse(FLAGS[key])
    def test_new_model_is_separately_identified(self):self.assertTrue(FLAGS['new_quota_contract_model_added']);self.assertTrue(FLAGS['production_contract_specified']);self.assertTrue(FLAGS['contract_model_validated'])
