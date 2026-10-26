//! D01 prototype. Internal leases own counters, never arena/control back-links.
use rml32::VmErrorKind;
use serde_json::{Value, json};
use std::cell::{Cell, RefCell};
use std::collections::{BTreeMap, BTreeSet};
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct KernelError {
    kind: VmErrorKind,
    message: String,
}
impl std::fmt::Display for KernelError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}: {}", self.kind, self.message)
    }
}
impl std::error::Error for KernelError {}
type Result<T> = std::result::Result<T, KernelError>;
fn invalid(message: &str) -> KernelError {
    KernelError {
        kind: VmErrorKind::InvalidState,
        message: message.into(),
    }
}
fn limit(message: &str) -> KernelError {
    KernelError {
        kind: VmErrorKind::Limit,
        message: message.into(),
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum State {
    Alive,
    Pending,
    Queued,
    Shell,
    Freed,
}
impl State {
    fn name(self) -> &'static str {
        match self {
            Self::Alive => "alive",
            Self::Pending => "pending",
            Self::Queued => "queued",
            Self::Shell => "shell",
            Self::Freed => "freed",
        }
    }
    fn valid(self) -> bool {
        matches!(self, Self::Alive | Self::Pending)
    }
}
#[derive(Default)]
struct Stats {
    objects: Cell<usize>,
    heads: Cell<usize>,
    controls: Cell<usize>,
}
struct Counts {
    id: String,
    origin: Rc<()>,
    ordinary: Cell<usize>,
    execs: Cell<usize>,
    state: Cell<State>,
    cleaned: Cell<u8>,
    queued: Cell<bool>,
    next: RefCell<Option<Rc<Counts>>>,
    capacity: usize,
}
impl Counts {
    fn view(&self) -> Value {
        json!({"state":self.state.get().name(),"refs":self.ordinary.get(),"execs":self.execs.get(),"cleaned":self.cleaned.get()})
    }
    fn unref(&self) {
        self.ordinary.set(
            self.ordinary
                .get()
                .checked_sub(1)
                .expect("ordinary underflow"),
        );
        if self.state.get() == State::Shell && self.ordinary.get() == 0 {
            self.state.set(State::Freed);
        }
    }
}
struct Ordinary(Rc<Counts>);
impl Ordinary {
    fn acquire(c: &Rc<Counts>) -> Result<Self> {
        let n = c
            .ordinary
            .get()
            .checked_add(1)
            .ok_or_else(|| limit("ordinary overflow"))?;
        // One count is always reserved for the allocation-free queue transition.
        if n >= c.capacity {
            return Err(limit("ordinary capacity reserved for queue"));
        }
        c.ordinary.set(n);
        Ok(Self(c.clone()))
    }
}
impl Drop for Ordinary {
    fn drop(&mut self) {
        self.0.unref();
    }
}
#[derive(Default)]
struct Queue {
    head: RefCell<Option<Rc<Counts>>>,
}
struct QueueHold(Rc<Counts>);
impl Drop for QueueHold {
    fn drop(&mut self) {
        self.0.unref();
    }
}
impl Queue {
    fn push(&self, c: &Rc<Counts>) {
        assert!(!c.queued.replace(true), "double enqueue");
        assert_eq!(c.state.get(), State::Pending);
        assert_eq!(c.execs.get(), 0);
        let n = c
            .ordinary
            .get()
            .checked_add(1)
            .expect("reserved queue count");
        assert!(n <= c.capacity);
        c.ordinary.set(n);
        c.state.set(State::Queued);
        *c.next.borrow_mut() = self.head.borrow_mut().take();
        *self.head.borrow_mut() = Some(c.clone());
    }
    fn pop(&self) -> Option<QueueHold> {
        let c = self.head.borrow_mut().take()?;
        *self.head.borrow_mut() = c.next.borrow_mut().take();
        assert!(c.queued.replace(false));
        Some(QueueHold(c))
    }
    fn nodes(&self) -> Vec<Rc<Counts>> {
        let mut out = Vec::new();
        let mut cursor = self.head.borrow().clone();
        let mut seen = BTreeSet::new();
        while let Some(c) = cursor {
            assert!(seen.insert(c.id.clone()), "queue cycle");
            cursor = c.next.borrow().clone();
            out.push(c);
        }
        out
    }
}
impl Drop for Queue {
    fn drop(&mut self) {
        while let Some(hold) = self.pop() {
            hold.0.state.set(State::Shell);
            drop(hold);
        }
    }
}
struct ExecLease {
    counts: Rc<Counts>,
    queue: Rc<Queue>,
}
impl ExecLease {
    fn acquire(c: &Rc<Counts>, q: &Rc<Queue>) -> Result<Self> {
        if !c.state.get().valid() {
            return Err(invalid("invalid execution object"));
        }
        let n = c
            .execs
            .get()
            .checked_add(1)
            .ok_or_else(|| limit("execution overflow"))?;
        if n > c.capacity {
            return Err(limit("execution capacity"));
        }
        c.execs.set(n);
        Ok(Self {
            counts: c.clone(),
            queue: q.clone(),
        })
    }
}
impl Drop for ExecLease {
    fn drop(&mut self) {
        let c = &self.counts;
        c.execs
            .set(c.execs.get().checked_sub(1).expect("execution underflow"));
        if c.execs.get() == 0 && c.state.get() == State::Pending {
            self.queue.push(c);
        }
    }
}
struct Head {
    id: String,
    origin: Rc<()>,
    creator: RefCell<Option<Ordinary>>,
    captures: RefCell<Vec<(String, Rc<Slot>)>>,
    owners: Cell<usize>,
    capacity: usize,
    stats: Rc<Stats>,
}
impl Drop for Head {
    fn drop(&mut self) {
        self.stats.heads.set(self.stats.heads.get() + 1);
    }
}
struct FunctionLease(Rc<Head>);
impl FunctionLease {
    fn acquire(h: &Rc<Head>) -> Result<Self> {
        let n = h
            .owners
            .get()
            .checked_add(1)
            .ok_or_else(|| limit("head overflow"))?;
        if n > h.capacity {
            return Err(limit("head capacity"));
        }
        h.owners.set(n);
        Ok(Self(h.clone()))
    }
}
impl Drop for FunctionLease {
    fn drop(&mut self) {
        let h = &self.0;
        h.owners
            .set(h.owners.get().checked_sub(1).expect("head underflow"));
        if h.owners.get() == 0 {
            let captures = std::mem::take(&mut *h.captures.borrow_mut());
            for (_, s) in captures {
                s.clear();
            }
            // Captured payload release precedes the one creator release.
            drop(h.creator.borrow_mut().take());
        }
    }
}
// Raw views are physical observation handles, never semantic owners.
#[derive(Clone)]
enum Raw {
    Undefined,
    Int(i32),
    Object(Rc<Counts>),
    Function(Rc<Head>),
}
enum Payload {
    Undefined,
    Int(i32),
    Object(Ordinary),
    Function(FunctionLease),
}
impl Payload {
    fn raw(&self) -> Raw {
        match self {
            Self::Undefined => Raw::Undefined,
            Self::Int(n) => Raw::Int(*n),
            Self::Object(o) => Raw::Object(o.0.clone()),
            Self::Function(f) => Raw::Function(f.0.clone()),
        }
    }
    fn acquire(v: &Raw) -> Result<Self> {
        Ok(match v {
            Raw::Undefined => Self::Undefined,
            Raw::Int(n) => Self::Int(*n),
            Raw::Object(c) => Self::Object(Ordinary::acquire(c)?),
            Raw::Function(h) => Self::Function(FunctionLease::acquire(h)?),
        })
    }
    fn view(&self) -> Value {
        self.raw().view()
    }
}
impl Raw {
    fn view(&self) -> Value {
        match self {
            Self::Undefined => json!(["undefined", null]),
            Self::Int(n) => json!(["int", n]),
            Self::Object(c) => json!(["object", c.id]),
            Self::Function(h) => json!(["function", h.id]),
        }
    }
    fn invalid(&self) -> bool {
        match self {
            Self::Object(c) => !c.state.get().valid(),
            Self::Function(h) => h
                .creator
                .borrow()
                .as_ref()
                .is_some_and(|o| !o.0.state.get().valid()),
            _ => false,
        }
    }
}
struct Slot {
    payload: RefCell<Payload>,
}
impl Slot {
    fn new(p: Payload) -> Rc<Self> {
        Rc::new(Self {
            payload: RefCell::new(p),
        })
    }
    fn raw(&self) -> Result<Raw> {
        Ok(self
            .payload
            .try_borrow()
            .map_err(|_| invalid("slot borrowed"))?
            .raw())
    }
    fn clear(&self) {
        let old = self.payload.replace(Payload::Undefined);
        drop(old);
    }
}
struct ObjectRecord {
    counts: Rc<Counts>,
    variables: BTreeMap<String, Rc<Slot>>,
    stats: Rc<Stats>,
}
impl Drop for ObjectRecord {
    fn drop(&mut self) {
        self.stats.objects.set(self.stats.objects.get() + 1);
    }
}
struct Control {
    origin: Rc<()>,
    closed: Cell<bool>,
    transferred: Cell<bool>,
    arena: RefCell<BTreeMap<String, ObjectRecord>>,
    history: RefCell<BTreeMap<String, Value>>,
    roots: RefCell<BTreeMap<String, Rc<Slot>>>,
    heads: RefCell<BTreeMap<String, Weak<Head>>>,
    frames: RefCell<BTreeMap<String, ExecLease>>,
    frame_order: RefCell<Vec<String>>,
    aliases: RefCell<BTreeMap<String, (String, Weak<Slot>)>>,
    retired: RefCell<BTreeMap<String, Rc<Cell<bool>>>>,
    queue: Rc<Queue>,
    capacity: usize,
    stats: Rc<Stats>,
}
impl Drop for Control {
    fn drop(&mut self) {
        // No user guard can outlive the last Rc<Control>. No callbacks run here.
        for name in std::mem::take(self.frame_order.get_mut()) {
            drop(self.frames.get_mut().remove(&name));
        }
        self.frames.get_mut().clear();
        for (_, s) in std::mem::take(self.roots.get_mut()) {
            s.clear();
        }
        for record in self.arena.get_mut().values_mut() {
            for (_, s) in std::mem::take(&mut record.variables) {
                s.clear();
            }
        }
        while let Some(q) = self.queue.pop() {
            q.0.state.set(State::Shell);
            drop(q);
        }
        self.arena.get_mut().clear();
        self.stats.controls.set(self.stats.controls.get() + 1);
    }
}
struct Kernel {
    control: Rc<Control>,
}
impl Drop for Kernel {
    fn drop(&mut self) {
        self.control.closed.set(true);
    }
}
struct HostRoot {
    control: Rc<Control>,
    name: String,
    retired: Rc<Cell<bool>>,
}
impl Drop for HostRoot {
    fn drop(&mut self) {
        self.retired.set(true);
    }
}
struct Fault {
    at: Option<usize>,
    steps: usize,
}
impl Fault {
    fn hit(&mut self) -> Result<()> {
        let i = self.steps;
        self.steps += 1;
        if self.at == Some(i) {
            Err(limit("injected preflight failure"))
        } else {
            Ok(())
        }
    }
}
impl Kernel {
    fn new(capacity: usize) -> Result<Self> {
        if capacity < 2 {
            return Err(limit("capacity requires a queue reserve"));
        }
        Ok(Self {
            control: Rc::new(Control {
                origin: Rc::new(()),
                closed: Cell::new(false),
                transferred: Cell::new(false),
                arena: RefCell::new(BTreeMap::new()),
                history: RefCell::new(BTreeMap::new()),
                roots: RefCell::new(BTreeMap::new()),
                heads: RefCell::new(BTreeMap::new()),
                frames: RefCell::new(BTreeMap::new()),
                frame_order: RefCell::new(Vec::new()),
                aliases: RefCell::new(BTreeMap::new()),
                retired: RefCell::new(BTreeMap::new()),
                queue: Rc::new(Queue::default()),
                capacity,
                stats: Rc::new(Stats::default()),
            }),
        })
    }
    fn counts(&self, id: &str) -> Result<Rc<Counts>> {
        self.control
            .arena
            .borrow()
            .get(id)
            .map(|r| r.counts.clone())
            .ok_or_else(|| invalid("missing object"))
    }
    fn valid_counts(&self, id: &str) -> Result<Rc<Counts>> {
        let c = self.counts(id)?;
        if !c.state.get().valid() {
            return Err(invalid("invalid object"));
        }
        Ok(c)
    }
    fn head(&self, id: &str) -> Result<Rc<Head>> {
        self.control
            .heads
            .borrow()
            .get(id)
            .and_then(Weak::upgrade)
            .filter(|h| h.owners.get() > 0)
            .ok_or_else(|| invalid("missing head"))
    }
    fn all_slots(&self) -> BTreeMap<String, Rc<Slot>> {
        let mut out = self.control.roots.borrow().clone();
        for r in self.control.arena.borrow().values() {
            out.extend(r.variables.clone());
        }
        for h in self
            .control
            .heads
            .borrow()
            .values()
            .filter_map(Weak::upgrade)
        {
            out.extend(h.captures.borrow().iter().cloned());
        }
        out
    }
    fn slot(&self, n: &str) -> Result<Rc<Slot>> {
        self.all_slots()
            .get(n)
            .cloned()
            .ok_or_else(|| invalid("missing slot"))
    }
    fn destination(&self, n: &str) -> Result<()> {
        if n.starts_with("cap:") {
            return Err(invalid("capture slots are private"));
        }
        if let Some(rest) = n.strip_prefix("var:") {
            let id = rest
                .split(':')
                .next()
                .ok_or_else(|| invalid("variable name"))?;
            self.valid_counts(id)?;
        }
        Ok(())
    }
    fn insert(&self, n: &str, s: Rc<Slot>) {
        if let Some(rest) = n.strip_prefix("var:") {
            let id = rest.split(':').next().unwrap();
            self.control
                .arena
                .borrow_mut()
                .get_mut(id)
                .unwrap()
                .variables
                .insert(n.to_owned(), s);
        } else {
            self.control.roots.borrow_mut().insert(n.to_owned(), s);
            if n.starts_with("host:") {
                self.control
                    .retired
                    .borrow_mut()
                    .insert(n.to_owned(), Rc::new(Cell::new(false)));
            }
        }
    }
    fn remove(&self, n: &str) {
        if let Some(rest) = n.strip_prefix("var:") {
            if let Some(r) = self
                .control
                .arena
                .borrow_mut()
                .get_mut(rest.split(':').next().unwrap())
            {
                r.variables.remove(n);
            }
        } else {
            self.control.roots.borrow_mut().remove(n);
            self.control.retired.borrow_mut().remove(n);
        }
        self.control
            .aliases
            .borrow_mut()
            .retain(|_, (name, _)| name != n);
    }
    fn parse_raw(&self, v: &Value) -> Result<Raw> {
        match v[0].as_str() {
            Some("undefined") => Ok(Raw::Undefined),
            Some("int") => Ok(Raw::Int(
                v[1].as_i64()
                    .ok_or_else(|| invalid("integer"))?
                    .try_into()
                    .map_err(|_| limit("integer"))?,
            )),
            Some("object") => Ok(Raw::Object(self.valid_counts(text(&v[1])?)?)),
            Some("function") => Ok(Raw::Function(self.head(text(&v[1])?)?)),
            _ => Err(invalid("unsupported payload")),
        }
    }
    // Check all final-head cleanup borrows before committing any release. Retains
    // have already been reserved, so owners reflect the proposed surviving edges.
    fn preflight_release(values: &[Raw]) -> Result<()> {
        let mut pending = BTreeMap::<String, (Rc<Head>, usize)>::new();
        for v in values {
            if let Raw::Function(h) = v {
                let e = pending.entry(h.id.clone()).or_insert((h.clone(), 0));
                e.1 += 1;
            }
        }
        let mut visited = BTreeSet::new();
        loop {
            let next = pending
                .values()
                .find(|(h, n)| *n == h.owners.get() && !visited.contains(&h.id))
                .map(|(h, _)| h.clone());
            let Some(h) = next else { break };
            visited.insert(h.id.clone());
            for (_, s) in h.captures.borrow().iter() {
                let p = s
                    .payload
                    .try_borrow_mut()
                    .map_err(|_| invalid("final capture borrowed"))?;
                if let Raw::Function(child) = p.raw() {
                    let e = pending.entry(child.id.clone()).or_insert((child, 0));
                    e.1 += 1;
                }
            }
        }
        Ok(())
    }
    fn apply(&self, a: &Value) -> Result<()> {
        self.apply_fault(a, None)
    }
    fn apply_fault(&self, a: &Value, at: Option<usize>) -> Result<()> {
        let mut f = Fault { at, steps: 0 };
        f.hit()?;
        let op = text(&a[0])?;
        if self.control.closed.get()
            && !matches!(op, "retire" | "drain-host" | "drain" | "exit" | "drop")
        {
            return Err(invalid("closed store"));
        }
        if matches!(
            op,
            "put"
                | "function"
                | "copy"
                | "move"
                | "overwrite"
                | "drop"
                | "drain-host"
                | "cancel"
                | "restart"
                | "invoke"
                | "expand"
        ) {
            let _guard = self
                .control
                .roots
                .try_borrow_mut()
                .map_err(|_| invalid("root registry borrowed"))?;
        }
        match op {
            "object" => {
                let id = text(&a[1])?;
                if self.control.arena.borrow().contains_key(id)
                    || self.control.history.borrow().contains_key(id)
                {
                    return Err(invalid("identity reuse"));
                }
                f.hit()?;
                f.hit()?;
                let c = Rc::new(Counts {
                    id: id.into(),
                    origin: self.control.origin.clone(),
                    ordinary: Cell::new(0),
                    execs: Cell::new(0),
                    state: Cell::new(State::Alive),
                    cleaned: Cell::new(0),
                    queued: Cell::new(false),
                    next: RefCell::new(None),
                    capacity: self.control.capacity,
                });
                self.control.arena.borrow_mut().insert(
                    id.into(),
                    ObjectRecord {
                        counts: c,
                        variables: BTreeMap::new(),
                        stats: self.control.stats.clone(),
                    },
                );
            }
            "put" => {
                let n = text(&a[1])?;
                self.destination(n)?;
                if self.slot(n).is_ok() {
                    return Err(invalid("occupied slot"));
                }
                let raw = self.parse_raw(&a[2])?;
                f.hit()?;
                let p = Payload::acquire(&raw)?;
                f.hit()?;
                self.insert(n, Slot::new(p));
            }
            "function" => {
                let id = text(&a[1])?;
                let dst = text(&a[4])?;
                self.destination(dst)?;
                if self.head(id).is_ok() || self.slot(dst).is_ok() {
                    return Err(invalid("occupied head or destination"));
                }
                let creator = if a[2].is_null() {
                    None
                } else {
                    Some(self.valid_counts(text(&a[2])?)?)
                };
                let mut raw = Vec::new();
                for s in a[3].as_array().ok_or_else(|| invalid("sources"))? {
                    raw.push(self.slot(text(s)?)?.raw()?);
                }
                // Raw native creation of invalid captures is intentionally excluded.
                if raw.iter().any(Raw::invalid) {
                    return Err(invalid("invalid capture creation"));
                }
                f.hit()?;
                let creator = creator.as_ref().map(Ordinary::acquire).transpose()?;
                let mut captures = Vec::new();
                for (i, v) in raw.iter().enumerate() {
                    captures.push((format!("cap:{id}:{i}"), Slot::new(Payload::acquire(v)?)));
                }
                let h = Rc::new(Head {
                    id: id.into(),
                    origin: self.control.origin.clone(),
                    creator: RefCell::new(creator),
                    captures: RefCell::new(captures),
                    owners: Cell::new(0),
                    capacity: self.control.capacity,
                    stats: self.control.stats.clone(),
                });
                let p = Payload::Function(FunctionLease::acquire(&h)?);
                f.hit()?;
                self.control
                    .heads
                    .borrow_mut()
                    .insert(id.into(), Rc::downgrade(&h));
                self.insert(dst, Slot::new(p));
            }
            "copy" | "overwrite" | "move" => {
                self.copy_event(op, text(&a[1])?, text(&a[2])?, &mut f)?
            }
            "drop" => {
                let n = text(&a[1])?;
                let s = self.slot(n)?;
                if n.starts_with("cap:") {
                    return Err(invalid("private capture"));
                }
                let mut g = s
                    .payload
                    .try_borrow_mut()
                    .map_err(|_| invalid("slot borrowed"))?;
                Self::preflight_release(&[g.raw()])?;
                f.hit()?;
                f.hit()?;
                let old = std::mem::replace(&mut *g, Payload::Undefined);
                drop(g);
                self.remove(n);
                drop(old);
            }
            "enter" => {
                let name = text(&a[1])?;
                let c = self.valid_counts(text(&a[2])?)?;
                if self.control.frames.borrow().contains_key(name) {
                    return Err(invalid("occupied frame"));
                }
                f.hit()?;
                let lease = ExecLease::acquire(&c, &self.control.queue)?;
                f.hit()?;
                self.control.frames.borrow_mut().insert(name.into(), lease);
                self.control.frame_order.borrow_mut().push(name.into());
            }
            "exit" => {
                let name = text(&a[1])?;
                if !self.control.frames.borrow().contains_key(name) {
                    return Err(invalid("missing frame"));
                }
                f.hit()?;
                f.hit()?;
                let frame = self.control.frames.borrow_mut().remove(name);
                self.control.frame_order.borrow_mut().retain(|n| n != name);
                drop(frame);
            }
            "delete" => {
                let c = self.valid_counts(text(&a[1])?)?;
                if c.execs.get() > 0 {
                    f.hit()?;
                    f.hit()?;
                    c.state.set(State::Pending);
                } else {
                    self.cleanup(&c, false, &mut f)?;
                }
            }
            "drain" => {
                let c = self
                    .control
                    .queue
                    .head
                    .borrow()
                    .clone()
                    .ok_or_else(|| invalid("empty queue"))?;
                self.cleanup(&c, true, &mut f)?;
            }
            "snapshot" => {
                self.slot(text(&a[1])?)?.raw()?;
                f.hit()?;
                f.hit()?;
            }
            "alias" => {
                let name = text(&a[1])?;
                let n = text(&a[2])?;
                let s = self.slot(n)?;
                if self.control.aliases.borrow().contains_key(name) {
                    return Err(invalid("occupied alias"));
                }
                f.hit()?;
                f.hit()?;
                self.control
                    .aliases
                    .borrow_mut()
                    .insert(name.into(), (n.into(), Rc::downgrade(&s)));
            }
            "retire" => {
                let n = text(&a[1])?;
                let flag = self
                    .control
                    .retired
                    .borrow()
                    .get(n)
                    .cloned()
                    .ok_or_else(|| invalid("not a host root"))?;
                f.hit()?;
                f.hit()?;
                flag.set(true);
            }
            "drain-host" => {
                let names: Vec<_> = self
                    .control
                    .retired
                    .borrow()
                    .iter()
                    .filter(|(_, v)| v.get())
                    .map(|(n, _)| n.clone())
                    .collect();
                self.release_named(&names, &mut f)?;
            }
            "close" => {
                f.hit()?;
                f.hit()?;
                self.control.closed.set(true);
            }
            "transfer" => {
                f.hit()?;
                f.hit()?;
                self.control.transferred.set(true);
            }
            "cancel" | "restart" => {
                let names: Vec<_> = self
                    .control
                    .roots
                    .borrow()
                    .keys()
                    .filter(|n| !n.starts_with("host:"))
                    .cloned()
                    .collect();
                self.release_named_and_frames(&names, true, &mut f)?;
            }
            "expand" => {
                let h = self.head(text(&a[1])?)?;
                let i = a[2].as_u64().ok_or_else(|| invalid("capture index"))? as usize;
                let name = h
                    .captures
                    .borrow()
                    .get(i)
                    .map(|(n, _)| n.clone())
                    .ok_or_else(|| invalid("capture index"))?;
                self.copy_event("copy", &name, text(&a[3])?, &mut f)?;
            }
            "invoke" => self.invoke(
                text(&a[1])?,
                a[2].as_array().ok_or_else(|| invalid("destinations"))?,
                &mut f,
            )?,
            _ => return Err(invalid("unknown event")),
        }
        self.reap();
        self.prune_aliases();
        if !matches!(op, "close" | "retire") {
            self.audit();
        }
        Ok(())
    }
    fn copy_event(&self, op: &str, src: &str, dst: &str, f: &mut Fault) -> Result<()> {
        let source = self.slot(src)?;
        let raw = source.raw()?;
        if src == dst {
            let mut guard = source
                .payload
                .try_borrow_mut()
                .map_err(|_| invalid("source borrowed"))?;
            let normalize = op != "move" && raw.invalid();
            if normalize {
                Self::preflight_release(std::slice::from_ref(&raw))?;
            }
            f.hit()?;
            f.hit()?;
            if normalize {
                let old = std::mem::replace(&mut *guard, Payload::Undefined);
                drop(guard);
                self.normalized(src);
                drop(old);
            }
            return Ok(());
        }
        self.destination(dst)?;
        let existing = self.slot(dst).ok();
        if existing.is_some() && op != "overwrite" {
            return Err(invalid("occupied destination"));
        }
        let normalize = op != "move" && raw.invalid();
        let value = if normalize {
            Raw::Undefined
        } else {
            raw.clone()
        };
        let target = existing
            .clone()
            .unwrap_or_else(|| Slot::new(Payload::Undefined));
        let mut sg = source
            .payload
            .try_borrow_mut()
            .map_err(|_| invalid("source borrowed"))?;
        let mut dg = target
            .payload
            .try_borrow_mut()
            .map_err(|_| invalid("target borrowed"))?;
        f.hit()?;
        let reserved = if op == "move" {
            None
        } else {
            Some(Payload::acquire(&value)?)
        };
        let mut releases = vec![dg.raw()];
        if normalize {
            releases.push(sg.raw());
        }
        Self::preflight_release(&releases)?;
        f.hit()?;
        let new = if op == "move" {
            std::mem::replace(&mut *sg, Payload::Undefined)
        } else {
            reserved.unwrap()
        };
        let old_dest = std::mem::replace(&mut *dg, new);
        let old_source = if normalize {
            Some(std::mem::replace(&mut *sg, Payload::Undefined))
        } else {
            None
        };
        drop(dg);
        drop(sg);
        if existing.is_none() {
            self.insert(dst, target);
        }
        if op == "move" {
            self.remove(src);
        } else if normalize {
            self.normalized(src);
        }
        drop(old_dest);
        drop(old_source);
        Ok(())
    }
    fn normalized(&self, src: &str) {
        // Frozen model observation aliases expire on source normalization. These
        // are not executable References. The actual stable slot remains in place.
        self.control
            .aliases
            .borrow_mut()
            .retain(|_, (n, _)| n != src);
        if let Some(flag) = self.control.retired.borrow().get(src) {
            flag.set(false);
        }
    }
    fn cleanup(&self, c: &Rc<Counts>, queued: bool, f: &mut Fault) -> Result<()> {
        if c.cleaned.get() != 0 {
            return Err(invalid("duplicate cleanup"));
        }
        let slots: Vec<_> = self
            .control
            .arena
            .borrow()
            .get(&c.id)
            .ok_or_else(|| invalid("missing record"))?
            .variables
            .iter()
            .map(|(n, s)| (n.clone(), s.clone()))
            .collect();
        let mut guards = slots
            .iter()
            .map(|(_, s)| {
                s.payload
                    .try_borrow_mut()
                    .map_err(|_| invalid("variable borrowed"))
            })
            .collect::<Result<Vec<_>>>()?;
        let raw: Vec<_> = guards.iter().map(|g| g.raw()).collect();
        Self::preflight_release(&raw)?;
        let mut old = Vec::with_capacity(guards.len());
        f.hit()?;
        f.hit()?;
        let hold = if queued {
            Some(self.control.queue.pop().expect("preflight queue head"))
        } else {
            c.state.set(State::Shell);
            None
        };
        for g in &mut guards {
            old.push(std::mem::replace(&mut **g, Payload::Undefined));
        }
        drop(guards);
        self.control
            .arena
            .borrow_mut()
            .get_mut(&c.id)
            .unwrap()
            .variables
            .clear();
        for (n, _) in &slots {
            self.control
                .aliases
                .borrow_mut()
                .retain(|_, (name, _)| name != n);
        }
        c.cleaned.set(1);
        drop(old);
        c.state.set(State::Shell);
        drop(hold);
        if c.ordinary.get() == 0 {
            c.state.set(State::Freed);
        }
        Ok(())
    }
    fn release_named(&self, names: &[String], f: &mut Fault) -> Result<()> {
        self.release_named_and_frames(names, false, f)
    }
    fn release_named_and_frames(
        &self,
        names: &[String],
        frames: bool,
        f: &mut Fault,
    ) -> Result<()> {
        let slots = names
            .iter()
            .map(|n| self.slot(n))
            .collect::<Result<Vec<_>>>()?;
        let mut guards = slots
            .iter()
            .map(|s| {
                s.payload
                    .try_borrow_mut()
                    .map_err(|_| invalid("retired root borrowed"))
            })
            .collect::<Result<Vec<_>>>()?;
        let raw: Vec<_> = guards.iter().map(|g| g.raw()).collect();
        Self::preflight_release(&raw)?;
        let mut old = Vec::with_capacity(guards.len());
        f.hit()?;
        f.hit()?;
        if frames {
            let order = std::mem::take(&mut *self.control.frame_order.borrow_mut());
            for name in order {
                drop(self.control.frames.borrow_mut().remove(&name));
            }
        }
        for g in &mut guards {
            old.push(std::mem::replace(&mut **g, Payload::Undefined));
        }
        drop(guards);
        for n in names {
            self.remove(n);
        }
        drop(old);
        Ok(())
    }
    fn invoke(&self, source: &str, destinations: &[Value], f: &mut Fault) -> Result<()> {
        let src = self.slot(source)?;
        let Raw::Function(h) = src.raw()? else {
            return Err(invalid("not a function"));
        };
        if h.creator
            .borrow()
            .as_ref()
            .is_some_and(|o| !o.0.state.get().valid())
        {
            return Err(invalid("invalid creator"));
        }
        let captures = h.captures.borrow().clone();
        if captures.len() != destinations.len() {
            return Err(invalid("capture arity"));
        }
        let unique = h.owners.get() == 1;
        let mut seen = BTreeSet::new();
        let mut targets = Vec::new();
        for d in destinations {
            let name = text(d)?;
            self.destination(name)?;
            if !seen.insert(name.to_owned()) || self.slot(name).is_ok() {
                return Err(invalid("occupied destination"));
            }
            targets.push((name.to_owned(), Slot::new(Payload::Undefined)));
        }
        let mut source_guard = src
            .payload
            .try_borrow_mut()
            .map_err(|_| invalid("function slot borrowed"))?;
        let mut guards = captures
            .iter()
            .map(|(_, s)| {
                s.payload
                    .try_borrow_mut()
                    .map_err(|_| invalid("capture borrowed"))
            })
            .collect::<Result<Vec<_>>>()?;
        f.hit()?;
        let mut reserved = Vec::new();
        let mut releases = Vec::new();
        if !unique {
            for g in &guards {
                let raw = g.raw();
                if raw.invalid() {
                    releases.push(raw);
                    reserved.push(Payload::Undefined);
                } else {
                    reserved.push(Payload::acquire(&raw)?);
                }
            }
            Self::preflight_release(&releases)?;
        }
        let mut old = Vec::with_capacity(guards.len());
        f.hit()?;
        if unique {
            for (g, (_, target)) in guards.iter_mut().zip(&targets) {
                *target.payload.borrow_mut() = std::mem::replace(&mut **g, Payload::Undefined);
            }
        } else {
            for (((g, (name, _)), (_, target)), payload) in
                guards.iter_mut().zip(&captures).zip(&targets).zip(reserved)
            {
                if g.raw().invalid() {
                    old.push(std::mem::replace(&mut **g, Payload::Undefined));
                    self.normalized(name);
                }
                *target.payload.borrow_mut() = payload;
            }
        }
        drop(guards);
        let old_source = if unique {
            h.captures.borrow_mut().clear();
            Some(std::mem::replace(&mut *source_guard, Payload::Undefined))
        } else {
            None
        };
        drop(source_guard);
        for (n, s) in targets {
            self.insert(&n, s);
        }
        if unique {
            self.remove(source);
        }
        drop(old);
        drop(old_source);
        Ok(())
    }
    fn reap(&self) {
        let ids: Vec<_> = self
            .control
            .arena
            .borrow()
            .iter()
            .filter(|(_, r)| r.counts.state.get() == State::Freed)
            .map(|(id, _)| id.clone())
            .collect();
        for id in ids {
            let record = self.control.arena.borrow_mut().remove(&id).unwrap();
            assert_eq!(record.counts.cleaned.get(), 1);
            assert!(record.variables.is_empty());
            self.control
                .history
                .borrow_mut()
                .insert(id, record.counts.view());
            drop(record);
        }
    }
    fn prune_aliases(&self) {
        let slots = self.all_slots();
        self.control.aliases.borrow_mut().retain(|_, (name, weak)| {
            weak.upgrade().is_some_and(|slot| {
                slots
                    .get(name)
                    .is_some_and(|actual| Rc::ptr_eq(actual, &slot))
            })
        });
    }
    fn snapshot(&self) -> Value {
        let mut objects = self.control.history.borrow().clone();
        for (id, r) in self.control.arena.borrow().iter() {
            objects.insert(id.clone(), r.counts.view());
        }
        let slots: BTreeMap<_, _> = self
            .all_slots()
            .iter()
            .map(|(n, s)| (n.clone(), s.payload.borrow().view()))
            .collect();
        let mut heads = BTreeMap::new();
        for h in self
            .control
            .heads
            .borrow()
            .values()
            .filter_map(Weak::upgrade)
            .filter(|h| h.owners.get() > 0)
        {
            heads.insert(h.id.clone(),json!({"creator":h.creator.borrow().as_ref().map(|c|c.0.id.clone()),"captures":h.captures.borrow().iter().map(|(n,_)|n.clone()).collect::<Vec<_>>(),"owners":h.owners.get()}));
        }
        let frames: BTreeMap<_, _> = self
            .control
            .frames
            .borrow()
            .iter()
            .map(|(n, e)| (n.clone(), e.counts.id.clone()))
            .collect();
        let aliases: BTreeMap<_, _> = self
            .control
            .aliases
            .borrow()
            .iter()
            .map(|(n, (s, w))| {
                assert!(w.upgrade().is_some());
                (n.clone(), s.clone())
            })
            .collect();
        let retired: Vec<_> = self
            .control
            .retired
            .borrow()
            .iter()
            .filter(|(_, v)| v.get())
            .map(|(n, _)| n.clone())
            .collect();
        json!({"objects":objects,"slots":slots,"heads":heads,"frames":frames,"queue":self.control.queue.nodes().iter().map(|c|c.id.clone()).collect::<Vec<_>>(),"aliases":aliases,"retired":retired,"closed":self.control.closed.get(),"transferred":self.control.transferred.get()})
    }
    fn audit(&self) {
        let slots = self.all_slots();
        let mut refs = BTreeMap::<String, usize>::new();
        let mut execs = BTreeMap::<String, usize>::new();
        let mut owners = BTreeMap::<String, usize>::new();
        for s in slots.values() {
            match s.payload.borrow().raw() {
                Raw::Object(c) => {
                    assert!(Rc::ptr_eq(&c.origin, &self.control.origin));
                    *refs.entry(c.id.clone()).or_default() += 1;
                }
                Raw::Function(h) => {
                    assert!(Rc::ptr_eq(&h.origin, &self.control.origin));
                    *owners.entry(h.id.clone()).or_default() += 1;
                }
                _ => {}
            }
        }
        for h in self
            .control
            .heads
            .borrow()
            .values()
            .filter_map(Weak::upgrade)
            .filter(|h| h.owners.get() > 0)
        {
            assert_eq!(
                h.owners.get(),
                *owners.get(&h.id).unwrap_or(&0),
                "head owners"
            );
            if let Some(c) = h.creator.borrow().as_ref() {
                assert!(Rc::ptr_eq(&c.0.origin, &self.control.origin));
                *refs.entry(c.0.id.clone()).or_default() += 1;
            }
        }
        let order = self.control.frame_order.borrow();
        assert_eq!(order.len(), self.control.frames.borrow().len());
        assert_eq!(
            order.iter().cloned().collect::<BTreeSet<_>>(),
            self.control
                .frames
                .borrow()
                .keys()
                .cloned()
                .collect::<BTreeSet<_>>()
        );
        for e in self.control.frames.borrow().values() {
            *execs.entry(e.counts.id.clone()).or_default() += 1;
        }
        let queue = self.control.queue.nodes();
        let ids: BTreeSet<_> = queue.iter().map(|c| c.id.clone()).collect();
        assert_eq!(ids.len(), queue.len());
        for c in queue {
            *refs.entry(c.id.clone()).or_default() += 1;
        }
        for (id, r) in self.control.arena.borrow().iter() {
            let c = &r.counts;
            assert_eq!(
                c.ordinary.get(),
                *refs.get(id).unwrap_or(&0),
                "ordinary count {id}"
            );
            assert_eq!(
                c.execs.get(),
                *execs.get(id).unwrap_or(&0),
                "execution count {id}"
            );
            assert_eq!(c.state.get() == State::Queued, ids.contains(id));
            assert_eq!(c.queued.get(), ids.contains(id));
            assert!(c.cleaned.get() <= 1);
            if matches!(c.state.get(), State::Shell | State::Freed) {
                assert_eq!(c.cleaned.get(), 1);
                assert!(r.variables.is_empty());
            }
            if matches!(c.state.get(), State::Queued | State::Shell | State::Freed) {
                assert_eq!(c.execs.get(), 0);
            }
            if c.state.get() == State::Shell {
                assert!(c.ordinary.get() > 0);
            }
            if c.state.get() == State::Freed {
                assert_eq!(c.ordinary.get(), 0);
            }
            if matches!(c.state.get(), State::Alive | State::Pending | State::Queued) {
                assert_eq!(c.cleaned.get(), 0);
            }
            if c.state.get() == State::Pending {
                assert!(c.execs.get() > 0);
            }
        }
        for (name, weak) in self.control.aliases.borrow().values() {
            let slot = weak.upgrade().expect("dangling observation alias");
            assert!(
                slots
                    .get(name)
                    .is_some_and(|actual| Rc::ptr_eq(actual, &slot))
            );
        }
        for (id, v) in self.control.history.borrow().iter() {
            assert_eq!(v["state"], "freed");
            assert_eq!(*refs.get(id).unwrap_or(&0), 0);
            assert_eq!(*execs.get(id).unwrap_or(&0), 0);
        }
    }
    fn object_input(&self, other: &Rc<Counts>) -> Result<Raw> {
        if !Rc::ptr_eq(&other.origin, &self.control.origin) {
            return Err(invalid("foreign origin"));
        }
        if !other.state.get().valid() {
            return Err(invalid("invalid object"));
        }
        Ok(Raw::Object(other.clone()))
    }
    fn function_input(&self, head: &Rc<Head>) -> Result<Raw> {
        if !Rc::ptr_eq(&head.origin, &self.control.origin) || head.owners.get() == 0 {
            return Err(invalid("foreign or retired head"));
        }
        Ok(Raw::Function(head.clone()))
    }
    fn export_host(&self, source: &str, name: &str) -> Result<HostRoot> {
        if !name.starts_with("host:") {
            return Err(invalid("host name"));
        }
        self.apply(&json!(["copy", source, name]))?;
        Ok(HostRoot {
            control: self.control.clone(),
            name: name.into(),
            retired: self.control.retired.borrow().get(name).unwrap().clone(),
        })
    }
    fn import_host(&self, root: &HostRoot, dst: &str) -> Result<()> {
        if !Rc::ptr_eq(&self.control, &root.control)
            || root.retired.get()
            || !self
                .control
                .retired
                .borrow()
                .get(&root.name)
                .is_some_and(|f| Rc::ptr_eq(f, &root.retired))
        {
            return Err(invalid("foreign or retired host root"));
        }
        self.apply(&json!(["copy", root.name, dst]))
    }
    fn into_store(self) -> Self {
        self.control.transferred.set(true);
        self
    }
}
fn text(v: &Value) -> Result<&str> {
    v.as_str().ok_or_else(|| invalid("expected string"))
}
#[cfg(test)]
mod tests {
    use super::*;
    fn world() -> Kernel {
        let k = Kernel::new(usize::MAX).unwrap();
        for id in ["a", "b"] {
            k.apply(&json!(["object", id])).unwrap();
        }
        k
    }
    fn run(k: &Kernel, actions: &[Value]) {
        for a in actions {
            k.apply(a).unwrap();
        }
    }
    fn object(k: &Kernel, id: &str) -> Value {
        k.snapshot()["objects"][id].clone()
    }
    fn weak_counts(k: &Kernel) -> Vec<Weak<Counts>> {
        k.control
            .arena
            .borrow()
            .values()
            .map(|r| Rc::downgrade(&r.counts))
            .collect()
    }
    #[test]
    fn live_zero_is_not_business_deletion() {
        let k = world();
        run(
            &k,
            &[json!(["put", "x", ["object", "a"]]), json!(["drop", "x"])],
        );
        assert_eq!(object(&k, "a")["state"], "alive");
        assert_eq!(object(&k, "a")["refs"], 0);
    }
    #[test]
    fn immediate_delete_shell_then_physical_reap() {
        let k = world();
        let weak = Rc::downgrade(&k.counts("a").unwrap());
        run(
            &k,
            &[json!(["put", "x", ["object", "a"]]), json!(["delete", "a"])],
        );
        assert_eq!(object(&k, "a")["state"], "shell");
        assert!(weak.upgrade().is_some());
        k.apply(&json!(["drop", "x"])).unwrap();
        assert_eq!(object(&k, "a")["state"], "freed");
        assert!(weak.upgrade().is_none());
        assert_eq!(k.control.stats.objects.get(), 1);
    }
    #[test]
    fn pending_copy_is_valid_then_last_exec_queues() {
        let k = world();
        run(
            &k,
            &[
                json!(["put", "x", ["object", "a"]]),
                json!(["enter", "f", "a"]),
                json!(["delete", "a"]),
                json!(["copy", "x", "y"]),
            ],
        );
        assert_eq!(object(&k, "a")["state"], "pending");
        assert_eq!(object(&k, "a")["refs"], 2);
        k.apply(&json!(["exit", "f"])).unwrap();
        assert_eq!(object(&k, "a")["refs"], 3);
        assert_eq!(k.snapshot()["queue"], json!(["a"]));
    }
    #[test]
    fn two_exec_leases_enqueue_once_and_lifo() {
        let k = world();
        run(
            &k,
            &[
                json!(["enter", "f1", "a"]),
                json!(["enter", "f2", "a"]),
                json!(["enter", "g", "b"]),
                json!(["delete", "a"]),
                json!(["delete", "a"]),
                json!(["delete", "b"]),
                json!(["exit", "f1"]),
            ],
        );
        assert_eq!(k.snapshot()["queue"], json!([]));
        run(&k, &[json!(["exit", "f2"]), json!(["exit", "g"])]);
        assert_eq!(k.snapshot()["queue"], json!(["b", "a"]));
        k.apply(&json!(["drain"])).unwrap();
        assert_eq!(object(&k, "b")["state"], "freed");
        assert_eq!(object(&k, "a")["state"], "queued");
    }
    #[test]
    fn creator_and_payload_counts_are_independent() {
        for captured in ["a", "b"] {
            let k = world();
            run(
                &k,
                &[
                    json!(["put", "x", ["object", captured]]),
                    json!(["function", "h", "a", ["x"], "f"]),
                    json!(["copy", "f", "g"]),
                ],
            );
            assert_eq!(
                object(&k, "a")["refs"],
                if captured == "a" { json!(3) } else { json!(1) }
            );
            run(&k, &[json!(["drop", "f"]), json!(["drop", "g"])]);
            assert_eq!(
                object(&k, "a")["refs"],
                if captured == "a" { json!(1) } else { json!(0) }
            );
        }
    }
    #[test]
    fn physical_function_views_do_not_retain_creator_after_final_owner() {
        let k = world();
        k.apply(&json!(["function", "h", "a", [], "f"])).unwrap();
        let physical = k.head("h").unwrap();
        let weak = Rc::downgrade(&physical);
        k.apply(&json!(["drop", "f"])).unwrap();
        assert_eq!(physical.owners.get(), 0);
        assert!(physical.creator.borrow().is_none());
        assert_eq!(object(&k, "a")["refs"], 0);
        drop(physical);
        assert!(weak.upgrade().is_none());
    }
    #[test]
    fn unique_move_vs_shared_copy_invalid_capture() {
        for shared in [false, true] {
            let k = world();
            run(
                &k,
                &[
                    json!(["put", "x", ["object", "b"]]),
                    json!(["function", "h", "a", ["x"], "f"]),
                    json!(["delete", "b"]),
                ],
            );
            if shared {
                k.apply(&json!(["copy", "f", "g"])).unwrap();
            }
            k.apply(&json!(["invoke", "f", ["out"]])).unwrap();
            assert_eq!(
                k.snapshot()["slots"]["out"],
                if shared {
                    json!(["undefined", null])
                } else {
                    json!(["object", "b"])
                }
            );
            assert_eq!(
                k.snapshot()["heads"].as_object().unwrap().contains_key("h"),
                shared
            );
        }
    }
    #[test]
    fn variable_function_creator_cycle_has_no_physical_arena_cycle() {
        let k = world();
        let weak = weak_counts(&k);
        run(
            &k,
            &[
                json!(["function", "h", "a", [], "var:a:f"]),
                json!(["delete", "a"]),
            ],
        );
        assert_eq!(object(&k, "a")["state"], "freed");
        assert!(weak[0].upgrade().is_none());
        let stats = k.control.stats.clone();
        drop(k);
        assert_eq!(stats.objects.get(), 2);
        assert_eq!(stats.controls.get(), 1);
    }
    #[test]
    fn two_heads_six_slots_and_final_control_drop() {
        let k = world();
        run(
            &k,
            &[
                json!(["put", "x", ["object", "a"]]),
                json!(["put", "y", ["object", "b"]]),
                json!(["function", "h", "a", ["x"], "f"]),
                json!(["function", "j", "b", ["y"], "g"]),
            ],
        );
        assert_eq!(k.all_slots().len(), 6);
        let weak = weak_counts(&k);
        let stats = k.control.stats.clone();
        drop(k);
        assert!(weak.iter().all(|w| w.upgrade().is_none()));
        assert_eq!(stats.heads.get(), 2);
        assert_eq!(stats.objects.get(), 2);
    }
    #[test]
    fn nested_head_release_cleans_captures_before_creator() {
        let k = world();
        run(
            &k,
            &[
                json!(["function", "inner", "a", [], "f"]),
                json!(["function", "outer", "b", ["f"], "g"]),
                json!(["drop", "f"]),
                json!(["drop", "g"]),
            ],
        );
        assert_eq!(k.snapshot()["heads"], json!({}));
        assert_eq!(object(&k, "a")["refs"], 0);
        assert_eq!(object(&k, "b")["refs"], 0);
    }
    #[test]
    fn self_copy_and_overwrite_release_invalid_payloads() {
        let k = world();
        run(
            &k,
            &[
                json!(["put", "x", ["object", "a"]]),
                json!(["delete", "a"]),
                json!(["copy", "x", "x"]),
            ],
        );
        assert_eq!(object(&k, "a")["state"], "freed");
        assert_eq!(k.snapshot()["slots"]["x"], json!(["undefined", null]));
        run(
            &k,
            &[
                json!(["put", "y", ["object", "b"]]),
                json!(["overwrite", "x", "y"]),
            ],
        );
        assert_eq!(object(&k, "b")["refs"], 0);
    }
    #[test]
    fn move_does_not_acquire_a_temporary_owner_at_capacity() {
        let k = Kernel::new(2).unwrap();
        run(
            &k,
            &[json!(["object", "a"]), json!(["put", "x", ["object", "a"]])],
        );
        assert_eq!(
            k.apply(&json!(["copy", "x", "y"])).unwrap_err().kind,
            VmErrorKind::Limit
        );
        k.apply(&json!(["move", "x", "y"])).unwrap();
        assert_eq!(object(&k, "a")["refs"], 1);
    }
    #[test]
    fn queue_uses_reserved_capacity_on_infallible_exec_drop() {
        let k = Kernel::new(2).unwrap();
        run(
            &k,
            &[
                json!(["object", "a"]),
                json!(["put", "x", ["object", "a"]]),
                json!(["enter", "f", "a"]),
                json!(["delete", "a"]),
                json!(["exit", "f"]),
            ],
        );
        assert_eq!(object(&k, "a")["refs"], 2);
        k.apply(&json!(["drain"])).unwrap();
        assert_eq!(object(&k, "a")["refs"], 1);
    }
    #[test]
    fn checked_counter_arithmetic_never_wraps() {
        let k = world();
        let c = k.counts("a").unwrap();
        c.ordinary.set(usize::MAX);
        assert_eq!(
            Ordinary::acquire(&c).err().unwrap().kind,
            VmErrorKind::Limit
        );
        assert_eq!(c.ordinary.get(), usize::MAX);
        c.ordinary.set(0);
        c.execs.set(usize::MAX);
        assert_eq!(
            ExecLease::acquire(&c, &k.control.queue).err().unwrap().kind,
            VmErrorKind::Limit
        );
        c.execs.set(0);
        k.apply(&json!(["function", "h", null, [], "f"])).unwrap();
        let h = k.head("h").unwrap();
        h.owners.set(usize::MAX);
        assert_eq!(
            FunctionLease::acquire(&h).err().unwrap().kind,
            VmErrorKind::Limit
        );
        h.owners.set(1);
        k.audit();
    }
    #[test]
    fn borrowed_variable_rejects_delete_and_drain_atomically() {
        for queued in [false, true] {
            let k = world();
            k.apply(&json!(["put", "var:a:v", ["object", "b"]]))
                .unwrap();
            if queued {
                run(
                    &k,
                    &[
                        json!(["enter", "f", "a"]),
                        json!(["delete", "a"]),
                        json!(["exit", "f"]),
                    ],
                );
            }
            let before = k.snapshot();
            let slot = k.slot("var:a:v").unwrap();
            let guard = slot.payload.borrow_mut();
            let action = if queued {
                json!(["drain"])
            } else {
                json!(["delete", "a"])
            };
            assert_eq!(
                k.apply(&action).unwrap_err().kind,
                VmErrorKind::InvalidState
            );
            drop(guard);
            assert_eq!(k.snapshot(), before);
            k.apply(&action).unwrap();
            assert_eq!(object(&k, "b")["refs"], 0);
        }
    }
    #[test]
    fn final_capture_borrow_rejects_drop_before_source_change() {
        let k = world();
        run(
            &k,
            &[
                json!(["put", "x", ["object", "b"]]),
                json!(["function", "h", "a", ["x"], "f"]),
            ],
        );
        let before = k.snapshot();
        let slot = k.slot("cap:h:0").unwrap();
        let guard = slot.payload.borrow_mut();
        assert!(k.apply(&json!(["drop", "f"])).is_err());
        drop(guard);
        assert_eq!(k.snapshot(), before);
        k.apply(&json!(["drop", "f"])).unwrap();
    }
    #[test]
    fn host_drop_while_registry_borrowed_marks_then_drains_once() {
        let k = world();
        k.apply(&json!(["put", "x", ["object", "a"]])).unwrap();
        let h = k.export_host("x", "host:h").unwrap();
        let guard = k.control.roots.borrow();
        drop(h);
        assert!(k.control.retired.borrow()["host:h"].get());
        assert!(k.apply(&json!(["drain-host"])).is_err());
        assert_eq!(k.counts("a").unwrap().ordinary.get(), 2);
        drop(guard);
        k.apply(&json!(["drain-host"])).unwrap();
        assert_eq!(object(&k, "a")["refs"], 1);
        k.apply(&json!(["drain-host"])).unwrap();
    }
    #[test]
    fn close_and_transfer_keep_external_root_and_origin_until_retirement() {
        let k = world();
        k.apply(&json!(["put", "x", ["object", "a"]])).unwrap();
        let h = k.export_host("x", "host:h").unwrap();
        let k = k.into_store();
        k.import_host(&h, "y").unwrap();
        let origin = k.control.origin.clone();
        let weak = Rc::downgrade(&k.control);
        let stats = k.control.stats.clone();
        k.apply(&json!(["close"])).unwrap();
        assert!(k.import_host(&h, "z").is_err());
        assert!(Rc::ptr_eq(&origin, &h.control.origin));
        drop(k);
        assert!(weak.upgrade().is_some());
        drop(h);
        assert!(weak.upgrade().is_none());
        assert_eq!(stats.controls.get(), 1);
        assert_eq!(stats.objects.get(), 2);
    }
    #[test]
    fn host_retain_is_a_distinct_root_and_foreign_handles_reject() {
        let a = world();
        let b = world();
        a.apply(&json!(["put", "x", ["object", "a"]])).unwrap();
        let h = a.export_host("x", "host:h").unwrap();
        let retained = a.export_host("host:h", "host:r").unwrap();
        assert!(b.import_host(&h, "x").is_err());
        assert!(b.object_input(&a.counts("a").unwrap()).is_err());
        drop(h);
        a.apply(&json!(["drain-host"])).unwrap();
        assert_eq!(object(&a, "a")["refs"], 2);
        a.import_host(&retained, "y").unwrap();
    }
    #[test]
    fn stale_retired_host_handle_cannot_target_reused_slot_name() {
        let k = world();
        k.apply(&json!(["put", "x", ["object", "a"]])).unwrap();
        let old = k.export_host("x", "host:h").unwrap();
        k.apply(&json!(["drop", "host:h"])).unwrap();
        let new = k.export_host("x", "host:h").unwrap();
        assert!(k.import_host(&old, "bad").is_err());
        drop(old);
        k.apply(&json!(["drain-host"])).unwrap();
        k.import_host(&new, "ok").unwrap();
    }
    #[test]
    fn cancel_restart_exit_frames_without_implicit_queue_drain() {
        for op in ["cancel", "restart"] {
            let k = world();
            run(
                &k,
                &[
                    json!(["enter", "f", "a"]),
                    json!(["delete", "a"]),
                    json!([op]),
                ],
            );
            assert_eq!(object(&k, "a")["state"], "queued");
        }
    }
    #[test]
    fn snapshot_and_alias_are_not_payload_owners() {
        let k = world();
        run(
            &k,
            &[
                json!(["put", "x", ["object", "a"]]),
                json!(["alias", "ref", "x"]),
                json!(["snapshot", "x"]),
            ],
        );
        assert_eq!(object(&k, "a")["refs"], 1);
        k.apply(&json!(["drop", "x"])).unwrap();
        assert_eq!(k.snapshot()["aliases"], json!({}));
    }
    #[test]
    fn invalid_use_and_object_id_reuse_reject() {
        let k = world();
        k.apply(&json!(["delete", "a"])).unwrap();
        let before = k.snapshot();
        for action in [
            json!(["enter", "f", "a"]),
            json!(["put", "x", ["object", "a"]]),
            json!(["object", "a"]),
        ] {
            assert!(k.apply(&action).is_err());
            assert_eq!(k.snapshot(), before);
        }
    }
    #[test]
    fn every_prepublication_checkpoint_rolls_back_reservations() {
        let actions = vec![
            json!(["put", "x", ["object", "a"]]),
            json!(["function", "h", "a", ["source"], "f"]),
            json!(["copy", "source", "x"]),
            json!(["overwrite", "source", "target"]),
            json!(["move", "source", "x"]),
            json!(["enter", "f", "a"]),
            json!(["delete", "a"]),
            json!(["invoke", "base", ["out"]]),
            json!(["cancel"]),
        ];
        for action in actions {
            for checkpoint in 0..3 {
                let k = world();
                run(
                    &k,
                    &[
                        json!(["put", "source", ["object", "b"]]),
                        json!(["put", "target", ["object", "a"]]),
                        json!(["function", "base", "a", ["source"], "base"]),
                    ],
                );
                let before = k.snapshot();
                assert_eq!(
                    k.apply_fault(&action, Some(checkpoint)).unwrap_err().kind,
                    VmErrorKind::Limit,
                    "{action} checkpoint {checkpoint}"
                );
                assert_eq!(k.snapshot(), before);
                k.audit();
            }
        }
    }
    #[test]
    fn drain_and_host_release_faults_preserve_queue_and_roots() {
        for action in [json!(["drain"]), json!(["drain-host"])] {
            for checkpoint in 0..3 {
                let k = world();
                run(
                    &k,
                    &[
                        json!(["put", "host:h", ["object", "a"]]),
                        json!(["retire", "host:h"]),
                        json!(["enter", "f", "b"]),
                        json!(["delete", "b"]),
                        json!(["exit", "f"]),
                    ],
                );
                let before = k.snapshot();
                assert!(k.apply_fault(&action, Some(checkpoint)).is_err());
                assert_eq!(k.snapshot(), before);
            }
        }
    }
    #[test]
    fn independent_audit_detects_ordinary_creator_and_queue_corruption() {
        let k = world();
        k.apply(&json!(["function", "h", "a", [], "f"])).unwrap();
        let c = k.counts("a").unwrap();
        c.ordinary.set(0);
        assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| k.audit())).is_err());
        c.ordinary.set(1);
        run(
            &k,
            &[
                json!(["enter", "g", "b"]),
                json!(["delete", "b"]),
                json!(["exit", "g"]),
            ],
        );
        let b = k.counts("b").unwrap();
        b.ordinary.set(0);
        assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| k.audit())).is_err());
        b.ordinary.set(1);
        k.audit();
    }
    #[test]
    fn independent_audit_detects_early_free_and_duplicate_cleanup() {
        let k = world();
        let c = k.counts("a").unwrap();
        c.state.set(State::Freed);
        assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| k.audit())).is_err());
        c.state.set(State::Alive);
        c.cleaned.set(2);
        assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| k.audit())).is_err());
        c.cleaned.set(0);
        k.audit();
    }
    #[test]
    fn duplicate_queue_insertion_is_detected() {
        let k = world();
        run(
            &k,
            &[
                json!(["enter", "f", "a"]),
                json!(["delete", "a"]),
                json!(["exit", "f"]),
            ],
        );
        let c = k.counts("a").unwrap();
        assert!(
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| k.control.queue.push(&c)))
                .is_err()
        );
        k.audit();
    }
    #[test]
    fn shared_capture_expansion_and_null_creator() {
        let k = world();
        run(
            &k,
            &[
                json!(["put", "x", ["object", "a"]]),
                json!(["function", "h", null, ["x"], "f"]),
                json!(["copy", "f", "g"]),
                json!(["expand", "h", 0, "out"]),
            ],
        );
        assert_eq!(object(&k, "a")["refs"], 3);
        assert!(k.head("h").unwrap().creator.borrow().is_none());
    }
    #[test]
    fn closing_and_retirement_are_safe_while_payload_is_borrowed() {
        let k = world();
        k.apply(&json!(["put", "host:h", ["object", "a"]])).unwrap();
        let slot = k.slot("host:h").unwrap();
        let guard = slot.payload.borrow_mut();
        k.apply(&json!(["retire", "host:h"])).unwrap();
        k.apply(&json!(["close"])).unwrap();
        assert!(k.apply(&json!(["drain-host"])).is_err());
        drop(guard);
        k.apply(&json!(["drain-host"])).unwrap();
        assert_eq!(object(&k, "a")["refs"], 0);
    }
    #[test]
    fn foreign_and_retired_function_views_cannot_be_imported() {
        let a = world();
        let b = world();
        a.apply(&json!(["function", "h", null, [], "f"])).unwrap();
        let view = a.head("h").unwrap();
        assert!(b.function_input(&view).is_err());
        assert!(a.function_input(&view).is_ok());
        a.apply(&json!(["drop", "f"])).unwrap();
        assert!(a.function_input(&view).is_err());
    }
    #[test]
    fn capture_observation_alias_expires_after_move_or_final_release() {
        for invoke in [false, true] {
            let k = world();
            run(
                &k,
                &[
                    json!(["put", "x", ["object", "b"]]),
                    json!(["function", "h", "a", ["x"], "f"]),
                    json!(["alias", "ref", "cap:h:0"]),
                ],
            );
            k.apply(&if invoke {
                json!(["invoke", "f", ["out"]])
            } else {
                json!(["drop", "f"])
            })
            .unwrap();
            assert_eq!(k.snapshot()["aliases"], json!({}));
        }
    }
    #[test]
    fn group_final_release_preflights_shared_capture_borrows() {
        let k = world();
        run(
            &k,
            &[
                json!(["put", "x", ["object", "b"]]),
                json!(["function", "h", "a", ["x"], "var:a:f"]),
                json!(["copy", "var:a:f", "var:a:g"]),
            ],
        );
        let before = k.snapshot();
        let slot = k.slot("cap:h:0").unwrap();
        let guard = slot.payload.borrow_mut();
        assert!(k.apply(&json!(["delete", "a"])).is_err());
        drop(guard);
        assert_eq!(k.snapshot(), before);
        k.apply(&json!(["delete", "a"])).unwrap();
        assert_eq!(object(&k, "a")["state"], "freed");
        assert_eq!(object(&k, "b")["refs"], 1);
    }
    #[test]
    fn cancellation_releases_frames_in_entry_order_then_queue_is_lifo() {
        let k = world();
        run(
            &k,
            &[
                json!(["enter", "z", "b"]),
                json!(["enter", "a", "a"]),
                json!(["delete", "a"]),
                json!(["delete", "b"]),
            ],
        );
        let before = k.snapshot();
        for checkpoint in 0..3 {
            assert!(k.apply_fault(&json!(["cancel"]), Some(checkpoint)).is_err());
            assert_eq!(k.snapshot(), before);
        }
        k.apply(&json!(["cancel"])).unwrap();
        assert_eq!(k.snapshot()["queue"], json!(["a", "b"]));
    }
    #[test]
    fn failed_pending_frame_reservation_does_not_enqueue_or_retire_object() {
        let k = world();
        run(&k, &[json!(["enter", "f", "a"]), json!(["delete", "a"])]);
        let before = k.snapshot();
        assert!(k.apply_fault(&json!(["enter", "g", "a"]), Some(2)).is_err());
        assert_eq!(k.snapshot(), before);
        k.audit();
    }
    #[test]
    #[ignore = "run by differential.py with deterministic temporary JSONL input"]
    fn replay_model_traces() {
        use std::io::BufRead;
        let path =
            std::env::var_os("RML32_LIFECYCLE_TRACE_INPUT").expect("explicit trace input required");
        let input = std::io::BufReader::new(std::fs::File::open(path).unwrap());
        let mut count = 0;
        let mut rejected = 0;
        for line in input.lines() {
            let row: Value = serde_json::from_str(&line.unwrap()).unwrap();
            let k = Kernel::new(usize::MAX).unwrap();
            let n = row["objects"].as_u64().unwrap();
            for i in 0..n {
                k.apply(&json!(["object", i.to_string()])).unwrap();
            }
            for a in row["trace"].as_array().unwrap() {
                k.apply(a).unwrap();
            }
            let before = k.snapshot();
            let result = k.apply(&row["action"]);
            let accepted = result.is_ok();
            assert_eq!(
                accepted,
                row["accepted"].as_bool().unwrap(),
                "row {count}: {row} error {:?}",
                result.err()
            );
            if !accepted {
                assert_eq!(
                    k.snapshot(),
                    before,
                    "rejection changed state at row {count}"
                );
                rejected += 1;
            }
            assert_eq!(
                k.snapshot(),
                row["expected"],
                "state divergence at row {count}: {row}"
            );
            k.audit();
            let weak = weak_counts(&k);
            let control = Rc::downgrade(&k.control);
            let stats = k.control.stats.clone();
            drop(k);
            assert!(control.upgrade().is_none());
            assert!(weak.iter().all(|w| w.upgrade().is_none()));
            assert_eq!(stats.objects.get(), n as usize);
            assert_eq!(stats.controls.get(), 1);
            count += 1;
        }
        assert!(count > 0);
        println!(
            "LIFECYCLE_REPLAY {}",
            json!({"transitions":count,"rejected":rejected,"accepted":count-rejected,"physical_teardowns":count})
        );
    }
}
