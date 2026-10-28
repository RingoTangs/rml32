import copy,itertools,importlib.util
from common import ROOT
from replacement_oracle import Reference
spec=importlib.util.spec_from_file_location('_2bl_profiles',ROOT/'docs/stage2bk-combination-audit/matrix.py');old=importlib.util.module_from_spec(spec);spec.loader.exec_module(old)
HIGH=dict(max_objects=3,max_function_heads=2,max_ordinary_refs_per_object=12,max_exec_refs_per_object=2,max_owners_per_header=12)
def a(op,**kw):return dict(op=op,**kw)
def put(dest,value):return a('put',dest=dest,value=value)
def make(n,kind,dest,value,creator=None):return a('make',id=n,kind=kind,dest=dest,value=value,creator=creator)
def families():
 result=[]
 base=[a('object',id='o0'),a('object',id='o1'),put('s0',['object','o0']),put('s1',['object','o1']),put('u',['undefined']),put('d1',['undefined'])]
 result.append(dict(name='object_swap',setup=base+[put('d0',['object','o1']),put('d1',['object','o0'])],writes=[['s0','d0'],['s1','d1']]))
 for kind in ('array','mapping','function'):
  start=copy.deepcopy(base)
  if kind=='function':start+=[a('object',id='o2'),make('child','array','build',['object','o1'])];value=['header','child'];creator='o2'
  else:value=['object','o1'];creator=None
  start.append(make('h0',kind,'d0',value,creator))
  if kind=='function':start.append(a('drop',slot='build'))
  for mode in ('same_header','last_owner','external_alias','retired_member'):
   setup=copy.deepcopy(start)
   if mode=='same_header':setup.append(put('d1',['header','h0']));writes=[['d1','d0'],['d0','d1']]
   elif mode=='external_alias':setup.append(put('alias',['header','h0']));writes=[['s0','d0']]
   elif mode=='retired_member':writes=[['u','d0'],['s0','m:h0']]
   else:writes=[['s0','d0']]
   result.append(dict(name=kind+'_'+mode,setup=setup,writes=writes))
 for function in (False,True):
  setup=copy.deepcopy(base)
  if function:setup.append(a('object',id='o2'))
  setup+=[make('child','array','build',['object','o1']),make('h0','function'if function else'array','d0',['header','child'],'o2'if function else None),make('h1','mapping','d1',['header','child']),a('drop',slot='build')]
  result.append(dict(name='function_dag'if function else'shared_dag',setup=setup,writes=[['u','d0'],['u','d1']]))
 result.append(dict(name='raw_occupied',setup=base+[put('d0',['object','o1'])],writes=[['s0','d0']],raw=True))
 assert len(result)==16
 return result

def configs(f):
 ref=Reference(HIGH)
 for x in f['setup']:ref.apply(x)
 ref.apply(a('prepare',writes=f['writes'],raw=f.get('raw',False)))
 if not f.get('raw'):ref.apply(a('commit'))
 peak=ref.peak;result=old.profiles()+[HIGH]
 fields={'objects':('max_objects',0),'heads':('max_function_heads',0),'ordinary':('max_ordinary_refs_per_object',2),'owners':('max_owners_per_header',1)}
 for key,(field,minimum)in fields.items():
  for delta in (-1,0,1):result.append(dict(HIGH,**{field:max(minimum,peak[key]+delta)}))
 seen=set();out=[]
 for q in result:
  k=tuple(q.items())
  if k not in seen:seen.add(k);out.append(q)
 return out

def interference(code):
 return {'source':put('s0',['int',1]),'destination':put('d0',['int',1]),'alias':a('drop',slot='alias'),'view':a('view',header='h0'),'unview':a('unview',header='h0'),'zero':a('zero')}[code]
def traces(quick=False):
 words=[[]]+[[x]for x in ('source','destination','alias','view','unview','zero')]+[list(x)for x in itertools.product(('source','destination','alias','view','unview','zero'),repeat=2)]
 for f in families():
  qs=[HIGH]if quick else configs(f)
  for p,q in enumerate(qs):
   for i,word in enumerate(words):
    for terminal in ('commit','abort'):
     yield dict(id=f'{f["name"]}-Q{p:03}-W{i:02}-{terminal}',family=f['name'],limits=q,setup=f['setup'],actions=[a('prepare',writes=f['writes'],raw=f.get('raw',False))]+[interference(x)for x in word]+[a(terminal)])

def borrow_cases():
 for f in families():
  if f.get('raw'):continue
  for target in ('d0','m:h0'):
   if target=='m:h0'and not any(x.get('id')=='h0'for x in f['setup']):continue
   yield dict(id=f['name']+'-borrow-'+target,family=f['name'],limits=HIGH,setup=f['setup'],actions=[a('prepare',writes=f['writes']),a('commit',borrow=target)])
