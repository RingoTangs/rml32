import copy
import unittest
from interleaving_oracle import Reference,Rejected,U
from matrix import HIGH,a,put,directed,metadata_cases

def world():
    r=Reference(HIGH)
    for x in [a('object',id='o0'),a('object',id='o1'),put('s0',['object','o0']),put('d0',['undefined'])]: r.apply(x)
    return r

def head(r): r.apply(a('make',id='h0',kind='function',creator='o0',dest='s0',value=['undefined']))

class OracleTests(unittest.TestCase):
    def test_pending_is_valid_until_final_execution_exit(self):
        r=world();r.apply(a('exec',id='e',object='o0'));r.apply(a('delete',object='o0'))
        self.assertTrue(r.valid('o0'));r.apply(a('exit',id='e'));self.assertFalse(r.valid('o0'));self.assertEqual(r.queue,['o0'])

    def test_duplicate_remove_never_duplicates_queue_responsibility(self):
        r=world();r.apply(a('exec',id='e',object='o0'))
        r.apply(a('delete',object='o0'));r.apply(a('delete',object='o0'));r.apply(a('exit',id='e'))
        with self.assertRaises(Rejected):r.apply(a('delete',object='o0'))
        self.assertEqual(r.queue,['o0'])

    def test_prospective_execution_reservation_prevents_early_queue(self):
        r=world()
        for x in [a('exec',id='e',object='o0'),a('prepare_exec',id='p',object='o0'),a('delete',object='o0'),a('exit',id='e')]:r.apply(x)
        self.assertEqual(r.queue,[]);r.apply(a('abort_exec',id='p'));self.assertEqual(r.queue,['o0'])

    def test_shell_ordinary_ceiling_includes_reserved_queue_unit(self):
        r=world();r.q=dict(HIGH,max_ordinary_refs_per_object=2)
        with self.assertRaises(Rejected):r.apply(put('extra',['object','o0']))
        r.apply(a('delete',object='o0'));r.apply(put('extra',['object','o0']))
        self.assertEqual(r.counts()[0]['o0'],2)

    def test_zero_owners_never_delete_an_alive_object(self):
        r=world();r.apply(a('drop',slot='s0'));self.assertEqual(r.objects['o0']['state'],'alive')

    def test_script_copy_normalizes_both_real_slots(self):
        r=world();r.apply(a('delete',object='o0'));r.apply(a('prepare',writes=[['s0','d0']],script=True));r.apply(a('commit'))
        self.assertEqual(r.slots['s0'],U);self.assertEqual(r.slots['d0'],U);self.assertEqual(r.objects['o0']['state'],'freed')

    def test_later_invalidation_does_not_rerun_copy_normalization(self):
        r=world();r.apply(a('prepare',writes=[['s0','d0']],script=True));r.apply(a('delete',object='o0'));r.apply(a('commit'))
        self.assertEqual(r.slots['d0'],('object','o0'))

    def test_raw_move_does_not_add_a_reference(self):
        r=world();r.q=dict(HIGH,max_ordinary_refs_per_object=2);r.apply(a('prepare_move',source='s0',dest='d0'));r.apply(a('commit'))
        self.assertEqual(r.counts()[0]['o0'],1);self.assertEqual(r.slots['s0'],U)

    def test_occupied_raw_destination_is_rejected_atomically(self):
        r=world();r.apply(put('d0',['int',7]));r.apply(a('prepare',writes=[['s0','d0']],raw=True));before=r.projection()
        with self.assertRaises(Rejected):r.apply(a('commit'))
        self.assertEqual(r.projection(),before)

    def test_stale_variable_source_does_not_publish_cached_value(self):
        r=world();r.apply(put('v:o0',['object','o1']));r.apply(a('prepare',writes=[['v:o0','d0']]));r.apply(a('delete',object='o0'));before=r.projection()
        with self.assertRaises(Rejected):r.apply(a('commit'))
        self.assertEqual(r.projection(),before)

    def test_pending_creator_can_publish(self):
        r=world();r.apply(a('exec',id='e',object='o0'));r.apply(a('prepare_head',id='h0',kind='function',creator='o0',dest='d0',value=['undefined']));r.apply(a('delete',object='o0'));r.apply(a('commit_head'))
        self.assertEqual(r.slots['d0'],('header','h0'))

    def test_invalid_creator_aborts_unpublished_head_and_ordinary_owner(self):
        r=world();r.apply(a('prepare_head',id='h0',kind='function',creator='o0',dest='d0',value=['undefined']));r.apply(a('delete',object='o0'))
        with self.assertRaises(Rejected):r.apply(a('commit_head'))
        self.assertNotIn('h0',r.heads);self.assertEqual(r.counts()[0]['o0'],1)

    def test_existing_function_raw_copy_does_not_duplicate_creator(self):
        r=world();head(r);r.apply(a('delete',object='o0'));r.apply(a('prepare',writes=[['s0','d0']]));r.apply(a('commit'))
        self.assertEqual(r.counts()[0]['o0'],1);self.assertEqual(r.counts()[1]['h0'],2)

    def test_invalid_function_script_copy_retires_last_head_once(self):
        r=world();head(r);r.apply(a('delete',object='o0'));r.apply(a('prepare',writes=[['s0','d0']],script=True));r.apply(a('commit'))
        self.assertTrue(r.heads['h0']['retired']);self.assertEqual(r.objects['o0']['state'],'freed');self.assertEqual(r.counts()[0]['o0'],0)

    def test_physical_view_does_not_keep_semantic_members_or_creator(self):
        r=world();head(r);r.apply(a('view',header='h0'));r.apply(a('drop',slot='s0'))
        self.assertTrue(r.heads['h0']['retired']);self.assertEqual(r.counts()[0]['o0'],0)

    def test_lifo_and_borrowed_head_preserve_partial_progress(self):
        r=world()
        for o in ['o0','o1']:
            for x in [a('exec',id=o,object=o),a('delete',object=o),a('exit',id=o)]:r.apply(x)
        r.apply(a('maintain',objects=2,borrow='v:o0'));self.assertEqual(r.queue,['o0']);self.assertTrue(r.objects['o1']['cleaned']);self.assertFalse(r.objects['o0']['cleaned'])

    def test_zero_budget_preserves_prepared_transaction_and_all_state(self):
        r=world();r.apply(a('prepare',writes=[['s0','d0']]));before=copy.deepcopy(r.__dict__);r.apply(a('zero'));self.assertEqual(r.__dict__,before)

    def test_directed_controls_have_all_lifecycle_states(self):
        found=set()
        for t in directed(True):
            r=Reference(t['limits'])
            for x in t['setup']+t['actions']:
                try:r.apply(x)
                except Rejected:
                    if not x.get('continue_on_error'):break
                found.update(o['state'] for o in r.objects.values())
        self.assertEqual(found,{'alive','pending','queued','shell','freed'})

    def test_metadata_cases_use_separate_budgets_and_explicit_views(self):
        ts=list(metadata_cases());self.assertEqual(len(ts),6)
        for t in ts:
            self.assertTrue(t['detail']);self.assertTrue(all(not x.get('objects',0) for x in t['actions']))
