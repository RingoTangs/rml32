"""Freeze 2BL, allowing only README prefixes and a cfg(test) module hook."""
import subprocess
from common import ROOT,HERE,COMMIT,authority,digest,load,require

def verify():
    b=load(HERE/'baseline-inputs.json')
    paths=subprocess.check_output(['git','ls-tree','-r','--name-only',COMMIT],cwd=ROOT,text=True).splitlines()
    require(b['base_commit']==COMMIT and set(paths)==set(b['files']) and len(paths)==1166,'baseline inventory')
    projection=authority('_2bm_projection','docs/stage2bg-host-function-test/source_projection.py')
    changed=[];bodies=0
    for n,h in b['files'].items():
        raw=(ROOT/n).read_bytes()
        if digest(raw)==h: continue
        changed.append(n);old=subprocess.check_output(['git','show',COMMIT+':'+n],cwd=ROOT)
        require(digest(old)==h,'pinned original bytes '+n)
        if n in b['allowed_prefixes']: require(raw.endswith(old),'historical prefix '+n)
        else:
            require(n=='src/boundary.rs' and n in b['allowed_changes'],'frozen file changed '+n)
            require(projection.cfg(projection.tokens(raw.decode()))==projection.cfg(projection.tokens(old.decode())),'production tokens changed')
            before=projection.test_bodies(old.decode());after=projection.test_bodies(raw.decode());require(all(x in after for x in before),'old test body changed');bodies+=len(before)
    expected={n for n in paths if n.startswith(('src/','tests/'))}|set(b['new_source'])
    actual=set(subprocess.check_output(['rg','--files','src','tests'],cwd=ROOT,text=True).splitlines())
    require(actual==expected,'source inventory')
    coverage=load(HERE/'case-coverage.json');prior=ROOT/coverage['prior_path']
    require(coverage['prior_sha256']==digest(prior.read_bytes()) and coverage['native_unknowns']==20 and coverage['native_unknowns_promoted']==0,'coverage provenance')
    require(len(coverage['new_test_names'])==len(set(coverage['new_test_names']))==9,'new test inventory')
    return dict(stage='2BM',status='pass',baseline_commit=COMMIT,baseline_files=len(paths),byte_frozen_files=len(paths)-len(changed),changed_files=sorted(changed),normal_build_tokens_unchanged=True,preserved_test_bodies_in_changed_modules=bodies,old_rust_tests=869,old_doctests=5,native_unknowns=20,native_unknowns_promoted=0)

if __name__=='__main__':
    import json
    print(json.dumps(verify(),sort_keys=True))
