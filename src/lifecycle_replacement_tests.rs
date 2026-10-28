//! 2BL actual transactions. Inputs contain actions only; no expected ledger.
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
}
impl Driver {
    fn new(limits: &Json) -> Self {
        let mut store =
            ObjectStore::with_bounded_lifecycle(ValueLimits::default(), q(limits)).unwrap();
        let program = store
            .register_program(
                prepare_program(image(&[fun(0, 0, 0, 2, &[(9, 0, 0x3b)])], 0, &[])).unwrap(),
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
        json!({"slots":slots,"objects":objects,"headers":headers,"function_heads_charged":raw["function_heads_charged"],"queue":raw["queue"]})
    }
    fn event(&mut self, a: &Json) -> Result<(), VmError> {
        match name(a, "op") {
            "object" => {
                let id = self.store.create(self.program, &[])?;
                self.objects.insert(name(a, "id").into(), id);
            }
            "make" => {
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
                let cell = self.life.cell(header)?;
                if let Some(g) = guard {
                    g.commit();
                }
                self.headers.insert(n.into(), (id, weak));
                let m = format!("m:{n}");
                self.known.insert(m.clone());
                self.members.insert(m, member);
                let dest = name(a, "dest");
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
                self.roots.remove(name(a, "slot"));
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
                    let value = plan.copy(source, |v| Ok(v.clone()))?;
                    plan.write(dest, value)?;
                }
                self.plan = Some(plan);
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
                    plan.commit()?;
                }
            }
            "abort" => {
                self.plan.take();
                self.raw.take();
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
        calls: usize,
    ) {
        let b = self.state(snapshots.0);
        let a = self.state(snapshots.1);
        writeln!(self.out,"{}",json!({"id":id,"step":step,"before":b,"after":a,"error":error,"setup_step":setup_step,"calls":calls})).unwrap();
        self.rows += 1;
    }
}
#[test]
#[ignore = "2BL actions-only replacement replay"]
fn replay_replacement_traces() {
    use std::io::{BufRead, Write};
    let input = std::fs::File::open(std::env::var_os("RML32_REPLACEMENTS_IN").unwrap()).unwrap();
    let output = std::io::BufWriter::new(
        std::fs::File::create(std::env::var_os("RML32_REPLACEMENTS_OUT").unwrap()).unwrap(),
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
            0,
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
                    calls,
                );
                if error.is_some() {
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
        "REPLACEMENT_REPLAY {}",
        json!({"traces":count,"rows":writer.rows,"states":writer.states.len()})
    );
}
fn high() -> Json {
    json!({"max_objects":3,"max_function_heads":2,"max_ordinary_refs_per_object":12,"max_exec_refs_per_object":2,"max_owners_per_header":12})
}
fn base() -> Driver {
    let mut d = Driver::new(&high());
    for n in ["o0", "o1", "o2"] {
        d.event(&json!({"op":"object","id":n})).unwrap();
    }
    for (n, v) in [
        ("s0", json!(["object", "o0"])),
        ("s1", json!(["object", "o1"])),
        ("u", json!(["undefined"])),
    ] {
        d.event(&json!({"op":"put","dest":n,"value":v})).unwrap();
    }
    d
}
#[test]
fn occupied_same_header_requires_peak_before_old_owner_release() {
    let mut limits = high();
    limits["max_owners_per_header"] = json!(1);
    let mut d = Driver::new(&limits);
    d.event(&json!({"op":"make","id":"h0","kind":"array","dest":"d0","value":["undefined"]}))
        .unwrap();
    let before = d.observe();
    d.event(&json!({"op":"prepare","writes":[["d0","d0"]]}))
        .unwrap();
    assert_eq!(
        d.event(&json!({"op":"commit"})).unwrap_err().kind,
        VmErrorKind::Limit
    );
    assert_eq!(d.observe(), before);
}
#[test]
fn final_root_replacement_clears_retained_members_and_creator_once() {
    for kind in ["array", "mapping", "function"] {
        let mut d = base();
        let value = if kind == "function" {
            d.event(&json!({"op":"make","id":"child","kind":"array","dest":"build","value":["object","o1"]})).unwrap();
            json!(["header", "child"])
        } else {
            json!(["object", "o1"])
        };
        d.event(&json!({"op":"make","id":"h0","kind":kind,"dest":"d0","creator":if kind=="function"{json!("o2")}else{Json::Null},"value":value})).unwrap();
        if kind == "function" {
            d.event(&json!({"op":"drop","slot":"build"})).unwrap();
        }
        d.event(&json!({"op":"view","header":"h0"})).unwrap();
        d.event(&json!({"op":"prepare","writes":[["u","d0"],["s0","m:h0"]]}))
            .unwrap();
        d.event(&json!({"op":"commit"})).unwrap();
        let p = d.observe();
        assert_eq!(p["headers"]["h0"]["retired"], true);
        assert_eq!(p["slots"]["m:h0"], json!(["undefined"]));
        assert_eq!(p["objects"]["o0"]["ordinary"], 1);
        assert_eq!(p["objects"]["o1"]["ordinary"], 1);
        assert_eq!(p["objects"]["o2"]["ordinary"], 0);
        assert!(d.views.contains_key("h0"));
    }
}
#[test]
fn shared_child_survives_external_alias_then_retires_exactly_once() {
    let mut d = base();
    for a in [
        json!({"op":"make","id":"child","kind":"array","dest":"alias","value":["object","o1"]}),
        json!({"op":"make","id":"h0","kind":"array","dest":"d0","value":["header","child"]}),
        json!({"op":"make","id":"h1","kind":"mapping","dest":"d1","value":["header","child"]}),
        json!({"op":"prepare","writes":[["u","d0"],["u","d1"]]}),
        json!({"op":"commit"}),
    ] {
        d.event(&a).unwrap();
    }
    assert_eq!(d.observe()["headers"]["child"]["owners"], 1);
    assert_eq!(d.observe()["objects"]["o1"]["ordinary"], 2);
    d.event(&json!({"op":"drop","slot":"alias"})).unwrap();
    assert_eq!(d.observe()["headers"]["child"]["retired"], true);
    assert_eq!(d.observe()["objects"]["o1"]["ordinary"], 1);
}
#[test]
fn occupied_raw_destination_rejects_without_transferring_payload() {
    let mut d = base();
    d.event(&json!({"op":"put","dest":"d0","value":["object","o1"]}))
        .unwrap();
    let before = d.observe();
    d.event(&json!({"op":"prepare","raw":true,"writes":[["s0","d0"]]}))
        .unwrap();
    assert_eq!(
        d.event(&json!({"op":"commit"})).unwrap_err().kind,
        VmErrorKind::InvalidState
    );
    assert_eq!(d.observe(), before);
}
#[test]
fn old_member_read_borrow_blocks_retirement_without_partial_publication() {
    let mut d = base();
    d.event(&json!({"op":"make","id":"h0","kind":"array","dest":"d0","value":["object","o1"]}))
        .unwrap();
    let before = d.observe();
    d.event(&json!({"op":"prepare","writes":[["s0","d0"]]}))
        .unwrap();
    assert_eq!(
        d.event(&json!({"op":"commit","borrow":"m:h0"}))
            .unwrap_err()
            .kind,
        VmErrorKind::InvalidState
    );
    assert_eq!(d.observe(), before);
}
