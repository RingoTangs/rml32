import unittest
from unittest.mock import patch
from copy import deepcopy
from common import HERE,load
from contract_spec import api,verify,migration,gates
from check_contract import arithmetic,verify as checks
class ContractTests(unittest.TestCase):
    def test_canonical_future_api(self):self.assertEqual(load(HERE/'api-contract.json'),api());self.assertFalse(api()['implemented'])
    def test_five_independent_explicit_limits(self):
        v=api()['types']['LifecycleLimits'];self.assertIsNone(v['default']);self.assertEqual(len(v['fields']),5);self.assertEqual(v['fields']['max_objects']['minimum'],0);self.assertEqual(v['fields']['max_ordinary_refs_per_object']['minimum'],2)
    def test_maintenance_scope_and_stop_reasons(self):
        a=api();self.assertEqual(set(a['types']['LifecycleBudget']['fields']),{'host_roots','objects','metadata_entries'});self.assertEqual(a['maintenance_contract']['per_unit_block_returns'],'Ok(progress)');self.assertFalse(a['maintenance_contract']['zero_budget_mutates'])
    def test_host_policy_and_gate_defaults(self):
        self.assertEqual(api()['policies']['host_transport'],'raw');self.assertFalse(api()['policies']['implicit_object_drain']);self.assertFalse(gates()['implementation_ready']);self.assertFalse(gates()['public_api_changed'])
    def test_stale_artifact_rejected(self):
        with patch('pathlib.Path.read_bytes',return_value=b'{}'):
            with self.assertRaisesRegex(AssertionError,'stale contract artifact'):verify()
    def test_corrupt_arithmetic_implementation_rejected(self):
        with patch('check_contract.checked_add',side_effect=lambda a,b,bits:a+b):
            with self.assertRaisesRegex(AssertionError,'checked arithmetic disagreement'):arithmetic()
    def test_corrupt_limit_validation_rejected(self):
        with patch('check_contract.LifecycleLimits.validate',return_value=None):
            with self.assertRaisesRegex(AssertionError,'limit validation disagreement'):arithmetic()
    def test_actual_model_inventory_and_unimplemented_migration(self):
        self.assertEqual(checks()['model_cases'],39);self.assertEqual(checks()['arithmetic_rows'],112);self.assertTrue(all(not r['implemented']for r in migration()['rows']))
