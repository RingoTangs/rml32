"""Actual check runner and read-only 2BA receipt verifier."""
import argparse
import gzip
import importlib.util
import json
from pathlib import Path
import subprocess
from verify_design import ROOT,HERE,PREFIX,baseline,case_map,digest,load,require,verify as design

def module(name,path):
    s=importlib.util.spec_from_file_location(name,path);m=importlib.util.module_from_spec(s);s.loader.exec_module(m);return m
OLD=module('_2ba_old',ROOT/'docs/stage2az-evidence/verify_validation.py')
RECORD=ROOT/'docs/stage2ba-validation.json'
COMMANDS=OLD.COMMANDS[:]+[
 ['python3','-B','docs/stage2ba-design/verify_design.py'],
 ['python3','-B','docs/stage2ba-design/explore.py'],
 ['python3','-B','-m','unittest','discover','-s','docs/stage2ba-design','-p','test_*.py']]
FLAGS=dict(runtime_changed=False,public_api_changed=False,dependencies_changed=False,
 model_changed=True,model_added=True,new_native_analysis=False,container_capture_ready=True,
 direct_owner_capture_ready=False,non_scalar_ready=False,stage2_complete=False,
 design_ready=True,implementation_ready=False)

def frozen():
    b=baseline();p=ROOT/'docs/stage2az-validation.json';r=load(p)
    raw=(ROOT/'docs/stage2az-evidence/validation-logs.json.gz').read_bytes()
    require(digest(p.read_bytes())==b['files']['docs/stage2az-validation.json'],'frozen receipt')
    require(digest(raw)==r['logs_sha256'],'frozen log hash')
    logs=json.loads(gzip.decompress(raw));v=OLD.validate_commands(logs['commands'])
    require(all(r[k]==x for k,x in v.items()),'frozen summary')
    return dict(stage='2AZ',method='frozen log replay',receipt_sha256=digest(p.read_bytes()),logs_sha256=digest(raw))

def validate_commands(commands):
    require([c['argv'] for c in commands]==COMMANDS,'command inventory')
    for c in commands:require(c['returncode']==0 and isinstance(c['stdout'],str) and isinstance(c['stderr'],str),'command failed')
    old=OLD.validate_commands(commands[:len(OLD.COMMANDS)])
    counts=old['python_test_counts']
    require(sum(counts.values())==195,'preserved Python count')
    require(json.loads(commands[-3]['stdout'])==design(),'design result')
    require(json.loads(commands[-2]['stdout'])==load(HERE/'enumeration.json'),'enumeration result')
    current=OLD.python_count(commands[-1]['stdout']+commands[-1]['stderr'])
    return dict(preserved_python_tests=195,new_python_tests=current,
        rust_tests_debug=644,rust_tests_release=644,doctests_each_profile=2,
        current_test_names=old['current_test_names'])

def delivery():
    paths=PREFIX|{str(p.relative_to(ROOT)) for p in HERE.rglob('*') if p.is_file() and '__pycache__' not in p.parts and p.name!='validation-logs.json.gz'}
    return {p:digest((ROOT/p).read_bytes()) for p in sorted(paths)}

def build(commands):
    base=baseline();validation=validate_commands(commands)
    corpus='docs/stage2aw-evidence/corpus-validation.json';c=load(ROOT/corpus)
    require(c['status']=='pass' and c['target_execution']=='none','corpus scope')
    return dict(stage='2BA',status='pass',baseline_stage='2AZ',baseline_commit=base['baseline_commit'],
        protected_previous_files=800,baseline_verification=frozen(),target_execution='none',
        **FLAGS,**validation,design_summary=design(),model_summary=load(HERE/'enumeration.json'),
        reused_corpus=dict(source=corpus,sha256=digest((ROOT/corpus).read_bytes()),rerun=False,result=c),
        artifact_sha256=delivery())

def verify():
    raw=(HERE/'validation-logs.json.gz').read_bytes();logs=json.loads(gzip.decompress(raw));r=load(RECORD)
    require(logs['stage']=='2BA' and logs['target_execution']=='none','log scope')
    require(r['logs_sha256']==digest(raw),'log hash')
    require({k:v for k,v in r.items() if k!='logs_sha256'}==build(logs['commands']),'receipt mismatch')
    return dict(stage='2BA',status='pass',protected_previous_files=800,
        preserved_python_tests=195,new_python_tests=r['new_python_tests'],rust_tests_each_profile=644,
        doctests_each_profile=2,target_execution='none',**FLAGS)

def main():
    p=argparse.ArgumentParser();p.add_argument('--write',action='store_true');a=p.parse_args()
    if a.write:
        baseline();frozen();commands=[]
        for argv in COMMANDS:
            print('Checking: '+' '.join(argv),flush=True)
            r=subprocess.run(argv,cwd=ROOT,capture_output=True,text=True)
            require(r.returncode==0,r.stdout+r.stderr)
            commands.append(dict(argv=argv,returncode=r.returncode,stdout=r.stdout,stderr=r.stderr))
        r=build(commands);raw=gzip.compress(json.dumps(dict(stage='2BA',target_execution='none',commands=commands),sort_keys=True).encode(),mtime=0)
        r['logs_sha256']=digest(raw);(HERE/'validation-logs.json.gz').write_bytes(raw)
        RECORD.write_text(json.dumps(r,sort_keys=True,indent=2)+'\n')
    print(json.dumps(verify(),sort_keys=True))
if __name__=='__main__':main()
