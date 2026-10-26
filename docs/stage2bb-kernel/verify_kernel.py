"""Verify the isolated source boundary, frozen evidence and kernel delivery."""
import json
from pathlib import Path
import re
import subprocess
from differential import ROOT,HERE,digest,load,require,verify as differential
COMMIT='99f5626ec402499b0d007bcfc04db9e6ee57c756'
PREFIX={'README.md','docs/magic_Linux32-rust-rewrite-roadmap.md'}
RUST={'tests/lifecycle_kernel.rs','tests/support/lifecycle_kernel.rs'}
FLAGS=dict(runtime_changed=False,public_api_changed=False,dependencies_changed=False,
    rust_source_changed=True,test_only_rust_kernel_added=True,model_changed=False,
    new_native_analysis=False,container_capture_ready=True,direct_owner_capture_ready=False,
    non_scalar_ready=False,stage2_complete=False,implementation_ready=False,
    kernel_prototype_validated=True)

def baseline():
    b=load(HERE/'baseline-inputs.json')
    require(b['stage']=='2BB' and b['baseline_stage']=='2BA' and b['baseline_commit']==COMMIT,'baseline identity')
    paths=subprocess.check_output(['git','ls-tree','-r','--name-only',COMMIT],cwd=ROOT).decode().splitlines()
    require(set(paths)==set(b['files']) and len(paths)==820,'baseline inventory')
    for p,h in b['files'].items():
        raw=(ROOT/p).read_bytes()
        if p in PREFIX:
            old=subprocess.check_output(['git','show',COMMIT+':'+p],cwd=ROOT)
            require(digest(old)==h and raw.endswith(old),'historical suffix changed: '+p)
        else:require(digest(raw)==h,'protected file changed: '+p)
    current=set(subprocess.check_output(['rg','--files','src','tests'],cwd=ROOT).decode().splitlines())
    current={p for p in current if p.endswith('.rs')}
    old={p for p in paths if p.endswith('.rs') and p.startswith(('src/','tests/'))}
    require(current==old|RUST,'unexpected Rust source boundary')
    return b

def inventory(data=None):
    r=load(HERE/'test-inventory.json') if data is None else data
    source=(ROOT/'tests/support/lifecycle_kernel.rs').read_text()
    require(r['source_sha256']==digest(source.encode()),'kernel source hash')
    ordinary=sorted('kernel::tests::'+n for n in re.findall(r'#\[test\]\s+fn (\w+)\(',source))
    ignored=['kernel::tests::'+n for n in re.findall(r'#\[test\]\s+#\[ignore[^\n]*\]\s+fn (\w+)\(',source)]
    require(r['stage']=='2BB' and r['ordinary']==ordinary and len(ordinary)==len(set(ordinary)) and len(ordinary)>0,'ordinary kernel inventory')
    require(ignored==[r['ignored_driver']]==['kernel::tests::replay_model_traces'],'ignored driver inventory')
    return r

def coverage(data=None):
    r=load(HERE/'case-coverage.json') if data is None else data
    prior_raw=(ROOT/'docs/stage2ba-design/case-map.json').read_bytes();prior=json.loads(prior_raw)
    require(r['source']=='docs/stage2ba-design/case-map.json' and r['source_sha256']==digest(prior_raw),'case source identity')
    require(len(r['cases'])==len(prior['cases'])==115,'case inventory')
    tests=set(inventory()['ordinary'])
    for c,p in zip(r['cases'],prior['cases']):
        require(c['id']==p['id'] and c['prior_category']==p['category'] and c['basis']==p['source_case']['basis'] and c['findings']==p['source_case']['findings'] and c['source_case_sha256']==p['source_case_sha256'],'case evidence changed')
        require(c['native_equivalence_proven'] is False,'native equivalence promoted')
        require(set(c['kernel_tests'])<=tests,'missing mapped kernel test')
        require(c['coverage']==('bounded-policy-slice' if c['kernel_tests'] else 'not-promoted'),'coverage status')
        require(c['basis']!='unresolved' or not c['kernel_tests'],'native unknown promoted')
    return dict(cases=115,policy_slices=sum(bool(c['kernel_tests']) for c in r['cases']),native_unknowns=sum(c['basis']=='unresolved' for c in r['cases']))

def verify():
    baseline();i=inventory();c=coverage();d=differential()
    return dict(stage='2BB',status='pass',protected_previous_files=820,
        ordinary_kernel_tests=len(i['ordinary']),separately_invoked_driver=i['ignored_driver'],
        **c,transitions=d['replay']['transitions'],target_execution='none',**FLAGS)
if __name__=='__main__':print(json.dumps(verify(),sort_keys=True))
