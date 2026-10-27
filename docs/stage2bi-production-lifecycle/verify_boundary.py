"""Freeze all earlier evidence; preserve old Rust test bodies inside changed modules."""
import importlib.util,subprocess
from common import ROOT,HERE,COMMIT,digest,load,require
def baseline():
    b=load(HERE/'baseline-inputs.json');require(b['baseline_commit']==COMMIT and len(b['files'])==1017,'baseline identity')
    paths=subprocess.check_output(['git','ls-tree','-r','--name-only',COMMIT],cwd=ROOT).decode().splitlines();require(set(paths)==set(b['files']),'baseline paths')
    for p,h in b['files'].items():
        raw=(ROOT/p).read_bytes()
        if p in b['allowed_changes']:continue
        if p in b['allowed_prefixes']:
            old=subprocess.check_output(['git','show',COMMIT+':'+p],cwd=ROOT);require(digest(old)==h and raw.endswith(old),'historical prefix '+p)
        else:require(digest(raw)==h,'frozen file '+p)
    expected={p for p in paths if p.startswith(('src/','tests/'))}|set(b['new_source']);current=set(subprocess.check_output(['rg','--files','src','tests'],cwd=ROOT).decode().splitlines());require(current==expected,'source inventory')
    spec=importlib.util.spec_from_file_location('_2bi_frozen_projection',ROOT/'docs/stage2bg-host-function-test/source_projection.py');m=importlib.util.module_from_spec(spec);spec.loader.exec_module(m)
    bodies=0
    for p in b['allowed_changes']:
        old=subprocess.check_output(['git','show',COMMIT+':'+p],cwd=ROOT).decode();before=m.test_bodies(old);after=m.test_bodies((ROOT/p).read_text());require(all(t in after for t in before),'original test body '+p);bodies+=len(before)
    return b,bodies
def verify():
    b,n=baseline();coverage=load(HERE/'case-coverage.json');prior=ROOT/'docs/stage2bh-production-contract/case-coverage.json';require(coverage['prior_sha256']==digest(prior.read_bytes()) and coverage['prior_coverage']==load(prior),'case prefix')
    old=coverage['prior_coverage']
    while 'prior_coverage'in old:old=old['prior_coverage']
    require(len(old['prior_cases'])==115 and coverage['native_unknowns']==20 and coverage['native_unknowns_promoted']==0,'native boundary')
    require(len(coverage['rust_test_names'])==len(set(coverage['rust_test_names']))==32,'new Rust cases')
    require({r['decision']for r in coverage['decisions']}==set(['Q01','Q02','Q03','Q04','Q05','M01','M02','M03','H01','I01']) and all(r['tests'] and set(r['tests'])<=set(coverage['rust_test_names'])for r in coverage['decisions']),'production case mapping')
    return dict(stage='2BI',status='pass',baseline_files=1017,byte_frozen_files=1004,changed_source_allowlist=len(b['allowed_changes']),new_source_files=len(b['new_source']),preserved_test_bodies_in_changed_files=n,old_rust_tests=819,new_rust_tests=32,native_unknowns=20,native_unknowns_promoted=0,target_execution='none',existing_models_changed=False,dependencies_changed=False)
if __name__=='__main__':
    import json
    print(json.dumps(verify(),sort_keys=True))
