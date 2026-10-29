//! Two independently owned preparations. Observers retain Weak slots only.
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

enum Prepared {
    Copy(CopyPlan),
    Head {
        cell: Rc<Slot>,
        guard: HeadReservation,
        dest: String,
    },
    Exec(ExecLease),
}
struct Driver {
    store: ObjectStore,
    life: Rc<StoreState>,
    program: ProgramId,
    objects: BTreeMap<String, ObjectId>,
    roots: BTreeMap<String, Rc<Slot>>,
    slots: BTreeMap<String, std::rc::Weak<Slot>>,
    headers: BTreeMap<String, (u64, crate::graph::WeakHeader)>,
    views: BTreeMap<String, value::Value>,
    transactions: BTreeMap<String, Prepared>,
    leases: BTreeMap<String, ExecLease>,
    detail: bool,
    progress: Json,
    slot_ids: BTreeMap<String, u64>,
}
fn text<'a>(a: &'a Json, key: &str) -> &'a str {
    a[key].as_str().expect("typed action")
}
fn invalid() -> VmError {
    VmError::new(
        VmErrorKind::InvalidState,
        "invalid pair action or expired slot",
    )
}
fn high() -> Json {
    json!({"max_objects":3,"max_function_heads":4,"max_ordinary_refs_per_object":32,"max_exec_refs_per_object":4,"max_owners_per_header":32})
}
impl Driver {
    fn new(q: &Json) -> Self {
        let mut store = ObjectStore::with_bounded_lifecycle(
            crate::ValueLimits::default(),
            LifecycleLimits {
                max_objects: q["max_objects"].as_u64().unwrap() as usize,
                max_function_heads: q["max_function_heads"].as_u64().unwrap() as usize,
                max_ordinary_refs_per_object: q["max_ordinary_refs_per_object"].as_u64().unwrap()
                    as usize,
                max_exec_refs_per_object: q["max_exec_refs_per_object"].as_u64().unwrap() as usize,
                max_owners_per_header: q["max_owners_per_header"].as_u64().unwrap() as usize,
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
            store,
            life,
            program,
            objects: BTreeMap::new(),
            roots: BTreeMap::new(),
            slots: BTreeMap::new(),
            headers: BTreeMap::new(),
            views: BTreeMap::new(),
            transactions: BTreeMap::new(),
            leases: BTreeMap::new(),
            detail: false,
            progress: Json::Null,
            slot_ids: BTreeMap::new(),
        }
    }
    fn slot(&self, n: &str) -> Result<Rc<Slot>, VmError> {
        if n.starts_with("p:") {
            return Err(invalid());
        }
        self.roots
            .get(n)
            .cloned()
            .or_else(|| self.slots.get(n).and_then(std::rc::Weak::upgrade))
            .ok_or_else(invalid)
    }
    fn slot_label(&self, cell: &Slot) -> String {
        self.slots
            .iter()
            .find(|(_, w)| w.upgrade().is_some_and(|c| c.id() == cell.id()))
            .expect("known cell")
            .0
            .clone()
    }
    fn value(&self, v: &Json) -> Result<value::Value, VmError> {
        Ok(match v[0].as_str().unwrap() {
            "undefined" => value::Value::Undefined,
            "int" => value::Value::Int(v[1].as_i64().unwrap() as i32),
            "object" => value::Value::Object(self.objects[v[1].as_str().unwrap()].slot),
            "header" => {
                let header = &self.headers[v[1].as_str().unwrap()];
                if self.private_header(header.0) {
                    return Err(invalid());
                }
                header.1.upgrade().ok_or_else(invalid)?
            }
            _ => return Err(invalid()),
        })
    }
    fn private_header(&self, id: u64) -> bool {
        self.transactions.values().any(|p| match p {
            Prepared::Head { cell, .. } => {
                cell.read().unwrap().counter().is_some_and(|c| c.id == id)
            }
            _ => false,
        })
    }
    fn label(&self, v: value::Value) -> Json {
        match v {
            value::Value::Undefined => json!(["undefined"]),
            value::Value::Int(n) => json!(["int", n]),
            value::Value::Object(o) => json!([
                "object",
                self.objects.iter().find(|(_, id)| id.slot == o).unwrap().0
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
    fn remember(&mut self, n: String, c: &Rc<Slot>) {
        self.slot_ids.insert(n.clone(), c.id());
        self.slots.insert(n, Rc::downgrade(c));
    }
    fn track_head(&mut self, n: &str, v: &value::Value) {
        let id = v.counter().unwrap().id;
        let weak = match v {
            value::Value::Array(a) => crate::graph::WeakHeader::Array(Rc::downgrade(&a.0)),
            value::Value::Mapping(m) => {
                crate::graph::WeakHeader::Mapping(Rc::downgrade(&m.0), Rc::downgrade(&m.1))
            }
            value::Value::Function(f) => crate::graph::WeakHeader::Function(Rc::downgrade(&f.0)),
            _ => unreachable!(),
        };
        let member = match v {
            value::Value::Function(f) => f.0.bound[0].clone(),
            _ => crate::container::lifecycle_members(v).unwrap()[0].clone(),
        };
        self.remember(format!("m:{n}"), &member);
        self.headers.insert(n.into(), (id, weak));
    }
    fn observe(&self) -> Json {
        let raw = self.life.contract_observation();
        let cells=self.slots.iter().map(|(n,w)|{
            let info=w.upgrade().map(|c|json!({"value":self.label(c.read().unwrap()),"revision":c.revision().unwrap()})).unwrap_or(Json::Null);
            (n.clone(),info)
        }).collect::<BTreeMap<_,_>>();
        let roots = self
            .roots
            .iter()
            .map(|(n, c)| (n.clone(), self.slot_label(c)))
            .collect::<BTreeMap<_, _>>();
        let objects = self
            .objects
            .iter()
            .map(|(n, o)| (n.clone(), raw["objects"][o.slot.get().to_string()].clone()))
            .collect::<BTreeMap<_, _>>();
        let headers = self
            .headers
            .iter()
            .map(|(n, (id, _))| (n.clone(), raw["heads"][id.to_string()].clone()))
            .collect::<BTreeMap<_, _>>();
        let transactions = self
            .transactions
            .iter()
            .map(|(n, p)| {
                (
                    n.clone(),
                    match p {
                        Prepared::Copy(_) => "copy",
                        Prepared::Head { .. } => "head",
                        Prepared::Exec(_) => "exec",
                    },
                )
            })
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
        let semantic = json!({"cells":cells,"roots":roots,"objects":objects,"headers":headers,"transactions":transactions,"leases":self.leases.keys().collect::<Vec<_>>(),"queue":queue,"function_heads_charged":raw["function_heads_charged"]});
        if self.detail {
            json!({"semantic":semantic,"raw":raw,"identities":{"cells":self.slot_ids,"heads":self.headers.iter().map(|(n,h)|(n.clone(),h.0)).collect::<BTreeMap<_,_>>()}})
        } else {
            semantic
        }
    }
    fn event(&mut self, a: &Json) -> Result<(), VmError> {
        if let Some(n) = a.get("borrow").and_then(Json::as_str) {
            let cell = self.slot(n)?;
            let mut inner = a.clone();
            inner.as_object_mut().unwrap().remove("borrow");
            return cell.with_read_borrow_for_test(|| self.event(&inner));
        }
        match text(a, "op") {
            "object" => {
                let o = self.store.create(self.program, &[ValueInput::Undefined])?;
                let c = self
                    .store
                    .inner
                    .as_ref()
                    .unwrap()
                    .objects_for_audit_for_test(o.slot)
                    .unwrap()[0]
                    .clone();
                self.remember(format!("v:{}", text(a, "id")), &c);
                self.objects.insert(text(a, "id").into(), o);
            }
            "put" => {
                let n = text(a, "dest");
                let v = self.value(&a["value"])?;
                if let Ok(c) = self.slot(n) {
                    c.replace(v)?;
                } else {
                    if self.slots.contains_key(n) {
                        return Err(invalid());
                    }
                    let c = self.life.cell(v)?;
                    self.remember(n.into(), &c);
                    self.roots.insert(n.into(), c);
                }
            }
            "alias_cell" => {
                let c = self.slot(text(a, "source"))?;
                self.roots.insert(text(a, "dest").into(), c);
            }
            "drop" => {
                self.roots.remove(text(a, "slot"));
            }
            "clear" => self
                .slot(text(a, "slot"))?
                .replace(value::Value::Undefined)?,
            "make" => {
                let n = text(a, "id");
                let v = self.value(&a["value"])?;
                let h = match text(a, "kind") {
                    "array" => self.life.array(vec![v])?,
                    "mapping" => self.life.mapping(vec![(value::Value::Int(0), v)])?,
                    _ => return Err(invalid()),
                };
                let c = self.life.cell(h.clone())?;
                self.track_head(n, &h);
                let dest = text(a, "dest");
                self.remember(format!("r:{n}"), &c);
                self.roots.insert(dest.into(), c);
            }
            "prepare" => {
                let tx = text(a, "tx");
                if self.transactions.contains_key(tx) {
                    return Err(invalid());
                }
                let p = match text(a, "kind") {
                    "copy" => {
                        let mut p = self.life.plan();
                        for pair in a["writes"].as_array().unwrap() {
                            let source = self.slot(pair[0].as_str().unwrap())?;
                            let dest = self.slot(pair[1].as_str().unwrap())?;
                            let script = a.get("script").and_then(Json::as_bool).unwrap_or(false);
                            let v = p.copy(source, |v| {
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
                            p.write(dest, v)?;
                        }
                        Prepared::Copy(p)
                    }
                    "head" => {
                        let owner = self.objects[text(a, "creator")].slot;
                        if !self.life.valid(owner) {
                            return Err(VmError::new(
                                VmErrorKind::InvalidObject,
                                "invalid creator",
                            ));
                        }
                        let member = self.life.cell(self.value(&a["value"])?)?;
                        let f = value::FunctionValue(Rc::new(value::FunctionData {
                            origin: self.store.inner.as_ref().unwrap().function_origin.clone(),
                            owner: Some(owner),
                            target: value::FunctionTarget::Global {
                                program: value::ProgramId(0),
                                function: 0,
                            },
                            bound: vec![member],
                            bound_count: 1,
                            capture_profile: crate::value::CaptureProfile::Container,
                            counter: Counter::new()?,
                        }));
                        let guard = self.life.reserve_function(&f)?;
                        let v = value::Value::Function(f);
                        let cell = self.life.plan().output(v.clone())?;
                        self.track_head(text(a, "head"), &v);
                        self.remember(format!("p:{tx}"), &cell);
                        Prepared::Head {
                            cell,
                            guard,
                            dest: text(a, "dest").into(),
                        }
                    }
                    "exec" => Prepared::Exec(self.life.exec(self.objects[text(a, "object")].slot)?),
                    _ => return Err(invalid()),
                };
                self.transactions.insert(tx.into(), p);
            }
            "commit" | "abort" => {
                let tx = text(a, "tx");
                let p = self.transactions.remove(tx).ok_or_else(invalid)?;
                let commit = text(a, "op") == "commit";
                match p {
                    Prepared::Copy(p) => {
                        if commit {
                            p.commit()?;
                        }
                    }
                    Prepared::Head { cell, guard, dest } => {
                        if commit {
                            if let Err(e) = guard.validate() {
                                drop(cell);
                                drop(guard);
                                return Err(e);
                            }
                            self.roots.insert(dest, cell);
                            guard.commit();
                        } else {
                            drop(cell);
                            drop(guard);
                        }
                    }
                    Prepared::Exec(e) => {
                        if commit {
                            self.leases.insert(format!("installed:{tx}"), e);
                        }
                    }
                }
            }
            "exec" => {
                let e = self.life.exec(self.objects[text(a, "object")].slot)?;
                self.leases.insert(text(a, "id").into(), e);
            }
            "exit" => {
                self.leases.remove(text(a, "id"));
            }
            "delete" => self.store.remove(self.objects[text(a, "object")])?,
            "view" => {
                let n = text(a, "header");
                if self
                    .headers
                    .get(n)
                    .is_some_and(|h| self.private_header(h.0))
                {
                    return Err(invalid());
                }
                if let Some(v) = self.headers.get(n).and_then(|h| h.1.upgrade()) {
                    self.views.insert(n.into(), v);
                }
            }
            "unview" => {
                self.views.remove(text(a, "header"));
            }
            "maintain" | "zero" => {
                let b = LifecycleBudget {
                    host_roots: 0,
                    objects: a["objects"].as_u64().unwrap_or(0) as usize,
                    metadata_entries: a["metadata"].as_u64().unwrap_or(0) as usize,
                };
                let p = self.store.maintain_lifecycle(b)?;
                self.progress = json!({"retired_roots":p.retired_roots,"drained_objects":p.drained_objects,"reaped_objects":p.reaped_objects,"reclaimed_heads":p.reclaimed_heads,"pruned_weak_entries":p.pruned_weak_entries,"metadata_examined":p.metadata_examined,"pending_roots":p.pending_roots,"queued_objects":p.queued_objects,"scan_complete":p.scan_complete,"stop":format!("{:?}",p.stop)});
            }
            _ => return Err(invalid()),
        }
        Ok(())
    }
}

#[test]
#[ignore = "2BN actions-only two-preparation replay"]
fn replay_pair_traces() {
    use std::io::{BufRead, Write};
    let input = std::fs::File::open(std::env::var_os("RML32_PAIRS_IN").unwrap()).unwrap();
    let mut out = std::io::BufWriter::new(
        std::fs::File::create(std::env::var_os("RML32_PAIRS_OUT").unwrap()).unwrap(),
    );
    let mut pool = BTreeMap::<String, usize>::new();
    let mut count = 0;
    let mut rows = 0;
    for line in std::io::BufReader::new(input).lines() {
        let t: Json = serde_json::from_str(&line.unwrap()).unwrap();
        assert!(t.get("expected").is_none());
        let mut d = Driver::new(&t["limits"]);
        d.detail = t.get("detail").and_then(Json::as_bool).unwrap_or(false);
        let registry = Rc::downgrade(&d.store.registry);
        let ctx = Rc::downgrade(&d.life.ctx);
        for (phase, actions) in [("setup", &t["setup"]), ("actions", &t["actions"])] {
            let mut stop = false;
            for (step, a) in actions.as_array().unwrap().iter().enumerate() {
                let before = d.observe();
                assert_eq!(before, d.observe());
                let (result, calls) = crate::copy_plan::with_workspace_count(|| {
                    if let Some(cut) = a.get("fail_after").and_then(Json::as_u64) {
                        crate::copy_plan::with_workspace_failure(cut as usize, || d.event(a))
                    } else {
                        d.event(a)
                    }
                });
                let after = d.observe();
                let mut ids = Vec::new();
                for value in [before, after] {
                    let key = value.to_string();
                    let id = if let Some(id) = pool.get(&key) {
                        *id
                    } else {
                        let id = pool.len();
                        writeln!(out, "{}", json!({"state":id,"value":value})).unwrap();
                        pool.insert(key, id);
                        id
                    };
                    ids.push(id);
                }
                writeln!(out,"{}",json!({"id":t["id"],"phase":phase,"step":step,"before":ids[0],"after":ids[1],"error":result.as_ref().err().map(|e|format!("{:?}",e.kind)),"calls":calls,"progress":std::mem::take(&mut d.progress)})).unwrap();
                rows += 1;
                if result.is_err()
                    && (phase == "setup"
                        || !a
                            .get("continue_on_error")
                            .and_then(Json::as_bool)
                            .unwrap_or(false))
                {
                    stop = true;
                    break;
                }
            }
            if stop {
                break;
            }
        }
        drop(d);
        assert!(registry.upgrade().is_none());
        assert!(ctx.upgrade().is_none());
        count += 1;
    }
    out.flush().unwrap();
    println!("PAIR_REPLAY {}", json!({"traces":count,"rows":rows}));
}

fn base() -> Driver {
    let mut d = Driver::new(&high());
    for a in [
        json!({"op":"object","id":"o0"}),
        json!({"op":"object","id":"o1"}),
        json!({"op":"put","dest":"s0","value":["object","o0"]}),
        json!({"op":"put","dest":"s1","value":["object","o1"]}),
        json!({"op":"put","dest":"d0","value":["undefined"]}),
        json!({"op":"put","dest":"d1","value":["undefined"]}),
        json!({"op":"put","dest":"u","value":["undefined"]}),
    ] {
        d.event(&a).unwrap();
    }
    d
}
fn copy(tx: &str, source: &str, dest: &str) -> Json {
    json!({"op":"prepare","kind":"copy","tx":tx,"writes":[[source,dest]]})
}
fn head(tx: &str) -> Json {
    json!({"op":"prepare","kind":"head","tx":tx,"head":format!("h{tx}"),"creator":"o0","value":["undefined"],"dest":format!("out:{tx}")})
}

#[test]
fn two_plans_retain_one_source_payload_until_the_last_plan_is_consumed() {
    for order in [["A", "B"], ["B", "A"]] {
        let mut d = base();
        d.event(&copy("A", "s0", "d0")).unwrap();
        d.event(&copy("B", "s0", "d1")).unwrap();
        d.event(&json!({"op":"drop","slot":"s0"})).unwrap();
        d.event(&json!({"op":"delete","object":"o0"})).unwrap();
        assert_eq!(d.observe()["objects"]["o0"]["ordinary"], 1);
        d.event(&json!({"op":"abort","tx":order[0]})).unwrap();
        assert!(!d.observe()["cells"]["s0"].is_null());
        assert_eq!(d.observe()["objects"]["o0"]["state"], "shell");
        d.event(&json!({"op":"abort","tx":order[1]})).unwrap();
        assert!(d.observe()["cells"]["s0"].is_null());
        assert_eq!(d.observe()["objects"]["o0"]["state"], "freed");
    }
}

#[test]
fn cell_aliases_and_watches_never_duplicate_a_payload_owner() {
    let mut d = base();
    d.event(&json!({"op":"alias_cell","source":"s0","dest":"alias"}))
        .unwrap();
    d.event(&copy("A", "s0", "d0")).unwrap();
    d.event(&copy("B", "alias", "d1")).unwrap();
    d.event(&json!({"op":"drop","slot":"s0"})).unwrap();
    for tx in ["A", "B"] {
        d.event(&json!({"op":"abort","tx":tx})).unwrap();
    }
    assert_eq!(d.observe()["objects"]["o0"]["ordinary"], 1);
    assert_eq!(d.observe()["cells"]["s0"]["revision"], 0);
    assert_eq!(d.observe()["roots"]["alias"], "s0");
    d.event(&json!({"op":"drop","slot":"alias"})).unwrap();
    assert!(d.observe()["cells"]["s0"].is_null());
}

#[test]
fn first_equal_write_invalidates_the_other_plan_and_keeps_the_first_result() {
    let mut d = base();
    d.event(&copy("A", "s0", "d0")).unwrap();
    d.event(&copy("B", "s0", "d0")).unwrap();
    d.event(&json!({"op":"commit","tx":"A"})).unwrap();
    assert_eq!(
        d.event(&json!({"op":"commit","tx":"B"})).unwrap_err().kind,
        VmErrorKind::InvalidState
    );
    assert_eq!(d.observe()["cells"]["d0"]["value"], json!(["object", "o0"]));
    assert_eq!(d.observe()["cells"]["d0"]["revision"], 1);
    assert_eq!(d.observe()["objects"]["o0"]["ordinary"], 2);
}

#[test]
fn one_copy_failure_preserves_the_other_head_or_execution_reservation() {
    for kind in ["head", "exec"] {
        let mut d = base();
        d.event(&copy("A", "s0", "d0")).unwrap();
        d.event(&if kind == "head" {
            head("B")
        } else {
            json!({"op":"prepare","kind":"exec","tx":"B","object":"o0"})
        })
        .unwrap();
        d.event(&json!({"op":"put","dest":"d0","value":["int",7]}))
            .unwrap();
        assert_eq!(
            d.event(&json!({"op":"commit","tx":"A"})).unwrap_err().kind,
            VmErrorKind::InvalidState
        );
        assert_eq!(d.observe()["transactions"], json!({"B":kind}));
        d.event(&json!({"op":"abort","tx":"B"})).unwrap();
        assert_eq!(d.observe()["objects"]["o0"]["ordinary"], 1);
        assert_eq!(d.observe()["objects"]["o0"]["exec"], 0);
    }
}

#[test]
fn a_second_head_capacity_refusal_does_not_consume_the_first_reservation() {
    let mut q = high();
    q["max_function_heads"] = json!(1);
    let mut d = Driver::new(&q);
    d.event(&json!({"op":"object","id":"o0"})).unwrap();
    d.event(&head("A")).unwrap();
    assert_eq!(d.event(&head("B")).unwrap_err().kind, VmErrorKind::Limit);
    assert_eq!(d.observe()["transactions"], json!({"A":"head"}));
    assert_eq!(d.observe()["function_heads_charged"], 1);
    d.event(&json!({"op":"abort","tx":"A"})).unwrap();
    assert_eq!(d.observe()["function_heads_charged"], 0);
}

#[test]
fn invalid_creator_rejection_returns_only_that_unpublished_heads_resources() {
    let mut d = base();
    for tx in ["A", "B"] {
        d.event(&head(tx)).unwrap();
    }
    d.event(&json!({"op":"delete","object":"o0"})).unwrap();
    assert_eq!(
        d.event(&json!({"op":"commit","tx":"A"})).unwrap_err().kind,
        VmErrorKind::InvalidObject
    );
    assert_eq!(d.observe()["function_heads_charged"], 1);
    assert_eq!(d.observe()["transactions"], json!({"B":"head"}));
    assert_eq!(d.observe()["objects"]["o0"]["ordinary"], 2);
    d.event(&json!({"op":"abort","tx":"B"})).unwrap();
    assert_eq!(d.observe()["objects"]["o0"]["ordinary"], 1);
}

#[test]
fn two_execution_preparations_queue_only_after_the_last_installed_lease_exits() {
    let mut q = high();
    q["max_ordinary_refs_per_object"] = json!(2);
    let mut d = Driver::new(&q);
    for a in [
        json!({"op":"object","id":"o0"}),
        json!({"op":"put","dest":"s0","value":["object","o0"]}),
        json!({"op":"exec","id":"base","object":"o0"}),
        json!({"op":"prepare","kind":"exec","tx":"A","object":"o0"}),
        json!({"op":"prepare","kind":"exec","tx":"B","object":"o0"}),
        json!({"op":"delete","object":"o0"}),
        json!({"op":"exit","id":"base"}),
        json!({"op":"commit","tx":"A"}),
        json!({"op":"abort","tx":"B"}),
    ] {
        d.event(&a).unwrap();
    }
    assert_eq!(d.observe()["objects"]["o0"]["state"], "pending");
    assert_eq!(d.observe()["objects"]["o0"]["exec"], 1);
    d.event(&json!({"op":"exit","id":"installed:A"})).unwrap();
    assert_eq!(d.observe()["queue"], json!(["o0"]));
    assert_eq!(d.observe()["objects"]["o0"]["ordinary"], 2);
}

#[test]
fn failed_commit_releases_its_last_watched_cell_without_publishing_writes() {
    let mut d = base();
    d.event(&copy("A", "s0", "d0")).unwrap();
    d.event(&copy("B", "s1", "d1")).unwrap();
    d.event(&json!({"op":"drop","slot":"s0"})).unwrap();
    d.event(&json!({"op":"delete","object":"o0"})).unwrap();
    let result =
        crate::copy_plan::with_workspace_failure(0, || d.event(&json!({"op":"commit","tx":"A"})));
    assert_eq!(result.unwrap_err().kind, VmErrorKind::Limit);
    assert!(d.observe()["cells"]["s0"].is_null());
    assert_eq!(d.observe()["objects"]["o0"]["state"], "freed");
    assert_eq!(d.observe()["cells"]["d0"]["value"], json!(["undefined"]));
    assert_eq!(d.observe()["transactions"], json!({"B":"copy"}));
}

#[test]
fn retired_member_stays_empty_until_the_other_stale_plan_releases_its_cell() {
    let mut d = base();
    d.event(&json!({"op":"make","id":"h0","kind":"array","dest":"d0","value":["object","o1"]}))
        .unwrap();
    d.event(&copy("A", "u", "d0")).unwrap();
    d.event(&copy("B", "m:h0", "d1")).unwrap();
    d.event(&json!({"op":"commit","tx":"A"})).unwrap();
    assert_eq!(d.observe()["cells"]["m:h0"]["value"], json!(["undefined"]));
    assert_eq!(
        d.event(&json!({"op":"commit","tx":"B"})).unwrap_err().kind,
        VmErrorKind::InvalidState
    );
    assert!(d.observe()["cells"]["m:h0"].is_null());
    assert_eq!(d.observe()["cells"]["d1"]["value"], json!(["undefined"]));
}

#[test]
fn unpublished_outputs_are_private_until_publication() {
    let mut d = base();
    d.event(&head("A")).unwrap();
    assert_eq!(
        d.event(&copy("B", "p:A", "d0")).unwrap_err().kind,
        VmErrorKind::InvalidState
    );
    assert_eq!(
        d.event(&json!({"op":"put","dest":"d0","value":["header","hA"]}))
            .unwrap_err()
            .kind,
        VmErrorKind::InvalidState
    );
    assert_eq!(
        d.event(&json!({"op":"view","header":"hA"}))
            .unwrap_err()
            .kind,
        VmErrorKind::InvalidState
    );
    d.event(&json!({"op":"commit","tx":"A"})).unwrap();
    d.event(&copy("B", "out:A", "d0")).unwrap();
    assert_eq!(d.observe()["headers"]["hA"]["owners"], 1);
    d.event(&json!({"op":"commit","tx":"B"})).unwrap();
    assert_eq!(d.observe()["headers"]["hA"]["owners"], 2);
}

#[test]
fn partial_object_maintenance_keeps_both_preparations_and_only_invalidates_cleaned_variables() {
    fn queued() -> Driver {
        let mut d = base();
        d.event(&copy("A", "v:o0", "d0")).unwrap();
        d.event(&copy("B", "v:o1", "d1")).unwrap();
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
    for workspace in [false, true] {
        let mut d = queued();
        if workspace {
            crate::copy_plan::with_workspace_failure(calls, || {
                d.event(&json!({"op":"maintain","objects":2}))
            })
            .unwrap();
        } else {
            d.event(&json!({"op":"maintain","objects":2,"borrow":"v:o0"}))
                .unwrap();
        }
        assert_eq!(d.progress["drained_objects"], 1);
        assert_eq!(
            d.progress["stop"],
            if workspace {
                "WorkspaceLimit"
            } else {
                "BorrowBlocked"
            }
        );
        assert_eq!(d.observe()["transactions"], json!({"A":"copy","B":"copy"}));
        assert_eq!(d.observe()["queue"], json!(["o0"]));
        assert_eq!(
            d.event(&json!({"op":"commit","tx":"B"})).unwrap_err().kind,
            VmErrorKind::InvalidState
        );
        d.event(&json!({"op":"commit","tx":"A"})).unwrap();
        d.event(&json!({"op":"maintain","objects":1})).unwrap();
        assert_eq!(d.progress["drained_objects"], 1);
    }
}
