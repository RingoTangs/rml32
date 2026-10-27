"""Actual D08 command logs; design proof is distinct from existing Rust path checks."""
import argparse,gzip,json,re,subprocess
from common import ROOT,HERE,digest,load,require,encoded
from historical import verify as historical
from reference_replay import verify as reference
from check_contract import verify as checks
from verify_boundary import PREFIX,FLAGS,baseline,verify as boundary
RECORD=ROOT/'docs/stage2bh-validation.json'
COMMANDS=[['python3','-B','docs/stage2bh-production-contract/historical.py','--write'],
 ['cargo','test','--locked','--offline'],['cargo','test','--release','--locked','--offline'],
 ['cargo','build','--locked','--offline'],['cargo','build','--release','--locked','--offline'],
 ['cargo','fmt','--check'],['cargo','clippy','--locked','--offline','--all-targets','--','-D','warnings'],
 ['python3','-B','docs/stage2bh-production-contract/check_contract.py','--write'],
 ['python3','-B','docs/stage2bh-production-contract/reference_replay.py','--write'],
 ['python3','-B','docs/stage2bh-production-contract/verify_boundary.py'],
 ['python3','-B','-m','unittest','discover','-s','docs/stage2bh-production-contract','-p','test_*.py']]
IGNORED={'kernel::tests::replay_model_traces','lifecycle_adapter::tests::replay_adapter_traces','vm::lifecycle_vm_tests::replay_lifecycle_vm_traces','boundary::lifecycle_boundary_tests::replay_host_lifecycle_traces','boundary::lifecycle_managed_tests::replay_managed_lifecycle_traces','boundary::lifecycle_host_function_tests::replay_host_function_traces'}
def validate_rust(out):
    names=re.findall(r'^test (\S+) \.\.\. ok$',out,re.M);expected=load(ROOT/'docs/stage2bg-validation.json')['current_test_names'];require(len(names)==len(set(names))==len(expected)==819 and set(names)==set(expected),'Rust scenario inventory')
    ignored=re.findall(r'^test (\S+) \.\.\. ignored(?:,.*)?$',out,re.M);require(len(ignored)==6 and set(ignored)==IGNORED,'ignored driver inventory')
    results=re.findall(r'test result: ok\. (\d+) passed; (\d+) failed; (\d+) ignored;',out);require(results and sum(int(n)for n,_,_ in results)==821 and all(int(f)==0 for _,f,_ in results) and sum(int(i)for _,_,i in results)==6,'Rust results')
    logs=json.loads(gzip.decompress((ROOT/'docs/stage2bg-host-function-test/validation-logs.json.gz').read_bytes()))['commands'];old=next(c for c in logs if c['argv'][:2]==['cargo','test']);docs=lambda text:sorted(n for n in re.findall(r'^test (.+) \.\.\. ok$',text,re.M)if ' 'in n);require(len(docs(out))==2 and docs(out)==docs(old['stdout']+old['stderr']),'exact frozen doctest identities');return sorted(names)
def validate_commands(commands):
    require([c['argv']for c in commands]==COMMANDS,'command inventory');require(all(c['returncode']==0 and isinstance(c['stdout'],str) and isinstance(c['stderr'],str)for c in commands),'command failure')
    require(json.loads(commands[0]['stdout'])==historical(),'historical result');debug=validate_rust(commands[1]['stdout']+commands[1]['stderr']);release=validate_rust(commands[2]['stdout']+commands[2]['stderr']);require(debug==release,'profile inventory')
    require(json.loads(commands[-4]['stdout'])==checks(),'contract result');require(json.loads(commands[-3]['stdout'])==reference(),'reference result');require(json.loads(commands[-2]['stdout'])==boundary(),'boundary result');out=commands[-1]['stdout']+commands[-1]['stderr'];counts=re.findall(r'^Ran (\d+) tests? in ',out,re.M);require(len(counts)==1 and int(counts[0])>0 and out.rstrip().endswith('OK'),'Python test results')
    return dict(current_test_names=debug,rust_tests_debug=819,rust_tests_release=819,new_rust_tests=0,doctests_each_profile=2,doctest_line_offsets_ignored=False,ignored_workers_each_profile=6,separate_workers_passed=True,current_python_tests=int(counts[0]),historical_python_tests=392)
def artifacts():
    paths=PREFIX|{str(p.relative_to(ROOT))for p in HERE.rglob('*')if p.is_file()and '__pycache__'not in p.parts and p.name!='validation-logs.json.gz'};return {p:digest((ROOT/p).read_bytes())for p in sorted(paths)}
def build(commands):
    b=baseline();tested=validate_commands(commands);corpus='docs/stage2aw-evidence/corpus-validation.json';raw=(ROOT/corpus).read_bytes();require(digest(raw)==b['files'][corpus],'corpus identity');c=json.loads(raw);require(c['status']=='pass' and c['target_execution']=='none','corpus scope')
    return dict(stage='2BH',status='pass',baseline_stage='2BG',baseline_commit=b['baseline_commit'],baseline_files=980,byte_frozen_files=978,target_execution='none',synthetic_vm_execution=True,business_bytecode_execution=False,**FLAGS,**tested,contract_summary=checks(),historical_summary=historical(),reference_summary=reference(),boundary_summary=boundary(),reused_corpus=dict(source=corpus,sha256=digest(raw),rerun=False,result=c),artifact_sha256=artifacts())
def verify():
    r=load(RECORD);raw=(HERE/'validation-logs.json.gz').read_bytes();logs=json.loads(gzip.decompress(raw));require(logs['stage']=='2BH' and logs['target_execution']=='none' and r['logs_sha256']==digest(raw),'validation logs');require({k:v for k,v in r.items()if k!='logs_sha256'}==build(logs['commands']),'receipt mismatch');return {k:r[k]for k in ('stage','status','baseline_files','byte_frozen_files','rust_tests_debug','rust_tests_release','new_rust_tests','doctests_each_profile','current_python_tests','historical_python_tests','target_execution',*FLAGS)}
def main():
    p=argparse.ArgumentParser();p.add_argument('--write',action='store_true');a=p.parse_args()
    if a.write:
        baseline();commands=[]
        for argv in COMMANDS:
            print('Checking: '+' '.join(argv),flush=True);r=subprocess.run(argv,cwd=ROOT,text=True,capture_output=True);require(r.returncode==0,r.stdout+r.stderr);commands.append(dict(argv=argv,returncode=r.returncode,stdout=r.stdout,stderr=r.stderr))
        record=build(commands);raw=gzip.compress(encoded(dict(stage='2BH',target_execution='none',commands=commands)),mtime=0);record['logs_sha256']=digest(raw);(HERE/'validation-logs.json.gz').write_bytes(raw);RECORD.write_bytes(encoded(record))
    print(json.dumps(verify(),sort_keys=True))
if __name__=='__main__':main()
