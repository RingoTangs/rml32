"""Synthetic integrity and namespace tests for the static corpus audit."""
import copy
import gzip
import importlib.util
from pathlib import Path
import unittest

spec = importlib.util.spec_from_file_location('audit', Path(__file__).with_name('analyze_corpus.py'))
audit = importlib.util.module_from_spec(spec)
spec.loader.exec_module(audit)


def program(nodes=None):
    return dict(source='server/x.pak/start.o',sha256='a'*64,mode='global',function_base=572,
        efuns_count=559,max_vars=2,max_functions=574,variables=[],dlts=[],functions=[],hidden=[],
        init=dict(offset=4,nodes_offset=8,nodes=nodes or []))


class CorpusTests(unittest.TestCase):
    def test_signed_flags_and_separate_namespaces(self):
        s,e,o,g=audit.audit([program([[1,0,0x35],[2,0,0x36],[572,-125,7],[559,0,6]])])
        self.assertEqual(s['histograms']['variable_bounds'],{'0x35:in_bounds':1,'0x36:out_of_bounds':1})
        self.assertEqual(o[2]['argc'],3)
        self.assertEqual(o[2]['global_function_slot'],572)
        self.assertEqual(o[2]['member_node_offset'],16)
        self.assertEqual(s['histograms']['efun_boundary'],{'between_efuns_and_function_base':1})

    def test_non_global_call_not_retained(self):
        s,e,o,g=audit.audit([program([[572,3,7],[573,-61,7]])])
        self.assertEqual(len(o),1)
        self.assertEqual(o[0]['other_flag_bits'],64)
        self.assertEqual(s['histograms']['all_opcode_counts']['0x07'],2)

    def test_negative_index_and_call_outlier_preserved(self):
        s,e,o,g=audit.audit([program([[-1,0,0x35],[559,-128,7]])])
        self.assertEqual(o[0]['variable_bounds'],'out_of_bounds')
        self.assertEqual(o[1]['function_interval'],'outside_function_interval')

    def test_ambiguous_binding_never_guessed(self):
        p=program([[0,0,0x35]])
        other=copy.deepcopy(p)
        other['source']='server/y.pak/start.o'
        s,e,o,g=audit.audit([p,other])
        self.assertEqual(s['ambiguous_prefixes'],['server'])
        self.assertTrue(all(x['variable_bounds']=='unbound' for x in o))

    def test_malformed_data_rejected(self):
        for node in ([0,0], [32768,0,1],[0,128,1],[0,0,86],[True,0,1]):
            with self.subTest(node=node), self.assertRaises(ValueError):
                audit.audit([program([node])])
        p=program();p['init']['offset']=9
        with self.assertRaises(ValueError):audit.audit([p])
        p=program();p['init']['nodes']=None
        with self.assertRaises(ValueError):audit.audit([p])

    def test_duplicates_and_invalid_inventory(self):
        p=program()
        with self.assertRaises(ValueError):audit.audit([p,p])
        p['max_vars']=-1
        with self.assertRaises(ValueError):audit.audit([p])

    def test_deterministic_artifacts(self):
        data=audit.audit([program([[0,0,0x35]])])
        first=audit.artifacts(*data)
        self.assertEqual(first,audit.artifacts(*data))
        self.assertEqual(first['corpus-occurrences.json.gz'][4:8],b'\0'*4)
        self.assertIn(b'0x35',gzip.decompress(first['corpus-occurrences.json.gz']))


if __name__=='__main__':unittest.main()
