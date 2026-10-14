"""Synthetic metadata tests; never execute bytecode."""
import importlib.util
from pathlib import Path
import sys
import unittest
sys.dont_write_bytecode = True
spec = importlib.util.spec_from_file_location('inheritance_report', Path(__file__).with_name('verify_inheritance.py'))
m = importlib.util.module_from_spec(spec)
spec.loader.exec_module(m)


def program(name, parents=(), cc=1, nf=0, nv=0):
    return dict(source='s/p.pak/' + name + '.o', constitution_count=cc,
                max_functions=nf, max_vars=nv, dlts=[], functions=[],
                inherits=[dict(name='/' + p + '.c', offsets=o) for p, o in parents])


class InheritanceTests(unittest.TestCase):
    def test_offline_name_policy(self):
        self.assertEqual(m.resolve_name('s/p.pak/a.o', '/dir/base.c'), 's/p.pak/dir/base.o')
        self.assertIsNone(m.resolve_name('not-package', '/base.c'))

    def test_leaf_depth_zero(self):
        self.assertEqual(m.analyze([program('a')])['summary']['depth_histogram'], {'0': 1})

    def test_multilevel_relocation(self):
        ps = [program('a', [('b', [1, 2, 3, 2, 1, 1])], 3, 3, 4),
              program('b', [('c', [1, 0, 0, 1, 1, 1])], 2, 1, 1),
              program('c', nf=1, nv=1)]
        report = m.analyze(ps)
        a = next(n for n in report['nodes'] if n['source'].endswith('/a.o'))
        self.assertEqual(a['depth'], 2)
        self.assertEqual(a['relocated_constitutions'][2]['base_function'], 2)
        self.assertEqual(report['summary']['relocation_issues'], {})

    def test_init_order_parent_before_child(self):
        r = m.analyze([program('a', [('b', [1, 0, 0, 1, 0, 0])], 2), program('b')])
        self.assertEqual(r['init_orders']['s/p.pak/a.o'], [1, 0])
        self.assertEqual(r['summary']['init_order_issues'], {})

    def test_concrete_inherited_call_target(self):
        a = program('a', [('b', [1, 1, 0, 1, 1, 0])], 2, 2)
        a['functions'] = [dict(index=0, offset=30, name='caller', code=dict(nodes=[[1, 0, 7]], nodes_offset=100))]
        b = program('b', nf=1)
        b['functions'] = [dict(index=0, offset=20, name='body', code=dict(nodes=[[0, 0, 27]], nodes_offset=80))]
        r = m.analyze([a, b])
        call = r['root_local_function_calls'][0]
        self.assertEqual(call['status'], 'concrete_resolved')
        self.assertEqual(call['target']['constitution'], 1)
        self.assertEqual(call['target']['body_program'], 's/p.pak/b.o')

    def test_empty_declaration_target_stays_unknown(self):
        a = program('a', nf=2)
        a['functions'] = [dict(index=0, offset=30, name='caller', code=dict(nodes=[[1, 0, 7]], nodes_offset=100)), dict(index=1, offset=50, name='decl', code=dict(nodes=[], nodes_offset=120))]
        r = m.analyze([a])
        self.assertEqual(r['root_local_function_calls'][0]['status'], 'declaration_alias_unknown')

    def test_same_name_concrete_parent_and_child_keep_distinct_slots(self):
        child = program('child', [('parent', [1, 1, 0, 1, 1, 0])], 2, 2)
        parent = program('parent', nf=1)
        child['functions'] = [dict(index=0, offset=30, name='same', code=dict(nodes=[[0, 0, 27]], nodes_offset=100))]
        parent['functions'] = [dict(index=0, offset=20, name='same', code=dict(nodes=[[0, 0, 27]], nodes_offset=80))]
        r = m.analyze([child, parent])
        slots = r['prealias_tables']['s/p.pak/child.o']
        self.assertEqual([entry['slot'] for entry in slots], [0, 1])
        self.assertEqual([entry['body_program'] for entry in slots], ['s/p.pak/child.o', 's/p.pak/parent.o'])
        self.assertEqual([entry['constitution'] for entry in slots], [0, 1])

    def test_same_name_candidate_does_not_resolve_empty_declaration(self):
        root = program('root', nf=3)
        root['functions'] = [
            dict(index=0, offset=30, name='caller', code=dict(nodes=[[1, 0, 7]], nodes_offset=100)),
            dict(index=1, offset=50, name='same', code=dict(nodes=[], nodes_offset=120)),
            dict(index=2, offset=70, name='same', code=dict(nodes=[[0, 0, 27]], nodes_offset=140))]
        r = m.analyze([root])
        self.assertEqual(r['root_local_function_calls'][0]['status'], 'declaration_alias_unknown')
        slots = r['prealias_tables']['s/p.pak/root.o']
        self.assertFalse(slots[1]['has_code'])
        self.assertTrue(slots[2]['has_code'])

    def test_missing_parent_is_preserved(self):
        r = m.analyze([program('a', [('absent', [1, 0, 0, 1, 0, 0])], 2)])
        self.assertEqual(r['summary']['statuses'], {'missing': 1})
        self.assertEqual(r['summary']['relocated_programs'], 0)

    def test_ambiguous_parent_is_preserved(self):
        r = m.analyze([program('a', [('b', [1, 0, 0, 1, 0, 0])], 2), program('b'), program('b')])
        self.assertEqual(r['summary']['statuses'], {'ambiguous': 1})

    def test_cycle_is_detected(self):
        ps = [program('a', [('b', [1, 0, 0, 2, 0, 0])], 2), program('b', [('a', [1, 0, 0, 2, 0, 0])], 2)]
        r = m.analyze(ps)
        self.assertEqual(r['summary']['cyclic_components_detected'], 1)
        self.assertEqual(r['summary']['relocated_programs'], 0)

    def test_duplicate_direct_parent(self):
        r = m.analyze([program('a', [('b', [1, 0, 0, 1, 0, 0]), ('b', [2, 0, 0, 1, 0, 0])], 3), program('b')])
        self.assertEqual(r['summary']['duplicate_direct_parent_programs'], 1)
        self.assertEqual(r['summary']['multiple_paths_to_same_ancestor'], 1)

    def test_diamond_has_multiple_ancestor_paths(self):
        ps = [program('a', [('b', [1, 0, 0, 2, 0, 0]), ('c', [3, 0, 0, 2, 0, 0])], 5),
              program('b', [('d', [1, 0, 0, 1, 0, 0])], 2),
              program('c', [('d', [1, 0, 0, 1, 0, 0])], 2), program('d')]
        self.assertEqual(m.analyze(ps)['summary']['multiple_paths_to_same_ancestor'], 1)

    def test_count_mismatch(self):
        r = m.analyze([program('a', [('b', [1, 0, 0, 2, 0, 0])], 2), program('b')])
        self.assertEqual(r['summary']['count_mismatch_edges'], 1)

    def test_interval_out_of_capacity(self):
        r = m.analyze([program('a', [('b', [1, 3, 0, 1, 2, 0])], 2, 4), program('b', nf=2)])
        self.assertEqual(r['summary']['interval_out_of_capacity_edges'], 1)

    def test_sibling_overlap_is_descriptive(self):
        ps = [program('a', [('b', [1, 0, 0, 1, 1, 0]), ('c', [2, 0, 0, 1, 1, 0])], 3, 1), program('b', nf=1), program('c', nf=1)]
        self.assertEqual(m.analyze(ps)['summary']['overlapping_sibling_intervals'], {'function': 1})

    def test_signed16_sum_overflow(self):
        value = m.intervals([1, 32767, 0, 1, 1, 0], program('b', nf=1), program('a', cc=2, nf=32767))
        self.assertFalse(value[1]['signed16_end_fits'])

    def test_negative_offset(self):
        value = m.intervals([1, -1, 0, 1, 1, 0], program('b', nf=1), program('a', cc=2, nf=1))
        self.assertFalse(value[1]['nonnegative'])

    def test_constitution_self_overlap(self):
        r = m.analyze([program('a', [('b', [0, 0, 0, 1, 0, 0])]), program('b')])
        self.assertEqual(r['summary']['constitution_reserved_self_overlap_edges'], 1)


if __name__ == '__main__':
    unittest.main()
