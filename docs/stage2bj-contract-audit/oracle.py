"""Composition/refinement of the byte-frozen 2BH semantic model, not a replacement."""
import copy,importlib.util,sys
from common import ROOT
spec=importlib.util.spec_from_file_location('_2bj_frozen_quota',ROOT/'docs/stage2bh-production-contract/quota_model.py');m=importlib.util.module_from_spec(spec);sys.modules[spec.name]=m;spec.loader.exec_module(m)
class Oracle:
 def __init__(self,limits):
  self.w=m.World(m.LifecycleLimits(**limits));self.objects={};self.headers={};self.known=set();self.staged={};self.execs={}
 def value(self,v):
  kind,n=v
  return (kind,self.objects[n]if kind=='object'else self.headers[n]if kind=='header'else None)
 def event(self,a):
  op=a['op'];w=self.w
  if op=='commit_head':
   token=a['token'];name,dest=self.staged.pop(token)
   try:h=w.apply('commit',token)
   except m.Rejected:
    w.apply('abort',token);raise
   self.headers[name]=h;self.known.add(dest);member='mem:'+name+':0';w.apply('member',h,member,m.SCALAR);self.known.add(member);return
  if op=='commit_copy':
   token=a['token'];self.staged.pop(token)
   try:w.apply('commit',token)
   except m.Rejected:w.apply('abort',token);raise
   return
  candidate=copy.deepcopy(self)
  try:result=candidate.atomic(a)
  except m.Rejected:raise
  self.__dict__.update(candidate.__dict__);return result
 def atomic(self,a):
  w=self.w;op=a['op']
  if op=='object':
   n=w.apply('reserve_object','create');w.apply('commit','create');self.objects[a['id']]=n;slot='var:'+a['id']+':0';w.apply('put',slot,m.SCALAR);self.known.add(slot)
  elif op in ('head','prepare_head'):
   creator=self.objects[a['creator']]if a.get('creator')is not None else None;token=a.get('token','head')
   n=w.apply('reserve_header',token,a['kind'],creator,a['dest'])
   if op=='head':
    w.apply('commit',token);self.headers[a['id']]=n;member='mem:'+a['id']+':0';w.apply('member',n,member,m.SCALAR);self.known.update([a['dest'],member])
   else:self.staged[token]=(a['id'],a['dest'])
  elif op=='put':w.apply('put',a['dest'],self.value(a['value']));self.known.add(a['dest'])
  elif op=='copy':w.apply('reserve_copy','copy',a['source'],a['dest'],script=a.get('script',False));w.apply('commit','copy');self.known.add(a['dest'])
  elif op=='move':w.apply('move',a['source'],a['dest']);self.known.add(a['dest'])
  elif op=='drop':w.apply('drop',a['slot'])
  elif op=='exec':w.apply('reserve_exec',a['id'],self.objects[a['object']],a['id']);w.apply('commit',a['id'])
  elif op=='exit':w.apply('exit',a['id'])
  elif op in ('delete','task_cleanup'):w.apply(op,self.objects[a['object']])
  elif op in ('view','drop_view'):w.apply(op,self.headers[a['header']])
  elif op=='retire_host':w.apply('retire_host',a['slot'])
  elif op=='host_safepoint':w.apply('host_safepoint')
  elif op=='prepare_copy':w.apply('reserve_copy',a['token'],a['source'],a['dest']);self.staged[a['token']]=('copy',a['dest'])
  elif op in ('abort_copy','abort_head'):w.apply('abort',a['token']);self.staged.pop(a['token'])
  elif op=='maintain':
   assert a['budget']['metadata_entries']==0,'concrete metadata has a separate independent oracle'
   blocked=[self.objects[a['borrow_object']]]if 'borrow_object'in a else []
   workspace=w.queue[a['workspace_after']:a['workspace_after']+1]if 'workspace_after'in a else []
   return w.apply('maintain',m.LifecycleBudget(**a['budget']),blocked_objects=blocked,workspace_objects=workspace)
  elif op in ('transfer','close'):pass
  elif op=='vm_pending':
   self.atomic(dict(op='object',id=a['id']));o=self.objects[a['id']]
   w.apply('reserve_exec','vm',o,'vm');w.apply('commit','vm');w.apply('delete',o);w.apply('exit','vm');w.apply('maintain',m.LifecycleBudget(0,1,0))
   return dict(returned=9,queued_before=1,drained=1)
  elif op=='foreign':raise m.Rejected('InvalidState','foreign origin')
  else:raise AssertionError('unsupported semantic action '+op)
 def projection(self):
  w=self.w;on={n:k for k,n in self.objects.items()};hn={n:k for k,n in self.headers.items()}
  def value(v):return [v[0],on[v[1]]if v[0]=='object'else hn[v[1]]if v[0]=='header'else None]
  objects={on[n]:dict(state=o.state,ordinary=w.used('ord:'+str(n)),exec=w.used('exec:'+str(n)),cleaned=o.cleaned)for n,o in w.objects.items()}
  headers={hn[n]:dict(kind=h.kind,owners=w.used('owner:'+str(n)),retired=h.retired)for n,h in w.headers.items()}
  return dict(objects=objects,headers=headers,slots={n:value(w.slots.get(n,m.SCALAR))for n in sorted(self.known)},queue=[on[n]for n in w.queue],function_heads_charged=w.used('heads'),pending_roots=len(w.retired))
