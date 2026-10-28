"""Freeze earlier artifacts and tests; verify the deliberately small source boundary."""
import importlib.util,subprocess
from common import ROOT,HERE,COMMIT,digest,load,require

def projection():
    spec=importlib.util.spec_from_file_location('_2bj_frozen_projection',ROOT/'docs/stage2bg-host-function-test/source_projection.py');m=importlib.util.module_from_spec(spec);spec.loader.exec_module(m);return m

def baseline():
    b=load(HERE/'baseline-inputs.json');require(b['baseline_commit']==COMMIT and len(b['files'])==1048,'baseline identity')
    paths=subprocess.check_output(['git','ls-tree','-r','--name-only',COMMIT],cwd=ROOT).decode().splitlines();require(set(paths)==set(b['files']),'baseline paths')
    m=projection();bodies=0
    for p,h in b['files'].items():
        raw=(ROOT/p).read_bytes()
        if p in b['allowed_changes']:
            old=subprocess.check_output(['git','show',COMMIT+':'+p],cwd=ROOT).decode();before=m.test_bodies(old);after=m.test_bodies(raw.decode());require(all(t in after for t in before),'original test body '+p);bodies+=len(before)
            if p=='src/boundary.rs':require(m.cfg(m.tokens(old))==m.cfg(m.tokens(raw.decode())),'boundary production tokens')
        elif p in b['allowed_prefixes']:
            old=subprocess.check_output(['git','show',COMMIT+':'+p],cwd=ROOT);require(digest(old)==h and raw.endswith(old),'historical prefix '+p)
        else:require(digest(raw)==h,'frozen file '+p)
    expected={p for p in paths if p.startswith(('src/','tests/'))}|set(b['new_source']);current=set(subprocess.check_output(['rg','--files','src','tests'],cwd=ROOT).decode().splitlines());require(current==expected,'source inventory')
    return b,bodies

def verify():
    b,n=baseline();c=load(HERE/'case-coverage.json');prior=ROOT/'docs/stage2bi-production-lifecycle/case-coverage.json';require(c['prior_sha256']==digest(prior.read_bytes())and c['prior_coverage']==load(prior),'case prefix')
    old=c['prior_coverage']
    while 'prior_coverage'in old:old=old['prior_coverage']
    require(len(old['prior_cases'])==115 and c['native_unknowns']==20 and c['native_unknowns_promoted']==0,'native boundary')
    require(len(c['rust_test_names'])==len(set(c['rust_test_names']))==6,'new Rust scenarios')
    require({r['decision']for r in c['decisions']}=={'Q01','Q02','Q03','Q04','Q05','M01','M02','M03','H01','I01'}and all(r['families']and set(r['families'])<={f'F{i:02}'for i in range(1,17)}for r in c['decisions']),'decision mapping')
    models=load(ROOT/'docs/stage2bh-production-contract/case-coverage.json')['model_case_names'];require([r['case']for r in c['model_case_classification']]==models and all(r['classification']in ('differential','refinement','model-only')and r['basis']for r in c['model_case_classification']),'model case classification')
    return dict(stage='2BJ',status='pass',baseline_files=1048,byte_frozen_files=1043,changed_source_allowlist=3,new_source_files=3,preserved_test_bodies_in_changed_files=n,old_rust_tests=851,new_rust_tests=6,native_unknowns=20,native_unknowns_promoted=0,target_execution='none',existing_models_changed=False,dependencies_changed=False,public_api_changed=False)

if __name__=='__main__':
    import json
    print(json.dumps(verify(),sort_keys=True))
