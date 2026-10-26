"""Current source checks, separate from historical baseline execution."""
import json,re,subprocess
from adapter_diff import ROOT,HERE,digest,load,require,verify as adapter,cases
from source_projection import production,test_bodies
COMMIT='7a215303c8828cfb6dd5c868499ef8f24c5d9d3d'
HOOKS={'src/lib.rs','src/cell.rs','src/ownership.rs','src/copy_plan.rs'}
PREFIX={'README.md','docs/magic_Linux32-rust-rewrite-roadmap.md'}
FLAGS=dict(runtime_changed=False,public_api_changed=False,dependencies_changed=False,
    rust_source_changed=True,test_only_real_value_adapter_added=True,model_changed=False,
    new_native_analysis=False,container_capture_ready=True,direct_owner_capture_ready=False,
    non_scalar_ready=False,stage2_complete=False,implementation_ready=False,
    real_type_adapter_validated=True,historical_python_tests_run_on_current_source=False)

def baseline():
    b=load(HERE/'baseline-inputs.json');require(b['baseline_commit']==COMMIT and b['stage']=='2BC' and b['baseline_stage']=='2BB','baseline identity')
    paths=subprocess.check_output(['git','ls-tree','-r','--name-only',COMMIT],cwd=ROOT).decode().splitlines()
    require(set(paths)==set(b['files']) and len(paths)==839 and set(b['allowed_test_hooks'])==HOOKS,'baseline inventory')
    for p,h in b['files'].items():
        raw=(ROOT/p).read_bytes()
        if p in HOOKS|PREFIX:
            old=subprocess.check_output(['git','show',COMMIT+':'+p],cwd=ROOT)
            require(digest(old)==h,'historical source identity')
            if p in PREFIX:require(raw.endswith(old),'historical suffix')
            else:
                require(production(old.decode())==production(raw.decode()),'non-test source changed: '+p)
                original=test_bodies(old.decode());now=test_bodies(raw.decode())
                require(all(now.get(k)==v for k,v in original.items()),'legacy test body changed: '+p)
        else:require(digest(raw)==h,'frozen input changed: '+p)
    source=(ROOT/'src/lib.rs').read_text();require('#[cfg(test)]\nmod lifecycle_adapter;' in source,'adapter must be test-only')
    rust=set(subprocess.check_output(['rg','--files','src','tests'],cwd=ROOT).decode().splitlines());rust={p for p in rust if p.endswith('.rs')}
    require(rust=={p for p in paths if p.endswith('.rs') and p.startswith(('src/','tests/'))}|{'src/lifecycle_adapter.rs'},'Rust source boundary')
    return b

def inventory():
    r=load(HERE/'test-inventory.json');raw=(ROOT/'src/lifecycle_adapter.rs').read_bytes();s=raw.decode()
    ordinary=sorted('lifecycle_adapter::tests::'+n for n in re.findall(r'#\[test\]\s+fn (\w+)\(',s))
    ignored=['lifecycle_adapter::tests::'+n for n in re.findall(r'#\[test\]\s+#\[ignore[^\n]*\]\s+fn (\w+)\(',s)]
    require(r['source_sha256']==digest(raw) and r['ordinary']==ordinary and ignored==[r['ignored_driver']]==['lifecycle_adapter::tests::replay_adapter_traces'],'adapter test inventory')
    require(len(ordinary)==len(set(ordinary)) and len(ordinary)>0,'duplicate or empty adapter inventory')
    return r

def coverage():
    r=load(HERE/'case-coverage.json');raw=(ROOT/'docs/stage2bb-kernel/case-coverage.json').read_bytes();prior=json.loads(raw)
    require(r['source_sha256']==digest(raw) and len(r['cases'])==len(prior['cases'])==115,'case inventory')
    tests=set(inventory()['ordinary'])
    for c,p in zip(r['cases'],prior['cases']):
        require(c['prior_case']==p,'case evidence changed')
        require(set(c['adapter_tests'])<=tests and c['native_equivalence_proven'] is False,'case test/claim')
        require(p['basis']!='unresolved' or not c['adapter_tests'],'native unknown promoted')
    return dict(cases=115,native_unknowns=sum(c['prior_case']['basis']=='unresolved' for c in r['cases']))

def verify():
    baseline();i=inventory();c=coverage();r=adapter()
    require(load(HERE/'trace-manifest.json')['cases']==cases(),'trace manifest changed')
    return dict(stage='2BC',status='pass',baseline_files=839,byte_frozen_files=833,
        test_hook_files=sorted(HOOKS),ordinary_adapter_tests=len(i['ordinary']),
        adapter_traces=r['replay']['traces'],adapter_steps=r['replay']['steps'],**c,target_execution='none',**FLAGS)
if __name__=='__main__':print(json.dumps(verify(),sort_keys=True))
