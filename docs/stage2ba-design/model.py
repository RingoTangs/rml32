"""Independent bounded lifecycle policy; no native or Rust execution."""
from copy import deepcopy
from dataclasses import dataclass, field
import json

class Rejected(Exception):
    pass

@dataclass
class Object:
    state: str = 'alive'
    refs: int = 0
    execs: int = 0
    cleaned: int = 0

@dataclass
class World:
    objects: dict = field(default_factory=dict)
    slots: dict = field(default_factory=dict)
    heads: dict = field(default_factory=dict)
    frames: dict = field(default_factory=dict)
    queue: list = field(default_factory=list)
    aliases: dict = field(default_factory=dict)
    retired: set = field(default_factory=set)
    closed: bool = False
    transferred: bool = False

    def key(self):
        return json.dumps(dict(objects={k:vars(v) for k,v in self.objects.items()},
            slots=self.slots, heads=self.heads, frames=self.frames, queue=self.queue,
            aliases=self.aliases, retired=sorted(self.retired), closed=self.closed, transferred=self.transferred),sort_keys=True)

    def apply(self, op, *args, fail=False):
        candidate=deepcopy(self)
        candidate._event(op,*args)
        audit(candidate)
        if fail: raise Rejected('prepublication failure')
        self.__dict__.update(candidate.__dict__)

    def _valid(self, o):
        if o not in self.objects or self.objects[o].state not in ('alive','pending'):
            raise Rejected('invalid object')

    def _ref(self,o,n):
        self.objects[o].refs+=n
        obj=self.objects[o]
        if obj.state=='shell' and obj.refs==0: obj.state='freed'

    def _hold(self,v,n):
        if v[0]=='object': self._ref(v[1],n)
        if v[0]=='function':
            h=self.heads[v[1]]; h['owners']+=n
            if h['owners']==0:
                captures=h['captures'][:]; creator=h['creator']
                del self.heads[v[1]]
                for c in captures: self._drop(c)
                if creator is not None:self._ref(creator,-1)

    def _put(self,s,v):
        if s in self.slots: raise Rejected('occupied slot')
        if v[0]=='object': self._valid(v[1])
        if v[0]=='function' and v[1] not in self.heads: raise Rejected('missing head')
        self.slots[s]=v; self._hold(v,1)

    def _drop(self,s):
        if s not in self.slots: raise Rejected('missing slot')
        self._hold(self.slots.pop(s),-1);self.retired.discard(s)
        self.aliases={k:v for k,v in self.aliases.items() if v!=s}

    def _clean(self,o):
        obj=self.objects[o]
        if obj.cleaned:raise AssertionError('duplicate cleanup')
        obj.cleaned+=1
        for s in list(self.slots):
            if s.startswith('var:'+o+':'):self._drop(s)

    def _event(self,op,*a):
        if self.closed and op not in ('retire','drain-host','drain','exit','drop'):
            raise Rejected('closed store')
        if op=='object':
            o,=a
            if o in self.objects:raise Rejected('identity reuse')
            self.objects[o]=Object()
        elif op=='put':self._put(*a)
        elif op=='drop':self._drop(*a)
        elif op=='function':
            h,creator,sources,dst=a
            if h in self.heads:raise Rejected('duplicate head')
            if creator is not None:self._valid(creator)
            values=[self.slots[s] for s in sources]
            self.heads[h]=dict(creator=creator,captures=[],owners=0)
            if creator is not None:self._ref(creator,1)
            for i,v in enumerate(values):
                c='cap:'+h+':'+str(i);self._put(c,v);self.heads[h]['captures'].append(c)
            self._put(dst,('function',h))
        elif op in ('copy','move','overwrite'):
            src,dst=a
            if src not in self.slots:raise Rejected('missing source')
            if src==dst and op=='move':return
            v=self.slots[src]
            if op!='move':
                o=v[1] if v[0]=='object' else self.heads[v[1]]['creator'] if v[0]=='function' else None
                if o is not None and self.objects[o].state in ('queued','shell','freed'):
                    self._drop(src);self._put(src,('undefined',None));v=('undefined',None)
            if src==dst:return
            if dst in self.slots:
                if op!='overwrite':raise Rejected('occupied destination')
                # Retain before releasing potentially last source/head owner.
                self._hold(v,1);old=self.slots[dst];self.slots[dst]=v;self._hold(old,-1)
            else:
                # Raw move may preserve invalid payload; copy has normalized above.
                self.slots[dst]=v;self._hold(v,1)
            if op=='move':self._drop(src)
        elif op=='enter':
            frame,o=a;self._valid(o)
            if frame in self.frames:raise Rejected('occupied frame')
            self.frames[frame]=o;self.objects[o].execs+=1
        elif op=='exit':
            f,=a
            if f not in self.frames:raise Rejected('missing frame')
            o=self.frames.pop(f);obj=self.objects[o];obj.execs-=1
            if obj.execs==0 and obj.state=='pending':
                obj.state='queued';self.queue.insert(0,o);self._ref(o,1)
        elif op=='delete':
            o,=a;self._valid(o);obj=self.objects[o]
            if obj.execs:obj.state='pending'
            else:
                # Mark invalid before cleanup in this policy to prevent reentrant use.
                obj.state='shell';self._clean(o)
                obj.state='shell' if obj.refs else 'freed'
        elif op=='drain':
            if not self.queue:raise Rejected('empty queue')
            o=self.queue.pop(0);self._clean(o);self.objects[o].state='shell';self._ref(o,-1)
        elif op=='retire':
            s,=a
            if not s.startswith('host:') or s not in self.slots:raise Rejected('not a host root')
            self.retired.add(s)
        elif op=='drain-host':
            for s in sorted(self.retired):self._drop(s)
        elif op=='alias':
            name,s=a
            if s not in self.slots or name in self.aliases:raise Rejected('invalid alias')
            self.aliases[name]=s
        elif op=='invoke':
            source,destinations=a
            v=self.slots[source]
            if v[0]!='function':raise Rejected('not a function')
            h=self.heads[v[1]]
            if h['creator'] is not None:self._valid(h['creator'])
            if len(destinations)!=len(h['captures']):raise Rejected('arity')
            unique=h['owners']==1
            for capture,dst in zip(list(h['captures']),destinations):
                self._event('move' if unique else 'copy',capture,dst)
            if unique:
                h['captures']=[]
                self._drop(source)
        elif op in ('cancel','restart'):
            for f in list(self.frames):self._event('exit',f)
            for s in list(self.slots):
                if s in self.slots and not s.startswith(('host:','var:','cap:')):self._drop(s)
            # No implicit queue drain: safety-point choice remains explicit.
        elif op=='expand':
            h,index,dst=a
            if h not in self.heads:raise Rejected('missing head')
            self._event('copy',self.heads[h]['captures'][index],dst)
        elif op=='close':self.closed=True
        elif op=='transfer':self.transferred=True
        elif op=='snapshot':
            if a[0] not in self.slots:raise Rejected('missing slot')
        else:raise Rejected('unknown event')


def audit(w):
    """Recompute ownership independently from slots, heads, frames and queue."""
    refs={o:0 for o in w.objects};execs=dict(refs);owners={h:0 for h in w.heads}
    for v in w.slots.values():
        if v[0]=='object':refs[v[1]]+=1
        if v[0]=='function':owners[v[1]]+=1
    for h,v in w.heads.items():
        assert owners[h]==v['owners'] and owners[h]>0,'head owner mismatch'
        if v['creator'] is not None:refs[v['creator']]+=1
        assert all(c in w.slots for c in v['captures']),'missing capture'
    for o in w.frames.values():execs[o]+=1
    assert len(w.queue)==len(set(w.queue)),'double enqueue'
    for o in w.queue:refs[o]+=1
    capture_slots={c for h in w.heads.values() for c in h['captures']}
    assert {s for s in w.slots if s.startswith('cap:')}==capture_slots,'orphan capture'
    for o,v in w.objects.items():
        assert v.refs==refs[o] and v.refs>=0,'ordinary reference mismatch'
        assert v.execs==execs[o] and v.execs>=0,'execution reference mismatch'
        assert (v.state=='queued')==(o in w.queue),'queue state mismatch'
        assert v.cleaned in (0,1),'duplicate cleanup'
        assert v.state not in ('shell','freed') or v.cleaned==1,'early cleanup state'
        assert v.state!='freed' or (not v.refs and not v.execs),'early free'
        assert v.state not in ('queued','shell','freed') or v.execs==0,'invalid active frame'
        assert not v.cleaned or not any(s.startswith('var:'+o+':') for s in w.slots),'uncleared variables'
        assert v.state!='shell' or v.refs>0,'unreleased shell'
        assert v.state!='pending' or v.execs>0,'pending without execution'
    assert all(s in w.slots for s in w.aliases.values()),'dangling alias'
    assert w.retired<=w.slots.keys(),'missing retired root'
