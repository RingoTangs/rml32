import gzip
import json
import unittest

from analyze_corpus import artifacts, audit, check_baseline, registry

EFUNS = registry([dict(index=str(i), name=f'efun{i}') for i in range(559)])


def program(nodes, *, source='gs/test.o', mode='object', init=None):
    return dict(source=source, sha256='0'*64, mode=mode, efuns_count=559,
                function_base=572 if mode=='global' else 0, max_functions=600,
                functions=[dict(name='f',index=7,code=dict(offset=8,nodes_offset=12,nodes=nodes))],
                hidden=[], init=dict(offset=0,nodes_offset=4,nodes=init or []))


class CorpusTests(unittest.TestCase):
    def test_create_flags_use_full_unsigned_byte_and_low_six_bound_count(self):
        _,_,rows,_=audit([program([[7,0,23],[7,63,23],[3,64,23],[572,-128,23],[356,-1,23]])],EFUNS)
        self.assertEqual([(r['global_bit'],r['selector_bit6'],r['bound_count']) for r in rows],
                         [(False,False,0),(False,False,63),(False,True,0),(True,False,0),(True,True,63)])
        self.assertEqual(rows[-1]['par8_unsigned'],255)

    def test_call_argc_includes_target_and_par16_is_not_a_target_slot(self):
        summary,_,rows,_=audit([program([[0,1,5],[4,127,5],[-1,-128,5]])],EFUNS)
        self.assertEqual(summary['histograms']['call_par16'],{'-1':1,'0':1,'4':1})
        self.assertTrue(all('numeric_namespace' not in r for r in rows))
        self.assertTrue(all(r['target_resolution']=='runtime_stack_value_not_statically_resolved' for r in rows))
        self.assertEqual(summary['histograms']['call_argc_status'],{'negative':1,'nonnegative':2})

    def test_global_numeric_boundaries_and_unresolved_service(self):
        p=program([[0,-128,23],[558,-128,23],[559,-128,23],[571,-128,23],[572,-128,23],[600,-128,23],[-1,-128,23]])
        g=program([],source='gs/start.o',mode='global')
        _,_,rows,_=audit([p,g],EFUNS)
        self.assertEqual([r['numeric_namespace'] for r in rows],
            ['efun_numeric_range','efun_numeric_range','pre_module_numeric_range',
             'pre_module_numeric_range','module_numeric_range','outside_global_capacity','negative'])
        self.assertEqual(rows[1]['efun_numeric_name'],'efun558')
        self.assertEqual(rows[4]['global_unit'],'gs/start.o')
        _,_,rows,_=audit([program([[572,-128,23]],source='aaa/test.o')],EFUNS)
        self.assertEqual(rows[0]['numeric_namespace'],'unresolved_global_unit')

    def test_ambiguous_service_does_not_select_a_global_unit(self):
        p=program([[572,-128,23]])
        gs=[program([],source=f'gs/start{i}.o',mode='global') for i in range(2)]
        summary,_,rows,_=audit([p]+gs,EFUNS)
        self.assertEqual(summary['ambiguous_global_services'],['gs'])
        self.assertIsNone(rows[0]['global_unit'])
        self.assertEqual(rows[0]['numeric_namespace'],'unresolved_global_unit')

    def test_object_slot_is_only_a_candidate_and_offsets_cover_all_segments(self):
        p=program([[0,0,1],[7,0,23]],init=[[0,1,5]])
        p['hidden']=[dict(name='hidden',index=-1,code=dict(offset=20,nodes_offset=24,nodes=[[4,1,5]]))]
        summary,_,rows,_=audit([p],EFUNS)
        self.assertEqual([r['member_node_offset'] for r in rows],[4,16,24])
        self.assertEqual([r['segment_kind'] for r in rows],['init','functions','hidden'])
        self.assertEqual(rows[1]['same_image_definition_candidate'],'f')
        self.assertEqual(rows[1]['numeric_namespace'],'object_relative')
        self.assertEqual(summary['totals']['instructions'],4)

    def test_malformed_nodes_profiles_offsets_and_duplicate_sources_fail(self):
        for nodes in ([[32768,0,23]],[[0,128,5]],[[0,0]],[[False,0,23]],[[0,0,86]]):
            with self.subTest(nodes=nodes),self.assertRaises(ValueError):audit([program(nodes)],EFUNS)
        p=program([])
        with self.assertRaises(ValueError):audit([p,p],EFUNS)
        p['efuns_count']=389
        with self.assertRaises(ValueError):audit([p],EFUNS)
        p=program([]);p['init']['nodes_offset']=-1
        with self.assertRaises(ValueError):audit([p],EFUNS)

    def test_registry_exact_abi_unique_slots_and_nonempty_names(self):
        rows=[dict(index=str(i),name=str(i)) for i in range(559)]
        for invalid in (rows[:-1],rows+[rows[0]],rows+[dict(index='559',name='other')]):
            with self.assertRaises(ValueError):registry(invalid)
        rows[0]['name']=''
        with self.assertRaises(ValueError):registry(rows)

    def test_baseline_checks_each_opcode_and_whole_corpus(self):
        summary,_,_,_=audit([program([[7,0,23],[0,1,5]])],EFUNS)
        baseline=dict(totals=dict(programs=1,functions=1,hidden_functions=0,instructions=2),opcodes={'0x5':1,'0x17':1})
        check_baseline(summary,baseline)
        for key in ('0x5','0x17'):
            baseline['opcodes'][key]=2
            with self.assertRaises(ValueError):check_baseline(summary,baseline)
            baseline['opcodes'][key]=1
        baseline['totals']['instructions']=3
        with self.assertRaises(ValueError):check_baseline(summary,baseline)

    def test_deterministic_artifacts_retain_duplicate_occurrences(self):
        results=audit([program([[7,0,23],[7,0,23],[4,1,5]])],EFUNS)
        a=artifacts(*results)
        self.assertEqual(a,artifacts(*results))
        rows=[json.loads(line) for line in gzip.decompress(a['corpus-occurrences.jsonl.gz']).splitlines()]
        self.assertEqual(rows,results[2]);self.assertEqual(len(rows),3)
        self.assertEqual(results[3][0]['occurrences'],2)


if __name__=='__main__':unittest.main()
