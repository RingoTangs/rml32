"""Actual command inventory, exact old/new Rust names and sealed source/artifact hashes."""
import argparse,gzip,json,re,subprocess
from common import ROOT,HERE,require,digest,encoded,load
from verify_boundary import verify as boundary
from historical import verify as historical
from reference_replay import verify as reference
from batch_diff import verify as batch
from combination_diff import verify as combinations
RECORD=ROOT/'docs/stage2bk-validation.json';PREFIX='docs/stage2bk-combination-audit/'
COMMANDS=[['python3','-B',PREFIX+'historical.py','--write'],['cargo','test','--locked','--offline'],['cargo','test','--release','--locked','--offline'],['cargo','build','--locked','--offline'],['cargo','build','--release','--locked','--offline'],['cargo','fmt','--check'],['cargo','clippy','--locked','--offline','--all-targets','--','-D','warnings'],['python3','-B',PREFIX+'reference_replay.py','--write'],['python3','-B',PREFIX+'batch_diff.py','--write'],['python3','-B',PREFIX+'combination_diff.py','--write'],['python3','-B',PREFIX+'verify_boundary.py'],['rustc','-Vv'],['rustup','target','list','--installed'],['python3','-B','-m','unittest','discover','-s',PREFIX.rstrip('/'),'-p','test_*.py']]
OLD_IGNORED={'kernel::tests::replay_model_traces','lifecycle_adapter::tests::replay_adapter_traces','vm::lifecycle_vm_tests::replay_lifecycle_vm_traces','boundary::lifecycle_boundary_tests::replay_host_lifecycle_traces','boundary::lifecycle_managed_tests::replay_managed_lifecycle_traces','boundary::lifecycle_host_function_tests::replay_host_function_traces','boundary::lifecycle_contract_tests::replay_production_contract_traces','replay_public_contract_traces'}
IGNORED=OLD_IGNORED|{'boundary::lifecycle_combination_tests::replay_combination_batches','boundary::lifecycle_combination_tests::production::replay_instrumented_combinations','replay_public_combinations'}
PYTHON_TESTS=28
def expected_tests():return set(load(ROOT/'docs/stage2bj-validation.json')['current_test_names'])|set(load(HERE/'case-coverage.json')['new_test_names'])
def validate_rust(out):
 passed=re.findall(r'^test (\S+) \.\.\. ok$',out,re.M);require(len(passed)==len(set(passed))==863 and set(passed)==expected_tests(),'Rust scenario inventory')
 ignored=re.findall(r'^test (\S+) \.\.\. ignored(?:,.*)?$',out,re.M);require(len(ignored)==len(set(ignored))==11 and set(ignored)==IGNORED,'ignored driver inventory')
 docs=re.findall(r'^test (.+ \(line \d+\).+) \.\.\. ok$',out,re.M);require(len(docs)==len(set(docs))==5 and sum('boundary::HostValue'in n for n in docs)==2 and sum('lifecycle::LifecycleLimits'in n for n in docs)==1 and sum('lifecycle::LifecycleBudget'in n for n in docs)==2,'doctest inventory')
 results=re.findall(r'test result: ok\. (\d+) passed; (\d+) failed; (\d+) ignored;',out);require(results and sum(int(n)for n,_,_ in results)==868 and all(int(f)==0 for _,f,_ in results)and sum(int(i)for _,_,i in results)==11,'Rust results');return sorted(passed)
def validate_commands(cs):
 require([c['argv']for c in cs]==COMMANDS,'command inventory');require(all(c['returncode']==0 and isinstance(c['stdout'],str)and isinstance(c['stderr'],str)for c in cs),'failed command')
 for i,verify in [(0,historical),(7,reference),(8,batch),(9,combinations),(10,boundary)]:require(json.loads(cs[i]['stdout'])==verify(),'subreport result '+str(i))
 names=validate_rust(cs[1]['stdout']+cs[1]['stderr']);require(names==validate_rust(cs[2]['stdout']+cs[2]['stderr']),'profile tests');require('host: aarch64-apple-darwin'in cs[11]['stdout']and cs[12]['stdout'].splitlines()==['aarch64-apple-darwin'],'target scope')
 out=cs[-1]['stdout']+cs[-1]['stderr'];require(re.findall(r'^Ran (\d+) tests? in ',out,re.M)==[str(PYTHON_TESTS)]and out.rstrip().endswith('OK'),'current Python results');return names

def artifacts():
 b=load(HERE/'baseline-inputs.json');paths={n for n in b['allowed_changes']+b['allowed_prefixes']if digest((ROOT/n).read_bytes())!=b['files'][n]}|set(b['new_source'])|{str(p.relative_to(ROOT))for p in HERE.rglob('*')if p.is_file()and '__pycache__'not in p.parts and p.name!='validation-log.json.gz'}
 return {n:digest((ROOT/n).read_bytes())for n in sorted(paths)}
def build(cs):
 names=validate_commands(cs);b=boundary();g=load(HERE/'gate-review.json');require(g['runtime_changed']is False and all(g[k]is False for k in ('public_api_changed','dependencies_changed','quota_semantics_changed','existing_models_changed','new_native_analysis','direct_owner_capture_ready','implementation_ready','stage2_complete','full_model_equivalence','native_equivalence_proven'))and g['new_batch_reference_layer_added']is True and g['new_production_deviations']==0 and g['native_unknowns']==20,'gate scope')
 return dict(stage='2BK',status='pass',baseline_files=1096,byte_frozen_files=b['byte_frozen_files'],baseline_snapshot_sha256=digest(encoded(load(HERE/'baseline-inputs.json')['files'])),current_test_names=names,old_rust_tests=857,new_rust_tests=6,rust_tests_debug=863,rust_tests_release=863,doctests_each_profile=5,ignored_drivers_each_profile=11,current_python_tests=PYTHON_TESTS,historical_python_tests=497,historical_run_on_current_source=False,quota_model_cases=39,arithmetic_rows=112,gates=g,boundary=b,historical=historical(),reference=reference(),batch=batch(),combinations=combinations(),reused_corpus=dict(path='docs/stage2aw-evidence/corpus-validation.json',sha256=digest((ROOT/'docs/stage2aw-evidence/corpus-validation.json').read_bytes()),rerun=False),artifact_sha256=artifacts())
def validate_record(r,raw):
 log=json.loads(gzip.decompress(raw));require(log['stage']=='2BK'and log['target_execution']=='none'and r['logs_sha256']==digest(raw),'validation logs');require({k:v for k,v in r.items()if k!='logs_sha256'}==build(log['commands']),'receipt mismatch')
def verify():
 r=load(RECORD);validate_record(r,(HERE/'validation-log.json.gz').read_bytes());return {k:r[k]for k in ('stage','status','baseline_files','byte_frozen_files','rust_tests_debug','rust_tests_release','new_rust_tests','doctests_each_profile','current_python_tests','historical_python_tests','gates')}
if __name__=='__main__':
 p=argparse.ArgumentParser();p.add_argument('--write',action='store_true');a=p.parse_args()
 if a.write:
  boundary();cs=[]
  for argv in COMMANDS:
   print('Checking: '+' '.join(argv),flush=True);r=subprocess.run(argv,cwd=ROOT,text=True,capture_output=True);require(r.returncode==0,r.stdout+r.stderr);cs.append(dict(argv=argv,returncode=r.returncode,stdout=r.stdout,stderr=r.stderr))
  r=build(cs);raw=gzip.compress(encoded(dict(stage='2BK',target_execution='none',commands=cs)),mtime=0);r['logs_sha256']=digest(raw);(HERE/'validation-log.json.gz').write_bytes(raw);RECORD.write_bytes(encoded(r))
 print(json.dumps(verify(),sort_keys=True))
