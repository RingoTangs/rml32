"""Independent strong-slot reachability and semantic ownership reference.

A CopyPlan holds existing Cell identities and physical Values, not extra payload
owners. Header members remain physical until header destruction, and are cleared
at semantic retirement even when another plan retains their Cell identity.
"""
import copy
from collections import Counter

U=('undefined',)
class Rejected(Exception):
    def __init__(self,kind): self.kind=kind;super().__init__(kind)
def need(ok,kind='InvalidState'):
    if not ok: raise Rejected(kind)

class Reference:
    def __init__(self,q):
        self.q=q;self.cells={};self.roots={};self.objects={};self.heads={};self.transactions={};self.leases={};self.views=set()
        self.queue=[];self.known_objects=[];self.known_heads=set();self.physical=set()
        self.peak=dict(objects=0,heads=0,ordinary=0,exec=0,owners=0)

    def valid(self,o): return o in self.objects and self.objects[o]['state'] in ('alive','pending')
    def cell(self,n):
        need(not n.startswith('p:'))
        identity=self.roots.get(n,n)
        need(identity in self.cells and self.cells[identity]['alive'])
        return identity
    def new_cell(self,n,value):
        need(n not in self.cells)
        self.cells[n]=dict(value=tuple(value),revision=0,alive=True)
        return n
    def assign(self,n,value):
        self.cells[n]['value']=tuple(value);self.cells[n]['revision']+=1
    def counts(self):
        ordinary=Counter(self.queue);owners=Counter()
        for c in self.cells.values():
            if not c['alive']:continue
            v=c['value']
            if v[0]=='object':ordinary[v[1]]+=1
            if v[0]=='header':owners[v[1]]+=1
        for h in self.heads.values():
            if not h['retired'] and h['creator'] is not None:ordinary[h['creator']]+=1
        execution=Counter(self.leases.values())
        execution.update(p['object'] for p in self.transactions.values() if p['kind']=='exec')
        return ordinary,owners,execution

    def reach(self):
        cells=set(self.roots.values())|{'v:'+o for o,x in self.objects.items() if not x['cleaned']}
        headers=set(self.views)
        for p in self.transactions.values():
            if p['kind']=='copy':
                cells.update(p['watch'])
                headers.update(v[1] for v in p['writes'].values() if v[0]=='header')
            if p['kind']=='head':cells.add(p['output'])
        while True:
            old=(len(cells),len(headers))
            for n in list(cells):
                v=self.cells[n]['value']
                if v[0]=='header':headers.add(v[1])
            for h in list(headers):
                if h in self.heads: cells.add('m:'+h)
            if old==(len(cells),len(headers)):break
        return cells,headers

    def settle(self):
        while True:
            live,physical=self.reach()
            for n,c in self.cells.items():c['alive']=n in live
            self.physical=physical
            ordinary,owners,execution=self.counts()
            dead=[n for n,h in self.heads.items() if not h['retired'] and owners[n]==0]
            if not dead:break
            for n in dead:
                self.heads[n]['retired']=True
                self.assign('m:'+n,U)
        # A Mapping's physical destructor invalidates its member Cell again.
        # This is a revision refinement, not another semantic owner release.
        for n,h in self.heads.items():
            if h['kind'] in ('mapping','function') and n not in physical and not h.get('physical_dropped'):
                self.assign('m:'+n,U);h['physical_dropped']=True
        for o,x in self.objects.items():
            if x['state']=='pending' and execution[o]==0:
                x['state']='queued';self.queue.insert(0,o)
        ordinary,_,_=self.counts()
        for o,x in self.objects.items():
            if x['state']=='shell' and ordinary[o]==0:x['state']='freed'

    def reserve(self,values=(),creator=None,new_head=False,extra_exec=None,extra_object=False):
        ordinary,owners,execution=self.counts()
        for v in values:
            if v[0]=='object':
                need(v[1] in self.objects);need(self.objects[v[1]]['state']!='freed','InvalidObject');ordinary[v[1]]+=1
            if v[0]=='header':
                need(v[1] in self.heads and not self.heads[v[1]]['retired']);owners[v[1]]+=1
        if creator is not None:
            need(self.valid(creator),'InvalidObject');ordinary[creator]+=1
        if extra_exec is not None:
            need(self.valid(extra_exec),'InvalidObject');execution[extra_exec]+=1
        required=dict(objects=len(self.objects)+extra_object,heads=sum(h['kind']=='function' for h in self.heads.values())+new_head,
                      ordinary=max((n+int(self.valid(o)) for o,n in ordinary.items()),default=0),
                      exec=max(execution.values(),default=0),owners=max([*owners.values(),int(new_head)],default=0))
        for k,v in required.items():self.peak[k]=max(self.peak[k],v)
        need(required['objects']<=self.q['max_objects'] and required['heads']<=self.q['max_function_heads'],'Limit')
        need(all(n<=self.q['max_ordinary_refs_per_object']-int(self.valid(o)) for o,n in ordinary.items()),'Limit')
        need(all(n<=self.q['max_owners_per_header'] for n in owners.values()),'Limit')
        need(all(n<=self.q['max_exec_refs_per_object'] for n in execution.values()),'Limit')

    def normalize(self,v):
        if v[0]=='object' and not self.valid(v[1]):return U
        if v[0]=='header':
            creator=self.heads[v[1]]['creator']
            if creator is not None and not self.valid(creator):return U
        return v

    def consume(self,tx):
        p=self.transactions.pop(tx,None)
        if p is None:return
        self.settle()
        if p['kind']=='head':
            # No transaction can borrow another one's unpublished output.
            del self.heads[p['head']]
            self.settle()

    def apply(self,a):
        candidate=copy.deepcopy(self)
        try:
            candidate.event(a)
            candidate.settle()
        except Rejected:
            self.peak={k:max(v,candidate.peak[k]) for k,v in self.peak.items()}
            if a['op']=='commit':self.consume(a['tx'])
            raise
        self.__dict__.update(candidate.__dict__)

    def event(self,a):
        op=a['op']
        value=a.get('value')
        header=value[1] if value and value[0]=='header' else a.get('header') if op=='view' else None
        if header is not None:
            need(not any(p['kind']=='head' and p['head']==header for p in self.transactions.values()))
        if op=='object':
            self.reserve(extra_object=True);n=a['id'];self.objects[n]=dict(state='alive',cleaned=False);self.known_objects.append(n);self.new_cell('v:'+n,U)
        elif op=='put':
            n=a['dest'];v=tuple(a['value'])
            try:identity=self.cell(n)
            except Rejected:
                need(n not in self.cells)
                self.reserve([v]);self.new_cell(n,v);self.roots[n]=n
            else:
                self.reserve([v]);need(a.get('borrow')!=n);self.assign(identity,v)
        elif op=='alias_cell':self.roots[a['dest']]=self.cell(a['source'])
        elif op=='drop':self.roots.pop(a['slot'],None)
        elif op=='clear':
            n=self.cell(a['slot']);need(a.get('borrow')!=a['slot']);self.assign(n,U)
        elif op=='make':
            v=tuple(a['value']);self.reserve([v]);n=a['id']
            self.heads[n]=dict(kind=a['kind'],creator=None,retired=False);self.known_heads.add(n)
            self.new_cell('m:'+n,v);dest=a['dest'];self.new_cell('r:'+n,('header',n));self.roots[dest]='r:'+n
        elif op=='prepare':
            tx=a['tx'];need(tx not in self.transactions);kind=a['kind']
            if kind=='copy':
                p=dict(kind=kind,watch={},writes={})
                for source,dest in a['writes']:
                    s=self.cell(source);d=self.cell(dest);v=self.cells[s]['value']
                    if a.get('script'):v=self.normalize(v)
                    p['watch'][s]=self.cells[s]['revision'];p['watch'][d]=self.cells[d]['revision']
                    updates=[(d,v)]
                    if v!=self.cells[s]['value']:updates.append((s,v))
                    for n,v in updates:
                        need(n not in p['writes'] or p['writes'][n]==v);p['writes'][n]=v
                self.transactions[tx]=p
            elif kind=='head':
                creator=a['creator'];v=tuple(a['value']);self.reserve([v],creator,True)
                n=a['head'];self.heads[n]=dict(kind='function',creator=creator,retired=False);self.known_heads.add(n)
                self.new_cell('m:'+n,v);output='p:'+tx;self.new_cell(output,('header',n))
                self.transactions[tx]=dict(kind=kind,head=n,output=output,dest=a['dest'])
            else:
                o=a['object'];self.reserve(extra_exec=o);self.transactions[tx]=dict(kind='exec',object=o)
        elif op in ('commit','abort'):
            tx=a['tx'];need(tx in self.transactions);p=self.transactions[tx]
            if op=='abort':self.consume(tx);return
            if p['kind']=='copy':
                need(all(self.cells[n]['revision']==v for n,v in p['watch'].items()))
                self.reserve(p['writes'].values())
                if a.get('borrow'):
                    target=self.cell(a['borrow']);projected=copy.deepcopy(self)
                    for n,v in p['writes'].items():projected.assign(n,v)
                    projected.settle()
                    need(target not in p['writes'])
                    need(not any(target=='m:'+n and not h['retired'] and projected.heads[n]['retired'] for n,h in self.heads.items()))
                for n,v in p['writes'].items():self.assign(n,v)
                self.settle()  # Plan still holds watched Cells until return.
                self.transactions.pop(tx)
            elif p['kind']=='head':
                need(self.valid(self.heads[p['head']]['creator']),'InvalidObject')
                self.roots[p['dest']]=p['output'];self.transactions.pop(tx)
            else:self.leases['installed:'+tx]=p['object'];self.transactions.pop(tx)
        elif op=='exec':
            self.reserve(extra_exec=a['object']);self.leases[a['id']]=a['object']
        elif op=='exit':self.leases.pop(a['id'],None)
        elif op=='delete':
            o=a['object'];need(self.valid(o),'InvalidObject')
            if self.counts()[2][o]:self.objects[o]['state']='pending'
            else:
                need(a.get('borrow')!='v:'+o)
                self.objects[o]=dict(state='shell',cleaned=True);self.assign('v:'+o,U)
        elif op=='view':
            if a['header'] in self.physical:self.views.add(a['header'])
        elif op=='unview':self.views.discard(a['header'])
        elif op in ('maintain','zero'):
            for _ in range(0 if a.get('workspace_blocked') else a.get('objects',0)):
                if not self.queue:break
                o=self.queue[0]
                if a.get('borrow')=='v:'+o:break
                self.queue.pop(0);self.objects[o]=dict(state='shell',cleaned=True);self.assign('v:'+o,U);self.settle()
        else:raise AssertionError('unknown action '+op)

    def projection(self):
        ordinary,owners,execution=self.counts()
        return dict(cells={n:dict(value=list(c['value']),revision=c['revision']) if c['alive'] else None for n,c in self.cells.items()},roots=self.roots.copy(),
                    objects={n:dict(**self.objects[n],ordinary=ordinary[n],exec=execution[n]) if n in self.objects else None for n in self.known_objects},
                    headers={n:dict(kind=self.heads[n]['kind'],retired=self.heads[n]['retired'],owners=owners[n]) if n in self.heads else None for n in self.known_heads},
                    transactions={n:p['kind'] for n,p in self.transactions.items()},leases=sorted(self.leases),queue=self.queue[:],
                    function_heads_charged=sum(h['kind']=='function' for h in self.heads.values()))
