"""Capture actual production builds/tests separately from frozen model and history."""
import argparse,gzip,json,re,subprocess
from common import ROOT,HERE,digest,load,require,encoded
from verify_boundary import baseline,verify as boundary
from historical import verify as historical
from reference_replay import verify as reference
from model_replay import verify as model
RECORD=ROOT/'docs/stage2bi-validation.json'
COMMANDS=[['python3','-B','docs/stage2bi-production-lifecycle/historical.py','--write'],
 ['cargo','test','--locked','--offline'],['cargo','test','--release','--locked','--offline'],
 ['cargo','build','--locked','--offline'],['cargo','build','--release','--locked','--offline'],
 ['cargo','fmt','--check'],['cargo','clippy','--locked','--offline','--all-targets','--','-D','warnings'],
 ['python3','-B','docs/stage2bi-production-lifecycle/model_replay.py','--write'],
 ['python3','-B','docs/stage2bi-production-lifecycle/reference_replay.py','--write'],
 ['python3','-B','docs/stage2bi-production-lifecycle/verify_boundary.py'],
 ['python3','-B','-m','unittest','discover','-s','docs/stage2bi-production-lifecycle','-p','test_*.py']]
IGNORED={'kernel::tests::replay_model_traces','lifecycle_adapter::tests::replay_adapter_traces','vm::lifecycle_vm_tests::replay_lifecycle_vm_traces','boundary::lifecycle_boundary_tests::replay_host_lifecycle_traces','boundary::lifecycle_managed_tests::replay_managed_lifecycle_traces','boundary::lifecycle_host_function_tests::replay_host_function_traces'}
def expected_tests():return set(load(ROOT/'docs/stage2bh-validation.json')['current_test_names'])|set(load(HERE/'case-coverage.json')['rust_test_names'])
def validate_rust(out):
    passed=re.findall(r'^test (\S+) \.\.\. ok$',out,re.M);require(len(passed)==len(set(passed))==851 and set(passed)==expected_tests(),'Rust scenario inventory')
    ignored=re.findall(r'^test (\S+) \.\.\. ignored(?:,.*)?$',out,re.M);require(len(ignored)==6 and set(ignored)==IGNORED,'ignored driver inventory')
    docs=re.findall(r'^test (.+ \(line \d+\).+) \.\.\. ok$',out,re.M);require(len(docs)==len(set(docs))==5 and sum('boundary::HostValue'in n for n in docs)==2 and sum('lifecycle::LifecycleLimits'in n for n in docs)==1 and sum('lifecycle::LifecycleBudget'in n for n in docs)==2,'doctest inventory')
    results=re.findall(r'test result: ok\. (\d+) passed; (\d+) failed; (\d+) ignored;',out);require(results and sum(int(n)for n,_,_ in results)==856 and all(int(f)==0 for _,f,_ in results) and sum(int(i)for _,_,i in results)==6,'Rust results')
    return sorted(passed)
def validate_commands(commands):
    require([c['argv']for c in commands]==COMMANDS,'command inventory');require(all(c['returncode']==0 and isinstance(c['stdout'],str)and isinstance(c['stderr'],str)for c in commands),'command failure')
    require(json.loads(commands[0]['stdout'])==historical(),'historical result')
    debug=validate_rust(commands[1]['stdout']+commands[1]['stderr']);release=validate_rust(commands[2]['stdout']+commands[2]['stderr']);require(debug==release,'profile inventory')
    require(json.loads(commands[-4]['stdout'])==model(),'model result');require(json.loads(commands[-3]['stdout'])==reference(),'reference result');require(json.loads(commands[-2]['stdout'])==boundary(),'boundary result')
    out=commands[-1]['stdout']+commands[-1]['stderr'];require(re.findall(r'^Ran (\d+) tests? in ',out,re.M)==['10'] and out.rstrip().endswith('OK'),'Python checks')
    return dict(current_test_names=debug,rust_tests_debug=851,rust_tests_release=851,old_rust_tests=819,new_rust_tests=32,doctests_each_profile=5,new_doctests=3,legacy_doctest_line_offsets_ignored=True,ignored_workers_each_profile=6,current_python_tests=10,quota_model_cases_reexecuted=39,arithmetic_rows=112,historical_python_tests=455,historical_python_tests_run_on_current_source=False)
def artifacts():
    b,_=baseline();paths=set(b['allowed_changes'])|set(b['new_source'])|set(b['allowed_prefixes'])|{str(p.relative_to(ROOT))for p in HERE.rglob('*')if p.is_file()and '__pycache__'not in p.parts and p.name!='validation-logs.json.gz'}
    return {p:digest((ROOT/p).read_bytes())for p in sorted(paths)}
def build(commands):
    tested=validate_commands(commands);scope=boundary();corpus='docs/stage2aw-evidence/corpus-validation.json';old,_=baseline();require(digest((ROOT/corpus).read_bytes())==old['files'][corpus],'corpus identity')
    return dict(stage='2BI',status='pass',baseline_commit=old['baseline_commit'],baseline_files=1017,byte_frozen_files=1004,target_execution='none',synthetic_vm_execution=True,business_bytecode_execution=False,**{k:v for k,v in load(HERE/'gate-review.json').items()if k not in ('stage','target_execution')},**tested,boundary_summary=scope,historical_summary=historical(),reference_summary=reference(),model_summary=model(),reused_corpus=dict(source=corpus,sha256=old['files'][corpus],rerun=False),artifact_sha256=artifacts())
def verify():
    r=load(RECORD);raw=(HERE/'validation-logs.json.gz').read_bytes();log=json.loads(gzip.decompress(raw));require(log['stage']=='2BI' and r['logs_sha256']==digest(raw),'validation logs');require({k:v for k,v in r.items()if k!='logs_sha256'}==build(log['commands']),'receipt mismatch')
    return {k:r[k]for k in ('stage','status','baseline_files','byte_frozen_files','rust_tests_debug','rust_tests_release','new_rust_tests','doctests_each_profile','current_python_tests','quota_model_cases_reexecuted','historical_python_tests','production_lifecycle_implemented','public_api_changed','dependencies_changed','implementation_ready','stage2_complete','target_execution')}
def main():
    p=argparse.ArgumentParser();p.add_argument('--write',action='store_true');a=p.parse_args()
    if a.write:
        baseline();commands=[]
        for argv in COMMANDS:
            print('Checking: '+' '.join(argv),flush=True);r=subprocess.run(argv,cwd=ROOT,text=True,capture_output=True);require(r.returncode==0,r.stdout+r.stderr);commands.append(dict(argv=argv,returncode=r.returncode,stdout=r.stdout,stderr=r.stderr))
        result=build(commands);raw=gzip.compress(encoded(dict(stage='2BI',target_execution='none',commands=commands)),mtime=0);result['logs_sha256']=digest(raw);(HERE/'validation-logs.json.gz').write_bytes(raw);RECORD.write_bytes(encoded(result))
    print(json.dumps(verify(),sort_keys=True))
if __name__=='__main__':main()
