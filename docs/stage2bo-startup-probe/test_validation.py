import copy
import gzip
import json
from pathlib import Path
import unittest
import verify

class Validation(unittest.TestCase):
    def report(self):
        # Actual command observations are bound to the receipt, not reconstructed
        # from the implementation or left in an environment-specific cache.
        return verify.load(verify.HERE/'probe-observations.json.gz')['debug']['aaa']
    def test_01_real_probe_report(self):verify.validate_probe('aaa',self.report())
    def test_02_rejects_execution_past_fence(self):
        for field,value in [('stop_pc',26),('executed_instructions',26),('initialization_complete',True)]:
            r=self.report();r[field]=value
            with self.assertRaises(AssertionError):verify.validate_probe('aaa',r)
    def test_03_rejects_extra_host_call(self):
        r=self.report();r['host_calls'].append(dict(slot=336,name='mkdir'))
        with self.assertRaises(AssertionError):verify.validate_probe('aaa',r)
    def test_04_rejects_wrong_array_initialization(self):
        r=self.report();r['globals'][2]['elements'][0]={'type':'int','value':0}
        with self.assertRaises(AssertionError):verify.validate_probe('aaa',r)
    def test_05_rejects_unauthenticated_or_wrong_service_report(self):
        r=self.report();r['sha256']='0'*64
        with self.assertRaises(AssertionError):verify.validate_probe('aaa',r)
        with self.assertRaises(AssertionError):verify.validate_probe('gs',self.report())
    def test_06_native_records_bind_the_preserved_elf(self):
        b=verify.load(verify.HERE/'native-evidence.json.gz');verify.validate_native(b)
        bad=copy.deepcopy(b);bad['records'][0]['subject']['digest']['sha256']='0'*64
        with self.assertRaises(AssertionError):verify.validate_native(bad)
    def test_07_rejects_failed_or_missing_commands(self):
        logs=[dict(cwd=cwd,argv=argv,returncode=0,stdout='',stderr='') for cwd,argv in verify.COMMANDS]
        with self.assertRaisesRegex(AssertionError,'command inventory'):verify.validate_commands(logs[:-1])
        logs[0]['returncode']=1
        with self.assertRaisesRegex(AssertionError,'failed command'):verify.validate_commands(logs)
    def test_08_rejects_resource_and_dependency_changes(self):
        for mutate in (lambda r:r['configuration']['values'].update(max_items=1),
                       lambda r:r['next_dependency'].update(pc=25),lambda r:r['globals'].pop()):
            r=self.report();mutate(r)
            with self.assertRaises(AssertionError):verify.validate_probe('aaa',r)

if __name__=='__main__':unittest.main()
