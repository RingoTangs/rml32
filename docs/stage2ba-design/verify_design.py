"""Read-only structural and frozen-input validation for D01."""
import hashlib
import json
from pathlib import Path
import subprocess
from collections import Counter
ROOT=Path(__file__).resolve().parents[2]
HERE=Path(__file__).resolve().parent
PREFIX={'README.md','docs/magic_Linux32-rust-rewrite-roadmap.md'}
COMMIT='c85a1c07bee399507cc3387e6177973aa822d2ef'
def digest(raw):return hashlib.sha256(raw).hexdigest()
def load(p):return json.loads(p.read_bytes())
def require(v,m):
    if not v:raise AssertionError(m)
def baseline():
    b=load(HERE/'baseline-inputs.json')
    require(b['baseline_commit']==COMMIT,'baseline commit')
    paths=subprocess.check_output(['git','ls-tree','-r','--name-only',COMMIT],cwd=ROOT).decode().splitlines()
    require(set(paths)==set(b['files']) and len(paths)==800,'baseline inventory')
    for p,h in b['files'].items():
        raw=(ROOT/p).read_bytes()
        if p in PREFIX:
            old=subprocess.check_output(['git','show',COMMIT+':'+p],cwd=ROOT)
            require(digest(old)==h and raw.endswith(old),'historical suffix: '+p)
        else:require(digest(raw)==h,'frozen file: '+p)
    return b

def case_map(data=None):
    m=load(HERE/'case-map.json') if data is None else data
    raw=(ROOT/m['source']).read_bytes();source=json.loads(raw)
    require(digest(raw)==m['source_sha256'],'case source hash')
    rows=m['cases'];require(len(rows)==len(source)==115,'case count')
    require([r['id'] for r in rows]==[c['id'] for c in source],'case inventory')
    for r,c in zip(rows,source):
        require(r['source_case']==c,'case content')
        require(r['source_case_sha256']==digest(json.dumps(c,sort_keys=True,separators=(',',':')).encode()),'case hash')
        require(r['category'] in ('modeled','partial-model','native-unknown','excluded'),'category')
        require(c['basis']!='unresolved' or r['category']=='native-unknown','native unknown promoted')
        require(r['category'] not in ('modeled','partial-model') or r['model_evidence'],'missing model evidence')
    return dict(sorted(Counter(r['category'] for r in rows).items()))
def verify():
    baseline();counts=case_map()
    migration=load(HERE/'api-migration.json')
    require(len(migration)==11 and all(r['implemented'] is False for r in migration),'migration scope')
    enumeration=load(HERE/'enumeration.json')
    require(enumeration['complete'] and len(enumeration['domains'])==2,'incomplete enumeration')
    for name,h in enumeration['source_sha256'].items():require(digest((HERE/name).read_bytes())==h,'stale model')
    require([(d['objects'],d['depth']) for d in enumeration['domains']]==[(1,6),(2,4)],'domain bounds')
    for d in enumeration['domains']:
        require(d['complete'] and d['states']<=200000 and d['accepted_transitions']+d['rejected_transitions']<=1000000,'capacity incomplete')
    return dict(stage='2BA',status='pass',protected_previous_files=800,cases=115,categories=counts,
        design_ready=True,implementation_ready=False,target_execution='none')
if __name__=='__main__':print(json.dumps(verify(),sort_keys=True))
