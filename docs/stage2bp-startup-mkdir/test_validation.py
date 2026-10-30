import copy,json,unittest
import verify
from model import cases,regular,Rejected
import focused

class Validation(unittest.TestCase):
    def raw(self):return verify.load(verify.HERE/'service-observations.json.gz')['debug']['log-dir']['aaa']
    def test_01_real_service_and_legacy_results(self):
        obs=verify.load(verify.HERE/'service-observations.json.gz')
        for profile,groups in obs.items():
            for mode,rows in groups.items():
                for name,raw in rows.items():verify.validate_service(profile,name,mode,raw)
    def test_02_rejects_wrong_fence_and_false_completion(self):
        for key,value in [('stop_pc',29),('executed_instructions',29),('initialization_complete',True)]:
            r=json.loads(self.raw());r[key]=value
            with self.assertRaises(AssertionError):verify.validate_service('debug','aaa','log-dir',json.dumps(r))
    def test_03_rejects_false_mkdir_result_and_extra_calls(self):
        for change in (lambda r:r['host_events'][1]['call'].update(result=0),lambda r:r['host_events'].append(r['host_events'][1])):
            r=json.loads(self.raw());change(r)
            with self.assertRaises(AssertionError):verify.validate_service('debug','aaa','log-dir',json.dumps(r))
    def test_04_rejects_missing_directory_and_wrong_next_blocker(self):
        for change in (lambda r:r['filesystem']['entries'].pop(),lambda r:r['next_dependency'].update(pc=36),lambda r:r.update(sha256='0'*64)):
            r=json.loads(self.raw());change(r)
            with self.assertRaises(AssertionError):verify.validate_service('debug','aaa','log-dir',json.dumps(r))
    def test_05_independent_paths_and_matrix_inventory(self):
        ts=list(cases());self.assertEqual(len(ts),324);self.assertEqual(len({t['id']for t in ts}),324)
        self.assertEqual(regular(b'//aaa///log/\0other'),b'/aaa/log')
        self.assertEqual(regular(b'/aaa/\xff/'),b'/aaa/\xff')
        for p in [b'/a/../b',b'/v/x',b'x']:
            with self.assertRaises(Rejected):regular(p)
    def test_06_rejects_changed_focused_state(self):
        ts=list(cases())
        rows=[json.loads(l)for l in verify.gzip.decompress((verify.HERE/'focused-observations.jsonl.gz').read_bytes()).splitlines()]
        focused.compare(ts,rows);bad=copy.deepcopy(rows);bad[0]['steps'][0]['report']['filesystem']['entries'].pop()
        with self.assertRaises(AssertionError):focused.compare(ts,bad)
    def test_07_rejects_failed_or_missing_validation_commands(self):
        logs=[dict(cwd=cwd,argv=argv,returncode=0,stdout='',stderr='')for cwd,argv in verify.COMMANDS]
        with self.assertRaisesRegex(AssertionError,'command inventory'):verify.validate_commands(logs[:-1])
        logs[0]['returncode']=1
        with self.assertRaisesRegex(AssertionError,'failed command'):verify.validate_commands(logs)
    def test_08_native_evidence_is_tied_to_preserved_binary(self):
        self.assertEqual(verify.native()['records'],8)
        evidence=verify.load(verify.HERE/'native-evidence.json.gz')
        self.assertEqual(len({r['subject']['digest']['sha256']for r in evidence['records']}),1)
        self.assertFalse(evidence['captures'])
if __name__=='__main__':unittest.main()
