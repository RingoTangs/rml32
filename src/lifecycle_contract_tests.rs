//! Input contains actions only. Actual observations are compared outside Rust.
use super::*;
use crate::cell::Cell as Slot;
use crate::copy_plan::CopyPlan;
use crate::lifecycle_adapter::runtime::{ExecLease, HeadReservation, StoreState};
use crate::{
    LifecycleBudget, LifecycleLimits, ProgramImage, ProgramMode, parse_program, prepare_program,
};
use serde_json::{Value as Json, json};
use std::collections::BTreeMap;
include!("../tests/boundary_support.rs.inc");

struct PendingHead {
    cell: Rc<Slot>,
    reservation: HeadReservation,
    name: String,
    dest: String,
    value: value::Value,
    member: Rc<Slot>,
}
enum Root {
    Slot(Rc<Slot>),
    Host { id: u64, handle: Option<HostValue> },
}
struct Driver {
    store: ObjectStore,
    registry: Rc<Registry>,
    program: ProgramId,
    life: Rc<StoreState>,
    objects: BTreeMap<String, ObjectId>,
    roots: BTreeMap<String, Root>,
    headers: BTreeMap<String, crate::graph::WeakHeader>,
    header_ids: BTreeMap<String, u64>,
    kinds: BTreeMap<String, String>,
    creators: BTreeMap<String, Option<String>>,
    views: BTreeMap<String, Vec<value::Value>>,
    execs: BTreeMap<String, ExecLease>,
    plans: BTreeMap<String, (CopyPlan, String, Rc<Slot>)>,
    heads: BTreeMap<String, PendingHead>,
    known_slots: BTreeSet<String>,
}
fn text<'a>(j: &'a Json, key: &str) -> &'a str {
    j[key].as_str().expect("typed action string")
}
fn number(j: &Json, key: &str) -> usize {
    j[key].as_u64().expect("typed unsigned field") as usize
}
fn progress(p: &crate::LifecycleProgress) -> Json {
    json!({"retired_roots":p.retired_roots,"drained_objects":p.drained_objects,"reaped_objects":p.reaped_objects,"reclaimed_heads":p.reclaimed_heads,"pruned_weak_entries":p.pruned_weak_entries,"metadata_examined":p.metadata_examined,"pending_roots":p.pending_roots,"queued_objects":p.queued_objects,"scan_complete":p.scan_complete,"stop":format!("{:?}",p.stop)})
}
impl Driver {
    fn new(q: &Json) -> Self {
        let mut store = ObjectStore::with_bounded_lifecycle(
            ValueLimits::default(),
            LifecycleLimits {
                max_objects: number(q, "max_objects"),
                max_function_heads: number(q, "max_function_heads"),
                max_ordinary_refs_per_object: number(q, "max_ordinary_refs_per_object"),
                max_exec_refs_per_object: number(q, "max_exec_refs_per_object"),
                max_owners_per_header: number(q, "max_owners_per_header"),
            },
        )
        .unwrap();
        let program = store
            .register_program(
                prepare_program(image(&[fun(0, 0, 0, 2, &[(9, 0, 0x3b)])], 1, &[])).unwrap(),
            )
            .unwrap();
        let life = store.registry.test_life.as_ref().unwrap().clone();
        Self {
            registry: store.registry.clone(),
            store,
            program,
            life,
            objects: BTreeMap::new(),
            roots: BTreeMap::new(),
            headers: BTreeMap::new(),
            header_ids: BTreeMap::new(),
            kinds: BTreeMap::new(),
            creators: BTreeMap::new(),
            views: BTreeMap::new(),
            execs: BTreeMap::new(),
            plans: BTreeMap::new(),
            heads: BTreeMap::new(),
            known_slots: BTreeSet::new(),
        }
    }
    fn value(&self, v: &Json) -> Result<value::Value, VmError> {
        match v[0].as_str().unwrap() {
            "scalar" => Ok(value::Value::Undefined),
            "object" => self
                .objects
                .get(v[1].as_str().unwrap())
                .map(|id| value::Value::Object(id.slot))
                .ok_or_else(|| VmError::new(VmErrorKind::InvalidObject, "unknown object label")),
            "header" => self
                .headers
                .get(v[1].as_str().unwrap())
                .and_then(|w| w.upgrade())
                .ok_or_else(|| invalid("inactive header label")),
            _ => Err(invalid("unknown value kind")),
        }
    }
    fn slot(&self, name: &str) -> Result<Rc<Slot>, VmError> {
        match self.roots.get(name) {
            Some(Root::Slot(c)) => Ok(c.clone()),
            _ => Err(invalid("missing stable slot")),
        }
    }
    fn read(&self, name: &str) -> Result<value::Value, VmError> {
        match self.roots.get(name) {
            Some(Root::Slot(c)) => c.read(),
            Some(Root::Host { id, .. }) => Ok(self
                .registry
                .entries
                .borrow()
                .iter()
                .find(|e| e.id == *id)
                .map(|e| e.payload.value.clone())
                .unwrap_or(value::Value::Undefined)),
            None => Err(invalid("missing source")),
        }
    }
    fn set(&mut self, name: &str, v: value::Value) -> Result<(), VmError> {
        if name.starts_with("host:") {
            let handle = self.store.registry.publish(v)?;
            self.roots.insert(
                name.into(),
                Root::Host {
                    id: handle.id,
                    handle: Some(handle),
                },
            );
        } else if let Some(Root::Slot(c)) = self.roots.get(name) {
            c.replace(v)?;
        } else {
            self.roots
                .insert(name.into(), Root::Slot(self.life.cell(v)?));
        }
        self.known_slots.insert(name.into());
        Ok(())
    }
    fn bind_head(
        &mut self,
        name: String,
        dest: String,
        v: value::Value,
        member: Rc<Slot>,
        cell: Rc<Slot>,
    ) {
        let id = v.counter().unwrap().id;
        let weak = match &v {
            value::Value::Function(f) => crate::graph::WeakHeader::Function(Rc::downgrade(&f.0)),
            value::Value::Array(a) => crate::graph::WeakHeader::Array(Rc::downgrade(&a.0)),
            value::Value::Mapping(m) => {
                crate::graph::WeakHeader::Mapping(Rc::downgrade(&m.0), Rc::downgrade(&m.1))
            }
            _ => unreachable!(),
        };
        self.headers.insert(name.clone(), weak);
        self.header_ids.insert(name.clone(), id);
        let member_name = format!("mem:{name}:0");
        self.known_slots.insert(member_name.clone());
        self.roots.insert(member_name, Root::Slot(member));
        self.known_slots.insert(dest.clone());
        self.roots.insert(dest, Root::Slot(cell));
    }
    fn head(&mut self, a: &Json, staged: bool) -> Result<(), VmError> {
        let name = text(a, "id").to_owned();
        let kind = text(a, "kind");
        let dest = text(a, "dest").to_owned();
        let creator = a.get("creator").and_then(Json::as_str).map(str::to_owned);
        let (v, member, guard) = match kind {
            "function" => {
                let owner = creator
                    .as_ref()
                    .map(|n| {
                        self.objects.get(n).map(|id| id.slot).ok_or_else(|| {
                            VmError::new(VmErrorKind::InvalidObject, "unknown creator")
                        })
                    })
                    .transpose()?;
                if owner.is_some_and(|o| !self.life.valid(o)) {
                    return Err(VmError::new(VmErrorKind::InvalidObject, "invalid creator"));
                }
                let member = self.life.cell(value::Value::Undefined)?;
                let f = value::FunctionValue(Rc::new(value::FunctionData {
                    origin: self.store.inner.as_ref().unwrap().function_origin.clone(),
                    owner,
                    target: value::FunctionTarget::Global {
                        program: value::ProgramId(0),
                        function: 0,
                    },
                    bound: vec![member.clone()],
                    bound_count: 1,
                    capture_profile: crate::value::CaptureProfile::Container,
                    counter: Counter::new()?,
                }));
                let guard = self.life.reserve_function(&f)?;
                (value::Value::Function(f), member, Some(guard))
            }
            "array" => {
                let v = self.life.array(vec![value::Value::Undefined])?;
                let member = crate::container::lifecycle_members(&v)?[0].clone();
                (v, member, None)
            }
            "mapping" => {
                let v = self
                    .life
                    .mapping(vec![(value::Value::Int(0), value::Value::Undefined)])?;
                let member = crate::container::lifecycle_members(&v)?[0].clone();
                (v, member, None)
            }
            _ => return Err(invalid("head kind")),
        };
        let cell = self.life.cell(v.clone())?;
        if staged {
            self.heads.insert(
                text(a, "token").into(),
                PendingHead {
                    cell,
                    reservation: guard.expect("Function reservation"),
                    name,
                    dest,
                    value: v,
                    member,
                },
            );
        } else {
            if let Some(g) = guard {
                g.commit();
            }
            self.kinds.insert(name.clone(), kind.into());
            self.creators.insert(name.clone(), creator);
            self.bind_head(name, dest, v, member, cell);
        }
        Ok(())
    }
    fn event(&mut self, a: &Json) -> Result<Json, VmError> {
        match text(a, "op") {
            "object" => {
                let id = self.store.create(self.program, &[ValueInput::Undefined])?;
                let name = text(a, "id");
                self.objects.insert(name.into(), id);
                let slot = format!("var:{name}:0");
                self.known_slots.insert(slot.clone());
                let cell = self
                    .store
                    .inner
                    .as_ref()
                    .unwrap()
                    .objects_for_audit_for_test(id.slot)
                    .unwrap()[0]
                    .clone();
                self.roots.insert(slot, Root::Slot(cell));
            }
            "head" => self.head(a, false)?,
            "prepare_head" => self.head(a, true)?,
            "commit_head" => {
                let pending = self
                    .heads
                    .remove(text(a, "token"))
                    .ok_or_else(|| invalid("missing head token"))?;
                pending.reservation.validate()?;
                let PendingHead {
                    cell,
                    reservation,
                    name,
                    dest,
                    value,
                    member,
                } = pending;
                reservation.commit();
                self.kinds.insert(name.clone(), "function".into());
                self.creators.insert(
                    name.clone(),
                    a.get("creator").and_then(Json::as_str).map(str::to_owned),
                );
                self.bind_head(name, dest, value, member, cell);
            }
            "abort_head" => {
                self.heads
                    .remove(text(a, "token"))
                    .ok_or_else(|| invalid("missing head token"))?;
            }
            "put" => {
                let v = self.value(&a["value"])?;
                self.set(text(a, "dest"), v)?;
            }
            "copy" => {
                let source = text(a, "source");
                let dest = text(a, "dest");
                let raw = self.read(source)?;
                let script = a.get("script").and_then(Json::as_bool).unwrap_or(false);
                let mut plan = self.life.plan();
                let v = if script {
                    plan.copy(self.slot(source)?, |v| {
                        Ok(match v {
                            value::Value::Object(o) if !self.life.valid(*o) => {
                                value::Value::Undefined
                            }
                            value::Value::Function(f)
                                if f.owner().is_some_and(|o| !self.life.valid(o)) =>
                            {
                                value::Value::Undefined
                            }
                            _ => v.clone(),
                        })
                    })?
                } else {
                    raw
                };
                if dest.starts_with("host:") {
                    self.set(dest, v)?;
                } else if let Some(Root::Slot(c)) = self.roots.get(dest) {
                    plan.write(c.clone(), v)?;
                    plan.commit()?;
                } else {
                    let c = plan.output(v)?;
                    plan.commit()?;
                    self.roots.insert(dest.into(), Root::Slot(c));
                    self.known_slots.insert(dest.into());
                }
            }
            "move" => {
                let source = self.slot(text(a, "source"))?;
                let dest = text(a, "dest");
                let mut p = self.life.plan();
                let c = p.transfer(source)?;
                p.commit()?;
                self.roots.insert(dest.into(), Root::Slot(c));
                self.known_slots.insert(dest.into());
            }
            "drop" => {
                let name = text(a, "slot");
                if name.starts_with("mem:") || name.starts_with("var:") {
                    self.slot(name)?.replace(value::Value::Undefined)?;
                } else {
                    self.roots.remove(name);
                }
            }
            "exec" => {
                let name = text(a, "id");
                let o = self.objects[text(a, "object")];
                let e = self.life.exec(o.slot)?;
                self.execs.insert(name.into(), e);
            }
            "exit" => {
                self.execs
                    .remove(text(a, "id"))
                    .ok_or_else(|| invalid("missing execution token"))?;
            }
            "delete" => self.store.remove(self.objects[text(a, "object")])?,
            "task_cleanup" => self
                .life
                .enqueue_task_cleanup(self.objects[text(a, "object")].slot),
            "view" => {
                let name = text(a, "header");
                let v = self.value(&json!(["header", name]))?;
                self.views.entry(name.into()).or_default().push(v);
            }
            "drop_view" => {
                self.views
                    .get_mut(text(a, "header"))
                    .and_then(Vec::pop)
                    .ok_or_else(|| invalid("missing physical view"))?;
            }
            "retire_host" => {
                let name = text(a, "slot");
                let guard = self.store.registry.entries.borrow();
                match self.roots.get_mut(name) {
                    Some(Root::Host { handle, .. }) => {
                        drop(handle.take());
                    }
                    _ => return Err(invalid("missing Host root")),
                };
                drop(guard);
            }
            "host_safepoint" => {
                self.store.registry.check()?;
            }
            "prepare_copy" => {
                let mut p = self.life.plan();
                let source = self.slot(text(a, "source"))?;
                let c = p.transfer(source)?;
                self.plans
                    .insert(text(a, "token").into(), (p, text(a, "dest").into(), c));
            }
            "commit_copy" => {
                let (p, name, c) = self
                    .plans
                    .remove(text(a, "token"))
                    .ok_or_else(|| invalid("missing copy token"))?;
                p.commit()?;
                self.known_slots.insert(name.clone());
                self.roots.insert(name, Root::Slot(c));
            }
            "abort_copy" => {
                self.plans
                    .remove(text(a, "token"))
                    .ok_or_else(|| invalid("missing copy token"))?;
            }
            "transfer" => {
                let placeholder = ObjectStore::with_bounded_lifecycle(
                    ValueLimits::default(),
                    self.life.ctx.quotas.unwrap(),
                )?;
                let old = std::mem::replace(&mut self.store, placeholder);
                self.store = Vm::new(old).into_objects();
            }
            "foreign" => {
                let other = ObjectStore::with_bounded_lifecycle(
                    ValueLimits::default(),
                    self.life.ctx.quotas.unwrap(),
                )?;
                let root = other.value(ValueInput::Undefined)?;
                self.store.retain(&root)?;
            }
            "close" => {
                let placeholder = ObjectStore::with_bounded_lifecycle(
                    ValueLimits::default(),
                    self.life.ctx.quotas.unwrap(),
                )?;
                drop(std::mem::replace(&mut self.store, placeholder));
                assert!(!self.registry.alive.get());
                for root in self.roots.values() {
                    if let Root::Host {
                        handle: Some(h), ..
                    } = root
                    {
                        assert_eq!(h.retain().unwrap_err().kind, VmErrorKind::InvalidState);
                    }
                }
            }
            "vm_pending" => {
                let id = self.store.create(self.program, &[ValueInput::Undefined])?;
                let name = text(a, "id");
                self.objects.insert(name.into(), id);
                let slot = format!("var:{name}:0");
                self.known_slots.insert(slot.clone());
                self.roots.insert(
                    slot,
                    Root::Slot(
                        self.store
                            .inner
                            .as_ref()
                            .unwrap()
                            .objects_for_audit_for_test(id.slot)
                            .unwrap()[0]
                            .clone(),
                    ),
                );
                let placeholder = ObjectStore::with_bounded_lifecycle(
                    ValueLimits::default(),
                    self.life.ctx.quotas.unwrap(),
                )?;
                let mut vm = Vm::new(std::mem::replace(&mut self.store, placeholder));
                vm.start(id, 0, &[])?;
                vm.run(1)?;
                vm.objects_mut().remove(id)?;
                assert_eq!(vm.run(64)?, RunOutcome::Finished(CompletionMeta::Returned));
                let result = vm.take_returned()?;
                assert_eq!(result.snapshot()?, ValueSnapshot::Int(9));
                let queued = vm
                    .maintain_lifecycle(LifecycleBudget {
                        host_roots: 0,
                        objects: 0,
                        metadata_entries: 0,
                    })?
                    .queued_objects;
                let drained = vm
                    .maintain_lifecycle(LifecycleBudget {
                        host_roots: 0,
                        objects: 1,
                        metadata_entries: 0,
                    })?
                    .drained_objects;
                self.store = vm.into_objects();
                return Ok(json!({"returned":9,"queued_before":queued,"drained":drained}));
            }
            "epoch_wrap" => {
                self.life.ctx.epoch.value.set(u64::MAX);
                let c = self.life.cell(value::Value::Undefined)?;
                drop(c);
            }
            "maintain" => {
                let b = &a["budget"];
                let budget = LifecycleBudget {
                    host_roots: number(b, "host_roots"),
                    objects: number(b, "objects"),
                    metadata_entries: number(b, "metadata_entries"),
                };
                let life = self.life.clone();
                let p = if let Some(name) = a.get("borrow_object").and_then(Json::as_str) {
                    let cell = self.slot(&format!("var:{name}:0"))?;
                    cell.with_read_borrow_for_test(|| self.store.maintain_lifecycle(budget))?
                } else if let Some(index) = a.get("borrow_index").and_then(Json::as_str) {
                    life.contract_borrow(index, || self.store.maintain_lifecycle(budget))?
                } else if let Some(after) = a.get("workspace_after").and_then(Json::as_u64) {
                    crate::copy_plan::with_workspace_failure(after as usize, || {
                        self.store.maintain_lifecycle(budget)
                    })?
                } else {
                    self.store.maintain_lifecycle(budget)?
                };
                return Ok(progress(&p));
            }
            _ => return Err(invalid("unknown contract action")),
        }
        Ok(Json::Null)
    }
    fn label(&self, v: &value::Value) -> Json {
        match v {
            value::Value::Object(id) => json!([
                "object",
                self.objects
                    .iter()
                    .find(|(_, v)| v.slot == *id)
                    .map(|(k, _)| k)
            ]),
            value::Value::Array(_) | value::Value::Mapping(_) | value::Value::Function(_) => {
                json!([
                    "header",
                    self.header_ids
                        .iter()
                        .find(|(_, n)| **n == v.counter().unwrap().id)
                        .map(|(k, _)| k)
                ])
            }
            _ => json!(["scalar", null]),
        }
    }
    fn observation(&self) -> Json {
        let raw = self.life.contract_observation();
        let object_names = self
            .objects
            .iter()
            .map(|(name, id)| (id.slot.get().to_string(), name.clone()))
            .collect::<BTreeMap<_, _>>();
        let objects = raw["objects"]
            .as_object()
            .unwrap()
            .iter()
            .map(|(id, v)| (object_names[id].clone(), v.clone()))
            .collect::<BTreeMap<_, _>>();
        let heads = self
            .header_ids
            .iter()
            .filter_map(|(name, id)| {
                raw["heads"]
                    .get(id.to_string())
                    .map(|h| (name.clone(), h.clone()))
            })
            .collect::<BTreeMap<_, _>>();
        let slots = self
            .known_slots
            .iter()
            .map(|n| {
                (
                    n.clone(),
                    self.read(n)
                        .map(|v| self.label(&v))
                        .unwrap_or(json!(["scalar", null])),
                )
            })
            .collect::<BTreeMap<_, _>>();
        let queue = raw["queue"]
            .as_array()
            .unwrap()
            .iter()
            .map(|id| object_names[&id.to_string()].clone())
            .collect::<Vec<_>>();
        let pending = self.registry.pending.get();
        json!({"semantic":{"objects":objects,"headers":heads,"slots":slots,"queue":queue,"function_heads_charged":raw["function_heads_charged"],"pending_roots":pending},"raw":raw})
    }
}

#[test]
#[ignore = "run by 2BJ contract_diff.py; actions-only input and independent oracle"]
fn replay_production_contract_traces() {
    use std::io::{BufRead, Write};
    let input = std::env::var_os("RML32_D08_TRACES").expect("explicit input");
    let output = std::env::var_os("RML32_D08_OBSERVATIONS").expect("explicit output");
    let reader = std::io::BufReader::new(std::fs::File::open(input).unwrap());
    let mut writer = std::io::BufWriter::new(std::fs::File::create(output).unwrap());
    let mut traces = 0;
    let mut steps = 0;
    for line in reader.lines() {
        let trace: Json = serde_json::from_str(&line.unwrap()).unwrap();
        assert!(trace.get("expected").is_none());
        let mut d = Driver::new(&trace["limits"]);
        let registry = Rc::downgrade(&d.store.registry);
        let ctx = Rc::downgrade(&d.life.ctx);
        for (step, a) in trace["actions"].as_array().unwrap().iter().enumerate() {
            let before = d.observation();
            let repeated = d.observation();
            assert_eq!(before, repeated, "observer changed state");
            let result = d.event(a);
            let after = d.observation();
            let accepted = result.is_ok();
            writeln!(writer,"{}",json!({"id":trace["id"],"step":step,"accepted":accepted,"error_kind":result.as_ref().err().map(|e|format!("{:?}",e.kind)),"progress":result.ok(),"before":before,"after":after})).unwrap();
            steps += 1;
            if !accepted
                && !a
                    .get("continue_on_error")
                    .and_then(Json::as_bool)
                    .unwrap_or(false)
            {
                break;
            }
        }
        drop(d);
        assert!(registry.upgrade().is_none());
        assert!(ctx.upgrade().is_none());
        traces += 1;
    }
    writer.flush().unwrap();
    println!(
        "PRODUCTION_CONTRACT_REPLAY {}",
        json!({"traces":traces,"steps":steps,"teardowns":traces})
    );
}
