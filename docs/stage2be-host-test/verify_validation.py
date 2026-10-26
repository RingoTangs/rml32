"""Actual command logs, pinned historical authority and independent 2BE receipt."""
import argparse,gzip,json,re,subprocess
from collections import Counter
from host_diff import ROOT,HERE,digest,load,require,verify as host
from historical import verify as historical
from reference_replay import verify as reference
from verify_runtime import HOOKS,PREFIX,NEW,FLAGS,baseline,inventory,verify as runtime
RECORD=ROOT/'docs/stage2be-validation.json'
COMMANDS=[['python3','-B','docs/stage2be-host-test/historical.py','--write'],
 ['cargo','test','--locked','--offline'],['cargo','test','--release','--locked','--offline'],
 ['cargo','build','--locked','--offline'],['cargo','build','--release','--locked','--offline'],
 ['cargo','fmt','--check'],['cargo','clippy','--locked','--offline','--all-targets','--','-D','warnings'],
 ['python3','-B','docs/stage2be-host-test/host_diff.py','--write'],
 ['python3','-B','docs/stage2be-host-test/reference_replay.py','--write'],
 ['python3','-B','docs/stage2be-host-test/verify_runtime.py'],
 ['python3','-B','-m','unittest','discover','-s','docs/stage2be-host-test','-p','test_*.py']]
IGNORED={'kernel::tests::replay_model_traces','lifecycle_adapter::tests::replay_adapter_traces','vm::lifecycle_vm_tests::replay_lifecycle_vm_traces','boundary::lifecycle_boundary_tests::replay_host_lifecycle_traces'}
def doctests(out):return Counter(re.sub(r'\(line \d+\)','(line)',n)for n in re.findall(r'^test (.+) \.\.\. ok$',out,re.M)if ' 'in n)
def validate_rust(out):
    base=load(ROOT/'docs/stage2bd-validation.json')['current_test_names'];expected=set(base)|set(inventory()['ordinary']);names=re.findall(r'^test (\S+) \.\.\. ok$',out,re.M)
    require(len(base)==len(set(base))==728 and len(names)==len(set(names)) and set(names)==expected,'Rust scenario inventory')
    ignored=re.findall(r'^test (\S+) \.\.\. ignored(?:,.*)?$',out,re.M);require(len(ignored)==4 and set(ignored)==IGNORED,'ignored worker inventory')
    summaries=re.findall(r'test result: ok\. (\d+) passed; (\d+) failed; (\d+) ignored;',out);require(summaries and sum(int(n)for n,_,_ in summaries)==len(expected)+2 and all(int(f)==0 for _,f,_ in summaries) and sum(int(i)for _,_,i in summaries)==4,'Rust results')
    logs=json.loads(gzip.decompress((ROOT/'docs/stage2bd-runtime-test/validation-logs.json.gz').read_bytes()))['commands'];old=next(c for c in logs if c['argv'][:2]==['cargo','test']);docs=doctests(out);require(sum(docs.values())==2 and docs==doctests(old['stdout']+old['stderr']),'doctest identity except shifted source lines');return sorted(names)
def validate_commands(commands):
    require([c['argv']for c in commands]==COMMANDS,'command inventory');require(all(c['returncode']==0 and isinstance(c['stdout'],str) and isinstance(c['stderr'],str)for c in commands),'command failure')
    require(json.loads(commands[0]['stdout'])==historical(),'historical result')
    debug=validate_rust(commands[1]['stdout']+commands[1]['stderr']);release=validate_rust(commands[2]['stdout']+commands[2]['stderr']);require(debug==release,'profile inventory difference')
    require(json.loads(commands[-4]['stdout'])==host(),'Host result');require(json.loads(commands[-3]['stdout'])==reference(),'reference result');require(json.loads(commands[-2]['stdout'])==runtime(),'runtime result')
    out=commands[-1]['stdout']+commands[-1]['stderr'];counts=re.findall(r'^Ran (\d+) tests? in ',out,re.M);require(len(counts)==1 and int(counts[0])>0 and out.rstrip().endswith('OK'),'Python results')
    return dict(current_test_names=debug,rust_tests_debug=len(debug),rust_tests_release=len(release),baseline_rust_tests_each_profile=728,new_host_tests_each_profile=len(inventory()['ordinary']),doctests_each_profile=2,doctest_line_offsets_ignored=True,ignored_workers_each_profile=4,separate_workers_passed=True,current_python_tests=int(counts[0]),historical_python_tests=316)
def artifacts():
    paths=HOOKS|PREFIX|NEW|{str(p.relative_to(ROOT))for p in HERE.rglob('*')if p.is_file()and '__pycache__'not in p.parts and p.name!='validation-logs.json.gz'};return {p:digest((ROOT/p).read_bytes())for p in sorted(paths)}
def build(commands):
    b=baseline();checks=validate_commands(commands);corpus='docs/stage2aw-evidence/corpus-validation.json';raw=(ROOT/corpus).read_bytes();require(digest(raw)==b['files'][corpus],'corpus identity');c=json.loads(raw);require(c['status']=='pass' and c['target_execution']=='none','corpus scope')
    return dict(stage='2BE',status='pass',baseline_stage='2BD',baseline_commit=b['baseline_commit'],baseline_files=893,byte_frozen_files=886,target_execution='none',synthetic_vm_execution=True,business_bytecode_execution=False,**FLAGS,**checks,historical_summary=historical(),host_summary=host(),reference_summary=reference(),runtime_summary=runtime(),reused_corpus=dict(source=corpus,sha256=digest(raw),rerun=False,result=c),artifact_sha256=artifacts())
def verify():
    r=load(RECORD);raw=(HERE/'validation-logs.json.gz').read_bytes();logs=json.loads(gzip.decompress(raw));require(logs['stage']=='2BE' and logs['target_execution']=='none' and r['logs_sha256']==digest(raw),'validation log identity')
    require({k:v for k,v in r.items()if k!='logs_sha256'}==build(logs['commands']),'receipt mismatch')
    return {k:r[k]for k in ('stage','status','baseline_files','byte_frozen_files','rust_tests_debug','rust_tests_release','doctests_each_profile','current_python_tests','historical_python_tests','target_execution',*FLAGS)}
def main():
    p=argparse.ArgumentParser();p.add_argument('--write',action='store_true');a=p.parse_args()
    if a.write:
        baseline();commands=[]
        for argv in COMMANDS:
            print('Checking: '+' '.join(argv),flush=True);r=subprocess.run(argv,cwd=ROOT,text=True,capture_output=True);require(r.returncode==0,r.stdout+r.stderr);commands.append(dict(argv=argv,returncode=r.returncode,stdout=r.stdout,stderr=r.stderr))
        record=build(commands);raw=gzip.compress(json.dumps(dict(stage='2BE',target_execution='none',commands=commands),sort_keys=True).encode(),mtime=0);record['logs_sha256']=digest(raw);(HERE/'validation-logs.json.gz').write_bytes(raw);RECORD.write_text(json.dumps(record,indent=2,sort_keys=True)+'\n')
    print(json.dumps(verify(),sort_keys=True))
if __name__=='__main__':main()
