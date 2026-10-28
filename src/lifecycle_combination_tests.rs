//! 2BK actions-only transaction driver; actual stable slots, no shadow owner ledger.
use super::*;
use crate::cell::Cell as Slot;
use crate::{ProgramImage, ProgramMode, parse_program, prepare_program};
use serde_json::{Value as Json, json};
use std::collections::BTreeMap;
include!("../tests/boundary_support.rs.inc");
fn quotas(q: &Json) -> crate::LifecycleLimits {
    crate::LifecycleLimits {
        max_objects: q["max_objects"].as_u64().unwrap() as usize,
        max_function_heads: q["max_function_heads"].as_u64().unwrap() as usize,
        max_ordinary_refs_per_object: q["max_ordinary_refs_per_object"].as_u64().unwrap() as usize,
        max_exec_refs_per_object: q["max_exec_refs_per_object"].as_u64().unwrap() as usize,
        max_owners_per_header: q["max_owners_per_header"].as_u64().unwrap() as usize,
    }
}
fn batch(trace: &Json) -> Vec<Json> {
    let mut s =
        ObjectStore::with_bounded_lifecycle(ValueLimits::default(), quotas(&trace["limits"]))
            .unwrap();
    let registry = Rc::downgrade(&s.registry);
    let life = s.registry.test_life.as_ref().unwrap().clone();
    let weak = Rc::downgrade(&life.ctx);
    let mut slots = BTreeMap::<String, Rc<Slot>>::new();
    let mut object = None;
    let mut header = None;
    let mut root = None;
    let mut plan = None;
    let mut rows = Vec::new();
    let observed_registry = s.registry.clone();
    for (step, a) in trace["actions"].as_array().unwrap().iter().enumerate() {
        fn snapshot(
            life: &crate::lifecycle_adapter::runtime::StoreState,
            registry: &Registry,
            slots: &BTreeMap<String, Rc<Slot>>,
            object: Option<ObjectId>,
            header: Option<u64>,
        ) -> Json {
            let raw = life.contract_observation();
            let values = slots
                .iter()
                .map(|(n, c)| {
                    let v = match c.read().unwrap() {
                        value::Value::Object(_) => json!(["object", "o0"]),
                        value::Value::Array(_) => json!(["header", "h0"]),
                        value::Value::Int(n) => json!(["int", n]),
                        value::Value::Undefined => json!(["undefined"]),
                        _ => panic!("unlisted value"),
                    };
                    (n.clone(), v)
                })
                .collect::<BTreeMap<_, _>>();
            json!({"slots":values,"ordinary":object.and_then(|id|raw["objects"][id.slot.get().to_string()]["ordinary"].as_u64()).unwrap_or(0),"header_owners":header.and_then(|id|raw["heads"][id.to_string()]["owners"].as_u64()).unwrap_or(0),"objects":raw["objects"].as_object().unwrap().len(),"pending_roots":registry.pending.get(),"queue":raw["queue"].as_array().unwrap().len()})
        }
        let before = snapshot(&life, &observed_registry, &slots, object, header);
        assert_eq!(
            before,
            snapshot(&life, &observed_registry, &slots, object, header)
        );
        let result = (|| -> Result<(), VmError> {
            match a["op"].as_str().unwrap() {
                "setup" => {
                    let p = s.register_program(
                        prepare_program(image(&[fun(0, 0, 0, 2, &[(9, 0, 0x3b)])], 0, &[]))
                            .unwrap(),
                    )?;
                    let id = s.create(p, &[])?;
                    object = Some(id);
                    slots.insert("s0".into(), life.cell(value::Value::Object(id.slot))?);
                    let array = life.array(vec![value::Value::Undefined])?;
                    header = Some(array.counter().unwrap().id);
                    slots.insert("s1".into(), life.cell(array)?);
                    for n in ["d0", "d1"] {
                        slots.insert(n.into(), life.cell(value::Value::Undefined)?);
                    }
                    root = Some(s.value(ValueInput::Undefined)?);
                }
                "prepare" => {
                    let mut p = life.plan();
                    let mut write = |src: &str, dst: &str| -> Result<(), VmError> {
                        let v = p.copy(slots[src].clone(), |v| Ok(v.clone()))?;
                        p.write(slots[dst].clone(), v)
                    };
                    match a["layout"].as_str().unwrap() {
                        "independent" => {
                            write("s0", "d0")?;
                            write("s1", "d1")?;
                        }
                        "fanout" => {
                            write("s0", "d0")?;
                            write("s0", "d1")?;
                        }
                        "same_equal" => {
                            write("s0", "d0")?;
                            write("s0", "d0")?;
                        }
                        "same_conflict" => {
                            write("s0", "d0")?;
                            write("s1", "d0")?;
                        }
                        "copy_move_alias" => {
                            write("s1", "s0")?;
                            let out = p.transfer(slots["s0"].clone())?;
                            slots.insert("d0".into(), out);
                        }
                        "moves" => {
                            for (src, dst) in [("s0", "d0"), ("s1", "d1")] {
                                let out = p.transfer(slots[src].clone())?;
                                slots.insert(dst.into(), out);
                            }
                        }
                        _ => panic!("layout"),
                    }
                    plan = Some(p);
                }
                "interfere" => match a["target"].as_str().unwrap() {
                    "retire" => {
                        let _g = observed_registry.entries.borrow();
                        drop(root.take());
                    }
                    "zero" => {
                        let p = s.maintain_lifecycle(crate::LifecycleBudget {
                            host_roots: 0,
                            objects: 0,
                            metadata_entries: 0,
                        })?;
                        assert_eq!(
                            (p.retired_roots, p.drained_objects, p.metadata_examined),
                            (0, 0, 0)
                        );
                    }
                    target => slots[target].replace(value::Value::Int(1))?,
                },
                "commit" => plan.take().unwrap().commit()?,
                "abort" => drop(plan.take()),
                _ => panic!("action"),
            }
            Ok(())
        })();
        let after = snapshot(&life, &observed_registry, &slots, object, header);
        rows.push(json!({"id":trace["id"],"step":step,"before":before,"after":after,"error":result.as_ref().err().map(|e|format!("{:?}",e.kind))}));
        if result.is_err() {
            break;
        }
    }
    drop(plan);
    drop(root);
    drop(slots);
    drop(s);
    drop(observed_registry);
    drop(life);
    assert!(registry.upgrade().is_none());
    assert!(weak.upgrade().is_none());
    rows
}
#[test]
#[ignore = "2BK actions-only driver"]
fn replay_combination_batches() {
    use std::io::{BufRead, Write};
    let input = std::fs::File::open(std::env::var_os("RML32_COMBINATIONS_IN").unwrap()).unwrap();
    let mut out = std::io::BufWriter::new(
        std::fs::File::create(std::env::var_os("RML32_COMBINATIONS_OUT").unwrap()).unwrap(),
    );
    let mut count = 0;
    for line in std::io::BufReader::new(input).lines() {
        let t: Json = serde_json::from_str(&line.unwrap()).unwrap();
        assert!(t.get("expected").is_none());
        for row in batch(&t) {
            writeln!(out, "{row}").unwrap();
        }
        count += 1;
    }
    out.flush().unwrap();
    println!("COMBINATION_BATCH_REPLAY {count}");
}
mod production {
    use crate::*;
    include!("../tests/lifecycle_combination_support.rs.inc");
    pub(super) fn counted_step(
        vm: &mut Vm,
        fail: Option<usize>,
    ) -> (Result<RunOutcome, VmError>, usize) {
        crate::copy_plan::with_workspace_count(|| match fail {
            Some(n) => crate::copy_plan::with_workspace_failure(n, || vm.run(1)),
            None => vm.run(1),
        })
    }
    fn weak_probe(s: &ObjectStore) -> Box<dyn Fn() -> serde_json::Value> {
        let registry = std::rc::Rc::downgrade(&s.registry);
        let ctx = std::rc::Rc::downgrade(&s.registry.test_life.as_ref().unwrap().ctx);
        Box::new(
            move || serde_json::json!({"registry_alive":registry.strong_count()>0,"context_alive":ctx.strong_count()>0}),
        )
    }
    #[test]
    #[ignore = "2BK private fault injection and Weak observations"]
    fn replay_instrumented_combinations() {
        replay_cases(counted_step, weak_probe);
    }
    #[test]
    fn workspace_count_scope_restores_after_nested_calls() {
        let (_, n) = crate::copy_plan::with_workspace_count(|| {
            let (_, inner) = crate::copy_plan::with_workspace_count(|| {
                let mut v = Vec::<u8>::new();
                crate::copy_plan::reserve(&mut v, 1).unwrap();
            });
            assert_eq!(inner, 1);
            let mut v = Vec::<u8>::new();
            crate::copy_plan::reserve(&mut v, 1).unwrap();
        });
        assert_eq!(n, 1);
    }
}
#[test]
fn batch_layout_controls_cover_commit_abort_and_conflict() {
    for layout in [
        "independent",
        "fanout",
        "same_equal",
        "same_conflict",
        "copy_move_alias",
        "moves",
    ] {
        let trace = json!({"id":"control","limits":{"max_objects":2,"max_function_heads":2,"max_ordinary_refs_per_object":6,"max_exec_refs_per_object":2,"max_owners_per_header":8},"actions":[{"op":"setup"},{"op":"prepare","layout":layout},{"op":"commit"}]});
        let rows = batch(&trace);
        let error = rows.last().unwrap()["error"].clone();
        assert_eq!(
            error,
            if matches!(layout, "same_conflict" | "copy_move_alias") {
                json!("InvalidState")
            } else {
                Json::Null
            }
        );
    }
}
#[test]
fn closing_with_borrow_deferred_retirement_releases_after_probe_owner_drops() {
    let s=ObjectStore::with_bounded_lifecycle(ValueLimits::default(),quotas(&json!({"max_objects":2,"max_function_heads":2,"max_ordinary_refs_per_object":6,"max_exec_refs_per_object":2,"max_owners_per_header":8}))).unwrap();
    let root = s.array(&[]).unwrap();
    let keep = s.registry.clone();
    let registry = Rc::downgrade(&keep);
    let ctx = Rc::downgrade(&keep.test_life.as_ref().unwrap().ctx);
    let borrow = keep.entries.borrow();
    drop(root);
    assert_eq!(keep.pending.get(), 1);
    drop(s);
    assert!(!keep.alive.get());
    assert!(keep.closed_store.borrow().is_some());
    assert!(ctx.upgrade().is_some());
    drop(borrow);
    drop(keep);
    assert!(registry.upgrade().is_none());
    assert!(ctx.upgrade().is_none());
}
#[test]
fn closing_callback_capture_cycle_drops_external_and_callback_roots() {
    let mut s=ObjectStore::with_bounded_lifecycle(ValueLimits::default(),quotas(&json!({"max_objects":2,"max_function_heads":2,"max_ordinary_refs_per_object":6,"max_exec_refs_per_object":2,"max_owners_per_header":8}))).unwrap();
    let captured = s.array(&[]).unwrap();
    let external = captured.retain().unwrap();
    let registry = Rc::downgrade(&s.registry);
    let ctx = Rc::downgrade(&s.registry.test_life.as_ref().unwrap().ctx);
    s.register_host_function(
        0,
        HostFunction {
            name: "cycle".into(),
            signature: crate::HostSignature {
                required: vec![],
                optional: vec![],
                rest: None,
                returns: crate::HostReturn::Void,
            },
            callback: Rc::new(move |_, _, _| {
                let _ = captured.snapshot();
                Ok(HostResult::Void)
            }),
        },
    )
    .unwrap();
    drop(s);
    assert!(registry.upgrade().is_some());
    assert_eq!(
        external.retain().unwrap_err().kind,
        VmErrorKind::InvalidState
    );
    drop(external);
    assert!(registry.upgrade().is_none());
    assert!(ctx.upgrade().is_none());
}
