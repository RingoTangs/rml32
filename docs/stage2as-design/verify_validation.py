"""Run/freeze 2AS design checks with --write; protect runtime and frozen history."""
import argparse
import gzip
import importlib.util
import json
import re
import subprocess

from verify_design import ROOT, HERE, safe_path, digest, canonical, require, build, check_outputs

ENTRY={'README.md','docs/magic_Linux32-rust-rewrite-roadmap.md'}
CORPUS='docs/stage2aq-evidence/corpus-validation.json'
COMMANDS=[
 ['python3','-B','docs/stage2ar-evidence/verify_evidence.py'],
 ['python3','-B','docs/stage2as-design/explore.py'],
 ['python3','-B','docs/stage2as-design/verify_design.py'],
 ['python3','-B','-m','unittest','discover','-s','docs/stage2as-design','-p','test_*.py'],
 ['cargo','test','--locked','--offline'],
 ['cargo','test','--release','--locked','--offline'],
 ['cargo','fmt','--check'],
 ['cargo','clippy','--locked','--offline','--all-targets','--','-D','warnings'],
]


def load(root,name):
    raw=safe_path(root,name).read_bytes()
    return json.loads(gzip.decompress(raw) if name.endswith('.gz') else raw)


def python_test_count(root=ROOT):
    return sum(len(re.findall(r'^    def test_\w+\(',p.read_text(),re.M)) for p in (root/'docs/stage2as-design').glob('test_*.py'))


def check_hashes(root,entries):
    for name,expected in entries.items():require(digest(safe_path(root,name).read_bytes())==expected,'protected input changed: '+name)


def check_entries(baseline,prior):
    require(set(baseline['before_entry_sha256'])==ENTRY,'entry inventory differs')
    require(all(prior['artifact_sha256'].get(p)==h for p,h in baseline['before_entry_sha256'].items()),'entry baseline differs')


def check_baseline(root=ROOT):
    baseline=load(root,'docs/stage2as-design/baseline-inputs.json')
    require(baseline['stage']=='2AS' and baseline['baseline_stage']=='2AR','baseline stage differs')
    expected=dict(status='pass',stage='2AR',anchors=85,delivery_files=28,expected_cases=51,functions=40,
                  implementation_ready=False,new_records=3,protected_files=601,python_tests=37,reused_records=50,
                  rust_tests_debug=558,rust_tests_release=558,selected_next='2AS-ownership-design',stage2_complete=False,target_execution='none')
    require(baseline['baseline_verification']==expected,'baseline verification differs')
    protected=baseline['protected_files'];require(len(protected)==628,'protected inventory size differs')
    prior=load(root,'docs/stage2ar-validation.json');check_entries(baseline,prior)
    required={str(p.relative_to(root)) for d in ('src','tests','docs') for p in (root/d).rglob('*')
              if p.is_file() and '__pycache__' not in p.parts and 'stage2as-design' not in p.parts}
    required|={'Cargo.toml','Cargo.lock','magic_Linux32','AGENTS.md'}
    required|={e['path'] for e in load(root,'docs/compatibility-evidence/input-manifest.json')}
    required-=ENTRY|{'docs/stage2as-validation.json'}
    require(set(protected)==required,'protected inventory differs')
    require(all(protected.get(p)==h for p,h in prior['artifact_sha256'].items() if p not in ENTRY),'prior 2AR delivery protection differs')
    check_hashes(root,protected)
    for name,old in baseline['before_entry_sha256'].items():
        lines=safe_path(root,name).read_text().splitlines(keepends=True)
        require(len(lines)>4 and lines[1].strip()=='' and '2AS' in lines[2] and lines[3].strip()=='','entry progress prefix differs')
        require(digest(''.join(lines[:2]+lines[4:]).encode())==old,'historical entry content changed')
    corpus=load(root,CORPUS)
    require(corpus['status']=='pass' and corpus['target_execution']=='none' and corpus['originals_unchanged']==36
            and corpus['known_errors']==['dba/pack_data/etc.pak/etc']
            and corpus['checked']==dict(packages=12,members=9527,programs=7159,inheritance=9933),'reused corpus boundary differs')
    return baseline


def delivery_files(root=ROOT):return {str(p.relative_to(root)) for p in (root/'docs/stage2as-design').iterdir() if p.is_file()}|ENTRY


def check_inventory(expected,actual):require(set(expected)==set(actual),'delivery inventory differs')


def passed_tests(output):return set(re.findall(r'^test (\S+) \.\.\. ok$',output,re.M))


def original_tests(root=ROOT):
    logs=load(root,'docs/stage2ar-evidence/validation-logs.json.gz')
    c=next(c for c in logs['commands'] if c['argv']==['cargo','test','--locked','--offline'])
    names=passed_tests(c['stdout']+c['stderr']);require(len(names)==558,'original test inventory differs');return names


def validate_commands(commands,names,python_tests):
    require([c['argv'] for c in commands]==COMMANDS,'command inventory differs')
    require(len(names)==558,'original test inventory differs')
    for c in commands:
        require(c['returncode']==0,'failed check')
        output=c['stdout']+c['stderr']
        if c['argv'][:2]==['cargo','test']:
            counts=re.findall(r'test result: ok\. (\d+) passed; (\d+) failed;',output)
            require(counts and sum(int(n) for n,_ in counts)==558 and all(int(n)==0 for _,n in counts),'Rust results differ')
            require(passed_tests(output)==names,'Rust test inventory differs')
        if 'unittest' in c['argv']:require(re.search(rf'Ran {python_tests} tests\b',output) and output.rstrip().endswith('OK'),'Python results differ')


def native_verify(root):
    spec=importlib.util.spec_from_file_location('_2as_2ar_evidence',root/'docs/stage2ar-evidence/verify_evidence.py')
    module=importlib.util.module_from_spec(spec);spec.loader.exec_module(module)
    return module.verify(root)


def verify(root=ROOT):
    baseline=check_baseline(root);record=load(root,'docs/stage2as-validation.json')
    expected=dict(stage='2AS',runtime_changed=False,public_api_changed=False,dependencies_changed=False,new_native_analysis=False,
                  target_execution='none',non_scalar_ready=False,stage2_complete=False,proposed_api_migration=True,
                  selected_next='2AT-host-boundary',rust_tests_debug=558,rust_tests_release=558,python_tests=python_test_count(root))
    require(all(record.get(k)==v for k,v in expected.items()),'receipt boundary differs')
    check_inventory(record['artifact_sha256'],delivery_files(root));check_hashes(root,record['artifact_sha256'])
    raw=safe_path(root,'docs/stage2as-design/validation-logs.json.gz').read_bytes()
    require(digest(raw)==record['logs_sha256'],'validation logs changed')
    commands=json.loads(gzip.decompress(raw))['commands'];validate_commands(commands,original_tests(root),python_test_count(root))
    outputs,summary=build(root);check_outputs(outputs,root/'docs/stage2as-design')
    enumeration=load(root,'docs/stage2as-design/enumeration.json')
    require(json.loads(commands[0]['stdout'])==native_verify(root),'frozen evidence result differs')
    require(json.loads(commands[1]['stdout'])==enumeration,'recorded enumeration differs')
    require(json.loads(commands[2]['stdout'])=={k:v for k,v in summary.items() if k!='source_sha256'},'recorded design result differs')
    require(record['corpus_validation']==dict(mode='reused',source=CORPUS,sha256=digest(safe_path(root,CORPUS).read_bytes())),'corpus reuse attribution differs')
    require(enumeration['complete_for_declared_profiles'] is True,'default bounded profiles incomplete')
    return dict(status='pass',stage='2AS',protected_files=len(baseline['protected_files']),delivery_files=len(delivery_files(root)),
                rust_tests_debug=558,rust_tests_release=558,python_tests=python_test_count(root),
                decisions=summary['decisions'],api_boundaries=summary['api_boundaries'],mapped_cases=summary['mapped_cases'],
                model_states=summary['enumeration_states'],model_accepted_transitions=summary['enumeration_accepted'],
                coverage=summary['coverage'],non_scalar_ready=False,stage2_complete=False,target_execution='none',selected_next='2AT-host-boundary')


def main():
    p=argparse.ArgumentParser(description=__doc__);p.add_argument('--write',action='store_true');args=p.parse_args()
    if args.write:
        check_baseline();commands=[]
        for argv in COMMANDS:
            print('Checking: '+' '.join(argv),flush=True)
            r=subprocess.run(argv,cwd=ROOT,capture_output=True,text=True)
            commands.append(dict(argv=argv,returncode=r.returncode,stdout=r.stdout,stderr=r.stderr))
            if r.returncode:raise RuntimeError(r.stdout+r.stderr)
        validate_commands(commands,original_tests(),python_test_count());check_baseline()
        logs=gzip.compress(canonical(dict(commands=commands)),mtime=0)
        (HERE/'validation-logs.json.gz').write_bytes(logs)
        record=dict(stage='2AS',runtime_changed=False,public_api_changed=False,dependencies_changed=False,new_native_analysis=False,
                    target_execution='none',non_scalar_ready=False,stage2_complete=False,proposed_api_migration=True,selected_next='2AT-host-boundary',
                    rust_tests_debug=558,rust_tests_release=558,python_tests=python_test_count(),logs_sha256=digest(logs),
                    corpus_validation=dict(mode='reused',source=CORPUS,sha256=digest((ROOT/CORPUS).read_bytes())),
                    artifact_sha256={p:digest((ROOT/p).read_bytes()) for p in sorted(delivery_files())})
        (ROOT/'docs/stage2as-validation.json').write_text(json.dumps(record,ensure_ascii=False,sort_keys=True,indent=2)+'\n')
    print(json.dumps(verify(),ensure_ascii=False,sort_keys=True))


if __name__=='__main__':main()
