//! All programs below are synthesized in memory. No reference package is executed.
use rml32::{
    ByteString, Completion, ObjectStore, ProgramImage, ProgramMode, RunOutcome, Value, Vm,
    VmErrorKind, VmLimits, parse_program, prepare_program,
};
use std::collections::BTreeSet;

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
    let mut out = Vec::new();
    for n in [0x10001, 0, 0, 0, strings.len() as i32] {
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
    for i in 0..vars {
        out.extend([0; 16]);
        string(&mut out, format!("v{i}\0").as_bytes());
    }
    short(&mut out, 559);
    short(
        &mut out,
        functions.iter().map(|f| f.index + 1).max().unwrap_or(0),
    );
    word(&mut out, 0);
    short(&mut out, 1);
    word(&mut out, 0);
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
    parse_program(&out, ProgramMode::Object).unwrap()
}
fn machine(image: ProgramImage, values: Vec<Value>) -> (Vm, rml32::ObjectId) {
    let mut store = ObjectStore::new();
    let program = store
        .register_program(prepare_program(image).unwrap())
        .unwrap();
    let object = store.create(program, values).unwrap();
    (Vm::new(store), object)
}
fn returns(vm: &mut Vm, value: Value) {
    assert_eq!(
        vm.run(1000).unwrap(),
        RunOutcome::Finished(Completion::Returned(value))
    );
}

#[test]
fn executes_all_integer_arithmetic_and_comparison_operations() {
    for (opcode, a, b, expected) in [
        (1, 7, 3, 10),
        (0x51, 7, 3, 4),
        (0x29, 7, 3, 21),
        (0x19, -7, 3, -2),
        (0x28, -7, 3, -1),
        (0x1c, 7, 3, 0),
        (0x1c, 7, 7, 1),
        (0x1e, 7, 3, 1),
        (0x20, 3, 7, 0),
        (0x25, 7, 7, 1),
        (0x27, -7, 3, 1),
    ] {
        let p = image(
            &[fun(
                0,
                0,
                0,
                2,
                &[(a, 0, 0x3b), (b, 0, 0x3b), (0, 0, opcode)],
            )],
            0,
            &[],
        );
        let (mut vm, o) = machine(p, vec![]);
        vm.start(o, 0, vec![]).unwrap();
        returns(&mut vm, Value::Int(expected));
    }
}
#[test]
fn integer_wrap_and_negation_match_in_debug_and_release() {
    for (op, a, b, expected) in [
        (1, i32::MAX, 1, i32::MIN),
        (0x51, i32::MIN, 1, i32::MAX),
        (0x29, i32::MAX, 2, -2),
    ] {
        let p = image(
            &[fun(0, 2, 0, 2, &[(0, 0, 0x37), (1, 0, 0x37), (0, 0, op)])],
            0,
            &[],
        );
        let (mut vm, o) = machine(p, vec![]);
        vm.start(o, 0, vec![Value::Int(a), Value::Int(b)]).unwrap();
        returns(&mut vm, Value::Int(expected));
    }
    let p = image(&[fun(0, 1, 0, 2, &[(0, 0, 0x37), (0, 0, 0x2b)])], 0, &[]);
    let (mut vm, o) = machine(p, vec![]);
    vm.start(o, 0, vec![Value::Int(i32::MIN)]).unwrap();
    returns(&mut vm, Value::Int(i32::MIN));
}
#[test]
fn division_and_remainder_faults_are_terminal_errors_not_host_panics() {
    for opcode in [0x19, 0x28] {
        for (a, b) in [(1, 0), (i32::MIN, -1)] {
            let p = image(
                &[fun(
                    0,
                    2,
                    0,
                    2,
                    &[(0, 0, 0x37), (1, 0, 0x37), (0, 0, opcode)],
                )],
                0,
                &[],
            );
            let offset = p.functions[0].code.nodes_offset + 4 * 4;
            let (mut vm, o) = machine(p, vec![]);
            vm.start(o, 0, vec![Value::Int(a), Value::Int(b)]).unwrap();
            let e = vm.run(100).unwrap_err();
            assert_eq!(e.kind, VmErrorKind::Arithmetic);
            assert_eq!(e.pc, Some(4));
            assert_eq!(e.offset, Some(offset));
            assert_eq!(vm.run(100).unwrap_err(), e);
        }
    }
}
#[test]
fn metadata_number_is_little_endian_signed_i32() {
    let mut f = fun(0, 0, 0, 2, &[(0, 0, 0x3c)]);
    f.meta = i32::MIN.to_le_bytes().to_vec();
    let (mut vm, o) = machine(image(&[f], 0, &[]), vec![]);
    vm.start(o, 0, vec![]).unwrap();
    returns(&mut vm, Value::Int(i32::MIN));
}
#[test]
fn assignment_preserves_top_and_argument_local_slots_have_distinct_directions() {
    let f = fun(
        0,
        2,
        2,
        2,
        &[
            (0, 0, 0x37),
            (1, 0, 0x37),
            (0, 0, 0x51),
            (-2, 0, 0x4d),
            (0, 0, 0x33),
            (9, 0, 0x3b),
            (0, 0, 0x4d),
            (0, 0, 0x33),
            (-2, 0, 0x37),
            (0, 0, 0x37),
            (0, 0, 1),
        ],
    );
    let (mut vm, o) = machine(image(&[f], 0, &[]), vec![]);
    vm.start(o, 0, vec![Value::Int(10), Value::Int(3)]).unwrap();
    returns(&mut vm, Value::Int(16));
}
#[test]
fn unallocated_local_read_returns_undefined_but_write_errors() {
    for no in [0, -1, -100] {
        let (mut vm, o) = machine(
            image(&[fun(0, 0, 0, 255, &[(no, 0, 0x37)])], 0, &[]),
            vec![],
        );
        vm.start(o, 0, vec![]).unwrap();
        returns(&mut vm, Value::Undefined);
    }
    let (mut vm, o) = machine(
        image(&[fun(0, 0, 0, 255, &[(1, 0, 0x3b), (0, 0, 0x4d)])], 0, &[]),
        vec![],
    );
    vm.start(o, 0, vec![]).unwrap();
    assert_eq!(vm.run(100).unwrap_err().kind, VmErrorKind::Bounds);
}
#[test]
fn pop_n_handles_nonpositive_counts_without_consuming_slots() {
    let p = image(
        &[fun(
            0,
            0,
            1,
            2,
            &[
                (10, 0, 0x3b),
                (11, 0, 0x3b),
                (2, 0, 0x32),
                (-1, 0, 0x32),
                (7, 0, 0x3b),
            ],
        )],
        0,
        &[],
    );
    let (mut vm, o) = machine(p, vec![]);
    vm.start(o, 0, vec![]).unwrap();
    returns(&mut vm, Value::Int(7));
}
#[test]
fn nested_calls_preserve_argument_order_and_cleanup_frame_storage() {
    let caller = fun(
        0,
        0,
        0,
        2,
        &[
            (90, 0, 0x3b),
            (3, 0, 0x3b),
            (10, 0, 0x3b),
            (1, 2, 0x07),
            (0, 0, 1),
        ],
    );
    let callee = fun(
        1,
        2,
        1,
        2,
        &[(0, 0, 0x37), (1, 0, 0x37), (0, 0, 0x51), (-1, 0, 0x4d)],
    );
    let (mut vm, o) = machine(image(&[caller, callee], 0, &[]), vec![]);
    vm.start(o, 0, vec![]).unwrap();
    returns(&mut vm, Value::Int(97));
    assert!(vm.stack().unwrap().is_empty());
    assert_eq!(vm.frame_depth(), 0);
}
#[test]
fn void_and_mixed_returns_supply_void_when_no_value_exists() {
    for ret in [1, 255] {
        let caller = fun(0, 0, 0, 1, &[(1, 0, 0x07), (0, 0, 0x33)]);
        let callee = fun(1, 0, 2, ret, &[]);
        let (mut vm, o) = machine(image(&[caller, callee], 0, &[]), vec![]);
        vm.start(o, 0, vec![]).unwrap();
        returns(&mut vm, Value::Void);
    }
}
#[test]
fn return_shape_and_type_errors_do_not_silently_succeed() {
    for (ret, nodes) in [
        (1, vec![(1, 0, 0x3b)]),
        (2, vec![]),
        (255, vec![(1, 0, 0x3b), (2, 0, 0x3b)]),
    ] {
        let (mut vm, o) = machine(image(&[fun(0, 0, 0, ret, &nodes)], 0, &[]), vec![]);
        vm.start(o, 0, vec![]).unwrap();
        assert_eq!(vm.run(100).unwrap_err().kind, VmErrorKind::InvalidProgram);
    }
    let (mut vm, o) = machine(image(&[fun(0, 0, 0, 2, &[(0, 0, 0x42)])], 0, &[]), vec![]);
    vm.start(o, 0, vec![]).unwrap();
    assert_eq!(vm.run(100).unwrap_err().kind, VmErrorKind::UnsupportedType);
}
#[test]
fn conditional_flags_choose_direction_and_pop_independently() {
    for flag in 0..4 {
        for condition in [0, 1] {
            let nodes = if flag & 2 == 0 {
                vec![
                    (condition, 0, 0x3b),
                    (3, flag, 0x21),
                    (0, 0, 0x33),
                    (10, 0, 0x3b),
                    (2, 0, 0x1f),
                    (0, 0, 0x33),
                    (20, 0, 0x3b),
                ]
            } else {
                vec![
                    (condition, 0, 0x3b),
                    (2, flag, 0x21),
                    (10, 0, 0x3b),
                    (1, 0, 0x1f),
                    (20, 0, 0x3b),
                ]
            };
            let (mut vm, o) = machine(image(&[fun(0, 0, 0, 2, &nodes)], 0, &[]), vec![]);
            vm.start(o, 0, vec![]).unwrap();
            returns(
                &mut vm,
                Value::Int(if (condition != 0) == (flag & 1 != 0) {
                    20
                } else {
                    10
                }),
            );
        }
    }
}
#[test]
fn empty_strings_are_truthy_and_undefined_is_false() {
    for value in [Value::bytes(b""), Value::Undefined, Value::Void] {
        let f = fun(
            0,
            1,
            0,
            2,
            &[
                (0, 0, 0x37),
                (2, 3, 0x21),
                (10, 0, 0x3b),
                (1, 0, 0x1f),
                (20, 0, 0x3b),
            ],
        );
        let (mut vm, o) = machine(image(&[f], 0, &[]), vec![]);
        vm.start(o, 0, vec![value.clone()]).unwrap();
        returns(
            &mut vm,
            Value::Int(if matches!(value, Value::Bytes(_)) {
                20
            } else {
                10
            }),
        );
    }
}
#[test]
fn budget_exhaustion_preserves_loop_state_and_can_resume() {
    // Increment object variable to 3, using relative offsets from the next PC.
    let f = fun(
        0,
        0,
        0,
        2,
        &[
            (0, 0, 0x3d),
            (1, 0, 0x3b),
            (0, 0, 1),
            (0, 0, 0x4e),
            (0, 0, 0x33),
            (0, 0, 0x3d),
            (3, 0, 0x3b),
            (0, 0, 0x27),
            (-9, 3, 0x21),
            (0, 0, 0x3d),
        ],
    );
    let p = image(&[f], 1, &[]);
    let (mut vm, o) = machine(p.clone(), vec![Value::Int(0)]);
    vm.start(o, 0, vec![]).unwrap();
    assert_eq!(vm.run(0).unwrap(), RunOutcome::BudgetExhausted);
    for _ in 0..100 {
        match vm.run(1).unwrap() {
            RunOutcome::BudgetExhausted => {}
            RunOutcome::Finished(c) => {
                assert_eq!(c, Completion::Returned(Value::Int(3)));
                break;
            }
        }
    }
    assert_eq!(
        vm.run(0).unwrap(),
        RunOutcome::Finished(Completion::Returned(Value::Int(3)))
    );
    assert_eq!(
        vm.objects().get(o).unwrap().variables().unwrap(),
        &[Value::Int(3)]
    );
    let (mut full, o) = machine(p, vec![Value::Int(0)]);
    full.start(o, 0, vec![]).unwrap();
    returns(&mut full, Value::Int(3));
}
#[test]
fn object_variables_are_isolated_and_handles_are_not_reused() {
    let p = prepare_program(image(
        &[fun(0, 0, 0, 2, &[(1, 0, 0x3b), (0, 0, 0x4e)])],
        1,
        &[],
    ))
    .unwrap();
    let mut store = ObjectStore::new();
    let pid = store.register_program(p).unwrap();
    assert!(store.create(pid, vec![]).is_err());
    let a = store.create(pid, vec![Value::Int(0)]).unwrap();
    let b = store.create(pid, vec![Value::Int(9)]).unwrap();
    let mut vm = Vm::new(store);
    vm.start(a, 0, vec![]).unwrap();
    returns(&mut vm, Value::Int(1));
    assert_eq!(
        vm.objects().get(b).unwrap().variables().unwrap(),
        &[Value::Int(9)]
    );
    vm.objects_mut().remove(a).unwrap();
    assert_eq!(
        vm.objects().get(a).unwrap_err().kind,
        VmErrorKind::InvalidObject
    );
    let c = vm.objects_mut().create(pid, vec![Value::Int(0)]).unwrap();
    assert_ne!(a, c);
    assert!(vm.start(a, 0, vec![]).is_err());
}
#[test]
fn copying_destroyed_object_values_yields_undefined() {
    let mut store = ObjectStore::new();
    let pid = store
        .register_program(
            prepare_program(image(&[fun(0, 1, 0, 255, &[(0, 0, 0x37)])], 0, &[])).unwrap(),
        )
        .unwrap();
    let dead = store.create(pid, vec![]).unwrap();
    let live = store.create(pid, vec![]).unwrap();
    store.remove(dead).unwrap();
    let mut vm = Vm::new(store);
    vm.start(live, 0, vec![Value::Object(dead)]).unwrap();
    returns(&mut vm, Value::Undefined);
    vm.start(live, 0, vec![Value::Object(live)]).unwrap();
    returns(&mut vm, Value::Object(live));
}
#[test]
fn pool_strings_use_c_termination_without_utf8_conversion() {
    for (raw, expected) in [
        (b"\xff\0abc\0".as_slice(), b"\xff".as_slice()),
        (b"ABZ".as_slice(), b"AB".as_slice()),
    ] {
        let p = image(&[fun(0, 0, 0, 4, &[(0, 0, 0x40)])], 0, &[raw]);
        assert_eq!(p.strings[0].as_ref().unwrap().bytes, raw);
        let (mut vm, o) = machine(p, vec![]);
        vm.start(o, 0, vec![]).unwrap();
        returns(&mut vm, Value::bytes(expected));
    }
}
#[test]
fn finish_stops_the_action_without_fabricating_a_return_value() {
    let p = image(&[fun(0, 0, 0, 255, &[(9, 0, 0x3b), (0, 0, 0x1d)])], 0, &[]);
    let (mut vm, o) = machine(p, vec![]);
    vm.start(o, 0, vec![]).unwrap();
    assert_eq!(
        vm.run(100).unwrap(),
        RunOutcome::Finished(Completion::ActionFinished)
    );
    assert_eq!(
        vm.stack().unwrap(),
        &[rml32::StackSnapshot::Value(Value::Int(9))]
    );
}
#[test]
fn fixed_argument_check_skips_default_code_count_and_validates_types() {
    let mut f = fun(0, 1, 0, 2, &[(0, 0, 0x37)]);
    f.nodes.insert(1, (999, 0, 0x3b));
    f.nodes[0].0 = 1;
    let mut p = image(&[f], 0, &[]);
    p.functions[0].arguments[0].r#type = 2;
    let (mut vm, o) = machine(p, vec![]);
    vm.start(o, 0, vec![]).unwrap();
    assert_eq!(vm.run(100).unwrap_err().kind, VmErrorKind::ArgumentCount);
    vm.start(o, 0, vec![Value::bytes(b"bad")]).unwrap();
    assert_eq!(vm.run(100).unwrap_err().kind, VmErrorKind::UnsupportedType);
    vm.start(o, 0, vec![Value::Int(8)]).unwrap();
    returns(&mut vm, Value::Int(8));
}
#[test]
fn stack_underflow_missing_calls_and_pc_overrun_have_locations() {
    let (mut vm, o) = machine(image(&[fun(0, 1, 0, 255, &[(0, 0, 0x33)])], 0, &[]), vec![]);
    vm.start(o, 0, vec![Value::Int(3)]).unwrap();
    let e = vm.run(100).unwrap_err();
    assert_eq!(e.kind, VmErrorKind::StackUnderflow);
    assert_eq!(e.function, Some(0));
    assert_eq!(e.pc, Some(2));
    assert!(e.offset.is_some());
    let (mut vm, o) = machine(image(&[fun(0, 0, 0, 255, &[(9, 0, 0x07)])], 0, &[]), vec![]);
    vm.start(o, 0, vec![]).unwrap();
    assert_eq!(vm.run(100).unwrap_err().kind, VmErrorKind::InvalidProgram);
    let mut f = fun(0, 0, 0, 255, &[(1, 0, 0x3b)]);
    f.nodes.pop();
    let (mut vm, o) = machine(image(&[f], 0, &[]), vec![]);
    vm.start(o, 0, vec![]).unwrap();
    assert_eq!(vm.run(100).unwrap_err().kind, VmErrorKind::Bounds);
}
#[test]
fn frame_and_stack_limits_terminate_recursion_and_allocation() {
    let p = image(&[fun(0, 0, 0, 255, &[(0, 0, 0x07)])], 0, &[]);
    let (vm, o) = machine(p, vec![]);
    let mut vm = Vm::with_limits(
        vm.into_objects(),
        VmLimits {
            max_reference_cells: 65536,
            max_stack: 5,
            max_frames: 3,
        },
    );
    vm.start(o, 0, vec![]).unwrap();
    assert_eq!(vm.run(100).unwrap_err().kind, VmErrorKind::Limit);
    let p = image(&[fun(0, 0, 2, 255, &[])], 0, &[]);
    let (vm, o) = machine(p, vec![]);
    let mut vm = Vm::with_limits(
        vm.into_objects(),
        VmLimits {
            max_reference_cells: 65536,
            max_stack: 1,
            max_frames: 3,
        },
    );
    vm.start(o, 0, vec![]).unwrap();
    assert_eq!(vm.run(100).unwrap_err().kind, VmErrorKind::Limit);
}
#[test]
fn unsupported_program_shapes_fail_explicitly() {
    let base = image(&[fun(0, 0, 0, 255, &[])], 0, &[]);
    let mut inherited = base.clone();
    inherited.constitution_count = Some(2);
    assert_eq!(
        prepare_program(inherited).unwrap_err().kind,
        VmErrorKind::UnsupportedFeature
    );
    let mut global = base.clone();
    global.mode = ProgramMode::Global;
    assert_eq!(
        prepare_program(global).unwrap_err().kind,
        VmErrorKind::UnsupportedFeature
    );
    for (opcode, b) in [(6, 0), (7, -128), (0x46, 0), (0x0e, 0)] {
        let p = image(&[fun(0, 0, 0, 255, &[(0, b, opcode)])], 0, &[]);
        let e = prepare_program(p).unwrap_err();
        assert_eq!(e.kind, VmErrorKind::UnsupportedFeature);
        assert_eq!(e.pc, Some(2));
        assert!(e.offset.is_some());
    }
    let mut optional = base.clone();
    optional.functions[0].max_args = 1;
    assert_eq!(
        prepare_program(optional).unwrap_err().kind,
        VmErrorKind::UnsupportedFeature
    );
    let mut real = base;
    real.functions[0].return_type = 5;
    assert_eq!(
        prepare_program(real).unwrap_err().kind,
        VmErrorKind::UnsupportedType
    );
}
#[test]
fn preparation_rejects_invalid_metadata_branches_duplicate_indices_and_bounds() {
    let base = image(&[fun(0, 0, 0, 255, &[])], 0, &[]);
    let mut metadata = base.clone();
    metadata.functions[0].code.nodes[2] = (0, 0, 0x3c);
    assert!(prepare_program(metadata).is_err());
    let mut branch = base.clone();
    branch.functions[0].code.nodes[2] = (999, 0, 0x1f);
    assert!(prepare_program(branch).is_err());
    let mut duplicate = base.clone();
    duplicate.functions.push(duplicate.functions[0].clone());
    assert!(prepare_program(duplicate).is_err());
    let mut vars = base;
    vars.functions[0].code.nodes[2] = (0, 0, 0x3d);
    assert!(prepare_program(vars).is_err());
}
#[test]
fn vm_state_requires_start_and_disallows_replacing_active_execution() {
    let (mut vm, o) = machine(image(&[fun(0, 0, 0, 255, &[])], 0, &[]), vec![]);
    assert_eq!(vm.run(0).unwrap_err().kind, VmErrorKind::InvalidState);
    assert!(vm.start(o, 9, vec![]).is_err());
    vm.start(o, 0, vec![]).unwrap();
    assert_eq!(
        vm.start(o, 0, vec![]).unwrap_err().kind,
        VmErrorKind::InvalidState
    );
    assert_eq!(vm.run(0).unwrap(), RunOutcome::BudgetExhausted);
    returns(&mut vm, Value::Void);
}
#[test]
fn supported_types_have_explicit_legacy_tags() {
    let (_, object) = machine(image(&[fun(0, 0, 0, 255, &[])], 0, &[]), vec![]);
    assert_eq!(
        [
            Value::Undefined.tag(),
            Value::Void.tag(),
            Value::Int(0).tag(),
            Value::bytes(b"").tag(),
            Value::Object(object).tag()
        ],
        [0, 1, 2, 4, 10]
    );
    let supported: BTreeSet<_> = rml32::prepared::SUPPORTED_OPCODES.iter().collect();
    assert_eq!(supported.len(), 52);
    assert_eq!(ByteString::new(b"\xff").bytes, b"\xff");
}

#[test]
fn noninteger_arithmetic_and_deleted_execution_object_fail_explicitly() {
    let p = image(
        &[fun(0, 2, 0, 255, &[(0, 0, 0x37), (1, 0, 0x37), (0, 0, 1)])],
        0,
        &[],
    );
    let (mut vm, o) = machine(p, vec![]);
    vm.start(o, 0, vec![Value::bytes(b"a"), Value::bytes(b"b")])
        .unwrap();
    assert_eq!(vm.run(100).unwrap_err().kind, VmErrorKind::UnsupportedType);
    vm.start(o, 0, vec![Value::Int(1), Value::Int(2)]).unwrap();
    assert_eq!(vm.run(1).unwrap(), RunOutcome::BudgetExhausted);
    vm.objects_mut().remove(o).unwrap();
    assert_eq!(vm.run(1).unwrap_err().kind, VmErrorKind::InvalidObject);
}

#[test]
fn unknown_flags_const_atomic_and_variadic_prototypes_are_rejected() {
    let base = image(&[fun(0, 1, 0, 255, &[(0, 0, 0x37)])], 0, &[]);
    let mut reference = base.clone();
    reference.functions[0].arguments[0].flag = 2;
    let mut constant = base.clone();
    constant.functions[0].arguments[0].attrib = 2;
    let mut atomic = base.clone();
    atomic.functions[0].attrib = 2;
    let mut variadic = base.clone();
    variadic.functions[0].attrib = 1;
    let mut const_return = base;
    let mut raw = vec![0; 64];
    raw[12] = 2;
    const_return.functions[0].raw_hex = rml32::hex(&raw);
    for p in [reference, constant, atomic, variadic, const_return] {
        assert_eq!(
            prepare_program(p).unwrap_err().kind,
            VmErrorKind::UnsupportedFeature
        );
    }
}

#[test]
fn nullable_prototypes_allow_undefined_without_numeric_conversion() {
    let mut p = image(&[fun(0, 1, 0, 2, &[(0, 0, 0x37)])], 0, &[]);
    p.functions[0].arguments[0].r#type = 2;
    p.functions[0].arguments[0].attrib = 8;
    let mut raw = vec![0; 64];
    raw[12] = 8;
    p.functions[0].raw_hex = rml32::hex(&raw);
    let (mut vm, o) = machine(p, vec![]);
    vm.start(o, 0, vec![Value::Undefined]).unwrap();
    returns(&mut vm, Value::Undefined);
}

fn real(x: f32) -> Value {
    Value::Real(rml32::RealBits::from_f32(x))
}
fn numeric_run(op: u8, left: Value, right: Value) -> Value {
    let p = image(
        &[fun(0, 2, 0, 255, &[(0, 0, 0x37), (1, 0, 0x37), (0, 0, op)])],
        0,
        &[],
    );
    let (mut vm, o) = machine(p, vec![]);
    vm.start(o, 0, vec![left, right]).unwrap();
    let RunOutcome::Finished(Completion::Returned(v)) = vm.run(100).unwrap() else {
        panic!("not returned");
    };
    v
}
#[test]
fn mixed_arithmetic_preserves_left_type_except_remainder() {
    for (op, integer, float) in [
        (1, 9, 9.5),
        (0x51, 4, 4.5),
        (0x29, 17, 17.5),
        (0x19, 2, 2.8),
    ] {
        assert_eq!(
            numeric_run(op, Value::Int(7), real(2.5)),
            Value::Int(integer)
        );
        assert_eq!(numeric_run(op, real(7.0), real(2.5)), real(float));
    }
    assert_eq!(numeric_run(1, real(2.5), Value::Int(7)), real(9.5));
    assert_eq!(numeric_run(0x28, Value::Int(7), real(2.5)), real(2.0));
    assert_eq!(numeric_run(0x28, real(-7.0), Value::Int(3)), real(-1.0));
}
#[test]
fn mixed_integer_arithmetic_does_not_round_integer_before_x87_operation() {
    assert_eq!(
        numeric_run(0x51, Value::Int(16_777_217), real(16_777_216.0)),
        Value::Int(1)
    );
    assert_eq!(
        numeric_run(0x51, real(16_777_216.0), Value::Int(16_777_217)),
        real(-1.0)
    );
    assert_eq!(
        numeric_run(0x28, Value::Int(16_777_217), real(2.0)),
        real(0.0)
    );
}
#[test]
fn mixed_product_avoids_binary64_double_rounding_at_binary32_midpoint() {
    // Exact integer products, rounded directly to binary32 with nearest-even:
    // 16777215*1090519041 -> 0x5a81ffff; 8388609*1619001343 -> 0x5a410001.
    for (r, i, bits) in [
        (16777215.0, 1090519041, 0x5a81ffff),
        (8388609.0, 1619001343, 0x5a410001),
    ] {
        assert_eq!(
            numeric_run(0x29, real(r), Value::Int(i)),
            Value::Real(rml32::RealBits::from_bits(bits))
        );
    }
}
#[test]
fn real_constant_uses_first_word_only_and_retains_nan_and_signed_zero_bits() {
    for bits in [0x80000000u32, 0x7fc01234, 0x3fc00000] {
        let mut f = fun(0, 0, 0, 3, &[(0, 0, 0x3f)]);
        f.meta.extend(bits.to_le_bytes());
        f.meta.extend(0xdeadbeefu32.to_le_bytes());
        let p = image(&[f], 0, &[]);
        let (mut vm, o) = machine(p, vec![]);
        vm.start(o, 0, vec![]).unwrap();
        returns(&mut vm, Value::Real(rml32::RealBits::from_bits(bits)));
    }
    let mut p = image(&[fun(0, 0, 0, 255, &[])], 0, &[]);
    p.functions[0].code.nodes[2] = (0, 0, 0x3f);
    p.functions[0].code.metadata_hex = "00000000".into();
    assert!(prepare_program(p).is_err());
}
#[test]
fn float_to_integer_conversion_uses_indefinite_min_instead_of_rust_saturation() {
    for (input, expected) in [
        (1.9, 1),
        (-1.9, -1),
        (f32::NAN, i32::MIN),
        (f32::INFINITY, i32::MIN),
        (2147483648.0, i32::MIN),
        (-2147483648.0, i32::MIN),
    ] {
        let p = image(&[fun(0, 1, 0, 2, &[(0, 0, 0x37), (2, 0, 0x0d)])], 0, &[]);
        let (mut vm, o) = machine(p, vec![]);
        vm.start(o, 0, vec![real(input)]).unwrap();
        returns(&mut vm, Value::Int(expected));
        let p = image(&[fun(0, 1, 0, 2, &[(0, 0, 0x37)])], 0, &[]);
        let (mut vm, o) = machine(p, vec![]);
        vm.start(o, 0, vec![real(input)]).unwrap();
        returns(&mut vm, Value::Int(expected));
    }
}
#[test]
fn typed_real_parameters_and_returns_coerce_numbers_and_preserve_nullable() {
    let mut p = image(&[fun(0, 1, 0, 3, &[(0, 0, 0x37)])], 0, &[]);
    p.functions[0].arguments[0].r#type = 3;
    let (mut vm, o) = machine(p, vec![]);
    vm.start(o, 0, vec![Value::Int(16777217)]).unwrap();
    returns(&mut vm, real(16777216.0));
    let mut p = image(&[fun(0, 1, 0, 2, &[(0, 0, 0x37)])], 0, &[]);
    p.functions[0].arguments[0].r#type = 2;
    let (mut vm, o) = machine(p, vec![]);
    vm.start(o, 0, vec![real(-2.9)]).unwrap();
    returns(&mut vm, Value::Int(-2));
}
#[test]
fn numeric_comparisons_round_integer_inputs_and_nan_is_unordered() {
    assert_eq!(
        numeric_run(0x1c, Value::Int(16777217), real(16777216.0)),
        Value::Int(1)
    );
    for op in [0x1c, 0x1e, 0x20, 0x25, 0x27, 0x2a] {
        for (a, b) in [
            (real(f32::NAN), Value::Int(1)),
            (Value::Int(1), real(f32::NAN)),
        ] {
            assert_eq!(numeric_run(op, a, b), Value::Int(i32::from(op == 0x2a)));
        }
    }
    assert_eq!(numeric_run(0x1c, real(-0.0), real(0.0)), Value::Int(1));
    assert_ne!(real(-0.0), real(0.0)); // structural equality deliberately differs
}
#[test]
fn real_zero_division_errors_but_libm_remainder_zero_produces_nan() {
    for rhs in [0.0, -0.0] {
        let p = image(
            &[fun(
                0,
                2,
                0,
                255,
                &[(0, 0, 0x37), (1, 0, 0x37), (0, 0, 0x19)],
            )],
            0,
            &[],
        );
        let (mut vm, o) = machine(p, vec![]);
        vm.start(o, 0, vec![real(1.0), real(rhs)]).unwrap();
        assert_eq!(vm.run(100).unwrap_err().kind, VmErrorKind::Arithmetic);
        let Value::Real(result) = numeric_run(0x28, real(1.0), real(rhs)) else {
            panic!();
        };
        assert!(result.to_f32().is_nan());
    }
    let Value::Real(result) = numeric_run(0x19, real(1.0), real(f32::NAN)) else {
        panic!();
    };
    assert!(result.to_f32().is_nan());
}
#[test]
fn bitwise_operations_and_logical_right_shift_follow_guarded_native_counts() {
    for (op, a, b, result) in [
        (2, 0x55, 0x0f, 5),
        (0x30, 0x50, 0x0f, 0x5f),
        (0x55, 0x55, 0x0f, 0x5a),
        (0x26, 1, 31, i32::MIN),
        (0x4b, -1, 1, i32::MAX),
    ] {
        assert_eq!(
            numeric_run(op, Value::Int(a), Value::Int(b)),
            Value::Int(result)
        );
    }
    for op in [0x26, 0x4b] {
        for count in [-1, 32, 33, i32::MAX] {
            assert_eq!(
                numeric_run(op, Value::Int(-1), Value::Int(count)),
                Value::Int(0)
            );
        }
    }
}
#[test]
fn unary_numeric_type_and_truth_operations_handle_real_values() {
    for (op, a, input, expected) in [
        (0x4a, 0, Value::Int(0), Value::Int(-1)),
        (0x2b, 0, real(0.0), real(-0.0)),
        (0x2d, 0, real(-0.0), Value::Int(1)),
        (0x2d, 0, real(f32::NAN), Value::Int(0)),
        (0x24, 3, real(1.0), Value::Int(1)),
        (0x24, 255, real(1.0), Value::Int(0)),
    ] {
        let p = image(&[fun(0, 1, 0, 255, &[(0, 0, 0x37), (a, 0, op)])], 0, &[]);
        let (mut vm, o) = machine(p, vec![]);
        vm.start(o, 0, vec![input]).unwrap();
        returns(&mut vm, expected);
    }
    for target in [2, 3] {
        let p = image(
            &[fun(0, 0, 0, 255, &[(0, 0, 0x42), (target, 0, 0x0d)])],
            0,
            &[],
        );
        let (mut vm, o) = machine(p, vec![]);
        vm.start(o, 0, vec![]).unwrap();
        returns(
            &mut vm,
            if target == 2 {
                Value::Int(0)
            } else {
                real(0.0)
            },
        );
    }
}

#[test]
fn subnormal_signed_zero_and_infinite_arithmetic_are_preserved() {
    let tiny = Value::Real(rml32::RealBits::from_bits(1));
    assert_eq!(
        numeric_run(1, tiny.clone(), tiny.clone()),
        Value::Real(rml32::RealBits::from_bits(2))
    );
    assert_eq!(numeric_run(0x19, tiny.clone(), real(2.0)), real(0.0));
    assert_eq!(numeric_run(0x19, real(-0.0), real(2.0)), real(-0.0));
    assert_eq!(
        numeric_run(0x29, real(f32::MAX), real(2.0)),
        real(f32::INFINITY)
    );
    let Value::Real(nan) = numeric_run(0x51, real(f32::INFINITY), real(f32::INFINITY)) else {
        panic!();
    };
    assert!(nan.to_f32().is_nan());
    assert_eq!(rml32::RealBits::from_bits(1).bits(), 1);
}

#[test]
fn numeric_casts_reject_non_numeric_sources_and_const_attributes() {
    let p = image(&[fun(0, 1, 0, 255, &[(0, 0, 0x37), (3, 0, 0x0d)])], 0, &[]);
    let (mut vm, o) = machine(p, vec![]);
    vm.start(o, 0, vec![Value::bytes(b"1.5")]).unwrap();
    assert_eq!(vm.run(100).unwrap_err().kind, VmErrorKind::UnsupportedType);
    let p = image(&[fun(0, 0, 0, 255, &[(1, 0, 0x3b), (2, 2, 0x0d)])], 0, &[]);
    let e = prepare_program(p).unwrap_err();
    assert_eq!(e.kind, VmErrorKind::UnsupportedFeature);
    assert_eq!(e.pc, Some(3));
}

#[test]
fn real_branching_and_budget_resume_keep_the_same_result() {
    for (input, expected) in [
        (real(-0.0), 10),
        (real(0.0), 10),
        (real(f32::NAN), 20),
        (real(1.0), 20),
    ] {
        let p = image(
            &[fun(
                0,
                1,
                0,
                2,
                &[
                    (0, 0, 0x37),
                    (2, 3, 0x21),
                    (10, 0, 0x3b),
                    (1, 0, 0x1f),
                    (20, 0, 0x3b),
                ],
            )],
            0,
            &[],
        );
        let (mut vm, o) = machine(p, vec![]);
        vm.start(o, 0, vec![input]).unwrap();
        for _ in 0..20 {
            match vm.run(1).unwrap() {
                RunOutcome::BudgetExhausted => {}
                RunOutcome::Finished(c) => {
                    assert_eq!(c, Completion::Returned(Value::Int(expected)));
                    break;
                }
            }
        }
        assert_eq!(
            vm.run(0).unwrap(),
            RunOutcome::Finished(Completion::Returned(Value::Int(expected)))
        );
    }
}

fn container_run(nodes: &[Node], args: Vec<Value>) -> Result<RunOutcome, rml32::VmError> {
    let (mut vm, o) = machine(image(&[fun(0, args.len(), 0, 255, nodes)], 0, &[]), vec![]);
    vm.start(o, 0, args).unwrap();
    vm.run(1000)
}
#[test]
fn array_constructor_preserves_top_first_order_and_empty_size() {
    let result = container_run(&[(11, 0, 0x3b), (22, 0, 0x3b), (2, 0, 0x16)], vec![]).unwrap();
    let RunOutcome::Finished(Completion::Returned(Value::Array(a))) = result else {
        panic!("array expected")
    };
    assert_eq!(a.elements().unwrap(), &[Value::Int(22), Value::Int(11)]);
    assert_eq!(
        container_run(&[(0, 0, 0x16), (0, 0, 0x4f)], vec![]).unwrap(),
        RunOutcome::Finished(Completion::Returned(Value::Int(0)))
    );
}
#[test]
fn mapping_constructor_uses_total_value_count_and_oldest_pushed_duplicate_wins() {
    let nodes = [
        (11, 0, 0x3b),
        (7, 0, 0x3b),
        (22, 0, 0x3b),
        (7, 0, 0x3b),
        (4, 0, 0x18),
        (7, 0, 0x3b),
        (0, 0, 0x43),
    ];
    assert_eq!(
        container_run(&nodes, vec![]).unwrap(),
        RunOutcome::Finished(Completion::Returned(Value::Int(11)))
    );
    assert_eq!(
        container_run(&[(0, 0, 0x18), (0, 0, 0x4f)], vec![]).unwrap(),
        RunOutcome::Finished(Completion::Returned(Value::Int(0)))
    );
}
#[test]
fn mapping_bytes_are_case_sensitive_non_utf8_and_length_sensitive() {
    let map = Value::mapping(vec![
        (Value::bytes(b"A\xff\0"), Value::Int(1)),
        (Value::bytes(b"a\xff\0"), Value::Int(2)),
        (Value::bytes(b"A\xff\0"), Value::Int(3)),
    ])
    .unwrap();
    let Value::Mapping(m) = &map else {
        unreachable!()
    };
    assert_eq!(m.entries().unwrap().len(), 2);
    for (key, expected) in [
        (b"A\xff\0".as_slice(), Value::Int(3)),
        (b"a\xff\0".as_slice(), Value::Int(2)),
        (b"A\xff".as_slice(), Value::Undefined),
    ] {
        assert_eq!(
            container_run(
                &[(0, 0, 0x37), (1, 0, 0x37), (0, 0, 0x43)],
                vec![map.clone(), Value::bytes(key)]
            )
            .unwrap(),
            RunOutcome::Finished(Completion::Returned(expected))
        );
    }
}
#[test]
fn array_query_truncates_real_index_and_reports_bounds_with_locations() {
    let a = Value::array(vec![Value::Int(10), Value::Int(20)]).unwrap();
    for key in [Value::Int(1), real(1.9)] {
        assert_eq!(
            container_run(
                &[(0, 0, 0x37), (1, 0, 0x37), (0, 0, 0x43)],
                vec![a.clone(), key]
            )
            .unwrap(),
            RunOutcome::Finished(Completion::Returned(Value::Int(20)))
        );
    }
    for key in [Value::Int(-1), Value::Int(2), real(f32::INFINITY)] {
        let e = container_run(
            &[(0, 0, 0x37), (1, 0, 0x37), (0, 0, 0x43)],
            vec![a.clone(), key],
        )
        .unwrap_err();
        assert_eq!(e.kind, VmErrorKind::Bounds);
        assert_eq!(e.pc, Some(4));
        assert!(e.offset.is_some());
    }
}
#[test]
fn malformed_container_counts_and_underflow_fail_without_panics() {
    for (nodes, kind) in [
        (&[(-1, 0, 0x16)][..], VmErrorKind::InvalidProgram),
        (
            &[(1, 0, 0x3b), (1, 0, 0x18)][..],
            VmErrorKind::InvalidProgram,
        ),
        (&[(2, 0, 0x16)][..], VmErrorKind::StackUnderflow),
        (&[(0, 0, 0x43)][..], VmErrorKind::StackUnderflow),
        (&[(0, 0, 0x4f)][..], VmErrorKind::StackUnderflow),
    ] {
        assert_eq!(container_run(nodes, vec![]).unwrap_err().kind, kind);
    }
}
#[test]
fn container_type_restrictions_and_access_modes_are_explicit() {
    assert_eq!(
        Value::mapping(vec![(Value::Undefined, Value::Int(1))])
            .unwrap_err()
            .kind,
        VmErrorKind::UnsupportedType
    );
    let a = Value::array(vec![]).unwrap();
    for op in [1, 0x1c, 0x0d] {
        let nodes = if op == 0x0d {
            vec![(0, 0, 0x37), (2, 0, op)]
        } else {
            vec![(0, 0, 0x37), (0, 0, 0x37), (0, 0, op)]
        };
        assert_eq!(
            container_run(&nodes, vec![a.clone()]).unwrap_err().kind,
            VmErrorKind::UnsupportedType
        );
    }
    for flag in [2, -1] {
        assert_eq!(
            prepare_program(image(&[fun(0, 0, 0, 255, &[(flag, 0, 0x43)])], 0, &[]))
                .unwrap_err()
                .kind,
            VmErrorKind::UnsupportedFeature
        );
    }
    assert_eq!(
        container_run(
            &[(0, 0, 0x37), (1, 0, 0x37), (0, 0, 0x43)],
            vec![a, Value::bytes(b"bad")]
        )
        .unwrap_err()
        .kind,
        VmErrorKind::UnsupportedType
    );
}
#[test]
fn nested_containers_share_identity_and_obey_resource_limits() {
    let nested = Value::array(vec![
        Value::mapping(vec![(Value::Int(1), Value::Int(9))]).unwrap(),
    ])
    .unwrap();
    assert_eq!(nested, nested.clone());
    assert_ne!(nested, Value::array(vec![]).unwrap());
    assert_eq!(
        container_run(
            &[
                (0, 0, 0x37),
                (0, 0, 0x3b),
                (0, 0, 0x43),
                (1, 0, 0x3b),
                (0, 0, 0x43)
            ],
            vec![nested]
        )
        .unwrap(),
        RunOutcome::Finished(Completion::Returned(Value::Int(9)))
    );
    let mut a = Value::Int(0);
    for _ in 0..64 {
        a = Value::array(vec![a]).unwrap();
    }
    assert_eq!(
        Value::array(vec![a.clone()]).unwrap_err().kind,
        VmErrorKind::Limit
    );
    assert_eq!(
        Value::mapping(vec![(Value::Int(0), a)]).unwrap_err().kind,
        VmErrorKind::Limit
    );
    assert!(Value::array(vec![Value::Undefined; 65536]).is_ok());
    assert_eq!(
        Value::array(vec![Value::Undefined; 65537])
            .unwrap_err()
            .kind,
        VmErrorKind::Limit
    );
}
#[test]
fn container_size_tags_truth_and_typed_calls_are_supported() {
    let values = [
        Value::array(vec![]).unwrap(),
        Value::mapping(vec![(Value::Int(1), Value::Int(2))]).unwrap(),
    ];
    for v in values {
        for (op, a, expected) in [
            (0x24, v.tag() as i16, 1),
            (0x2d, 0, 0),
            (0x4f, 0, if v.tag() == 6 { 0 } else { 1 }),
        ] {
            assert_eq!(
                container_run(&[(0, 0, 0x37), (a, 0, op)], vec![v.clone()]).unwrap(),
                RunOutcome::Finished(Completion::Returned(Value::Int(expected)))
            );
        }
        let mut p = image(
            &[
                fun(0, 1, 0, v.tag(), &[(0, 0, 0x37), (1, 1, 0x07)]),
                fun(1, 1, 0, v.tag(), &[(0, 0, 0x37)]),
            ],
            0,
            &[],
        );
        p.functions[1].arguments[0].r#type = v.tag();
        let (mut vm, o) = machine(p, vec![]);
        vm.start(o, 0, vec![v.clone()]).unwrap();
        assert_eq!(vm.run(1).unwrap(), RunOutcome::BudgetExhausted);
        returns(&mut vm, v);
    }
    for (v, n) in [
        (Value::bytes(b"\xff\0"), 2),
        (Value::Int(42), 0),
        (Value::Undefined, 0),
        (Value::Void, 0),
    ] {
        assert_eq!(
            container_run(&[(0, 0, 0x37), (0, 0, 0x4f)], vec![v]).unwrap(),
            RunOutcome::Finished(Completion::Returned(Value::Int(n)))
        );
    }
}
#[test]
fn reassigning_object_variable_preserves_shared_container_and_object_isolation() {
    let old = Value::array(vec![Value::Int(7)]).unwrap();
    let (mut vm, o) = machine(
        image(&[fun(0, 0, 0, 6, &[(0, 0, 0x16), (0, 0, 0x4e)])], 1, &[]),
        vec![old.clone()],
    );
    let pid = vm.objects().get(o).unwrap().program();
    let other = vm.objects_mut().create(pid, vec![old.clone()]).unwrap();
    vm.start(o, 0, vec![]).unwrap();
    assert!(matches!(
        vm.run(100).unwrap(),
        RunOutcome::Finished(Completion::Returned(Value::Array(_)))
    ));
    assert_eq!(
        vm.objects().get(other).unwrap().variables().unwrap(),
        std::slice::from_ref(&old)
    );
    let Value::Array(a) = old else { unreachable!() };
    assert_eq!(a.elements().unwrap(), &[Value::Int(7)]);
}
#[test]
fn member_copy_normalizes_destroyed_object_without_mutating_container() {
    let p = prepare_program(image(
        &[fun(
            0,
            1,
            0,
            255,
            &[(0, 0, 0x37), (0, 0, 0x3b), (0, 0, 0x43)],
        )],
        0,
        &[],
    ))
    .unwrap();
    let mut store = ObjectStore::new();
    let pid = store.register_program(p).unwrap();
    let live = store.create(pid, vec![]).unwrap();
    let dead = store.create(pid, vec![]).unwrap();
    store.remove(dead).unwrap();
    let original = Value::array(vec![Value::Object(dead)]).unwrap();
    let mut vm = Vm::new(store);
    vm.start(live, 0, vec![original.clone()]).unwrap();
    returns(&mut vm, Value::Undefined);
    let Value::Array(a) = original else {
        unreachable!()
    };
    assert_eq!(a.elements().unwrap(), &[Value::Object(dead)]);
}

#[test]
fn reference_assignments_preserve_result_and_update_stable_local_slots() {
    for nodes in [
        vec![(9, 0, 0x3b), (-1, 0, 0x39), (0, 0, 0x03)],
        vec![(-1, 0, 0x39), (9, 0, 0x3b), (0, 0, 0x04)],
    ] {
        let (mut vm, o) = machine(image(&[fun(0, 0, 1, 2, &nodes)], 0, &[]), vec![]);
        vm.start(o, 0, vec![]).unwrap();
        returns(&mut vm, Value::Int(9));
    }
    let nodes = [
        (9, 0, 0x3b),
        (0, 0, 0x39),
        (0, 0, 0x03),
        (0, 0, 0x33),
        (0, 0, 0x37),
    ];
    assert_eq!(
        container_run(&nodes, vec![Value::Int(1)]).unwrap(),
        RunOutcome::Finished(Completion::Returned(Value::Int(9)))
    );
}
#[test]
fn object_reference_assignment_keeps_objects_isolated_and_allows_container_rebinding() {
    let array = Value::array(vec![Value::Int(4)]).unwrap();
    let nodes = [(0, 0, 0x37), (0, 0, 0x3e), (0, 0, 0x03)];
    let (mut vm, a) = machine(
        image(&[fun(0, 1, 0, 6, &nodes)], 1, &[]),
        vec![Value::Int(1)],
    );
    let pid = vm.objects().get(a).unwrap().program();
    let b = vm.objects_mut().create(pid, vec![Value::Int(2)]).unwrap();
    vm.start(a, 0, vec![array.clone()]).unwrap();
    returns(&mut vm, array.clone());
    assert_eq!(
        vm.objects().get(a).unwrap().variables().unwrap(),
        vec![array]
    );
    assert_eq!(
        vm.objects().get(b).unwrap().variables().unwrap(),
        vec![Value::Int(2)]
    );
}
#[test]
fn shared_array_and_mapping_member_writes_are_visible_to_aliases() {
    for container in [
        Value::array(vec![Value::Int(1)]).unwrap(),
        Value::mapping(vec![(Value::Int(0), Value::Int(1))]).unwrap(),
    ] {
        let nodes = [
            (9, 0, 0x3b),
            (0, 0, 0x3e),
            (0, 0, 0x3b),
            (0, 0, 0x44),
            (0, 0, 0x03),
            (0, 0, 0x33),
            (1, 0, 0x3d),
            (0, 0, 0x3b),
            (0, 0, 0x43),
        ];
        let (mut vm, o) = machine(
            image(&[fun(0, 0, 0, 2, &nodes)], 2, &[]),
            vec![container.clone(), container.clone()],
        );
        vm.start(o, 0, vec![]).unwrap();
        returns(&mut vm, Value::Int(9));
        assert_eq!(
            vm.objects().get(o).unwrap().variables().unwrap(),
            vec![container.clone(), container]
        );
    }
}
#[test]
fn member_refs_keep_old_slots_after_variable_rebinding_with_native_release_difference() {
    for (old, new, expected) in [
        (
            Value::array(vec![Value::Int(8)]).unwrap(),
            Value::array(vec![]).unwrap(),
            Value::Int(8),
        ),
        (
            Value::mapping(vec![(Value::Int(0), Value::Int(8))]).unwrap(),
            Value::mapping(vec![]).unwrap(),
            Value::Undefined,
        ),
    ] {
        // No host copy of old header is retained. The member Ref alone survives.
        let nodes = [
            (0, 0, 0x3e),
            (0, 0, 0x3b),
            (0, 0, 0x44),
            (0, 0, 0x37),
            (0, 0, 0x4e),
            (0, 0, 0x33),
            (0, 0, 0x14),
        ];
        let (mut vm, o) = machine(image(&[fun(0, 1, 0, 255, &nodes)], 1, &[]), vec![old]);
        vm.start(o, 0, vec![new]).unwrap();
        returns(&mut vm, expected);
    }
}
#[test]
fn temporary_array_and_mapping_conversion_release_headers_at_the_right_time() {
    for (op, key, expected) in [(0x16, 0, Value::Int(9)), (0x18, 7, Value::Undefined)] {
        let mut nodes = vec![(9, 0, 0x3b)];
        if op == 0x18 {
            nodes.push((7, 0, 0x3b));
        }
        nodes.extend([
            (if op == 0x16 { 1 } else { 2 }, 0, op),
            (0, 0, 0x13),
            (0, 0, 0x13),
            (key, 0, 0x3b),
            (0, 0, 0x44),
            (0, 0, 0x14),
        ]);
        assert_eq!(
            container_run(&nodes, vec![]).unwrap(),
            RunOutcome::Finished(Completion::Returned(expected))
        );
    }
}
#[test]
fn missing_mapping_reference_inserts_undefined_before_assignment() {
    let m = Value::mapping(vec![]).unwrap();
    let nodes = [(0, 0, 0x3e), (7, 0, 0x3b), (1, 0, 0x44), (0, 0, 0x14)];
    let (mut vm, o) = machine(image(&[fun(0, 0, 0, 255, &nodes)], 1, &[]), vec![m.clone()]);
    vm.start(o, 0, vec![]).unwrap();
    returns(&mut vm, Value::Undefined);
    let Value::Mapping(m) = m else { unreachable!() };
    assert_eq!(
        m.entries().unwrap(),
        vec![(Value::Int(7), Value::Undefined)]
    );
}
#[test]
fn failed_member_container_write_preserves_value_and_successful_prior_insertion() {
    for existing in [false, true] {
        let m = Value::mapping(if existing {
            vec![(Value::Int(7), Value::Int(5))]
        } else {
            vec![]
        })
        .unwrap();
        let nodes = [
            (0, 0, 0x37),
            (0, 0, 0x3e),
            (7, 0, 0x3b),
            (0, 0, 0x44),
            (0, 0, 0x03),
        ];
        let (mut vm, o) = machine(image(&[fun(0, 1, 0, 255, &nodes)], 1, &[]), vec![m.clone()]);
        vm.start(o, 0, vec![Value::array(vec![]).unwrap()]).unwrap();
        let e = vm.run(100).unwrap_err();
        assert_eq!(e.kind, VmErrorKind::UnsupportedType);
        assert_eq!(e.pc, Some(6));
        assert!(
            e.program.is_some() && e.object.is_some() && e.function.is_some() && e.offset.is_some()
        );
        assert_eq!(vm.run(100).unwrap_err(), e);
        let Value::Mapping(m) = m else { unreachable!() };
        assert_eq!(
            m.get(&Value::Int(7)).unwrap(),
            Some(if existing {
                Value::Int(5)
            } else {
                Value::Undefined
            })
        );
    }
}
#[test]
fn nested_member_writes_and_depth_recompute_follow_shared_identity() {
    let inner = Value::array(vec![Value::Int(1)]).unwrap();
    let outer = Value::array(vec![inner.clone()]).unwrap();
    let nodes = [
        (9, 0, 0x3b),
        (0, 0, 0x3e),
        (0, 0, 0x3b),
        (0, 0, 0x44),
        (0, 0, 0x3b),
        (0, 0, 0x44),
        (0, 0, 0x03),
    ];
    let (mut vm, o) = machine(
        image(&[fun(0, 0, 0, 2, &nodes)], 1, &[]),
        vec![outer.clone()],
    );
    vm.start(o, 0, vec![]).unwrap();
    returns(&mut vm, Value::Int(9));
    let Value::Array(inner) = inner else {
        unreachable!()
    };
    assert_eq!(inner.elements().unwrap(), vec![Value::Int(9)]);
    // Replace the nested container itself with a scalar: no new graph edges.
    let nodes = [
        (3, 0, 0x3b),
        (0, 0, 0x3e),
        (0, 0, 0x3b),
        (0, 0, 0x44),
        (0, 0, 0x03),
    ];
    let (mut vm, o) = machine(
        image(&[fun(0, 0, 0, 2, &nodes)], 1, &[]),
        vec![outer.clone()],
    );
    vm.start(o, 0, vec![]).unwrap();
    returns(&mut vm, Value::Int(3));
    let Value::Array(a) = outer else {
        unreachable!()
    };
    assert_eq!(a.depth().unwrap(), 1);
}
#[test]
fn depth_memoizes_shared_subgraphs_and_obeys_existing_limit() {
    let mut value = Value::Int(0);
    for _ in 0..64 {
        value = Value::array(vec![value.clone(), value]).unwrap();
    }
    let Value::Array(a) = &value else {
        unreachable!()
    };
    assert_eq!(a.depth().unwrap(), 64);
    assert!(Value::array(vec![value]).is_err());
}
#[test]
fn reverse_array_modes_read_and_write_with_truncated_real_indices() {
    for key in [Value::Int(1), real(1.9)] {
        let a = Value::array(vec![Value::Int(4), Value::Int(5), Value::Int(6)]).unwrap();
        assert_eq!(
            container_run(
                &[(0, 0, 0x37), (1, 0, 0x37), (1, 0, 0x43)],
                vec![a.clone(), key.clone()]
            )
            .unwrap(),
            RunOutcome::Finished(Completion::Returned(Value::Int(6)))
        );
        let nodes = [
            (9, 0, 0x3b),
            (0, 0, 0x3e),
            (0, 0, 0x37),
            (1, 0, 0x44),
            (0, 0, 0x03),
        ];
        let (mut vm, o) = machine(image(&[fun(0, 1, 0, 2, &nodes)], 1, &[]), vec![a.clone()]);
        vm.start(o, 0, vec![key]).unwrap();
        returns(&mut vm, Value::Int(9));
        let Value::Array(a) = a else { unreachable!() };
        assert_eq!(
            a.elements().unwrap(),
            vec![Value::Int(4), Value::Int(5), Value::Int(9)]
        );
    }
    for key in [0, 4, -1] {
        assert_eq!(
            container_run(
                &[(0, 0, 0x37), (key, 0, 0x3b), (1, 0, 0x43)],
                vec![Value::array(vec![Value::Int(1); 3]).unwrap()]
            )
            .unwrap_err()
            .kind,
            VmErrorKind::Bounds
        );
    }
}
#[test]
fn reference_writes_preserve_raw_byte_keys_and_scalar_types() {
    for value in [
        Value::Int(2),
        real(2.5),
        Value::Undefined,
        Value::Void,
        Value::bytes(b"\xff\0"),
    ] {
        let m = Value::mapping(vec![(Value::bytes(b"A\xff"), Value::Int(1))]).unwrap();
        let nodes = [
            (0, 0, 0x37),
            (0, 0, 0x3e),
            (1, 0, 0x37),
            (0, 0, 0x44),
            (0, 0, 0x03),
        ];
        let (mut vm, o) = machine(image(&[fun(0, 2, 0, 255, &nodes)], 1, &[]), vec![m.clone()]);
        vm.start(o, 0, vec![value.clone(), Value::bytes(b"a\xff")])
            .unwrap();
        returns(&mut vm, value.clone());
        let Value::Mapping(m) = m else { unreachable!() };
        assert_eq!(m.entries().unwrap().len(), 2);
        assert_eq!(m.get(&Value::bytes(b"A\xff")).unwrap(), Some(Value::Int(1)));
        assert_eq!(m.get(&Value::bytes(b"a\xff")).unwrap(), Some(value));
    }
}
#[test]
fn reference_snapshot_and_budget_resume_keep_stable_target_identity() {
    let nodes = [(0, 0, 0x39), (0, 0, 0x14)];
    let (mut vm, o) = machine(image(&[fun(0, 1, 0, 2, &nodes)], 0, &[]), vec![]);
    vm.start(o, 0, vec![Value::Int(8)]).unwrap();
    assert_eq!(vm.run(3).unwrap(), RunOutcome::BudgetExhausted);
    let before = vm.stack().unwrap();
    assert!(
        matches!(before.last(),Some(rml32::StackSnapshot::Reference {kind:rml32::ReferenceKind::Local,valid:true,target_id,..}) if *target_id!=0)
    );
    assert_eq!(vm.run(0).unwrap(), RunOutcome::BudgetExhausted);
    assert_eq!(vm.stack().unwrap(), before);
    returns(&mut vm, Value::Int(8));
}
#[test]
fn pending_references_survive_nested_calls_without_being_passed_as_arguments() {
    let f0 = fun(0, 0, 1, 2, &[(-1, 0, 0x39), (1, 0, 0x07), (0, 0, 0x04)]);
    let f1 = fun(1, 0, 0, 2, &[(9, 0, 0x3b)]);
    let (mut vm, o) = machine(image(&[f0, f1], 0, &[]), vec![]);
    vm.start(o, 0, vec![]).unwrap();
    returns(&mut vm, Value::Int(9));
}
#[test]
fn references_cannot_escape_via_returns_arguments_or_containers() {
    for nodes in [
        vec![(0, 0, 0x39)],
        vec![(0, 0, 0x39), (1, 0, 0x16)],
        vec![(0, 0, 0x39), (0, 1, 0x07)],
        vec![(0, 0, 0x39), (0, 0, 0x4d)],
        vec![(0, 0, 0x42), (-1, 0, 0x39), (9, 0, 0x3b), (-2, 0, 0x4d)],
    ] {
        assert_eq!(
            container_run(&nodes, vec![Value::Int(1)]).unwrap_err().kind,
            VmErrorKind::UnsupportedFeature,
            "nodes: {nodes:?}"
        );
    }
    for op in [0x03, 0x04, 0x13, 0x14, 0x44] {
        assert_eq!(
            container_run(&[(0, 0, op)], vec![]).unwrap_err().kind,
            VmErrorKind::StackUnderflow
        );
    }
    assert_eq!(
        container_run(&[(1, 0, 0x3b), (0, 0, 0x13)], vec![])
            .unwrap_err()
            .kind,
        VmErrorKind::UnsupportedType
    );
    assert_eq!(
        container_run(&[(1, 0, 0x3b), (0, 0, 0x14)], vec![])
            .unwrap_err()
            .kind,
        VmErrorKind::UnsupportedType
    );
}
#[test]
fn deleting_object_invalidates_pending_object_reference_and_does_not_reuse_id() {
    let nodes = [(0, 0, 0x3e), (0, 0, 0x14)];
    let (mut vm, o) = machine(
        image(&[fun(0, 0, 0, 255, &nodes)], 1, &[]),
        vec![Value::Int(1)],
    );
    vm.start(o, 0, vec![]).unwrap();
    vm.run(3).unwrap();
    let pid = vm.objects().get(o).unwrap().program();
    let before = vm.stack().unwrap();
    vm.objects_mut().remove(o).unwrap();
    let other = vm.objects_mut().create(pid, vec![Value::Int(2)]).unwrap();
    assert_ne!(o, other);
    let after = vm.stack().unwrap();
    match (&before[0], &after[0]) {
        (
            rml32::StackSnapshot::Reference {
                target_id: a,
                valid: true,
                ..
            },
            rml32::StackSnapshot::Reference {
                target_id: b,
                valid: false,
                ..
            },
        ) => assert_eq!(a, b),
        _ => panic!("reference snapshots expected"),
    };
    assert_eq!(vm.run(1).unwrap_err().kind, VmErrorKind::InvalidObject);
}
#[test]
fn temporary_reference_cell_limit_fails_before_mutation_and_releases_capacity() {
    let nodes = [(0, 0, 0x16), (0, 0, 0x13)];
    let (mut vm, o) = machine(image(&[fun(0, 0, 0, 255, &nodes)], 0, &[]), vec![]);
    vm = Vm::with_limits(
        vm.into_objects(),
        VmLimits {
            max_reference_cells: 0,
            ..VmLimits::default()
        },
    );
    vm.start(o, 0, vec![]).unwrap();
    assert_eq!(vm.run(100).unwrap_err().kind, VmErrorKind::Limit);
    let nodes = [
        (0, 0, 0x16),
        (0, 0, 0x13),
        (0, 0, 0x33),
        (0, 0, 0x16),
        (0, 0, 0x13),
        (0, 0, 0x14),
    ];
    let (mut vm, o) = machine(image(&[fun(0, 0, 0, 6, &nodes)], 0, &[]), vec![]);
    vm = Vm::with_limits(
        vm.into_objects(),
        VmLimits {
            max_reference_cells: 1,
            ..VmLimits::default()
        },
    );
    vm.start(o, 0, vec![]).unwrap();
    assert!(matches!(
        vm.run(100).unwrap(),
        RunOutcome::Finished(Completion::Returned(Value::Array(_)))
    ));
}
#[test]
fn reference_limit_does_not_insert_missing_mapping_key_when_query_fails() {
    let map = Value::mapping(vec![]).unwrap();
    let nodes = [(0, 0, 0x3e), (7, 0, 0x3b), (0, 0, 0x44)];
    let (vm, o) = machine(
        image(&[fun(0, 0, 0, 255, &nodes)], 1, &[]),
        vec![map.clone()],
    );
    let mut vm = Vm::with_limits(
        vm.into_objects(),
        VmLimits {
            max_reference_cells: 0,
            ..VmLimits::default()
        },
    );
    vm.start(o, 0, vec![]).unwrap();
    assert_eq!(vm.run(100).unwrap_err().kind, VmErrorKind::Limit);
    let Value::Mapping(m) = map else {
        unreachable!()
    };
    assert!(m.entries().unwrap().is_empty());
}
#[test]
fn mapping_capacity_and_member_bounds_errors_leave_containers_unchanged() {
    let map = Value::mapping(
        (0..65536)
            .map(|i| (Value::Int(i), Value::Undefined))
            .collect(),
    )
    .unwrap();
    let nodes = [(0, 0, 0x3e), (0, 0, 0x37), (0, 0, 0x44)];
    let (mut vm, o) = machine(
        image(&[fun(0, 1, 0, 255, &nodes)], 1, &[]),
        vec![map.clone()],
    );
    vm.start(o, 0, vec![Value::Int(65536)]).unwrap();
    assert_eq!(vm.run(100).unwrap_err().kind, VmErrorKind::Limit);
    let Value::Mapping(m) = map else {
        unreachable!()
    };
    assert_eq!(m.entries().unwrap().len(), 65536);
    for (value, key) in [
        (Value::array(vec![]).unwrap(), Value::Int(0)),
        (Value::array(vec![Value::Int(1)]).unwrap(), Value::Int(-1)),
    ] {
        let (mut vm, o) = machine(image(&[fun(0, 1, 0, 255, &nodes)], 1, &[]), vec![value]);
        vm.start(o, 0, vec![key]).unwrap();
        assert_eq!(vm.run(100).unwrap_err().kind, VmErrorKind::Bounds);
    }
}
#[test]
fn mapping_insertion_keeps_existing_member_reference_stable() {
    let map = Value::mapping(vec![(Value::Int(7), Value::Int(3))]).unwrap();
    let nodes = [
        (0, 0, 0x3e),
        (7, 0, 0x3b),
        (0, 0, 0x44), // retain old member Ref
        (0, 0, 0x3e),
        (8, 0, 0x3b),
        (0, 0, 0x44),
        (0, 0, 0x33), // insert another key
        (9, 0, 0x3b),
        (0, 0, 0x04),
    ];
    let (mut vm, o) = machine(image(&[fun(0, 0, 0, 2, &nodes)], 1, &[]), vec![map.clone()]);
    vm.start(o, 0, vec![]).unwrap();
    returns(&mut vm, Value::Int(9));
    let Value::Mapping(m) = map else {
        unreachable!()
    };
    assert_eq!(m.get(&Value::Int(7)).unwrap(), Some(Value::Int(9)));
    assert_eq!(m.get(&Value::Int(8)).unwrap(), Some(Value::Undefined));
}
#[test]
fn member_write_can_store_live_object_and_rejects_cycles_before_change() {
    let a = Value::array(vec![Value::Int(1)]).unwrap();
    let nodes = [
        (0, 0, 0x37),
        (0, 0, 0x3e),
        (0, 0, 0x3b),
        (0, 0, 0x44),
        (0, 0, 0x03),
    ];
    let (mut vm, o) = machine(image(&[fun(0, 1, 0, 255, &nodes)], 1, &[]), vec![a.clone()]);
    vm.start(o, 0, vec![Value::Object(o)]).unwrap();
    returns(&mut vm, Value::Object(o));
    vm.start(o, 0, vec![a.clone()]).unwrap();
    assert_eq!(vm.run(100).unwrap_err().kind, VmErrorKind::UnsupportedType);
    let Value::Array(a) = a else { unreachable!() };
    assert_eq!(a.elements().unwrap(), vec![Value::Object(o)]);
}
#[test]
fn reference_modes_invalid_targets_and_invalid_keys_fail_explicitly() {
    for value in [Value::bytes(b"x"), Value::Int(1)] {
        let nodes = [(0, 0, 0x3e), (0, 0, 0x3b), (0, 0, 0x44)];
        let (mut vm, o) = machine(image(&[fun(0, 0, 0, 255, &nodes)], 1, &[]), vec![value]);
        vm.start(o, 0, vec![]).unwrap();
        assert_eq!(vm.run(100).unwrap_err().kind, VmErrorKind::UnsupportedType);
    }
    for op in [0x43, 0x44] {
        for mode in [-1, 2] {
            assert_eq!(
                prepare_program(image(&[fun(0, 0, 0, 255, &[(mode, 0, op)])], 0, &[]))
                    .unwrap_err()
                    .kind,
                VmErrorKind::UnsupportedFeature
            );
        }
    }
    let nodes = [(0, 0, 0x3e), (0, 0, 0x42), (0, 0, 0x44)];
    let (mut vm, o) = machine(
        image(&[fun(0, 0, 0, 255, &nodes)], 1, &[]),
        vec![Value::mapping(vec![]).unwrap()],
    );
    vm.start(o, 0, vec![]).unwrap();
    assert_eq!(vm.run(100).unwrap_err().kind, VmErrorKind::UnsupportedType);
}

fn update_machine(
    op: u8,
    operand: i16,
    left: Value,
    right: Option<Value>,
) -> (Vm, rml32::ObjectId) {
    let mut nodes = vec![(0, 0, 0x3e)];
    let args: Vec<_> = right.into_iter().collect();
    if op == 0x2f {
        nodes.push((0, 0, 0x37));
    }
    nodes.push((operand, 0, op));
    if op == 0x22 {
        nodes.push((0, 0, 0x3d));
    }
    let (mut vm, o) = machine(
        image(&[fun(0, args.len(), 0, 255, &nodes)], 1, &[]),
        vec![left],
    );
    vm.start(o, 0, args).unwrap();
    (vm, o)
}
fn updated(op: u8, operand: i16, left: Value, right: Option<Value>) -> Value {
    let (mut vm, o) = update_machine(op, operand, left, right);
    let RunOutcome::Finished(Completion::Returned(value)) = vm.run(100).unwrap() else {
        panic!("value return expected")
    };
    assert_eq!(
        vm.objects().get(o).unwrap().variables().unwrap(),
        vec![value.clone()]
    );
    value
}
#[test]
fn increment_accepts_signed_par16_and_wraps_integer_extremes() {
    for (left, delta, expected) in [
        (5, 1, 6),
        (5, -1, 4),
        (5, 0, 5),
        (5, 7, 12),
        (0, i16::MIN, -32768),
        (0, i16::MAX, 32767),
        (i32::MAX, 1, i32::MIN),
        (i32::MIN, -1, i32::MAX),
    ] {
        assert_eq!(
            updated(0x22, delta, Value::Int(left), None),
            Value::Int(expected)
        );
    }
}
#[test]
fn increment_real_rounding_special_values_and_zero_sign_follow_fiadd_store() {
    for (left, delta, expected) in [
        (1.5, 1, 2.5),
        (1.5, -1, 0.5),
        (-0.0, 0, 0.0),
        (16777216.0, 1, 16777216.0),
        (16777216.0, 3, 16777220.0),
        (f32::from_bits(1), 0, f32::from_bits(1)),
        (f32::MAX, 32767, f32::MAX),
        (f32::INFINITY, -1, f32::INFINITY),
        (f32::NEG_INFINITY, 1, f32::NEG_INFINITY),
    ] {
        assert_eq!(updated(0x22, delta, real(left), None), real(expected));
    }
    let Value::Real(r) = updated(0x22, 0, real(f32::NAN), None) else {
        unreachable!()
    };
    assert!(r.to_f32().is_nan());
}
#[test]
fn increment_consumes_reference_without_leaving_expression_result() {
    let (mut vm, o) = machine(
        image(&[fun(0, 0, 0, 1, &[(0, 0, 0x3e), (1, 0, 0x22)])], 1, &[]),
        vec![Value::Int(1)],
    );
    vm.start(o, 0, vec![]).unwrap();
    returns(&mut vm, Value::Void);
    assert!(vm.stack().unwrap().is_empty());
    assert_eq!(
        vm.objects().get(o).unwrap().variables().unwrap(),
        vec![Value::Int(2)]
    );
    assert_eq!(
        container_run(
            &[(0, 0, 0x39), (2, 0, 0x22), (0, 0, 0x37)],
            vec![Value::Int(7)]
        )
        .unwrap(),
        RunOutcome::Finished(Completion::Returned(Value::Int(9)))
    );
    let nodes = [
        (5, 0, 0x3b),
        (-1, 0, 0x4d),
        (0, 0, 0x33),
        (-1, 0, 0x39),
        (-2, 0, 0x22),
        (-1, 0, 0x37),
    ];
    let (mut vm, o) = machine(image(&[fun(0, 0, 1, 2, &nodes)], 0, &[]), vec![]);
    vm.start(o, 0, vec![]).unwrap();
    returns(&mut vm, Value::Int(3));
}
#[test]
fn compound_selectors_cover_all_ten_operations_and_leave_result() {
    for (selector, expected) in [
        (2, 17),
        (3, 11),
        (4, 42),
        (5, 4),
        (6, 2),
        (7, 15),
        (8, 13),
        (9, 1),
        (10, 112),
        (11, 2),
    ] {
        assert_eq!(
            updated(0x2f, selector, Value::Int(14), Some(Value::Int(3))),
            Value::Int(expected)
        );
    }
    assert_eq!(
        updated(0x2f, 2, Value::Int(i32::MAX), Some(Value::Int(1))),
        Value::Int(i32::MIN)
    );
    assert_eq!(
        updated(0x2f, 3, Value::Int(i32::MIN), Some(Value::Int(1))),
        Value::Int(i32::MAX)
    );
    assert_eq!(
        updated(0x2f, 4, Value::Int(i32::MAX), Some(Value::Int(2))),
        Value::Int(-2)
    );
}
#[test]
fn compound_mixed_arithmetic_preserves_target_type_except_remainder() {
    for (selector, int_expected, float_expected) in
        [(2, 9, 9.5), (3, 4, 4.5), (4, 17, 17.5), (5, 2, 2.8)]
    {
        assert_eq!(
            updated(0x2f, selector, Value::Int(7), Some(real(2.5))),
            Value::Int(int_expected)
        );
        assert_eq!(
            updated(0x2f, selector, real(7.0), Some(real(2.5))),
            real(float_expected)
        );
    }
    assert_eq!(updated(0x2f, 2, real(2.5), Some(Value::Int(7))), real(9.5));
    assert_eq!(updated(0x2f, 3, real(2.5), Some(Value::Int(7))), real(-4.5));
    assert_eq!(
        updated(0x2f, 4, Value::Int(-7), Some(real(2.5))),
        Value::Int(-17)
    );
    assert_eq!(updated(0x2f, 11, Value::Int(7), Some(real(2.5))), real(2.0));
    assert_eq!(
        updated(0x2f, 11, real(-7.0), Some(Value::Int(3))),
        real(-1.0)
    );
}
#[test]
fn compound_inline_multiply_matches_x87_precision_and_float_store_before_ftol() {
    assert_eq!(
        updated(0x2f, 3, Value::Int(16777217), Some(real(16777216.0))),
        Value::Int(1)
    );
    assert_eq!(
        updated(0x2f, 3, real(16777216.0), Some(Value::Int(16777217))),
        real(-1.0)
    );
    assert_eq!(
        updated(0x2f, 4, Value::Int(16777217), Some(real(1.0))),
        Value::Int(16777216)
    );
    for (r, i, bits) in [
        (16777215.0, 1090519041, 0x5a81ffff),
        (8388609.0, 1619001343, 0x5a410001),
    ] {
        assert_eq!(
            updated(0x2f, 4, real(r), Some(Value::Int(i))),
            Value::Real(rml32::RealBits::from_bits(bits))
        );
    }
    assert_eq!(
        updated(0x2f, 4, Value::Int(1), Some(real(f32::INFINITY))),
        Value::Int(i32::MIN)
    );
}
#[test]
fn compound_float_special_values_and_shift_edges_keep_numeric_policy() {
    assert_eq!(updated(0x2f, 4, real(-0.0), Some(real(2.0))), real(-0.0));
    assert_eq!(
        updated(0x2f, 2, real(f32::INFINITY), Some(Value::Int(1))),
        real(f32::INFINITY)
    );
    assert_eq!(
        updated(0x2f, 4, real(f32::from_bits(1)), Some(Value::Int(1))),
        real(f32::from_bits(1))
    );
    for (selector, right) in [(2, real(f32::NAN)), (11, real(0.0))] {
        let Value::Real(r) = updated(0x2f, selector, real(1.0), Some(right)) else {
            unreachable!()
        };
        assert!(r.to_f32().is_nan());
    }
    for (shift, rsh, lsh) in [(-1, 0, 0), (0, -1, -1), (31, 1, i32::MIN), (32, 0, 0)] {
        assert_eq!(
            updated(0x2f, 9, Value::Int(-1), Some(Value::Int(shift))),
            Value::Int(rsh)
        );
        assert_eq!(
            updated(0x2f, 10, Value::Int(-1), Some(Value::Int(shift))),
            Value::Int(lsh)
        );
    }
}
#[test]
fn numeric_update_errors_preserve_target_and_operand_stack_with_locations() {
    let mut cases = vec![
        (
            0x22,
            1,
            Value::Undefined,
            None,
            VmErrorKind::UnsupportedType,
        ),
        (
            0x22,
            0,
            Value::bytes(b"x"),
            None,
            VmErrorKind::UnsupportedType,
        ),
        (
            0x22,
            1,
            Value::array(vec![]).unwrap(),
            None,
            VmErrorKind::UnsupportedType,
        ),
    ];
    for selector in [5, 11] {
        for (left, right) in [
            (Value::Int(3), Value::Int(0)),
            (Value::Int(i32::MIN), Value::Int(-1)),
        ] {
            cases.push((0x2f, selector, left, Some(right), VmErrorKind::Arithmetic));
        }
    }
    cases.push((
        0x2f,
        5,
        real(1.0),
        Some(real(-0.0)),
        VmErrorKind::Arithmetic,
    ));
    for selector in 2..=11 {
        cases.push((
            0x2f,
            selector,
            Value::bytes(b"x"),
            Some(Value::Int(1)),
            VmErrorKind::UnsupportedType,
        ));
    }
    for selector in 6..=10 {
        cases.push((
            0x2f,
            selector,
            Value::Int(1),
            Some(real(1.0)),
            VmErrorKind::UnsupportedType,
        ));
    }
    for (op, selector, left, right, kind) in cases {
        let expected = left.clone();
        let (mut vm, o) = update_machine(op, selector, left, right);
        let prefix = if op == 0x22 { 3 } else { 4 };
        assert_eq!(vm.run(prefix).unwrap(), RunOutcome::BudgetExhausted);
        let stack = vm.stack().unwrap();
        let e = vm.run(1).unwrap_err();
        assert_eq!(e.kind, kind);
        assert_eq!(e.pc, Some(prefix));
        assert!(
            e.offset.is_some()
                && e.program.is_some()
                && e.object.is_some()
                && e.function.is_some()
                && !e.source.is_empty()
        );
        assert_eq!(vm.stack().unwrap(), stack);
        assert_eq!(
            vm.objects().get(o).unwrap().variables().unwrap(),
            vec![expected]
        );
        assert_eq!(vm.run(100).unwrap_err(), e);
    }
}
#[test]
fn compound_rejects_invalid_selector_during_preparation_and_bad_stack_forms_at_runtime() {
    for selector in [-32768, -1, 0, 1, 12, 32767] {
        let e = prepare_program(image(&[fun(0, 0, 0, 255, &[(selector, 0, 0x2f)])], 0, &[]))
            .unwrap_err();
        assert_eq!(e.kind, VmErrorKind::UnsupportedFeature);
        assert_eq!(e.pc, Some(2));
        assert!(e.offset.is_some());
    }
    for (nodes, kind) in [
        (vec![(1, 0, 0x22)], VmErrorKind::StackUnderflow),
        (vec![(2, 0, 0x2f)], VmErrorKind::StackUnderflow),
        (
            vec![(0, 0, 0x3e), (2, 0, 0x2f)],
            VmErrorKind::StackUnderflow,
        ),
        (
            vec![(1, 0, 0x3b), (1, 0, 0x22)],
            VmErrorKind::UnsupportedType,
        ),
        (
            vec![(1, 0, 0x3b), (0, 0, 0x3e), (2, 0, 0x2f)],
            VmErrorKind::UnsupportedType,
        ),
        (
            vec![(0, 0, 0x3e), (0, 0, 0x3e), (2, 0, 0x2f)],
            VmErrorKind::UnsupportedFeature,
        ),
    ] {
        let (mut vm, o) = machine(
            image(&[fun(0, 0, 0, 255, &nodes)], 1, &[]),
            vec![Value::Int(3)],
        );
        vm.start(o, 0, vec![]).unwrap();
        assert_eq!(vm.run(100).unwrap_err().kind, kind);
        assert_eq!(
            vm.objects().get(o).unwrap().variables().unwrap(),
            vec![Value::Int(3)]
        );
    }
}
#[test]
fn numeric_updates_reach_shared_members_and_keep_object_isolation() {
    for container in [
        Value::array(vec![Value::Int(5)]).unwrap(),
        Value::mapping(vec![(Value::Int(0), Value::Int(5))]).unwrap(),
    ] {
        let nodes = [
            (0, 0, 0x3e),
            (0, 0, 0x3b),
            (0, 0, 0x44),
            (2, 0, 0x22),
            (0, 0, 0x3e),
            (0, 0, 0x3b),
            (0, 0, 0x44),
            (3, 0, 0x3b),
            (4, 0, 0x2f),
        ];
        let (mut vm, o) = machine(
            image(&[fun(0, 0, 0, 2, &nodes)], 1, &[]),
            vec![container.clone()],
        );
        let pid = vm.objects().get(o).unwrap().program();
        let other = vm.objects_mut().create(pid, vec![Value::Int(100)]).unwrap();
        vm.start(o, 0, vec![]).unwrap();
        returns(&mut vm, Value::Int(21));
        let result = match container {
            Value::Array(a) => a.elements().unwrap()[0].clone(),
            Value::Mapping(m) => m.get(&Value::Int(0)).unwrap().unwrap(),
            _ => unreachable!(),
        };
        assert_eq!(result, Value::Int(21));
        assert_eq!(
            vm.objects().get(other).unwrap().variables().unwrap(),
            vec![Value::Int(100)]
        );
    }
}
#[test]
fn numeric_updates_on_temporaries_and_local_slots_use_existing_lifetime_rules() {
    let nodes = [
        (7, 0, 0x3b),
        (1, 0, 0x16),
        (0, 0, 0x13),
        (0, 0, 0x3b),
        (0, 0, 0x44),
        (3, 0, 0x3b),
        (2, 0, 0x2f),
    ];
    assert_eq!(
        container_run(&nodes, vec![]).unwrap(),
        RunOutcome::Finished(Completion::Returned(Value::Int(10)))
    );
    let nodes = [(0, 0, 0x39), (3, 0, 0x3b), (4, 0, 0x2f)];
    assert_eq!(
        container_run(&nodes, vec![Value::Int(7)]).unwrap(),
        RunOutcome::Finished(Completion::Returned(Value::Int(21)))
    );
    // A missing mapping member was inserted by the prior instruction; numeric
    // update errors must leave that successful insertion intact.
    let m = Value::mapping(vec![]).unwrap();
    let nodes = [
        (0, 0, 0x3e),
        (7, 0, 0x3b),
        (0, 0, 0x44),
        (1, 0, 0x3b),
        (2, 0, 0x2f),
    ];
    let (mut vm, o) = machine(image(&[fun(0, 0, 0, 255, &nodes)], 1, &[]), vec![m.clone()]);
    vm.start(o, 0, vec![]).unwrap();
    assert_eq!(vm.run(100).unwrap_err().kind, VmErrorKind::UnsupportedType);
    let Value::Mapping(m) = m else { unreachable!() };
    assert_eq!(m.get(&Value::Int(7)).unwrap(), Some(Value::Undefined));
}
#[test]
fn numeric_update_budget_resume_commits_once_and_deleted_objects_error_without_changes() {
    for (op, a, right, expected, prefix) in [
        (0x22, 1, None, Value::Int(6), 3),
        (0x2f, 2, Some(Value::Int(3)), Value::Int(8), 4),
    ] {
        let (mut vm, o) = update_machine(op, a, Value::Int(5), right.clone());
        assert_eq!(vm.run(prefix).unwrap(), RunOutcome::BudgetExhausted);
        let before = vm.stack().unwrap();
        assert_eq!(vm.run(0).unwrap(), RunOutcome::BudgetExhausted);
        assert_eq!(before, vm.stack().unwrap());
        assert_eq!(vm.run(1).unwrap(), RunOutcome::BudgetExhausted);
        assert_eq!(
            vm.objects().get(o).unwrap().variables().unwrap(),
            vec![expected.clone()]
        );
        returns(&mut vm, expected);
        let (mut vm, o) = update_machine(op, a, Value::Int(5), right);
        vm.run(prefix).unwrap();
        let before = vm.stack().unwrap();
        vm.objects_mut().remove(o).unwrap();
        assert_eq!(vm.run(1).unwrap_err().kind, VmErrorKind::InvalidObject);
        assert_eq!(vm.stack().unwrap().len(), before.len());
    }
}

// Stage 2H: synthetic reference-argument programs; no shipped code executes.
fn ref_arg(p: &mut ProgramImage, f: usize, arg: usize, ty: u8, attrib: u8) {
    let a = &mut p.functions[f].arguments[arg];
    a.flag = 1;
    a.r#type = ty;
    a.attrib = attrib;
    let mut raw = vec![0; 20];
    raw[4] = 1;
    raw[5] = ty;
    raw[6] = attrib;
    raw[8..10].copy_from_slice(&a.index.to_le_bytes());
    a.raw_hex = rml32::hex(&raw);
}
fn object_values(vm: &Vm, o: rml32::ObjectId) -> Vec<Value> {
    vm.objects().get(o).unwrap().variables().unwrap()
}

#[test]
fn reference_arguments_write_through_local_argument_and_object_slots() {
    for (source, vars) in [(0x39, 0), (0x3e, 1)] {
        let caller = fun(
            0,
            1,
            0,
            255,
            &[
                (0, 0, source),
                (1, 1, 0x07),
                (0, 0, 0x33),
                (0, 0, if source == 0x39 { 0x37 } else { 0x3d }),
            ],
        );
        let callee = fun(1, 1, 0, 1, &[(9, 0, 0x3b), (0, 0, 0x4d), (0, 0, 0x33)]);
        let mut p = image(&[caller, callee], vars, &[]);
        ref_arg(&mut p, 1, 0, 2, 0);
        let (mut vm, o) = machine(
            p,
            if vars == 1 {
                vec![Value::Int(3)]
            } else {
                vec![]
            },
        );
        vm.start(o, 0, vec![Value::Int(3)]).unwrap();
        returns(&mut vm, Value::Int(9));
        assert_eq!(vm.frame_depth(), 0);
    }
}

#[test]
fn reference_parameters_keep_identity_through_nested_forwarding_and_aliases() {
    let caller = fun(
        0,
        1,
        0,
        255,
        &[(0, 0, 0x39), (1, 1, 0x07), (0, 0, 0x33), (0, 0, 0x37)],
    );
    let middle = fun(
        1,
        1,
        0,
        1,
        &[(0, 0, 0x39), (0, 0, 0x39), (2, 2, 0x07), (0, 0, 0x33)],
    );
    let leaf = fun(
        2,
        2,
        0,
        1,
        &[
            (11, 0, 0x3b),
            (0, 0, 0x4d),
            (0, 0, 0x33),
            (1, 0, 0x39),
            (1, 0, 0x22),
        ],
    );
    let mut p = image(&[caller, middle, leaf], 0, &[]);
    ref_arg(&mut p, 1, 0, 2, 0);
    ref_arg(&mut p, 2, 0, 2, 0);
    ref_arg(&mut p, 2, 1, 2, 0);
    let (mut vm, o) = machine(p, vec![]);
    vm.start(o, 0, vec![Value::Int(3)]).unwrap();
    vm.run(8).unwrap(); // caller/middle args plus two forwarded aliases
    let ids: Vec<_> = vm
        .stack()
        .unwrap()
        .iter()
        .filter_map(|e| match e {
            rml32::StackSnapshot::Reference { target_id, .. } => Some(*target_id),
            _ => None,
        })
        .collect();
    assert!(ids.len() >= 3);
    assert!(ids.iter().all(|id| *id == ids[0]));
    returns(&mut vm, Value::Int(12));
}

#[test]
fn reference_entry_is_rejected_without_replacing_previous_execution_state() {
    let mut p = image(
        &[
            fun(0, 0, 0, 2, &[(7, 0, 0x3b)]),
            fun(1, 1, 0, 255, &[(0, 0, 0x37)]),
        ],
        0,
        &[],
    );
    ref_arg(&mut p, 1, 0, 255, 0);
    let (mut vm, o) = machine(p, vec![]);
    vm.start(o, 0, vec![]).unwrap();
    returns(&mut vm, Value::Int(7));
    let before = vm.stack().unwrap();
    assert_eq!(
        vm.start(o, 1, vec![Value::Int(3)]).unwrap_err().kind,
        VmErrorKind::UnsupportedFeature
    );
    assert_eq!(vm.stack().unwrap(), before);
    returns(&mut vm, Value::Int(7));
}

#[test]
fn reference_function_preparation_requires_check_args_and_known_descriptors() {
    let base = image(&[fun(0, 1, 0, 255, &[])], 0, &[]);
    for case in 0..4 {
        let mut p = base.clone();
        ref_arg(&mut p, 0, 0, 255, 0);
        match case {
            0 => p.functions[0].code.nodes[0].2 = 0x1b,
            1 => p.functions[0].arguments[0].flag = 3,
            2 => p.functions[0].arguments[0].attrib = 16,
            _ => p.functions[0].arguments[0].attrib = 2,
        }
        assert_eq!(
            prepare_program(p).unwrap_err().kind,
            VmErrorKind::UnsupportedFeature
        );
    }
}

#[test]
fn reference_binding_errors_leave_callers_stack_and_frame_intact() {
    for case in 0..4 {
        let nodes = match case {
            0 => vec![(8, 0, 0x3b), (1, 1, 0x07)], // value to reference
            1 => vec![(0, 0, 0x39), (1, 1, 0x07)], // reference to mixed value
            2 => vec![(0, 0, 0x39), (1, 0, 0x07)], // wrong count
            _ => vec![(8, 0, 0x3b), (-1, 0, 0x39), (1, 1, 0x07)], // eval-slot owner
        };
        let mut p = image(&[fun(0, 1, 0, 255, &nodes), fun(1, 1, 0, 255, &[])], 0, &[]);
        if case != 1 {
            ref_arg(&mut p, 1, 0, 255, 0);
        }
        let (mut vm, o) = machine(p, vec![]);
        vm.start(o, 0, vec![Value::Int(4)]).unwrap();
        vm.run(2 + nodes.len() - 1).unwrap();
        let before = vm.stack().unwrap();
        let error = vm.run(1).unwrap_err();
        assert_eq!(error.function, Some(0));
        assert_eq!(error.pc, Some(2 + nodes.len() - 1));
        assert_eq!(
            error.kind,
            if case == 2 {
                VmErrorKind::ArgumentCount
            } else {
                VmErrorKind::UnsupportedFeature
            }
        );
        assert_eq!(vm.stack().unwrap(), before);
        assert_eq!(vm.frame_depth(), 1);
    }
}

#[test]
fn reference_numeric_types_are_exact_while_value_parameters_coerce() {
    for is_ref in [false, true] {
        let source = if is_ref { 0x39 } else { 0x37 };
        let mut p = image(
            &[
                fun(0, 1, 0, 255, &[(0, 0, source), (1, 1, 0x07)]),
                fun(1, 1, 0, 255, &[(0, 0, 0x37)]),
            ],
            0,
            &[],
        );
        p.functions[1].arguments[0].r#type = 3;
        if is_ref {
            ref_arg(&mut p, 1, 0, 3, 0);
        }
        let (mut vm, o) = machine(p, vec![]);
        vm.start(o, 0, vec![Value::Int(4)]).unwrap();
        if is_ref {
            let e = vm.run(100).unwrap_err();
            assert_eq!(
                (e.function, e.pc, e.kind),
                (Some(1), Some(0), VmErrorKind::UnsupportedType)
            );
            assert_eq!(
                vm.stack().unwrap()[0],
                rml32::StackSnapshot::Value(Value::Int(4))
            );
        } else {
            returns(&mut vm, real(4.0));
        }
    }
}

#[test]
fn argument_check_failure_does_not_commit_earlier_numeric_conversion() {
    let mut p = image(&[fun(0, 2, 0, 255, &[])], 0, &[]);
    p.functions[0].arguments[0].r#type = 3;
    p.functions[0].arguments[1].r#type = 2;
    let (mut vm, o) = machine(p, vec![]);
    vm.start(o, 0, vec![Value::Int(7), Value::bytes(b"bad")])
        .unwrap();
    let before = vm.stack().unwrap();
    assert_eq!(vm.run(1).unwrap_err().kind, VmErrorKind::UnsupportedType);
    assert_eq!(vm.stack().unwrap(), before);
}

#[test]
fn typed_reference_dead_object_normalization_is_nullable_and_batch_atomic() {
    for (ty, nullable, later_bad, normalized, success) in [
        (10, 0, false, false, false),
        (10, 8, false, true, true),
        (10, 8, true, false, false),
        (255, 0, false, false, true),
    ] {
        let mut p = image(
            &[
                fun(0, 0, 0, 255, &[(0, 0, 0x3b), (0, 0, 0x3e), (1, 2, 0x07)]),
                fun(1, 2, 0, 255, &[]),
            ],
            1,
            &[],
        );
        ref_arg(&mut p, 1, 0, ty, nullable);
        p.functions[1].arguments[1].r#type = if later_bad { 4 } else { 2 };
        let (mut vm, o) = machine(p, vec![Value::Undefined]);
        let pid = vm.objects().get(o).unwrap().program();
        let dead = vm
            .objects_mut()
            .create(pid, vec![Value::Undefined])
            .unwrap();
        // Recreate the caller with a live Object so it is not normalized by start.
        let caller = vm
            .objects_mut()
            .create(pid, vec![Value::Object(dead)])
            .unwrap();
        vm.start(caller, 0, vec![]).unwrap();
        vm.run(5).unwrap(); // paused after 07, before callee 0f
        assert_eq!(vm.frame_depth(), 2);
        vm.objects_mut().remove(dead).unwrap();
        let before = vm.stack().unwrap();
        let result = vm.run(1);
        if success {
            assert_eq!(result.unwrap(), RunOutcome::BudgetExhausted);
        } else {
            let e = result.unwrap_err();
            assert_eq!((e.function, e.pc), (Some(1), Some(0)));
            assert_eq!(vm.stack().unwrap(), before);
        }
        assert_eq!(
            object_values(&vm, caller)[0],
            if normalized {
                Value::Undefined
            } else {
                Value::Object(dead)
            }
        );
    }
}

#[test]
fn aliased_nullable_parameters_normalize_one_dead_cell_once() {
    let mut p = image(
        &[
            fun(0, 0, 0, 255, &[(0, 0, 0x3e), (0, 0, 0x3e), (1, 2, 0x07)]),
            fun(1, 2, 0, 255, &[]),
        ],
        1,
        &[],
    );
    ref_arg(&mut p, 1, 0, 10, 8);
    ref_arg(&mut p, 1, 1, 10, 8);
    let (mut vm, o) = machine(p, vec![Value::Undefined]);
    let pid = vm.objects().get(o).unwrap().program();
    let caller = vm
        .objects_mut()
        .create(pid, vec![Value::Object(o)])
        .unwrap();
    vm.start(caller, 0, vec![]).unwrap();
    vm.run(5).unwrap();
    vm.objects_mut().remove(o).unwrap();
    vm.run(1).unwrap();
    assert_eq!(object_values(&vm, caller), vec![Value::Undefined]);
}

#[test]
fn forwarded_member_reference_keeps_scalar_write_restriction() {
    for container in [
        Value::array(vec![Value::Int(5)]).unwrap(),
        Value::mapping(vec![(Value::Int(0), Value::Int(5))]).unwrap(),
    ] {
        let mut p = image(
            &[
                fun(
                    0,
                    0,
                    0,
                    255,
                    &[(0, 0, 0x3e), (0, 0, 0x3b), (0, 0, 0x44), (1, 1, 0x07)],
                ),
                fun(1, 1, 0, 255, &[(0, 0, 0x39), (2, 1, 0x07)]),
                fun(2, 1, 0, 255, &[(0, 0, 0x16), (0, 0, 0x4d)]),
            ],
            1,
            &[],
        );
        ref_arg(&mut p, 1, 0, 255, 0);
        ref_arg(&mut p, 2, 0, 255, 0);
        let (mut vm, o) = machine(p, vec![container.clone()]);
        vm.start(o, 0, vec![]).unwrap();
        let e = vm.run(100).unwrap_err();
        assert_eq!(
            (e.function, e.pc, e.kind),
            (Some(2), Some(3), VmErrorKind::UnsupportedType)
        );
        match container {
            Value::Array(a) => assert_eq!(a.elements().unwrap(), vec![Value::Int(5)]),
            Value::Mapping(m) => assert_eq!(m.get(&Value::Int(0)).unwrap(), Some(Value::Int(5))),
            _ => unreachable!(),
        }
    }
}

#[test]
fn reference_assignment_does_not_permanently_enforce_entry_type() {
    let mut p = image(
        &[
            fun(0, 0, 0, 255, &[(0, 0, 0x3e), (1, 1, 0x07)]),
            fun(1, 1, 0, 255, &[(0, 0, 0x42), (0, 0, 0x4d)]),
        ],
        1,
        &[],
    );
    ref_arg(&mut p, 1, 0, 2, 0);
    let (mut vm, o) = machine(p, vec![Value::Int(5)]);
    vm.start(o, 0, vec![]).unwrap();
    returns(&mut vm, Value::Undefined);
    assert_eq!(object_values(&vm, o), vec![Value::Undefined]);
}

#[test]
fn body_error_keeps_committed_writes_and_new_start_releases_frames() {
    let mut p = image(
        &[
            fun(0, 0, 0, 255, &[(0, 0, 0x3e), (1, 1, 0x07)]),
            fun(
                1,
                1,
                0,
                255,
                &[
                    (9, 0, 0x3b),
                    (0, 0, 0x4d),
                    (0, 0, 0x33),
                    (1, 0, 0x3b),
                    (0, 0, 0x3b),
                    (0, 0, 0x19),
                ],
            ),
            fun(2, 0, 0, 2, &[(3, 0, 0x3b)]),
        ],
        1,
        &[],
    );
    ref_arg(&mut p, 1, 0, 2, 0);
    let (mut vm, o) = machine(p, vec![Value::Int(5)]);
    vm.start(o, 0, vec![]).unwrap();
    assert!(vm.run(100).is_err());
    assert_eq!(object_values(&vm, o), vec![Value::Int(9)]);
    assert_eq!(vm.frame_depth(), 2);
    vm.start(o, 2, vec![]).unwrap();
    returns(&mut vm, Value::Int(3));
    assert_eq!(vm.frame_depth(), 0);
}

#[test]
fn reference_return_is_rejected_before_callee_storage_is_removed() {
    let mut p = image(
        &[
            fun(0, 1, 0, 255, &[(0, 0, 0x39), (1, 1, 0x07)]),
            fun(1, 1, 0, 255, &[(0, 0, 0x39)]),
        ],
        0,
        &[],
    );
    ref_arg(&mut p, 1, 0, 255, 0);
    let (mut vm, o) = machine(p, vec![]);
    vm.start(o, 0, vec![Value::Int(3)]).unwrap();
    assert_eq!(
        vm.run(100).unwrap_err().kind,
        VmErrorKind::UnsupportedFeature
    );
    assert_eq!(vm.frame_depth(), 2);
    assert_eq!(vm.stack().unwrap().len(), 3);
}

#[test]
fn reference_recursion_hits_frame_limit_without_rebinding_or_target_change() {
    let mut p = image(
        &[
            fun(0, 0, 0, 255, &[(0, 0, 0x3e), (1, 1, 0x07)]),
            fun(1, 1, 0, 255, &[(0, 0, 0x39), (1, 1, 0x07)]),
        ],
        1,
        &[],
    );
    ref_arg(&mut p, 1, 0, 2, 0);
    let (vm, o) = machine(p, vec![Value::Int(5)]);
    let mut vm = Vm::with_limits(
        vm.into_objects(),
        VmLimits {
            max_frames: 3,
            ..VmLimits::default()
        },
    );
    vm.start(o, 0, vec![]).unwrap();
    let e = vm.run(100).unwrap_err();
    assert_eq!(e.kind, VmErrorKind::Limit);
    assert_eq!(vm.frame_depth(), 3);
    assert_eq!(object_values(&vm, o), vec![Value::Int(5)]);
}

#[test]
fn reference_reads_normalize_dead_source_cells_after_successful_read() {
    for read_op in [0x37, 0x14] {
        let caller = if read_op == 0x37 {
            fun(0, 1, 0, 255, &[(0, 0, 0x39), (1, 1, 0x07)])
        } else {
            fun(0, 1, 0, 255, &[(0, 0, 0x3e), (0, 0, 0x14)])
        };
        let mut p = image(&[caller, fun(1, 1, 0, 255, &[(0, 0, 0x37)])], 1, &[]);
        ref_arg(&mut p, 1, 0, 255, 0);
        let (mut vm, o) = machine(p, vec![Value::Undefined]);
        let pid = vm.objects().get(o).unwrap().program();
        let caller = vm
            .objects_mut()
            .create(pid, vec![Value::Object(o)])
            .unwrap();
        vm.start(caller, 0, vec![Value::Object(o)]).unwrap();
        vm.run(if read_op == 0x37 { 6 } else { 3 }).unwrap();
        vm.objects_mut().remove(o).unwrap();
        vm.run(1).unwrap();
        if read_op == 0x37 {
            assert_eq!(
                vm.stack().unwrap()[0],
                rml32::StackSnapshot::Value(Value::Undefined)
            );
        } else {
            assert_eq!(object_values(&vm, caller), vec![Value::Undefined]);
        }
        returns(&mut vm, Value::Undefined);
    }
}

#[test]
fn by_value_scalar_is_isolated_but_container_identity_remains_shared() {
    let caller = fun(
        0,
        1,
        0,
        255,
        &[(0, 0, 0x37), (1, 1, 0x07), (0, 0, 0x33), (0, 0, 0x37)],
    );
    let callee = fun(1, 1, 0, 1, &[(9, 0, 0x3b), (0, 0, 0x4d), (0, 0, 0x33)]);
    let (mut vm, o) = machine(image(&[caller, callee], 0, &[]), vec![]);
    vm.start(o, 0, vec![Value::Int(5)]).unwrap();
    returns(&mut vm, Value::Int(5));
    let caller = fun(
        0,
        1,
        0,
        255,
        &[(0, 0, 0x37), (1, 1, 0x07), (0, 0, 0x33), (0, 0, 0x37)],
    );
    let callee = fun(
        1,
        1,
        0,
        1,
        &[
            (0, 0, 0x39),
            (0, 0, 0x3b),
            (0, 0, 0x44),
            (9, 0, 0x3b),
            (0, 0, 0x04),
            (0, 0, 0x33),
        ],
    );
    let (mut vm, o) = machine(image(&[caller, callee], 0, &[]), vec![]);
    let a = Value::array(vec![Value::Int(5)]).unwrap();
    vm.start(o, 0, vec![a.clone()]).unwrap();
    returns(&mut vm, a.clone());
    let Value::Array(a) = a else { unreachable!() };
    assert_eq!(a.elements().unwrap(), vec![Value::Int(9)]);
}

#[test]
fn reference_parameter_cleanup_releases_member_and_temporary_quota() {
    for member in [true, false] {
        let mut nodes = vec![];
        for key in 0..2 {
            if member {
                nodes.extend([(0, 0, 0x3e), (key, 0, 0x3b), (0, 0, 0x44)]);
            } else {
                nodes.extend([(0, 0, 0x16), (0, 0, 0x13)]);
            }
            nodes.extend([(1, 1, 0x07), (0, 0, 0x33)]);
        }
        nodes.push((7, 0, 0x3b));
        let mut p = image(&[fun(0, 0, 0, 2, &nodes), fun(1, 1, 0, 1, &[])], 1, &[]);
        ref_arg(&mut p, 1, 0, 255, 0);
        let (vm, o) = machine(p, vec![Value::mapping(vec![]).unwrap()]);
        let mut vm = Vm::with_limits(
            vm.into_objects(),
            VmLimits {
                max_reference_cells: 1,
                ..VmLimits::default()
            },
        );
        vm.start(o, 0, vec![]).unwrap();
        returns(&mut vm, Value::Int(7));
    }
}

#[test]
fn deleted_reference_owner_is_rechecked_before_argument_entry() {
    let mut p = image(
        &[
            fun(0, 0, 0, 255, &[(0, 0, 0x3e), (1, 1, 0x07)]),
            fun(1, 1, 0, 255, &[]),
        ],
        1,
        &[],
    );
    ref_arg(&mut p, 1, 0, 255, 0);
    let (mut vm, o) = machine(p, vec![Value::Int(5)]);
    vm.start(o, 0, vec![]).unwrap();
    vm.run(4).unwrap();
    vm.objects_mut().remove(o).unwrap();
    let before = vm.stack().unwrap();
    let e = vm.run(1).unwrap_err();
    assert_eq!(
        (e.function, e.pc, e.kind),
        (Some(1), Some(0), VmErrorKind::InvalidObject)
    );
    assert_eq!(vm.stack().unwrap(), before);
}

#[test]
fn declared_local_reference_survives_callee_return_without_rebinding() {
    let caller = fun(
        0,
        0,
        1,
        255,
        &[
            (5, 0, 0x3b),
            (-1, 0, 0x4d),
            (0, 0, 0x33),
            (-1, 0, 0x39),
            (1, 1, 0x07),
            (0, 0, 0x33),
            (-1, 0, 0x37),
        ],
    );
    let mut p = image(
        &[caller, fun(1, 1, 0, 1, &[(0, 0, 0x39), (2, 0, 0x22)])],
        0,
        &[],
    );
    ref_arg(&mut p, 1, 0, 2, 0);
    let (mut vm, o) = machine(p, vec![]);
    vm.start(o, 0, vec![]).unwrap();
    returns(&mut vm, Value::Int(7));
    assert_eq!(vm.frame_depth(), 0);
}

#[test]
fn reference_read_stack_limit_does_not_normalize_dead_source() {
    let mut p = image(
        &[
            fun(0, 0, 0, 255, &[(0, 0, 0x3e), (1, 1, 0x07)]),
            fun(1, 1, 0, 255, &[(0, 0, 0x37)]),
        ],
        1,
        &[],
    );
    ref_arg(&mut p, 1, 0, 255, 0);
    let (vm, o) = machine(p, vec![Value::Undefined]);
    let mut vm = Vm::with_limits(
        vm.into_objects(),
        VmLimits {
            max_stack: 1,
            ..VmLimits::default()
        },
    );
    let pid = vm.objects().get(o).unwrap().program();
    let caller = vm
        .objects_mut()
        .create(pid, vec![Value::Object(o)])
        .unwrap();
    vm.start(caller, 0, vec![]).unwrap();
    vm.run(6).unwrap();
    vm.objects_mut().remove(o).unwrap();
    let before = vm.stack().unwrap();
    assert_eq!(vm.run(1).unwrap_err().kind, VmErrorKind::Limit);
    assert_eq!(vm.stack().unwrap(), before);
    assert_eq!(object_values(&vm, caller), vec![Value::Object(o)]);
}
