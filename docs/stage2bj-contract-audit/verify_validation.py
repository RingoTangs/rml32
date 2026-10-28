"""Actual commands, frozen scenario inventory, independent replay reports and hashes."""
import argparse,gzip,json,re,subprocess
from common import ROOT,HERE,digest,load,require,encoded
from verify_boundary import baseline,verify as boundary
from historical import verify as historical
from reference_replay import verify as reference
from model_replay import verify as model
from contract_diff import verify as contract
from public_diff import verify as public
from baseline_proof import verify as proof

RECORD=ROOT/'docs/stage2bj-validation.json'
PREFIX='docs/stage2bj-contract-audit/'
COMMANDS=[['python3','-B',PREFIX+'historical.py','--write'],
 ['cargo','test','--locked','--offline'],['cargo','test','--release','--locked','--offline'],
 ['cargo','build','--locked','--offline'],['cargo','build','--release','--locked','--offline'],
 ['cargo','fmt','--check'],['cargo','clippy','--locked','--offline','--all-targets','--','-D','warnings'],
 ['python3','-B',PREFIX+'model_replay.py','--write'],['python3','-B',PREFIX+'reference_replay.py','--write'],
 ['python3','-B',PREFIX+'contract_diff.py','--write'],['python3','-B',PREFIX+'public_diff.py','--write'],
 ['python3','-B',PREFIX+'baseline_proof.py','--write'],['python3','-B',PREFIX+'verify_boundary.py'],
 ['rustc','-Vv'],['rustup','target','list','--installed'],
 ['python3','-B','-m','unittest','discover','-s',PREFIX.rstrip('/'),'-p','test_*.py']]
IGNORED={'kernel::tests::replay_model_traces','lifecycle_adapter::tests::replay_adapter_traces','vm::lifecycle_vm_tests::replay_lifecycle_vm_traces','boundary::lifecycle_boundary_tests::replay_host_lifecycle_traces','boundary::lifecycle_managed_tests::replay_managed_lifecycle_traces','boundary::lifecycle_host_function_tests::replay_host_function_traces','boundary::lifecycle_contract_tests::replay_production_contract_traces','replay_public_contract_traces'}
CURRENT_PYTHON_TESTS=32

def expected_tests():return set(load(ROOT/'docs/stage2bi-validation.json')['current_test_names'])|set(load(HERE/'case-coverage.json')['rust_test_names'])

def validate_rust(out):
    passed=re.findall(r'^test (\S+) \.\.\. ok$',out,re.M);require(len(passed)==len(set(passed))==857 and set(passed)==expected_tests(),'Rust scenario inventory')
    ignored=re.findall(r'^test (\S+) \.\.\. ignored(?:,.*)?$',out,re.M);require(len(ignored)==len(set(ignored))==8 and set(ignored)==IGNORED,'ignored driver inventory')
    docs=re.findall(r'^test (.+ \(line \d+\).+) \.\.\. ok$',out,re.M);require(len(docs)==len(set(docs))==5 and sum('boundary::HostValue'in n for n in docs)==2 and sum('lifecycle::LifecycleLimits'in n for n in docs)==1 and sum('lifecycle::LifecycleBudget'in n for n in docs)==2,'doctest inventory')
    results=re.findall(r'test result: ok\. (\d+) passed; (\d+) failed; (\d+) ignored;',out);require(results and sum(int(n)for n,_,_ in results)==862 and all(int(f)==0 for _,f,_ in results)and sum(int(i)for _,_,i in results)==8,'Rust results')
    return sorted(passed)

def validate_commands(commands):
    require([c['argv']for c in commands]==COMMANDS,'command inventory');require(all(c['returncode']==0 and isinstance(c['stdout'],str)and isinstance(c['stderr'],str)for c in commands),'command failure')
    for i,verify in [(0,historical),(7,model),(8,reference),(9,contract),(10,public),(11,proof),(12,boundary)]:require(json.loads(commands[i]['stdout'])==verify(),'subreport result '+str(i))
    debug=validate_rust(commands[1]['stdout']+commands[1]['stderr']);release=validate_rust(commands[2]['stdout']+commands[2]['stderr']);require(debug==release,'profile scenario inventory')
    targets=load(HERE/'targets.json');require('host: '+targets['host_target']in commands[13]['stdout']and commands[14]['stdout'].splitlines()==targets['installed_targets'],'target identity')
    require(targets['rust32_execution']=='not_run'and targets['model_unsigned_widths']==[32,64]and targets['arithmetic_rows']==112,'target scope')
    out=commands[-1]['stdout']+commands[-1]['stderr'];require(re.findall(r'^Ran (\d+) tests? in ',out,re.M)==[str(CURRENT_PYTHON_TESTS)]and out.rstrip().endswith('OK'),'Python checks')
    return dict(current_test_names=debug,rust_tests_debug=857,rust_tests_release=857,old_rust_tests=851,new_rust_tests=6,doctests_each_profile=5,ignored_workers_each_profile=8,current_python_tests=CURRENT_PYTHON_TESTS,historical_python_tests=465,historical_python_tests_run_on_current_source=False,quota_model_cases_reexecuted=39,arithmetic_rows=112,contract_traces_each_profile=2592,public_traces_each_profile=972,rust32_execution='not_run')

def artifacts():
    b,_=baseline();paths=set(b['allowed_changes'])|set(b['new_source'])|set(b['allowed_prefixes'])|{str(p.relative_to(ROOT))for p in HERE.rglob('*')if p.is_file()and '__pycache__'not in p.parts and p.name!='validation-logs.json.gz'}
    return {p:digest((ROOT/p).read_bytes())for p in sorted(paths)}

def build(commands):
    tested=validate_commands(commands);scope=boundary();old,_=baseline();corpus='docs/stage2aw-evidence/corpus-validation.json';require(digest((ROOT/corpus).read_bytes())==old['files'][corpus],'corpus identity')
    gates=load(HERE/'gate-review.json');require(all(gates[k]is False for k in ('public_api_changed','dependencies_changed','existing_models_changed','quota_semantics_changed','direct_owner_capture_ready','non_scalar_ready','implementation_ready','stage2_complete','new_native_analysis','full_model_equivalence','native_equivalence_proven'))and gates['new_maintenance_reference_model_added']is True and gates['runtime_changed']is True and gates['native_unknowns']==20,'gate scope')
    return dict(stage='2BJ',status='pass',baseline_commit=old['baseline_commit'],baseline_files=1048,byte_frozen_files=1043,target_execution='none',synthetic_vm_execution=True,business_bytecode_execution=False,**{k:v for k,v in gates.items()if k not in ('stage','target_execution')},**tested,boundary_summary=scope,historical_summary=historical(),reference_summary=reference(),model_summary=model(),contract_summary=contract(),public_summary=public(),baseline_proof_summary=proof(),reused_corpus=dict(source=corpus,sha256=old['files'][corpus],rerun=False),artifact_sha256=artifacts())

def validate_record(record,raw):
    log=json.loads(gzip.decompress(raw));require(log['stage']=='2BJ'and log['target_execution']=='none'and record['logs_sha256']==digest(raw),'validation logs')
    require({k:v for k,v in record.items()if k!='logs_sha256'}==build(log['commands']),'receipt mismatch')

def verify():
    r=load(RECORD);validate_record(r,(HERE/'validation-logs.json.gz').read_bytes())
    return {k:r[k]for k in ('stage','status','baseline_files','byte_frozen_files','rust_tests_debug','rust_tests_release','new_rust_tests','doctests_each_profile','current_python_tests','historical_python_tests','contract_traces_each_profile','public_traces_each_profile','new_maintenance_reference_model_added','public_api_changed','implementation_ready','stage2_complete','rust32_execution','target_execution')}

def main():
    p=argparse.ArgumentParser();p.add_argument('--write',action='store_true');a=p.parse_args()
    if a.write:
        baseline();commands=[]
        for argv in COMMANDS:
            print('Checking: '+' '.join(argv),flush=True);r=subprocess.run(argv,cwd=ROOT,text=True,capture_output=True);require(r.returncode==0,r.stdout+r.stderr);commands.append(dict(argv=argv,returncode=r.returncode,stdout=r.stdout,stderr=r.stderr))
        result=build(commands);raw=gzip.compress(encoded(dict(stage='2BJ',target_execution='none',commands=commands)),mtime=0);result['logs_sha256']=digest(raw);(HERE/'validation-logs.json.gz').write_bytes(raw);RECORD.write_bytes(encoded(result))
    print(json.dumps(verify(),sort_keys=True))
if __name__=='__main__':main()
