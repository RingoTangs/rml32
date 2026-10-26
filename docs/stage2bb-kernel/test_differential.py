from copy import deepcopy
import io
import json
import unittest
from differential import domain,compare_domains,load,HERE,verify

class DifferentialTests(unittest.TestCase):
    def test_small_domain_complete_and_reproducible(self):
        a=io.BytesIO();b=io.BytesIO();x=domain(a,1,2);y=domain(b,1,2)
        self.assertEqual(x,y);self.assertEqual(a.getvalue(),b.getvalue());self.assertTrue(x['complete'])
        rows=[json.loads(line) for line in a.getvalue().splitlines()]
        self.assertEqual(len(rows),x['accepted_transitions']+x['rejected_transitions'])
        self.assertTrue(any(not row['accepted'] for row in rows))
    def test_state_cap_reports_incomplete(self):
        self.assertFalse(domain(io.BytesIO(),1,2,max_states=1)['complete'])
    def test_transition_cap_reports_incomplete(self):
        self.assertFalse(domain(io.BytesIO(),1,2,max_transitions=1)['complete'])
    def test_capacity_cutoff_rejected(self):
        r=load(HERE/'differential.json')['domains'];r[0]['complete']=False
        with self.assertRaises(AssertionError):compare_domains(r)
    def test_frozen_counts_checked(self):
        r=load(HERE/'differential.json')['domains'];r[0]['states']+=1
        with self.assertRaises(AssertionError):compare_domains(r)
    def test_current_report(self):self.assertEqual(verify()['replay']['transitions'],14948)
    def bad(self,change):
        r=load(HERE/'differential.json');change(r)
        with self.assertRaises(AssertionError):verify(r)
    def test_wrong_source_rejected(self):self.bad(lambda r:r['source_sha256'].update({'tests/lifecycle_kernel.rs':'0'*64}))
    def test_wrong_replay_count_rejected(self):self.bad(lambda r:r['replay'].update(transitions=1))
    def test_native_claim_rejected(self):self.bad(lambda r:r.update(native_execution=True))
    def test_wrong_log_hash_rejected(self):self.bad(lambda r:r.update(replay_log_sha256='0'*64))
    def test_wrong_input_hash_rejected(self):self.bad(lambda r:r.update(input_sha256='0'*64))
