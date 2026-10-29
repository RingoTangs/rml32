//! Normal-library controls for serial operations and real nested frames.
use rml32::*;
use serde_json::{Value as Json, json};
include!("boundary_support.rs.inc");
const ZERO: LifecycleBudget = LifecycleBudget {
    host_roots: 0,
    objects: 0,
    metadata_entries: 0,
};
const ALL: LifecycleBudget = LifecycleBudget {
    host_roots: 64,
    objects: 64,
    metadata_entries: 4096,
};
fn high() -> Json {
    json!({"max_objects":3,"max_function_heads":4,"max_ordinary_refs_per_object":32,"max_exec_refs_per_object":4,"max_owners_per_header":32})
}
fn limits(q: &Json) -> LifecycleLimits {
    LifecycleLimits {
        max_objects: q["max_objects"].as_u64().unwrap() as usize,
        max_function_heads: q["max_function_heads"].as_u64().unwrap() as usize,
        max_ordinary_refs_per_object: q["max_ordinary_refs_per_object"].as_u64().unwrap() as usize,
        max_exec_refs_per_object: q["max_exec_refs_per_object"].as_u64().unwrap() as usize,
        max_owners_per_header: q["max_owners_per_header"].as_u64().unwrap() as usize,
    }
}
fn finish(vm: &mut Vm) -> Result<HostValue, VmError> {
    assert_eq!(vm.run(128)?, RunOutcome::Finished(CompletionMeta::Returned));
    vm.take_returned()
}
fn info(v: &HostValue) -> FunctionSnapshot {
    let ValueSnapshot::Function(f) = v.snapshot().unwrap() else {
        panic!("function")
    };
    f
}
fn member(v: &HostValue) -> ValueSnapshot {
    match v.snapshot().unwrap() {
        ValueSnapshot::Array { elements, .. } => elements[0].clone(),
        ValueSnapshot::Mapping { entries, .. } => entries[0].1.clone(),
        _ => panic!("container"),
    }
}
fn case(t: &Json) -> Json {
    let mut phase = "setup";
    let result = (|| -> Result<Json, VmError> {
        let mut s =
            ObjectStore::with_bounded_lifecycle(ValueLimits::default(), limits(&t["limits"]))?;
        let p = s.register_program(
            prepare_program_with_container_captures(
                image(
                    &[
                        fun(0, 0, 0, 2, &[(9, 0, 0x3b)]),
                        fun(1, 0, 0, 9, &[(0, 0, 0x17)]),
                        fun(
                            2,
                            2,
                            0,
                            255,
                            &[
                                (1, 0, 0x37),
                                (0, 0, 0x37),
                                (0, 0, 0x13),
                                (0, 0, 0x3b),
                                (0, 0, 0x44),
                                (0, 0, 3),
                            ],
                        ),
                        fun(3, 0, 0, 2, &[(0, 0, 0x07)]),
                    ],
                    0,
                    &[],
                ),
                GlobalBankSchema { slots: 0 },
            )
            .unwrap(),
        )?;
        let victim = s.create(p, &[])?;
        let reader = if t["kind"] == "members" {
            Some(s.create(p, &[])?)
        } else {
            None
        };
        let mut vm = Vm::new(s);
        match t["kind"].as_str().unwrap() {
            "members" => {
                let raw = if t["payload"] == "object" {
                    vm.objects().value(ValueInput::Object(victim))?
                } else {
                    vm.start(victim, 1, &[])?;
                    finish(&mut vm)?
                };
                let mut alias = if t["alias"] == true {
                    Some(raw.retain()?)
                } else {
                    None
                };
                let container = if t["container"] == "array" {
                    vm.objects().array(&[ValueInput::Handle(&raw)])?
                } else {
                    vm.objects()
                        .mapping(&[(ValueInput::Int(0), ValueInput::Handle(&raw))])?
                };
                drop(raw);
                for (name, n) in [("first", 7), ("second", 8)] {
                    phase = name;
                    vm.start(
                        reader.unwrap(),
                        2,
                        &[ValueInput::Handle(&container), ValueInput::Int(n)],
                    )?;
                    drop(finish(&mut vm)?);
                    assert_eq!(member(&container), ValueSnapshot::Int(n));
                }
                phase = "release";
                vm.objects_mut().remove(victim)?;
                let first = vm.maintain_lifecycle(ALL)?.reaped_objects;
                assert_eq!(first, usize::from(alias.is_none()));
                drop(alias.take());
                let second = vm.maintain_lifecycle(ALL)?.reaped_objects;
                assert_eq!(first + second, 1);
                Ok(
                    json!({"value":8,"reaped_before_alias_drop":first,"reaped_after_alias_drop":second,"frames":vm.frame_depth()}),
                )
            }
            "heads" => {
                phase = "first";
                vm.start(victim, 1, &[])?;
                let first = finish(&mut vm)?;
                let id = info(&first).id;
                phase = "second";
                vm.start(victim, 1, &[])?;
                let second = finish(&mut vm)?;
                assert_ne!(id, info(&second).id);
                assert_eq!(info(&first).owners, 1);
                assert_eq!(info(&second).owners, 1);
                phase = "release";
                vm.objects_mut().remove(victim)?;
                drop(first);
                let a = vm.maintain_lifecycle(ALL)?;
                assert_eq!(a.reaped_objects, 0);
                assert_eq!(a.reclaimed_heads, 1);
                let retained = second.retain()?;
                assert_eq!(info(&retained).id, info(&second).id);
                drop(retained);
                drop(second);
                let b = vm.maintain_lifecycle(ALL)?;
                assert_eq!(b.reaped_objects, 1);
                assert_eq!(b.reclaimed_heads, 1);
                Ok(
                    json!({"independent_heads":2,"reclaimed":a.reclaimed_heads+b.reclaimed_heads,"reaped":b.reaped_objects,"frames":vm.frame_depth()}),
                )
            }
            "frames" => {
                let root = vm.objects().value(ValueInput::Object(victim))?;
                phase = "start";
                vm.start(victim, 3, &[])?;
                phase = "nested";
                assert_eq!(vm.run(3)?, RunOutcome::BudgetExhausted);
                assert_eq!(vm.frame_depth(), 2);
                vm.objects_mut().remove(victim)?;
                assert_eq!(vm.run(0)?, RunOutcome::BudgetExhausted);
                assert_eq!(vm.frame_depth(), 2);
                assert_eq!(vm.maintain_lifecycle(ZERO)?.queued_objects, 0);
                assert_eq!(finish(&mut vm)?.snapshot()?, ValueSnapshot::Int(9));
                assert_eq!(vm.maintain_lifecycle(ZERO)?.queued_objects, 1);
                let drained = vm
                    .maintain_lifecycle(LifecycleBudget { objects: 1, ..ZERO })?
                    .drained_objects;
                assert_eq!(drained, 1);
                assert_eq!(vm.maintain_lifecycle(ALL)?.reaped_objects, 0);
                drop(root);
                assert_eq!(vm.maintain_lifecycle(ALL)?.reaped_objects, 1);
                Ok(json!({"peak_frames":2,"drained":drained,"reaped":1,"frames":vm.frame_depth()}))
            }
            _ => panic!("public family"),
        }
    })();
    match result {
        Ok(v) => json!({"id":t["id"],"result":v}),
        Err(e) => json!({"id":t["id"],"phase":phase,"error":format!("{:?}",e.kind)}),
    }
}
#[test]
fn serial_member_writes_release_old_payloads_without_rolling_back_the_first_write() {
    for container in ["array", "mapping"] {
        for payload in ["object", "function"] {
            for alias in [false, true] {
                let t = json!({"id":"control","kind":"members","limits":high(),"container":container,"payload":payload,"alias":alias});
                let r = case(&t);
                assert!(r.get("result").is_some(), "{t} -> {r}");
            }
        }
    }
}
#[test]
fn independent_function_heads_each_hold_exactly_one_creator_responsibility() {
    let t = json!({"id":"control","kind":"heads","limits":high()});
    let r = case(&t);
    assert!(r.get("result").is_some(), "{r}");
}
#[test]
fn real_nested_frames_keep_pending_until_both_execution_leases_exit() {
    let t = json!({"id":"control","kind":"frames","limits":high()});
    let r = case(&t);
    assert!(r.get("result").is_some(), "{r}");
}
#[test]
#[ignore = "2BN normal-library serial and nested-frame replay"]
fn replay_public_pairs() {
    use std::io::{BufRead, Write};
    let input = std::fs::File::open(std::env::var_os("RML32_PAIRS_IN").unwrap()).unwrap();
    let mut out = std::io::BufWriter::new(
        std::fs::File::create(std::env::var_os("RML32_PAIRS_OUT").unwrap()).unwrap(),
    );
    let mut count = 0;
    for line in std::io::BufReader::new(input).lines() {
        let t: Json = serde_json::from_str(&line.unwrap()).unwrap();
        assert!(t.get("expected").is_none());
        writeln!(out, "{}", case(&t)).unwrap();
        count += 1;
    }
    out.flush().unwrap();
    println!("PUBLIC_PAIR_REPLAY {count}");
}
