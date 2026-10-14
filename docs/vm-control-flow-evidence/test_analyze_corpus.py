"""Synthetic metadata and corpus-audit regression tests; never run bytecode."""
import struct
import unittest
from analyze_corpus import OPS, audit, metadata_pair


class MetadataTests(unittest.TestCase):
    def test_signed_values_are_preserved(self):
        self.assertEqual(metadata_pair(0, 0, struct.pack('<ii', -7, 2147483647)),
                         (0, [-7, 2147483647]))

    def test_pair_boundary_is_inclusive(self):
        self.assertEqual(metadata_pair(4, 0, bytes(12)), (4, [0, 0]))

    def test_negative_short_is_not_unsigned(self):
        with self.assertRaises(ValueError):
            metadata_pair(-1, 1, bytes(65544))

    def test_negative_high_byte_is_not_unsigned(self):
        with self.assertRaises(ValueError):
            metadata_pair(0, -1, bytes(8))

    def test_truncated_pair_is_rejected(self):
        with self.assertRaises(ValueError):
            metadata_pair(0, 0, bytes(7))

    def test_nonzero_high_byte_address(self):
        self.assertEqual(metadata_pair(2, 1, bytes(65546)), (65538, [0, 0]))


class AuditTests(unittest.TestCase):
    def image(self):
        def code(op):
            return dict(nodes=[[0, 0, op]], metadata_hex=bytes(8).hex(),
                        offset=10, nodes_offset=20, metadata_offset=24)
        return dict(source='fixture/object.o', sha256='synthetic', strings=[],
                    init=code(0x0c), functions=[dict(name='ordinary', index=0, code=code(0x0a))],
                    hidden=[dict(name='hidden', code=code(0x0b))])

    def test_all_segment_classes_are_counted(self):
        summary, examples = audit([self.image()], {o:dict(name=str(o)) for o in OPS})
        self.assertEqual(summary['totals']['instructions'], 3)
        self.assertEqual(summary['opcodes']['0x0a']['by_segment_kind'], {'functions':1})
        self.assertEqual(summary['opcodes']['0x0b']['by_segment_kind'], {'hidden':1})
        self.assertEqual(summary['opcodes']['0x0c']['by_segment_kind'], {'init':1})
        self.assertEqual(examples['0x0a']['member_node_offset'], 20)
        self.assertEqual(examples['0x0a']['member_metadata_offset'], 24)

    def test_bad_metadata_is_reported(self):
        image = self.image()
        image['functions'][0]['code']['metadata_hex'] = ''
        summary, _ = audit([image], {o:dict(name=str(o)) for o in OPS})
        self.assertTrue(any(e['kind']=='metadata' for e in summary['errors']))

    def test_case_target_uses_advanced_pc(self):
        image = self.image()
        code = image['functions'][0]['code']
        code['nodes'].append([0, 0, 0x49])
        code['metadata_hex'] = struct.pack('<ii', 0, 77).hex()
        summary, examples = audit([image], {o:dict(name=str(o)) for o in OPS})
        self.assertEqual(examples['0x0a']['taken_branch_target'], 1)
        self.assertFalse(any(e['kind']=='branch-target' and e['function']=='ordinary'
                             for e in summary['errors']))

    def test_invalid_branch_is_reported(self):
        image = self.image()
        image['init']['nodes'] = [[5, 0, 0x0c]]
        summary, _ = audit([image], {o:dict(name=str(o)) for o in OPS})
        self.assertTrue(any(e['kind']=='branch-target' for e in summary['errors']))

    def test_null_string_index_is_rejected(self):
        image = self.image()
        image['strings'] = [None]
        summary, _ = audit([image], {o:dict(name=str(o)) for o in OPS})
        self.assertTrue(any(e['kind']=='string-index' for e in summary['errors']))

    def test_loop_slot_outside_locals_is_rejected(self):
        image = self.image()
        image['functions'][0].update(local_count=1, max_args=0)
        image['functions'][0]['code']['nodes'] = [[-2, 0, 0x23]]
        summary, _ = audit([image], {o:dict(name=str(o)) for o in OPS})
        self.assertTrue(any(e['kind']=='loop-slot' for e in summary['errors']))

    def test_generation_is_deterministic(self):
        actions = {o:dict(name=str(o)) for o in OPS}
        self.assertEqual(audit([self.image()], actions), audit([self.image()], actions))


if __name__ == '__main__':
    unittest.main()
