//! Public projections use real VM frames; private prepared transactions are not
//! exposed as an application API by this test.
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
fn limits(q: &Json) -> LifecycleLimits {
    LifecycleLimits {
        max_objects: q["max_objects"].as_u64().unwrap() as usize,
        max_function_heads: q["max_function_heads"].as_u64().unwrap() as usize,
        max_ordinary_refs_per_object: q["max_ordinary_refs_per_object"].as_u64().unwrap() as usize,
        max_exec_refs_per_object: q["max_exec_refs_per_object"].as_u64().unwrap() as usize,
        max_owners_per_header: q["max_owners_per_header"].as_u64().unwrap() as usize,
    }
}
fn high() -> Json {
    json!({"max_objects":3,"max_function_heads":2,"max_ordinary_refs_per_object":16,"max_exec_refs_per_object":3,"max_owners_per_header":16})
}
fn finish(vm: &mut Vm) -> Result<HostValue, VmError> {
    assert_eq!(vm.run(128)?, RunOutcome::Finished(CompletionMeta::Returned));
    vm.take_returned()
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
                        fun(2, 1, 0, 255, &[(0, 0, 0x37), (0, 0, 0x3b), (0, 0, 0x43)]),
                    ],
                    1,
                    &[],
                ),
                GlobalBankSchema { slots: 0 },
            )
            .unwrap(),
        )?;
        let victim = s.create(p, &[ValueInput::Int(8)])?;
        let reader = s.create(p, &[ValueInput::Undefined])?;
        let mut vm = Vm::new(s);
        let raw = if t["payload"] == "object" {
            vm.objects().value(ValueInput::Object(victim))?
        } else {
            vm.start(victim, 1, &[])?;
            finish(&mut vm)?
        };
        let container = if t["container"] == "array" {
            vm.objects().array(&[ValueInput::Handle(&raw)])?
        } else {
            vm.objects()
                .mapping(&[(ValueInput::Int(0), ValueInput::Handle(&raw))])?
        };
        let alias = if t["alias"] == true {
            Some(container.retain()?)
        } else {
            None
        };
        let state = t["state"].as_str().unwrap();
        phase = "transition";
        match state {
            "alive" => (),
            "pending" => {
                vm.start(victim, 2, &[ValueInput::Handle(&container)])?;
                vm.objects_mut().remove(victim)?;
                vm.objects_mut().remove(victim)?;
                assert!(vm.objects().get(victim).is_ok());
                assert_eq!(vm.run(0)?, RunOutcome::BudgetExhausted);
                assert_eq!(vm.maintain_lifecycle(ZERO)?.queued_objects, 0);
            }
            "queued" => {
                vm.start(victim, 0, &[])?;
                vm.objects_mut().remove(victim)?;
                drop(finish(&mut vm)?);
                assert_eq!(vm.maintain_lifecycle(ZERO)?.queued_objects, 1);
            }
            "shell" => vm.objects_mut().remove(victim)?,
            _ => panic!("declared public state"),
        }
        let raw_before = raw.snapshot()?;
        let transport = raw.retain()?;
        match (&raw_before, transport.snapshot()?) {
            (ValueSnapshot::Function(want), ValueSnapshot::Function(got)) => {
                assert_eq!(want.id, got.id)
            }
            (want, got) => assert_eq!(*want, got),
        }
        drop(transport);
        phase = "start";
        if state != "pending" {
            vm.start(reader, 2, &[ValueInput::Handle(&container)])?;
        }
        phase = "copy";
        let result = finish(&mut vm)?;
        let valid = state == "alive" || state == "pending";
        let expected = if valid {
            raw_before.clone()
        } else {
            ValueSnapshot::Undefined
        };
        // Function snapshots include owner counts, so compare stable identity.
        match (&expected, result.snapshot()?) {
            (ValueSnapshot::Function(want), ValueSnapshot::Function(got)) => {
                assert_eq!(want.id, got.id)
            }
            (want, got) => assert_eq!(*want, got),
        }
        match (t["payload"].as_str().unwrap(), raw.snapshot()?) {
            ("object", ValueSnapshot::Object(o)) => assert_eq!(o, victim),
            ("function", ValueSnapshot::Function(_)) => (),
            _ => panic!("Host transport normalized"),
        }
        if let Some(alias) = &alias {
            let value = if t["container"] == "array" {
                alias.elements()?[0].clone()
            } else {
                alias.get(ValueInput::Int(0))?.unwrap()
            };
            if !valid {
                assert_eq!(value, ValueSnapshot::Undefined);
            }
        }
        drop(result);
        phase = "release";
        if state == "alive" {
            vm.objects_mut().remove(victim)?;
        }
        let before = vm.maintain_lifecycle(ALL)?;
        assert_eq!(
            before.reaped_objects, 0,
            "raw Host owner/creator keeps shell"
        );
        assert_eq!(
            before.drained_objects,
            usize::from(state == "pending" || state == "queued")
        );
        drop(raw);
        drop(alias);
        drop(container);
        let after = vm.maintain_lifecycle(ALL)?;
        assert_eq!(after.reaped_objects, 1, "last owner releases exactly once");
        assert_eq!(vm.maintain_lifecycle(ALL)?.reaped_objects, 0);
        assert_eq!(
            vm.objects()
                .value(ValueInput::Object(victim))
                .unwrap_err()
                .kind,
            VmErrorKind::InvalidState
        );
        let replacement = vm.objects_mut().create(p, &[ValueInput::Undefined])?;
        assert_ne!(replacement, victim);
        assert_eq!(vm.frame_depth(), 0);
        Ok(
            json!({"copy_valid":valid,"drained":before.drained_objects,"reaped":after.reaped_objects,"old_id_rejected":true,"frames":0}),
        )
    })();
    match result {
        Ok(value) => json!({"id":t["id"],"result":value}),
        Err(error) => json!({"id":t["id"],"phase":phase,"error":format!("{:?}",error.kind)}),
    }
}

#[test]
fn real_frames_copy_pending_values_and_normalize_queued_or_shell_members() {
    for payload in ["object", "function"] {
        for container in ["array", "mapping"] {
            for state in ["alive", "pending", "queued", "shell"] {
                for alias in [false, true] {
                    let t = json!({"id":"control","limits":high(),"payload":payload,"container":container,"state":state,"alias":alias});
                    let r = case(&t);
                    assert!(r.get("result").is_some(), "{t} -> {r}");
                }
            }
        }
    }
}

#[test]
fn deletion_after_argument_copy_keeps_host_callback_and_completion_raw() {
    use std::{cell::Cell, rc::Rc};
    for function in [false, true] {
        let mut s =
            ObjectStore::with_bounded_lifecycle(ValueLimits::default(), limits(&high())).unwrap();
        let p = s
            .register_program(
                prepare_program_with_container_captures(
                    image(
                        &[
                            fun(0, 0, 0, 2, &[(9, 0, 0x3b)]),
                            fun(1, 0, 0, 9, &[(0, 0, 0x17)]),
                            fun(2, 1, 0, 255, &[(0, 0, 0x37), (0, 1, 6)]),
                        ],
                        0,
                        &[],
                    ),
                    GlobalBankSchema { slots: 0 },
                )
                .unwrap(),
            )
            .unwrap();
        let victim = s.create(p, &[]).unwrap();
        let caller = s.create(p, &[]).unwrap();
        let calls = Rc::new(Cell::new(0));
        let count = calls.clone();
        s.register_host_function(
            0,
            HostFunction {
                name: "echo".into(),
                signature: HostSignature {
                    required: vec![HostType::Any],
                    optional: vec![],
                    rest: None,
                    returns: HostReturn::Mixed,
                },
                callback: Rc::new(move |_, _, _| {
                    count.set(count.get() + 1);
                    Ok(HostResult::Borrowed(0))
                }),
            },
        )
        .unwrap();
        let mut vm = Vm::new(s);
        let raw = if function {
            vm.start(victim, 1, &[]).unwrap();
            finish(&mut vm).unwrap()
        } else {
            vm.objects().value(ValueInput::Object(victim)).unwrap()
        };
        vm.start(caller, 2, &[ValueInput::Handle(&raw)]).unwrap();
        assert_eq!(vm.run(3).unwrap(), RunOutcome::BudgetExhausted);
        vm.objects_mut().remove(victim).unwrap();
        let result = finish(&mut vm).unwrap();
        assert_eq!(calls.get(), 1);
        match (raw.snapshot().unwrap(), result.snapshot().unwrap()) {
            (ValueSnapshot::Function(a), ValueSnapshot::Function(b)) => assert_eq!(a.id, b.id),
            (a, b) => assert_eq!(a, b),
        }
        assert_eq!(
            vm.take_returned().unwrap_err().kind,
            VmErrorKind::InvalidState
        );
        assert_eq!(calls.get(), 1);
        drop(result);
        drop(raw);
        assert_eq!(vm.maintain_lifecycle(ALL).unwrap().reaped_objects, 1);
    }
}

#[test]
#[ignore = "2BM normal-library state interleaving replay"]
fn replay_public_interleavings() {
    use std::io::{BufRead, Write};
    let input = std::fs::File::open(std::env::var_os("RML32_INTERLEAVINGS_IN").unwrap()).unwrap();
    let mut out = std::io::BufWriter::new(
        std::fs::File::create(std::env::var_os("RML32_INTERLEAVINGS_OUT").unwrap()).unwrap(),
    );
    let mut traces = 0;
    for line in std::io::BufReader::new(input).lines() {
        let t: Json = serde_json::from_str(&line.unwrap()).unwrap();
        assert!(t.get("expected").is_none());
        writeln!(out, "{}", case(&t)).unwrap();
        traces += 1;
    }
    out.flush().unwrap();
    println!("PUBLIC_INTERLEAVING_REPLAY {traces}");
}
