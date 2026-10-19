"""Synthetic audit tests; no reference binary or bytecode execution."""
import copy
import gzip
import json
from pathlib import Path
import tempfile
import unittest
from analyze_corpus import audit, artifacts, save_or_check


def image():
    def code(nodes):
        return dict(offset=10, nodes_offset=20, nodes=nodes)
    return dict(source='data/test.pak/a.o', sha256='synthetic',
                init=code([[0,0,1],[-32768,-128,0x10]]),
                functions=[dict(name='f', index=0, min_args=1, max_args=2, local_count=3,
                                arguments=[dict(index=0,flag=1)], code=code([[32767,127,0x10]]))],
                hidden=[dict(name='hidden', code=code([[0,0,0x10]]))])


class AuditTests(unittest.TestCase):
    def test_all_contexts_signed_operands_and_offsets(self):
        summary, examples, rows = audit([image()])
        self.assertEqual(summary['totals'], dict(programs=1, ordinary_functions=1,
                                                hidden_functions=1, instructions=4))
        self.assertEqual(summary['occurrence_count'], 3)
        self.assertEqual(summary['by_segment_kind'], dict(init=1, functions=1, hidden=1))
        self.assertEqual(summary['by_source_package'], {'data/test.pak':3})
        self.assertEqual(rows[0]['member_node_offset'],24)
        self.assertEqual(rows[0]['raw_operands'],[-32768,-128])
        self.assertEqual(rows[1]['prototype'],dict(min_args=1,max_args=2,local_count=3))
        self.assertEqual(rows[1]['argument_flags'],[dict(index=0,flag=1)])
        self.assertEqual(summary['by_slot_index'],{'-32768':1,'0':1,'32767':1})
        self.assertEqual(examples[0]['snippet'][0]['node'],[0,0,1])

    def test_every_occurrence_retained_but_representatives_grouped(self):
        p = image()
        p['init']['nodes'].append([-32768,-128,0x10])
        summary, examples, rows = audit([p])
        self.assertEqual(len(rows),4)
        self.assertEqual(len(examples),3)
        self.assertEqual(summary['raw_operand_combinations'][0]['count'],2)

    def test_no_opcode_is_valid(self):
        p = image()
        for code in [p['init'],p['functions'][0]['code'],p['hidden'][0]['code']]:
            code['nodes'] = [[0,0,1]]
        summary, examples, rows = audit([p])
        self.assertEqual((summary['occurrence_count'],examples,rows),(0,[],[]))

    def test_malformed_nodes_rejected(self):
        for node in ([0,0], [32768,0,69], [0,-129,69], [0,0,86],
                     [True,0,69], [0,0,'69'], 'bad'):
            with self.subTest(node=node):
                p = image()
                p['init']['nodes'] = [node]
                with self.assertRaises(ValueError):
                    audit([p])

    def test_invalid_container_and_offset_rejected(self):
        with self.assertRaises(ValueError):
            audit({})
        for offset in (-1, True, '20', 9):
            p = image()
            p['init']['nodes_offset'] = offset
            with self.assertRaises(ValueError):
                audit([p])

    def test_deterministic_output_and_no_input_mutation(self):
        p = image()
        before = copy.deepcopy(p)
        first = artifacts(*audit([p]))
        self.assertEqual(first,artifacts(*audit([p])))
        self.assertEqual(p,before)
        self.assertEqual(len(json.loads(first['corpus-occurrences.json'])),3)

    def test_check_does_not_repair_stale_artifact(self):
        output = artifacts(*audit([image()]))
        with tempfile.TemporaryDirectory() as tmp:
            directory = Path(tmp)
            save_or_check(output,directory,True)
            save_or_check(output,directory)
            stale = directory/'corpus-summary.json'
            stale.write_bytes(b'corrupt')
            with self.assertRaises(ValueError):
                save_or_check(output,directory)
            self.assertEqual(stale.read_bytes(),b'corrupt')
            save_or_check(output,directory,True)
            save_or_check(output,directory)


if __name__ == '__main__':
    unittest.main()
