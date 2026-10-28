"""Independent simultaneous replacement and fixed-point DAG retirement reference.
No Rust counters, runtime cursor or expected states are inputs to this model.
"""
import copy
from collections import Counter
U=('undefined',)
class Rejected(Exception):
 def __init__(self,kind):self.kind=kind;super().__init__(kind)
class Reference:
 def __init__(self,q):
  self.q=q;self.objects=[];self.heads={};self.slots={};self.known=set();self.version={};self.watch={};self.writes={};self.views=set();self.raw=False;self.peak=dict(objects=0,heads=0,ordinary=0,owners=0)
 def counts(self):
  ordinary=Counter({n:0 for n in self.objects});owners=Counter({n:0 for n in self.heads})
  for v in self.slots.values():
   if v[0]=='object':ordinary[v[1]]+=1
   if v[0]=='header':owners[v[1]]+=1
  for h in self.heads.values():
   if h['kind']=='function'and not h['retired']and h['creator']is not None:ordinary[h['creator']]+=1
  return ordinary,owners
 def reserve(self,values=(),creator=None,new_kind=None):
  ordinary,owners=self.counts()
  for v in values:
   if v[0]=='object':ordinary[v[1]]+=1
   if v[0]=='header':owners[v[1]]+=1
  if creator is not None:ordinary[creator]+=1
  heads=sum(h['kind']=='function'for h in self.heads.values())+int(new_kind=='function')
  required=dict(objects=len(self.objects),heads=heads,ordinary=max(ordinary.values(),default=0)+1,owners=max([*owners.values(),int(new_kind is not None)],default=0))
  for k,v in required.items():self.peak[k]=max(self.peak[k],v)
  if heads>self.q['max_function_heads']or any(n>self.q['max_ordinary_refs_per_object']-1 for n in ordinary.values())or any(n>self.q['max_owners_per_header']for n in owners.values()):raise Rejected('Limit')
 def assign(self,name,value):self.slots[name]=tuple(value);self.known.add(name);self.version[name]=self.version.get(name,0)+1
 def retire(self):
  while True:
   _,owners=self.counts();dead=[n for n,h in self.heads.items()if not h['retired']and owners[n]==0]
   if not dead:return
   for n in dead:
    h=self.heads[n];h['retired']=True;self.assign(h['member'],U)
 def apply(self,a):
  c=copy.deepcopy(self)
  try:c.event(a)
  except Rejected:
   # Peak calibration is independent of publication, including rejected attempts.
   self.peak={k:max(self.peak[k],c.peak[k])for k in self.peak};raise
  self.__dict__.update(c.__dict__)
 def event(self,a):
  op=a['op']
  if op=='object':
   self.peak['objects']=max(self.peak['objects'],len(self.objects)+1)
   if len(self.objects)>=self.q['max_objects']:raise Rejected('Limit')
   self.objects.append(a['id']);return
  if op=='make':
   n=a['id'];creator=a.get('creator');v=tuple(a['value']);kind=a['kind'];self.reserve([v],creator,kind)
   self.heads[n]=dict(kind=kind,creator=creator,member='m:'+n,retired=False);self.assign('m:'+n,v);self.assign(a['dest'],('header',n));return
  if op=='put':self.reserve([tuple(a['value'])]);self.assign(a['dest'],a['value']);self.retire();return
  if op=='drop':
   self.slots.pop(a['slot'],None);self.retire();return
  if op=='view':
   if a['header']in self.heads and not self.heads[a['header']]['retired']:self.views.add(a['header'])
   return
  if op=='unview':self.views.discard(a['header']);return
  if op=='zero':return
  if op=='prepare':
   self.writes={};self.watch={};self.raw=a.get('raw',False)
   for source,dest in a['writes']:
    self.watch[source]=self.version[source];self.watch[dest]=self.version[dest];v=self.slots.get(source,U)
    if dest in self.writes and self.writes[dest]!=v:raise Rejected('InvalidState')
    self.writes[dest]=v
   return
  if op=='abort':self.writes={};return
  if op=='commit':
   if any(self.version[n]!=v for n,v in self.watch.items()):raise Rejected('InvalidState')
   if self.raw:raise Rejected('InvalidState')
   self.reserve(self.writes.values())
   if a.get('borrow'):
    # Actual read borrow permits reading, and blocks mutation of any written or
    # recursively retired cell. All selected borrows are in this release set.
    target=a['borrow'];_,owners=self.counts();projected=dict(self.slots);projected.update(self.writes)
    dying=set()
    while True:
     refs=Counter(v[1]for v in projected.values()if v[0]=='header');more={n for n,h in self.heads.items()if not h['retired']and refs[n]==0}-dying
     if not more:break
     dying|=more
     for n in more:projected[self.heads[n]['member']]=U
    if target in self.writes or any(self.heads[n]['member']==target for n in dying):raise Rejected('InvalidState')
   for dest,v in self.writes.items():self.assign(dest,v)
   self.retire();self.writes={};return
  raise AssertionError('unlisted action '+op)
 def projection(self):
  ordinary,owners=self.counts()
  return dict(slots={n:list(self.slots.get(n,U))for n in sorted(self.known)},objects={n:dict(ordinary=ordinary[n],state='alive',exec=0,cleaned=False)for n in self.objects},headers={n:dict(kind=h['kind'],owners=owners[n],retired=h['retired'])for n,h in self.heads.items()},function_heads_charged=sum(h['kind']=='function'for h in self.heads.values()),queue=[])
