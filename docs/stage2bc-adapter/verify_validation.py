"""Current delivery receipt; historical checks are explicitly baseline-only."""
import argparse,gzip,json,re,subprocess
from adapter_diff import ROOT,HERE,digest,load,require,verify as adapter
from verify_adapter import FLAGS,HOOKS,PREFIX,baseline,inventory,verify as delivery_check
from historical import verify as historical
from reference_replay import verify as reference
RECORD=ROOT/'docs/stage2bc-validation.json'
COMMANDS=[
 ['python3','-B','docs/stage2bc-adapter/historical.py','--write'],
 ['cargo','test','--locked','--offline'],['cargo','test','--release','--locked','--offline'],
 ['cargo','build','--locked','--offline'],['cargo','build','--release','--locked','--offline'],
 ['cargo','fmt','--check'],['cargo','clippy','--locked','--offline','--all-targets','--','-D','warnings'],
 ['python3','-B','docs/stage2bc-adapter/adapter_diff.py','--write'],
 ['python3','-B','docs/stage2bc-adapter/reference_replay.py','--write'],
 ['python3','-B','docs/stage2bc-adapter/verify_adapter.py'],
 ['python3','-B','-m','unittest','discover','-s','docs/stage2bc-adapter','-p','test_*.py']]

def validate_rust(output):
    names=re.findall(r'^test (\S+) \.\.\. ok$',output,re.M)
    base=load(ROOT/'docs/stage2bb-validation.json')['current_test_names'];new=inventory()['ordinary'];expected=set(base)|set(new)
    require(len(base)==len(set(base))==679 and len(names)==len(set(names)) and set(names)==expected,'current Rust scenario inventory')
    ignored=re.findall(r'^test (\S+) \.\.\. ignored(?:,.*)?$',output,re.M)
    required={inventory()['ignored_driver'],'kernel::tests::replay_model_traces'}
    require(len(ignored)==2 and set(ignored)==required,'unexpected ignored scenario')
    summaries=re.findall(r'test result: ok\. (\d+) passed; (\d+) failed; (\d+) ignored;',output)
    require(summaries and sum(int(n) for n,_,_ in summaries)==len(expected)+2 and all(int(f)==0 for _,f,_ in summaries) and sum(int(i) for _,_,i in summaries)==2,'current Rust result summaries')
    docs=sorted(n for n in re.findall(r'^test (.+) \.\.\. ok$',output,re.M) if ' ' in n)
    prior=json.loads(gzip.decompress((ROOT/'docs/stage2bb-kernel/validation-logs.json.gz').read_bytes()))['commands'];old=next(c for c in prior if c['argv'][:2]==['cargo','test'])
    olddocs=sorted(n for n in re.findall(r'^test (.+) \.\.\. ok$',old['stdout']+old['stderr'],re.M) if ' ' in n)
    require(len(docs)==len(olddocs)==2 and docs==olddocs,'doctest identities changed')
    return sorted(names)

def validate_commands(commands):
    require([c['argv'] for c in commands]==COMMANDS,'command inventory')
    require(all(c['returncode']==0 and isinstance(c['stdout'],str) and isinstance(c['stderr'],str) for c in commands),'failed or malformed command')
    require(json.loads(commands[0]['stdout'])==historical(),'historical report differs')
    debug=validate_rust(commands[1]['stdout']+commands[1]['stderr']);release=validate_rust(commands[2]['stdout']+commands[2]['stderr'])
    require(debug==release,'profile scenario difference')
    require(json.loads(commands[-4]['stdout'])==adapter(),'adapter log result')
    require(json.loads(commands[-3]['stdout'])==reference(),'reference log result')
    require(json.loads(commands[-2]['stdout'])==delivery_check(),'delivery log result')
    out=commands[-1]['stdout']+commands[-1]['stderr'];counts=re.findall(r'^Ran (\d+) tests? in ',out,re.M)
    require(len(counts)==1 and int(counts[0])>0 and out.rstrip().endswith('OK'),'current Python tests failed')
    return dict(current_test_names=debug,rust_tests_debug=len(debug),rust_tests_release=len(release),
        baseline_rust_tests_each_profile=679,new_adapter_tests_each_profile=len(inventory()['ordinary']),
        doctests_each_profile=2,ignored_drivers_each_profile=2,separate_drivers_passed=True,
        current_python_tests=int(counts[0]),historical_python_tests=267)

def artifacts():
    paths=HOOKS|PREFIX|{'src/lifecycle_adapter.rs'}|{str(p.relative_to(ROOT)) for p in HERE.rglob('*') if p.is_file() and '__pycache__' not in p.parts and p.name!='validation-logs.json.gz'}
    return {p:digest((ROOT/p).read_bytes()) for p in sorted(paths)}
def build(commands):
    b=baseline();validation=validate_commands(commands);corpus='docs/stage2aw-evidence/corpus-validation.json'
    raw=(ROOT/corpus).read_bytes();require(digest(raw)==b['files'][corpus],'frozen corpus hash');c=json.loads(raw)
    require(c['status']=='pass' and c['target_execution']=='none','corpus scope')
    return dict(stage='2BC',status='pass',baseline_stage='2BB',baseline_commit=b['baseline_commit'],
        baseline_files=839,byte_frozen_files=833,test_hook_files=sorted(HOOKS),target_execution='none',
        business_bytecode_execution=False,**FLAGS,**validation,historical_summary=historical(),
        adapter_summary=adapter(),reference_summary=reference(),delivery_summary=delivery_check(),
        reused_corpus=dict(source=corpus,sha256=digest(raw),rerun=False,result=c),artifact_sha256=artifacts())
def verify():
    r=load(RECORD);raw=(HERE/'validation-logs.json.gz').read_bytes();logs=json.loads(gzip.decompress(raw))
    require(logs['stage']=='2BC' and logs['target_execution']=='none','log scope')
    require(r['logs_sha256']==digest(raw),'validation logs hash')
    require({k:v for k,v in r.items() if k!='logs_sha256'}==build(logs['commands']),'receipt mismatch')
    return {k:r[k] for k in ('stage','status','baseline_files','byte_frozen_files','rust_tests_debug','rust_tests_release','doctests_each_profile','historical_python_tests','current_python_tests','target_execution',*FLAGS)}
def main():
    p=argparse.ArgumentParser();p.add_argument('--write',action='store_true');a=p.parse_args()
    if a.write:
        baseline();commands=[]
        for argv in COMMANDS:
            print('Checking: '+' '.join(argv),flush=True)
            r=subprocess.run(argv,cwd=ROOT,text=True,capture_output=True)
            require(r.returncode==0,r.stdout+r.stderr)
            commands.append(dict(argv=argv,returncode=r.returncode,stdout=r.stdout,stderr=r.stderr))
        record=build(commands);raw=gzip.compress(json.dumps(dict(stage='2BC',target_execution='none',commands=commands),sort_keys=True).encode(),mtime=0)
        record['logs_sha256']=digest(raw);(HERE/'validation-logs.json.gz').write_bytes(raw);RECORD.write_text(json.dumps(record,indent=2,sort_keys=True)+'\n')
    print(json.dumps(verify(),sort_keys=True))
if __name__=='__main__':main()
