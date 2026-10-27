"""Current test-only managed path, frozen production projection and scenario inventory."""
from collections import Counter
import re,subprocess,json
from managed_diff import ROOT,HERE,COMMIT,digest,load,require,cases,verify as managed
from source_projection import production,test_bodies
HOOKS={'src/boundary.rs','src/object.rs','src/vm.rs','src/lifecycle_runtime.rs'}
PREFIX={'README.md','docs/magic_Linux32-rust-rewrite-roadmap.md'}
NEW={'src/lifecycle_managed_tests.rs'}
FLAGS=dict(runtime_changed=False,public_api_changed=False,dependencies_changed=False,model_changed=False,new_native_analysis=False,rust_source_changed=True,
 test_only_managed_lifecycle_added=True,managed_task_path_validated=True,task_parameter_ownership_validated=True,deferred_task_cleanup_policy=True,managed_delete_aborts_next_positive_step=True,
 host_registry_path_validated=True,closed_store_storage_retention_validated=True,direct_host_callbacks_validated=True,host_raw_transport_policy=True,container_capture_ready=True,direct_owner_capture_ready=False,non_scalar_ready=False,implementation_ready=False,stage2_complete=False,historical_python_tests_run_on_current_source=False)
def baseline():
    b=load(HERE/'baseline-inputs.json');require(b['stage']=='2BF' and b['baseline_stage']=='2BE' and b['baseline_commit']==COMMIT,'baseline identity');paths=subprocess.check_output(['git','ls-tree','-r','--name-only',COMMIT],cwd=ROOT).decode().splitlines();require(len(paths)==921 and set(paths)==set(b['files']) and set(b['allowed_test_hooks'])==HOOKS,'baseline inventory')
    for p,h in b['files'].items():
        raw=(ROOT/p).read_bytes()
        if p in HOOKS|PREFIX:
            old=subprocess.check_output(['git','show',COMMIT+':'+p],cwd=ROOT);require(digest(old)==h,'baseline source '+p)
            if p in PREFIX:require(raw.endswith(old),'historical suffix '+p)
            else:
                if p!='src/lifecycle_runtime.rs':require(production(old.decode())==production(raw.decode()),'production projection '+p)
                if p=='src/boundary.rs':
                    blocks=lambda text:re.findall(r'/// ```compile_fail\n(.*?)/// ```',text,re.S)
                    require(len(blocks(old.decode()))==2 and blocks(old.decode())==blocks(raw.decode()),'legacy doctest bodies')
                before=Counter(tuple(t)for t in test_bodies(old.decode()));after=Counter(tuple(t)for t in test_bodies(raw.decode()));require(not(before-after),'legacy test body '+p)
        else:require(digest(raw)==h,'frozen input changed '+p)
    require('#[cfg(test)]\nmod lifecycle_adapter;'in(ROOT/'src/lib.rs').read_text(),'adapter cfg gate');require('#[path = "lifecycle_runtime.rs"]'in(ROOT/'src/lifecycle_adapter.rs').read_text(),'runtime path');require('#[cfg(test)]\n#[path = "lifecycle_managed_tests.rs"]\nmod lifecycle_managed_tests;'in(ROOT/'src/boundary.rs').read_text(),'managed fixture gate')
    rust={p for p in subprocess.check_output(['rg','--files','src','tests'],cwd=ROOT).decode().splitlines()if p.endswith('.rs')};old={p for p in paths if p.endswith('.rs')and p.startswith(('src/','tests/'))};require(rust==old|NEW,'Rust file inventory');return b

def inventory():
    raw=(ROOT/'src/lifecycle_managed_tests.rs').read_bytes();s=raw.decode();ordinary=sorted('boundary::lifecycle_managed_tests::'+n for n in re.findall(r'#\[test\]\s+fn (\w+)\(',s));ignored=['boundary::lifecycle_managed_tests::'+n for n in re.findall(r'#\[test\]\s+#\[ignore[^\n]*\]\s+fn (\w+)\(',s)];require(len(ordinary)==len(set(ordinary)) and len(ordinary)>=26 and ignored==['boundary::lifecycle_managed_tests::replay_managed_lifecycle_traces'],'managed fixture inventory');return dict(source_sha256=digest(raw),ordinary=ordinary,ignored_driver=ignored[0])
def coverage():
    r=load(HERE/'case-coverage.json');prior=ROOT/'docs/stage2be-host-test/case-coverage.json';require(r['prior_sha256']==digest(prior.read_bytes()) and r['prior_coverage']==load(prior),'case prefix');require(len(r['prior_coverage']['prior_cases'])==115 and r['native_unknowns']==20 and r['native_unknowns_promoted']==0,'native scope');tests=set(inventory()['ordinary']);require(r['rust_policy_only'] is True and all(set(g['tests'])<=tests and g['tests']for g in r['managed_groups']),'managed evidence mapping');require(set().union(*(set(g['tests'])for g in r['managed_groups']))==tests,'scenario group coverage');return dict(cases=115,native_unknowns=20,managed_groups=len(r['managed_groups']))
def verify():
    baseline();i=inventory();require(load(HERE/'test-inventory.json')==i,'test inventory stale');require(load(HERE/'trace-manifest.json')['cases']==json.loads(json.dumps(cases())),'timeline manifest');c=coverage();r=managed();return dict(stage='2BF',status='pass',baseline_files=921,byte_frozen_files=915,test_hook_files=sorted(HOOKS),ordinary_managed_tests=len(i['ordinary']),managed_traces=r['replay']['traces'],managed_steps=r['replay']['steps'],frozen_model_traces=r['replay']['frozen_model_traces'],target_execution='none',**c,**FLAGS)
if __name__=='__main__':print(json.dumps(verify(),sort_keys=True))
