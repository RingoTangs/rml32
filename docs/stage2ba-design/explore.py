"""Finite event grammar, explicit safety-point drain, no progress assumption."""
from collections import deque
from copy import deepcopy
import hashlib
import json
from pathlib import Path
from model import World, Rejected

HERE=Path(__file__).resolve().parent

def actions(w):
    for o in w.objects:
        yield ('delete',o)
        yield ('enter','f',o)
        for s in ('a','host:b'):
            yield ('put',s,('object',o))
            if not w.heads:yield ('function','h',o,[],s)
    for s in sorted(w.slots):
        if s.startswith('cap:'):continue
        yield ('drop',s)
        yield ('snapshot',s)
        if s.startswith('host:'):yield ('retire',s)
        for dst in ('a','host:b'):
            for op in ('copy','move','overwrite'):yield (op,s,dst)
    yield ('exit','f')
    yield ('drain',)
    yield ('drain-host',)
    yield ('close',)
    yield ('transfer',)

def explore(objects,depth,max_states=200000,max_transitions=1000000):
    w=World()
    for i in range(objects):w.apply('object',str(i))
    q=deque([(w,0)]);seen={w.key()};accepted=rejected=0;complete=True
    while q:
        w,d=q.popleft()
        if d==depth:continue
        for a in actions(w):
            if accepted+rejected>=max_transitions:complete=False;break
            c=deepcopy(w)
            try:c.apply(*a)
            except Rejected:rejected+=1;continue
            accepted+=1;k=c.key()
            if k not in seen:
                if len(seen)>=max_states:complete=False;break
                seen.add(k);q.append((c,d+1))
        if not complete:break
    return dict(objects=objects,depth=depth,max_heads=1,max_frames=1,max_external_slots=2,
        max_states=max_states,max_transitions=max_transitions,states=len(seen),
        accepted_transitions=accepted,rejected_transitions=rejected,
        complete=complete,state_set_sha256=hashlib.sha256('\n'.join(sorted(seen)).encode()).hexdigest())

def report():
    domains=[explore(1,6),explore(2,4)]
    return dict(stage='2BA',domains=domains,complete=all(x['complete'] for x in domains),
        full_state_space_proof=False,native_execution=False,rust_execution=False,
        grammar='actions(world); zero-capture heads in enumeration; captures covered by hand tests',
        source_sha256={p:hashlib.sha256((HERE/p).read_bytes()).hexdigest() for p in ('model.py','explore.py')})

if __name__=='__main__':
    import sys
    r=report();assert r['complete'],'incomplete exploration: cap reached'
    data=json.dumps(r,indent=2,sort_keys=True)+'\n';path=HERE/'enumeration.json'
    if '--write' in sys.argv:path.write_text(data)
    else:assert path.read_text()==data,'stale enumeration'
    print(data)
