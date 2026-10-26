"""Run real checks, freeze their logs and read-only verify the 2BB receipt."""
import argparse
import gzip
import importlib.util
import json
import re
import subprocess
import sys
from verify_kernel import ROOT,HERE,PREFIX,RUST,FLAGS,baseline,inventory,verify as kernel
from differential import digest,load,require,verify as differential

# Import old verifiers only for frozen log replay, never whole-delivery checks.
sys.path.insert(0,str(ROOT/'docs/stage2ba-design'))
spec=importlib.util.spec_from_file_location('_2bb_frozen_ba',ROOT/'docs/stage2ba-design/verify_validation.py')
OLD=importlib.util.module_from_spec(spec);spec.loader.exec_module(OLD)
sys.path.pop(0)
RECORD=ROOT/'docs/stage2bb-validation.json'
PYTHON=[
 ('docs/stage2as-design','test_model.py',50),('docs/stage2as-design','test_explore.py',6),
 ('docs/stage2ax-evidence','test_verify_evidence.py',37),('docs/stage2ax-evidence','test_verify_validation.py',15),
 ('docs/stage2ay-evidence','test_verify_evidence.py',20),('docs/stage2ay-evidence','test_verify_validation.py',15),
 ('docs/stage2az-evidence','test_verify_evidence.py',37),('docs/stage2az-evidence','test_verify_validation.py',15),
 ('docs/stage2ba-design','test_*.py',40)]
COMMANDS=[['python3','-B','docs/stage2az-evidence/verify_evidence.py'],
          ['python3','-B','docs/stage2ba-design/explore.py']]+[
 ['python3','-B','-m','unittest','discover','-s',folder,'-p',pattern] for folder,pattern,_ in PYTHON]+[
 ['cargo','test','--locked','--offline'],['cargo','test','--release','--locked','--offline'],
 ['cargo','build','--locked','--offline'],['cargo','fmt','--check'],
 ['cargo','clippy','--locked','--offline','--all-targets','--','-D','warnings'],
 ['python3','-B','docs/stage2bb-kernel/differential.py','--write'],
 ['python3','-B','docs/stage2bb-kernel/verify_kernel.py'],
 ['python3','-B','-m','unittest','discover','-s','docs/stage2bb-kernel','-p','test_*.py']]

def frozen():
    b=baseline();p=ROOT/'docs/stage2ba-validation.json';r=load(p)
    raw=(ROOT/'docs/stage2ba-design/validation-logs.json.gz').read_bytes()
    require(digest(p.read_bytes())==b['files']['docs/stage2ba-validation.json'],'frozen receipt identity')
    require(digest(raw)==r['logs_sha256'],'frozen logs hash')
    logs=json.loads(gzip.decompress(raw));require(logs['stage']=='2BA' and logs['target_execution']=='none','frozen log scope')
    result=OLD.validate_commands(logs['commands'])
    require(all(r[k]==v for k,v in result.items()),'frozen log summary')
    require(r['preserved_python_tests']+r['new_python_tests']==235,'frozen Python inventory')
    return dict(stage='2BA',method='frozen log replay; no whole-delivery verifier',receipt_sha256=digest(p.read_bytes()),logs_sha256=digest(raw))

def validate_rust(output):
    names=re.findall(r'^test (\S+) \.\.\. ok$',output,re.M)
    base=load(ROOT/'docs/stage2ba-validation.json')['current_test_names']
    new=inventory()['ordinary'];expected=set(base)|set(new)
    require(len(base)==644 and len(names)==len(set(names)) and set(names)==expected,'Rust scenario identity')
    ignored=re.findall(r'^test (\S+) \.\.\. ignored(?:,.*)?$',output,re.M)
    require(ignored==[inventory()['ignored_driver']],'unexpected ignored Rust scenario')
    summaries=re.findall(r'test result: ok\. (\d+) passed; (\d+) failed; (\d+) ignored;',output)
    require(summaries and sum(int(n) for n,_,_ in summaries)==len(expected)+2 and
            all(int(f)==0 for _,f,_ in summaries) and sum(int(i) for _,_,i in summaries)==1,'Rust result summary')
    docs=sorted(n for n in re.findall(r'^test (.+) \.\.\. ok$',output,re.M) if ' ' in n)
    oldlogs=json.loads(gzip.decompress((ROOT/'docs/stage2ba-design/validation-logs.json.gz').read_bytes()))['commands']
    old=next(c for c in oldlogs if c['argv'][:2]==['cargo','test'])
    olddocs=sorted(n for n in re.findall(r'^test (.+) \.\.\. ok$',old['stdout']+old['stderr'],re.M) if ' ' in n)
    require(len(docs)==len(olddocs)==2 and docs==olddocs,'doctest identities')
    return sorted(names)

def validate_commands(commands):
    require([c['argv'] for c in commands]==COMMANDS,'command inventory')
    require(all(c['returncode']==0 and isinstance(c['stdout'],str) and isinstance(c['stderr'],str) for c in commands),'failed or malformed command')
    require(json.loads(commands[0]['stdout'])==load(ROOT/'docs/stage2az-evidence/summary.json'),'evidence summary')
    require(json.loads(commands[1]['stdout'])==load(ROOT/'docs/stage2ba-design/enumeration.json'),'frozen enumeration')
    counts={};rust=[]
    for c in commands:
        a=c['argv'];output=c['stdout']+c['stderr']
        if 'unittest' in a:counts[a[a.index('-s')+1]+'/'+a[a.index('-p')+1]]=OLD.OLD.python_count(output)
        if a[:2]==['cargo','test']:rust.append(validate_rust(output))
    require(len(rust)==2 and rust[0]==rust[1],'debug/release inventory differs')
    require(all(counts[p+'/'+pattern]==n for p,pattern,n in PYTHON),'historical Python test count')
    require(json.loads(commands[-3]['stdout'])==differential(),'differential log result')
    require(json.loads(commands[-2]['stdout'])==kernel(),'kernel log result')
    return dict(preserved_python_tests=235,new_python_tests=counts['docs/stage2bb-kernel/test_*.py'],
        baseline_rust_tests_each_profile=644,new_rust_tests_each_profile=len(inventory()['ordinary']),
        rust_tests_debug=len(rust[0]),rust_tests_release=len(rust[1]),doctests_each_profile=2,
        ignored_driver_each_profile=1,separate_driver_passed=True,current_test_names=rust[0],python_test_counts=counts)

def delivery():
    paths=PREFIX|RUST|{str(p.relative_to(ROOT)) for p in HERE.rglob('*') if p.is_file() and '__pycache__' not in p.parts and p.name!='validation-logs.json.gz'}
    return {p:digest((ROOT/p).read_bytes()) for p in sorted(paths)}

def build(commands):
    b=baseline();checks=validate_commands(commands);corpus='docs/stage2aw-evidence/corpus-validation.json'
    require(digest((ROOT/corpus).read_bytes())==b['files'][corpus],'reused corpus hash')
    c=load(ROOT/corpus);require(c['status']=='pass' and c['target_execution']=='none','corpus scope')
    return dict(stage='2BB',status='pass',baseline_stage='2BA',baseline_commit=b['baseline_commit'],
        protected_previous_files=820,target_execution='none',rust_kernel_execution=True,
        business_bytecode_execution=False,**FLAGS,**checks,baseline_verification=frozen(),
        kernel_summary=kernel(),differential_summary=differential(),
        reused_corpus=dict(source=corpus,sha256=digest((ROOT/corpus).read_bytes()),rerun=False,result=c),
        artifact_sha256=delivery())

def verify():
    raw=(HERE/'validation-logs.json.gz').read_bytes();logs=json.loads(gzip.decompress(raw));r=load(RECORD)
    require(logs['stage']=='2BB' and logs['target_execution']=='none','log scope')
    require(digest(raw)==r['logs_sha256'],'validation log hash')
    require({k:v for k,v in r.items() if k!='logs_sha256'}==build(logs['commands']),'receipt mismatch')
    return {k:r[k] for k in ('stage','status','protected_previous_files','preserved_python_tests','new_python_tests',
        'rust_tests_debug','rust_tests_release','doctests_each_profile','target_execution',*FLAGS)}

def main():
    p=argparse.ArgumentParser();p.add_argument('--write',action='store_true');a=p.parse_args()
    if a.write:
        baseline();frozen();commands=[]
        for argv in COMMANDS:
            print('Checking: '+' '.join(argv),flush=True)
            r=subprocess.run(argv,cwd=ROOT,text=True,capture_output=True)
            require(r.returncode==0,r.stdout+r.stderr)
            commands.append(dict(argv=argv,returncode=r.returncode,stdout=r.stdout,stderr=r.stderr))
        record=build(commands)
        raw=gzip.compress(json.dumps(dict(stage='2BB',target_execution='none',commands=commands),sort_keys=True).encode(),mtime=0)
        record['logs_sha256']=digest(raw);(HERE/'validation-logs.json.gz').write_bytes(raw)
        RECORD.write_text(json.dumps(record,indent=2,sort_keys=True)+'\n')
    print(json.dumps(verify(),sort_keys=True))
if __name__=='__main__':main()
