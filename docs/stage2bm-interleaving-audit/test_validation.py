import copy
import unittest
from common import HERE,load
from matrix import HIGH,a,put
from interleaving_oracle import Reference
from interleaving_diff import compare
from public_replay import compare as compare_public,cases
from verify_validation import COMMANDS,IGNORED,expected_tests,validate_rust,validate_commands

def specimen():
    t=dict(id='specimen',family='specimen',limits=HIGH,setup=[a('object',id='o0'),put('s0',['object','o0']),put('d0',['undefined'])],actions=[a('prepare',writes=[['s0','d0']]),a('commit')])
    oracle=Reference(HIGH);rows=[];before=oracle.projection()
    for x in t['setup']:oracle.apply(x)
    rows.append(dict(id=t['id'],step=0,before=before,after=oracle.projection(),error=None,setup_step=None,calls=0,progress=None))
    for i,x in enumerate(t['actions']):
        before=oracle.projection();oracle.apply(x)
        rows.append(dict(id=t['id'],step=i+1,before=before,after=oracle.projection(),error=None,setup_step=None,calls=0,progress=None))
    return t,rows

class ObservationTests(unittest.TestCase):
    def rejected(self, mutate):
        t,rows=specimen();mutate(rows)
        with self.assertRaises(AssertionError):compare([t],rows)
    def test_valid_specimen(self):
        t,rows=specimen();self.assertEqual(compare([t],rows)[0]['committed'],1)
    def test_missing_checkpoint(self):self.rejected(lambda rows:rows.pop())
    def test_extra_checkpoint(self):self.rejected(lambda rows:rows.append(copy.deepcopy(rows[-1])))
    def test_wrong_trace_identity(self):self.rejected(lambda rows:rows[1].update(id='other'))
    def test_wrong_step_order(self):self.rejected(lambda rows:rows.reverse())
    def test_counter_pollution(self):self.rejected(lambda rows:rows[-1]['after']['objects']['o0'].update(ordinary=999))
    def test_false_success_error(self):self.rejected(lambda rows:rows[-1].update(error='Limit'))
    def test_discontinuous_before(self):self.rejected(lambda rows:rows[-1]['before']['slots'].update(s0=['undefined']))

def public_specimen():
    ts=[t for t in cases() if t['limits']==HIGH]
    rows=[dict(id=t['id'],result=dict(copy_valid=t['state'] in ('alive','pending'),drained=int(t['state'] in ('pending','queued')),reaped=1,old_id_rejected=True,frames=0)) for t in ts]
    return ts,rows

class PublicTests(unittest.TestCase):
    def test_complete_controls(self):
        ts,rows=public_specimen();self.assertEqual(compare_public(ts,rows)['accepted'],32)
    def test_missing_public_control(self):
        ts,rows=public_specimen()
        with self.assertRaises(AssertionError):compare_public(ts,rows[:-1])
    def test_wrong_pending_copy_semantics(self):
        ts,rows=public_specimen();next(r for t,r in zip(ts,rows) if t['state']=='pending')['result']['copy_valid']=False
        with self.assertRaises(AssertionError):compare_public(ts,rows)
    def test_high_quota_refusal_is_not_coverage(self):
        ts,rows=public_specimen();rows[0]=dict(id=ts[0]['id'],phase='setup',error='Limit')
        with self.assertRaises(AssertionError):compare_public(ts,rows)

def cargo_log():
    ordinary=[f'test {n} ... ok' for n in sorted(expected_tests())]
    ignored=[f'test {n} ... ignored, explicit replay' for n in sorted(IGNORED)]
    docs=[f'test src/boundary.rs - boundary::HostValue (line {n}) - compile fail ... ok' for n in (531,534)]
    docs += ['test src/lifecycle.rs - lifecycle::LifecycleLimits (line 6) - compile fail ... ok']
    docs += [f'test src/lifecycle.rs - lifecycle::LifecycleBudget (line {n}) - compile fail ... ok' for n in (30,33)]
    return '\n'.join(ordinary+ignored+docs+['test result: ok. 883 passed; 0 failed; 15 ignored;'])

class ReceiptTests(unittest.TestCase):
    def test_exact_old_and_new_inventory(self):self.assertEqual(len(validate_rust(cargo_log())),878)
    def test_missing_old_test(self):
        text=cargo_log();name=load(HERE.parent/'stage2bl-validation.json')['current_test_names'][0]
        with self.assertRaises(AssertionError):validate_rust(text.replace(f'test {name} ... ok',''))
    def test_duplicate_test(self):
        text=cargo_log()
        with self.assertRaises(AssertionError):validate_rust(text+'\n'+text.splitlines()[0])
    def test_missing_ignored_driver(self):
        text=cargo_log();name=sorted(IGNORED)[0]
        with self.assertRaises(AssertionError):validate_rust(text.replace(f'test {name} ... ignored, explicit replay',''))
    def test_missing_doctest(self):
        text=cargo_log()
        with self.assertRaises(AssertionError):validate_rust('\n'.join(l for l in text.splitlines() if '(line 6)' not in l))
    def test_failed_command(self):
        commands=[dict(argv=argv,returncode=1,stdout='',stderr='failed') for argv in COMMANDS]
        with self.assertRaisesRegex(AssertionError,'failed command'):validate_commands(commands)
    def test_reordered_commands(self):
        commands=[dict(argv=argv,returncode=0,stdout='',stderr='') for argv in reversed(COMMANDS)]
        with self.assertRaisesRegex(AssertionError,'command inventory'):validate_commands(commands)
