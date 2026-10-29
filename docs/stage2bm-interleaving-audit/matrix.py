"""Finite actions-only domain; no expected values or counters go to Rust."""
import copy
import sys
from common import ROOT, authority
from interleaving_oracle import Reference, Rejected

sys.path.append(str(ROOT/'docs/stage2bl-replacement-audit'))
old = authority('_2bm_bl_layouts', 'docs/stage2bl-replacement-audit/matrix.py')
HIGH = dict(max_objects=3,max_function_heads=2,max_ordinary_refs_per_object=16,max_exec_refs_per_object=3,max_owners_per_header=16)
FIELDS = dict(objects='max_objects',heads='max_function_heads',ordinary='max_ordinary_refs_per_object',exec='max_exec_refs_per_object',owners='max_owners_per_header')
def a(op, **kw): return dict(op=op, **kw)
def put(dest, value): return a('put',dest=dest,value=value)
def profiles(f):
    # Existing explicit profiles plus independently calculated limits on this
    # extended path, including execution reservations absent from 2BL.
    qs=old.configs(f)+[HIGH]
    r=Reference(HIGH)
    for action in f['setup']: r.apply(action)
    for o in r.objects:
        c=copy.deepcopy(r)
        for x in [a('exec',id='e0',object=o),a('prepare_exec',id='e1',object=o),a('delete',object=o),a('exit',id='e0'),a('abort_exec',id='e1')]: c.apply(x)
        for key,field in FIELDS.items():
            for delta in (-1,0,1):
                q=dict(HIGH);q[field]=max(2 if key=='ordinary' else 1 if key in ('exec','owners') else 0,c.peak[key]+delta);qs.append(q)
    seen=set()
    for q in qs:
        key=tuple(q[k] for k in FIELDS.values())
        if key not in seen: seen.add(key);yield q

def extras(f):
    return [None,put('s0',['int',1]),put('d0',['int',1]),a('drop',slot='alias'),a('view',header='h0'),a('unview',header='h0'),a('zero')]

def traces(quick=False):
    for f in old.families():
        objects=[x['id'] for x in f['setup'] if x['op']=='object']
        for qi,q in enumerate([HIGH] if quick else profiles(f)):
            for obj in objects:
                routes=[[a('delete',object=obj)], [a('exec',id='e0',object=obj),a('delete',object=obj),a('exit',id='e0'),a('maintain',objects=1)]]
                for ri,route in enumerate(routes):
                    for begin in range(len(route)+1):
                        for end in range(begin,len(route)+1):
                            for ei,extra in enumerate(extras(f) if q==HIGH else [None]):
                                for terminal in ('commit','abort'):
                                    actions=route[:begin]+[a('prepare',writes=f['writes'],raw=f.get('raw',False))]+route[begin:end]+([extra] if extra else [])+[a(terminal,continue_on_error=True)]+route[end:]
                                    yield dict(id=f'{f["name"]}-Q{qi:03}-{obj}-R{ri}-{begin}{end}-E{ei}-{terminal}',family=f['name'],limits=q,setup=f['setup'],actions=actions)

def directed(quick=False):
    setup=[a('object',id='o0'),a('object',id='o1'),put('s0',['object','o0']),put('d0',['undefined'])]
    families=[]
    route=[a('exec',id='e0',object='o0'),a('delete',object='o0'),a('exit',id='e0'),a('maintain',objects=1)]
    for script in (False,True):
        families.append(('object_script' if script else 'object_raw',setup,a('prepare',writes=[['s0','d0']],script=script),route,'commit','abort'))
        fun=setup+[a('make',id='h0',kind='function',dest='s0',value=['undefined'],creator='o0')]
        families.append(('function_script' if script else 'function_raw',fun,a('prepare',writes=[['s0','d0']],script=script),route,'commit','abort'))
    families.append(('raw_empty_move',setup,a('prepare_move',source='s0',dest='d0'),route,'commit','abort'))
    families.append(('unpublished_function',setup,a('prepare_head',id='h0',kind='function',dest='d0',value=['undefined'],creator='o0'),route,'commit_head','abort_head'))
    families.append(('prospective_exec',setup+[a('exec',id='e0',object='o0')],a('prepare_exec',id='ep',object='o0'),[a('delete',object='o0'),a('exit',id='e0')],'commit_exec','abort_exec'))
    for source,dest in [('v:o0','d0'),('s0','v:o0')]:
        families.append(('variable_source' if source.startswith('v:') else 'variable_destination',setup+[put('v:o0',['object','o1'])],a('prepare',writes=[[source,dest]],script=True),route,'commit','abort'))
    for fi,(family,base,prepare,path,commit,abort) in enumerate(families):
        qs=[HIGH] if quick else list(old.old.profiles())+[HIGH]
        for qi,q in enumerate(qs):
            for begin in range(len(path)+1):
                for end in range(begin,len(path)+1):
                    for terminal in (commit,abort):
                        finish=a(terminal,continue_on_error=True)
                        if family=='prospective_exec': finish.update(id='ep',dest='installed')
                        actions=path[:begin]+[prepare]+path[begin:end]+[finish]+path[end:]
                        if family=='prospective_exec': actions += [a('exit',id='installed'),a('maintain',objects=1)]
                        yield dict(id=f'D{fi}-Q{qi:03}-{begin}{end}-{terminal}',family=family,limits=q,setup=base,actions=actions)
    # Freed/reaped and two-object queue order are explicit reachable controls.
    for qi,q in enumerate([HIGH] if quick else list(old.old.profiles())+[HIGH]):
        for order in [('o0','o1'),('o1','o0')]:
            actions=[a('exec',id=o,object=o) for o in order]+[a('delete',object=o) for o in order]+[a('exit',id=o) for o in order]
            actions += [a('maintain',objects=1),a('maintain',objects=1),a('drop',slot='s0'),a('zero')]
            yield dict(id=f'L-Q{qi:03}-'+'.'.join(order),family='lifo_freed',limits=q,setup=setup,actions=actions)

def metadata_cases():
    for kind in ('array','mapping','function'):
        for hold in (False,True):
            setup=[a('object',id='o0'),a('make',id='h0',kind=kind,dest='s0',value=['undefined'],creator='o0' if kind=='function' else None)]
            actions=([a('view',header='h0')] if hold else [])+[a('delete',object='o0'),a('drop',slot='s0'),a('maintain',metadata=1),a('zero')]
            if hold: actions += [a('unview',header='h0')]
            actions += [a('maintain',metadata=1) for _ in range(24)]+[a('put',dest='s0',value=['object','o0'],continue_on_error=True)]
            yield dict(id=f'M-{kind}-{hold}',family='metadata',limits=HIGH,setup=setup,actions=actions,detail=True)
