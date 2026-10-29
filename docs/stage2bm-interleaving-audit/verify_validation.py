import argparse
import gzip
import json
import re
import subprocess
from common import ROOT,HERE,COMMIT,digest,encoded,load,require
from historical import verify as historical
from reference_replay import verify as reference
from interleaving_diff import verify as interleaving
from public_replay import verify as public
from verify_boundary import verify as boundary

PREFIX='docs/stage2bm-interleaving-audit/'
RECORD=ROOT/'docs/stage2bm-validation.json'
# Replay producers save their actual child commands and observations separately;
# these report commands recompute from those saved observations without claiming
# to execute the Rust drivers a second time.
COMMANDS=[['python3','-B',PREFIX+'historical.py'],['cargo','test','--locked','--offline'],['cargo','test','--release','--locked','--offline'],['cargo','build','--locked','--offline'],['cargo','build','--release','--locked','--offline'],['cargo','fmt','--check'],['cargo','clippy','--locked','--offline','--all-targets','--','-D','warnings'],['python3','-B',PREFIX+'reference_replay.py'],['python3','-B',PREFIX+'interleaving_diff.py'],['python3','-B',PREFIX+'public_replay.py'],['python3','-B',PREFIX+'verify_boundary.py'],['rustc','-Vv'],['rustup','target','list','--installed'],['python3','-B','-m','unittest','discover','-s',PREFIX.rstrip('/'),'-p','test_*.py']]
OLD_IGNORED={'kernel::tests::replay_model_traces','lifecycle_adapter::tests::replay_adapter_traces','vm::lifecycle_vm_tests::replay_lifecycle_vm_traces','boundary::lifecycle_boundary_tests::replay_host_lifecycle_traces','boundary::lifecycle_managed_tests::replay_managed_lifecycle_traces','boundary::lifecycle_host_function_tests::replay_host_function_traces','boundary::lifecycle_contract_tests::replay_production_contract_traces','replay_public_contract_traces','boundary::lifecycle_combination_tests::replay_combination_batches','boundary::lifecycle_combination_tests::production::replay_instrumented_combinations','replay_public_combinations','boundary::lifecycle_replacement_tests::replay_replacement_traces','replay_public_replacements'}
IGNORED=OLD_IGNORED|{'boundary::lifecycle_interleaving_tests::replay_interleaving_traces','replay_public_interleavings'}
PYTHON_TESTS=38

def expected_tests():
    return set(load(ROOT/'docs/stage2bl-validation.json')['current_test_names'])|set(load(HERE/'case-coverage.json')['new_test_names'])

def validate_rust(out):
    passed=re.findall(r'^test (\S+) \.\.\. ok$',out,re.M)
    require(len(passed)==len(set(passed))==878 and set(passed)==expected_tests(),'Rust scenario inventory')
    ignored=re.findall(r'^test (\S+) \.\.\. ignored(?:,.*)?$',out,re.M)
    require(len(ignored)==len(set(ignored))==15 and set(ignored)==IGNORED,'ignored drivers')
    docs=re.findall(r'^test (.+ \(line \d+\).+) \.\.\. ok$',out,re.M)
    require(len(docs)==len(set(docs))==5 and sum('boundary::HostValue' in n for n in docs)==2 and sum('lifecycle::LifecycleLimits' in n for n in docs)==1 and sum('lifecycle::LifecycleBudget' in n for n in docs)==2,'doctest inventory')
    results=re.findall(r'test result: ok\. (\d+) passed; (\d+) failed; (\d+) ignored;',out)
    require(results and sum(int(n) for n,_,_ in results)==883 and all(int(f)==0 for _,f,_ in results) and sum(int(i) for _,_,i in results)==15,'Rust results')
    return sorted(passed)

def validate_commands(commands):
    require([c['argv'] for c in commands]==COMMANDS,'command inventory')
    require(all(c['returncode']==0 and isinstance(c['stdout'],str) and isinstance(c['stderr'],str) for c in commands),'failed command')
    reports={}
    for i,key,verify in [(0,'historical',historical),(7,'reference',reference),(8,'interleaving',interleaving),(9,'public',public),(10,'boundary',boundary)]:
        value=verify();require(json.loads(commands[i]['stdout'])==value,'command report '+key);reports[key]=value
    names=validate_rust(commands[1]['stdout']+commands[1]['stderr'])
    require(names==validate_rust(commands[2]['stdout']+commands[2]['stderr']),'profile inventory')
    require('host: aarch64-apple-darwin' in commands[11]['stdout'] and commands[12]['stdout'].splitlines()==['aarch64-apple-darwin'],'target evidence')
    out=commands[-1]['stdout']+commands[-1]['stderr']
    require(re.findall(r'^Ran (\d+) tests? in ',out,re.M)==[str(PYTHON_TESTS)] and out.rstrip().endswith('OK'),'Python test inventory')
    return names,reports

def artifacts():
    b=load(HERE/'baseline-inputs.json')
    paths={n for n in b['allowed_changes']+b['allowed_prefixes'] if digest((ROOT/n).read_bytes())!=b['files'][n]}|set(b['new_source'])
    paths|={str(p.relative_to(ROOT)) for p in HERE.rglob('*') if p.is_file() and '__pycache__' not in p.parts and p.name!='validation-log.json.gz'}
    return {n:digest((ROOT/n).read_bytes()) for n in sorted(paths)}

def build(commands):
    names,reports=validate_commands(commands);g=load(HERE/'gate-review.json')
    require(all(g[k] is False for k in ('runtime_changed','public_api_changed','dependencies_changed','quota_semantics_changed','existing_models_changed','new_native_analysis','direct_owner_capture_ready','implementation_ready','stage2_complete','full_model_equivalence','native_equivalence_proven')) and g['new_production_deviations']==0 and g['native_unknowns']==20 and g['native_unknowns_promoted']==0,'gate scope')
    return dict(stage='2BM',status='pass',baseline_commit=COMMIT,baseline_files=1166,byte_frozen_files=reports['boundary']['byte_frozen_files'],current_test_names=names,old_rust_tests=869,new_rust_tests=9,rust_tests_debug=878,rust_tests_release=878,doctests_each_profile=5,ignored_drivers_each_profile=15,current_python_tests=PYTHON_TESTS,historical_python_tests=557,historical_run_on_current_source=False,quota_model_cases=39,arithmetic_rows=112,gates=g,**reports,reused_corpus=dict(path='docs/stage2aw-evidence/corpus-validation.json',sha256=digest((ROOT/'docs/stage2aw-evidence/corpus-validation.json').read_bytes()),rerun=False),artifact_sha256=artifacts())

def validate_record(record,raw):
    log=json.loads(gzip.decompress(raw))
    require(log['stage']=='2BM' and log['target_execution']=='none' and record['logs_sha256']==digest(raw),'validation log identity')
    require({k:v for k,v in record.items() if k!='logs_sha256'}==build(log['commands']),'receipt mismatch')

def verify():
    r=load(RECORD);validate_record(r,(HERE/'validation-log.json.gz').read_bytes())
    return {k:r[k] for k in ('stage','status','baseline_files','byte_frozen_files','rust_tests_debug','rust_tests_release','new_rust_tests','doctests_each_profile','current_python_tests','historical_python_tests','gates')}

if __name__=='__main__':
    p=argparse.ArgumentParser();p.add_argument('--write',action='store_true');args=p.parse_args()
    if args.write:
        boundary();commands=[]
        for argv in COMMANDS:
            print('Checking: '+' '.join(argv),flush=True)
            r=subprocess.run(argv,cwd=ROOT,text=True,capture_output=True);require(r.returncode==0,r.stdout+r.stderr)
            commands.append(dict(argv=argv,returncode=r.returncode,stdout=r.stdout,stderr=r.stderr))
        record=build(commands);raw=gzip.compress(encoded(dict(stage='2BM',target_execution='none',commands=commands)),mtime=0);record['logs_sha256']=digest(raw)
        (HERE/'validation-log.json.gz').write_bytes(raw);RECORD.write_bytes(encoded(record))
    print(json.dumps(verify(),sort_keys=True))
