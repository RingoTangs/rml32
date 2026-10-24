import unittest
from copy import deepcopy
from explore import BOUNDS, explore, key, seed


class ExplorationTests(unittest.TestCase):
    def test_zero_depth_has_one_seed_state_and_explicit_boundary(self):
        bounds=dict(BOUNDS,depth=0);r=explore('ownership',bounds)
        self.assertEqual(r['states'],1);self.assertTrue(r['completed_within_profile']);self.assertEqual(r['reached_depth'],0)

    def test_state_cutoff_is_not_reported_as_complete(self):
        r=explore('ownership',dict(BOUNDS,max_states=1))
        self.assertTrue(r['cutoff']);self.assertFalse(r['completed_within_profile'])

    def test_transition_cutoff_is_not_reported_as_complete(self):
        r=explore('graph',dict(BOUNDS,max_transitions=1))
        self.assertTrue(r['cutoff']);self.assertFalse(r['completed_within_profile'])

    def test_enumeration_is_deterministic(self):
        bounds=dict(BOUNDS,depth=1)
        self.assertEqual(explore('graph',bounds),explore('graph',bounds))

    def test_quotient_keeps_aliases_and_semantic_host_roots(self):
        w=seed('ownership');a=key(w);w.apply(dict(op='observe',root='b'));self.assertEqual(a,key(w))
        w.apply(dict(op='export',source='b',lease='h0'));self.assertNotEqual(a,key(w))

    def test_graph_seed_enumerates_actual_cycle_rejections(self):
        r=explore('graph',dict(BOUNDS,depth=1))
        self.assertGreater(r['rejected_transitions'].get('Cycle',0),0)


if __name__=='__main__':unittest.main()
