"""Current source boundary and real-frame lifecycle delivery validator."""
from collections import Counter
import json,re,subprocess
from vm_diff import ROOT,HERE,digest,load,require,verify as vm,cases
from source_projection import production,test_bodies
COMMIT='1306a9bca296813f83df779fb62aa1a0f651d20c'
HOOKS={'src/lib.rs','src/cell.rs','src/container.rs','src/object.rs','src/vm.rs','src/lifecycle_adapter.rs'}
PREFIX={'README.md','docs/magic_Linux32-rust-rewrite-roadmap.md'}
NEW={'src/lifecycle_runtime.rs','src/lifecycle_vm_tests.rs'}
FLAGS=dict(runtime_changed=False,public_api_changed=False,dependencies_changed=False,rust_source_changed=True,
    test_only_store_frame_lifecycle_added=True,store_frame_path_validated=True,model_changed=False,new_native_analysis=False,
    container_capture_ready=True,direct_owner_capture_ready=False,non_scalar_ready=False,stage2_complete=False,
    implementation_ready=False,historical_python_tests_run_on_current_source=False)

def baseline():
    b=load(HERE/'baseline-inputs.json');require(b['stage']=='2BD' and b['baseline_commit']==COMMIT and b['baseline_stage']=='2BC','baseline identity')
    paths=subprocess.check_output(['git','ls-tree','-r','--name-only',COMMIT],cwd=ROOT).decode().splitlines();require(len(paths)==865 and set(paths)==set(b['files']) and set(b['allowed_test_hooks'])==HOOKS,'baseline inventory')
    for p,h in b['files'].items():
        raw=(ROOT/p).read_bytes()
        if p in HOOKS|PREFIX:
            old=subprocess.check_output(['git','show',COMMIT+':'+p],cwd=ROOT);require(digest(old)==h,'baseline source')
            if p in PREFIX:require(raw.endswith(old),'historical suffix')
            else:
                if p!='src/lifecycle_adapter.rs':require(production(old.decode())==production(raw.decode()),'production projection: '+p)
                before=Counter(tuple(t) for t in test_bodies(old.decode()));after=Counter(tuple(t) for t in test_bodies(raw.decode()));require(not(before-after),'legacy test body: '+p)
        else:require(digest(raw)==h,'frozen input changed: '+p)
    require('#[cfg(test)]\nmod lifecycle_adapter;' in (ROOT/'src/lib.rs').read_text(),'adapter must stay test-only')
    require('#[cfg(test)]\n#[path = "lifecycle_vm_tests.rs"]\nmod lifecycle_vm_tests;' in (ROOT/'src/vm.rs').read_text(),'VM test module gate')
    rust=set(subprocess.check_output(['rg','--files','src','tests'],cwd=ROOT).decode().splitlines());rust={p for p in rust if p.endswith('.rs')}
    old={p for p in paths if p.endswith('.rs') and p.startswith(('src/','tests/'))};require(rust==old|NEW,'unexpected Rust files')
    return b

def inventory():
    r=load(HERE/'test-inventory.json');raw=(ROOT/'src/lifecycle_vm_tests.rs').read_bytes();s=raw.decode()
    observed=sorted('vm::lifecycle_vm_tests::'+n for n in re.findall(r'#\[test\]\s+fn (\w+)\(',s));ignored=['vm::lifecycle_vm_tests::'+n for n in re.findall(r'#\[test\]\s+#\[ignore[^\n]*\]\s+fn (\w+)\(',s)]
    require(r['source_sha256']==digest(raw) and r['ordinary']==observed and len(observed)==len(set(observed)) and observed,'VM scenario inventory')
    require(ignored==[r['ignored_driver']]==['vm::lifecycle_vm_tests::replay_lifecycle_vm_traces'],'VM worker inventory');return r

def coverage():
    r=load(HERE/'case-coverage.json');raw=(ROOT/'docs/stage2bc-adapter/case-coverage.json').read_bytes();prior=json.loads(raw)
    require(digest(raw)==r['source_sha256'] and len(r['cases'])==len(prior['cases'])==115,'case inventory')
    tests=set(inventory()['ordinary'])
    for c,p in zip(r['cases'],prior['cases']):
        require(c['prior_case']==p and c['native_equivalence_proven'] is False,'native evidence changed')
        require(set(c['runtime_tests'])<=tests,'mapped runtime test missing')
        require(p['prior_case']['basis']!='unresolved' or not c['runtime_tests'],'unknown promoted')
    return dict(cases=115,native_unknowns=sum(p['prior_case']['basis']=='unresolved' for p in prior['cases']))

def verify():
    baseline();i=inventory();c=coverage();r=vm();require(load(HERE/'trace-manifest.json')['cases']==json.loads(json.dumps(cases())),'timeline manifest')
    return dict(stage='2BD',status='pass',baseline_files=865,byte_frozen_files=857,test_hook_files=sorted(HOOKS),ordinary_vm_tests=len(i['ordinary']),vm_traces=r['replay']['traces'],vm_steps=r['replay']['steps'],**c,target_execution='none',**FLAGS)
if __name__=='__main__':print(json.dumps(verify(),sort_keys=True))
