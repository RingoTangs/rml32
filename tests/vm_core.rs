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
    assert!(vm.stack().is_empty());
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
    assert_eq!(vm.objects().get(o).unwrap().variables(), &[Value::Int(3)]);
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
    assert_eq!(vm.objects().get(b).unwrap().variables(), &[Value::Int(9)]);
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
    assert_eq!(vm.stack(), &[Value::Int(9)]);
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
    for (opcode, b) in [(6, 0), (7, -128), (0x16, 0), (0x0e, 0)] {
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
    real.functions[0].return_type = 3;
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
    assert_eq!(supported.len(), 28);
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
fn reference_const_atomic_and_variadic_prototypes_are_rejected() {
    let base = image(&[fun(0, 1, 0, 255, &[(0, 0, 0x37)])], 0, &[]);
    let mut reference = base.clone();
    reference.functions[0].arguments[0].flag = 1;
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
