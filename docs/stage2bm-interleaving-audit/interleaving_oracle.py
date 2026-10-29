"""Declarative slot graph and object state machine, independent of Rust counters.

CopyPlan snapshots are physical values, not prospective semantic owners. A staged
Function output and an execution reservation do own resources. D08's abstract
reserve_copy is therefore not applied at the earlier CopyPlan construction point.
"""
import copy
from collections import Counter

U = ('undefined',)

class Rejected(Exception):
    def __init__(self, kind):
        self.kind = kind
        super().__init__(kind)

def need(ok, kind='InvalidState'):
    if not ok:
        raise Rejected(kind)

class Reference:
    def __init__(self, limits):
        self.q = limits
        self.objects = {}
        self.heads = {}
        self.slots = {}
        self.known = set()
        self.versions = {}
        self.execs = {}
        self.queue = []
        self.views = set()
        self.plan = None
        self.pending = None
        self.known_heads = set()
        self.known_objects = set()
        self.object_order = []
        self.peak = dict(objects=0, heads=0, ordinary=0, exec=0, owners=0)

    def known_objects_order(self):
        return self.object_order

    def counts(self):
        ordinary = Counter()
        owners = Counter()
        for value in self.slots.values():
            if value[0] == 'object': ordinary[value[1]] += 1
            if value[0] == 'header': owners[value[1]] += 1
        for h in self.heads.values():
            if not h['retired'] and h['creator'] is not None:
                ordinary[h['creator']] += 1
        ordinary.update(self.queue)
        return ordinary, owners

    def valid(self, obj):
        return obj in self.objects and self.objects[obj]['state'] in ('alive', 'pending')

    def reserve(self, values=(), creator=None, kind=None):
        ordinary, owners = self.counts()
        for value in values:
            if value[0] == 'object':
                need(value[1] in self.objects)
                need(self.objects[value[1]]['state'] != 'freed', 'InvalidObject')
                ordinary[value[1]] += 1
            if value[0] == 'header':
                need(value[1] in self.heads and not self.heads[value[1]]['retired'])
                owners[value[1]] += 1
        if creator is not None:
            need(self.valid(creator), 'InvalidObject')
            ordinary[creator] += 1
        heads = sum(h['kind'] == 'function' for h in self.heads.values()) + (kind == 'function')
        # Alive/Pending reserve one ordinary unit for eventual queue ownership.
        required = dict(objects=len(self.objects), heads=heads,
                        ordinary=max((n + self.valid(o) for o, n in ordinary.items()), default=0),
                        exec=max(Counter(self.execs.values()).values(), default=0),
                        owners=max([*owners.values(), int(kind is not None)], default=0))
        for k, n in required.items(): self.peak[k] = max(self.peak[k], n)
        need(heads <= self.q['max_function_heads'], 'Limit')
        need(all(n <= self.q['max_ordinary_refs_per_object'] - self.valid(o) for o, n in ordinary.items()), 'Limit')
        need(all(n <= self.q['max_owners_per_header'] for n in owners.values()), 'Limit')

    def assign(self, slot, value):
        self.known.add(slot)
        self.slots[slot] = tuple(value)
        self.versions[slot] = self.versions.get(slot, 0) + 1

    def settle(self):
        while True:
            ordinary, owners = self.counts()
            dead = [n for n, h in self.heads.items() if not h['retired'] and not owners[n]]
            if not dead: break
            for n in dead:
                self.heads[n]['retired'] = True
                self.assign('m:' + n, U)
        ordinary, _ = self.counts()
        for n, obj in self.objects.items():
            if obj['state'] == 'shell' and not ordinary[n]: obj['state'] = 'freed'

    def normalize(self, value):
        if value[0] == 'object' and not self.valid(value[1]): return U
        if value[0] == 'header':
            creator = self.heads[value[1]]['creator']
            if creator is not None and not self.valid(creator): return U
        return value

    def apply(self, action):
        candidate = copy.deepcopy(self)
        try:
            candidate.event(action)
        except Rejected:
            self.peak = {k: max(v, candidate.peak[k]) for k, v in self.peak.items()}
            # Actual commit consumes its CopyPlan even when validation fails.
            if action['op'] == 'commit':
                self.plan = None
                self.slots.pop('tmp:move', None)
            if action['op'] == 'commit_head':
                self.slots.pop('tmp:head', None)
                if self.pending:
                    n, _ = self.pending
                    self.settle()
                    del self.heads[n]  # guard abort returns unpublished head capacity
                self.pending = None
            raise
        self.__dict__.update(candidate.__dict__)

    def event(self, a):
        op = a['op']
        if op == 'object':
            self.peak['objects'] = max(self.peak['objects'], len(self.objects) + 1)
            need(len(self.objects) < self.q['max_objects'], 'Limit')
            self.objects[a['id']] = dict(state='alive', cleaned=False)
            self.known_objects.add(a['id'])
            self.object_order.append(a['id'])
            self.assign('v:' + a['id'], U)
        elif op in ('make', 'prepare_head'):
            kind, creator = a['kind'], a.get('creator')
            value = tuple(a['value'])
            self.reserve([value], creator, kind)
            n = a['id']
            self.heads[n] = dict(kind=kind, creator=creator, retired=False)
            self.known_heads.add(n)
            self.assign('m:' + n, value)
            self.assign('tmp:head' if op == 'prepare_head' else a['dest'], ('header', n))
            if op == 'prepare_head': self.pending = (n, a['dest'])
        elif op == 'put':
            value = tuple(a['value'])
            # StoreState::cell normalizes a freed ID when creating a new slot;
            # replacing an existing slot instead attempts a raw lease and rejects.
            if a['dest'] not in self.slots and value[0] == 'object' and self.objects.get(value[1], {}).get('state') == 'freed': value = U
            self.reserve([value])
            self.assign(a['dest'], value)
            self.settle()
        elif op == 'drop':
            if a['slot'].startswith(('m:', 'v:')): self.assign(a['slot'], U)
            else: self.slots.pop(a['slot'], None)
            self.settle()
        elif op in ('view', 'unview'):
            if op == 'unview': self.views.discard(a['header'])
            elif a['header'] in self.heads and not self.heads[a['header']]['retired']: self.views.add(a['header'])
        elif op == 'prepare':
            self.plan = dict(watch={}, writes={}, raw=a.get('raw', False), move=None)
            for source, dest in a['writes']:
                value = self.slots.get(source, U)
                if a.get('script'): value = self.normalize(value)
                for slot in (source, dest): self.plan['watch'][slot] = self.versions[slot]
                updates = [(dest, value)]
                if value != self.slots.get(source, U): updates.append((source, value))
                for slot, v in updates:
                    need(slot not in self.plan['writes'] or self.plan['writes'][slot] == v)
                    self.plan['writes'][slot] = v
        elif op == 'prepare_move':
            source = a['source']
            self.assign('tmp:move', U)
            self.plan = dict(watch={source:self.versions[source]}, writes={}, raw=False, move=(source,a['dest']))
        elif op == 'commit':
            p = self.plan
            need(p is not None)
            if p['raw']:
                need(all(self.slots[d] == U for d in p['writes']))
            else:
                need(all(self.versions[n] == v for n, v in p['watch'].items()))
            self.reserve(p['writes'].values() if not p['raw'] else ())
            if a.get('borrow'):
                projected = copy.deepcopy(self)
                for dest, value in p['writes'].items(): projected.assign(dest, value)
                projected.settle()
                target = a['borrow']
                need(target not in p['writes'] and self.slots.get(target, U) == projected.slots.get(target, U))
                need(not any(target=='m:'+n and not h['retired'] and projected.heads[n]['retired'] for n,h in self.heads.items()))
            for dest, value in p['writes'].items(): self.assign(dest, value)
            if p['move']:
                source, dest = p['move']
                need(not a.get('borrow') or a['borrow'] != source)
                self.assign(dest, self.slots[source])
                self.assign(source, U)
                self.slots.pop('tmp:move', None)
            self.plan = None
            self.settle()
        elif op == 'abort':
            self.plan = None
            self.slots.pop('tmp:move', None)
        elif op == 'commit_head':
            n, dest = self.pending
            creator = self.heads[n]['creator']
            need(creator is None or self.valid(creator), 'InvalidObject')
            self.assign(dest, self.slots.pop('tmp:head'))
            self.pending = None
            self.settle()
        elif op == 'abort_head':
            n, _ = self.pending
            self.slots.pop('tmp:head')
            self.settle()
            del self.heads[n]
            self.pending = None
        elif op in ('exec', 'prepare_exec'):
            o = a['object']
            need(self.valid(o), 'InvalidObject')
            n = Counter(self.execs.values())[o] + 1
            self.peak['exec'] = max(self.peak['exec'], n)
            need(n <= self.q['max_exec_refs_per_object'], 'Limit')
            self.execs[a['id']] = o
        elif op == 'commit_exec':
            self.execs[a['dest']] = self.execs.pop(a['id'])
        elif op in ('exit', 'abort_exec'):
            o = self.execs.pop(a['id'], None)
            if o is not None and self.objects[o]['state'] == 'pending' and o not in self.execs.values():
                self.objects[o]['state'] = 'queued'
                self.queue.insert(0, o)
        elif op == 'delete':
            o = a['object']
            need(self.valid(o), 'InvalidObject')
            if o in self.execs.values(): self.objects[o]['state'] = 'pending'
            else:
                need(a.get('borrow') != 'v:' + o)
                self.objects[o] = dict(state='shell', cleaned=True)
                self.assign('v:' + o, U)
                self.settle()
        elif op in ('zero', 'maintain'):
            if op == 'zero': return
            for _ in range(0 if a.get('workspace_blocked') else a.get('objects', 0)):
                if not self.queue: break
                o = self.queue[0]
                if a.get('borrow') == 'v:' + o: break
                self.queue.pop(0)
                self.objects[o] = dict(state='shell', cleaned=True)
                self.assign('v:' + o, U)
                self.settle()
            # Concrete metadata is verified by the separately frozen snapshot
            # oracle, then projected back via the validated visited key set.
        else:
            raise AssertionError('unknown action ' + op)

    def projection(self):
        ordinary, owners = self.counts()
        execs = Counter(self.execs.values())
        return dict(slots={n:list(self.slots.get(n,U)) for n in sorted(self.known)},
                    objects={n:dict(**self.objects[n],ordinary=ordinary[n],exec=execs[n]) if n in self.objects else None for n in sorted(self.known_objects)},
                    headers={n:dict(kind=self.heads[n]['kind'],owners=owners[n],retired=self.heads[n]['retired']) if n in self.heads else None for n in sorted(self.known_heads)},
                    function_heads_charged=sum(h['kind']=='function' for h in self.heads.values()),queue=self.queue[:])
