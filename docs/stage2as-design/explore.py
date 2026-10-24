"""Deterministic bounded exploration of the independent design model.

Quotient only by live-ID alpha-renaming; identity/alias structure, root kinds,
creator state and borrow flags remain. No claim about native/Rust execution.
"""
import argparse
from collections import Counter, deque
from copy import deepcopy
import hashlib
import json
from pathlib import Path
from model import World, Rejected, HEADER_TAGS, audit

HERE=Path(__file__).resolve().parent
ROOT_NAMES=('a','b','c','d')
HOST_NAMES=('h0','h1')
BOUNDS=dict(depth=3,max_states=20000,max_transitions=200000,max_headers=2,max_slots=6,
            max_roots=4,max_host_roots=2,max_items=4,max_depth=4,max_nodes=16)


def key(world):
    cells={old:i for i,old in enumerate(sorted(world.slots))}
    heads={old:i for i,old in enumerate(sorted(world.headers))}
    def value(v):return (v[0],heads[v[1]]) if v[0] in HEADER_TAGS else v
    data=dict(slots=[(cells[c],value(s.value),s.refs,s.storage) for c,s in sorted(world.slots.items())],
              headers=[(heads[h],v.kind,[cells[c] for c in v.cells],v.owners,v.creator,v.origin)
                       for h,v in sorted(world.headers.items())],
              roots=[(n,cells[r.cell],r.kind,r.readonly) for n,r in sorted(world.roots.items())],
              host=[(n,value(v)) for n,v in sorted(world.host.items())],
              objects=sorted(world.objects.items()),pending=sorted(world.pending),borrowed=sorted(cells[c] for c in world.borrowed))
    return json.dumps(data,sort_keys=True,separators=(',',':'))


def seed(profile):
    w=World(max_items=BOUNDS['max_items'],max_depth=BOUNDS['max_depth'],max_nodes=BOUNDS['max_nodes'])
    w.apply(dict(op='put',root='a',value=['int',7]))
    if profile=='ownership':
        w.apply(dict(op='build',root='b',kind='function',sources=['a']))
    elif profile=='graph':
        w.apply(dict(op='build',root='b',kind='array',sources=['a']))
        w.apply(dict(op='member-ref',source='b',index=0,root='a'))
        w.apply(dict(op='build',root='c',kind='function',sources=['b']))
    elif profile=='source-copy':
        w.apply(dict(op='object',id='o'))
        w.apply(dict(op='put',root='b',value=['object','o']))
        w.apply(dict(op='delete-object',id='o'))
    else:raise ValueError('unknown profile')
    return w


def actions(w,bounds):
    available=[n for n in ROOT_NAMES if n not in w.roots and n not in w.host]
    roots=sorted(w.roots)
    free_host=[n for n in HOST_NAMES if n not in w.host]
    for src in roots:
        yield dict(op='observe',root=src)
        yield dict(op='drop',root=src)
        yield dict(op='unborrow' if w.roots[src].cell in w.borrowed else 'borrow',root=src)
        for dst in available:
            yield dict(op='copy',source=src,root=dst)
            yield dict(op='move',source=src,root=dst)
            yield dict(op='alias',source=src,root=dst)
            if len(w.headers)<bounds['max_headers']:
                for kind in ['array','mapping','function']:
                    yield dict(op='build',root=dst,kind=kind,sources=[src])
            v=w.slots[w.roots[src].cell].value
            if v[0] in ('array','mapping'):
                for index in range(len(w.headers[v[1]].cells)):
                    yield dict(op='member-ref',source=src,index=index,root=dst)
            if v[0]=='function' and len(w.headers[v[1]].cells)==1:
                yield dict(op='invoke',root=src,outputs=[dst])
        for dst in roots:
            yield dict(op='write',target=dst,source=src)
            yield dict(op='batch-copy',sources=[src],targets=[dst])
        for host in free_host:
            yield dict(op='export',source=src,lease=host)
    for host in sorted(w.host):
        yield dict(op='release',lease=host)
        yield dict(op='retire',lease=host)
        for name in available:yield dict(op='import',lease=host,root=name)
        for name in free_host:yield dict(op='retain',source=host,lease=name)
    for name in available:yield dict(op='put',root=name,value=['int',7])


def explore(profile,bounds=None):
    bounds=dict(BOUNDS if bounds is None else bounds)
    initial=seed(profile)
    queue=deque([(initial,0,[])])
    seen={key(initial)}
    accepted=0;rejected=Counter();outside=0;cutoff=False;max_depth=0
    while queue:
        w,depth,trace=queue.popleft()
        max_depth=max(max_depth,depth)
        if depth>=bounds['depth']:continue
        for action in actions(w,bounds):
            if accepted+sum(rejected.values())>=bounds['max_transitions']:
                cutoff=True;break
            candidate=deepcopy(w);before=candidate.fingerprint()
            try:
                candidate.apply(action)
                audit(candidate)
            except Rejected as error:
                assert candidate.fingerprint()==before, ('failed action changed state',trace+[action])
                rejected[error.code]+=1
                continue
            except AssertionError as error:
                raise AssertionError(dict(profile=profile,trace=trace+[action],error=str(error))) from error
            if len(candidate.headers)>bounds['max_headers'] or len(candidate.slots)>bounds['max_slots']:
                outside+=1
                continue
            accepted+=1
            identity=key(candidate)
            if identity in seen:continue
            if len(seen)>=bounds['max_states']:
                cutoff=True;break
            seen.add(identity);queue.append((candidate,depth+1,trace+[action]))
        if cutoff:break
    return dict(profile=profile,bounds=bounds,states=len(seen),accepted_transitions=accepted,
                rejected_transitions=dict(sorted(rejected.items())),outside_profile=outside,
                completed_within_profile=not cutoff,cutoff=cutoff,reached_depth=max_depth,
                state_set_sha256=hashlib.sha256('\n'.join(sorted(seen)).encode()).hexdigest(),
                limitations=['Live-ID alpha-renaming quotient; observation IDs are not executable handles.',
                             'Finite declared grammar/seeds/depth only; excluded transitions are not native rejections.',
                             'Publication atomicity is a model construction, not proof of future Rust implementation.'])


def build_report():
    profiles=[explore(name) for name in ['ownership','graph','source-copy']]
    return dict(stage='2AS',kind='independent-design-model',native_execution=False,rust_execution=False,
                input_sha256={name:hashlib.sha256((HERE/name).read_bytes()).hexdigest() for name in ('model.py','explore.py')},
                profiles=profiles,complete_for_declared_profiles=all(p['completed_within_profile'] for p in profiles),
                full_state_space_proof=False)


def main():
    p=argparse.ArgumentParser(description=__doc__);p.add_argument('--write',action='store_true');args=p.parse_args()
    report=build_report();text=json.dumps(report,ensure_ascii=False,sort_keys=True,indent=2)+'\n'
    path=HERE/'enumeration.json'
    if args.write:path.write_text(text)
    elif not path.is_file() or path.read_text()!=text:raise ValueError('stale enumeration report')
    print(json.dumps(report,sort_keys=True))


if __name__=='__main__':main()
