import unittest
from verify_kernel import coverage,inventory,load,HERE,verify

class KernelDeliveryTests(unittest.TestCase):
    def test_scope_and_freeze(self):
        r=verify();self.assertEqual(r['protected_previous_files'],820)
        self.assertFalse(r['implementation_ready']);self.assertFalse(r['runtime_changed'])
    def test_inventory_matches_rust_source(self):self.assertGreater(len(inventory()['ordinary']),0)
    def test_altered_inventory_rejected(self):
        r=load(HERE/'test-inventory.json');r['ordinary'].pop()
        with self.assertRaises(AssertionError):inventory(r)
    def test_extra_ignored_test_rejected(self):
        r=load(HERE/'test-inventory.json');r['ignored_driver']='skip_something'
        with self.assertRaises(AssertionError):inventory(r)
    def test_all_original_cases_retained(self):
        r=coverage();self.assertEqual(r['cases'],115);self.assertEqual(r['native_unknowns'],20)
    def bad(self,change):
        r=load(HERE/'case-coverage.json');change(r)
        with self.assertRaises(AssertionError):coverage(r)
    def test_missing_case_rejected(self):self.bad(lambda r:r['cases'].pop())
    def test_changed_basis_rejected(self):self.bad(lambda r:r['cases'][0].update(basis='design'))
    def test_changed_finding_rejected(self):self.bad(lambda r:r['cases'][0].update(findings=[]))
    def test_unknown_promotion_rejected(self):
        self.bad(lambda r:next(c for c in r['cases'] if c['basis']=='unresolved').update(kernel_tests=['kernel::tests::live_zero_is_not_business_deletion'],coverage='bounded-policy-slice'))
    def test_equivalence_claim_rejected(self):self.bad(lambda r:r['cases'][0].update(native_equivalence_proven=True))
    def test_nonexistent_test_rejected(self):self.bad(lambda r:r['cases'][0].update(kernel_tests=['kernel::tests::missing']))
