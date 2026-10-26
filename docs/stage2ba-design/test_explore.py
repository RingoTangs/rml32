import unittest
from explore import explore
class ExplorationTests(unittest.TestCase):
    def test_small_complete(self):self.assertTrue(explore(1,2)['complete'])
    def test_state_cap_incomplete(self):self.assertFalse(explore(1,2,max_states=1)['complete'])
    def test_transition_cap_incomplete(self):self.assertFalse(explore(1,2,max_transitions=1)['complete'])
    def test_deterministic(self):self.assertEqual(explore(1,2),explore(1,2))
