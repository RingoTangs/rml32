import copy
import unittest

from analyze_corpus import artifacts, audit, check_baseline, registry


def program(nodes, *, init=None, mode='object', source='gs/test.o'):
    code = dict(offset=8, nodes_offset=12, nodes=nodes)
    return dict(source=source, sha256='0' * 64, mode=mode, efuns_count=559,
                functions=[dict(name='test', index=4, code=code)], hidden=[],
                init=dict(offset=0, nodes_offset=4, nodes=init or []))


SLOTS = registry([dict(index=str(i), name=f'efun{i}') for i in range(559)])


class CorpusTests(unittest.TestCase):
    def test_zero_multiple_and_signed_arguments_are_not_masked(self):
        summary, _, rows, _ = audit([program([[0, 0, 6], [558, 127, 6],
                                            [1, -128, 6], [1, 64, 6], [1, -1, 6]])], SLOTS)
        self.assertEqual([r['par8_signed'] for r in rows], [0, 127, -128, 64, -1])
        self.assertEqual([r['par8_unsigned'] for r in rows], [0, 127, 128, 64, 255])
        self.assertEqual(summary['histograms']['argc_status'], {'negative': 2, 'nonnegative': 3})

    def test_namespace_and_registry_failures_are_separate(self):
        slots = copy.deepcopy(SLOTS)
        del slots[42]
        _, _, rows, _ = audit([program([[-1, 0, 6], [559, 0, 6], [42, 0, 6], [558, 0, 6]])], slots)
        self.assertEqual([r['slot_status'] for r in rows],
                         ['negative_slot', 'outside_efun_namespace', 'unknown_registry_slot', 'registered'])

    def test_init_hidden_global_and_member_offsets(self):
        p = program([[2, 0, 1], [3, 2, 6]], init=[[1, 0, 6]], mode='global')
        p['hidden'] = [dict(name='hidden', index=-1,
                            code=dict(offset=20, nodes_offset=24, nodes=[[4, 1, 6]]))]
        summary, examples, rows, _ = audit([p], SLOTS)
        self.assertEqual(summary['totals'], dict(programs=1, ordinary_functions=1,
                                                hidden_functions=1, instructions=4))
        self.assertEqual([r['member_node_offset'] for r in rows], [4, 16, 24])
        self.assertEqual([r['segment_kind'] for r in rows], ['init', 'functions', 'hidden'])
        self.assertEqual(len(examples), 3)
        self.assertEqual(rows[1]['function_index'], 4)

    def test_registry_requires_exact_abi_and_unique_slots(self):
        rows = [dict(index=str(i), name=str(i)) for i in range(559)]
        for invalid in (rows[:-1], rows + [rows[0]], rows + [dict(index='559', name='extra')]):
            with self.assertRaises(ValueError):
                registry(invalid)

    def test_malformed_nodes_and_offsets_are_rejected(self):
        for nodes in ([[0, 128, 6]], [[32768, 0, 6]], [[0, 0]], [[False, 0, 6]], [[0, 0, 86]]):
            with self.subTest(nodes=nodes), self.assertRaises(ValueError):
                audit([program(nodes)], SLOTS)
        p = program([])
        p['init']['nodes_offset'] = -1
        with self.assertRaises(ValueError):
            audit([p], SLOTS)

    def test_duplicate_source_and_wrong_profile_are_rejected(self):
        p = program([])
        with self.assertRaises(ValueError):
            audit([p, p], SLOTS)
        p['efuns_count'] = 389
        with self.assertRaises(ValueError):
            audit([p], SLOTS)

    def test_baseline_checks_both_total_and_per_efun_usage(self):
        summary, _, rows, _ = audit([program([[0, 0, 6]])], SLOTS)
        baseline = dict(totals=dict(programs=1, functions=1, hidden_functions=0, instructions=1),
                        opcodes={'0x6': 1})
        check_baseline(summary, rows, baseline, [dict(name='efun0', static_call_sites='1')])
        full_usage = dict(name='efun0', static_call_sites='1', programs='1',
                          aaa='0', ccs='0', csa='0', dba='0', gs='1')
        check_baseline(summary, rows, baseline, [full_usage])
        for change in (dict(programs='2'), dict(gs='0')):
            with self.assertRaises(ValueError):
                check_baseline(summary, rows, baseline, [full_usage | change])
        with self.assertRaises(ValueError):
            check_baseline(summary, rows, baseline, [full_usage, full_usage])
        for usage in ([], [dict(name='efun0', static_call_sites='2')]):
            with self.assertRaises(ValueError):
                check_baseline(summary, rows, baseline, usage)
        baseline['opcodes']['0x6'] = 2
        with self.assertRaises(ValueError):
            check_baseline(summary, rows, baseline, [])

    def test_artifacts_are_deterministic_and_preserve_all_occurrences(self):
        import gzip
        import json
        result = audit([program([[1, 2, 6], [1, 2, 6]])], SLOTS)
        first = artifacts(*result)
        self.assertEqual(first, artifacts(*result))
        decoded = [json.loads(line) for line in gzip.decompress(first['corpus-occurrences.jsonl.gz']).splitlines()]
        self.assertEqual(decoded, result[2])
        self.assertEqual(len(decoded), 2)


if __name__ == '__main__':
    unittest.main()
