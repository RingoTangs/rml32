import copy,itertools,unittest
from common import HERE,load
from matrix import HIGH,a,put,cp,hp,ep,make,seed,families,schedules
from pair_oracle import Reference,Rejected,U

def world():
    r=Reference(HIGH)
    for x in seed():r.apply(x)
    return r
def prepare_heads(r):
    r.apply(make('c0','array','capture',['undefined']))
    for tx in ('A','B'):r.apply(hp(tx))

class ReferenceTests(unittest.TestCase):
    def test_two_watches_keep_exactly_one_payload_owner(self):
        r=world();r.apply(cp('A','s0','d0'));r.apply(cp('B','s0','d1'));r.apply(a('drop',slot='s0'))
        self.assertEqual(r.counts()[0]['o0'],1);self.assertTrue(r.cells['s0']['alive'])

    def test_last_abort_releases_a_rootless_source_cell(self):
        r=world();r.apply(cp('A','s0','d0'));r.apply(cp('B','s0','d1'));r.apply(a('drop',slot='s0'));r.apply(a('delete',object='o0'))
        r.apply(a('abort',tx='A'));self.assertEqual(r.objects['o0']['state'],'shell')
        r.apply(a('abort',tx='B'));self.assertEqual(r.objects['o0']['state'],'freed');self.assertIsNone(r.projection()['cells']['s0'])

    def test_cell_aliases_share_identity_without_retaining_twice(self):
        r=world();r.apply(a('alias_cell',source='s0',dest='alias'));self.assertEqual(r.roots['alias'],'s0');self.assertEqual(r.counts()[0]['o0'],1)

    def test_shared_source_copies_both_succeed(self):
        r=world()
        for x in [cp('A','s0','d0'),cp('B','s0','d1'),a('commit',tx='A'),a('commit',tx='B')]:r.apply(x)
        self.assertEqual(r.counts()[0]['o0'],3)

    def test_equal_destination_write_still_invalidates_second_plan(self):
        r=world();r.apply(cp('A','s0','d0'));r.apply(cp('B','s0','d0'));r.apply(a('commit',tx='A'))
        with self.assertRaises(Rejected):r.apply(a('commit',tx='B'))
        self.assertEqual(r.cells['d0']['value'],('object','o0'));self.assertEqual(r.cells['d0']['revision'],1)

    def test_first_abort_leaves_the_second_version_valid(self):
        r=world()
        for x in [cp('A','s0','d0'),cp('B','s1','d0'),a('abort',tx='A'),a('commit',tx='B')]:r.apply(x)
        self.assertEqual(r.cells['d0']['value'],('object','o1'))

    def test_source_destination_crossing_rejects_stale_source(self):
        r=world();r.apply(cp('A','s0','d0'));r.apply(cp('B','d0','d1'));r.apply(a('commit',tx='A'))
        with self.assertRaises(Rejected):r.apply(a('commit',tx='B'))
        self.assertEqual(r.cells['d1']['value'],U)

    def test_quota_failure_consumes_only_the_failed_commit(self):
        r=world();r.q=dict(HIGH,max_ordinary_refs_per_object=2)
        r.apply(cp('A','s0','d0'));r.apply(cp('B','s1','d1'))
        with self.assertRaises(Rejected):r.apply(a('commit',tx='A'))
        self.assertEqual(set(r.transactions),{'B'});self.assertEqual(r.cells['d0']['value'],U)

    def test_copy_preparation_does_not_reserve_an_extra_owner(self):
        r=world();r.q=dict(HIGH,max_ordinary_refs_per_object=2);r.apply(cp('A','s0','d0'));r.apply(cp('B','s0','d1'))
        self.assertEqual(r.counts()[0]['o0'],1)

    def test_normalizing_one_source_invalidates_another_old_plan(self):
        r=world();r.apply(a('delete',object='o0'));r.apply(cp('A','s0','d0',True));r.apply(cp('B','s0','d1'));r.apply(a('commit',tx='A'))
        with self.assertRaises(Rejected):r.apply(a('commit',tx='B'))
        self.assertEqual(r.cells['s0']['value'],U)

    def test_each_new_head_has_one_creator_owner(self):
        r=world();prepare_heads(r);self.assertEqual(r.counts()[0]['o0'],3)
        r.apply(a('abort',tx='A'));self.assertEqual(r.counts()[0]['o0'],2)

    def test_second_head_refusal_preserves_first(self):
        r=world();r.apply(make('c0','array','capture',['undefined']));r.q=dict(HIGH,max_function_heads=1);r.apply(hp('A'))
        with self.assertRaises(Rejected):r.apply(hp('B'))
        self.assertEqual(set(r.transactions),{'A'})

    def test_invalid_creator_commit_releases_only_its_own_head(self):
        r=world();prepare_heads(r);r.apply(a('delete',object='o0'))
        with self.assertRaises(Rejected):r.apply(a('commit',tx='A'))
        self.assertEqual(set(r.transactions),{'B'});self.assertNotIn('hA',r.heads);self.assertIn('hB',r.heads)

    def test_two_execution_preparations_delay_queue_until_both_exit(self):
        r=world()
        for x in [ep('A'),ep('B'),a('delete',object='o0'),a('abort',tx='A')]:r.apply(x)
        self.assertEqual(r.objects['o0']['state'],'pending');r.apply(a('abort',tx='B'));self.assertEqual(r.queue,['o0'])

    def test_execution_commit_transfers_without_double_charging(self):
        r=world();r.apply(ep('A'));r.apply(ep('B'));r.apply(a('commit',tx='A'));self.assertEqual(r.counts()[2]['o0'],2)

    def test_duplicate_transaction_id_is_rejected_without_replacing_owner(self):
        r=world();r.apply(ep('A'));before=r.projection()
        with self.assertRaises(Rejected):r.apply(ep('A'))
        self.assertEqual(r.projection(),before)

    def test_unpublished_output_cannot_be_used_as_a_source(self):
        r=world();r.apply(make('c0','array','capture',['undefined']));r.apply(hp('A'))
        with self.assertRaises(Rejected):r.apply(cp('B','p:A','d0'))
        self.assertEqual(set(r.transactions),{'A'})

    def test_unpublished_header_descriptors_and_views_are_private(self):
        r=world();r.apply(make('c0','array','capture',['undefined']));r.apply(hp('A'))
        for action in [put('d0',['header','hA']),a('view',header='hA')]:
            with self.assertRaises(Rejected):r.apply(action)
        self.assertEqual(set(r.transactions),{'A'})

    def test_expired_and_undefined_slots_are_distinct(self):
        r=world();r.apply(a('drop',slot='u'));self.assertIsNone(r.projection()['cells']['u']);self.assertEqual(r.projection()['cells']['d0']['value'],['undefined'])

    def test_physical_value_snapshots_do_not_hold_semantic_owners(self):
        r=world();r.apply(make('h0','array','source',['object','o1']));r.apply(cp('A','source','d0'));r.apply(put('source',['undefined']))
        self.assertTrue(r.heads['h0']['retired']);self.assertIn('h0',r.physical);self.assertEqual(r.counts()[1]['h0'],0)

    def test_mapping_physical_revision_is_separate_from_owner_release(self):
        r=world();r.apply(make('h0','mapping','source',['object','o1']));r.apply(cp('B','m:h0','d1'));r.apply(put('source',['undefined']))
        self.assertEqual(r.cells['m:h0']['revision'],2);self.assertEqual(r.counts()[0]['o1'],1)

class DomainTests(unittest.TestCase):
    def test_all_six_pair_kinds_have_controls(self):self.assertEqual({f['pair'] for f in families()},{'CC','CH','CE','HH','HE','EE'})
    def test_each_layout_has_twenty_four_schedules(self):
        for f in families():self.assertEqual(len(list(schedules(f))),24)
    def test_schedules_never_terminate_before_prepare(self):
        for _,actions in schedules(families()[0]):
            prepared=set()
            for x in actions:
                if x['op']=='prepare':prepared.add(x['tx'])
                else:self.assertIn(x['tx'],prepared)
    def test_snapshot_overlay_is_bound_to_preexisting_index(self):
        b=load(HERE/'baseline-inputs.json');self.assertEqual(len(b['files']),1203);self.assertEqual(len(b['index_sha256']),64)
