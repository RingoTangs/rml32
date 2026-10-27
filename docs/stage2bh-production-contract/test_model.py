import unittest
from copy import deepcopy
from quota_model import World,LifecycleLimits as L,LifecycleBudget as B,Rejected,audit,checked_add,SCALAR

def world(**limits):
    values=dict(max_objects=2,max_function_heads=2,max_ordinary_refs_per_object=4,max_exec_refs_per_object=2,max_owners_per_header=3);values.update(limits);return World(L(**values))
def obj(w,token='o'):
    n=w.apply('reserve_object',token);w.apply('commit',token);return n
def head(w,kind='function',creator=0,dest='host:f',token='h'):
    n=w.apply('reserve_header',token,kind,creator if kind=='function'else None,dest);w.apply('commit',token);return n
def frame(w,o=0,name='f',token='e'):
    w.apply('reserve_exec',token,o,name);w.apply('commit',token)
def maintain(w,host=0,objects=0,metadata=0,**kw):return w.apply('maintain',B(host,objects,metadata),**kw)
class ModelTests(unittest.TestCase):
    def rejected(self,w,op,*args,kind='Limit',**kw):
        before=deepcopy(w)
        with self.assertRaises(Rejected)as caught:w.apply(op,*args,**kw)
        self.assertEqual(caught.exception.kind,kind);self.assertEqual(w,before)
    def test_explicit_limits_have_no_defaults(self):
        with self.assertRaises(TypeError):L()
    def test_limit_validation(self):
        for field,value in [('max_ordinary_refs_per_object',1),('max_exec_refs_per_object',0),('max_owners_per_header',0),('max_objects',2**32)]:
            with self.assertRaises(Rejected):world(**{field:value})
    def test_both_unsigned_widths(self):
        for bits in (32,64):
            self.assertEqual(checked_add((1<<bits)-2,1,bits),(1<<bits)-1)
            with self.assertRaises(Rejected):checked_add((1<<bits)-1,1,bits)
            with self.assertRaises(Rejected):checked_add(-1,1,bits)
    def test_zero_object_quota(self):self.rejected(world(max_objects=0),'reserve_object','r')
    def test_zero_function_quota_still_allows_arrays(self):
        w=world(max_function_heads=0);obj(w);self.rejected(w,'reserve_header','h','function',0,'host:f');head(w,'array',dest='host:a');audit(w)
    def test_object_reservation_abort_leaves_id_gap(self):
        w=world(max_objects=1);self.assertEqual(w.apply('reserve_object','r'),0);w.apply('abort','r');self.assertEqual(obj(w),1)
    def test_double_abort_is_rejected(self):
        w=world();w.apply('reserve_object','r');w.apply('abort','r');self.rejected(w,'abort','r',kind='InvalidState')
    def test_duplicate_reservation_is_rejected(self):
        w=world();w.apply('reserve_object','r');self.rejected(w,'reserve_object','r',kind='InvalidState')
    def test_failed_publication_releases_candidate_reservations(self):
        w=world();obj(w);self.rejected(w,'reserve_header','h','function',0,'host:f',fail=True);self.assertEqual(w.used('heads'),0)
    def test_header_reserves_creator_before_publication(self):
        w=world();obj(w);w.apply('reserve_header','h','function',0,'host:f');self.assertEqual(w.used('ord:0'),1);self.assertEqual(w.used('heads'),1);w.apply('abort','h');self.assertEqual(w.used('ord:0'),0)
    def test_invalidated_creator_cannot_publish_new_head(self):
        w=world();obj(w);w.apply('reserve_header','h','function',0,'host:f');w.apply('delete',0);self.rejected(w,'commit','h',kind='InvalidObject');w.apply('abort','h');audit(w)
    def test_head_copies_never_duplicate_creator(self):
        w=world();obj(w);h=head(w);w.apply('reserve_copy','r','host:f','host:g');self.assertEqual(w.used('ord:0'),1);w.apply('commit','r');self.assertEqual(w.headers[h].owners,2);w.apply('drop','host:f');self.assertEqual(w.used('ord:0'),1);w.apply('drop','host:g');self.assertEqual(w.used('ord:0'),0)
    def test_peak_limit_blocks_net_neutral_copy(self):
        w=world(max_owners_per_header=2);h=head(w,'array',dest='a');w.apply('put','b',('header',h));self.rejected(w,'reserve_copy','r','a','b');self.assertEqual(w.headers[h].owners,2)
    def test_raw_move_does_not_reserve_owner(self):
        w=world(max_owners_per_header=1);h=head(w,'array',dest='a');w.apply('move','a','b');self.assertEqual(w.used('owner:'+str(h)),1);self.assertEqual(w.slots['a'],SCALAR)
    def test_exec_handoff_counts_prospective_lease(self):
        w=world(max_exec_refs_per_object=1);obj(w);frame(w);self.rejected(w,'reserve_exec','new',0,'next');self.assertEqual(w.frames,{'f':0})
    def test_exec_handoff_with_two_units(self):
        w=world();obj(w);frame(w);w.apply('reserve_exec','new',0,'next');w.apply('commit','new');w.apply('exit','f');self.assertEqual(w.frames,{'next':0});self.assertEqual(w.used('exec:0'),1)
    def test_pending_prospective_abort_uses_reserved_queue_reference(self):
        w=world();obj(w);frame(w);w.apply('reserve_exec','new',0,'next');w.apply('delete',0);w.apply('exit','f');self.assertEqual(w.objects[0].state,'pending');w.apply('abort','new');self.assertEqual(w.queue,[0]);self.assertEqual(w.used('ord:0'),1)
    def test_ordinary_limit_keeps_queue_space(self):
        w=world(max_ordinary_refs_per_object=2);obj(w);w.apply('put','host:a',('object',0));self.rejected(w,'reserve_copy','r','host:a','host:b');frame(w);w.apply('delete',0);w.apply('exit','f');self.assertEqual(w.used('ord:0'),2);audit(w)
    def test_shell_raw_retain_and_script_copy_are_distinct(self):
        w=world(max_ordinary_refs_per_object=2);obj(w);w.apply('put','host:a',('object',0));w.apply('delete',0);w.apply('reserve_copy','r','host:a','host:b');w.apply('commit','r');self.assertEqual(w.used('ord:0'),2);w.apply('reserve_copy','s','host:a','host:c',script=True);w.apply('commit','s');self.assertEqual(w.slots['host:a'],SCALAR);self.assertEqual(w.slots['host:c'],SCALAR);self.assertEqual(w.used('ord:0'),1)
    def test_freed_raw_id_does_not_resurrect(self):
        w=world();obj(w);w.apply('delete',0);self.rejected(w,'put','host:a',('object',0),kind='InvalidObject');maintain(w,metadata=8);self.rejected(w,'put','host:a',('object',0),kind='InvalidObject')
    def test_source_version_conflict_keeps_reservation_until_abort(self):
        w=world();obj(w);w.apply('put','host:a',('object',0));w.apply('reserve_copy','r','host:a','host:b');w.apply('put','host:a',SCALAR);self.rejected(w,'commit','r',kind='InvalidState');w.apply('abort','r');self.assertEqual(w.used('ord:0'),0)
    def test_failed_immediate_delete_is_atomic(self):
        w=world();obj(w);w.apply('put','var:0:a',SCALAR);self.rejected(w,'delete',0,borrowed=True,kind='InvalidState');self.rejected(w,'delete',0,workspace=True);self.assertFalse(w.objects[0].cleaned)
    def test_zero_exec_task_cleanup_is_deferred_and_idempotent(self):
        w=world();obj(w);w.apply('put','var:0:a',('object',0));w.apply('task_cleanup',0);w.apply('task_cleanup',0);self.assertEqual(w.queue,[0]);self.assertFalse(w.objects[0].cleaned);self.assertEqual(w.used('ord:0'),2)
    def test_zero_budget_changes_nothing(self):
        w=world();obj(w);w.apply('task_cleanup',0);before=deepcopy(w);p=maintain(w);self.assertEqual(w,before);self.assertEqual(p['stop'],'BudgetExhausted');self.assertEqual(p['drained_objects'],0)
    def test_lifo_partial_workspace_progress(self):
        w=world();obj(w,'a');obj(w,'b');w.apply('task_cleanup',0);w.apply('task_cleanup',1);p=maintain(w,objects=2,workspace_objects=(0,));self.assertEqual(p['drained_objects'],1);self.assertEqual(p['stop'],'WorkspaceLimit');self.assertEqual(w.queue,[0]);self.assertEqual(w.objects[1].state,'freed');self.assertFalse(w.objects[0].cleaned)
    def test_borrowed_head_is_not_skipped(self):
        w=world();obj(w,'a');obj(w,'b');w.apply('task_cleanup',0);w.apply('task_cleanup',1);p=maintain(w,objects=2,blocked_objects=(1,));self.assertEqual(p['stop'],'BorrowBlocked');self.assertEqual(w.queue,[1,0]);self.assertEqual(p['drained_objects'],0)
    def test_root_budget_and_borrowed_retry(self):
        w=world();obj(w);w.apply('put','host:a',('object',0));w.apply('put','host:b',('object',0));w.apply('retire_host','host:b');w.apply('retire_host','host:a');p=maintain(w,host=1);self.assertEqual(p['retired_roots'],1);self.assertEqual(w.retired,{'host:b'});p=maintain(w,host=1,blocked_roots=('host:b',));self.assertEqual(p['stop'],'BorrowBlocked');p=maintain(w,host=1);self.assertEqual(p['retired_roots'],1);self.assertEqual(w.used('ord:0'),0)
    def test_host_safepoint_never_drains_object_queue(self):
        w=world();obj(w);w.apply('task_cleanup',0);w.apply('put','host:a',SCALAR);w.apply('retire_host','host:a');w.apply('host_safepoint');self.assertEqual(w.queue,[0]);self.assertFalse(w.objects[0].cleaned)
    def test_reap_returns_capacity_without_reusing_id(self):
        w=world(max_objects=1);obj(w);w.apply('delete',0);self.rejected(w,'reserve_object','r');p=maintain(w,metadata=1);self.assertEqual(p['reaped_objects'],1);self.assertEqual(obj(w),1)
    def test_retired_head_charged_until_metadata_reclaimed(self):
        w=world(max_function_heads=1);obj(w);h=head(w);w.apply('view',h);w.apply('drop','host:f');maintain(w,metadata=8);self.rejected(w,'reserve_header','new','function',0,'host:g');w.apply('drop_view',h);p=maintain(w,metadata=16);self.assertEqual(p['reclaimed_heads'],1);head(w,dest='host:g');audit(w)
    def test_metadata_cursor_resumes_and_new_generation_gets_pass(self):
        w=world();obj(w);h=head(w);p=maintain(w,metadata=1);self.assertEqual(p['metadata_examined'],1);w.apply('drop','host:f');p=maintain(w,metadata=1);self.assertEqual(p['reclaimed_heads'],1);self.assertFalse(p['scan_complete']);p=maintain(w,metadata=1);self.assertEqual(p['stop'],'Complete')
    def test_epoch_wrap_marks_rescan_without_failing_drop(self):
        w=world();obj(w);w.epoch=2**64-1;w.scan_epoch=0;w.scan_pending=[];w.scan_complete=True
        w.apply('put','host:x',SCALAR);self.assertEqual(w.epoch,0);self.assertTrue(w.force_rescan);self.assertFalse(w.scan_done())
        p=maintain(w,metadata=1);self.assertEqual(p['metadata_examined'],1);self.assertTrue(p['scan_complete']);self.assertFalse(w.force_rescan)
    def test_weak_rows_are_origin_local_and_budgeted(self):
        a=world();b=world();obj(a);obj(b);a.apply('weak','wa','object',0);b.apply('weak','wb','object',0);a.apply('delete',0);before=deepcopy(b);p=maintain(a,metadata=16);self.assertEqual(p['pruned_weak_entries'],1);self.assertEqual(b,before);self.assertFalse(a.objects)
    def test_metadata_borrow_preserves_cursor(self):
        w=world();obj(w);before=deepcopy(w);p=maintain(w,metadata=1,blocked_metadata=(('object',0),));self.assertEqual(p['stop'],'BorrowBlocked');self.assertEqual(p['metadata_examined'],0);self.assertEqual(w.scan_pending,[('object',0)]);self.assertEqual(w.ledger,before.ledger)
    def test_invalid_mode_and_closed_epoch_fail_before_work(self):
        w=world();w.mode=False;self.rejected(w,'maintain',B(1,1,1),kind='UnsupportedFeature');w.mode=True;w.closed=True;self.rejected(w,'maintain',B(1,1,1),kind='InvalidState')
    def test_namespace_exhaustion_is_not_quota_recovery(self):
        w=world();w.next_object=2**32-1;self.rejected(w,'reserve_object','r');w.next_header=2**64-1;self.rejected(w,'reserve_header','h','array',None,'host:a')
    def test_negative_audits_detect_missing_and_duplicate_accounting(self):
        w=world();obj(w);head(w);bad=deepcopy(w);bad.ledger['ord:0']=0
        with self.assertRaises(AssertionError):audit(bad)
        bad=deepcopy(w);bad.ledger['heads']+=1
        with self.assertRaises(AssertionError):audit(bad)
        bad=deepcopy(w);bad.headers[1].creator_held=False
        with self.assertRaises(AssertionError):audit(bad)
    def test_forged_reservation_is_detected(self):
        w=world();w.apply('reserve_object','r');w.reservations['r'].claims['objects']=2;w.ledger['objects']=2
        with self.assertRaises(AssertionError):audit(w)
    def test_dag_members_release_once_without_duplicate_creator(self):
        w=world();obj(w);inner=head(w,'array',dest='host:a');w.apply('member',inner,'member:a',('object',0));f=head(w,dest='host:f');w.apply('member',f,'capture:f',('header',inner));self.assertEqual(w.used('ord:0'),2);w.apply('drop','host:a');w.apply('drop','host:f');self.assertEqual(w.used('ord:0'),0);audit(w)
if __name__=='__main__':unittest.main()
