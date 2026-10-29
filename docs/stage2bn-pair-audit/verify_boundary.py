import gzip,subprocess
from common import ROOT,HERE,COMMIT,authority,digest,load,require

def verify():
    b=load(HERE/'baseline-inputs.json')
    require(b['base_commit']==COMMIT and len(b['files'])==1203,'snapshot identity')
    require(subprocess.run(['git','merge-base','--is-ancestor',COMMIT,'HEAD'],cwd=ROOT,capture_output=True).returncode==0,'baseline parent is not in current history')
    require(digest(subprocess.check_output(['git','ls-files','--stage','-z'],cwd=ROOT))==b['index_sha256'],'pre-existing index changed')
    require(digest(gzip.decompress((HERE/'baseline-overlay.patch.gz').read_bytes()))==b['overlay_sha256'],'overlay bytes')
    require(digest(subprocess.check_output(['git','diff','--cached','--binary',COMMIT],cwd=ROOT))==b['overlay_sha256'],'staged overlay changed')
    paths=subprocess.check_output(['git','ls-files','-z'],cwd=ROOT).decode().split('\0');require(set(paths)-{''}==set(b['files']),'snapshot inventory')
    projection=authority('_2bn_projection','docs/stage2bg-host-function-test/source_projection.py');changed=[];bodies=0
    for n,h in b['files'].items():
        raw=(ROOT/n).read_bytes()
        if digest(raw)==h:continue
        changed.append(n);old=subprocess.check_output(['git','show',':'+n],cwd=ROOT);require(digest(old)==h,'snapshot original '+n)
        if n in b['allowed_prefixes']:require(raw.endswith(old),'historical prefix '+n)
        else:
            require(n=='src/boundary.rs' and n in b['allowed_changes'],'frozen source changed '+n)
            require(projection.cfg(projection.tokens(raw.decode()))==projection.cfg(projection.tokens(old.decode())),'production tokens changed')
            before=projection.test_bodies(old.decode());after=projection.test_bodies(raw.decode());require(all(x in after for x in before),'old test body changed');bodies+=len(before)
    expected={n for n in b['files'] if n.startswith(('src/','tests/'))}|set(b['new_source'])
    require(set(subprocess.check_output(['rg','--files','src','tests'],cwd=ROOT,text=True).splitlines())==expected,'source inventory')
    c=load(HERE/'case-coverage.json');require(c['prior_sha256']==digest((ROOT/c['prior_path']).read_bytes()),'coverage authority')
    require(c['native_unknowns']==20 and c['native_unknowns_promoted']==0 and len(c['new_test_names'])==len(set(c['new_test_names']))==14,'coverage inventory')
    return dict(stage='2BN',status='pass',baseline_commit=COMMIT,baseline_overlay_sha256=b['overlay_sha256'],baseline_files=1203,byte_frozen_files=1203-len(changed),changed_files=sorted(changed),index_unchanged=True,normal_build_tokens_unchanged=True,preserved_test_bodies_in_changed_modules=bodies,old_rust_tests=878,old_doctests=5,native_unknowns=20,native_unknowns_promoted=0)
if __name__=='__main__':
    import json
    print(json.dumps(verify(),sort_keys=True))
