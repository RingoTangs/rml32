"""Synthetic metadata tests; no reference bytecode is executed."""
import copy
import importlib.util
from pathlib import Path
import struct
import sys
import unittest
sys.dont_write_bytecode = True
import verify_lifecycle as v


def fixture(name=b'create', nodes=None, mode='object'):
    function = dict(offset=10, name='create', index=2, attrib=0, return_type=0,
                    min_args=0, max_args=0, arguments=[], locals=[], code=dict(nodes=nodes or []))
    program = dict(source='synthetic.o', mode=mode, constitution_count=1, functions=[function], hidden=[],
                   init=dict(nodes=[[0, 0, 0x2e], [0, 0, 0x48], [0, 0, 0x1d]],
                             nodes_offset=100, metadata_hex=''))
    raw = dict(name_hex=name.hex(), c_name_hex=name.split(b'\0', 1)[0].hex(),
               header_hex='00' * 64, return_attrib=0, arguments=[], locals=[])
    return [program], dict(init_orders={'synthetic.o': [0]} if mode == 'object' else {}), {('synthetic.o', 10): raw}


class InventoryTests(unittest.TestCase):
    def analyze(self, *args):
        return v.inventory(*args)

    def test_empty_corpus(self):
        _, c = self.analyze([], dict(init_orders={}), {})
        self.assertEqual(c['programs'], 0)
        self.assertEqual(c['init_nodes'], 0)

    def test_empty_code_is_declaration(self):
        _, c = self.analyze(*fixture())
        self.assertEqual(c['lifecycle']['create']['declarations'], 1)
        self.assertEqual(c['lifecycle']['create']['bodies'], 0)

    def test_nonempty_code_is_body(self):
        _, c = self.analyze(*fixture(nodes=[[0, 0, 0x49]]))
        self.assertEqual(c['lifecycle']['create']['bodies'], 1)

    def test_exact_c_name_before_nul(self):
        _, c = self.analyze(*fixture(name=b'create\0suffix'))
        self.assertEqual(c['lifecycle']['create']['records'], 1)

    def test_prefix_does_not_match(self):
        _, c = self.analyze(*fixture(name=b'create_more'))
        self.assertEqual(c['lifecycle']['create']['records'], 0)

    def test_case_sensitive(self):
        _, c = self.analyze(*fixture(name=b'Create'))
        self.assertEqual(c['lifecycle']['create']['records'], 0)

    def test_display_does_not_drive_matching(self):
        _, c = self.analyze(*fixture(name=b'\xffcreate'))
        self.assertEqual(c['lifecycle']['create']['records'], 0)

    def test_destruct_distinct(self):
        _, c = self.analyze(*fixture(name=b'destruct'))
        self.assertEqual(c['lifecycle']['destruct']['records'], 1)
        self.assertEqual(c['lifecycle']['create']['records'], 0)

    def test_minimal_init_classification(self):
        _, c = self.analyze(*fixture())
        self.assertEqual(c['init_minimal_2e_48_1d'], 1)
        self.assertEqual(c['init_opcode_counts'], {'1d': 1, '2e': 1, '48': 1})

    def test_init_operand_difference_is_still_opcode_minimal(self):
        p, g, r = fixture()
        p[0]['init']['nodes'][0][0] = 12
        _, c = self.analyze(p, g, r)
        self.assertEqual(c['init_minimal_2e_48_1d'], 1)

    def test_nonminimal_init_metadata(self):
        p, g, r = fixture()
        p[0]['init']['nodes'].insert(0, [0, 0, 0x3b])
        p[0]['init']['metadata_hex'] = '01020304'
        _, c = self.analyze(p, g, r)
        self.assertEqual(c['init_other'], 1)
        self.assertEqual(c['init_metadata_nonempty'], 1)

    def test_parent_first_permutation_accepts_self_last(self):
        p, g, r = fixture()
        p[0]['constitution_count'] = 4
        g['init_orders']['synthetic.o'] = [2, 3, 1, 0]
        _, c = self.analyze(p, g, r)
        self.assertEqual(c['init_order_issues'], [])

    def test_bad_order_rejected_in_summary(self):
        for order in (None, [0, 1], [1, 1, 0], [0, 2, 1]):
            p, g, r = fixture()
            p[0]['constitution_count'] = 3
            g['init_orders']['synthetic.o'] = order
            _, c = self.analyze(p, g, r)
            self.assertEqual(c['init_order_issues'], ['synthetic.o'])

    def test_nonpositive_constitution_count_is_issue(self):
        for count in (0, -1):
            p, g, r = fixture()
            p[0]['constitution_count'] = count
            g['init_orders']['synthetic.o'] = []
            _, c = self.analyze(p, g, r)
            self.assertEqual(c['init_order_issues'], ['synthetic.o'])

    def test_global_has_no_object_order(self):
        _, c = self.analyze(*fixture(mode='global'))
        self.assertEqual(c['object_init_orders_verified'], 0)
        self.assertEqual(c['init_order_issues'], [])

    def test_global_order_is_issue(self):
        p, g, r = fixture(mode='global')
        g['init_orders']['synthetic.o'] = [0]
        _, c = self.analyze(p, g, r)
        self.assertEqual(c['init_order_issues'], ['synthetic.o'])

    def test_hidden_and_duplicate_records_do_not_inflate_program_count(self):
        p, g, r = fixture()
        p[0]['hidden'] = [copy.deepcopy(p[0]['functions'][0])]
        _, c = self.analyze(p, g, r)
        self.assertEqual(c['lifecycle']['create']['records'], 2)
        self.assertEqual(c['lifecycle']['create']['programs'], 1)
        self.assertEqual(c['lifecycle']['create']['group'], {'functions': 1, 'hidden': 1})

    def test_parameter_attributes_and_signed_default_retained(self):
        p, g, r = fixture()
        raw = r['synthetic.o', 10]
        raw['arguments'] = [dict(flag=1, type=4, attrib=8, default_action_count=-1)]
        p[0]['functions'][0].update(min_args=0, max_args=1, attrib=4)
        _, c = self.analyze(p, g, r)
        stats = c['lifecycle']['create']
        self.assertEqual(stats['argument_flags'], {'1': 1})
        self.assertEqual(stats['argument_attribs'], {'8': 1})
        self.assertEqual(stats['min_max'], {'0/1': 1})
        self.assertEqual(stats['nonzero_default_descriptors'], 1)

    def test_raw_function_recovers_bytes_and_default(self):
        spec = importlib.util.spec_from_file_location('decoder', v.ROOT / v.INPUTS[3])
        decoder = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(decoder)
        header = bytearray(64)
        header[12] = 8
        descriptor = bytearray(20)
        descriptor[4:7] = bytes([1, 4, 8])
        struct.pack_into('<hh', descriptor, 8, 0, -3)
        raw = bytes(header) + struct.pack('<i', 7) + b'create\0' + struct.pack('<h', 1) + descriptor + struct.pack('<i', 2) + b'x\0' + struct.pack('<h', 0)
        fun = dict(offset=0, name='create', arguments=[{}], locals=[], code=dict(offset=len(raw)))
        result = v.raw_function(raw, fun, decoder)
        self.assertEqual(result['c_name_hex'], b'create'.hex())
        self.assertEqual(result['return_attrib'], 8)
        self.assertEqual(result['arguments'][0]['default_action_count'], -3)
        fun['code']['offset'] -= 1
        with self.assertRaisesRegex(ValueError, 'code offset'):
            v.raw_function(raw, fun, decoder)


if __name__ == '__main__':
    unittest.main()
