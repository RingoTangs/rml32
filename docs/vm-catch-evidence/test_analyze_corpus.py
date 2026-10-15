"""Synthetic structural fixtures; never execute reference bytecode."""
import unittest
from analyze_corpus import audit


def program(nodes, init=None):
    def code(value):
        return dict(offset=20, nodes_offset=40, nodes=value)
    return dict(source='synthetic.pak/member', sha256='0'*64,
                functions=[dict(name='sample', index=3, code=code(nodes))], hidden=[],
                init=code(init or []))


class CatchCorpusTests(unittest.TestCase):
    def test_empty_input(self):
        summary, examples = audit([])
        self.assertEqual(summary['totals']['programs'], 0)
        self.assertEqual(summary['errors'], [])
        self.assertEqual(examples, {})
        self.assertTrue(all(v['count'] == 0 for v in summary['opcodes'].values()))

    def test_nested_candidate_intervals(self):
        # Outer target8, inner target5. These are lexical intervals, not a CFG proof.
        nodes = [[7,0,0x2c], [3,0,0x2c], [0,0,0x49], [0,0,0x1a],
                 [1,0,0x1f], [0,0,0x0e], [0,0,0x1a], [1,0,0x1f], [0,0,0x0e]]
        summary, examples = audit([program(nodes)])
        patterns = summary['lexical_candidate_patterns']
        self.assertEqual(patterns['max_candidate_interval_overlap'], 2)
        self.assertEqual(patterns['contains_nested_start'], 1)
        self.assertEqual(patterns['return_before_first_lexical_end'], 2)
        self.assertEqual(patterns['end_then_jump_immediately_before_target'], 2)
        self.assertEqual(summary['candidate_target_opcodes'], {'0x0e': 2})
        self.assertEqual(examples['contains_nested_start']['member_node_offset'], 40)
        self.assertEqual(examples['contains_nested_start']['candidate_target_member_offset'], 72)

    def test_out_of_bounds_target_reports_both_directions(self):
        for operand in (-2, 1):
            with self.subTest(operand=operand):
                summary, _ = audit([program([[operand,0,0x2c]])])
                self.assertEqual(summary['errors'][0]['kind'], 'candidate-target-bounds')
                self.assertEqual(summary['checks']['candidate_targets_checked'], 1)

    def test_truncated_node_rejects_segment_without_guessing(self):
        summary, _ = audit([program([[2,0,0x2c], [0,0]])])
        self.assertEqual(summary['errors'][0]['kind'], 'malformed-node')
        self.assertEqual(summary['errors'][0]['member_node_offset'], 44)
        self.assertEqual(summary['checks']['segments_skipped_malformed'], 1)
        self.assertEqual(summary['opcodes']['0x2c']['count'], 0)

    def test_operand_width_and_type_validation(self):
        for node in ([32768,0,0x2c], [0,-129,0x1a], [False,0,0x0e], [0,0,256], None):
            with self.subTest(node=node):
                summary, _ = audit([program([node])])
                self.assertEqual(summary['errors'][0]['kind'], 'malformed-node')

    def test_loop_and_crosscall_candidates_not_execution(self):
        summary, examples = audit([program([[3,0,0x2c], [0,0,0x23], [0,0,0x08],
                                           [0,0,0x1a], [0,0,0x0e]])])
        patterns = summary['lexical_candidate_patterns']
        self.assertEqual(patterns['contains_loop'], 1)
        self.assertEqual(patterns['contains_cross_object_call'], 1)
        self.assertEqual(patterns['contains_any_call'], 1)
        self.assertEqual(examples['contains_loop']['lexical_region'], [1,4])
        self.assertIn('candidate', summary['target_formula_status'])

    def test_nonforward_interval_not_treated_as_body(self):
        summary, _ = audit([program([[-1,0,0x2c]])])
        self.assertEqual(summary['checks']['nonforward_candidate_targets'], 1)
        self.assertEqual(summary['lexical_candidate_patterns'], {})

    def test_initialization_segment_is_scanned(self):
        summary, examples = audit([program([], [[0,0,0x0e]])])
        self.assertEqual(summary['opcodes']['0x0e']['by_segment_kind'], {'init': 1})
        self.assertEqual(examples['opcode_0x0e']['segment_kind'], 'init')

    def test_no_invented_pairing_for_end_without_start(self):
        summary, _ = audit([program([[0,0,0x1a], [0,0,0x0e]])])
        self.assertEqual(summary['opcodes']['0x1a']['count'], 1)
        self.assertEqual(summary['lexical_candidate_patterns'], {})


if __name__ == '__main__':
    unittest.main()
