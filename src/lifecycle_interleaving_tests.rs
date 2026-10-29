//! 2BM state transitions and actual transactions. Inputs contain actions only; no expected ledger.
use super::*;
use crate::cell::Cell as Slot;
use crate::copy_plan::CopyPlan;
use crate::{
    LifecycleBudget, LifecycleLimits, ProgramImage, ProgramMode, parse_program, prepare_program,
};
use serde_json::{Value as Json, json};
use std::collections::BTreeMap;
include!("../tests/boundary_support.rs.inc");
fn q(j: &Json) -> LifecycleLimits {
    LifecycleLimits {
        max_objects: j["max_objects"].as_u64().unwrap() as usize,
        max_function_heads: j["max_function_heads"].as_u64().unwrap() as usize,
        max_ordinary_refs_per_object: j["max_ordinary_refs_per_object"].as_u64().unwrap() as usize,
        max_exec_refs_per_object: j["max_exec_refs_per_object"].as_u64().unwrap() as usize,
        max_owners_per_header: j["max_owners_per_header"].as_u64().unwrap() as usize,
    }
}
fn name<'a>(a: &'a Json, k: &str) -> &'a str {
    a[k].as_str().expect("typed action")
}
struct Driver {
    store: ObjectStore,
    life: Rc<crate::lifecycle_adapter::runtime::StoreState>,
    program: ProgramId,
    objects: BTreeMap<String, ObjectId>,
    roots: BTreeMap<String, Rc<Slot>>,
    members: BTreeMap<String, Rc<Slot>>,
    headers: BTreeMap<String, (u64, crate::graph::WeakHeader)>,
    known: BTreeSet<String>,
    views: BTreeMap<String, value::Value>,
    plan: Option<CopyPlan>,
    raw: Option<(Rc<Slot>, Rc<Slot>)>,
    execs: BTreeMap<String, crate::lifecycle_adapter::runtime::ExecLease>,
    pending_head: Option<(
        String,
        String,
        crate::lifecycle_adapter::runtime::HeadReservation,
    )>,
    move_dest: Option<String>,
    detail: bool,
    progress: Json,
}
impl Driver {
    fn new(limits: &Json) -> Self {
        let mut store =
            ObjectStore::with_bounded_lifecycle(ValueLimits::default(), q(limits)).unwrap();
        let program = store
            .register_program(
                prepare_program(image(&[fun(0, 0, 0, 2, &[(9, 0, 0x3b)])], 1, &[])).unwrap(),
            )
            .unwrap();
        let life = store.registry.test_life.as_ref().unwrap().clone();
        Self {
            store,
            life,
            program,
            objects: BTreeMap::new(),
            roots: BTreeMap::new(),
            members: BTreeMap::new(),
            headers: BTreeMap::new(),
            known: BTreeSet::new(),
            views: BTreeMap::new(),
            plan: None,
            raw: None,
            execs: BTreeMap::new(),
            pending_head: None,
            move_dest: None,
            detail: false,
            progress: Json::Null,
        }
    }
    fn slot(&self, n: &str) -> Rc<Slot> {
        self.roots
            .get(n)
            .or_else(|| self.members.get(n))
            .expect("known stable slot")
            .clone()
    }
    fn value(&self, v: &Json) -> value::Value {
        match v[0].as_str().unwrap() {
            "undefined" => value::Value::Undefined,
            "int" => value::Value::Int(v[1].as_i64().unwrap() as i32),
            "object" => value::Value::Object(self.objects[v[1].as_str().unwrap()].slot),
            "header" => self.headers[v[1].as_str().unwrap()]
                .1
                .upgrade()
                .expect("live header"),
            _ => panic!("value kind"),
        }
    }
    fn label(&self, v: value::Value) -> Json {
        match v {
            value::Value::Undefined => json!(["undefined"]),
            value::Value::Int(n) => json!(["int", n]),
            value::Value::Object(id) => json!([
                "object",
                self.objects.iter().find(|(_, v)| v.slot == id).unwrap().0
            ]),
            v => json!([
                "header",
                self.headers
                    .iter()
                    .find(|(_, h)| h.0 == v.counter().unwrap().id)
                    .unwrap()
                    .0
            ]),
        }
    }
    fn observe(&self) -> Json {
        let raw = self.life.contract_observation();
        let slots = self
            .known
            .iter()
            .map(|n| {
                let v = self
                    .roots
                    .get(n)
                    .or_else(|| self.members.get(n))
                    .map(|c| self.label(c.read().unwrap()))
                    .unwrap_or(json!(["undefined"]));
                (n.clone(), v)
            })
            .collect::<BTreeMap<_, _>>();
        let objects = self
            .objects
            .iter()
            .map(|(n, id)| (n.clone(), raw["objects"][id.slot.get().to_string()].clone()))
            .collect::<BTreeMap<_, _>>();
        let headers = self
            .headers
            .iter()
            .map(|(n, (id, _))| (n.clone(), raw["heads"][id.to_string()].clone()))
            .collect::<BTreeMap<_, _>>();
        let queue = raw["queue"]
            .as_array()
            .unwrap()
            .iter()
            .map(|id| {
                self.objects
                    .iter()
                    .find(|(_, o)| o.slot.get() as u64 == id.as_u64().unwrap())
                    .unwrap()
                    .0
                    .clone()
            })
            .collect::<Vec<_>>();
        let semantic = json!({"slots":slots,"objects":objects,"headers":headers,"function_heads_charged":raw["function_heads_charged"],"queue":queue});
        if self.detail {
            json!({"semantic":semantic,"raw":raw})
        } else {
            semantic
        }
    }
    fn event(&mut self, a: &Json) -> Result<(), VmError> {
        if let Some(n) = a.get("borrow").and_then(Json::as_str) {
            let cell = self.slot(n);
            let mut inner = a.clone();
            inner.as_object_mut().unwrap().remove("borrow");
            return cell.with_read_borrow_for_test(|| self.event(&inner));
        }
        match name(a, "op") {
            "object" => {
                let id = self.store.create(self.program, &[ValueInput::Undefined])?;
                let n = name(a, "id");
                let cell = self
                    .store
                    .inner
                    .as_ref()
                    .unwrap()
                    .objects_for_audit_for_test(id.slot)
                    .unwrap()[0]
                    .clone();
                self.members.insert(format!("v:{n}"), cell);
                self.known.insert(format!("v:{n}"));
                self.objects.insert(n.into(), id);
            }
            "make" | "prepare_head" => {
                if let Some(n) = a.get("creator").and_then(Json::as_str)
                    && !self.life.valid(self.objects[n].slot)
                {
                    return Err(VmError::new(VmErrorKind::InvalidObject, "invalid creator"));
                }
                let v = self.value(&a["value"]);
                let n = name(a, "id");
                let mut guard = None;
                let header = match name(a, "kind") {
                    "array" => self.life.array(vec![v])?,
                    "mapping" => self.life.mapping(vec![(value::Value::Int(0), v)])?,
                    "function" => {
                        let member = self.life.cell(v)?;
                        let f = value::FunctionValue(Rc::new(value::FunctionData {
                            origin: self.store.inner.as_ref().unwrap().function_origin.clone(),
                            owner: a
                                .get("creator")
                                .and_then(Json::as_str)
                                .map(|n| self.objects[n].slot),
                            target: value::FunctionTarget::Global {
                                program: value::ProgramId(0),
                                function: 0,
                            },
                            bound: vec![member],
                            bound_count: 1,
                            capture_profile: crate::value::CaptureProfile::Container,
                            counter: Counter::new()?,
                        }));
                        guard = Some(self.life.reserve_function(&f)?);
                        value::Value::Function(f)
                    }
                    _ => panic!("header kind"),
                };
                let member = match &header {
                    value::Value::Function(f) => f.0.bound[0].clone(),
                    _ => crate::container::lifecycle_members(&header)?[0].clone(),
                };
                let id = header.counter().unwrap().id;
                let weak = match &header {
                    value::Value::Array(h) => crate::graph::WeakHeader::Array(Rc::downgrade(&h.0)),
                    value::Value::Mapping(h) => {
                        crate::graph::WeakHeader::Mapping(Rc::downgrade(&h.0), Rc::downgrade(&h.1))
                    }
                    value::Value::Function(h) => {
                        crate::graph::WeakHeader::Function(Rc::downgrade(&h.0))
                    }
                    _ => unreachable!(),
                };
                let staged = name(a, "op") == "prepare_head";
                let cell = if staged {
                    self.life.plan().output(header)?
                } else {
                    self.life.cell(header)?
                };
                let dest = if staged { "tmp:head" } else { name(a, "dest") };
                if staged {
                    self.pending_head =
                        Some((n.into(), name(a, "dest").into(), guard.take().unwrap()));
                } else if let Some(g) = guard {
                    g.validate()?;
                    g.commit();
                }
                self.headers.insert(n.into(), (id, weak));
                let m = format!("m:{n}");
                self.known.insert(m.clone());
                self.members.insert(m, member);
                self.known.insert(dest.into());
                self.roots.insert(dest.into(), cell);
            }
            "put" => {
                let dest = name(a, "dest");
                let value = self.value(&a["value"]);
                if self.roots.contains_key(dest) || self.members.contains_key(dest) {
                    self.slot(dest).replace(value)?;
                } else {
                    self.roots.insert(dest.into(), self.life.cell(value)?);
                }
                self.known.insert(dest.into());
            }
            "drop" => {
                let n = name(a, "slot");
                if let Some(cell) = self.members.get(n) {
                    cell.replace(value::Value::Undefined)?;
                } else {
                    self.roots.remove(n);
                }
            }
            "view" => {
                let n = name(a, "header");
                if let Some((id, h)) = self.headers.get(n) {
                    let raw = self.life.contract_observation();
                    if raw["heads"][id.to_string()]["retired"] == false {
                        self.views.insert(n.into(), h.upgrade().unwrap());
                    }
                }
            }
            "unview" => {
                self.views.remove(name(a, "header"));
            }
            "zero" => {
                let before = self.life.contract_observation();
                let p = self.store.maintain_lifecycle(LifecycleBudget {
                    host_roots: 0,
                    objects: 0,
                    metadata_entries: 0,
                })?;
                assert_eq!(
                    (p.retired_roots, p.drained_objects, p.metadata_examined),
                    (0, 0, 0)
                );
                assert_eq!(before, self.life.contract_observation());
            }
            "prepare" => {
                if a.get("raw").and_then(Json::as_bool).unwrap_or(false) {
                    let pair = &a["writes"][0];
                    self.raw = Some((
                        self.slot(pair[0].as_str().unwrap()),
                        self.slot(pair[1].as_str().unwrap()),
                    ));
                    return Ok(());
                }
                let mut plan = self.life.plan();
                for pair in a["writes"].as_array().unwrap() {
                    let source = self.slot(pair[0].as_str().unwrap());
                    let dest = self.slot(pair[1].as_str().unwrap());
                    let script = a.get("script").and_then(Json::as_bool).unwrap_or(false);
                    let value = plan.copy(source, |v| {
                        Ok(
                            if script
                                && match v {
                                    value::Value::Object(o) => !self.life.valid(*o),
                                    value::Value::Function(f) => {
                                        f.owner().is_some_and(|o| !self.life.valid(o))
                                    }
                                    _ => false,
                                }
                            {
                                value::Value::Undefined
                            } else {
                                v.clone()
                            },
                        )
                    })?;
                    plan.write(dest, value)?;
                }
                self.plan = Some(plan);
            }
            "prepare_move" => {
                let mut p = self.life.plan();
                let output = p.transfer(self.slot(name(a, "source")))?;
                self.roots.insert("tmp:move".into(), output);
                self.known.insert("tmp:move".into());
                self.move_dest = Some(name(a, "dest").into());
                self.plan = Some(p);
            }
            "commit_head" => {
                let (_, dest, guard) = self.pending_head.take().expect("prepared head");
                if let Err(e) = guard.validate() {
                    self.roots.remove("tmp:head");
                    drop(guard);
                    return Err(e);
                }
                let cell = self.roots.remove("tmp:head").unwrap();
                self.known.insert(dest.clone());
                self.roots.insert(dest, cell);
                guard.commit();
            }
            "abort_head" => {
                self.roots.remove("tmp:head");
                drop(self.pending_head.take());
            }
            "exec" | "prepare_exec" => {
                let e = self.life.exec(self.objects[name(a, "object")].slot)?;
                self.execs.insert(name(a, "id").into(), e);
            }
            "commit_exec" => {
                let e = self.execs.remove(name(a, "id")).expect("prepared lease");
                self.execs.insert(name(a, "dest").into(), e);
            }
            "exit" | "abort_exec" => {
                self.execs.remove(name(a, "id"));
            }
            "delete" => {
                self.store.remove(self.objects[name(a, "object")])?;
            }
            "maintain" => {
                let budget = LifecycleBudget {
                    host_roots: 0,
                    objects: a["objects"].as_u64().unwrap_or(0) as usize,
                    metadata_entries: a["metadata"].as_u64().unwrap_or(0) as usize,
                };
                let p = if let Some(index) = a.get("borrow_index").and_then(Json::as_str) {
                    self.life
                        .contract_borrow(index, || self.store.maintain_lifecycle(budget))?
                } else {
                    self.store.maintain_lifecycle(budget)?
                };
                self.progress = json!({"retired_roots":p.retired_roots,"drained_objects":p.drained_objects,"reaped_objects":p.reaped_objects,"reclaimed_heads":p.reclaimed_heads,"pruned_weak_entries":p.pruned_weak_entries,"metadata_examined":p.metadata_examined,"pending_roots":p.pending_roots,"queued_objects":p.queued_objects,"scan_complete":p.scan_complete,"stop":format!("{:?}",p.stop)});
            }
            "commit" => {
                if let Some((s, d)) = self.raw.take() {
                    return Slot::transfer_batch(Vec::new(), vec![(s, d)], Vec::new(), Vec::new());
                }
                let plan = self.plan.take().expect("prepared transaction");
                if let Some(n) = a.get("borrow").and_then(Json::as_str) {
                    let cell = self.slot(n);
                    cell.with_read_borrow_for_test(|| plan.commit())?;
                } else {
                    let result = plan.commit();
                    if let Err(e) = result {
                        self.roots.remove("tmp:move");
                        self.move_dest.take();
                        return Err(e);
                    }
                }
                if let Some(dest) = self.move_dest.take() {
                    let cell = self.roots.remove("tmp:move").unwrap();
                    self.known.insert(dest.clone());
                    self.roots.insert(dest, cell);
                }
            }
            "abort" => {
                self.plan.take();
                self.raw.take();
                self.roots.remove("tmp:move");
                self.move_dest.take();
            }
            _ => panic!("action"),
        }
        Ok(())
    }
}
struct Writer<W: std::io::Write> {
    out: W,
    states: BTreeMap<String, usize>,
    rows: usize,
}
impl<W: std::io::Write> Writer<W> {
    fn state(&mut self, value: Json) -> usize {
        let key = value.to_string();
        if let Some(id) = self.states.get(&key) {
            return *id;
        }
        let id = self.states.len();
        writeln!(self.out, "{}", json!({"state":id,"value":value})).unwrap();
        self.states.insert(key, id);
        id
    }
    fn row(
        &mut self,
        id: &Json,
        step: usize,
        snapshots: (Json, Json),
        error: Option<String>,
        setup_step: Option<usize>,
        work: (usize, Json),
    ) {
        let (calls, progress) = work;
        let b = self.state(snapshots.0);
        let a = self.state(snapshots.1);
        writeln!(self.out,"{}",json!({"id":id,"step":step,"before":b,"after":a,"error":error,"setup_step":setup_step,"calls":calls,"progress":progress})).unwrap();
        self.rows += 1;
    }
}
#[test]
#[ignore = "2BM actions-only state interleaving replay"]
fn replay_interleaving_traces() {
    use std::io::{BufRead, Write};
    let input = std::fs::File::open(std::env::var_os("RML32_INTERLEAVINGS_IN").unwrap()).unwrap();
    let output = std::io::BufWriter::new(
        std::fs::File::create(std::env::var_os("RML32_INTERLEAVINGS_OUT").unwrap()).unwrap(),
    );
    let mut writer = Writer {
        out: output,
        states: BTreeMap::new(),
        rows: 0,
    };
    let mut count = 0;
    for line in std::io::BufReader::new(input).lines() {
        let t: Json = serde_json::from_str(&line.unwrap()).unwrap();
        assert!(t.get("expected").is_none());
        let mut d = Driver::new(&t["limits"]);
        d.detail = t.get("detail").and_then(Json::as_bool).unwrap_or(false);
        let registry = Rc::downgrade(&d.store.registry);
        let ctx = Rc::downgrade(&d.life.ctx);
        let before = d.observe();
        let mut error = None;
        let mut setup_step = None;
        for (i, a) in t["setup"].as_array().unwrap().iter().enumerate() {
            if let Err(e) = d.event(a) {
                error = Some(format!("{:?}", e.kind));
                setup_step = Some(i);
                break;
            }
        }
        writer.row(
            &t["id"],
            0,
            (before, d.observe()),
            error.clone(),
            setup_step,
            (0, Json::Null),
        );
        if error.is_none() {
            for (i, a) in t["actions"].as_array().unwrap().iter().enumerate() {
                let before = d.observe();
                assert_eq!(before, d.observe(), "observer modified state");
                let (r, calls) = crate::copy_plan::with_workspace_count(|| {
                    if let Some(n) = a.get("fail_after").and_then(Json::as_u64) {
                        crate::copy_plan::with_workspace_failure(n as usize, || d.event(a))
                    } else {
                        d.event(a)
                    }
                });
                let error = r.err().map(|e| format!("{:?}", e.kind));
                writer.row(
                    &t["id"],
                    i + 1,
                    (before, d.observe()),
                    error.clone(),
                    None,
                    (calls, std::mem::take(&mut d.progress)),
                );
                if error.is_some()
                    && !a
                        .get("continue_on_error")
                        .and_then(Json::as_bool)
                        .unwrap_or(false)
                {
                    break;
                }
            }
        }
        drop(d);
        assert!(registry.upgrade().is_none());
        assert!(ctx.upgrade().is_none());
        count += 1;
    }
    writer.out.flush().unwrap();
    println!(
        "INTERLEAVING_REPLAY {}",
        json!({"traces":count,"rows":writer.rows,"states":writer.states.len()})
    );
}

fn high() -> Json {
    json!({"max_objects":3,"max_function_heads":2,"max_ordinary_refs_per_object":16,"max_exec_refs_per_object":3,"max_owners_per_header":16})
}
fn base() -> Driver {
    let mut d = Driver::new(&high());
    for a in [
        json!({"op":"object","id":"o0"}),
        json!({"op":"object","id":"o1"}),
        json!({"op":"put","dest":"s0","value":["object","o0"]}),
        json!({"op":"put","dest":"d0","value":["undefined"]}),
    ] {
        d.event(&a).unwrap();
    }
    d
}

#[test]
fn script_normalization_is_frozen_at_copy_preparation_not_replayed_at_commit() {
    for before in [false, true] {
        let mut d = base();
        if before {
            d.event(&json!({"op":"delete","object":"o0"})).unwrap();
        }
        d.event(&json!({"op":"prepare","writes":[["s0","d0"]],"script":true}))
            .unwrap();
        if !before {
            d.event(&json!({"op":"delete","object":"o0"})).unwrap();
        }
        d.event(&json!({"op":"commit"})).unwrap();
        assert_eq!(
            d.observe()["slots"]["d0"],
            if before {
                json!(["undefined"])
            } else {
                json!(["object", "o0"])
            }
        );
        assert_eq!(d.observe()["slots"]["s0"], d.observe()["slots"]["d0"]);
    }
}

#[test]
fn prepared_function_accepts_pending_creator_and_aborts_after_queue_invalidation() {
    for exit in [false, true] {
        let mut d = base();
        d.event(&json!({"op":"exec","id":"e0","object":"o0"}))
            .unwrap();
        d.event(&json!({"op":"prepare_head","id":"h0","kind":"function","dest":"d0","creator":"o0","value":["undefined"]})).unwrap();
        d.event(&json!({"op":"delete","object":"o0"})).unwrap();
        if exit {
            d.event(&json!({"op":"exit","id":"e0"})).unwrap();
        }
        let result = d.event(&json!({"op":"commit_head"}));
        if exit {
            assert_eq!(result.unwrap_err().kind, VmErrorKind::InvalidObject);
            assert_eq!(d.observe()["function_heads_charged"], 0);
            assert_eq!(d.observe()["objects"]["o0"]["ordinary"], 2);
            assert_eq!(d.observe()["slots"]["d0"], json!(["undefined"]));
        } else {
            result.unwrap();
            assert_eq!(d.observe()["function_heads_charged"], 1);
            assert_eq!(d.observe()["objects"]["o0"]["state"], "pending");
        }
    }
}

#[test]
fn aborting_last_prospective_execution_lease_uses_reserved_queue_unit_once() {
    let mut q = high();
    q["max_ordinary_refs_per_object"] = json!(2);
    let mut d = Driver::new(&q);
    for a in [
        json!({"op":"object","id":"o0"}),
        json!({"op":"put","dest":"s0","value":["object","o0"]}),
        json!({"op":"exec","id":"installed","object":"o0"}),
        json!({"op":"prepare_exec","id":"prospective","object":"o0"}),
        json!({"op":"delete","object":"o0"}),
        json!({"op":"exit","id":"installed"}),
    ] {
        d.event(&a).unwrap();
    }
    assert_eq!(d.observe()["objects"]["o0"]["state"], "pending");
    d.event(&json!({"op":"abort_exec","id":"prospective"}))
        .unwrap();
    assert_eq!(d.observe()["queue"], json!(["o0"]));
    assert_eq!(d.observe()["objects"]["o0"]["ordinary"], 2);
    d.event(&json!({"op":"abort_exec","id":"prospective"}))
        .unwrap();
    assert_eq!(d.observe()["queue"], json!(["o0"]));
}

#[test]
fn variable_cleanup_invalidates_prepared_source_and_destination_versions() {
    for writes in [json!([["v:o0", "d0"]]), json!([["s0", "v:o0"]])] {
        let mut d = base();
        d.event(&json!({"op":"put","dest":"v:o0","value":["object","o1"]}))
            .unwrap();
        d.event(&json!({"op":"prepare","writes":writes})).unwrap();
        d.event(&json!({"op":"delete","object":"o0"})).unwrap();
        let before = d.observe();
        assert_eq!(
            d.event(&json!({"op":"commit"})).unwrap_err().kind,
            VmErrorKind::InvalidState
        );
        assert_eq!(d.observe(), before);
        assert_eq!(d.observe()["slots"]["v:o0"], json!(["undefined"]));
    }
}

#[test]
fn raw_move_preserves_invalid_payload_at_ceiling_without_adding_an_owner() {
    let mut q = high();
    q["max_ordinary_refs_per_object"] = json!(2);
    let mut d = Driver::new(&q);
    for a in [
        json!({"op":"object","id":"o0"}),
        json!({"op":"put","dest":"s0","value":["object","o0"]}),
        json!({"op":"put","dest":"d0","value":["undefined"]}),
        json!({"op":"prepare_move","source":"s0","dest":"d0"}),
        json!({"op":"delete","object":"o0"}),
        json!({"op":"commit"}),
    ] {
        d.event(&a).unwrap();
    }
    assert_eq!(d.observe()["slots"]["s0"], json!(["undefined"]));
    assert_eq!(d.observe()["slots"]["d0"], json!(["object", "o0"]));
    assert_eq!(d.observe()["objects"]["o0"]["ordinary"], 1);
}

#[test]
fn borrowed_queued_variable_preserves_head_then_retry_finishes_remaining_object() {
    let mut d = base();
    for o in ["o0", "o1"] {
        d.event(&json!({"op":"exec","id":o,"object":o})).unwrap();
        d.event(&json!({"op":"delete","object":o})).unwrap();
        d.event(&json!({"op":"exit","id":o})).unwrap();
    }
    d.event(&json!({"op":"maintain","objects":2,"borrow":"v:o0"}))
        .unwrap();
    assert_eq!(d.progress["stop"], "BorrowBlocked");
    assert_eq!(d.progress["drained_objects"], 1);
    assert_eq!(d.observe()["queue"], json!(["o0"]));
    d.event(&json!({"op":"maintain","objects":1})).unwrap();
    assert_eq!(d.progress["drained_objects"], 1);
    assert_eq!(d.observe()["queue"], json!([]));
}

#[test]
fn workspace_failure_after_one_drain_preserves_partial_progress_and_remaining_queue() {
    fn queued() -> Driver {
        let mut d = base();
        for o in ["o0", "o1"] {
            for a in [
                json!({"op":"exec","id":o,"object":o}),
                json!({"op":"delete","object":o}),
                json!({"op":"exit","id":o}),
            ] {
                d.event(&a).unwrap();
            }
        }
        d
    }
    let mut control = queued();
    let (_, calls) = crate::copy_plan::with_workspace_count(|| {
        control
            .event(&json!({"op":"maintain","objects":1}))
            .unwrap()
    });
    assert!(calls > 0);
    let mut d = queued();
    crate::copy_plan::with_workspace_failure(calls, || {
        d.event(&json!({"op":"maintain","objects":2}))
    })
    .unwrap();
    assert_eq!(d.progress["drained_objects"], 1);
    assert_eq!(d.progress["stop"], "WorkspaceLimit");
    assert_eq!(d.observe()["queue"], json!(["o0"]));
    assert_eq!(d.observe()["objects"]["o0"]["cleaned"], false);
    d.event(&json!({"op":"maintain","objects":1})).unwrap();
    assert_eq!(d.progress["drained_objects"], 1);
    assert_eq!(d.observe()["queue"], json!([]));
}
