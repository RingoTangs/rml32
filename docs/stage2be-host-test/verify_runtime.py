"""Freeze baseline, production tokens, legacy scenarios and capability gates."""
from collections import Counter
import json,re,subprocess
from host_diff import ROOT,HERE,COMMIT,digest,load,require,cases,verify as host
from source_projection import production,test_bodies
HOOKS={'src/boundary.rs','src/object.rs','src/vm.rs','src/lifecycle_adapter.rs','src/lifecycle_runtime.rs'}
PREFIX={'README.md','docs/magic_Linux32-rust-rewrite-roadmap.md'}
NEW={'src/lifecycle_boundary_tests.rs'}
FLAGS=dict(runtime_changed=False,public_api_changed=False,dependencies_changed=False,model_changed=False,new_native_analysis=False,rust_source_changed=True,
 test_only_host_boundary_lifecycle_added=True,host_registry_path_validated=True,closed_store_storage_retention_validated=True,direct_host_callbacks_validated=True,
 host_raw_transport_policy=True,container_capture_ready=True,direct_owner_capture_ready=False,non_scalar_ready=False,implementation_ready=False,stage2_complete=False,historical_python_tests_run_on_current_source=False)
def baseline():
    b=load(HERE/'baseline-inputs.json');require(b['stage']=='2BE' and b['baseline_stage']=='2BD' and b['baseline_commit']==COMMIT,'baseline identity')
    paths=subprocess.check_output(['git','ls-tree','-r','--name-only',COMMIT],cwd=ROOT).decode().splitlines();require(len(paths)==893 and set(paths)==set(b['files']) and set(b['allowed_test_hooks'])==HOOKS,'baseline inventory')
    for p,h in b['files'].items():
        raw=(ROOT/p).read_bytes()
        if p in HOOKS|PREFIX:
            old=subprocess.check_output(['git','show',COMMIT+':'+p],cwd=ROOT);require(digest(old)==h,'baseline source '+p)
            if p in PREFIX:require(raw.endswith(old),'historical suffix '+p)
            else:
                if p not in {'src/lifecycle_adapter.rs','src/lifecycle_runtime.rs'}:require(production(old.decode())==production(raw.decode()),'production projection '+p)
                before=Counter(tuple(t)for t in test_bodies(old.decode()));after=Counter(tuple(t)for t in test_bodies(raw.decode()));require(not(before-after),'legacy test body '+p)
        else:require(digest(raw)==h,'frozen input changed '+p)
    require('#[cfg(test)]\nmod lifecycle_adapter;'in(ROOT/'src/lib.rs').read_text(),'adapter cfg gate')
    require('#[path = "lifecycle_runtime.rs"]'in(ROOT/'src/lifecycle_adapter.rs').read_text(),'runtime path')
    require('#[cfg(test)]\n#[path = "lifecycle_boundary_tests.rs"]\nmod lifecycle_boundary_tests;'in(ROOT/'src/boundary.rs').read_text(),'Host fixture gate')
    rust={p for p in subprocess.check_output(['rg','--files','src','tests'],cwd=ROOT).decode().splitlines()if p.endswith('.rs')};old={p for p in paths if p.endswith('.rs')and p.startswith(('src/','tests/'))};require(rust==old|NEW,'Rust file inventory')
    return b

def inventory():
    raw=(ROOT/'src/lifecycle_boundary_tests.rs').read_bytes();s=raw.decode();observed=sorted('boundary::lifecycle_boundary_tests::'+n for n in re.findall(r'#\[test\]\s+fn (\w+)\(',s));ignored=['boundary::lifecycle_boundary_tests::'+n for n in re.findall(r'#\[test\]\s+#\[ignore[^\n]*\]\s+fn (\w+)\(',s)]
    require(len(observed)==len(set(observed)) and len(observed)>=30 and ignored==['boundary::lifecycle_boundary_tests::replay_host_lifecycle_traces'],'Host fixture inventory')
    return dict(source_sha256=digest(raw),ordinary=observed,ignored_driver=ignored[0])
def coverage():
    r=load(HERE/'case-coverage.json');prior=load(ROOT/'docs/stage2bd-runtime-test/case-coverage.json');require(r['prior_sha256']==digest((ROOT/'docs/stage2bd-runtime-test/case-coverage.json').read_bytes()) and r['prior_cases']==prior['cases'],'case prefix')
    require(len(r['prior_cases'])==115 and r['native_unknowns']==20 and r['native_unknowns_promoted']==0,'native scope')
    known=set(inventory()['ordinary']);require(r['rust_policy_only'] is True and all(set(x['tests'])<=known and x['tests']for x in r['host_groups']),'Host evidence mapping')
    return dict(cases=115,native_unknowns=20,host_groups=len(r['host_groups']))
def verify():
    baseline();i=inventory();require(load(HERE/'test-inventory.json')==i,'test inventory stale');require(load(HERE/'trace-manifest.json')['cases']==json.loads(json.dumps(cases())),'trace manifest');c=coverage();r=host()
    return dict(stage='2BE',status='pass',baseline_files=893,byte_frozen_files=886,test_hook_files=sorted(HOOKS),ordinary_host_tests=len(i['ordinary']),host_traces=r['replay']['traces'],host_steps=r['replay']['steps'],target_execution='none',**c,**FLAGS)
if __name__=='__main__':print(json.dumps(verify(),sort_keys=True))
