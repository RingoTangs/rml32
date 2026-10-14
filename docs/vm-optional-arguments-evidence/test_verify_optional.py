#!/usr/bin/env python3
"""Synthetic metadata checks; never execute VM instructions."""
import importlib.util
from pathlib import Path
import sys
import struct
import unittest
sys.dont_write_bytecode = True
spec = importlib.util.spec_from_file_location('optional_stats', Path(__file__).with_name('verify_optional.py'))
stats = importlib.util.module_from_spec(spec)
spec.loader.exec_module(stats)


class OptionalStatisticsTests(unittest.TestCase):
    def test_fixed_arity_range(self):
        self.assertEqual(stats.relation(2, 2, 2), 'within_bounds')

    def test_partial_defaults(self):
        self.assertEqual(stats.relation(2, 1, 3), 'within_bounds')
        self.assertEqual(stats.layout([3, 2], 9), {'valid': True, 'spans': [[1, 4], [4, 6]], 'end': 6})

    def test_all_optional(self):
        self.assertEqual(stats.relation(0, 0, 3), 'within_bounds')

    def test_excess_is_raw_relation_not_rejection(self):
        self.assertEqual(stats.relation(8, 1, 2), 'above_max')

    def test_below_min(self):
        self.assertEqual(stats.relation(0, 1, 2), 'below_min')

    def test_invalid_default_code_range(self):
        self.assertFalse(stats.layout([4, 6], 10)['valid'])
        self.assertFalse(stats.layout([-1], 10)['valid'])

    def test_empty_default_spans(self):
        self.assertEqual(stats.layout([0, 0], 1)['end'], 1)

    def test_native_entry_targets(self):
        f = {'min_args': 0, 'max_args': 2,
             'arguments': [{'index': 0, 'flag': 0, 'type': 255}, {'index': 1, 'flag': 0, 'type': 255}],
             'code': {'nodes': [[5, 0, 15]] + [[0, 0, 1]] * 8}}
        raws = []
        for n in (3, 2):
            b = bytearray(20)
            struct.pack_into('<h', b, 10, n)
            raws.append({'raw_hex': b.hex()})
        result = stats.entry_layout(f, raws)
        self.assertEqual([t['target'] for t in result['argc_targets']], [1, 4, 6])
        self.assertTrue(result['entry_a_equals_length_sum'])
        self.assertFalse(result['target_outside_code'])

    def test_omitted_reference_and_default_hole_are_visible(self):
        f = {'min_args': 0, 'max_args': 2,
             'arguments': [{'index': 0, 'flag': 1, 'type': 255},
                           {'index': 1, 'flag': 0, 'type': 255}],
             'code': {'nodes': [[2, 0, 15], [0, 0, 7], [0, 0, 0x47], [0, 0, 0x1b]]}}
        raws = []
        for n in (0, 1):
            b = bytearray(20)
            struct.pack_into('<h', b, 10, n)
            raws.append({'raw_hex': b.hex()})
        result = stats.entry_layout(f, raws)
        self.assertEqual(result['omission_cases'][0]['omitted_reference_indices'], [0])
        self.assertTrue(result['omission_cases'][0]['zero_default_hole_before_expression'])
        self.assertTrue(result['default_contains_call'])
        self.assertTrue(result['default_tail_layout_matches'])

    def test_entry_a_mismatch_is_reported(self):
        f = {'min_args': 0, 'max_args': 0, 'arguments': [],
             'code': {'nodes': [[5, 0, 15], [0, 0, 1]]}}
        result = stats.entry_layout(f, [])
        self.assertFalse(result['entry_a_equals_length_sum'])
        self.assertTrue(result['target_outside_code'])

    def test_entry_layout_negative_default_does_not_index_backwards(self):
        raw = bytearray(20)
        struct.pack_into('<h', raw, 10, -2)
        f = {'min_args': 0, 'max_args': 1,
             'arguments': [{'index': 0, 'flag': 0, 'type': 255}],
             'code': {'nodes': [[-1, 0, 15], [0, 0, 7], [0, 0, 0x47], [0, 0, 0x1b]]}}
        result = stats.entry_layout(f, [{'raw_hex': raw.hex()}])
        self.assertFalse(result['spans']['valid'])
        self.assertFalse(result['default_tail_layout_matches'])
        self.assertEqual(result['default_opcodes'], {})
        self.assertIsNone(result['after_default_expressions_opcode'])

    def test_entry_layout_default_overflow_does_not_slice_partial_region(self):
        raw = bytearray(20)
        struct.pack_into('<h', raw, 10, 20)
        f = {'min_args': 0, 'max_args': 1,
             'arguments': [{'index': 0, 'flag': 0, 'type': 255}],
             'code': {'nodes': [[21, 0, 15], [0, 0, 7]]}}
        result = stats.entry_layout(f, [{'raw_hex': raw.hex()}])
        self.assertFalse(result['spans']['valid'])
        self.assertFalse(result['default_tail_layout_matches'])
        self.assertTrue(result['target_outside_code'])
        self.assertEqual(result['default_opcodes'], {})

    def test_entry_layout_body_target_overflow_is_false(self):
        raw = bytearray(20)
        struct.pack_into('<h', raw, 10, 1)
        f = {'min_args': 0, 'max_args': 1,
             'arguments': [{'index': 0, 'flag': 0, 'type': 255}],
             'code': {'nodes': [[2, 0, 15], [0, 0, 7], [0, 0, 0x47]]}}
        result = stats.entry_layout(f, [{'raw_hex': raw.hex()}])
        self.assertTrue(result['spans']['valid'])
        self.assertFalse(result['default_tail_layout_matches'])
        self.assertTrue(result['target_outside_code'])
        self.assertIsNone(result['body_opcode'])

    def test_reference_and_vararg_shape_preserved(self):
        f = {'attrib': 2, 'min_args': 0, 'max_args': 2,
             'arguments': [{'flag': 1}, {'flag': 0}], 'code': {'nodes': [[0, 0, 15]]}}
        self.assertEqual(stats.shape(f)['reference_arguments'], 1)
        self.assertEqual(stats.shape(f)['attrib_low2'], 2)


if __name__ == '__main__':
    unittest.main()
