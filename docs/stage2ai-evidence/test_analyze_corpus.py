import copy
import json
from pathlib import Path
import tempfile
import unittest

from analyze_corpus import OPS, artifacts, audit, check_baseline, verify_outputs


def program(nodes, source='gs/test.o', mode='object'):
    return dict(source=source, sha256='0'*64, mode=mode, efuns_count=559,
                functions=[dict(name='f', index=7, code=dict(offset=8, nodes_offset=12, nodes=nodes))],
                hidden=[], init=dict(offset=0, nodes_offset=4, nodes=[]))


class CorpusTests(unittest.TestCase):
    def test_all_seven_opcodes_count_duplicate_occurrences(self):
        summary, examples = audit([program([[0, 0, op] for op in OPS]*2)])
        self.assertEqual(summary['occurrence_count'], 14)
        self.assertEqual(summary['histograms']['opcodes'], {f'{op:02x}': 2 for op in OPS})
        self.assertIn('not proven operands', summary['limitation'])

    def test_adjacency_is_recorded_without_claiming_operand_sides(self):
        summary, examples = audit([program([[0, 0, 0x40], [-1, 0, 0x37], [0, 0, 1]])])
        self.assertEqual(summary['histograms']['preceding_pair'], {'01:shared_string_pool_index:local_slot': 1})
        row = examples[0]
        self.assertEqual(row['pc'], 2)
        self.assertEqual(row['member_node_offset'], 20)
        self.assertEqual([n['pc'] for n in row['snippet']], [0, 1, 2])
        self.assertNotIn('lhs', row)
        self.assertNotIn('rhs', row)

    def test_first_instruction_and_unknown_origin(self):
        summary, _ = audit([program([[0, 0, 1], [0, 0, 6], [0, 0, 28]])])
        self.assertEqual(summary['histograms']['preceding_origin'],
                         {'01:segment_start': 1, '1c:other_instruction': 1})

    def test_all_contexts_and_offsets(self):
        p = program([[0, 0, 1]], mode='global')
        p['init']['nodes'] = [[0, 0, 28]]
        p['hidden'] = [dict(name='hidden', index=-1, code=dict(offset=20, nodes_offset=24, nodes=[[0, 0, 42]]))]
        summary, examples = audit([p])
        self.assertEqual(summary['totals'], dict(programs=1, ordinary_functions=1, hidden_functions=1, instructions=3))
        self.assertEqual(summary['histograms']['contexts'],
                         {'01:global:functions': 1, '1c:global:init': 1, '2a:global:hidden': 1})
        self.assertEqual({r['member_node_offset'] for r in examples}, {4, 12, 24})

    def test_samples_are_deterministic_and_bounded_by_category(self):
        a = program([[0, 0, 1]]*10, source='gs/a.o')
        b = program([[0, 0, 1]]*10, source='gs/b.o')
        first = audit([a, b])
        self.assertEqual(artifacts(*first), artifacts(*audit([b, a])))
        self.assertTrue(all(row['source'] == 'gs/a.o' for row in first[1]))
        self.assertEqual(len(first[1]), 2)

    def test_malformed_profile_hash_offsets_and_nodes_rejected(self):
        for field, value in [('mode', 'other'), ('efuns_count', 389), ('sha256', 'z'*64)]:
            p = program([]); p[field] = value
            with self.subTest(field=field), self.assertRaises(ValueError):
                audit([p])
        for node in ([False, 0, 1], [0, 128, 1], [32768, 0, 1], [0, 0, 86], [0, 0]):
            with self.subTest(node=node), self.assertRaises(ValueError):
                audit([program([node])])
        p = program([]); p['init']['nodes_offset'] = -1
        with self.assertRaises(ValueError): audit([p])
        p = program([])
        with self.assertRaises(ValueError): audit([p, p])

    def test_independent_baseline_detects_wrong_count_each_opcode(self):
        summary, _ = audit([program([[0, 0, op] for op in OPS])])
        baseline = dict(totals=dict(programs=1, functions=1, hidden_functions=0, instructions=7),
                        opcodes={hex(op): 1 for op in OPS})
        check_baseline(summary, baseline)
        for op in OPS:
            bad = copy.deepcopy(baseline); bad['opcodes'][hex(op)] = 2
            with self.assertRaises(ValueError): check_baseline(summary, bad)
        baseline['totals']['programs'] = 2
        with self.assertRaises(ValueError): check_baseline(summary, baseline)

    def test_default_verification_does_not_write_and_detects_corruption(self):
        outputs = artifacts(*audit([program([[0, 0, 1]])]))
        with tempfile.TemporaryDirectory() as tmp:
            directory = Path(tmp)
            with self.assertRaises(ValueError): verify_outputs(outputs, directory)
            self.assertEqual(list(directory.iterdir()), [])
            verify_outputs(outputs, directory, write=True)
            verify_outputs(outputs, directory)
            for name in outputs:
                original = (directory/name).read_bytes()
                (directory/name).write_bytes(original+b' ')
                with self.assertRaises(ValueError): verify_outputs(outputs, directory)
                self.assertEqual((directory/name).read_bytes(), original+b' ')
                (directory/name).write_bytes(original)

    def test_changed_input_hash_and_sample_offset_fail_verification(self):
        summary, examples = audit([program([[0, 0, 1]])])
        summary['input_sha256'] = {'input': 'a'*64}
        with tempfile.TemporaryDirectory() as tmp:
            directory = Path(tmp)
            verify_outputs(artifacts(summary, examples), directory, write=True)
            for kind in ('hash', 'offset'):
                bad_summary, bad_examples = copy.deepcopy(summary), copy.deepcopy(examples)
                if kind == 'hash': bad_summary['input_sha256']['input'] = 'b'*64
                else: bad_examples[0]['member_node_offset'] += 4
                with self.assertRaises(ValueError):
                    verify_outputs(artifacts(bad_summary, bad_examples), directory)


if __name__ == '__main__': unittest.main()
