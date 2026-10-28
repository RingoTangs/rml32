//! Normal-library VM writes into already occupied container member slots.
use rml32::*;
use serde_json::{Value as Json, json};
include!("boundary_support.rs.inc");
fn limits(q: &Json) -> LifecycleLimits {
    LifecycleLimits {
        max_objects: q["max_objects"].as_u64().unwrap() as usize,
        max_function_heads: q["max_function_heads"].as_u64().unwrap() as usize,
        max_ordinary_refs_per_object: q["max_ordinary_refs_per_object"].as_u64().unwrap() as usize,
        max_exec_refs_per_object: q["max_exec_refs_per_object"].as_u64().unwrap() as usize,
        max_owners_per_header: q["max_owners_per_header"].as_u64().unwrap() as usize,
    }
}
fn case(t: &Json) -> Json {
    let mut phase = "setup";
    let result = (|| -> Result<Json, VmError> {
        let mut s =
            ObjectStore::with_bounded_lifecycle(ValueLimits::default(), limits(&t["limits"]))?;
        let assignment = fun(
            0,
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
        );
        let maker = fun(1, 1, 0, 9, &[(0, 0, 0x37), (0, 1, 0x17)]);
        let p = s.register_program(
            prepare_program_with_container_captures(
                image(&[assignment, maker], 0, &[]),
                GlobalBankSchema { slots: 0 },
            )
            .unwrap(),
        )?;
        let active = s.create(p, &[])?;
        let victim = s.create(p, &[])?;
        let kind = t["old"].as_str().unwrap();
        let mut vm = Vm::new(s);
        let payload = vm.objects().value(ValueInput::Object(victim))?;
        let old = match kind {
            "object" => payload.retain()?,
            "array" => vm.objects().array(&[ValueInput::Handle(&payload)])?,
            "mapping" => vm
                .objects()
                .mapping(&[(ValueInput::Int(0), ValueInput::Handle(&payload))])?,
            "function" => {
                let capture = vm.objects().array(&[ValueInput::Handle(&payload)])?;
                vm.start(active, 1, &[ValueInput::Handle(&capture)])?;
                assert_eq!(vm.run(64)?, RunOutcome::Finished(CompletionMeta::Returned));
                vm.take_returned()?
            }

            _ => panic!("old payload kind"),
        };
        let mut alias = if t["alias"].as_bool().unwrap() {
            Some(old.retain()?)
        } else {
            None
        };
        let container = if t["container"] == "array" {
            vm.objects().array(&[ValueInput::Handle(&old)])?
        } else {
            vm.objects()
                .mapping(&[(ValueInput::Int(0), ValueInput::Handle(&old))])?
        };
        drop(old);
        drop(payload);
        let before = container.snapshot()?;
        phase = "start";
        vm.start(
            active,
            0,
            &[ValueInput::Handle(&container), ValueInput::Int(7)],
        )?;
        phase = "prefix";
        vm.run(7)?;
        let stack = vm.stack()?;
        phase = "target";
        if let Err(e) = vm.run(1) {
            assert_eq!(e.kind, VmErrorKind::Limit);
            assert_eq!(container.snapshot()?, before);
            assert_eq!(vm.stack()?, stack);
            assert_eq!(vm.run(64).unwrap_err(), e);
            return Ok(json!({"target":"rejected","atomic":true}));
        }
        phase = "finish";
        assert_eq!(vm.run(64)?, RunOutcome::Finished(CompletionMeta::Returned));
        drop(vm.take_returned()?);
        match container.snapshot()? {
            ValueSnapshot::Array { elements, .. } => {
                assert_eq!(elements, vec![ValueSnapshot::Int(7)])
            }
            ValueSnapshot::Mapping { entries, .. } => assert_eq!(
                entries,
                vec![(ValueSnapshot::Int(0), ValueSnapshot::Int(7))]
            ),
            _ => panic!("container"),
        }
        vm.objects_mut().remove(victim)?;
        let budget = LifecycleBudget {
            host_roots: 32,
            objects: 32,
            metadata_entries: 4096,
        };
        let first = vm.maintain_lifecycle(budget)?.reaped_objects;
        assert_eq!(
            first,
            usize::from(alias.is_none()),
            "old member release/alias accounting"
        );
        drop(alias.take());
        let second = vm.maintain_lifecycle(budget)?.reaped_objects;
        assert_eq!(first + second, 1, "victim not released exactly once");
        assert_eq!(vm.frame_depth(), 0);
        Ok(
            json!({"target":"accepted","reaped_before_alias_drop":first,"reaped_after_alias_drop":second,"frames":0}),
        )
    })();
    match result {
        Ok(v) => json!({"id":t["id"],"result":v}),
        Err(e) => {
            json!({"id":t["id"],"phase":phase,"error":format!("{:?}",e.kind),"message":e.message})
        }
    }
}
#[test]
#[ignore = "2BL public occupied-member replay"]
fn replay_public_replacements() {
    use std::io::{BufRead, Write};
    let input = std::fs::File::open(std::env::var_os("RML32_REPLACEMENTS_IN").unwrap()).unwrap();
    let mut out = std::io::BufWriter::new(
        std::fs::File::create(std::env::var_os("RML32_REPLACEMENTS_OUT").unwrap()).unwrap(),
    );
    let mut count = 0;
    for line in std::io::BufReader::new(input).lines() {
        let t: Json = serde_json::from_str(&line.unwrap()).unwrap();
        assert!(t.get("expected").is_none());
        writeln!(out, "{}", case(&t)).unwrap();
        count += 1;
    }
    out.flush().unwrap();
    println!("PUBLIC_REPLACEMENT_REPLAY {count}");
}
#[test]
fn occupied_public_members_release_old_objects_graphs_and_function_captures() {
    for old in ["object", "array", "mapping", "function"] {
        for container in ["array", "mapping"] {
            for alias in [false, true] {
                let t = json!({"id":"control","old":old,"container":container,"alias":alias,"limits":{"max_objects":3,"max_function_heads":2,"max_ordinary_refs_per_object":12,"max_exec_refs_per_object":2,"max_owners_per_header":12}});
                let r = case(&t);
                assert_eq!(r["result"]["target"], "accepted", "{t} -> {r}");
            }
        }
    }
}
