"""Independent design transition system, not Rust code or a native execution oracle.

Only explicit slots/host roots own payloads. Ref aliases own a slot, not a second
copy of its payload. Every action runs on a private candidate; publication occurs
only after graph checks and a separately implemented ownership audit succeed.
"""
from copy import deepcopy
from dataclasses import asdict, dataclass, field
import json


class Rejected(Exception):
    def __init__(self, code):
        super().__init__(code)
        self.code = code


UNDEFINED = ('undefined', None)
HEADER_TAGS = {'array', 'mapping', 'function'}


@dataclass
class Slot:
    value: tuple = UNDEFINED
    refs: int = 0
    storage: str = 'vm'


@dataclass
class Header:
    kind: str
    cells: list = field(default_factory=list)
    owners: int = 0
    creator: str | None = None
    origin: str = 'store'


@dataclass
class Root:
    cell: int
    kind: str = 'vm'
    readonly: bool = False


class World:
    def __init__(self, max_items=65536, max_depth=64, max_nodes=262144, max_host_roots=65536):
        self.slots = {}
        self.headers = {}
        self.roots = {}
        self.host = {}
        self.pending = set()
        self.objects = {}
        self.borrowed = set()
        self.next_cell = 1
        self.next_header = 1
        self.events = []
        self.max_items = max_items
        self.max_depth = max_depth
        self.max_nodes = max_nodes
        self.max_host_roots = max_host_roots
        self._fail_at = None
        self._checkpoint = 0

    def state(self):
        """Canonical committed state; excludes candidate-local fault machinery."""
        return dict(slots={str(k):asdict(v) for k,v in sorted(self.slots.items())},
                    headers={str(k):asdict(v) for k,v in sorted(self.headers.items())},
                    roots={k:asdict(v) for k,v in sorted(self.roots.items())},
                    host=dict(sorted(self.host.items())), pending=sorted(self.pending), objects=dict(sorted(self.objects.items())),
                    borrowed=sorted(self.borrowed), next_cell=self.next_cell,
                    next_header=self.next_header, events=deepcopy(self.events),
                    limits=[self.max_items,self.max_depth,self.max_nodes,self.max_host_roots])

    def fingerprint(self):
        return json.dumps(self.state(), sort_keys=True, separators=(',',':'))

    def _tick(self):
        current = self._checkpoint
        self._checkpoint += 1
        if self._fail_at == current:
            raise Rejected('Limit')

    def apply(self, action, fail_at=None):
        candidate = deepcopy(self)
        candidate._fail_at = fail_at
        candidate._checkpoint = 0
        if action['op'] != 'retire':
            candidate._tick()
        if candidate.pending and candidate.borrowed and action['op'] not in ('unborrow','retire'):
            raise Rejected('InvalidState')
        result = candidate._dispatch(action)
        candidate._drain()
        candidate._graph()
        audit(candidate)
        if action['op'] != 'retire':
            candidate._tick()  # Last fallible prepublication point in this model.
        checkpoints = candidate._checkpoint
        candidate._fail_at = None
        candidate._checkpoint = 0
        self.__dict__.update(candidate.__dict__)
        return dict(result=result, checkpoints=checkpoints)

    def _drain(self):
        # A retired lease remains a private owning root until the no-borrow epoch.
        if not self.borrowed:
            for name in sorted(self.pending):
                self._drop_value(self.host.pop(name))
            self.pending.clear()

    def _root(self, name):
        if name not in self.roots:
            raise Rejected('InvalidState')
        return self.roots[name]

    def _fresh_root(self, name):
        if name in self.roots or name in self.host:
            raise Rejected('InvalidState')

    def _read(self, cell):
        if cell not in self.slots or cell in self.borrowed:
            raise Rejected('InvalidState')
        return self.slots[cell].value

    def _hold(self, value):
        tag, payload = value
        if tag == 'foreign':
            raise Rejected('ForeignOrigin')
        if tag in HEADER_TAGS:
            if payload not in self.headers or self.headers[payload].kind != tag:
                raise Rejected('InvalidState')
            if self.headers[payload].origin != 'store':
                raise Rejected('ForeignOrigin')
            self.headers[payload].owners += 1

    def _drop_value(self, value):
        tag, payload = value
        if tag in HEADER_TAGS:
            header = self.headers[payload]
            header.owners -= 1
            if header.owners < 0:
                raise AssertionError('negative header owners')
            if header.owners == 0:
                self._release_header(payload)

    def _new_slot(self, value=UNDEFINED, storage='vm'):
        self._tick()
        cell = self.next_cell
        self.next_cell += 1
        self._hold(value)
        self.slots[cell] = Slot(value=value, storage=storage)
        return cell

    def _add_root(self, name, cell, kind='vm', readonly=False):
        self._fresh_root(name)
        self.slots[cell].refs += 1
        self.roots[name] = Root(cell,kind,readonly)

    def _new_root(self, name, value, kind='vm'):
        self._fresh_root(name)
        cell = self._new_slot(value)
        self._add_root(name,cell,kind)
        return cell

    def _unref_cell(self, cell):
        slot = self.slots[cell]
        slot.refs -= 1
        if slot.refs < 0:
            raise AssertionError('negative slot refs')
        if slot.refs == 0:
            if cell in self.borrowed:
                raise Rejected('InvalidState')
            value = slot.value
            del self.slots[cell]
            self.events.append(['release-cell',cell])
            self._drop_value(value)

    def _drop_root(self, name):
        root = self.roots.pop(name)
        self._unref_cell(root.cell)

    def _release_header(self, identity):
        header = self.headers[identity]
        cells = list(header.cells)
        if any(c in self.borrowed for c in cells):
            raise Rejected('InvalidState')
        # Mapping payload clearing precedes deferred cell/node lifetime.
        if header.kind == 'mapping':
            old = [self.slots[c].value for c in cells]
            for cell in cells:
                self.slots[cell].value = UNDEFINED
            for value in old:
                self._drop_value(value)
        del self.headers[identity]
        self.events.append(['release-header',identity])
        for cell in cells:
            self._unref_cell(cell)

    def _writes(self, updates):
        """Acquire/check all destinations, retain all new payloads, then release old."""
        merged = {}
        for cell, value in updates:
            if cell in merged and merged[cell] != value:
                raise Rejected('ConflictingWrite')
            merged[cell] = value
        self._tick()
        for cell in merged:
            self._read(cell)
        old = [(cell,self.slots[cell].value) for cell in merged]
        for value in merged.values():
            self._hold(value)
        for cell, value in merged.items():
            self.slots[cell].value = value
        self._graph()  # Final proposed edges, not old graph plus proposed edges.
        for cell, value in old:
            if value != merged[cell]:
                self.events.append(['write',cell])
            self._drop_value(value)

    def _normalized(self, value):
        tag, payload = value
        if tag == 'foreign':
            raise Rejected('ForeignOrigin')
        dead = tag == 'object' and not self.objects.get(payload,False)
        if tag == 'function':
            h = self.headers[payload]
            if h.origin != 'store':
                raise Rejected('ForeignOrigin')
            dead = h.creator is not None and not self.objects.get(h.creator,False)
        return UNDEFINED if dead else value

    def _copy_cells(self, sources, destinations):
        updates = []
        for src, dst in zip(sources,destinations):
            raw = self._read(src)
            value = self._normalized(raw)
            if raw != value:
                updates.append((src,value))
                self.events.append(['normalize',src])
            updates.append((dst,value))
        self._writes(updates)

    def _header(self, kind, values, creator=None):
        if len(values) > self.max_items or (kind == 'function' and len(values) > 63):
            raise Rejected('Limit')
        if kind == 'function' and creator is not None and not self.objects.get(creator,False):
            raise Rejected('InvalidObject')
        self._tick()
        identity = self.next_header
        self.next_header += 1
        header = Header(kind,creator=creator)
        self.headers[identity] = header
        for value in values:
            cell = self._new_slot(value,'capture' if kind=='function' else 'member')
            self.slots[cell].refs = 1  # Single edge from the new header.
            header.cells.append(cell)
        return (kind,identity)

    def _graph(self):
        """Post-state DFS: sharing is allowed, active-path recurrence is a cycle."""
        if len(self.slots)+len(self.headers) > self.max_nodes:
            raise Rejected('Limit')
        active, memo = set(), {}
        def visit(identity):
            if identity in active:
                raise Rejected('Cycle')
            if identity in memo:
                return memo[identity]
            active.add(identity)
            h = self.headers[identity]
            if len(h.cells) > self.max_items:
                raise Rejected('Limit')
            children = [self.slots[c].value[1] for c in h.cells if self.slots[c].value[0] in HEADER_TAGS]
            depth = 0 if h.kind=='function' and not children else 1
            for cell in h.cells:
                tag, payload = self.slots[cell].value
                if tag in HEADER_TAGS:
                    depth = max(depth,1+visit(payload))
            active.remove(identity)
            if depth > self.max_depth:
                raise Rejected('Limit')
            memo[identity] = depth
            return depth
        for identity in sorted(self.headers):
            visit(identity)

    def _invoke(self, action):
        name = action['root']
        source = self._root(name)
        if source.kind == 'ref':
            raise Rejected('UnsupportedShape')
        tag, identity = self._read(source.cell)
        if tag != 'function':
            raise Rejected('UnsupportedType')
        header = self.headers[identity]
        if header.origin != 'store':
            raise Rejected('ForeignOrigin')
        if header.creator is not None and not self.objects.get(header.creator,False):
            raise Rejected('InvalidObject')
        output = action['outputs']
        new = action.get('new',[])
        if len(set(new)) != len(new) or name in new:
            raise Rejected('UnsupportedShape')
        for n in new:
            if self._root(n).kind == 'ref':
                raise Rejected('UnsupportedShape')
            self._read(self.roots[n].cell)
        if len(header.cells)+len(new) > 127:
            raise Rejected('ArgumentCount')
        if len(output) != len(header.cells) or len(set(output)) != len(output):
            raise Rejected('ArgumentCount')
        for n in output:
            self._fresh_root(n)
        mode = 'unique' if header.owners == 1 else 'shared'
        # Destination reservation precedes any source update in the Rust policy.
        destinations = [self._new_root(n,UNDEFINED) for n in output]
        activation = self._new_root('@activation',UNDEFINED,'activation')
        self._writes([(activation,('function',identity)),(source.cell,UNDEFINED)])
        self._drop_root(name)
        if mode == 'unique':
            values = [self._read(c) for c in header.cells]
            self._writes(list(zip(destinations,values))+[(c,UNDEFINED) for c in header.cells])
            self._drop_root('@activation')
        else:
            cells = list(header.cells)
            self._drop_root('@activation')  # Another semantic owner keeps header/captures alive.
            self._copy_cells(cells,destinations)
        self.events.append(['arguments',output+new])
        self.events.append(['invoke',mode])
        return mode

    def _dispatch(self, a):
        op = a['op']
        if op == 'put':
            value = tuple(a.get('value',UNDEFINED))
            if value[0] not in {'undefined','void','int','real','bytes','object','foreign'}:
                raise Rejected('UnsupportedShape')  # Headers are opaque, never forged IDs.
            self._new_root(a['root'],value)
        elif op == 'object':
            if a['id'] in self.objects:
                raise Rejected('InvalidState')
            self.objects[a['id']] = True
        elif op == 'delete-object':
            if a['id'] not in self.objects:
                raise Rejected('InvalidObject')
            self.objects[a['id']] = False
        elif op == 'build':
            kind = a['kind']
            if kind not in HEADER_TAGS:
                raise Rejected('UnsupportedType')
            names = a['sources']
            if len(names) != len(set(names)) or a['root'] in names:
                raise Rejected('UnsupportedShape')
            sources = [self._root(n) for n in names]
            if any(r.kind=='ref' for r in sources):
                raise Rejected('UnsupportedShape')
            # Model build consumes source roots (17 raw move / container construction).
            values = [self._read(r.cell) for r in sources]
            value = self._header(kind,values,a.get('creator'))
            self._new_root(a['root'],value)
            self._writes([(r.cell,UNDEFINED) for r in sources])
            for n in names:
                self._drop_root(n)
        elif op in ('copy','move','deref'):
            source = self._root(a['source'])
            if op == 'deref' and source.kind != 'ref':
                raise Rejected('UnsupportedShape')
            if op == 'move' and source.kind == 'ref':
                raise Rejected('UnsupportedShape')
            dst = self._new_root(a['root'],UNDEFINED)
            if op in ('copy','deref'):
                self._copy_cells([source.cell],[dst])
                if op == 'deref':
                    self._drop_root(a['source'])
            else:
                raw = self._read(source.cell)
                self._writes([(dst,raw),(source.cell,UNDEFINED)])
                self._drop_root(a['source'])
        elif op == 'alias':
            src = self._root(a['source'])
            if self.slots[src.cell].storage == 'capture':
                raise Rejected('UnsupportedShape')
            self._add_root(a['root'],src.cell,'ref',a.get('readonly',False))
        elif op in ('member-ref','member-copy'):
            tag, identity = self._read(self._root(a['source']).cell)
            if tag not in {'array','mapping'}:
                raise Rejected('UnsupportedType')
            cells = self.headers[identity].cells
            index = a['index']
            if not isinstance(index,int) or not 0 <= index < len(cells):
                raise Rejected('Bounds')
            if op == 'member-ref':
                self._add_root(a['root'],cells[index],'ref',a.get('readonly',False))
            else:
                dst = self._new_root(a['root'],UNDEFINED)
                self._copy_cells([cells[index]],[dst])
        elif op == 'write':
            target = self._root(a['target'])
            if target.readonly:
                raise Rejected('Readonly')
            raw = self._read(self._root(a['source']).cell)
            if self.slots[target.cell].storage == 'member' and raw[0] in ('array','mapping'):
                raise Rejected('UnsupportedType')
            self._writes([(target.cell,raw)])
        elif op == 'batch-copy':
            roots = [self._root(n) for n in a['targets']]
            if any(r.readonly for r in roots):
                raise Rejected('Readonly')
            sources = [self._root(n).cell for n in a['sources']]
            if len(sources)!=len(roots):
                raise Rejected('ArgumentCount')
            for src, target in zip(sources,roots):
                if self.slots[target.cell].storage=='member' and self._read(src)[0] in ('array','mapping'):
                    raise Rejected('UnsupportedType')
            self._copy_cells(sources,[r.cell for r in roots])
        elif op == 'drop':
            self._root(a['root'])
            self._drop_root(a['root'])
        elif op == 'export':
            if len(self.host)>=self.max_host_roots:
                raise Rejected('Limit')
            self._fresh_root(a['lease'])
            value = self._read(self._root(a['source']).cell)
            self._hold(value)
            self.host[a['lease']] = value
        elif op == 'retain':
            if len(self.host)>=self.max_host_roots:
                raise Rejected('Limit')
            self._fresh_root(a['lease'])
            if a['source'] not in self.host or a['source'] in self.pending:
                raise Rejected('InvalidState')
            value = self.host[a['source']]
            self._hold(value)
            self.host[a['lease']] = value
        elif op == 'import':
            if a['lease'] not in self.host or a['lease'] in self.pending:
                raise Rejected('InvalidState')
            self._new_root(a['root'],self.host[a['lease']])
            # Explicit raw ownership transport. Entry COPY is a separate operation.
        elif op == 'retire':
            if a['lease'] not in self.host or a['lease'] in self.pending:
                raise Rejected('InvalidState')
            self.pending.add(a['lease'])
        elif op == 'release':
            if a['lease'] not in self.host or a['lease'] in self.pending:
                raise Rejected('InvalidState')
            self._drop_value(self.host.pop(a['lease']))
        elif op == 'invoke':
            return self._invoke(a)
        elif op in ('borrow','unborrow'):
            cell = self._root(a['root']).cell
            if op == 'borrow':
                if cell in self.borrowed:
                    raise Rejected('InvalidState')
                self.borrowed.add(cell)
            else:
                if cell not in self.borrowed:
                    raise Rejected('InvalidState')
                self.borrowed.remove(cell)
        elif op == 'observe':
            return self.observe(a['root'])
        else:
            raise Rejected('UnsupportedAction')
        return None

    def observe(self, name):
        """Frozen, non-callable metadata; no Value/Header/Cell handle escapes."""
        root = self._root(name)
        seen = set()
        def view(value):
            tag,payload = value
            if tag not in HEADER_TAGS:
                return (tag,payload)
            if payload in seen:
                return ('alias',payload)
            seen.add(payload)
            h = self.headers[payload]
            if tag == 'function':
                return ('function',payload,h.creator,len(h.cells))  # Captures remain hidden.
            return (tag,payload,tuple(view(self._read(c)) for c in h.cells))
        return view(self._read(root.cell))


def audit(world):
    """Independent recount, from owning relations rather than transition counters."""
    slot_refs = {c:0 for c in world.slots}
    owners = {h:0 for h in world.headers}
    for root in world.roots.values():
        assert root.cell in slot_refs, 'root targets released cell'
        slot_refs[root.cell] += 1
    for h in world.headers.values():
        assert len(h.cells)==len(set(h.cells)), 'duplicate owned capture/member slot'
        for cell in h.cells:
            assert cell in slot_refs, 'header targets released cell'
            slot_refs[cell] += 1
    values = [s.value for s in world.slots.values()]+list(world.host.values())
    for tag, identity in values:
        if tag in HEADER_TAGS:
            assert identity in owners, 'payload targets released header'
            assert world.headers[identity].kind==tag, 'header type mismatch'
            owners[identity] += 1
    for cell, expected in slot_refs.items():
        assert expected>0 and world.slots[cell].refs==expected, 'slot ref ledger differs'
    for identity, expected in owners.items():
        assert expected>0 and world.headers[identity].owners==expected, 'header owner ledger differs'
    assert world.borrowed <= set(world.slots), 'borrow outlives cell'
    assert world.pending <= set(world.host), 'pending release lacks owning registry entry'
    released = [e[1] for e in world.events if e[0]=='release-header']
    assert len(released)==len(set(released)), 'header released twice'
    assert not set(released)&set(world.headers), 'released header still live'
    # Independent cycle oracle: Kahn elimination, not the mutation DFS.
    edges = {identity:set() for identity in world.headers}
    incoming = {identity:0 for identity in world.headers}
    for identity, h in world.headers.items():
        for cell in h.cells:
            tag, child = world.slots[cell].value
            if tag in HEADER_TAGS:
                edges[identity].add(child)
        for child in edges[identity]:
            incoming[child] += 1
    queue = [identity for identity,n in incoming.items() if n==0]
    removed = 0
    while queue:
        identity = queue.pop()
        removed += 1
        for child in edges[identity]:
            incoming[child] -= 1
            if incoming[child]==0:
                queue.append(child)
    assert removed==len(world.headers), 'cycle escaped post-state validation'
