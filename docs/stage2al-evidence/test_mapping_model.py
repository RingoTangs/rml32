import gzip
import json
import unittest

from capture_native import HERE
from mapping_model import Cursor, Mapping, hash_key


class DeductionTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        bundle = json.loads(gzip.decompress((HERE / 'native.json.gz').read_bytes()))
        cls.table = bytes.fromhex(bundle['hash_table']['bytes_hex'])
        cls.cases = json.loads((HERE / 'expected-cases.json').read_text())['cases']

    def make(self, keys=(), hint=0):
        mapping = Mapping(self.table, hint)
        for key in keys:
            mapping.insert(key, key)
        return mapping

    def remaining(self, cursor):
        output = None; rows = []
        for _ in range(50):
            valid, output = cursor.advance(output)
            if not valid:
                return rows
            rows.append(list(output))
        self.fail('unexpected nontermination')

    def test_frozen_hash_vectors_and_capacity_thresholds(self):
        for case in self.cases:
            if case['category'] == 'hash':
                setup = case['setup']; expected = case['expected']
                key = setup['key_int'] if 'key_int' in setup else bytes.fromhex(setup['key_hex'])
                h = hash_key(key, self.table)
                with self.subTest(case=case['id']):
                    self.assertEqual((h, h & 7), (expected['hash'], expected['bucket']))
            if case['category'] == 'allocate':
                m = self.make(hint=case['setup']['hint'])
                self.assertEqual((len(m.buckets), m.unfilled),
                                 (case['expected']['capacity'], case['expected']['unfilled']))

    def test_head_insertion_overwrite_and_full_byte_equality(self):
        m = self.make([0, 1, 2]); m.insert(1, 99)
        self.assertEqual(self.remaining(Cursor(m)), [[2, 2, 0], [1, 99, 1], [0, 0, 2]])
        self.assertEqual(m.count, 3)
        m = self.make([b'a', b'a\0', b'a\0x', b'a\0y'])
        self.assertEqual(m.count, 4)
        self.assertEqual(m.order(), [b'a\0y', b'a\0x', b'a\0', b'a'])

    def test_growth_is_empty_bucket_budget_not_item_count(self):
        m = self.make(range(16))
        self.assertEqual((len(m.buckets), m.count, m.unfilled), (8, 16, 5))
        m = self.make([0, 128, 256, 384, 640, 16, 32, 48, 64, 80])
        self.assertEqual((len(m.buckets), m.unfilled), (16, 5))
        self.assertEqual(m.order(), [256, 0, 16, 32, 48, 64, 80, 128, 384, 640])

    def test_delete_current_or_next_repositions_scan(self):
        for key in (2, 1):
            m = self.make([0, 1, 2]); cursor = Cursor(m)
            self.assertEqual(cursor.advance(None)[1], [2, 2, 0])
            m.delete(key)
            self.assertEqual(self.remaining(cursor), [[0, 0, 1]])
        m = self.make([0]); m.delete(0); m.delete(0)
        self.assertEqual((m.count, m.unfilled), (0, 6))

    def test_insert_head_can_repeat_old_output(self):
        m = self.make([0, 1, 2]); cursor = Cursor(m); cursor.advance(None)
        m.insert(3, 3)
        self.assertEqual(self.remaining(cursor), [[2, 2, 1], [1, 1, 2], [0, 0, 3]])

    def test_earlier_bucket_insertion_invisible_later_visible(self):
        m = self.make([16]); cursor = Cursor(m); cursor.advance(None); m.insert(0, 0)
        self.assertEqual(self.remaining(cursor), [])
        m = self.make([0]); cursor = Cursor(m); cursor.advance(None); m.insert(32, 32)
        self.assertEqual(self.remaining(cursor), [[32, 32, 1]])

    def test_growth_after_exhaustion_can_resume(self):
        m = self.make([0]); cursor = Cursor(m)
        cursor.advance(None)
        self.assertEqual(cursor.advance(9), (False, 9))
        self.assertEqual((cursor.bucket, cursor.ordinal), (8, 2))
        for key in (128, 384, 16, 32, 48, 64, 80):
            m.insert(key, key)
        # The failed scan left subindex=1; newly reachable bucket8 skips its head.
        self.assertEqual(cursor.advance(None), (True, [384, 384, 2]))

    def test_binding_array_identity_rechecked_each_time(self):
        cursor = Cursor(self.make([0, 1, 2]))
        a = [9, 9, 9]; valid, output = cursor.advance(a)
        self.assertTrue(valid); self.assertIs(output, a)
        old = list(a); b = [8, 8, 8]
        _, output = cursor.advance(b)
        self.assertIs(output, b); self.assertEqual(a, old)
        self.assertEqual(b, [1, 1, 1])
        short = [7, 7]; _, output = cursor.advance(short)
        self.assertIsNot(output, short); self.assertEqual(short, [7, 7])

    def test_empty_binding_unchanged_and_ordinal_wrap(self):
        cursor = Cursor(self.make()); binding = [1, 2, 3]
        self.assertEqual(cursor.advance(binding), (False, binding))
        self.assertEqual(cursor.advance(binding), (False, binding))
        self.assertEqual(cursor.ordinal, 2)
        cursor = Cursor(self.make([0, 1])); cursor.ordinal = 2147483647
        self.assertEqual(self.remaining(cursor), [[1, 1, 2147483647], [0, 0, -2147483648]])
        cursor = Cursor(self.make([0])); cursor.subindex = 2**31
        self.assertEqual(cursor.advance(None), (True, [0, 0, 0]))

    def test_owner_rebinding_and_out_of_profile_keys(self):
        m = self.make([0]); cursor = Cursor(m); m = self.make([16])
        self.assertEqual(self.remaining(cursor), [[0, 0, 0]])
        for key in (True, 'a', 2**31, -2**31-1, 1.0):
            with self.assertRaises(ValueError):
                hash_key(key, self.table)


if __name__ == '__main__':
    unittest.main()
