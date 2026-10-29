import copy,gzip,json,unittest
from unittest.mock import patch
from common import HERE,load,digest,encoded
from matrix import HIGH,a,cp,seed
from pair_oracle import Reference
from pair_diff import compare,input_digest
from public_replay import cases,compare as public_compare
from verify_validation import COMMANDS,IGNORED,expected_tests,validate_rust,validate_commands,validate_record

def specimen():
    t=dict(id='sample',family='sample',pair='CC',limits=HIGH,setup=seed(),actions=[cp('A','s0','d0'),cp('B','s0','d0'),a('abort',tx='A'),a('commit',tx='B')])
    r=Reference(HIGH);rows=[]
    for phase in ('setup','actions'):
        for i,x in enumerate(t[phase]):
            before=r.projection();r.apply(x);rows.append(dict(id=t['id'],phase=phase,step=i,before=before,after=r.projection(),error=None,calls=0,progress=None))
    return t,rows
class ObservationTests(unittest.TestCase):
    def rejected(self,mutate):
        t,rows=specimen();mutate(rows)
        with self.assertRaises(AssertionError):compare([t],rows)
    def test_valid_double_preparation(self):
        t,rows=specimen();self.assertEqual(compare([t],rows)[0]['prepared'],2)
    def test_missing_row(self):self.rejected(lambda rs:rs.pop())
    def test_extra_row(self):self.rejected(lambda rs:rs.append(copy.deepcopy(rs[-1])))
    def test_wrong_order(self):self.rejected(lambda rs:rs.reverse())
    def test_transaction_identity_pollution(self):self.rejected(lambda rs:rs[-1]['before']['transactions'].update(A='head'))
    def test_false_expired_cell(self):self.rejected(lambda rs:rs[-1]['after']['cells'].update(s0=None))
    def test_duplicate_payload_owners(self):self.rejected(lambda rs:rs[-1]['after']['objects']['o0'].update(ordinary=99))
    def test_revision_pollution(self):self.rejected(lambda rs:rs[-1]['after']['cells']['d0'].update(revision=0))
    def test_input_digest_changes_with_transaction_identity(self):
        t,_=specimen();other=copy.deepcopy(t);other['actions'][0]['tx']='B';self.assertNotEqual(input_digest([t]),input_digest([other]))

def public_specimen():
    ts=[t for t in cases() if t['limits']==HIGH];rows=[]
    for t in ts:
        if t['kind']=='members':r=dict(value=8,reaped_before_alias_drop=int(not t['alias']),reaped_after_alias_drop=int(t['alias']),frames=0)
        elif t['kind']=='heads':r=dict(independent_heads=2,reclaimed=2,reaped=1,frames=0)
        else:r=dict(peak_frames=2,drained=1,reaped=1,frames=0)
        rows.append(dict(id=t['id'],result=r))
    return ts,rows
class PublicTests(unittest.TestCase):
    def test_all_ten_controls(self):
        ts,rs=public_specimen();self.assertEqual(public_compare(ts,rs)['accepted'],10)
    def test_missing_control(self):
        ts,rs=public_specimen()
        with self.assertRaises(AssertionError):public_compare(ts,rs[:-1])
    def test_wrong_nested_frame_result(self):
        ts,rs=public_specimen();rs[-1]['result']['peak_frames']=1
        with self.assertRaises(AssertionError):public_compare(ts,rs)
    def test_sufficient_quota_failure_is_not_success_coverage(self):
        ts,rs=public_specimen();rs[0]=dict(id=ts[0]['id'],phase='setup',error='Limit')
        with self.assertRaises(AssertionError):public_compare(ts,rs)

def cargo_log():
    lines=[f'test {n} ... ok' for n in sorted(expected_tests())]+[f'test {n} ... ignored, driver' for n in sorted(IGNORED)]
    lines += [f'test src/boundary.rs - boundary::HostValue (line {n}) - compile fail ... ok' for n in (531,534)]
    lines += ['test src/lifecycle.rs - lifecycle::LifecycleLimits (line 6) - compile fail ... ok']
    lines += [f'test src/lifecycle.rs - lifecycle::LifecycleBudget (line {n}) - compile fail ... ok' for n in (30,33)]
    return '\n'.join(lines+['test result: ok. 897 passed; 0 failed; 17 ignored;'])
class ReceiptTests(unittest.TestCase):
    def test_complete_inventory(self):self.assertEqual(len(validate_rust(cargo_log())),892)
    def test_missing_old_scenario(self):
        text=cargo_log();name=load(HERE.parent/'stage2bm-validation.json')['current_test_names'][0]
        with self.assertRaises(AssertionError):validate_rust(text.replace(f'test {name} ... ok',''))
    def test_missing_driver(self):
        text=cargo_log();name=sorted(IGNORED)[0]
        with self.assertRaises(AssertionError):validate_rust(text.replace(f'test {name} ... ignored, driver',''))
    def test_missing_doctest(self):
        with self.assertRaises(AssertionError):validate_rust('\n'.join(l for l in cargo_log().splitlines() if '(line 6)' not in l))
    def test_command_failure(self):
        cs=[dict(argv=v,returncode=1,stdout='',stderr='failed') for v in COMMANDS]
        with self.assertRaisesRegex(AssertionError,'failed command'):validate_commands(cs)
    def test_command_reordering(self):
        cs=[dict(argv=v,returncode=0,stdout='',stderr='') for v in reversed(COMMANDS)]
        with self.assertRaisesRegex(AssertionError,'command inventory'):validate_commands(cs)
    def test_receipt_log_hash_mismatch(self):
        raw=gzip.compress(encoded(dict(stage='2BN',target_execution='none',commands=[])),mtime=0)
        with self.assertRaisesRegex(AssertionError,'validation log identity'):validate_record(dict(logs_sha256='bad'),raw)
    def test_receipt_artifact_tampering(self):
        raw=gzip.compress(encoded(dict(stage='2BN',target_execution='none',commands=[])),mtime=0)
        record=dict(stage='2BN',artifact_sha256={'source':'tampered'},logs_sha256=digest(raw))
        with patch('verify_validation.build',return_value=dict(stage='2BN',artifact_sha256={'source':'original'})):
            with self.assertRaisesRegex(AssertionError,'receipt mismatch'):validate_record(record,raw)
