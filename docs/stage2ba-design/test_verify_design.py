from copy import deepcopy
import unittest
from verify_design import case_map,load,HERE,verify
class DesignTests(unittest.TestCase):
    def test_design(self):self.assertFalse(verify()['implementation_ready'])
    def bad(self,mutate):
        m=load(HERE/'case-map.json');mutate(m)
        with self.assertRaises(AssertionError):case_map(m)
    def test_missing_case(self):self.bad(lambda m:m['cases'].pop())
    def test_changed_original(self):self.bad(lambda m:m['cases'][0]['source_case'].update(basis='design'))
    def test_bad_hash(self):self.bad(lambda m:m['cases'][0].update(source_case_sha256='0'*64))
    def test_unknown_promotion(self):
        self.bad(lambda m:next(r for r in m['cases'] if r['category']=='native-unknown').update(category='modeled'))
