"""D08 design-only quota/maintenance model; not Rust execution or allocator proof."""
from dataclasses import dataclass, field
from copy import deepcopy

class Rejected(Exception):
    def __init__(self, kind, message):
        self.kind=kind
        super().__init__(message)

def need(ok,message,kind='InvalidState'):
    if not ok: raise Rejected(kind,message)

def checked_add(a,b,bits):
    need(type(a)is int and type(b)is int and 0<=a<1<<bits and 0<=b<1<<bits,'invalid unsigned operand','Limit')
    need(a+b<1<<bits,'checked addition overflow','Limit')
    return a+b

@dataclass(frozen=True)
class LifecycleLimits:
    max_objects:int
    max_function_heads:int
    max_ordinary_refs_per_object:int
    max_exec_refs_per_object:int
    max_owners_per_header:int
    def validate(self,bits):
        need(bits in (32,64),'unsupported model width')
        for value in self.__dict__.values(): need(type(value)is int and 0<=value<1<<bits,'limit outside usize','Limit')
        need(self.max_objects<=2**32-1,'object identity domain','Limit')
        need(self.max_ordinary_refs_per_object>=2,'queue reference requires two ordinary units','Limit')
        need(self.max_exec_refs_per_object>=1 and self.max_owners_per_header>=1,'reference limits must be positive','Limit')

@dataclass(frozen=True)
class LifecycleBudget:
    host_roots:int
    objects:int
    metadata_entries:int
    def validate(self,bits):
        for value in self.__dict__.values():need(type(value)is int and 0<=value<1<<bits,'budget outside usize','Limit')

@dataclass
class Object:
    state:str='alive'
    cleaned:bool=False
    queue_reserved:bool=True

@dataclass
class Header:
    kind:str
    creator:int|None=None
    creator_held:bool=False
    owners:int=0
    members:list=field(default_factory=list)
    retired:bool=False
    views:int=0

@dataclass
class Reservation:
    kind:str
    claims:dict
    data:dict

SCALAR=('scalar',None)

@dataclass
class World:
    limits:LifecycleLimits
    bits:int=64
    mode:bool=True
    closed:bool=False
    objects:dict=field(default_factory=dict)
    headers:dict=field(default_factory=dict)
    slots:dict=field(default_factory=dict)
    versions:dict=field(default_factory=dict)
    frames:dict=field(default_factory=dict)
    reservations:dict=field(default_factory=dict)
    ledger:dict=field(default_factory=dict)
    retired:set=field(default_factory=set)
    queue:list=field(default_factory=list)
    weak_rows:dict=field(default_factory=dict)
    next_object:int=0
    next_header:int=1
    epoch:int=0
    scan_epoch:int=0
    scan_pending:list|None=None
    scan_complete:bool=True
    force_rescan:bool=False
    def __post_init__(self):self.limits.validate(self.bits)
    def touch(self):
        if self.epoch==2**64-1:self.epoch=0;self.force_rescan=True
        else:self.epoch+=1
        self.scan_complete=False
    def used(self,key):return self.ledger.get(key,0)
    def reserved(self,key):return sum(r.claims.get(key,0)for r in self.reservations.values())
    def cap(self,key):
        if key=='objects':return self.limits.max_objects
        if key=='heads':return self.limits.max_function_heads
        kind,identity=key.split(':');identity=int(identity)
        if kind=='ord':return self.limits.max_ordinary_refs_per_object-int(self.objects[identity].state in ('alive','pending'))
        if kind=='exec':return self.limits.max_exec_refs_per_object
        if kind=='owner':return self.limits.max_owners_per_header
        raise AssertionError('unknown resource')
    def charge(self,claims):
        for key,n in claims.items():need(checked_add(self.used(key),n,self.bits)<=self.cap(key),'quota exceeded: '+key,'Limit')
        for key,n in claims.items():self.ledger[key]=self.used(key)+n
    def uncharge(self,claims):
        for key,n in claims.items():
            assert self.used(key)>=n,'counter underflow'
            value=self.used(key)-n
            if value:self.ledger[key]=value
            else:self.ledger.pop(key,None)
    def valid_object(self,o):need(o in self.objects and self.objects[o].state in ('alive','pending'),'invalid object','InvalidObject')
    def claims(self,value):
        kind,identity=value
        if kind=='object':
            need(identity in self.objects and self.objects[identity].state!='freed','invalid raw object','InvalidObject')
            return {'ord:'+str(identity):1}
        if kind=='header':
            need(identity in self.headers and not self.headers[identity].retired,'retired or unknown head')
            return {'owner:'+str(identity):1}
        need(kind=='scalar','unknown value')
        return {}
    def add_reservation(self,token,kind,claims,data):
        need(token not in self.reservations,'duplicate reservation')
        self.charge(claims);self.reservations[token]=Reservation(kind,claims,data)
    def consume(self,token):return self.reservations.pop(token)
    def maybe_free(self,o):
        obj=self.objects[o]
        if obj.state=='shell' and self.used('ord:'+str(o))==0:obj.state='freed';self.touch()
    def maybe_retire(self,h):
        header=self.headers[h]
        if not header.retired and header.owners==0 and self.reserved('owner:'+str(h))==0:
            header.retired=True
            for name in list(header.members):self.drop_slot(name)
            if header.creator_held:
                header.creator_held=False;self.uncharge({'ord:'+str(header.creator):1});self.maybe_free(header.creator)
            self.touch()
    def release(self,value):
        self.uncharge(self.claims_for_release(value))
        if value[0]=='object':self.maybe_free(value[1])
        if value[0]=='header':
            self.headers[value[1]].owners-=1;self.maybe_retire(value[1])
    def claims_for_release(self,value):
        return {'ord:'+str(value[1]):1}if value[0]=='object'else {'owner:'+str(value[1]):1}if value[0]=='header'else {}
    def bump_slot(self,name):self.versions[name]=self.versions.get(name,0)+1
    def install_reserved(self,name,value):
        old=self.slots.get(name,SCALAR);self.slots[name]=value;self.bump_slot(name)
        if value[0]=='header':self.headers[value[1]].owners+=1
        self.release(old);self.touch()
    def drop_slot(self,name):
        if name not in self.slots:return
        self.retired.discard(name);old=self.slots.pop(name);self.bump_slot(name);self.release(old);self.touch()
    def push(self,o):
        obj=self.objects[o];assert obj.queue_reserved and o not in self.queue
        assert obj.state=='pending' and self.used('exec:'+str(o))==0
        obj.state='queued';self.charge({'ord:'+str(o):1});self.queue.insert(0,o);self.touch()
    def maybe_enqueue(self,o):
        if self.objects[o].state=='pending' and self.used('exec:'+str(o))==0:self.push(o)
    def clean(self,o):
        obj=self.objects[o];assert not obj.cleaned,'duplicate cleanup'
        for name in list(self.slots):
            if name.startswith('var:'+str(o)+':'):self.drop_slot(name)
        obj.cleaned=True;self.touch()
    def metadata_keys(self):
        return [('object',k)for k in sorted(self.objects)]+[('header',k)for k in sorted(self.headers)]+[('weak',k)for k in sorted(self.weak_rows)]
    def data_alive(self,kind,identity):
        if kind=='object':return identity in self.objects
        if kind=='header':return identity in self.headers and (not self.headers[identity].retired or self.headers[identity].views>0)
        return False
    def scan_done(self):return not self.metadata_keys() or (not self.force_rescan and self.scan_complete and self.scan_epoch==self.epoch)
    def progress(self,p,stop=None):
        p.update(pending_roots=len(self.retired),queued_objects=len(self.queue),scan_complete=self.scan_done())
        p['stop']=stop or ('Complete'if not self.retired and not self.queue and p['scan_complete']else 'BudgetExhausted')
        return p
    def maintain(self,budget,blocked_roots=(),blocked_objects=(),workspace_objects=(),blocked_metadata=()):
        need(self.mode,'legacy lifecycle mode','UnsupportedFeature');need(not self.closed,'closed epoch')
        budget.validate(self.bits)
        p=dict(retired_roots=0,drained_objects=0,reaped_objects=0,reclaimed_heads=0,pruned_weak_entries=0,metadata_examined=0)
        if not any(budget.__dict__.values()):return self.progress(p)
        for name in sorted(self.retired)[:budget.host_roots]:
            if name in blocked_roots:return self.progress(p,'BorrowBlocked')
            self.drop_slot(name);p['retired_roots']+=1
        while self.queue and p['drained_objects']<budget.objects:
            o=self.queue[0]
            if o in blocked_objects:return self.progress(p,'BorrowBlocked')
            if o in workspace_objects:return self.progress(p,'WorkspaceLimit')
            # A successful atomic unit holds its queue owner throughout cleanup.
            self.queue.pop(0);self.clean(o);self.objects[o].state='shell';self.uncharge({'ord:'+str(o):1});self.maybe_free(o);p['drained_objects']+=1
        while p['metadata_examined']<budget.metadata_entries:
            if self.force_rescan:self.scan_pending=None;self.force_rescan=False
            if self.scan_pending is None:
                self.scan_pending=self.metadata_keys();self.scan_epoch=self.epoch;self.scan_complete=False
            if not self.scan_pending:
                if self.scan_epoch==self.epoch:self.scan_complete=True;break
                self.scan_pending=None;continue
            key=self.scan_pending[0]
            if key in blocked_metadata:return self.progress(p,'BorrowBlocked')
            self.scan_pending.pop(0);p['metadata_examined']+=1;kind,identity=key
            if kind=='object' and identity in self.objects and self.objects[identity].state=='freed':
                assert self.used('ord:'+str(identity))==self.used('exec:'+str(identity))==0
                del self.objects[identity];self.uncharge({'objects':1});p['reaped_objects']+=1;self.touch()
            elif kind=='header' and identity in self.headers:
                h=self.headers[identity]
                if h.retired and h.views==0:
                    assert h.owners==0 and not h.creator_held
                    del self.headers[identity]
                    if h.kind=='function':self.uncharge({'heads':1});p['reclaimed_heads']+=1
                    self.touch()
            elif kind=='weak' and identity in self.weak_rows:
                target=self.weak_rows[identity]
                if not self.data_alive(*target):del self.weak_rows[identity];p['pruned_weak_entries']+=1
            if not self.scan_pending and not self.force_rescan and self.scan_epoch==self.epoch:self.scan_complete=True
        return self.progress(p)
    def apply(self,op,*args,fail=False,**kw):
        candidate=deepcopy(self)
        if op!='maintain':need(not candidate.closed,'closed epoch')
        result=candidate._op(op,*args,**kw)
        if fail:raise Rejected('Limit','injected precommit failure')
        audit(candidate);self.__dict__.update(candidate.__dict__);return result
    def _op(self,op,*a,**kw):
        if op=='reserve_object':
            token,=a;need(self.next_object<2**32-1,'object ID exhausted','Limit');identity=self.next_object
            self.add_reservation(token,'object',{'objects':1},dict(id=identity));self.next_object+=1;return identity
        if op=='reserve_header':
            token,kind,creator,dest=a;need(kind in ('function','array','mapping'),'header kind')
            need(kind=='function' or creator is None,'container creator forbidden')
            if creator is not None:self.valid_object(creator)
            need(self.next_header<2**64-1,'header ID exhausted','Limit');h=self.next_header
            claims={'owner:'+str(h):1}
            if kind=='function':claims['heads']=1
            if creator is not None:claims['ord:'+str(creator)]=1
            self.add_reservation(token,'header',claims,dict(id=h,kind=kind,creator=creator,dest=dest,version=self.versions.get(dest,0)));self.next_header+=1;return h
        if op=='reserve_copy':
            token,source,dest=a;need(source in self.slots,'missing source');value=self.slots[source];normalize=kw.get('script',False)
            if normalize and ((value[0]=='object' and self.objects[value[1]].state not in ('alive','pending')) or (value[0]=='header' and self.headers[value[1]].kind=='function' and self.headers[value[1]].creator is not None and self.objects[self.headers[value[1]].creator].state not in ('alive','pending'))):value=SCALAR
            self.add_reservation(token,'copy',self.claims(value),dict(value=value,source=source,dest=dest,version=self.versions.get(dest,0),source_version=self.versions[source],normalize=normalize and value!=self.slots[source]));return
        if op=='reserve_exec':
            token,o,frame=a;self.valid_object(o);need(frame not in self.frames and all(r.kind!='exec' or r.data['frame']!=frame for r in self.reservations.values()),'duplicate frame')
            self.add_reservation(token,'exec',{'exec:'+str(o):1},dict(object=o,frame=frame));return
        if op=='commit':
            token,=a;need(token in self.reservations,'missing reservation');r=self.reservations[token];d=r.data
            if r.kind in ('copy','header'):need(self.versions.get(d['dest'],0)==d['version'],'destination changed')
            if r.kind=='copy':need(self.versions.get(d['source'],0)==d['source_version'],'source changed')
            if r.kind=='header' and d['creator']is not None:self.valid_object(d['creator'])
            self.consume(token)
            if r.kind=='object':self.objects[d['id']]=Object();self.touch();return d['id']
            if r.kind=='header':
                self.headers[d['id']]=Header(d['kind'],d['creator'],d['creator']is not None)
                self.install_reserved(d['dest'],('header',d['id']));return d['id']
            if r.kind=='copy':
                self.install_reserved(d['dest'],d['value'])
                if d['normalize'] and d['source']!=d['dest']:
                    old=self.slots[d['source']];self.slots[d['source']]=SCALAR;self.bump_slot(d['source']);self.release(old)
                return
            self.frames[d['frame']]=d['object'];return
        if op=='abort':
            token,=a;need(token in self.reservations,'missing reservation');r=self.consume(token);self.uncharge(r.claims)
            for key in r.claims:
                if key.startswith('owner:')and int(key.split(':')[1])in self.headers:self.maybe_retire(int(key.split(':')[1]))
                if key.startswith('ord:'):self.maybe_free(int(key.split(':')[1]))
                if key.startswith('exec:'):self.maybe_enqueue(int(key.split(':')[1]))
            return
        if op=='put':
            name,value=a;self.charge(self.claims(value));self.install_reserved(name,value);return
        if op=='member':
            h,name,value=a;need(h in self.headers and not self.headers[h].retired,'inactive header');need(name not in self.slots and all(name not in v.members for v in self.headers.values()),'duplicate member')
            self.charge(self.claims(value));self.install_reserved(name,value);self.headers[h].members.append(name);return
        if op=='drop':self.drop_slot(*a);return
        if op=='move':
            source,dest=a;need(source in self.slots,'missing source')
            if source==dest:return
            value=self.slots[source];old=self.slots.get(dest,SCALAR);self.slots[source]=SCALAR;self.slots[dest]=value;self.bump_slot(source);self.bump_slot(dest);self.release(old);self.touch();return
        if op=='exit':
            frame,=a;need(frame in self.frames,'missing frame');o=self.frames.pop(frame);self.uncharge({'exec:'+str(o):1});self.maybe_enqueue(o);return
        if op in ('delete','task_cleanup'):
            o,=a;need(o in self.objects,'unknown object','InvalidObject');obj=self.objects[o]
            if op=='task_cleanup' and obj.state in ('queued','shell','freed'):return
            self.valid_object(o)
            if op=='task_cleanup' or self.used('exec:'+str(o)):
                obj.state='pending';self.maybe_enqueue(o);return
            need(not kw.get('borrowed',False),'borrow conflict');need(not kw.get('workspace',False),'workspace failure','Limit')
            obj.state='shell';self.clean(o);self.maybe_free(o);return
        if op=='retire_host':
            name,=a;need(name.startswith('host:') and name in self.slots,'missing Host root');self.retired.add(name);return
        if op=='host_safepoint':
            need(not kw.get('borrowed',False),'root registry borrowed')
            for name in sorted(self.retired):self.drop_slot(name)
            return
        if op in ('view','drop_view'):
            h,=a;need(h in self.headers,'unknown header')
            self.headers[h].views+=1 if op=='view' else -1;need(self.headers[h].views>=0,'view underflow');self.touch();return
        if op=='weak':
            name,kind,identity=a;need(name not in self.weak_rows and kind in ('object','header'),'weak row identity');self.weak_rows[name]=(kind,identity);self.touch();return
        if op=='maintain':return self.maintain(*a,**kw)
        raise Rejected('InvalidState','unknown operation')

def audit(w):
    expected={}
    def add(key,n=1):expected[key]=expected.get(key,0)+n
    for o in w.objects:add('objects')
    for h,data in w.headers.items():
        if data.kind=='function':add('heads')
        owners=sum(v==('header',h)for v in w.slots.values())
        assert data.owners==owners,'header slot count'
        total=owners+w.reserved('owner:'+str(h))
        assert data.creator_held==(total>0 and data.creator is not None),'creator presence'
        assert data.retired==(total==0),'head retirement'
        assert data.views>=0
        if data.creator_held:add('ord:'+str(data.creator))
    for value in w.slots.values():
        if value[0]=='object':assert value[1]in w.objects;add('ord:'+str(value[1]))
        if value[0]=='header':assert value[1]in w.headers;add('owner:'+str(value[1]))
    for o in w.frames.values():assert o in w.objects;add('exec:'+str(o))
    assert len(w.queue)==len(set(w.queue)),'duplicate queue node'
    for o in w.queue:add('ord:'+str(o))
    for token,r in w.reservations.items():
        assert token and r.kind in ('object','header','copy','exec') and all(n>0 for n in r.claims.values()),'reservation shape'
        if r.kind=='object':assert r.claims=={'objects':1},'object reservation'
        if r.kind=='exec':assert r.claims=={'exec:'+str(r.data['object']):1},'exec reservation'
        if r.kind=='copy':assert r.claims==w.claims(r.data['value']),'copy reservation'
        if r.kind=='header':
            claims={'owner:'+str(r.data['id']):1}
            if r.data['kind']=='function':claims['heads']=1
            if r.data['creator']is not None:claims['ord:'+str(r.data['creator'])]=1
            assert r.claims==claims,'head reservation'
        for key,n in r.claims.items():add(key,n)
    assert {k:n for k,n in w.ledger.items()if n}=={k:n for k,n in expected.items()if n},'quota ledger mismatch'
    for key,n in w.ledger.items():assert 0<=n<1<<w.bits and n<=w.cap(key),'capacity or arithmetic violation'
    for o,data in w.objects.items():
        assert data.queue_reserved,'missing preallocated queue node'
        assert (o in w.queue)==(data.state=='queued'),'queue/state mismatch'
        if data.state=='pending':assert w.used('exec:'+str(o))>0,'pending without execution guard'
        if data.state in ('shell','freed'):assert data.cleaned,'unclean shell'
        if data.state=='freed':assert w.used('ord:'+str(o))==w.used('exec:'+str(o))==0,'freed owner'
    assert w.retired<=w.slots.keys() and all(k.startswith('host:')for k in w.retired),'retired roots'
    # Header-member cycles are excluded; creator/variable relationships are separate.
    visiting=set();done=set()
    def visit(h):
        assert h not in visiting,'header cycle'
        if h in done:return
        visiting.add(h)
        for member in w.headers[h].members:
            value=w.slots.get(member,SCALAR)
            if value[0]=='header':visit(value[1])
        visiting.remove(h);done.add(h)
    for h in w.headers:visit(h)
