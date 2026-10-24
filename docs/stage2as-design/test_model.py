"""Design invariants and counterexamples, independent of Rust/native execution."""
import copy
import unittest
from model import World, Rejected, UNDEFINED, audit


def put(w, name, tag='int', value=7):
    return w.apply(dict(op='put',root=name,value=[tag,value]))


def build(w, name, kind, sources, creator=None):
    return w.apply(dict(op='build',root=name,kind=kind,sources=sources,creator=creator))


def val(w, root):
    return w.slots[w.roots[root].cell].value


def function(w, name='f', capture=7):
    put(w,'x',value=capture);build(w,name,'function',['x']);return val(w,name)[1]


class ModelTests(unittest.TestCase):
    def rejected(self,w,action,code,fail_at=None):
        before=w.fingerprint()
        with self.assertRaises(Rejected) as error:w.apply(action,fail_at)
        self.assertEqual(error.exception.code,code)
        self.assertEqual(w.fingerprint(),before)
        audit(w)

    def test_copy_adds_one_payload_owner_move_transfers_it(self):
        w=World();h=function(w)
        w.apply(dict(op='copy',source='f',root='g'));self.assertEqual(w.headers[h].owners,2)
        w.apply(dict(op='move',source='g',root='m'));self.assertEqual(w.headers[h].owners,2)
        self.assertNotIn('g',w.roots)
        w.apply(dict(op='drop',root='m'));self.assertEqual(w.headers[h].owners,1)

    def test_reference_alias_owns_cell_not_another_function_payload(self):
        w=World();h=function(w);w.apply(dict(op='alias',source='f',root='r'))
        self.assertEqual(w.slots[w.roots['f'].cell].refs,2);self.assertEqual(w.headers[h].owners,1)
        mode=w.apply(dict(op='invoke',root='f',outputs=['arg']))['result']
        self.assertEqual(mode,'unique');self.assertEqual(val(w,'r'),UNDEFINED)
        self.assertEqual(val(w,'arg'),('int',7));self.assertNotIn(h,w.headers)

    def test_observe_and_python_deepcopy_do_not_change_semantic_branch(self):
        w=World();function(w);before=w.fingerprint()
        snapshots=[w.observe('f') for _ in range(5)];mirror=copy.deepcopy(w)
        self.assertEqual(w.fingerprint(),before);self.assertEqual(snapshots[0],snapshots[-1])
        self.assertEqual(w.apply(dict(op='invoke',root='f',outputs=['a']))['result'],'unique')
        self.assertEqual(mirror.apply(dict(op='invoke',root='f',outputs=['a']))['result'],'unique')

    def test_observation_cannot_be_reimported_as_owned_lease(self):
        w=World();function(w);snap=w.observe('f')
        self.rejected(w,dict(op='import',lease=snap,root='forged'),'InvalidState')

    def test_host_roots_are_real_owners_and_select_shared(self):
        w=World();h=function(w);w.apply(dict(op='export',source='f',lease='host'))
        self.assertEqual(w.headers[h].owners,2)
        self.assertEqual(w.apply(dict(op='invoke',root='f',outputs=['a']))['result'],'shared')
        self.assertEqual(w.headers[h].owners,1)
        w.apply(dict(op='import',lease='host',root='again'))
        self.assertEqual(w.headers[h].owners,2)
        w.apply(dict(op='release',lease='host'));self.assertEqual(w.headers[h].owners,1)

    def test_retain_is_explicit_and_release_does_not_double_free(self):
        w=World();h=function(w);w.apply(dict(op='export',source='f',lease='h1'))
        w.apply(dict(op='retain',source='h1',lease='h2'));self.assertEqual(w.headers[h].owners,3)
        for root in ['h1','h2']:w.apply(dict(op='release',lease=root))
        w.apply(dict(op='drop',root='f'));self.assertNotIn(h,w.headers)
        self.rejected(w,dict(op='release',lease='h2'),'InvalidState')

    def test_shared_call_copies_and_preserves_capture_order(self):
        w=World();put(w,'a',value=10);put(w,'b',value=20);build(w,'f','function',['a','b'])
        w.apply(dict(op='copy',source='f',root='saved'))
        self.assertEqual(w.apply(dict(op='invoke',root='f',outputs=['a0','a1']))['result'],'shared')
        self.assertEqual((val(w,'a0'),val(w,'a1')),(('int',10),('int',20)))
        self.assertEqual(w.apply(dict(op='invoke',root='saved',outputs=['b0','b1']))['result'],'unique')
        self.assertEqual((val(w,'b0'),val(w,'b1')),(('int',10),('int',20)))

    def test_shared_copy_normalizes_actual_dead_object_capture(self):
        w=World();w.apply(dict(op='object',id='o'));put(w,'raw','object','o');build(w,'f','function',['raw'])
        h=val(w,'f')[1];w.apply(dict(op='copy',source='f',root='saved'));w.apply(dict(op='delete-object',id='o'))
        w.apply(dict(op='invoke',root='f',outputs=['arg']))
        self.assertEqual(val(w,'arg'),UNDEFINED);self.assertEqual(w.slots[w.headers[h].cells[0]].value,UNDEFINED)
        w.apply(dict(op='invoke',root='saved',outputs=['again']));self.assertEqual(val(w,'again'),UNDEFINED)

    def test_unique_move_does_not_normalize_dead_object_capture(self):
        w=World();w.apply(dict(op='object',id='o'));put(w,'raw','object','o');build(w,'f','function',['raw'])
        w.apply(dict(op='delete-object',id='o'));w.apply(dict(op='invoke',root='f',outputs=['arg']))
        self.assertEqual(val(w,'arg'),('object','o'))

    def test_function_dead_creator_normalization_is_distinct_from_outer_creator(self):
        w=World();w.apply(dict(op='object',id='inner-owner'));put(w,'x');build(w,'inner','function',['x'],'inner-owner')
        build(w,'outer','function',['inner']);w.apply(dict(op='copy',source='outer',root='saved'))
        w.apply(dict(op='delete-object',id='inner-owner'));w.apply(dict(op='invoke',root='outer',outputs=['arg']))
        self.assertEqual(val(w,'arg'),UNDEFINED)
        self.assertEqual(w.headers[val(w,'saved')[1]].creator,None)

    def test_ownerless_function_is_not_dead(self):
        w=World();h=function(w);w.apply(dict(op='copy',source='f',root='g'))
        self.assertEqual(val(w,'g'),('function',h))

    def test_deleted_creator_raw_call_fails_before_commit(self):
        w=World();w.apply(dict(op='object',id='o'));put(w,'x');build(w,'f','function',['x'],'o')
        w.apply(dict(op='delete-object',id='o'))
        self.rejected(w,dict(op='invoke',root='f',outputs=['arg']),'InvalidObject')

    def test_shallow_container_copy_does_not_normalize_member(self):
        w=World();w.apply(dict(op='object',id='o'));put(w,'x','object','o');build(w,'array','array',['x'])
        w.apply(dict(op='delete-object',id='o'));w.apply(dict(op='copy',source='array',root='alias'))
        h=val(w,'array')[1];self.assertEqual(w.slots[w.headers[h].cells[0]].value,('object','o'))
        w.apply(dict(op='member-copy',source='array',index=0,root='read'))
        self.assertEqual(val(w,'read'),UNDEFINED);self.assertEqual(w.slots[w.headers[h].cells[0]].value,UNDEFINED)

    def test_array_capture_shares_header_and_retains_member_identity(self):
        w=World();put(w,'x');build(w,'array','array',['x']);a=val(w,'array')[1]
        w.apply(dict(op='copy',source='array',root='outside'));build(w,'f','function',['array'])
        w.apply(dict(op='copy',source='f',root='saved'));w.apply(dict(op='invoke',root='f',outputs=['arg']))
        self.assertEqual(val(w,'arg'),val(w,'outside'));self.assertEqual(w.headers[a].owners,3)

    def test_array_final_header_preserves_referenced_element_payload(self):
        w=World();put(w,'x',value=9);build(w,'array','array',['x'])
        w.apply(dict(op='member-ref',source='array',index=0,root='r'));w.apply(dict(op='drop',root='array'))
        self.assertEqual(val(w,'r'),('int',9));self.assertFalse(w.headers)
        w.apply(dict(op='drop',root='r'));self.assertFalse(w.slots)

    def test_mapping_final_header_clears_referenced_value(self):
        w=World();put(w,'x',value=9);build(w,'map','mapping',['x'])
        w.apply(dict(op='member-ref',source='map',index=0,root='r'));w.apply(dict(op='drop',root='map'))
        self.assertEqual(val(w,'r'),UNDEFINED);self.assertFalse(w.headers)
        w.apply(dict(op='drop',root='r'));self.assertFalse(w.slots)

    def test_shared_container_header_does_not_double_count_function_member(self):
        w=World();h=function(w);build(w,'array','array',['f']);w.apply(dict(op='copy',source='array',root='other'))
        self.assertEqual(w.headers[h].owners,1)
        self.assertEqual(w.headers[val(w,'array')[1]].owners,2)

    def test_function_container_function_cycle_is_rejected_atomically(self):
        w=World();put(w,'x');build(w,'array','array',['x']);w.apply(dict(op='member-ref',source='array',index=0,root='r'))
        build(w,'f','function',['array'])
        self.rejected(w,dict(op='write',target='r',source='f'),'Cycle')

    def test_longer_function_container_cycle_is_rejected(self):
        w=World();put(w,'x');build(w,'array','array',['x']);w.apply(dict(op='member-ref',source='array',index=0,root='r'))
        build(w,'f','function',['array']);build(w,'outer','function',['f'])
        self.rejected(w,dict(op='write',target='r',source='outer'),'Cycle')

    def test_shared_dag_is_accepted_not_misclassified_as_cycle(self):
        w=World();put(w,'x');build(w,'leaf','array',['x']);w.apply(dict(op='copy',source='leaf',root='other'))
        build(w,'parent','mapping',['leaf','other']);audit(w)
        h=val(w,'parent')[1];children=[w.slots[c].value for c in w.headers[h].cells]
        self.assertEqual(children[0],children[1])

    def test_replacing_old_edge_uses_final_graph_and_releases_old_header(self):
        w=World();put(w,'x');build(w,'old','array',['x']);old=val(w,'old')[1]
        build(w,'parent','mapping',['old']);w.apply(dict(op='member-ref',source='parent',index=0,root='r'))
        put(w,'new',value=42);w.apply(dict(op='write',target='r',source='new'))
        self.assertNotIn(old,w.headers);self.assertEqual(val(w,'r'),('int',42))

    def test_batch_copy_failure_does_not_normalize_earlier_source(self):
        w=World();w.apply(dict(op='object',id='o'));put(w,'dead','object','o');put(w,'b');put(w,'t0');put(w,'t1')
        w.apply(dict(op='delete-object',id='o'));w.apply(dict(op='borrow',root='t1'))
        self.rejected(w,dict(op='batch-copy',sources=['dead','b'],targets=['t0','t1']),'InvalidState')
        self.assertEqual(val(w,'dead'),('object','o'))

    def test_conflicting_duplicate_writes_are_rejected(self):
        w=World();put(w,'a',value=1);put(w,'b',value=2);put(w,'target')
        self.rejected(w,dict(op='batch-copy',sources=['a','b'],targets=['target','target']),'ConflictingWrite')

    def test_identical_duplicate_writes_are_coalesced(self):
        w=World();put(w,'a',value=1);put(w,'target')
        w.apply(dict(op='batch-copy',sources=['a','a'],targets=['target','target']))
        self.assertEqual(val(w,'target'),('int',1))

    def test_readonly_alias_rejects_write(self):
        w=World();put(w,'a');put(w,'b');w.apply(dict(op='alias',source='a',root='r',readonly=True))
        self.rejected(w,dict(op='write',target='r',source='b'),'Readonly')

    def test_function_capture_cannot_accept_reference_operand(self):
        w=World();put(w,'x');w.apply(dict(op='alias',source='x',root='r'))
        self.rejected(w,dict(op='build',kind='function',root='f',sources=['r']),'UnsupportedShape')

    def test_foreign_origin_is_rejected_before_publication(self):
        w=World();self.rejected(w,dict(op='put',root='x',value=['foreign','other']),'ForeignOrigin')

    def test_graph_depth_limit_includes_function_edges(self):
        w=World(max_depth=2);put(w,'x');build(w,'a','array',['x']);build(w,'f','function',['a'])
        self.rejected(w,dict(op='build',kind='function',root='outer',sources=['f']),'Limit')

    def test_graph_item_and_work_limits_are_explicit_policy(self):
        w=World(max_items=1);put(w,'a');put(w,'b')
        self.rejected(w,dict(op='build',kind='array',root='c',sources=['a','b']),'Limit')
        w=World(max_nodes=2);put(w,'a')
        self.rejected(w,dict(op='build',kind='function',root='f',sources=['a']),'Limit')

    def test_every_create_checkpoint_preserves_state_on_failure(self):
        base=World();put(base,'x');action=dict(op='build',kind='function',root='f',sources=['x'])
        count=copy.deepcopy(base).apply(action)['checkpoints']
        for point in range(count):
            with self.subTest(point=point):self.rejected(copy.deepcopy(base),action,'Limit',point)

    def test_every_unique_and_shared_call_checkpoint_preserves_state(self):
        for shared in [False,True]:
            base=World();function(base)
            if shared:base.apply(dict(op='copy',source='f',root='saved'))
            action=dict(op='invoke',root='f',outputs=['arg'])
            count=copy.deepcopy(base).apply(action)['checkpoints']
            for point in range(count):
                with self.subTest(shared=shared,point=point):self.rejected(copy.deepcopy(base),action,'Limit',point)

    def test_independent_audit_detects_corrupted_owner_counter(self):
        w=World();h=function(w);w.headers[h].owners+=1
        with self.assertRaisesRegex(AssertionError,'owner ledger'):audit(w)

    def test_independent_audit_detects_corrupted_slot_counter(self):
        w=World();put(w,'x');w.slots[w.roots['x'].cell].refs+=1
        with self.assertRaisesRegex(AssertionError,'slot ref ledger'):audit(w)

    def test_final_function_release_cascades_capture_cleanup_once(self):
        w=World();function(w,'inner');inner=val(w,'inner')[1];build(w,'outer','function',['inner']);outer=val(w,'outer')[1]
        w.apply(dict(op='drop',root='outer'));self.assertFalse(w.headers);self.assertFalse(w.slots)
        releases=[e[1] for e in w.events if e[0]=='release-header']
        self.assertEqual(releases,[outer,inner])

    def test_return_move_does_not_insert_copy_normalization(self):
        w=World();w.apply(dict(op='object',id='o'));put(w,'result','object','o');w.apply(dict(op='delete-object',id='o'))
        w.apply(dict(op='move',source='result',root='returned'));self.assertEqual(val(w,'returned'),('object','o'))

    def test_borrowed_target_prevents_last_mapping_cleanup_atomically(self):
        w=World();put(w,'x');build(w,'map','mapping',['x']);w.apply(dict(op='member-ref',source='map',index=0,root='r'))
        w.apply(dict(op='borrow',root='r'));self.rejected(w,dict(op='drop',root='map'),'InvalidState')

    def test_raw_host_transport_and_entry_copy_are_separate(self):
        w=World();w.apply(dict(op='object',id='o'));put(w,'x','object','o');w.apply(dict(op='export',source='x',lease='h'))
        w.apply(dict(op='delete-object',id='o'));w.apply(dict(op='import',lease='h',root='transport'))
        self.assertEqual(val(w,'transport'),('object','o'))
        w.apply(dict(op='copy',source='transport',root='entry'));self.assertEqual(val(w,'entry'),UNDEFINED)
        self.assertEqual(w.host['h'],('object','o'))  # Distinct host root, not same source Cell.

    def test_scalar_capture_bits_and_bytes_are_preserved(self):
        for tag,payload in [('undefined',None),('void',None),('int',-2147483648),('real',0x7fc01234),('bytes','ff0080')]:
            with self.subTest(tag=tag):
                w=World();put(w,'x',tag,payload);build(w,'f','function',['x'])
                w.apply(dict(op='invoke',root='f',outputs=['a']))
                self.assertEqual(val(w,'a'),(tag,payload))

    def test_mapping_capture_is_shallow_and_both_container_cycles_are_checked(self):
        for kind in ['array','mapping']:
            with self.subTest(kind=kind):
                w=World();put(w,'x');build(w,'c',kind,['x']);h=val(w,'c')[1]
                w.apply(dict(op='copy',source='c',root='outside'));build(w,'f','function',['c'])
                w.apply(dict(op='invoke',root='f',outputs=['a']));self.assertEqual(val(w,'a'),(kind,h))
                w=World();put(w,'x');build(w,'c',kind,['x']);w.apply(dict(op='member-ref',source='c',index=0,root='r'))
                build(w,'f','function',['c']);self.rejected(w,dict(op='write',target='r',source='f'),'Cycle')

    def test_bound_arguments_precede_new_arguments_and_count_is_checked(self):
        w=World();put(w,'x',value=10);build(w,'f','function',['x']);put(w,'new',value=20)
        w.apply(dict(op='invoke',root='f',outputs=['arg'],new=['new']))
        self.assertIn(['arguments',['arg','new']],w.events)
        self.assertEqual([val(w,n) for n in ['arg','new']],[('int',10),('int',20)])
        w=World()
        for i in range(64):put(w,str(i))
        self.rejected(w,dict(op='build',kind='function',root='f',sources=[str(i) for i in range(64)]),'Limit')

    def test_shared_dag_audit_is_linear_in_edges_not_path_multiplicity(self):
        w=World();put(w,'x');build(w,'node','array',['x'])
        for i in range(25):
            w.apply(dict(op='copy',source='node',root='alias'))
            build(w,'next','array',['node','alias'])
            w.apply(dict(op='move',source='next',root='node'))
        audit(w)

    def test_reference_cannot_be_raw_moved_or_escape_through_container_build(self):
        w=World();put(w,'x');w.apply(dict(op='alias',source='x',root='r',readonly=True))
        self.rejected(w,dict(op='move',source='r',root='m'),'UnsupportedShape')
        self.rejected(w,dict(op='build',root='a',kind='array',sources=['r']),'UnsupportedShape')

    def test_member_container_rewiring_stays_rejected_even_if_acyclic(self):
        w=World();put(w,'x');build(w,'a','array',['x']);w.apply(dict(op='member-ref',source='a',index=0,root='r'))
        put(w,'y');build(w,'b','array',['y'])
        self.rejected(w,dict(op='write',target='r',source='b'),'UnsupportedType')

    def test_retired_snapshot_identity_is_not_reused(self):
        w=World();function(w);old=w.observe('f');w.apply(dict(op='drop',root='f'))
        function(w);new=w.observe('f');self.assertNotEqual(old[1],new[1])

    def test_raw_create_preserves_already_dead_object_or_function_payload(self):
        for tag in ['object','function']:
            with self.subTest(tag=tag):
                w=World();w.apply(dict(op='object',id='o'))
                if tag=='object':put(w,'raw','object','o')
                else:
                    put(w,'x');build(w,'raw','function',['x'],'o')
                expected=val(w,'raw');w.apply(dict(op='delete-object',id='o'))
                build(w,'outer','function',['raw']);w.apply(dict(op='invoke',root='outer',outputs=['arg']))
                self.assertEqual(val(w,'arg'),expected)

    def test_dereference_normalizes_source_visible_through_other_alias(self):
        w=World();w.apply(dict(op='object',id='o'));put(w,'raw','object','o')
        for name in ['r1','r2']:w.apply(dict(op='alias',source='raw',root=name))
        w.apply(dict(op='delete-object',id='o'));w.apply(dict(op='deref',source='r1',root='result'))
        self.assertEqual(val(w,'result'),UNDEFINED);self.assertEqual(val(w,'r2'),UNDEFINED)
        self.assertNotIn('r1',w.roots);self.assertEqual(w.slots[w.roots['raw'].cell].refs,2)

    def test_selected_member_copy_normalizes_both_container_kinds(self):
        for kind in ['array','mapping']:
            with self.subTest(kind=kind):
                w=World();w.apply(dict(op='object',id='o'));put(w,'raw','object','o');build(w,'c',kind,['raw'])
                w.apply(dict(op='member-ref',source='c',index=0,root='r'))
                w.apply(dict(op='delete-object',id='o'));w.apply(dict(op='member-copy',source='c',index=0,root='result'))
                self.assertEqual(val(w,'r'),UNDEFINED);self.assertEqual(val(w,'result'),UNDEFINED)

    def test_host_root_limit_failure_does_not_add_owner(self):
        w=World(max_host_roots=1);h=function(w);w.apply(dict(op='export',source='f',lease='h0'))
        self.rejected(w,dict(op='retain',source='h0',lease='h1'),'Limit');self.assertEqual(w.headers[h].owners,2)

    def test_drop_retirement_defers_cleanup_until_borrow_epoch_ends(self):
        w=World();put(w,'x');build(w,'m','mapping',['x'])
        w.apply(dict(op='member-ref',source='m',index=0,root='r'))
        w.apply(dict(op='export',source='m',lease='host'));w.apply(dict(op='drop',root='m'))
        w.apply(dict(op='borrow',root='r'));w.apply(dict(op='retire',lease='host'))
        self.assertEqual(w.pending,{'host'});self.assertEqual(len(w.headers),1)
        self.rejected(w,dict(op='import',lease='host',root='again'),'InvalidState')
        w.apply(dict(op='unborrow',root='r'));self.assertFalse(w.pending);self.assertFalse(w.host)
        self.assertEqual(val(w,'r'),UNDEFINED);self.assertFalse(w.headers)

    def test_drop_retirement_is_nonfallible_at_model_checkpoint_injection(self):
        w=World();function(w);w.apply(dict(op='export',source='f',lease='host'))
        w.apply(dict(op='retire',lease='host'),fail_at=0)
        self.assertFalse(w.host);self.assertFalse(w.pending)


if __name__=='__main__':unittest.main()
