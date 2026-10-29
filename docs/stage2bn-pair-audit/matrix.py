import copy
import itertools
import sys
from common import ROOT,authority
from pair_oracle import Reference,Rejected
sys.path.append(str(ROOT/'docs/stage2bm-interleaving-audit'))
old=authority('_2bn_bm_matrix','docs/stage2bm-interleaving-audit/matrix.py')
HIGH=dict(max_objects=3,max_function_heads=4,max_ordinary_refs_per_object=32,max_exec_refs_per_object=4,max_owners_per_header=32)
FIELDS=dict(objects='max_objects',heads='max_function_heads',ordinary='max_ordinary_refs_per_object',exec='max_exec_refs_per_object',owners='max_owners_per_header')
def a(op,**kw):return dict(op=op,**kw)
def put(n,v):return a('put',dest=n,value=v)
def seed():return [a('object',id='o0'),a('object',id='o1'),put('s0',['object','o0']),put('s1',['object','o1']),put('d0',['undefined']),put('d1',['undefined']),put('u',['undefined'])]
def cp(tx,s,d,script=False):return a('prepare',tx=tx,kind='copy',writes=[[s,d]],script=script)
def hp(tx,creator='o0',capture='c0'):return a('prepare',tx=tx,kind='head',head='h'+tx,creator=creator,value=['header',capture],dest='out:'+tx)
def ep(tx,o='o0'):return a('prepare',tx=tx,kind='exec',object=o)
def make(n,kind,dest,v):return a('make',id=n,kind=kind,dest=dest,value=v)

def families():
    result=[]
    for name,first,second in [
        ('independent',cp('A','s0','d0'),cp('B','s1','d1')),
        ('shared_source',cp('A','s0','d0'),cp('B','s0','d1')),
        ('same_destination_equal',cp('A','s0','d0'),cp('B','s0','d0')),
        ('same_destination_different',cp('A','s0','d0'),cp('B','s1','d0')),
        ('forward',cp('A','s0','d0'),cp('B','d0','d1')),
        ('reverse',cp('A','s0','d0'),cp('B','s1','s0')),
        ('mutual',cp('A','s0','s1'),cp('B','s1','s0')),
    ]:result.append(dict(name=name,pair='CC',setup=seed(),prepares={'A':first,'B':second}))
    result.append(dict(name='retired_member',pair='CC',setup=seed()+[make('h0','array','d0',['object','o1'])],prepares={'A':cp('A','u','d0'),'B':cp('B','s0','m:h0')}))
    result.append(dict(name='shared_dag',pair='CC',setup=seed()+[make('child','array','build',['object','o1']),make('h0','array','d0',['header','child']),make('h1','mapping','d1',['header','child']),a('drop',slot='build')],prepares={'A':cp('A','u','d0'),'B':cp('B','u','d1')}))
    result.append(dict(name='function_member',pair='CC',setup=seed()+[make('c0','array','capture',['undefined']),hp('Z'),a('commit',tx='Z'),make('h0','mapping','d0',['header','hZ']),a('drop',slot='out:Z')],prepares={'A':cp('A','u','d0'),'B':cp('B','m:h0','d1')}))
    for pair in ('CH','CE','HH','HE','EE'):
        for same in (False,True):
            setup=seed()+[make('c0','array','capture',['undefined'])]
            obj='o0' if same else 'o1'
            first={'C':cp('A','s0','d0',True),'H':hp('A'),'E':ep('A')}[pair[0]]
            second={'H':hp('B',obj),'E':ep('B',obj)}[pair[1]]
            result.append(dict(name=pair+('_same' if same else '_different'),pair=pair,setup=setup,prepares={'A':first,'B':second}))
    return result

def schedules(f):
    for order in itertools.permutations(('PA','PB','TA','TB')):
        if order.index('PA')>order.index('TA') or order.index('PB')>order.index('TB'):continue
        for ends in itertools.product(('commit','abort'),repeat=2):
            actions=[]
            for event in order:
                tx=event[1];actions.append(copy.deepcopy(f['prepares'][tx]) if event[0]=='P' else a(ends[tx=='B'],tx=tx))
            yield '.'.join(order)+'.'+'.'.join(ends),actions

def configs(f):
    qs=list(old.old.old.profiles())+[HIGH];peak={k:0 for k in FIELDS}
    for _,actions in schedules(f):
        r=Reference(HIGH)
        for x in f['setup']+actions:
            try:r.apply(x)
            except Rejected:pass
        for k in peak:peak[k]=max(peak[k],r.peak[k])
    for k,field in FIELDS.items():
        for delta in (-1,0,1):
            q=dict(HIGH);q[field]=max(2 if k=='ordinary' else 1 if k in ('exec','owners') else 0,peak[k]+delta);qs.append(q)
    seen=set()
    for q in qs:
        key=tuple(q[k] for k in FIELDS.values())
        if key not in seen:seen.add(key);yield q

def trace(f,ident,q,actions,**kw):
    actions=[dict(x,continue_on_error=True) for x in actions]
    return dict(id=ident,family=f['name'],pair=f['pair'],limits=q,setup=f['setup'],actions=actions,**kw)

def core(quick=False):
    for f in families():
        for qi,q in enumerate([HIGH] if quick else configs(f)):
            for si,(name,actions) in enumerate(schedules(f)):
                yield trace(f,f'{f["name"]}-Q{qi:03}-S{si:02}',q,actions)

def merges(left,right):
    if not left:yield right;return
    if not right:yield left;return
    for tail in merges(left[1:],right):yield [left[0]]+tail
    for tail in merges(left,right[1:]):yield [right[0]]+tail

def interferences(quick=False):
    for f in families():
        extras=[put('s0',['int',7]),put('d0',['int',8]),a('drop',slot='s0'),a('alias_cell',source='s0',dest='alias'),a('zero')]
        if any(x.get('id')=='h0' for x in f['setup']):extras += [a('view',header='h0'),a('unview',header='h0')]
        for si,(_,actions) in enumerate(schedules(f)):
            for position in range(5):
                for ei,extra in enumerate(extras):
                    yield trace(f,f'{f["name"]}-S{si:02}-I{position}-{ei}',HIGH,actions[:position]+[extra]+actions[position:])
            if quick:continue
            for obj in ('o0','o1'):
                for ri,route in enumerate([[a('delete',object=obj)],[a('exec',id='base',object=obj),a('delete',object=obj),a('exit',id='base'),a('maintain',objects=1)]]):
                    for mi,merged in enumerate(merges(actions,route)):
                        yield trace(f,f'{f["name"]}-S{si:02}-{obj}-R{ri}-M{mi:02}',HIGH,merged)

def lifetime_cases():
    for kind in ('object','array','mapping','function'):
        setup=seed()
        source='s0'
        if kind in ('array','mapping'):setup+=[make('h0',kind,'value',['object','o0'])];source='value'
        if kind=='function':setup+=[make('c0','array','capture',['object','o0']),hp('Z'),a('commit',tx='Z')];source='out:Z'
        for ends in itertools.product(('commit','abort'),repeat=2):
            for order in [('A','B'),('B','A')]:
                for alias in (False,True):
                    actions=[cp('A',source,'d0'),cp('B',source,'d1')]
                    if alias:actions.append(a('alias_cell',source=source,dest='alias'))
                    actions += [a('drop',slot=source),a('delete',object='o0'),a('zero')]
                    actions += [a(ends[tx=='B'],tx=tx) for tx in order]
                    actions += [a('drop',slot='alias'),a('zero')]
                    f=dict(name='lifetime_'+kind,pair='CC',setup=setup)
                    yield trace(f,f'life-{kind}-{ends}-{order}-{alias}',HIGH,actions)

def metadata_cases():
    # Only metadata work is mixed with transaction termination, never with object
    # drain in one call. Eligibility is independently checked before scanning.
    for kind,hold,first,second in itertools.product(('array','mapping','function'),(False,True),('commit','abort'),('commit','abort')):
        source='source';header='h0'
        setup=seed()
        if kind=='function':
            setup += [make('c0','array','capture',['undefined']),hp('Z'),a('commit',tx='Z')];source='out:Z';header='hZ'
        else:setup += [make('h0',kind,source,['object','o0'])]
        actions=[cp('A',source,'d0'),cp('B',source,'d1')]
        if hold:actions.append(a('view',header=header))
        actions += [a('drop',slot=source),a(first,tx='A'),a('maintain',metadata=1),a('zero'),a(second,tx='B')]
        actions += [a('maintain',metadata=1) for _ in range(16)]
        if hold:actions.append(a('unview',header=header))
        actions += [a('maintain',metadata=1) for _ in range(24)]
        f=dict(name='metadata',pair='CC',setup=setup)
        yield trace(f,f'meta-{kind}-{hold}-{first}-{second}',HIGH,actions,detail=True)
