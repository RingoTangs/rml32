"""Freeze current 2AR checks with --write; verify receipt without re-running analysis."""
import argparse
import gzip
import json
from pathlib import Path
import re
import subprocess

from verify_evidence import ROOT, HERE, safe_path, load, digest, canonical, require, verify as evidence_verify
from render import build, check_outputs

ENTRY={'README.md','docs/magic_Linux32-rust-rewrite-roadmap.md'}
REPORT='docs/vm-capture-ownership-semantics.md'
CORPUS='docs/stage2aq-evidence/corpus-validation.json'
COMMANDS=[
 ['python3','-B','docs/stage2ar-evidence/verify_evidence.py'],
 ['python3','-B','docs/stage2ar-evidence/render.py'],
 ['python3','-B','-m','unittest','discover','-s','docs/stage2ar-evidence','-p','test_*.py'],
 ['cargo','test','--locked','--offline'],
 ['cargo','test','--release','--locked','--offline'],
 ['cargo','fmt','--check'],
 ['cargo','clippy','--locked','--offline','--all-targets','--','-D','warnings'],
]


def python_test_count(root=ROOT):
    return sum(len(re.findall(r'^    def test_\w+\(',p.read_text(),re.M))
               for p in (root/'docs/stage2ar-evidence').glob('test_*.py'))


def check_hashes(root,entries):
    for name,expected in entries.items():
        require(digest(safe_path(root,name).read_bytes())==expected,'protected input changed: '+name)


def check_entry_baseline(baseline,prior):
    require(set(baseline['before_entry_sha256'])==ENTRY,'entry inventory mismatch')
    require(all(prior['artifact_sha256'].get(p)==h for p,h in baseline['before_entry_sha256'].items()),'entry baseline mismatch')


def check_baseline(root=ROOT):
    baseline=load(root,'docs/stage2ar-evidence/baseline-inputs.json')
    require(baseline['stage']=='2AR' and baseline['baseline_stage']=='2AQ','baseline stage mismatch')
    expected=dict(status='pass',stage='2AQ',protected_files=576,delivery_files=26,rust_tests_debug=558,
                  rust_tests_release=558,python_tests=55,gaps=24,capability_gates=13,selected_next='P05',
                  stage2_complete=False,target_execution='none')
    require(baseline['baseline_verification']==expected,'baseline verification mismatch')
    protected=baseline['protected_files']
    require(len(protected)==601,'protected inventory size mismatch')
    prior=load(root,'docs/stage2aq-validation.json');check_entry_baseline(baseline,prior)
    required={str(p.relative_to(root)) for d in ('src','tests','docs') for p in (root/d).rglob('*')
              if p.is_file() and '__pycache__' not in p.parts and 'stage2ar-evidence' not in p.parts}
    required|={'Cargo.toml','Cargo.lock','magic_Linux32','AGENTS.md'}
    required|={e['path'] for e in load(root,'docs/compatibility-evidence/input-manifest.json')}
    required-=ENTRY|{REPORT,'docs/stage2ar-validation.json'}
    require(set(protected)==required,'protected inventory mismatch')
    require(all(protected.get(p)==h for p,h in prior['artifact_sha256'].items() if p not in ENTRY),'2AQ delivery protection mismatch')
    check_hashes(root,protected)
    for name,old in baseline['before_entry_sha256'].items():
        lines=safe_path(root,name).read_text().splitlines(keepends=True)
        require(len(lines)>4 and lines[1].strip()=='' and '2AR' in lines[2] and lines[3].strip()=='','entry progress prefix differs')
        require(digest(''.join(lines[:2]+lines[4:]).encode())==old,'historical entry content changed')
    corpus=load(root,CORPUS)
    require(corpus['status']=='pass' and corpus['target_execution']=='none' and corpus['originals_unchanged']==36
            and corpus['known_errors']==['dba/pack_data/etc.pak/etc']
            and corpus['checked']==dict(packages=12,members=9527,programs=7159,inheritance=9933),'reused corpus boundary differs')
    return baseline


def delivery_files(root=ROOT):
    return {str(p.relative_to(root)) for p in (root/'docs/stage2ar-evidence').iterdir() if p.is_file()}|ENTRY|{REPORT}


def check_inventory(expected,actual):
    require(set(expected)==set(actual),'delivery inventory mismatch')


def passed_tests(output):
    return set(re.findall(r'^test (\S+) \.\.\. ok$',output,re.M))


def original_tests(root=ROOT):
    logs=load(root,'docs/stage2aq-evidence/validation-logs.json.gz')
    c=next(c for c in logs['commands'] if c['argv']==['cargo','test','--locked','--offline'])
    names=passed_tests(c['stdout']+c['stderr']);require(len(names)==558,'original test inventory differs');return names


def validate_commands(commands,names,python_tests):
    require([c['argv'] for c in commands]==COMMANDS,'command inventory mismatch')
    require(len(names)==558,'original test inventory differs')
    for c in commands:
        require(c['returncode']==0,'failed check')
        output=c['stdout']+c['stderr']
        if c['argv'][:2]==['cargo','test']:
            counts=re.findall(r'test result: ok\. (\d+) passed; (\d+) failed;',output)
            require(counts and sum(int(n) for n,_ in counts)==558 and all(int(n)==0 for _,n in counts),'Rust results differ')
            require(passed_tests(output)==names,'Rust test inventory differs')
        if 'unittest' in c['argv']:
            require(re.search(rf'Ran {python_tests} tests\b',output) and output.rstrip().endswith('OK'),'Python results differ')


def verify(root=ROOT):
    baseline=check_baseline(root)
    record=load(root,'docs/stage2ar-validation.json')
    expected=dict(stage='2AR',runtime_changed=False,public_api_changed=False,dependencies_changed=False,
                  rust_tests_debug=558,rust_tests_release=558,python_tests=python_test_count(root),
                  supported_opcodes=75,corpus_intersection=73,stage2_complete=False,new_native_analysis=True,
                  target_execution='none',implementation_ready=False,selected_next='2AS-ownership-design')
    require(all(record.get(k)==v for k,v in expected.items()),'receipt boundary mismatch')
    check_inventory(record['artifact_sha256'],delivery_files(root));check_hashes(root,record['artifact_sha256'])
    raw=safe_path(root,'docs/stage2ar-evidence/validation-logs.json.gz').read_bytes()
    require(digest(raw)==record['logs_sha256'],'validation logs changed')
    commands=json.loads(gzip.decompress(raw))['commands'];validate_commands(commands,original_tests(root),python_test_count(root))
    outputs,summary=build(root);check_outputs(outputs,root/'docs/stage2ar-evidence')
    require(json.loads(commands[0]['stdout'])==evidence_verify(root),'recorded evidence differs')
    require(json.loads(commands[1]['stdout'])=={k:v for k,v in summary.items() if k!='source_sha256'},'recorded summary differs')
    require(record['corpus_validation']==dict(mode='reused',source=CORPUS,sha256=digest(safe_path(root,CORPUS).read_bytes())),'reused corpus attribution differs')
    return dict(status='pass',stage='2AR',protected_files=len(baseline['protected_files']),delivery_files=len(delivery_files(root)),
                rust_tests_debug=558,rust_tests_release=558,python_tests=python_test_count(root),
                new_records=summary['new_records'],reused_records=summary['reused_records'],functions=summary['functions'],
                anchors=summary['anchors'],expected_cases=summary['expected_cases'],implementation_ready=False,
                selected_next='2AS-ownership-design',stage2_complete=False,target_execution='none')


def main():
    parser=argparse.ArgumentParser(description=__doc__);parser.add_argument('--write',action='store_true');args=parser.parse_args()
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
        record=dict(stage='2AR',runtime_changed=False,public_api_changed=False,dependencies_changed=False,
                    rust_tests_debug=558,rust_tests_release=558,python_tests=python_test_count(),supported_opcodes=75,
                    corpus_intersection=73,stage2_complete=False,new_native_analysis=True,target_execution='none',
                    implementation_ready=False,selected_next='2AS-ownership-design',logs_sha256=digest(logs),
                    corpus_validation=dict(mode='reused',source=CORPUS,sha256=digest((ROOT/CORPUS).read_bytes())),
                    artifact_sha256={p:digest((ROOT/p).read_bytes()) for p in sorted(delivery_files())})
        (ROOT/'docs/stage2ar-validation.json').write_text(json.dumps(record,ensure_ascii=False,sort_keys=True,indent=2)+'\n')
    print(json.dumps(verify(),ensure_ascii=False,sort_keys=True))


if __name__=='__main__':main()
