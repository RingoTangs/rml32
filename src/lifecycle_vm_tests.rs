use super::*;
use crate::{ProgramImage, ProgramMode, ValueLimits, parse_program, prepare_program};
type Node = (i16, i8, u8);
#[derive(Clone)]
struct Fun {
    index: i16,
    args: usize,
    locals: usize,
    ret: u8,
    nodes: Vec<Node>,
    meta: Vec<u8>,
}
fn fun(index: i16, args: usize, locals: usize, ret: u8, nodes: &[Node]) -> Fun {
    let mut body = vec![(0, 0, 0x0f), (0, 0, 0x1b)];
    body.extend(nodes);
    body.push((0, 0, 0x49));
    Fun {
        index,
        args,
        locals,
        ret,
        nodes: body,
        meta: vec![],
    }
}
fn word(out: &mut Vec<u8>, n: i32) {
    out.extend(n.to_le_bytes());
}
fn short(out: &mut Vec<u8>, n: i16) {
    out.extend(n.to_le_bytes());
}
fn string(out: &mut Vec<u8>, s: &[u8]) {
    word(out, s.len() as i32);
    out.extend(s);
}
fn code(out: &mut Vec<u8>, nodes: &[Node], meta: &[u8]) {
    word(out, 0);
    word(out, nodes.len() as i32);
    for (a, b, op) in nodes {
        short(out, *a);
        out.extend([*b as u8, *op]);
    }
    word(out, meta.len() as i32);
    out.extend(meta);
    word(out, 0);
}
fn image(functions: &[Fun], vars: usize, strings: &[&[u8]]) -> ProgramImage {
    image_mode(functions, vars, strings, ProgramMode::Object)
}
fn global_image(functions: &[Fun], vars: usize, strings: &[&[u8]]) -> ProgramImage {
    image_mode(functions, vars, strings, ProgramMode::Global)
}
fn image_mode(
    functions: &[Fun],
    vars: usize,
    strings: &[&[u8]],
    mode: ProgramMode,
) -> ProgramImage {
    let mut out = Vec::new();
    for n in [
        0x10001,
        0,
        if mode == ProgramMode::Global { 559 } else { 0 },
        0,
        strings.len() as i32,
    ] {
        word(&mut out, n);
    }
    for s in strings {
        string(&mut out, s);
    }
    let mut names = Vec::<u8>::new();
    let mut offsets = Vec::new();
    for f in functions {
        offsets.push(names.len());
        names.extend(format!("f{}\0", f.index).as_bytes());
    }
    word(&mut out, functions.len() as i32);
    word(&mut out, names.len() as i32);
    out.extend(names);
    short(&mut out, vars as i16);
    short(&mut out, vars as i16);
    for i in 0..if mode == ProgramMode::Object { vars } else { 0 } {
        out.extend([0; 16]);
        string(&mut out, format!("v{i}\0").as_bytes());
    }
    short(&mut out, 559);
    short(
        &mut out,
        functions.iter().map(|f| f.index + 1).max().unwrap_or(0),
    );
    word(&mut out, 0);
    if mode == ProgramMode::Object {
        short(&mut out, 1);
        word(&mut out, 0);
    }
    word(&mut out, functions.len() as i32);
    for (f, offset) in functions.iter().zip(offsets) {
        let mut dlt = [0; 20];
        dlt[..4].copy_from_slice(&(offset as u32).to_le_bytes());
        dlt[4..8].copy_from_slice(&0x8000000u32.to_le_bytes());
        dlt[16..18].copy_from_slice(&f.index.to_le_bytes());
        out.extend(dlt);
        let mut raw = [0; 64];
        raw[10..12].copy_from_slice(&(f.locals as i16).to_le_bytes());
        raw[13] = f.ret;
        raw[14] = f.args as u8;
        raw[15] = f.args as u8;
        out.extend(raw);
        string(&mut out, format!("f{}\0", f.index).as_bytes());
        for (count, is_local) in [(f.args, false), (f.locals, true)] {
            short(&mut out, count as i16);
            for i in 0..count {
                let mut arg = [0; 20];
                arg[5] = 255;
                let index = if is_local { -(i as i16) - 1 } else { i as i16 };
                arg[8..10].copy_from_slice(&index.to_le_bytes());
                out.extend(arg);
                string(&mut out, b"slot\0");
            }
        }
        code(&mut out, &f.nodes, &f.meta);
    }
    code(&mut out, &[(0, 0, 0x1d)], &[]);
    parse_program(&out, mode).unwrap()
}

fn machine(functions: &[Fun], vars: Vec<Value>) -> (Vm, ObjectId) {
    let mut store = ObjectStore::with_test_lifecycle(ValueLimits::default()).unwrap();
    let p = store
        .register_program(prepare_program(image(functions, vars.len(), &[])).unwrap())
        .unwrap();
    let id = store.create(p, vars).unwrap();
    (Vm::new(store), id)
}
fn snap(vm: &Vm) -> serde_json::Value {
    vm.lifecycle_audit_for_test();
    vm.objects.lifecycle_snapshot_for_test()
}
#[test]
fn normal_return_releases_real_frame_once() {
    let (mut vm, id) = machine(&[fun(0, 0, 0, 255, &[(7, 0, 0x3b)])], vec![]);
    vm.start(id, 0, vec![]).unwrap();
    assert_eq!(snap(&vm)["objects"]["0"]["execs"], 1);
    vm.run(100).unwrap();
    assert_eq!(snap(&vm)["objects"]["0"]["execs"], 0);
}
#[test]
fn pending_return_queues_without_automatic_drain() {
    let (mut vm, id) = machine(&[fun(0, 0, 0, 255, &[(7, 0, 0x3b)])], vec![]);
    vm.start(id, 0, vec![]).unwrap();
    vm.objects_mut().remove(id).unwrap();
    assert_eq!(snap(&vm)["objects"]["0"]["state"], "pending");
    vm.run(100).unwrap();
    assert_eq!(snap(&vm)["queue"], serde_json::json!([0]));
    let probe = vm.objects.probe_for_test(id);
    vm.objects.drain_one_for_test().unwrap();
    assert!(probe.upgrade().is_none());
    assert_eq!(snap(&vm)["objects"]["0"]["state"], "freed");
}
fn function_machine(functions: &[Fun], vars: usize) -> (Vm, ObjectId) {
    let mut store = ObjectStore::with_test_lifecycle(ValueLimits::default()).unwrap();
    let p = store
        .register_program(
            crate::prepare_program_with_function_values(image(functions, vars, &[]), None).unwrap(),
        )
        .unwrap();
    let id = store.create(p, vec![Value::Undefined; vars]).unwrap();
    (Vm::new(store), id)
}
fn catch_fun(index: i16, body: &[Node]) -> Fun {
    let mut nodes = vec![((body.len() + 2) as i16, 0, 0x2c)];
    nodes.extend(body);
    nodes.extend([(0, 0, 0x1a), (1, 0, 0x1f), (0, 0, 0x0e)]);
    fun(index, 0, 0, 255, &nodes)
}
#[test]
fn inspection_clones_never_acquire_execution_references() {
    let (mut vm, id) = machine(&[fun(0, 0, 0, 255, &[(7, 0, 0x3b)])], vec![]);
    vm.start(id, 0, vec![]).unwrap();
    let before = snap(&vm);
    for _ in 0..50 {
        let _view = vm.context().unwrap();
        let _stack = vm.stack().unwrap();
    }
    assert_eq!(snap(&vm), before);
}
#[test]
fn recursive_frames_each_hold_one_execution_reference() {
    let (mut vm, id) = machine(
        &[
            fun(0, 0, 0, 255, &[(1, 0, 7)]),
            fun(1, 0, 0, 255, &[(2, 0, 7)]),
            fun(2, 0, 0, 255, &[(9, 0, 0x3b)]),
        ],
        vec![],
    );
    vm.start(id, 0, vec![]).unwrap();
    vm.run(6).unwrap();
    assert_eq!(vm.frames.len(), 3);
    assert_eq!(snap(&vm)["objects"]["0"]["execs"], 3);
    vm.objects.remove(id).unwrap();
    vm.run(4).unwrap();
    assert_eq!(snap(&vm)["objects"]["0"]["execs"], 2);
    vm.run(2).unwrap();
    assert_eq!(snap(&vm)["queue"], serde_json::json!([0]));
}
#[test]
fn sticky_failure_retains_frames_until_successful_restart_or_cancel() {
    let (mut vm, id) = machine(
        &[
            fun(0, 0, 0, 255, &[(1, 0, 7)]),
            fun(1, 0, 0, 255, &[(1, 0, 0x3b), (0, 0, 0x3b), (0, 0, 0x19)]),
            fun(2, 0, 0, 255, &[(7, 0, 0x3b)]),
        ],
        vec![],
    );
    vm.start(id, 0, vec![]).unwrap();
    assert!(vm.run(100).is_err());
    vm.objects.remove(id).unwrap();
    let before = snap(&vm);
    assert!(vm.run(0).is_err());
    assert_eq!(snap(&vm), before);
    assert!(vm.start(id, 99, vec![]).is_err());
    assert_eq!(snap(&vm), before);
    vm.start(id, 2, vec![]).unwrap();
    assert_eq!(snap(&vm)["objects"]["0"]["execs"], 1);
    vm.cancel_execution_for_test();
    assert_eq!(snap(&vm)["queue"], serde_json::json!([0]));
}
#[test]
fn catch_unwind_releases_children_but_retains_owner() {
    let (mut vm, id) = machine(
        &[
            catch_fun(0, &[(1, 0, 7)]),
            fun(1, 0, 0, 255, &[(2, 0, 7)]),
            fun(2, 0, 0, 255, &[(1, 0, 0x3b), (0, 0, 0x3b), (0, 0, 0x19)]),
        ],
        vec![],
    );
    vm.start(id, 0, vec![]).unwrap();
    vm.run(7).unwrap();
    assert_eq!(vm.frames.len(), 3);
    vm.objects.remove(id).unwrap();
    vm.run(5).unwrap();
    assert_eq!(vm.frames.len(), 1);
    assert_eq!(snap(&vm)["objects"]["0"]["execs"], 1);
    vm.run(2).unwrap();
    assert_eq!(snap(&vm)["queue"], serde_json::json!([0]));
}
#[test]
fn real_variable_function_creator_cycle_is_broken_only_at_drain() {
    let (mut vm, id) = function_machine(
        &[
            fun(
                0,
                0,
                0,
                255,
                &[(1, 0, 0x17), (0, 0, 0x4e), (0, 0, 0x33), (7, 0, 0x3b)],
            ),
            fun(1, 0, 0, 255, &[(8, 0, 0x3b)]),
        ],
        1,
    );
    vm.start(id, 0, vec![]).unwrap();
    vm.run(5).unwrap();
    assert_eq!(snap(&vm)["objects"]["0"]["refs"], 1);
    vm.objects.remove(id).unwrap();
    vm.objects.remove(id).unwrap();
    vm.run(2).unwrap();
    assert_eq!(snap(&vm)["objects"]["0"]["refs"], 2);
    let probe = vm.objects.probe_for_test(id);
    vm.objects.drain_one_for_test().unwrap();
    assert!(probe.upgrade().is_none());
    assert_eq!(snap(&vm)["objects"]["0"]["refs"], 0);
}
#[test]
fn immediate_delete_and_shell_reap_clear_actual_variables_once() {
    let (mut vm, id) = machine(&[fun(0, 0, 0, 255, &[(7, 0, 0x3b)])], vec![Value::Int(9)]);
    let root = vm.test_cell(Value::Object(id)).unwrap();
    let variable = vm.objects.get(id).unwrap().variables[0].clone();
    let probe = vm.objects.probe_for_test(id);
    vm.objects.remove(id).unwrap();
    assert_eq!(variable.read().unwrap(), Value::Undefined);
    assert_eq!(snap(&vm)["objects"]["0"]["state"], "shell");
    let mut plan = vm.test_plan();
    let value = vm.copy_cell(&mut plan, root.clone()).unwrap();
    let out = plan.output(value).unwrap();
    plan.commit().unwrap();
    assert_eq!(root.read().unwrap(), Value::Undefined);
    assert_eq!(out.read().unwrap(), Value::Undefined);
    assert!(probe.upgrade().is_some());
    vm.objects.reap_for_test();
    assert!(probe.upgrade().is_none());
    snap(&vm);
}
#[test]
fn borrowed_variable_rejects_drain_without_detaching_queue() {
    let (mut vm, id) = machine(&[fun(0, 0, 0, 255, &[(7, 0, 0x3b)])], vec![Value::Int(9)]);
    vm.start(id, 0, vec![]).unwrap();
    vm.objects.remove(id).unwrap();
    vm.run(100).unwrap();
    let variable = vm.objects.objects_for_audit_for_test(id).unwrap()[0].clone();
    let before = snap(&vm);
    variable.with_read_borrow_for_test(|| assert!(vm.objects.drain_one_for_test().is_err()));
    assert_eq!(snap(&vm), before);
    vm.objects.drain_one_for_test().unwrap();
    snap(&vm);
}
#[test]
fn explicit_extraction_exits_frames_preserves_mode_and_does_not_drain() {
    let (mut vm, id) = machine(&[fun(0, 0, 0, 255, &[(7, 0, 0x3b)])], vec![]);
    vm.start(id, 0, vec![]).unwrap();
    vm.objects.remove(id).unwrap();
    let origin = vm.objects.function_origin.clone();
    let mut store = vm.into_objects();
    assert!(Rc::ptr_eq(&origin, &store.function_origin));
    assert!(store.test_life.is_some());
    assert_eq!(
        store.lifecycle_snapshot_for_test()["queue"],
        serde_json::json!([0])
    );
    store.drain_one_for_test().unwrap();
}
#[test]
fn queued_reference_reserve_is_available_at_last_frame_exit() {
    let mut store = ObjectStore::with_test_capacity(ValueLimits::default(), 2).unwrap();
    let p = store
        .register_program(
            prepare_program(image(&[fun(0, 0, 0, 255, &[(7, 0, 0x3b)])], 0, &[])).unwrap(),
        )
        .unwrap();
    let id = store.create(p, vec![]).unwrap();
    let root = store.test_cell(Value::Object(id)).unwrap();
    let mut vm = Vm::new(store);
    assert!(vm.test_cell(Value::Object(id)).is_err());
    vm.start(id, 0, vec![]).unwrap();
    vm.objects.remove(id).unwrap();
    vm.run(100).unwrap();
    assert_eq!(snap(&vm)["objects"]["0"]["refs"], 2);
    drop(root);
    vm.objects.drain_one_for_test().unwrap();
    snap(&vm);
}
#[test]
fn managed_entry_and_loop_remain_gated() {
    let (mut vm, id) = machine(&[fun(0, 0, 1, 255, &[(0, -1, 0x12)])], vec![]);
    let program = vm.objects.get(id).unwrap().program();
    let before = snap(&vm);
    assert_eq!(
        vm.begin_initialize(program).unwrap_err().kind,
        VmErrorKind::UnsupportedFeature
    );
    assert_eq!(
        vm.begin_construct(program, None).unwrap_err().kind,
        VmErrorKind::UnsupportedFeature
    );
    assert_eq!(snap(&vm), before);
    vm.start(id, 0, vec![]).unwrap();
    assert_eq!(
        vm.run(100).unwrap_err().kind,
        VmErrorKind::UnsupportedFeature
    );
    snap(&vm);
}
#[test]
fn global_frames_have_no_object_execution_lease() {
    let mut store = ObjectStore::with_test_lifecycle(ValueLimits::default()).unwrap();
    let schema = crate::GlobalBankSchema { slots: 0 };
    let p = store
        .register_program(
            crate::prepare_program_with_globals(
                image(&[fun(0, 0, 0, 255, &[(559, -128, 7)])], 0, &[]),
                schema,
            )
            .unwrap(),
        )
        .unwrap();
    let o = store.create(p, vec![]).unwrap();
    let global = store
        .register_program(
            crate::prepare_global_program(
                global_image(&[fun(559, 0, 0, 255, &[(8, 0, 0x3b)])], 0, &[]),
                schema,
            )
            .unwrap(),
        )
        .unwrap();
    store.register_globals(vec![]).unwrap();
    store.bind_global_program(global).unwrap();
    let mut vm = Vm::new(store);
    vm.start(o, 0, vec![]).unwrap();
    vm.run(3).unwrap();
    assert_eq!(vm.frames.len(), 2);
    assert_eq!(snap(&vm)["objects"]["0"]["execs"], 1);
    vm.objects.remove(o).unwrap();
    vm.run(5).unwrap();
    assert_eq!(snap(&vm)["queue"], serde_json::json!([0]));
}
#[test]
fn indirect_two_object_exits_have_lifo_queue_and_separate_creator_ref() {
    let (mut vm, a) = function_machine(
        &[
            fun(0, 0, 0, 255, &[(1, 0, 0x17)]),
            fun(1, 0, 0, 255, &[(8, 0, 0x3b)]),
            fun(2, 1, 0, 255, &[(0, 0, 0x37), (0, 1, 5)]),
        ],
        0,
    );
    let program = vm.objects.get(a).unwrap().program();
    let b = vm.objects.create(program, vec![]).unwrap();
    vm.start(b, 0, vec![]).unwrap();
    vm.run(100).unwrap();
    let Completion::Returned(value) = vm.completion.as_ref().unwrap().value.clone() else {
        unreachable!()
    };
    vm.start(a, 2, vec![value]).unwrap();
    vm.run(4).unwrap();
    assert_eq!(vm.frames.len(), 2);
    vm.objects.remove(a).unwrap();
    vm.objects.remove(b).unwrap();
    vm.run(4).unwrap();
    assert_eq!(snap(&vm)["queue"], serde_json::json!([1]));
    assert_eq!(snap(&vm)["objects"]["1"]["refs"], 2);
    vm.run(1).unwrap();
    assert_eq!(snap(&vm)["queue"], serde_json::json!([0, 1]));
    vm.objects.drain_one_for_test().unwrap();
    vm.objects.drain_one_for_test().unwrap();
    snap(&vm);
}
#[test]
fn store_drop_closes_epoch_and_allows_late_private_slot_release() {
    let (mut vm, id) = machine(
        &[fun(0, 0, 0, 255, &[(7, 0, 0x3b)])],
        vec![Value::Undefined],
    );
    let variable = vm.objects.get(id).unwrap().variables[0].clone();
    variable.replace(Value::Object(id)).unwrap();
    let state = vm.objects.test_life.as_ref().unwrap().clone();
    vm.start(id, 0, vec![]).unwrap();
    variable.with_read_borrow_for_test(|| drop(vm));
    assert!(!state.valid(id));
    assert!(variable.replace(Value::Undefined).is_err());
    drop(variable);
    assert_eq!(state.snapshot()["objects"]["0"]["refs"], 0);
}
#[test]
fn execution_audit_detects_missing_binding_and_restores_safely() {
    let (mut vm, id) = machine(&[fun(0, 0, 0, 255, &[(7, 0, 0x3b)])], vec![]);
    vm.start(id, 0, vec![]).unwrap();
    let lease = vm.exec_leases.pop().unwrap();
    assert!(
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(
            || vm.lifecycle_audit_for_test()
        ))
        .is_err()
    );
    vm.exec_leases.push(lease);
    snap(&vm);
}
#[test]
fn actual_container_instruction_binds_all_function_and_object_members() {
    let (mut vm, id) = function_machine(
        &[
            fun(0, 0, 0, 255, &[(1, 0, 0x17), (0, 0, 0x34), (2, 0, 0x16)]),
            fun(1, 0, 0, 255, &[(8, 0, 0x3b)]),
        ],
        0,
    );
    vm.start(id, 0, vec![]).unwrap();
    vm.run(100).unwrap();
    snap(&vm);
    assert_eq!(snap(&vm)["objects"]["0"]["refs"], 1);
    vm.claim_returned();
    assert_eq!(snap(&vm)["objects"]["0"]["refs"], 0);
}
#[test]
fn host_program_registration_is_rejected_before_execution() {
    let mut store = ObjectStore::with_test_lifecycle(ValueLimits::default()).unwrap();
    let mut p = prepare_program(image(&[fun(0, 0, 0, 255, &[(7, 0, 0x3b)])], 0, &[])).unwrap();
    p.allow_host_calls = true;
    assert_eq!(
        store.register_program(p).unwrap_err().kind,
        VmErrorKind::UnsupportedFeature
    );
    assert_eq!(
        store.lifecycle_snapshot_for_test()["resident"],
        serde_json::json!([])
    );
}
#[test]
fn immediate_cleanup_and_start_allocation_failures_are_atomic() {
    let (mut vm, id) = machine(&[fun(0, 0, 0, 255, &[(7, 0, 0x3b)])], vec![Value::Int(9)]);
    let before = snap(&vm);
    crate::copy_plan::with_workspace_failure(0, || assert!(vm.objects.remove(id).is_err()));
    assert_eq!(snap(&vm), before);
    crate::cell::with_allocation_failure(0, || {
        assert!(vm.start(id, 0, vec![Value::Int(3)]).is_err())
    });
    assert_eq!(snap(&vm), before);
}
#[test]
#[ignore = "explicitly run by vm_diff.py with frozen-model timeline input"]
fn replay_lifecycle_vm_traces() {
    use std::io::BufRead;
    let path =
        std::env::var_os("RML32_VM_LIFECYCLE_TRACES").expect("explicit trace input required");
    let reader = std::io::BufReader::new(std::fs::File::open(path).unwrap());
    let mut traces = 0;
    let mut steps = 0;
    for line in reader.lines() {
        let case: serde_json::Value = serde_json::from_str(&line.unwrap()).unwrap();
        let functions = case["functions"]
            .as_array()
            .unwrap()
            .iter()
            .map(|f| {
                fun(
                    f["slot"].as_i64().unwrap() as i16,
                    f["args"].as_u64().unwrap() as usize,
                    0,
                    255,
                    &f["nodes"]
                        .as_array()
                        .unwrap()
                        .iter()
                        .map(|n| {
                            (
                                n[0].as_i64().unwrap() as i16,
                                n[1].as_i64().unwrap() as i8,
                                n[2].as_u64().unwrap() as u8,
                            )
                        })
                        .collect::<Vec<_>>(),
                )
            })
            .collect::<Vec<_>>();
        let vars = case["vars"].as_u64().unwrap() as usize;
        let count = case["objects"].as_u64().unwrap() as usize;
        let mut storage = ObjectStore::with_test_lifecycle(ValueLimits::default()).unwrap();
        let image = image(&functions, vars, &[]);
        let program = if case["global"] == true {
            crate::prepare_program_with_globals(image, crate::GlobalBankSchema { slots: 0 })
                .unwrap()
        } else if case["function_values"] == true {
            crate::prepare_program_with_function_values(image, None).unwrap()
        } else {
            prepare_program(image).unwrap()
        };
        let p = storage.register_program(program).unwrap();
        for _ in 0..count {
            storage.create(p, vec![Value::Int(9); vars]).unwrap();
        }
        if case["global"] == true {
            let p = storage
                .register_program(
                    crate::prepare_global_program(
                        global_image(&[fun(559, 0, 0, 255, &[(8, 0, 0x3b)])], 0, &[]),
                        crate::GlobalBankSchema { slots: 0 },
                    )
                    .unwrap(),
                )
                .unwrap();
            storage.register_globals(vec![]).unwrap();
            storage.bind_global_program(p).unwrap();
        }
        let probes = (0..count)
            .map(|i| storage.probe_for_test(ObjectId(i as u32)))
            .collect::<Vec<_>>();
        let weak = Rc::downgrade(storage.test_life.as_ref().unwrap());
        let origin = storage.function_origin.clone();
        let mut vm = Some(Vm::new(storage));
        let mut extracted = None;
        for row in case["steps"].as_array().unwrap() {
            let a = &row["command"];
            match a["op"].as_str().unwrap() {
                "start" => {
                    let vm = vm.as_mut().unwrap();
                    let args = if a["result_argument"] == true {
                        let Completion::Returned(v) = vm.completion.as_ref().unwrap().value.clone()
                        else {
                            panic!("result")
                        };
                        vec![v]
                    } else {
                        vec![]
                    };
                    vm.start(
                        ObjectId(a["object"].as_u64().unwrap() as u32),
                        a["function"].as_i64().unwrap() as i16,
                        args,
                    )
                    .unwrap();
                }
                "run" => {
                    let result = vm
                        .as_mut()
                        .unwrap()
                        .run(a["budget"].as_u64().unwrap() as usize);
                    assert_eq!(
                        result.is_err(),
                        a["error"] == true,
                        "{} {:?}",
                        case["id"],
                        result
                    );
                }
                "delete" => {
                    let objects = if let Some(vm) = &mut vm {
                        &mut vm.objects
                    } else {
                        extracted.as_mut().unwrap()
                    };
                    objects
                        .remove(ObjectId(a["object"].as_u64().unwrap() as u32))
                        .unwrap();
                }
                "drain" => {
                    let objects = if let Some(vm) = &mut vm {
                        &mut vm.objects
                    } else {
                        extracted.as_mut().unwrap()
                    };
                    objects.drain_one_for_test().unwrap();
                }
                "reap" => {
                    let objects = if let Some(vm) = &mut vm {
                        &mut vm.objects
                    } else {
                        extracted.as_mut().unwrap()
                    };
                    objects.reap_for_test();
                }
                "cancel" => vm.as_mut().unwrap().cancel_execution_for_test(),
                "extract" => {
                    extracted = Some(vm.take().unwrap().into_objects());
                    assert!(Rc::ptr_eq(
                        &origin,
                        &extracted.as_ref().unwrap().function_origin
                    ));
                }
                "rebuild" => vm = Some(Vm::new(extracted.take().unwrap())),
                "inspect" => {
                    let vm = vm.as_ref().unwrap();
                    for _ in 0..50 {
                        if !vm.frames.is_empty() {
                            let _ = vm.context().unwrap();
                        }
                        let _ = vm.stack().unwrap();
                    }
                }
                _ => panic!("unknown command"),
            }
            let objects = if let Some(vm) = &vm {
                vm.lifecycle_audit_for_test();
                &vm.objects
            } else {
                let s = extracted.as_ref().unwrap();
                s.test_life.as_ref().unwrap().audit(&[], &[]);
                s
            };
            let actual = objects.lifecycle_snapshot_for_test();
            assert_eq!(
                actual["objects"], row["expected"]["objects"],
                "{} step {}",
                case["id"], steps
            );
            assert_eq!(
                actual["queue"], row["expected"]["queue"],
                "{} queue",
                case["id"]
            );
            assert_eq!(
                actual["resident"], row["expected"]["resident"],
                "{} resident",
                case["id"]
            );
            let frames = vm
                .as_ref()
                .map(|vm| {
                    vm.frames
                        .iter()
                        .map(|f| {
                            serde_json::json!([
                                f.active_object.map(|id| id.get()),
                                f.body.function()
                            ])
                        })
                        .collect::<Vec<_>>()
                })
                .unwrap_or_default();
            assert_eq!(
                serde_json::json!(frames),
                row["frames"],
                "{} frame timeline",
                case["id"]
            );
            steps += 1;
        }
        drop(vm);
        drop(extracted);
        assert!(weak.upgrade().is_none());
        assert!(probes.iter().all(|p| p.upgrade().is_none()));
        traces += 1;
    }
    assert!(traces > 0);
    println!(
        "VM_LIFECYCLE_REPLAY {}",
        serde_json::json!({"traces":traces,"steps":steps,"physical_teardowns":traces})
    );
}
#[test]
fn failed_indirect_commit_releases_prepared_callee_execution_lease() {
    let (mut vm, a) = function_machine(
        &[
            fun(0, 0, 0, 255, &[(1, 0, 0x17)]),
            fun(1, 0, 0, 255, &[(8, 0, 0x3b)]),
            fun(2, 1, 0, 255, &[(0, 0, 0x37), (0, 1, 5)]),
        ],
        0,
    );
    let p = vm.objects.get(a).unwrap().program();
    let b = vm.objects.create(p, vec![]).unwrap();
    vm.start(b, 0, vec![]).unwrap();
    vm.run(100).unwrap();
    let Completion::Returned(value) = vm.completion.as_ref().unwrap().value.clone() else {
        unreachable!()
    };
    vm.start(a, 2, vec![value]).unwrap();
    vm.run(3).unwrap();
    let source = vm.value_cell(vm.stack.len() - 1).unwrap();
    source.with_read_borrow_for_test(|| assert!(vm.run(1).is_err()));
    let state = snap(&vm);
    assert_eq!(vm.frames.len(), 1);
    assert_eq!(state["objects"]["1"]["execs"], 0);
    assert_eq!(state["queue"], serde_json::json!([]));
}
#[test]
fn direct_method_installs_the_target_object_execution_lease() {
    let mut store = ObjectStore::with_test_lifecycle(ValueLimits::default()).unwrap();
    let mut p = prepare_program(image(
        &[
            fun(0, 1, 0, 255, &[(0, 0, 0x40), (0, 0, 0x37), (0, 0, 8)]),
            fun(1, 0, 0, 255, &[(8, 0, 0x3b)]),
        ],
        0,
        &[b"go\0"],
    ))
    .unwrap();
    p.methods = Some(BTreeMap::from([(b"go".to_vec(), 1)]));
    let p = store.register_program(p).unwrap();
    let a = store.create(p, vec![]).unwrap();
    let b = store.create(p, vec![]).unwrap();
    let mut vm = Vm::new(store);
    vm.start(a, 0, vec![Value::Object(b)]).unwrap();
    vm.run(5).unwrap();
    assert_eq!(vm.frames.len(), 2);
    assert_eq!(snap(&vm)["objects"]["1"]["execs"], 1);
    vm.objects.remove(b).unwrap();
    vm.run(5).unwrap();
    assert_eq!(snap(&vm)["queue"], serde_json::json!([1]));
    vm.objects.drain_one_for_test().unwrap();
    snap(&vm);
}
#[test]
fn pending_object_copy_and_raw_return_keep_shell_until_completion_retires() {
    let (mut vm, id) = machine(&[fun(0, 1, 0, 255, &[(0, 0, 0x37)])], vec![]);
    vm.start(id, 0, vec![Value::Object(id)]).unwrap();
    vm.objects.remove(id).unwrap();
    vm.run(100).unwrap();
    assert_eq!(snap(&vm)["objects"]["0"]["refs"], 2);
    vm.objects.drain_one_for_test().unwrap();
    let probe = vm.objects.probe_for_test(id);
    assert_eq!(snap(&vm)["objects"]["0"]["state"], "shell");
    vm.claim_returned();
    assert_eq!(snap(&vm)["objects"]["0"]["state"], "freed");
    assert!(probe.upgrade().is_some());
    vm.objects.reap_for_test();
    assert!(probe.upgrade().is_none());
}
#[test]
fn freed_id_import_does_not_resurrect_an_object_record_or_counter() {
    let (mut vm, a) = machine(&[fun(0, 1, 0, 255, &[(0, 0, 0x37)])], vec![]);
    let p = vm.objects.get(a).unwrap().program();
    let b = vm.objects.create(p, vec![]).unwrap();
    vm.objects.remove(a).unwrap();
    vm.objects.reap_for_test();
    vm.start(b, 0, vec![Value::Object(a)]).unwrap();
    vm.run(100).unwrap();
    assert_eq!(snap(&vm)["objects"]["0"]["refs"], 0);
    assert_eq!(snap(&vm)["objects"]["0"]["state"], "freed");
    assert_eq!(
        vm.completion.as_ref().unwrap().value,
        Completion::Returned(Value::Undefined)
    );
}
