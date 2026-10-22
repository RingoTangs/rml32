import copy
import json
import unittest

from analyze_corpus import cross_check, integer, scan


def program(nodes, mode='object', kind='functions'):
    p = dict(source='aaa/test.o', mode=mode, efuns_count=559, sha256='synthetic',
             functions=[], hidden=[], init=dict(nodes_offset=32, nodes=[]))
    if kind == 'init':
        p['init']['nodes'] = nodes
    else:
        p[kind] = [dict(name='fixture', index=572, code=dict(nodes_offset=100, nodes=nodes))]
    return p


EFUNS = {i: 'efun_' + str(i) for i in range(559)}


class CorpusTests(unittest.TestCase):
    def test_signed_global_flags_and_absolute_slot(self):
        summary, rows, _, _ = scan([program([[572, -128, 23], [356, -1, 23]])], EFUNS)
        self.assertEqual([r['selector'] for r in rows], ['c', 'K'])
        self.assertEqual([r['bound_count'] for r in rows], [0, 63])
        self.assertEqual([r['slot'] for r in rows], [572, 356])
        self.assertEqual(summary['totals']['global_k'], 1)

    def test_local_creations_and_calls_are_counted_without_resolving_targets(self):
        summary, rows, _, _ = scan([program([[1, 0, 23], [0, 1, 5]])], EFUNS)
        self.assertEqual(rows, [])
        self.assertEqual(summary['totals']['creates_17'], 1)
        self.assertEqual(summary['totals']['calls_05'], 1)

    def test_member_location_and_context(self):
        _, rows, _, examples = scan([program([[0, 0, 55], [600, -127, 23]], 'global')], EFUNS)
        self.assertEqual(rows[0]['member_node_offset'], 104)
        self.assertEqual(rows[0]['mode'], 'global')
        self.assertIsNone(rows[0]['efun_numeric_name'])
        self.assertEqual(examples[0]['snippet'][0]['pc'], 0)

    def test_init_context_kept_separate(self):
        _, rows, _, _ = scan([program([[356, -64, 23]], kind='init')], EFUNS)
        self.assertEqual(rows[0]['segment_kind'], 'init')
        self.assertIsNone(rows[0]['function_index'])

    def test_shapes_distinguish_capture_count_and_keep_first_example(self):
        _, rows, inventory, examples = scan([program([[356, -64, 23], [356, -64, 23], [356, -63, 23]])], EFUNS)
        self.assertEqual(len(rows), 3)
        self.assertEqual([r['occurrences'] for r in inventory], [2, 1])
        self.assertEqual([r['pc'] for r in examples], [0, 2])

    def test_duplicate_sources_and_wrong_abi_rejected(self):
        p = program([])
        with self.assertRaises(ValueError):
            scan([p, p], EFUNS)
        p['efuns_count'] = 389
        with self.assertRaises(ValueError):
            scan([p], EFUNS)

    def test_invalid_nodes_and_boolean_operands_rejected(self):
        for node in [[0, 128, 23], [32768, 0, 23], [0, 0, 86], [True, 0, 23], [0, 23]]:
            with self.subTest(node=node), self.assertRaises(ValueError):
                scan([program([node])], EFUNS)

    def test_registry_must_cover_all_slots(self):
        registry = copy.copy(EFUNS)
        del registry[558]
        with self.assertRaises(ValueError):
            scan([], registry)

    def test_cross_check_rejects_location_change_even_with_same_totals(self):
        # Use actual independent input counts; only mutate one location in memory.
        from analyze_corpus import ROOT, INPUTS, artifacts
        import gzip
        data = artifacts()
        summary = json.loads(data['corpus-summary.json'])
        rows = [json.loads(line) for line in gzip.decompress(data['corpus-occurrences.jsonl.gz']).splitlines()]
        prior = [json.loads(line) for line in gzip.decompress((ROOT / INPUTS[3]).read_bytes()).splitlines()]
        rows[0]['member_node_offset'] += 4
        with self.assertRaisesRegex(ValueError, 'location/operand'):
            cross_check(summary, rows, json.loads((ROOT / INPUTS[2]).read_bytes()),
                        json.loads((ROOT / INPUTS[4]).read_bytes()), prior)


if __name__ == '__main__':
    unittest.main()
