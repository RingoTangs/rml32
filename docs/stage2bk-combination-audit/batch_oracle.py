"""Independent watch-set/parallel-publication reference atop frozen quota primitives."""
import copy,importlib.util,sys
from common import ROOT,require
s=importlib.util.spec_from_file_location('_2bk_quota',ROOT/'docs/stage2bh-production-contract/quota_model.py');m=importlib.util.module_from_spec(s);sys.modules[s.name]=m;s.loader.exec_module(m)
class BatchOracle:
 def __init__(self,q):self.w=m.World(m.LifecycleLimits(**q));self.versions=dict.fromkeys(['s0','s1','d0','d1'],0);self.watch={};self.writes={};self.moves=[];self.root=True
 def event(self,a):
  op=a['op'];w=self.w
  if op=='setup':
   c=copy.deepcopy(w);c.apply('reserve_object','o');c.apply('commit','o');c.apply('put','s0',('object',0));c.apply('reserve_header','h','array',None,'s1');c.apply('commit','h');c.apply('put','d0',m.SCALAR);c.apply('put','d1',m.SCALAR);c.apply('put','host:r',m.SCALAR);self.w=c;return
  if op=='prepare':
   layout=a['layout'];self.watch={};self.writes={};self.moves=[]
   def write(src,dst):
    self.watch[src]=self.versions[src];self.watch[dst]=self.versions[dst];v=w.slots[src]
    if dst in self.writes and self.writes[dst]!=v:raise m.Rejected('InvalidState','conflicting writes')
    self.writes[dst]=v
   if layout=='independent':write('s0','d0');write('s1','d1')
   elif layout=='fanout':write('s0','d0');write('s0','d1')
   elif layout=='same_equal':write('s0','d0');write('s0','d0')
   elif layout=='same_conflict':write('s0','d0');write('s1','d0')
   elif layout=='copy_move_alias':write('s1','s0');self.moves=[('s0','d0')]
   elif layout=='moves':self.moves=[('s0','d0'),('s1','d1')]
   for src,dst in self.moves:self.watch[src]=self.versions[src];self.watch[dst]=self.versions[dst]
   return
  if op=='interfere':
   target=a['target']
   if target=='retire':
    if self.root:w.apply('retire_host','host:r');self.root=False
   elif target=='zero':return
   else:w.apply('put',target,m.SCALAR);self.versions[target]+=1
   return
  if op=='abort':return
  require(op=='commit','reference action')
  if any(self.versions[n]!=v for n,v in self.watch.items()):raise m.Rejected('InvalidState','stale batch')
  if any(src in self.writes for src,_ in self.moves):raise m.Rejected('InvalidState','move source write overlap')
  c=copy.deepcopy(w);claims={}
  for value in self.writes.values():
   for k,n in c.claims(value).items():claims[k]=claims.get(k,0)+n
  # Reserve the aggregate peak before releasing any old destination payload.
  c.charge(claims)
  for dest,v in self.writes.items():c.install_reserved(dest,v)
  for src,dst in self.moves:c.apply('move',src,dst)
  m.audit(c);self.w=c
 def projection(self):
  w=self.w
  def v(n):
   value=w.slots.get(n,m.SCALAR)
   if value[0]=='scalar':return ['int',1]if self.versions[n]else ['undefined']
   return [value[0],'o0'if value[0]=='object'else 'h0']
  return dict(slots={n:v(n)for n in self.versions if n in w.slots},ordinary=w.used('ord:0'),header_owners=sum(w.used('owner:'+str(n))for n in w.headers),objects=len(w.objects),pending_roots=len(w.retired),queue=len(w.queue))
