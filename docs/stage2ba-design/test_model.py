from copy import deepcopy
import unittest
from model import World, Rejected, audit

class LifecycleTests(unittest.TestCase):
    def world(self):
        w=World();w.apply('object','a');w.apply('object','b');return w
    def test_live_zero_does_not_delete(self):
        w=self.world();w.apply('put','x',('object','a'));w.apply('drop','x')
        self.assertEqual(w.objects['a'].state,'alive')
    def test_immediate_delete_retained_shell(self):
        w=self.world();w.apply('put','x',('object','a'));w.apply('delete','a')
        self.assertEqual(w.objects['a'].state,'shell');w.apply('drop','x')
        self.assertEqual(w.objects['a'].state,'freed')
    def test_pending_and_queue_hold(self):
        w=self.world();w.apply('enter','f','a');w.apply('delete','a')
        self.assertEqual(w.objects['a'].state,'pending')
        w.apply('put','x',('object','a'));w.apply('exit','f')
        self.assertEqual(w.objects['a'].refs,2);w.apply('drain')
        self.assertEqual(w.objects['a'].state,'shell');w.apply('drop','x')
    def test_two_frames_and_lifo(self):
        w=self.world()
        for o in ('a','b'):w.apply('enter',o,o);w.apply('delete',o)
        w.apply('exit','a');w.apply('exit','b');self.assertEqual(w.queue,['b','a'])
        w.apply('drain');self.assertEqual(w.objects['b'].state,'freed')
    def test_creator_and_capture_same_or_distinct(self):
        for captured in ('a','b'):
            w=self.world();w.apply('put','x',('object',captured))
            w.apply('function','h','a',['x'],'f');w.apply('copy','f','g')
            self.assertEqual(w.objects['a'].refs,3 if captured=='a' else 1)
            w.apply('drop','f');self.assertIn('h',w.heads)
            w.apply('drop','g');self.assertFalse(w.heads);audit(w)
    def test_move_and_expansion(self):
        w=self.world();w.apply('put','x',('object','b'))
        w.apply('function','h','a',['x'],'f');w.apply('move','f','g')
        w.apply('expand','h',0,'out');self.assertEqual(w.objects['b'].refs,3)
    def test_object_variable_breaks_creator_cycle(self):
        w=self.world();w.apply('function','h','a',[],'var:a:f')
        w.apply('delete','a');self.assertEqual(w.objects['a'].state,'freed')
    def test_host_retirement_close_transfer(self):
        w=self.world();w.apply('put','host:x',('object','a'));w.apply('transfer')
        w.apply('close');w.apply('retire','host:x');self.assertEqual(w.objects['a'].refs,1)
        w.apply('drain-host');self.assertEqual(w.objects['a'].refs,0)
    def test_atomic_failure(self):
        w=self.world();before=w.key()
        with self.assertRaises(Rejected):w.apply('put','x',('object','a'),fail=True)
        self.assertEqual(w.key(),before)
    def test_invalid_copy_normalizes_source(self):
        w=self.world();w.apply('put','x',('object','a'));w.apply('delete','a');w.apply('copy','x','y')
        self.assertEqual(w.slots['x'],('undefined',None));self.assertEqual(w.objects['a'].state,'freed')
    def test_corrupt_counter(self):
        w=self.world();w.objects['a'].refs=1
        with self.assertRaises(AssertionError):audit(w)
    def test_corrupt_queue_ref(self):
        w=self.world();w.apply('enter','f','a');w.apply('delete','a');w.apply('exit','f');w.objects['a'].refs-=1
        with self.assertRaises(AssertionError):audit(w)
    def test_corrupt_creator_release(self):
        w=self.world();w.apply('function','h','a',[],'f');w.objects['a'].refs-=1
        with self.assertRaises(AssertionError):audit(w)
    def test_early_free_and_double_enqueue(self):
        w=self.world();w.apply('put','x',('object','a'));w.objects['a'].state='freed'
        with self.assertRaises(AssertionError):audit(w)
        w=self.world();w.queue=['a','a']
        with self.assertRaises(AssertionError):audit(w)
    def test_two_heads_six_slots(self):
        w=self.world();w.apply('put','x',('object','a'));w.apply('put','y',('object','b'))
        w.apply('function','h','a',['x'],'f');w.apply('function','j','b',['y'],'g')
        self.assertEqual(len(w.slots),6);audit(w)
    def test_snapshot_adds_no_owner(self):
        w=self.world();w.apply('put','x',('object','a'));before=w.key();w.apply('snapshot','x')
        self.assertEqual(w.key(),before)
    def test_overwrite_releases(self):
        w=self.world();w.apply('put','x',('object','a'));w.apply('put','y',('object','b'))
        w.apply('overwrite','x','y');self.assertEqual(w.objects['b'].refs,0)
    def test_invalid_use_atomic(self):
        w=self.world();w.apply('delete','a');before=w.key()
        with self.assertRaises(Rejected):w.apply('enter','f','a')
        self.assertEqual(w.key(),before)

    def test_alias_does_not_add_payload_owner(self):
        w=self.world();w.apply('put','x',('object','a'));w.apply('alias','ref','x')
        self.assertEqual(w.objects['a'].refs,1)
    def test_unique_move_vs_shared_copy(self):
        for shared in (False,True):
            w=self.world();w.apply('put','x',('object','b'))
            w.apply('function','h','a',['x'],'f');w.apply('delete','b')
            if shared:w.apply('copy','f','g')
            w.apply('invoke','f',['out'])
            self.assertEqual(w.slots['out'][0],'undefined' if shared else 'object')
            self.assertEqual('h' in w.heads,shared)
    def test_cancel_restart_release_frames_without_drain(self):
        for event in ('cancel','restart'):
            w=self.world();w.apply('enter','f','a');w.apply('delete','a');w.apply(event)
            self.assertEqual(w.objects['a'].state,'queued');w.apply('drain')

    def test_pending_copy_remains_valid(self):
        w=self.world();w.apply('put','x',('object','a'));w.apply('enter','f','a')
        w.apply('delete','a');w.apply('copy','x','y')
        self.assertEqual(w.slots['y'],('object','a'))
    def test_host_retain_has_independent_retirement(self):
        w=self.world();w.apply('put','host:x',('object','a'))
        w.apply('copy','host:x','host:y');w.apply('retire','host:x');w.apply('drain-host')
        self.assertEqual(w.objects['a'].refs,1)
    def test_duplicate_cleanup_detected(self):
        w=self.world();w.apply('delete','a');w.objects['a'].cleaned=2
        with self.assertRaises(AssertionError):audit(w)
    def test_function_creation_failure_releases_nothing(self):
        w=self.world();w.apply('put','x',('object','a'));before=w.key()
        with self.assertRaises(Rejected):w.apply('function','h','a',['x'],'f',fail=True)
        self.assertEqual(w.key(),before)

    def test_self_copy_normalizes_invalid_payload(self):
        w=self.world();w.apply('put','x',('object','a'));w.apply('delete','a')
        w.apply('copy','x','x');self.assertEqual(w.slots['x'],('undefined',None))
