"""Focused small fixtures for evidence checks."""
import copy
import struct
import unittest
from unittest.mock import patch
import verify_evidence as v


class EvidenceTests(unittest.TestCase):
    def fixture(self):
        return {'evidence_id':'ev_x','provider':{'id':'ghidra','version':'12.1.4'},
         'subject':{'digest':{'sha256':v.reuse.TARGET}},'raw_result':{'complete':True},
         'analysis_profile':{'digest':v.PROFILE},'operation':'analyze_function',
         'normalized_result':{'procedure':{'name':'tiny','address':'0x10',
          'body':{'available':True,'contains_entry':True,'ranges':[{'start':'0x10','end':'0x11'}],
                  'total_bytes':2}},'assembly':['0x10: RET']}}

    def run_fresh(self,r):
        with patch.dict(v.INVENTORY,{'tiny':'0x10'},clear=True): v.validate_fresh([r])

    def test_valid_tiny(self): self.run_fresh(self.fixture())

    def test_fresh_failures(self):
        for key in ['raw','profile','provider','target','address','range','total','instruction','inventory']:
            with self.subTest(key=key):
                r=self.fixture(); n=r['normalized_result']; p=n['procedure']
                if key=='raw':r['raw_result']=None
                if key=='profile':r['analysis_profile']['digest']='bad'
                if key=='provider':r['provider']['version']='bad'
                if key=='target':r['subject']['digest']['sha256']='bad'
                if key=='address':p['address']='0x12'
                if key=='range':p['body']['ranges'][0]['start']='0x12'
                if key=='total':p['body']['total_bytes']=3
                if key=='instruction':n['assembly']=['0x20: RET']
                if key=='inventory':p['name']='other'
                with self.assertRaises(ValueError):self.run_fresh(r)

    def test_selected_site_missing(self):
        with patch.dict(v.SITES,{'tiny':['0x10: NOP']},clear=True):
            with self.assertRaisesRegex(ValueError,'selected'):self.run_fresh(self.fixture())

    def test_noncontiguous_body(self):
        r=self.fixture();p=r['normalized_result']['procedure']
        p['body']['ranges']=[{'start':'0x10','end':'0x10'},{'start':'0x20','end':'0x20'}]
        r['normalized_result']['assembly'].append('0x20: RET');self.run_fresh(r)
        r['normalized_result']['assembly'].append('0x15: NOP')
        with self.assertRaises(ValueError):self.run_fresh(r)

    def refs(self):
        return [{'id':'finding','classification':'observation','status':'answered','statement':'static','evidence_ids':['ev_x']}], [{'id':'case','opcode':'01','left':{'type':'Bytes','hex':'61'},'right':{'type':'Int'},'classification':'static-deduction','expected':{'type':'Bytes','hex':'6131'},'evidence_ids':['ev_x']}]

    def test_valid_references(self):v.validate_references(*self.refs(),{'ev_x'})

    def test_reference_failures(self):
        for key in ['ref','status','classification','opcode','type','caseclass','hex','duplicate','expected']:
            with self.subTest(key=key):
                f,c=self.refs()
                if key=='ref':f[0]['evidence_ids']=['bad']
                if key=='status':f[0]['status']='done'
                if key=='classification':f[0]['classification']='runtime-observed'
                if key=='opcode':c[0]['opcode']='ff'
                if key=='type':c[0]['left']['type']='bad'
                if key=='caseclass':c[0]['classification']='executed'
                if key=='hex':c[0]['left']['hex']='zz'
                if key=='duplicate':c.append(copy.deepcopy(c[0]))
                if key=='expected':c[0]['expected']={}
                with self.assertRaises(ValueError):v.validate_references(f,c,{'ev_x'})

    def format_fixture(self):
        data=bytearray(160);data[:6]=b'\x7fELF\x01\x01'
        struct.pack_into('<I',data,28,52);struct.pack_into('<HH',data,42,32,1)
        struct.pack_into('<IIIIIIII',data,52,1,100,0x1000,0,60,60,4,1);data[100:103]=b'%g\0'
        o={'source':'magic_Linux32','segment_index':0,'segment_file_offset':100,'segment_virtual_address':0x1000,'segment_file_size':60,'virtual_address':'0x1000','bytes_hex':'256700','file_offset':100,'evidence_id':'ev_x','native_use_site':'0x20'}
        r=[{'evidence_id':'ev_x','normalized_result':{'assembly':['0x20: PUSH 0x1000']}}]
        return data,o,r

    def test_valid_format(self):
        data,o,r=self.format_fixture();v.validate_format(data,[o],r)

    def test_format_failures(self):
        for key in ['bytes','offset','segment','site','header']:
            with self.subTest(key=key):
                data,o,r=self.format_fixture()
                if key=='bytes':data[100]=0
                if key=='offset':o['file_offset']=101
                if key=='segment':o['segment_file_size']=59
                if key=='site':o['native_use_site']='0x21'
                if key=='header':data[4]=2
                with self.assertRaises(ValueError):v.validate_format(data,[o],r)


if __name__=='__main__':unittest.main()
