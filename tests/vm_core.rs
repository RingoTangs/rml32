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
    assert_eq!(
        vm.start(o, 0, vec![]).unwrap_err().kind,
        VmErrorKind::ArgumentCount
    );
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
    for (opcode, b) in [(6, 0), (7, -128), (0x46, 0), (0x11, 0)] {
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
        VmErrorKind::InvalidProgram
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
    assert_eq!(supported.len(), 73);
    assert_eq!(ByteString::new(b"\xff").bytes, b"\xff");
}

// Stage 2Y: synthesized range queries only; the retained business corpus stays read-only.
#[test]
fn ranges_arrays_have_independent_top_slots_and_shared_nested_values() {
    let nested = Value::array(vec![Value::Int(8)]).unwrap();
    let source = Value::array(vec![Value::Int(1), nested.clone()]).unwrap();
    let result = range_result(source.clone(), Value::Int(0), Value::Int(1), 0).unwrap();
    assert_ne!(result, source);
    let Value::Array(a) = &result else { panic!() };
    assert_eq!(a.elements().unwrap(), vec![Value::Int(1), nested.clone()]);
    // Assign through the result's top-level member, then a shared nested member.
    let mutate = fun(
        0,
        0,
        0,
        255,
        &[
            (9, 0, 0x3b),
            (0, 0, 0x3e),
            (0, 0, 0x3b),
            (0, 0, 0x44),
            (0, 0, 0x03),
        ],
    );
    let (mut vm, o) = machine(image(&[mutate], 1, &[]), vec![result.clone()]);
    vm.start(o, 0, vec![]).unwrap();
    returns(&mut vm, Value::Int(9));
    assert_eq!(a.elements().unwrap()[0], Value::Int(9));
    let Value::Array(source) = &source else {
        panic!()
    };
    assert_eq!(source.elements().unwrap()[0], Value::Int(1));
    let (mut vm, o) = machine(
        image(
            &[fun(
                0,
                0,
                0,
                255,
                &[
                    (7, 0, 0x3b),
                    (0, 0, 0x3e),
                    (0, 0, 0x3b),
                    (0, 0, 0x44),
                    (0, 0, 0x03),
                ],
            )],
            1,
            &[],
        ),
        vec![nested],
    );
    vm.start(o, 0, vec![]).unwrap();
    returns(&mut vm, Value::Int(7));
    let Value::Array(shared) = &source.elements().unwrap()[1] else {
        panic!()
    };
    assert_eq!(shared.elements().unwrap(), vec![Value::Int(7)]);
    let Value::Array(shared) = &a.elements().unwrap()[1] else {
        panic!()
    };
    assert_eq!(shared.elements().unwrap(), vec![Value::Int(7)]);
}
#[test]
fn ranges_array_normalization_updates_only_selected_direct_dead_objects() {
    let (mut vm, o) = machine(image(&[range_function(0, 0)], 0, &[]), vec![]);
    let program = vm.objects().get(o).unwrap().program();
    let dead = vm.objects_mut().create(program, vec![]).unwrap();
    let nested = Value::array(vec![Value::Object(dead)]).unwrap();
    let source = Value::array(vec![
        Value::Object(dead),
        Value::Object(dead),
        nested.clone(),
    ])
    .unwrap();
    vm.objects_mut().remove(dead).unwrap();
    vm.start(o, 0, vec![source.clone(), Value::Int(1), Value::Int(2)])
        .unwrap();
    vm.run(5).unwrap();
    vm.run(0).unwrap();
    let Value::Array(a) = &source else { panic!() };
    assert_eq!(a.elements().unwrap()[1], Value::Object(dead));
    assert_eq!(
        returned_array(&mut vm),
        vec![Value::Undefined, nested.clone()]
    );
    assert_eq!(
        a.elements().unwrap(),
        vec![Value::Object(dead), Value::Undefined, nested.clone()]
    );
    let Value::Array(nested) = nested else {
        panic!()
    };
    assert_eq!(nested.elements().unwrap(), vec![Value::Object(dead)]);
}
#[test]
fn ranges_array_depth_item_limits_and_shared_subgraphs_are_preserved() {
    let shared = Value::array(vec![Value::Int(1)]).unwrap();
    let source = Value::array(vec![shared.clone(); 65536]).unwrap();
    let Value::Array(result) = range_result(source, Value::Int(0), Value::Int(65535), 0).unwrap()
    else {
        panic!()
    };
    assert_eq!(result.elements().unwrap(), vec![shared; 65536]);
    let mut deep = Value::Int(1);
    for _ in 0..64 {
        deep = Value::array(vec![deep]).unwrap();
    }
    let Value::Array(result) = range_result(deep, Value::Int(0), Value::Int(0), 0).unwrap() else {
        panic!()
    };
    assert_eq!(result.depth().unwrap(), 64);
}
#[test]
fn ranges_stack_underflow_and_byte_limit_preserve_operands_and_diagnostics() {
    for count in 0..3 {
        let nodes = vec![(0, 0, 0x3b); count];
        let mut f = fun(0, 0, 0, 255, &nodes);
        f.nodes.insert(f.nodes.len() - 1, (0, 0, 0x45));
        let (mut vm, o) = machine(image(&[f], 0, &[]), vec![]);
        vm.start(o, 0, vec![]).unwrap();
        vm.run(2 + count).unwrap();
        let before = vm.stack().unwrap();
        assert_eq!(vm.run(1).unwrap_err().kind, VmErrorKind::StackUnderflow);
        assert_eq!(vm.stack().unwrap(), before);
    }
    let (mut vm, o) = machine(image(&[range_function(0, 0)], 0, &[]), vec![]);
    vm.start(
        o,
        0,
        vec![
            Value::bytes(vec![b'x'; 65537]),
            Value::Int(0),
            Value::Int(65536),
        ],
    )
    .unwrap();
    vm.run(5).unwrap();
    let before = vm.stack().unwrap();
    let e = vm.run(1).unwrap_err();
    assert_eq!(e.kind, VmErrorKind::Limit);
    assert_eq!(e.object, Some(o));
    assert_eq!(e.function, Some(0));
    assert_eq!(e.pc, Some(5));
    assert_eq!(vm.stack().unwrap(), before);
}
#[test]
fn ranges_errors_inside_catch_remain_fatal_and_restart_keeps_numeric_routing() {
    let bad = catch_fun(
        0,
        0,
        0,
        255,
        &[(0, 0, 0x16), (0, 0, 0x42), (0, 0, 0x3b), (0, 0, 0x45)],
    );
    let good = catch_fun(1, 0, 0, 255, &[(1, 0, 0x3b), (0, 0, 0x3b), (0, 0, 0x19)]);
    let (mut vm, o) = machine(image(&[bad, good], 0, &[]), vec![]);
    vm.start(o, 0, vec![]).unwrap();
    assert_eq!(vm.run(100).unwrap_err().kind, VmErrorKind::UnsupportedType);
    vm.start(o, 1, vec![]).unwrap();
    returns(&mut vm, Value::Int(1));
}
#[test]
fn ranges_inherited_body_uses_parent_pool_and_restores_child() {
    let (parent, child) = inherited_pair(
        &[(0, 0, 0x40), (1, 0, 0x3b), (3, 0, 0x3b), (0, 0, 0x45)],
        &[(0, 0, 0x07), (0, 0, 0x33), (0, 0, 0x40)],
    );
    let (mut vm, o) = linked_vm(&linked_catalog(parent, child));
    vm.start(o, 0, vec![]).unwrap();
    returns(&mut vm, Value::bytes(b"are"));
    vm.start(o, 1, vec![]).unwrap();
    returns(&mut vm, Value::bytes(b"child"));
}
#[test]
fn ranges_cross_object_result_and_return_context_survive_single_steps() {
    let target = other_target(
        image(&[range_function(1, 127)], 0, &[]),
        vec![method(b"slice", 0)],
    );
    let (mut vm, caller, target) = other_machine(
        image(&[other_wrapper(3, 0)], 0, &[]),
        target,
        vec![],
        vec![],
    );
    vm.start(
        caller,
        0,
        vec![
            Value::Object(target),
            Value::bytes(b"slice"),
            Value::bytes(b"abcdef"),
            Value::Int(2),
            Value::Int(2),
        ],
    )
    .unwrap();
    loop {
        if let RunOutcome::Finished(result) = vm.run(1).unwrap() {
            assert_eq!(result, Completion::Returned(Value::bytes(b"cde")));
            break;
        }
    }
    assert_eq!(vm.frame_depth(), 0);
    assert!(vm.objects().get(caller).is_ok());
    assert!(vm.objects().get(target).is_ok());
}
#[test]
fn ranges_lifecycle_helper_rejection_cleans_initialization_and_construction() {
    let helper = fun(
        2,
        0,
        0,
        1,
        &[
            (0, 0, 0x40),
            (0, 0, 0x3b),
            (0, 0, 0x3b),
            (0, 0, 0x45),
            (0, 0, 0x33),
        ],
    );
    let mut p = image(
        &[
            fun(0, 0, 0, 1, &[(2, 0, 0x07)]),
            fun(1, 0, 0, 1, &[]),
            helper,
        ],
        0,
        &[b"x"],
    );
    set_init(&mut p, &[(2, 0, 0x07), (0, 0, 0x48)]);
    let (mut vm, id) = init_vm(&init_catalog(p.clone()), "root");
    let o = vm.begin_initialize(id).unwrap();
    let e = vm.run(100).unwrap_err();
    assert_eq!(e.kind, VmErrorKind::UnsupportedFeature);
    assert_eq!(e.function, Some(2));
    assert!(vm.objects().get(o).is_err());
    assert!(vm.stack().unwrap().is_empty());
    lifecycle_name(&mut p, 0, b"create");
    lifecycle_name(&mut p, 1, b"destruct");
    set_init(&mut p, &[(0, 0, 0x48)]);
    let c = single_construct_catalog(
        p,
        rml32::LifecycleBinding {
            create: Some(0),
            destruct: Some(1),
        },
    );
    let (mut vm, id) = construct_vm(&c, "root");
    let o = vm.begin_construct(id, None).unwrap();
    let e = vm.run(100).unwrap_err();
    assert_eq!(e.kind, VmErrorKind::UnsupportedFeature);
    assert_eq!(
        e.lifecycle.as_ref().unwrap().phase,
        rml32::LifecyclePhase::Create
    );
    assert!(vm.rollback_errors().is_empty());
    assert!(vm.objects().get(o).is_err());
    assert!(vm.stack().unwrap().is_empty());
}
#[test]
fn ranges_rollback_helper_rejection_preserves_primary_error_and_cleans_task() {
    let mut p = image(
        &[
            fun(0, 0, 0, 1, &[(0, 0, 0x33)]),
            fun(1, 0, 0, 1, &[(2, 0, 0x07)]),
            fun(
                2,
                0,
                0,
                1,
                &[(0, 0, 0x40), (0, 0, 0x3b), (0, 0, 0x3b), (0, 0, 0x45)],
            ),
        ],
        0,
        &[b"x"],
    );
    lifecycle_name(&mut p, 0, b"create");
    lifecycle_name(&mut p, 1, b"destruct");
    set_init(&mut p, &[(0, 0, 0x48)]);
    let c = single_construct_catalog(
        p,
        rml32::LifecycleBinding {
            create: Some(0),
            destruct: Some(1),
        },
    );
    let (mut vm, id) = construct_vm(&c, "root");
    let o = vm.begin_construct(id, None).unwrap();
    assert_eq!(vm.run(100).unwrap_err().kind, VmErrorKind::StackUnderflow);
    assert_eq!(vm.rollback_errors().len(), 1);
    let e = &vm.rollback_errors()[0];
    assert_eq!(e.kind, VmErrorKind::UnsupportedFeature);
    assert_eq!(e.function, Some(2));
    assert_eq!(
        e.lifecycle.as_ref().unwrap().phase,
        rml32::LifecyclePhase::RollbackDestruct
    );
    assert!(vm.objects().get(o).is_err());
    assert!(vm.stack().unwrap().is_empty());
}
fn range_function(mode: i16, par8: i8) -> Fun {
    fun(
        0,
        3,
        0,
        255,
        &[(0, 0, 0x37), (1, 0, 0x37), (2, 0, 0x37), (mode, par8, 0x45)],
    )
}
fn range_result(
    source: Value,
    begin: Value,
    end: Value,
    mode: i16,
) -> Result<Value, rml32::VmError> {
    let (mut vm, o) = machine(image(&[range_function(mode, 0)], 0, &[]), vec![]);
    vm.start(o, 0, vec![source, begin, end]).unwrap();
    match vm.run(100)? {
        RunOutcome::Finished(Completion::Returned(v)) => Ok(v),
        other => panic!("unexpected {other:?}"),
    }
}
#[test]
fn ranges_inclusive_modes_clip_both_types_and_preserve_order() {
    for (mode, begin, end, expected) in [
        (0, 1, 3, vec![1, 2, 3]),
        (1, 1, 1, vec![1, 2, 3, 4]),
        (2, 3, 3, vec![2, 3]),
        (3, 3, 1, vec![2, 3, 4]),
        (0, -8, 99, vec![0, 1, 2, 3, 4]),
        (0, 4, 1, vec![]),
        (0, 0, -1, vec![]),
        (0, 99, 100, vec![]),
        (1, 0, 0, vec![0, 1, 2, 3, 4]),
        (1, 0, -5, vec![0, 1, 2, 3, 4]),
        (2, 0, 4, vec![4]),
        (3, -5, 0, vec![4]),
    ] {
        let begin = Value::Int(begin);
        let end = Value::Int(end);
        let bytes = range_result(
            Value::bytes([10, 11, 12, 13, 14]),
            begin.clone(),
            end.clone(),
            mode,
        )
        .unwrap();
        assert_eq!(
            bytes,
            Value::bytes(expected.iter().map(|i| i + 10).collect::<Vec<_>>())
        );
        let values = (0..5).map(Value::Int).collect();
        let Value::Array(result) =
            range_result(Value::array(values).unwrap(), begin, end, mode).unwrap()
        else {
            panic!()
        };
        assert_eq!(
            result.elements().unwrap(),
            expected
                .into_iter()
                .map(|i| Value::Int(i.into()))
                .collect::<Vec<_>>()
        );
    }
}
#[test]
fn ranges_empty_single_and_extreme_endpoints_are_safe() {
    for mode in 0..=3 {
        for (begin, end) in [(0, 0), (i32::MIN, i32::MAX), (i32::MAX, i32::MIN)] {
            assert_eq!(
                range_result(Value::bytes([]), Value::Int(begin), Value::Int(end), mode).unwrap(),
                Value::bytes([])
            );
            let source = Value::array(vec![]).unwrap();
            let result =
                range_result(source.clone(), Value::Int(begin), Value::Int(end), mode).unwrap();
            assert_ne!(source, result); // Explicit Rust empty-array identity policy.
            let Value::Array(result) = result else {
                panic!()
            };
            assert!(result.elements().unwrap().is_empty());
        }
        assert_eq!(
            range_result(Value::bytes(b"x"), Value::Int(0), Value::Int(0), mode).unwrap(),
            Value::bytes(b"x")
        );
    }
    assert_eq!(
        range_result(
            Value::bytes(b"xy"),
            Value::Int(i32::MIN),
            Value::Int(i32::MAX),
            0
        )
        .unwrap(),
        Value::bytes(b"xy")
    );
    assert_eq!(
        range_result(
            Value::bytes(b"xy"),
            Value::Int(i32::MAX),
            Value::Int(i32::MIN),
            3
        )
        .unwrap(),
        Value::bytes(b"xy")
    );
}
#[test]
fn ranges_real_endpoints_use_legacy_truncation_and_indefinite_min() {
    for (begin, end, mode, expected) in [
        (real(1.9), real(2.9), 0, b"bc".as_slice()),
        (real(-0.0), real(0.0), 0, b"a"),
        (real(f32::NAN), Value::Int(2), 0, b"abc"),
        (Value::Int(0), real(f32::INFINITY), 0, b""),
        (Value::Int(0), real(f32::NEG_INFINITY), 1, b"abcd"),
        (real(2147483648.0), Value::Int(1), 0, b"ab"),
        (real(2147483520.0), Value::Int(3), 0, b""),
        (Value::Int(1), real(1.9), 1, b"bcd"),
    ] {
        assert_eq!(
            range_result(Value::bytes(b"abcd"), begin, end, mode).unwrap(),
            Value::bytes(expected)
        );
    }
}
#[test]
fn ranges_bytes_apply_strncpy_padding_only_inside_selected_region() {
    for (begin, end, expected) in [
        (0, 3, vec![0x41, 0, 0, 0]),
        (2, 3, vec![0x42, 0xff]),
        (1, 2, vec![0, 0]),
    ] {
        assert_eq!(
            range_result(
                Value::bytes([0x41, 0, 0x42, 0xff]),
                Value::Int(begin),
                Value::Int(end),
                0
            )
            .unwrap(),
            Value::bytes(expected)
        );
    }
}
#[test]
fn ranges_bytes_result_limit_is_independent_of_source_length() {
    let bytes = Value::bytes(vec![b'x'; 65537]);
    let result = range_result(bytes.clone(), Value::Int(0), Value::Int(65535), 0).unwrap();
    assert_eq!(result, Value::bytes(vec![b'x'; 65536]));
    assert_eq!(
        range_result(bytes.clone(), Value::Int(0), Value::Int(65536), 0)
            .unwrap_err()
            .kind,
        VmErrorKind::Limit
    );
    assert_eq!(
        range_result(bytes, Value::Int(65536), Value::Int(65536), 0).unwrap(),
        Value::bytes(b"x")
    );
}
#[test]
fn ranges_types_and_reference_operands_remain_explicit_errors() {
    for source in [
        Value::Int(1),
        Value::Undefined,
        Value::Void,
        Value::mapping(vec![]).unwrap(),
    ] {
        assert_eq!(
            range_result(source, Value::Int(0), Value::Int(0), 0)
                .unwrap_err()
                .kind,
            VmErrorKind::UnsupportedType
        );
    }
    for args in [
        vec![Value::bytes(b"x"), Value::bytes(b"0"), Value::Int(0)],
        vec![Value::bytes(b"x"), Value::Int(0), Value::Undefined],
    ] {
        let (mut vm, o) = machine(image(&[range_function(0, 0)], 0, &[]), vec![]);
        vm.start(o, 0, args).unwrap();
        vm.run(5).unwrap();
        let before = vm.stack().unwrap();
        let e = vm.run(1).unwrap_err();
        assert_eq!(e.kind, VmErrorKind::UnsupportedType);
        assert_eq!(e.pc, Some(5));
        assert!(e.offset.is_some());
        assert_eq!(vm.stack().unwrap(), before);
    }
    for position in 0..3 {
        let mut f = range_function(0, 0);
        f.nodes[2 + position].2 = 0x39;
        let (mut vm, o) = machine(image(&[f], 0, &[]), vec![]);
        vm.start(o, 0, vec![Value::bytes(b"x"), Value::Int(0), Value::Int(0)])
            .unwrap();
        vm.run(5).unwrap();
        let before = vm.stack().unwrap();
        assert_eq!(vm.run(1).unwrap_err().kind, VmErrorKind::UnsupportedFeature);
        assert_eq!(vm.stack().unwrap(), before);
    }
}
#[test]
fn ranges_zero_budget_and_single_step_match_continuous_execution() {
    let (mut vm, o) = machine(image(&[range_function(3, -128)], 0, &[]), vec![]);
    vm.start(
        o,
        0,
        vec![Value::bytes(b"abcde"), Value::Int(3), Value::Int(1)],
    )
    .unwrap();
    vm.run(5).unwrap();
    let before = vm.stack().unwrap();
    assert_eq!(vm.run(0).unwrap(), RunOutcome::BudgetExhausted);
    assert_eq!(vm.stack().unwrap(), before);
    assert_eq!(vm.run(1).unwrap(), RunOutcome::BudgetExhausted);
    assert_eq!(
        vm.stack().unwrap().last(),
        Some(&rml32::StackSnapshot::Value(Value::bytes(b"cde")))
    );
    returns(&mut vm, Value::bytes(b"cde"));
    assert_eq!(
        range_result(Value::bytes(b"abcde"), Value::Int(3), Value::Int(1), 3).unwrap(),
        Value::bytes(b"cde")
    );
}
#[test]
fn ranges_preparation_rejects_bad_modes_init_and_default_expressions() {
    for mode in [i16::MIN, -1, 4, i16::MAX] {
        let e = prepare_program(image(&[range_function(mode, 0)], 0, &[])).unwrap_err();
        assert_eq!(e.kind, VmErrorKind::UnsupportedFeature);
        assert_eq!(e.pc, Some(5));
    }
    let mut p = image(&[], 0, &[]);
    set_init(&mut p, &[(0, 0, 0x45), (0, 0, 0x48)]);
    let e = prepare_init(&init_catalog(p), "root").unwrap_err();
    assert_eq!(e.kind, VmErrorKind::UnsupportedFeature);
    assert_eq!(e.offset, Some(9000));
    let f = optional_fun(
        0,
        1,
        0,
        255,
        &[vec![(0, 0, 0x40), (0, 0, 0x3b), (1, 0, 0x3b), (0, 0, 0x45)]],
        &[(0, 0, 0x37)],
    );
    let mut p = image(&[f], 0, &[b"xy"]);
    optional_meta(&mut p, 0, 0, false, &[4]);
    assert_eq!(
        prepare_program(p).unwrap_err().kind,
        VmErrorKind::UnsupportedFeature
    );
}

#[test]
fn noninteger_arithmetic_and_deleted_execution_object_fail_explicitly() {
    let p = image(
        &[fun(0, 2, 0, 255, &[(0, 0, 0x37), (1, 0, 0x37), (0, 0, 1)])],
        0,
        &[],
    );
    let (mut vm, o) = machine(p, vec![]);
    vm.start(o, 0, vec![Value::bytes(b"b"), Value::bytes(b"a")])
        .unwrap();
    assert_eq!(vm.run(100).unwrap_err().kind, VmErrorKind::UnsupportedType);
    vm.start(o, 0, vec![Value::Int(1), Value::Int(2)]).unwrap();
    assert_eq!(vm.run(1).unwrap(), RunOutcome::BudgetExhausted);
    vm.objects_mut().remove(o).unwrap();
    assert_eq!(vm.run(1).unwrap_err().kind, VmErrorKind::InvalidObject);
}

#[test]
fn unknown_flags_const_and_atomic_prototypes_are_rejected() {
    let base = image(&[fun(0, 1, 0, 255, &[(0, 0, 0x37)])], 0, &[]);
    let mut reference = base.clone();
    reference.functions[0].arguments[0].flag = 2;
    let mut constant = base.clone();
    constant.functions[0].arguments[0].attrib = 2;
    let mut atomic = base.clone();
    atomic.functions[0].attrib = 2;
    let mut const_return = base;
    let mut raw = vec![0; 64];
    raw[12] = 2;
    const_return.functions[0].raw_hex = rml32::hex(&raw);
    for p in [reference, constant, atomic, const_return] {
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

// Stage 2J: optional/variadic entry metadata and code are synthesized together.
fn optional_fun(
    index: i16,
    args: usize,
    locals: usize,
    ret: u8,
    defaults: &[Vec<Node>],
    body: &[Node],
) -> Fun {
    let mut f = fun(index, args, locals, ret, &[]);
    let count: usize = defaults.iter().map(Vec::len).sum();
    f.nodes = vec![((count + usize::from(count > 0)) as i16, 0, 0x0f)];
    for block in defaults {
        f.nodes.extend(block);
    }
    if count > 0 {
        f.nodes.push((0, 0, 0x47));
    }
    f.nodes.push((0, 0, 0x1b));
    f.nodes.extend(body);
    f.nodes.push((0, 0, 0x49));
    f
}
fn optional_meta(p: &mut ProgramImage, function: usize, min: u8, variadic: bool, lengths: &[i16]) {
    let f = &mut p.functions[function];
    f.min_args = min;
    f.attrib = i16::from(variadic);
    let mut raw = vec![0; 64];
    raw[10..12].copy_from_slice(&f.local_count.to_le_bytes());
    raw[13] = f.return_type;
    raw[14] = f.max_args;
    raw[15] = min;
    raw[8..10].copy_from_slice(&f.attrib.to_le_bytes());
    f.raw_hex = rml32::hex(&raw);
    for (arg, length) in f.arguments.iter_mut().zip(lengths) {
        arg.default_action_count = *length;
        let mut raw = vec![0; 20];
        raw[5] = arg.r#type;
        raw[8..10].copy_from_slice(&arg.index.to_le_bytes());
        raw[10..12].copy_from_slice(&length.to_le_bytes());
        arg.raw_hex = rml32::hex(&raw);
    }
}
fn returned_array(vm: &mut Vm) -> Vec<Value> {
    let RunOutcome::Finished(Completion::Returned(Value::Array(a))) = vm.run(1000).unwrap() else {
        panic!("array expected")
    };
    a.elements().unwrap()
}

#[test]
fn defaults_preserve_declaration_order_for_every_accepted_arity() {
    for provided in 0usize..=3 {
        let defaults = vec![
            vec![(10, 0, 0x3b)],
            vec![(20, 0, 0x3b)],
            vec![(30, 0, 0x3b)],
        ];
        let f = optional_fun(0, 3, 0, 255, &defaults, &[(0, 0, 0x15)]);
        let mut p = image(&[f], 0, &[]);
        optional_meta(&mut p, 0, 0, false, &[1, 1, 1]);
        let (mut vm, o) = machine(p, vec![]);
        let args = (0..provided).map(|i| Value::Int(100 + i as i32)).collect();
        vm.start(o, 0, args).unwrap();
        let expected = (0..3)
            .map(|i| {
                Value::Int(if i < provided {
                    100 + i as i32
                } else {
                    10 * (i + 1) as i32
                })
            })
            .collect::<Vec<_>>();
        assert_eq!(returned_array(&mut vm), expected);
        assert_eq!(vm.frame_depth(), 0);
    }
}

#[test]
fn optional_mixed_parameters_keep_actual_count_and_have_no_writable_slot() {
    for provided in 0..=2 {
        let f = optional_fun(
            0,
            2,
            0,
            255,
            &[],
            &[(2, 0, 0x37), (0, 0, 0x34), (2, 0, 0x16)],
        );
        let mut p = image(&[f], 0, &[]);
        optional_meta(&mut p, 0, 0, false, &[0, 0]);
        let (mut vm, o) = machine(p, vec![]);
        vm.start(o, 0, vec![Value::Int(7); provided]).unwrap();
        assert_eq!(
            returned_array(&mut vm),
            vec![Value::Int(provided as i32), Value::Undefined]
        );
    }
    for op in [0x39, 0x4d] {
        let nodes = if op == 0x4d {
            vec![(7, 0, 0x3b), (0, 0, op)]
        } else {
            vec![(0, 0, op)]
        };
        let mut p = image(&[optional_fun(0, 1, 0, 255, &[], &nodes)], 0, &[]);
        optional_meta(&mut p, 0, 0, false, &[0]);
        let (mut vm, o) = machine(p, vec![]);
        vm.start(o, 0, vec![]).unwrap();
        assert_eq!(vm.run(100).unwrap_err().kind, VmErrorKind::Bounds);
    }
}

#[test]
fn default_results_do_not_fill_trailing_mixed_and_are_not_retyped_at_resort() {
    let f = optional_fun(0, 2, 0, 255, &[vec![(7, 0, 0x3b)], vec![]], &[(0, 0, 0x34)]);
    let mut p = image(&[f], 0, &[]);
    optional_meta(&mut p, 0, 0, false, &[1, 0]);
    p.functions[0].arguments[0].r#type = 4; // Native resort does not re-check defaults.
    let (mut vm, o) = machine(p, vec![]);
    vm.start(o, 0, vec![]).unwrap();
    returns(&mut vm, Value::Int(1));
    vm.start(o, 0, vec![Value::bytes(b"nine")]).unwrap();
    vm.run(1).unwrap(); // argc < max but all explicit defaults supplied: visit 47.
    let before = vm.stack().unwrap();
    vm.run(1).unwrap();
    assert_eq!(vm.stack().unwrap(), before);
    returns(&mut vm, Value::Int(1));
}

#[test]
fn variadic_extras_are_direct_slots_and_explicit_array_keeps_argument_order() {
    for min in [0, 1] {
        let mut p = image(&[optional_fun(0, 1, 0, 255, &[], &[(0, 0, 0x15)])], 0, &[]);
        optional_meta(&mut p, 0, min, true, &[0]);
        let (mut vm, o) = machine(p, vec![]);
        let args = vec![Value::Int(1), Value::bytes(b"two"), real(3.0)];
        vm.start(o, 0, args.clone()).unwrap();
        assert_eq!(returned_array(&mut vm), args);
        if min == 0 {
            vm.start(o, 0, vec![]).unwrap();
            assert_eq!(returned_array(&mut vm), Vec::<Value>::new());
        }
    }
    let mut p = image(&[optional_fun(0, 0, 0, 255, &[], &[(2, 0, 0x37)])], 0, &[]);
    optional_meta(&mut p, 0, 0, true, &[]);
    let (mut vm, o) = machine(p, vec![]);
    vm.start(o, 0, vec![Value::Int(1), Value::Int(2), Value::Int(3)])
        .unwrap();
    returns(&mut vm, Value::Int(3));
}

#[test]
fn variadic_default_calls_keep_count_for_missing_full_and_extra_arguments() {
    for argc in 0..=3 {
        let f = optional_fun(0, 1, 0, 2, &[vec![(7, 0, 0x3b)]], &[(0, 0, 0x34)]);
        let mut p = image(&[f], 0, &[]);
        optional_meta(&mut p, 0, 0, true, &[1]);
        let (mut vm, o) = machine(p, vec![]);
        vm.start(o, 0, vec![Value::Int(9); argc]).unwrap();
        returns(&mut vm, Value::Int(argc.max(1) as i32));
    }
}

#[test]
fn optional_calls_preserve_caller_prefix_and_reference_identity_during_resort() {
    let caller = fun(
        0,
        1,
        0,
        255,
        &[(0, 0, 0x39), (1, 1, 0x07), (0, 0, 0x33), (0, 0, 0x37)],
    );
    let leaf = optional_fun(
        1,
        3,
        0,
        1,
        &[vec![], vec![(20, 0, 0x3b)], vec![(30, 0, 0x3b)]],
        &[(0, 0, 0x39), (2, 0, 0x22)],
    );
    let mut p = image(&[caller, leaf], 0, &[]);
    optional_meta(&mut p, 1, 1, false, &[0, 1, 1]);
    ref_arg(&mut p, 1, 0, 2, 0);
    let (mut vm, o) = machine(p, vec![]);
    vm.start(o, 0, vec![Value::Int(5)]).unwrap();
    vm.run(4).unwrap();
    let before = vm.stack().unwrap();
    let id = match before.last().unwrap() {
        rml32::StackSnapshot::Reference { target_id, .. } => *target_id,
        _ => panic!(),
    };
    vm.run(4).unwrap();
    assert!(
        vm.stack()
            .unwrap()
            .iter()
            .any(|e| matches!(e,rml32::StackSnapshot::Reference{target_id,..}if *target_id==id))
    );
    returns(&mut vm, Value::Int(7));
}

#[test]
fn defaults_can_pause_at_each_instruction_without_repeating_initialization() {
    let f = optional_fun(
        0,
        2,
        1,
        255,
        &[vec![(10, 0, 0x3b)], vec![(20, 0, 0x3b)]],
        &[(0, 0, 0x15)],
    );
    let mut p = image(&[f], 0, &[]);
    optional_meta(&mut p, 0, 0, false, &[1, 1]);
    let (mut vm, o) = machine(p, vec![]);
    vm.start(o, 0, vec![]).unwrap();
    for step in 0..6 {
        assert_eq!(vm.run(1).unwrap(), RunOutcome::BudgetExhausted);
        let before = vm.stack().unwrap();
        assert_eq!(vm.run(0).unwrap(), RunOutcome::BudgetExhausted);
        assert_eq!(vm.stack().unwrap(), before, "step {step}");
    }
    assert_eq!(
        returned_array(&mut vm),
        vec![Value::Int(10), Value::Int(20)]
    );
}

#[test]
fn default_object_and_container_values_use_existing_shared_identity() {
    let defaults = vec![
        vec![(0, 0, 0x3d)],
        vec![(1, 0, 0x3b), (1, 0, 0x16)],
        vec![(0, 0, 0x18)],
    ];
    let mut p = image(
        &[optional_fun(0, 3, 0, 255, &defaults, &[(0, 0, 0x15)])],
        1,
        &[],
    );
    optional_meta(&mut p, 0, 0, false, &[1, 2, 1]);
    let shared = Value::array(vec![Value::Int(5)]).unwrap();
    let (mut vm, o) = machine(p, vec![shared.clone()]);
    vm.start(o, 0, vec![]).unwrap();
    let result = returned_array(&mut vm);
    assert_eq!(result[0], shared);
    let Value::Array(a) = &result[1] else {
        panic!()
    };
    assert_eq!(a.elements().unwrap(), vec![Value::Int(1)]);
    let Value::Mapping(m) = &result[2] else {
        panic!()
    };
    assert_eq!(m.entries().unwrap().len(), 0);
}

#[test]
fn optional_entry_arity_checks_do_not_replace_previous_completed_state() {
    let mut p = image(&[optional_fun(0, 2, 0, 255, &[], &[])], 0, &[]);
    optional_meta(&mut p, 0, 1, false, &[0, 0]);
    let (mut vm, o) = machine(p, vec![]);
    vm.start(o, 0, vec![Value::Int(5)]).unwrap();
    returns(&mut vm, Value::Void);
    let before = vm.stack().unwrap();
    for args in [vec![], vec![Value::Int(1); 3]] {
        assert_eq!(
            vm.start(o, 0, args).unwrap_err().kind,
            VmErrorKind::ArgumentCount
        );
        assert_eq!(vm.stack().unwrap(), before);
        assert_eq!(vm.frame_depth(), 0);
    }
}

#[test]
fn omitted_reference_and_unbound_variadic_reference_are_rejected_before_call() {
    for omitted in [true, false] {
        let caller = fun(0, 1, 0, 255, &[(0, 0, 0x39), (1, 1, 0x07)]);
        let leaf = optional_fun(1, if omitted { 2 } else { 0 }, 0, 255, &[], &[]);
        let mut p = image(&[caller, leaf], 0, &[]);
        optional_meta(&mut p, 1, 0, !omitted, if omitted { &[0, 0] } else { &[] });
        if omitted {
            ref_arg(&mut p, 1, 1, 255, 0);
        }
        let (mut vm, o) = machine(p, vec![]);
        vm.start(o, 0, vec![Value::Int(1)]).unwrap();
        vm.run(3).unwrap();
        let before = vm.stack().unwrap();
        let e = vm.run(1).unwrap_err();
        assert_eq!(e.kind, VmErrorKind::UnsupportedFeature);
        assert_eq!(e.pc, Some(3));
        assert_eq!(vm.stack().unwrap(), before);
        assert_eq!(vm.frame_depth(), 1);
    }
}

#[test]
fn unsupported_default_code_and_invalid_layouts_fail_preparation() {
    for case in 0..10 {
        let block = match case {
            0 => vec![(0, 0, 0x07)],
            1 => vec![(0, 0, 0x1f)],
            2 => vec![(1, 0, 0x16)],
            3 => vec![(1, 0, 0x3b), (2, 0, 0x3b)],
            _ => vec![(1, 0, 0x3b)],
        };
        let mut p = image(
            &[optional_fun(
                0,
                1,
                0,
                255,
                std::slice::from_ref(&block),
                &[],
            )],
            0,
            &[],
        );
        optional_meta(&mut p, 0, 0, false, &[block.len() as i16]);
        match case {
            4 => p.functions[0].arguments[0].default_action_count = -1,
            5 => p.functions[0].arguments[0].default_action_count = 100,
            6 => p.functions[0].code.nodes[0].0 = 0,
            7 => p.functions[0].code.nodes[2].2 = 0x1b,
            8 => p.functions[0].code.nodes.insert(4, (0, 0, 0x47)),
            9 => p.functions[0].code.nodes.insert(4, (-5, 0, 0x1f)),
            _ => {}
        }
        assert!(prepare_program(p).is_err(), "case {case}");
    }
}

#[test]
fn mixed_holes_before_later_defaults_are_rejected_per_invocation() {
    let mut p = image(
        &[optional_fun(
            0,
            2,
            0,
            255,
            &[vec![], vec![(8, 0, 0x3b)]],
            &[(0, 0, 0x15)],
        )],
        0,
        &[],
    );
    optional_meta(&mut p, 0, 0, false, &[0, 1]);
    let (mut vm, o) = machine(p, vec![]);
    assert_eq!(
        vm.start(o, 0, vec![]).unwrap_err().kind,
        VmErrorKind::UnsupportedFeature
    );
    vm.start(o, 0, vec![Value::Int(1)]).unwrap();
    assert_eq!(returned_array(&mut vm), vec![Value::Int(1), Value::Int(8)]);
}

#[test]
fn default_stack_limit_and_deleted_object_leave_prior_results_for_diagnostics() {
    let mut p = image(
        &[optional_fun(
            0,
            2,
            0,
            255,
            &[vec![(8, 0, 0x3b)], vec![(0, 0, 0x3d)]],
            &[],
        )],
        1,
        &[],
    );
    optional_meta(&mut p, 0, 0, false, &[1, 1]);
    let (vm, o) = machine(p, vec![Value::Int(5)]);
    let mut vm = Vm::with_limits(
        vm.into_objects(),
        VmLimits {
            max_stack: 1,
            ..VmLimits::default()
        },
    );
    vm.start(o, 0, vec![]).unwrap();
    vm.run(2).unwrap();
    let before = vm.stack().unwrap();
    let e = vm.run(1).unwrap_err();
    assert_eq!((e.kind, e.pc), (VmErrorKind::Limit, Some(2)));
    assert_eq!(vm.stack().unwrap(), before);
    vm.start(o, 0, vec![]).unwrap();
    vm.run(2).unwrap();
    vm.objects_mut().remove(o).unwrap();
    assert_eq!(vm.run(1).unwrap_err().kind, VmErrorKind::InvalidObject);
}

#[test]
fn argument_array_failure_does_not_normalize_reference_targets() {
    let caller = fun(0, 0, 0, 255, &[(0, 0, 0x3e), (1, 1, 0x07)]);
    let leaf = optional_fun(1, 1, 0, 255, &[], &[(0, 0, 0x15)]);
    let mut p = image(&[caller, leaf], 1, &[]);
    optional_meta(&mut p, 1, 1, true, &[0]);
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
    assert_eq!(object_values(&vm, caller), vec![Value::Object(o)]);
    assert_eq!(vm.stack().unwrap(), before);
}

#[test]
fn argument_array_success_normalizes_reference_targets_and_dereferences_them() {
    let caller = fun(0, 0, 0, 255, &[(0, 0, 0x3e), (1, 1, 0x07)]);
    let leaf = optional_fun(1, 1, 0, 255, &[], &[(0, 0, 0x15)]);
    let mut p = image(&[caller, leaf], 1, &[]);
    optional_meta(&mut p, 1, 1, true, &[0]);
    ref_arg(&mut p, 1, 0, 255, 0);
    let (mut vm, o) = machine(p, vec![Value::Undefined]);
    let pid = vm.objects().get(o).unwrap().program();
    let caller = vm
        .objects_mut()
        .create(pid, vec![Value::Object(o)])
        .unwrap();
    vm.start(caller, 0, vec![]).unwrap();
    vm.run(6).unwrap();
    vm.objects_mut().remove(o).unwrap();
    assert_eq!(returned_array(&mut vm), vec![Value::Undefined]);
    assert_eq!(object_values(&vm, caller), vec![Value::Undefined]);
}

#[test]
fn default_literals_decode_numeric_metadata_strings_and_undefined() {
    let mut f = optional_fun(
        0,
        4,
        0,
        255,
        &[
            vec![(0, 0, 0x3c)],
            vec![(4, 0, 0x3f)],
            vec![(0, 0, 0x40)],
            vec![(0, 0, 0x42)],
        ],
        &[(0, 0, 0x15)],
    );
    f.meta.extend(123456i32.to_le_bytes());
    f.meta.extend(1.5f32.to_bits().to_le_bytes());
    f.meta.extend(0u32.to_le_bytes());
    let mut p = image(&[f], 0, &[b"literal\0"]);
    optional_meta(&mut p, 0, 0, false, &[1, 1, 1, 1]);
    let (mut vm, o) = machine(p, vec![]);
    vm.start(o, 0, vec![]).unwrap();
    assert_eq!(
        returned_array(&mut vm),
        vec![
            Value::Int(123456),
            real(1.5),
            Value::bytes(b"literal"),
            Value::Undefined
        ]
    );
}

#[test]
fn optional_type_check_is_atomic_before_any_default_is_evaluated() {
    let mut p = image(
        &[optional_fun(
            0,
            3,
            0,
            255,
            &[vec![], vec![], vec![(7, 0, 0x3b)]],
            &[],
        )],
        0,
        &[],
    );
    optional_meta(&mut p, 0, 2, false, &[0, 0, 1]);
    p.functions[0].arguments[0].r#type = 3;
    p.functions[0].arguments[1].r#type = 2;
    let (mut vm, o) = machine(p, vec![]);
    vm.start(o, 0, vec![Value::Int(5), Value::bytes(b"bad")])
        .unwrap();
    let before = vm.stack().unwrap();
    let e = vm.run(1).unwrap_err();
    assert_eq!((e.kind, e.pc), (VmErrorKind::UnsupportedType, Some(0)));
    assert_eq!(vm.stack().unwrap(), before);
}

// Stage 2L: every linked program here is synthetic, never a shipped member.
fn inherit(child: &mut ProgramImage, parent: &ProgramImage, name: &[u8], bases: [i16; 3]) {
    child.inherits.push(rml32::program::Inherit {
        name: Some(ByteString::new(name)),
        offsets: [
            bases[0],
            bases[1],
            bases[2],
            parent.constitution_count.unwrap(),
            parent.max_functions,
            parent.max_vars,
        ],
    });
}
fn linked_catalog(parent: ProgramImage, child: ProgramImage) -> rml32::ProgramCatalog {
    let mut c = rml32::ProgramCatalog::new();
    c.insert("parent", parent).unwrap();
    c.insert("child", child).unwrap();
    c.bind_inherit("child", b"parent\0", "parent");
    c
}
fn link(c: &rml32::ProgramCatalog) -> Result<rml32::PreparedProgram, rml32::VmError> {
    rml32::prepare_linked_program(c, "child", rml32::LinkLimits::default())
}
fn inherited_pair(parent_nodes: &[Node], child_nodes: &[Node]) -> (ProgramImage, ProgramImage) {
    let p = image(&[fun(0, 0, 0, 255, parent_nodes)], 1, &[b"parent\0"]);
    let mut c = image(&[fun(1, 0, 0, 255, child_nodes)], 2, &[b"child\0"]);
    c.constitution_count = Some(2);
    inherit(&mut c, &p, b"parent\0", [1, 0, 1]);
    (p, c)
}
fn linked_vm(c: &rml32::ProgramCatalog) -> (Vm, rml32::ObjectId) {
    let p = link(c).unwrap();
    let n = p.variable_count();
    let mut store = ObjectStore::new();
    let id = store.register_program(p).unwrap();
    let o = store.create(id, vec![Value::Int(0); n]).unwrap();
    (Vm::new(store), o)
}
#[test]
fn linked_calls_use_body_string_pool_and_restore_caller() {
    let (p, c) = inherited_pair(&[(0, 0, 0x40)], &[(0, 0, 0x07), (0, 0, 0x33), (0, 0, 0x40)]);
    let catalog = linked_catalog(p, c);
    let (mut vm, o) = linked_vm(&catalog);
    vm.start(o, 0, vec![]).unwrap();
    assert_eq!(
        vm.run(100).unwrap(),
        RunOutcome::Finished(Completion::Returned(Value::bytes(b"parent")))
    );
    vm.start(o, 1, vec![]).unwrap();
    while vm.run(1).unwrap() == RunOutcome::BudgetExhausted {}
    assert_eq!(
        vm.run(0).unwrap(),
        RunOutcome::Finished(Completion::Returned(Value::bytes(b"child")))
    );
}
#[test]
fn linked_variables_relocate_and_objects_remain_isolated() {
    let (p, c) = inherited_pair(
        &[(9, 0, 0x3b), (0, 0, 0x4e), (0, 0, 0x33), (0, 0, 0x3d)],
        &[(0, 0, 0x07)],
    );
    let catalog = linked_catalog(p, c);
    let prepared = link(&catalog).unwrap();
    let mut store = ObjectStore::new();
    let id = store.register_program(prepared).unwrap();
    let a = store
        .create(id, vec![Value::Int(1), Value::Int(2)])
        .unwrap();
    let b = store
        .create(id, vec![Value::Int(3), Value::Int(4)])
        .unwrap();
    let mut vm = Vm::new(store);
    vm.start(a, 1, vec![]).unwrap();
    assert_eq!(
        vm.run(100).unwrap(),
        RunOutcome::Finished(Completion::Returned(Value::Int(9)))
    );
    assert_eq!(
        vm.objects().get(a).unwrap().variables().unwrap(),
        vec![Value::Int(1), Value::Int(9)]
    );
    assert_eq!(
        vm.objects().get(b).unwrap().variables().unwrap(),
        vec![Value::Int(3), Value::Int(4)]
    );
}
#[test]
fn linked_multilevel_calls_add_function_bases() {
    let p = image(
        &[
            fun(0, 0, 0, 255, &[(42, 0, 0x3b)]),
            fun(1, 0, 0, 255, &[(0, 0, 0x07)]),
        ],
        0,
        &[],
    );
    let mut mid = image(&[], 0, &[]);
    mid.max_functions = 3;
    mid.constitution_count = Some(2);
    inherit(&mut mid, &p, b"base", [1, 1, 0]);
    let mut root = image(&[], 0, &[]);
    root.max_functions = 5;
    root.constitution_count = Some(3);
    inherit(&mut root, &mid, b"middle", [1, 2, 0]);
    let mut catalog = rml32::ProgramCatalog::new();
    catalog.insert("base", p).unwrap();
    catalog.insert("mid", mid).unwrap();
    catalog.insert("child", root).unwrap();
    catalog.bind_inherit("mid", b"base", "base");
    catalog.bind_inherit("child", b"middle", "mid");
    let prepared = link(&catalog).unwrap();
    assert_eq!(prepared.function(4).unwrap().index(), 4);
    let (mut vm, o) = linked_vm(&catalog);
    vm.start(o, 4, vec![]).unwrap();
    assert_eq!(
        vm.run(100).unwrap(),
        RunOutcome::Finished(Completion::Returned(Value::Int(42)))
    );
}
#[test]
fn linked_declarations_bind_but_concrete_same_names_stay_distinct() {
    let (mut p, mut c) = inherited_pair(&[(11, 0, 0x3b)], &[(22, 0, 0x3b)]);
    p.functions[0].name = Some(ByteString::new(b"same"));
    c.functions[0].name = p.functions[0].name.clone();
    let catalog = linked_catalog(p.clone(), c.clone());
    let (mut vm, o) = linked_vm(&catalog);
    for (slot, v) in [(0, 11), (1, 22)] {
        vm.start(o, slot, vec![]).unwrap();
        assert_eq!(
            vm.run(100).unwrap(),
            RunOutcome::Finished(Completion::Returned(Value::Int(v)))
        );
    }
    c.functions[0].code.nodes.clear();
    let catalog = linked_catalog(p, c);
    let (mut vm, o) = linked_vm(&catalog);
    vm.start(o, 1, vec![]).unwrap();
    assert_eq!(
        vm.run(100).unwrap(),
        RunOutcome::Finished(Completion::Returned(Value::Int(11)))
    );
}
#[test]
fn linked_unimplemented_and_missing_slots_differ() {
    let (p, mut c) = inherited_pair(&[(1, 0, 0x3b)], &[]);
    c.functions[0].code.nodes.clear();
    c.max_functions = 3;
    let catalog = linked_catalog(p, c);
    let prepared = link(&catalog).unwrap();
    assert!(prepared.function(1).is_none());
    let (mut vm, o) = linked_vm(&catalog);
    assert_eq!(
        vm.start(o, 1, vec![]).unwrap_err().kind,
        VmErrorKind::UnsupportedFeature
    );
    assert_eq!(
        vm.start(o, 2, vec![]).unwrap_err().kind,
        VmErrorKind::InvalidProgram
    );
}
#[test]
fn linked_errors_report_root_slot_and_actual_body_source() {
    let (p, c) = inherited_pair(&[(0, 0, 0x33)], &[(0, 0, 0x07)]);
    let catalog = linked_catalog(p, c);
    let (mut vm, o) = linked_vm(&catalog);
    vm.start(o, 1, vec![]).unwrap();
    let e = vm.run(100).unwrap_err();
    assert_eq!(e.source, "parent");
    assert_eq!(e.function, Some(0));
    assert!(e.program.is_some());
    assert_eq!(e.pc, Some(2));
    assert!(e.offset.is_some());
}
#[test]
fn linked_rejects_layout_errors_before_registration() {
    let (p, c) = inherited_pair(&[(1, 0, 0x3b)], &[]);
    for case in 0..7 {
        let mut c = c.clone();
        match case {
            0 => c.inherits[0].offsets[3] = 2,
            1 => c.inherits[0].offsets[0] = 0,
            2 => c.inherits[0].offsets[1] = 1,
            3 => c.inherits[0].offsets[2] = 2,
            4 => c.constitution_count = Some(3),
            5 => c.inherits[0].offsets[0] = -1,
            _ => c.functions[0].index = Some(0),
        }
        assert!(link(&linked_catalog(p.clone(), c)).is_err(), "case {case}");
    }
}
#[test]
fn linked_rejects_missing_ambiguous_bindings_and_limits() {
    let (p, c) = inherited_pair(&[(1, 0, 0x3b)], &[]);
    let mut catalog = rml32::ProgramCatalog::new();
    catalog.insert("child", c).unwrap();
    catalog.insert("parent", p).unwrap();
    assert!(link(&catalog).is_err());
    catalog.bind_inherit("child", b"parent\0", "parent");
    for limits in [
        rml32::LinkLimits {
            max_depth: 0,
            ..Default::default()
        },
        rml32::LinkLimits {
            max_dependencies: 0,
            ..Default::default()
        },
        rml32::LinkLimits {
            max_function_slots: 1,
            ..Default::default()
        },
    ] {
        assert_eq!(
            rml32::prepare_linked_program(&catalog, "child", limits)
                .unwrap_err()
                .kind,
            VmErrorKind::Limit
        );
    }
    catalog.bind_inherit("child", b"parent\0", "other");
    assert!(link(&catalog).unwrap_err().message.contains("ambiguous"));
}
#[test]
fn linked_rejects_prototype_conflicts_and_lifecycle_execution() {
    let (mut p, mut c) = inherited_pair(&[(1, 0, 0x3b)], &[]);
    c.functions[0].name = p.functions[0].name.clone();
    c.functions[0].code.nodes.clear();
    c.functions[0].return_type = 2;
    assert_eq!(
        link(&linked_catalog(p.clone(), c.clone()))
            .unwrap_err()
            .kind,
        VmErrorKind::InvalidProgram
    );
    c.functions[0].return_type = 255;
    p.functions[0].name = Some(ByteString::new(b"create\0"));
    let catalog = linked_catalog(p, c);
    let (mut vm, o) = linked_vm(&catalog);
    assert_eq!(
        vm.start(o, 0, vec![]).unwrap_err().kind,
        VmErrorKind::UnsupportedFeature
    );
}
#[test]
fn linked_deleted_object_fails_while_paused() {
    let (p, c) = inherited_pair(&[(0, 0, 0x3d)], &[(0, 0, 0x07)]);
    let (mut vm, o) = linked_vm(&linked_catalog(p, c));
    vm.start(o, 1, vec![]).unwrap();
    assert_eq!(vm.run(3).unwrap(), RunOutcome::BudgetExhausted);
    vm.objects_mut().remove(o).unwrap();
    assert_eq!(vm.run(1).unwrap_err().kind, VmErrorKind::InvalidObject);
}

#[test]
fn linked_multiple_parents_keep_separate_contexts() {
    let a = image(&[fun(0, 0, 0, 255, &[(0, 0, 0x40)])], 0, &[b"a\0"]);
    let b = image(&[fun(0, 0, 0, 255, &[(0, 0, 0x40)])], 0, &[b"b\0"]);
    let mut c = image(
        &[fun(
            2,
            0,
            0,
            255,
            &[(0, 0, 0x07), (1, 0, 0x07), (2, 0, 0x16)],
        )],
        0,
        &[],
    );
    c.constitution_count = Some(3);
    inherit(&mut c, &a, b"a", [1, 0, 0]);
    inherit(&mut c, &b, b"b", [2, 1, 0]);
    let mut catalog = rml32::ProgramCatalog::new();
    catalog.insert("a", a).unwrap();
    catalog.insert("b", b).unwrap();
    catalog.insert("child", c).unwrap();
    catalog.bind_inherit("child", b"a", "a");
    catalog.bind_inherit("child", b"b", "b");
    let (mut vm, o) = linked_vm(&catalog);
    vm.start(o, 2, vec![]).unwrap();
    assert_eq!(
        returned_array(&mut vm),
        vec![Value::bytes(b"b"), Value::bytes(b"a")]
    );
}
#[test]
fn linked_reference_parameters_keep_object_cell_identity() {
    let mut p = image(
        &[fun(0, 1, 0, 1, &[(9, 0, 0x3b), (0, 0, 0x4d), (0, 0, 0x33)])],
        0,
        &[],
    );
    ref_arg(&mut p, 0, 0, 2, 0);
    let mut c = image(
        &[fun(
            1,
            0,
            0,
            255,
            &[(0, 0, 0x3e), (0, 1, 0x07), (0, 0, 0x33), (0, 0, 0x3d)],
        )],
        1,
        &[],
    );
    c.constitution_count = Some(2);
    inherit(&mut c, &p, b"parent\0", [1, 0, 0]);
    let (mut vm, o) = linked_vm(&linked_catalog(p, c));
    vm.start(o, 1, vec![]).unwrap();
    assert_eq!(
        vm.run(100).unwrap(),
        RunOutcome::Finished(Completion::Returned(Value::Int(9)))
    );
    assert_eq!(object_values(&vm, o), vec![Value::Int(9)]);
}
#[test]
fn linked_alias_uses_implementation_optional_defaults_and_metadata() {
    let mut f = optional_fun(0, 1, 0, 255, &[vec![(0, 0, 0x3c)]], &[(0, 0, 0x37)]);
    f.meta = 12345i32.to_le_bytes().to_vec();
    let mut p = image(&[f], 0, &[]);
    optional_meta(&mut p, 0, 0, false, &[1]);
    let mut c = image(&[fun(1, 1, 0, 255, &[])], 0, &[]);
    c.functions[0].name = p.functions[0].name.clone();
    c.functions[0].code.nodes.clear();
    c.functions[0].min_args = 0;
    c.functions[0].max_args = 0;
    c.constitution_count = Some(2);
    inherit(&mut c, &p, b"parent\0", [1, 0, 0]);
    let (mut vm, o) = linked_vm(&linked_catalog(p, c));
    vm.start(o, 1, vec![]).unwrap();
    assert_eq!(
        vm.run(100).unwrap(),
        RunOutcome::Finished(Completion::Returned(Value::Int(12345)))
    );
}
#[test]
fn linked_alias_search_prefers_nearest_lower_and_skips_aliases_private() {
    let mut p = image(
        &[
            fun(0, 0, 0, 255, &[(10, 0, 0x3b)]),
            fun(1, 0, 0, 255, &[(20, 0, 0x3b)]),
        ],
        0,
        &[],
    );
    for f in &mut p.functions {
        f.name = Some(ByteString::new(b"same"));
    }
    for private in [false, true] {
        let mut p = p.clone();
        if private {
            p.functions[1].attrib |= 4;
        }
        let mut c = image(&[fun(2, 0, 0, 255, &[])], 0, &[]);
        c.functions[0].name = Some(ByteString::new(b"same"));
        c.functions[0].code.nodes.clear();
        c.constitution_count = Some(2);
        inherit(&mut c, &p, b"parent\0", [1, 0, 0]);
        let (mut vm, o) = linked_vm(&linked_catalog(p, c));
        vm.start(o, 2, vec![]).unwrap();
        assert_eq!(
            vm.run(100).unwrap(),
            RunOutcome::Finished(Completion::Returned(Value::Int(if private {
                10
            } else {
                20
            })))
        );
    }
}
#[test]
fn linked_rejects_cycles_repeated_ancestors_and_nonobject_dependencies() {
    let (mut p, c) = inherited_pair(&[(1, 0, 0x3b)], &[]);
    p.inherits.push(rml32::program::Inherit {
        name: Some(ByteString::new(b"self")),
        offsets: [0, 0, 0, 1, 1, 1],
    });
    let mut catalog = linked_catalog(p, c);
    catalog.bind_inherit("parent", b"self", "parent");
    assert!(link(&catalog).unwrap_err().message.contains("cycle"));
    let (p, mut c) = inherited_pair(&[(1, 0, 0x3b)], &[]);
    c.inherits.push(c.inherits[0].clone());
    assert!(
        link(&linked_catalog(p, c))
            .unwrap_err()
            .message
            .contains("ancestor")
    );
    let (mut p, c) = inherited_pair(&[(1, 0, 0x3b)], &[]);
    p.mode = ProgramMode::Global;
    assert_eq!(
        link(&linked_catalog(p, c)).unwrap_err().kind,
        VmErrorKind::UnsupportedFeature
    );
}

#[test]
fn linked_variadic_body_retains_effective_argument_count() {
    let mut p = image(&[optional_fun(0, 0, 0, 255, &[], &[(0, 0, 0x34)])], 0, &[]);
    optional_meta(&mut p, 0, 0, true, &[]);
    let mut c = image(
        &[fun(
            1,
            0,
            0,
            255,
            &[(3, 0, 0x3b), (4, 0, 0x3b), (0, 2, 0x07)],
        )],
        0,
        &[],
    );
    c.constitution_count = Some(2);
    inherit(&mut c, &p, b"parent\0", [1, 0, 0]);
    let (mut vm, o) = linked_vm(&linked_catalog(p, c));
    vm.start(o, 1, vec![]).unwrap();
    assert_eq!(
        vm.run(100).unwrap(),
        RunOutcome::Finished(Completion::Returned(Value::Int(2)))
    );
}
#[test]
fn linked_preserves_imported_aliases_and_skips_them_as_candidates() {
    let mut p = image(
        &[fun(0, 0, 0, 255, &[]), fun(1, 0, 0, 255, &[(77, 0, 0x3b)])],
        0,
        &[],
    );
    for f in &mut p.functions {
        f.name = Some(ByteString::new(b"same"));
    }
    p.functions[0].code.nodes.clear();
    let mut c = image(&[fun(2, 0, 0, 255, &[])], 0, &[]);
    c.functions[0].name = Some(ByteString::new(b"same"));
    c.functions[0].code.nodes.clear();
    c.constitution_count = Some(2);
    inherit(&mut c, &p, b"parent\0", [1, 0, 0]);
    let (mut vm, o) = linked_vm(&linked_catalog(p, c));
    for slot in [0, 2] {
        vm.start(o, slot, vec![]).unwrap();
        assert_eq!(
            vm.run(100).unwrap(),
            RunOutcome::Finished(Completion::Returned(Value::Int(77)))
        );
    }
}
#[test]
fn linked_reference_and_return_attribute_mismatches_are_explicit() {
    let (mut p, mut c) = inherited_pair(&[(1, 0, 0x3b)], &[]);
    c.functions[0].name = p.functions[0].name.clone();
    c.functions[0].code.nodes.clear();
    let mut raw = vec![0; 64];
    raw[12] = 8;
    c.functions[0].raw_hex = rml32::hex(&raw);
    assert_eq!(
        link(&linked_catalog(p.clone(), c.clone()))
            .unwrap_err()
            .kind,
        VmErrorKind::UnsupportedFeature
    );
    // A second declaration with byte-identical type/attrib but different reference flag.
    p = image(&[fun(0, 1, 0, 255, &[(0, 0, 0x37)])], 1, &[]);
    c.functions[0] = p.functions[0].clone();
    c.functions[0].index = Some(1);
    c.functions[0].code.nodes.clear();
    c.inherits[0].offsets[4] = 1;
    ref_arg(&mut p, 0, 0, 255, 0);
    assert_eq!(
        link(&linked_catalog(p, c)).unwrap_err().kind,
        VmErrorKind::UnsupportedFeature
    );
}

#[test]
fn linked_failure_does_not_change_registered_objects_or_program_ids() {
    let mut store = ObjectStore::new();
    let first = store
        .register_program(
            prepare_program(image(&[fun(0, 0, 0, 255, &[(5, 0, 0x3b)])], 1, &[])).unwrap(),
        )
        .unwrap();
    let o = store.create(first, vec![Value::Int(8)]).unwrap();
    let (p, mut c) = inherited_pair(&[], &[]);
    c.inherits[0].offsets[3] = 7;
    assert!(link(&linked_catalog(p, c)).is_err());
    assert_eq!(
        store.get(o).unwrap().variables().unwrap(),
        vec![Value::Int(8)]
    );
    let next = store
        .register_program(prepare_program(image(&[], 0, &[])).unwrap())
        .unwrap();
    assert_eq!(next.get(), first.get() + 1);
}
#[test]
fn linked_rejects_diamond_even_when_intervals_are_disjoint() {
    let base = image(&[], 0, &[]);
    let mut left = image(&[], 0, &[]);
    left.constitution_count = Some(2);
    inherit(&mut left, &base, b"base", [1, 0, 0]);
    let right = left.clone();
    let mut root = image(&[], 0, &[]);
    root.constitution_count = Some(5);
    inherit(&mut root, &left, b"left", [1, 0, 0]);
    inherit(&mut root, &right, b"right", [3, 0, 0]);
    let mut c = rml32::ProgramCatalog::new();
    for (key, image) in [
        ("base", base),
        ("left", left),
        ("right", right),
        ("child", root),
    ] {
        c.insert(key, image).unwrap();
    }
    c.bind_inherit("left", b"base", "base");
    c.bind_inherit("right", b"base", "base");
    c.bind_inherit("child", b"left", "left");
    c.bind_inherit("child", b"right", "right");
    assert_eq!(link(&c).unwrap_err().kind, VmErrorKind::UnsupportedFeature);
}
#[test]
fn linked_name_matching_is_byte_exact_and_upper_candidates_are_used() {
    let mut p = image(&[fun(1, 0, 0, 255, &[(8, 0, 0x3b)])], 0, &[]);
    p.max_functions = 2;
    let mut c = image(&[fun(2, 0, 0, 255, &[])], 0, &[]);
    c.max_functions = 4;
    c.constitution_count = Some(2);
    inherit(&mut c, &p, b"parent\0", [1, 0, 0]);
    // Local slot 2 is below imported body slot 1 only after relocation by 2.
    c.inherits[0].offsets[1] = 2;
    c.functions[0].index = Some(0);
    c.functions[0].code.nodes.clear();
    c.functions[0].name = p.functions[0].name.clone();
    let (mut vm, o) = linked_vm(&linked_catalog(p.clone(), c.clone()));
    vm.start(o, 0, vec![]).unwrap();
    assert_eq!(
        vm.run(100).unwrap(),
        RunOutcome::Finished(Completion::Returned(Value::Int(8)))
    );
    c.functions[0].name.as_mut().unwrap().bytes.push(255);
    let (mut vm, o) = linked_vm(&linked_catalog(p, c));
    assert_eq!(
        vm.start(o, 0, vec![]).unwrap_err().kind,
        VmErrorKind::UnsupportedFeature
    );
}

// Stage 2N: opted-in initialization executes only these in-memory fixtures.
fn set_init(p: &mut ProgramImage, nodes: &[Node]) {
    p.init.nodes = nodes.to_vec();
    p.init.nodes_offset = 9000;
    p.init.metadata_hex.clear();
}
fn init_catalog(p: ProgramImage) -> rml32::ProgramCatalog {
    let mut c = rml32::ProgramCatalog::new();
    c.insert("root", p).unwrap();
    c
}
fn prepare_init(
    c: &rml32::ProgramCatalog,
    root: &str,
) -> Result<rml32::PreparedProgram, rml32::VmError> {
    rml32::prepare_initializable_program(c, root, rml32::LinkLimits::default())
}
fn init_vm(c: &rml32::ProgramCatalog, root: &str) -> (Vm, rml32::ProgramId) {
    let p = prepare_init(c, root).unwrap();
    assert!(p.is_initializable());
    let mut store = ObjectStore::new();
    let id = store.register_program(p).unwrap();
    (Vm::new(store), id)
}
#[test]
fn initialization_creates_undefined_values_and_finishes_exactly_on_48() {
    let mut p = image(&[fun(0, 0, 0, 255, &[(0, 0, 0x3d)])], 2, &[]);
    set_init(&mut p, &[(0, 0, 0x2e), (0, 0, 0x48), (0, 0, 0x1d)]);
    let (mut vm, id) = init_vm(&init_catalog(p), "root");
    let o = vm.begin_initialize(id).unwrap();
    assert_eq!(object_values(&vm, o), vec![Value::Undefined; 2]);
    assert_eq!(
        vm.objects().get(o).unwrap().initialization_state(),
        rml32::InitializationState::Initializing
    );
    assert_eq!(vm.run(0).unwrap(), RunOutcome::BudgetExhausted);
    assert_eq!(vm.run(1).unwrap(), RunOutcome::BudgetExhausted);
    assert_eq!(
        vm.run(1).unwrap(),
        RunOutcome::Finished(Completion::Initialized(o))
    );
    assert_eq!(vm.frame_depth(), 0);
    assert!(vm.stack().unwrap().is_empty());
    assert_eq!(
        vm.objects().get(o).unwrap().initialization_state(),
        rml32::InitializationState::Initialized
    );
    assert_eq!(
        vm.run(0).unwrap(),
        RunOutcome::Finished(Completion::Initialized(o))
    );
    vm.start(o, 0, vec![]).unwrap();
    assert_eq!(
        vm.run(100).unwrap(),
        RunOutcome::Finished(Completion::Returned(Value::Undefined))
    );
}
#[test]
fn initialization_is_opt_in_and_existing_prepare_interfaces_ignore_init() {
    let mut p = image(&[fun(0, 0, 0, 255, &[(5, 0, 0x3b)])], 0, &[]);
    set_init(&mut p, &[(0, 0, 0x06)]);
    let plain = prepare_program(p.clone()).unwrap();
    assert!(!plain.is_initializable());
    let c = init_catalog(p);
    assert!(
        !rml32::prepare_linked_program(&c, "root", Default::default())
            .unwrap()
            .is_initializable()
    );
    assert_eq!(
        prepare_init(&c, "root").unwrap_err().kind,
        VmErrorKind::UnsupportedFeature
    );
    let mut store = ObjectStore::new();
    let id = store.register_program(plain).unwrap();
    let o = store.create(id, vec![]).unwrap();
    let mut vm = Vm::new(store);
    vm.start(o, 0, vec![]).unwrap();
    assert_eq!(
        vm.run(100).unwrap(),
        RunOutcome::Finished(Completion::Returned(Value::Int(5)))
    );
    assert_eq!(
        vm.begin_initialize(id).unwrap_err().kind,
        VmErrorKind::UnsupportedFeature
    );
    assert_eq!(
        vm.run(0).unwrap(),
        RunOutcome::Finished(Completion::Returned(Value::Int(5)))
    );
}
#[test]
fn initialization_preparation_rejects_empty_missing_return_and_function_only_nodes() {
    for (nodes, kind) in [
        (vec![], VmErrorKind::InvalidProgram),
        (vec![(0, 0, 0x2e)], VmErrorKind::InvalidProgram),
    ] {
        let mut p = image(&[], 0, &[]);
        set_init(&mut p, &nodes);
        let e = prepare_init(&init_catalog(p), "root").unwrap_err();
        assert_eq!(e.kind, kind);
        assert_eq!(e.initialization, Some(0));
        assert_eq!(e.function, None);
        assert_eq!(e.source, "root");
    }
    for op in [0x0f, 0x1b, 0x47, 0x49, 0x34, 0x15, 0x37, 0x39, 0x4d, 0x06] {
        let mut p = image(&[], 0, &[]);
        set_init(&mut p, &[(0, 0, op), (0, 0, 0x48)]);
        let e = prepare_init(&init_catalog(p), "root").unwrap_err();
        assert_eq!(e.kind, VmErrorKind::UnsupportedFeature, "{op:x}");
        assert_eq!(e.pc, Some(0));
        assert_eq!(e.offset, Some(9000));
    }
}
#[test]
fn initialization_rejects_ordinary_function_48_but_allows_null_action() {
    let p = image(&[fun(0, 0, 0, 255, &[(0, 0, 0x48)])], 0, &[]);
    assert_eq!(
        prepare_program(p).unwrap_err().kind,
        VmErrorKind::UnsupportedFeature
    );
    let (mut vm, o) = machine(
        image(&[fun(0, 0, 0, 255, &[(0, 0, 0x2e), (6, 0, 0x3b)])], 0, &[]),
        vec![],
    );
    vm.start(o, 0, vec![]).unwrap();
    assert_eq!(
        vm.run(100).unwrap(),
        RunOutcome::Finished(Completion::Returned(Value::Int(6)))
    );
}
#[test]
fn initialization_balance_failure_removes_only_owned_object_and_is_sticky() {
    let mut p = image(&[], 1, &[]);
    set_init(&mut p, &[(7, 0, 0x3b), (0, 0, 0x48)]);
    let (mut vm, id) = init_vm(&init_catalog(p), "root");
    let manual = vm.objects_mut().create(id, vec![Value::Int(99)]).unwrap();
    assert_eq!(
        vm.objects().get(manual).unwrap().initialization_state(),
        rml32::InitializationState::Manual
    );
    let o = vm.begin_initialize(id).unwrap();
    let e = vm.run(100).unwrap_err();
    assert_eq!(e.kind, VmErrorKind::InvalidProgram);
    assert_eq!(e.function, None);
    assert_eq!(e.initialization, Some(0));
    assert_eq!(e.object, Some(o));
    assert_eq!(e.program, Some(id));
    assert_eq!(e.source, "root");
    assert_eq!(e.pc, Some(1));
    assert_eq!(e.offset, Some(9004));
    assert!(vm.objects().get(o).is_err());
    assert_eq!(object_values(&vm, manual), vec![Value::Int(99)]);
    assert_eq!(vm.frame_depth(), 0);
    assert!(vm.stack().unwrap().is_empty());
    assert_eq!(vm.run(1).unwrap_err(), e);
    let again = vm.begin_initialize(id).unwrap();
    assert_ne!(again, o);
    assert_eq!(vm.run(100).unwrap_err().object, Some(again));
}
#[test]
fn initialization_finish_action_is_failure_even_inside_called_function() {
    for nested in [false, true] {
        let mut p = image(&[fun(0, 0, 0, 255, &[(0, 0, 0x1d)])], 0, &[]);
        set_init(
            &mut p,
            &[(0, 0, if nested { 0x07 } else { 0x1d }), (0, 0, 0x48)],
        );
        let (mut vm, id) = init_vm(&init_catalog(p), "root");
        let o = vm.begin_initialize(id).unwrap();
        let e = vm.run(100).unwrap_err();
        assert_eq!(e.kind, VmErrorKind::UnsupportedFeature);
        assert_eq!(e.initialization, Some(0));
        assert_eq!(e.function, if nested { Some(0) } else { None });
        assert!(vm.objects().get(o).is_err());
    }
}
#[test]
fn initialization_external_delete_and_into_objects_cancel_owned_storage() {
    let mut p = image(&[], 0, &[]);
    set_init(&mut p, &[(0, 0, 0x2e), (0, 0, 0x48)]);
    let c = init_catalog(p);
    let (mut vm, id) = init_vm(&c, "root");
    let o = vm.begin_initialize(id).unwrap();
    vm.objects_mut().remove(o).unwrap();
    let e = vm.run(1).unwrap_err();
    assert_eq!(e.kind, VmErrorKind::InvalidObject);
    assert_eq!(e.initialization, Some(0));
    assert_eq!(vm.frame_depth(), 0);
    let (mut vm, id) = init_vm(&c, "root");
    let manual = vm.objects_mut().create(id, vec![]).unwrap();
    let o = vm.begin_initialize(id).unwrap();
    assert_eq!(vm.run(1).unwrap(), RunOutcome::BudgetExhausted);
    let store = vm.into_objects();
    assert!(store.get(o).is_err());
    assert!(store.get(manual).is_ok());
}
#[test]
fn initialization_excludes_concurrent_starts_and_preflight_preserves_paused_work() {
    let mut p = image(&[fun(0, 0, 0, 255, &[])], 0, &[]);
    set_init(&mut p, &[(0, 0, 0x2e), (0, 0, 0x48)]);
    let (mut vm, id) = init_vm(&init_catalog(p), "root");
    let o = vm.begin_initialize(id).unwrap();
    assert_eq!(
        vm.start(o, 0, vec![]).unwrap_err().kind,
        VmErrorKind::InvalidState
    );
    assert_eq!(
        vm.begin_initialize(id).unwrap_err().kind,
        VmErrorKind::InvalidState
    );
    assert_eq!(
        vm.run(2).unwrap(),
        RunOutcome::Finished(Completion::Initialized(o))
    );
    vm.start(o, 0, vec![]).unwrap();
    assert_eq!(
        vm.begin_initialize(id).unwrap_err().kind,
        VmErrorKind::InvalidState
    );
    assert_eq!(
        vm.run(100).unwrap(),
        RunOutcome::Finished(Completion::Returned(Value::Void))
    );
}

fn initializing_family() -> rml32::ProgramCatalog {
    let mut a = image(&[], 1, &[b"a\0"]);
    set_init(
        &mut a,
        &[
            (0, 0, 0x40),
            (0, 0, 0x4e),
            (0, 0, 0x33),
            (0, 0, 0x48),
            (0, 0, 0x1d),
        ],
    );
    let mut b = a.clone();
    b.strings = vec![Some(ByteString::new(b"b\0"))];
    let mut c = a.clone();
    c.strings = vec![Some(ByteString::new(b"root\0"))];
    c.max_vars = 3;
    c.constitution_count = Some(3);
    inherit(&mut c, &a, b"a", [1, 0, 1]);
    inherit(&mut c, &b, b"b", [2, 0, 2]);
    let mut catalog = rml32::ProgramCatalog::new();
    catalog.insert("a", a).unwrap();
    catalog.insert("b", b).unwrap();
    catalog.insert("root", c).unwrap();
    catalog.bind_inherit("root", b"a", "a");
    catalog.bind_inherit("root", b"b", "b");
    catalog
}
#[test]
fn initialization_multiple_parents_share_root_storage_and_keep_source_pools() {
    let (mut vm, id) = init_vm(&initializing_family(), "root");
    let o = vm.begin_initialize(id).unwrap();
    assert_eq!(vm.run(4).unwrap(), RunOutcome::BudgetExhausted);
    assert_eq!(
        object_values(&vm, o),
        vec![Value::Undefined, Value::bytes(b"a"), Value::Undefined]
    );
    assert_eq!(vm.run(4).unwrap(), RunOutcome::BudgetExhausted);
    assert_eq!(
        object_values(&vm, o),
        vec![Value::Undefined, Value::bytes(b"a"), Value::bytes(b"b")]
    );
    assert_eq!(
        vm.run(4).unwrap(),
        RunOutcome::Finished(Completion::Initialized(o))
    );
    assert_eq!(
        object_values(&vm, o),
        vec![
            Value::bytes(b"root"),
            Value::bytes(b"a"),
            Value::bytes(b"b")
        ]
    );
    let other = vm.begin_initialize(id).unwrap();
    assert_eq!(object_values(&vm, other), vec![Value::Undefined; 3]);
    vm.run(100).unwrap();
    assert_eq!(object_values(&vm, o), object_values(&vm, other));
}
#[test]
fn initialization_multilevel_relocates_calls_variables_and_metadata() {
    let mut base = image(&[fun(0, 0, 0, 255, &[(83, 0, 0x3b)])], 1, &[]);
    set_init(
        &mut base,
        &[(0, 0, 0x07), (0, 0, 0x4e), (0, 0, 0x33), (0, 0, 0x48)],
    );
    let mut mid = image(&[], 2, &[]);
    mid.max_functions = 2;
    mid.constitution_count = Some(2);
    inherit(&mut mid, &base, b"base", [1, 1, 1]);
    set_init(
        &mut mid,
        &[(0, 0, 0x3c), (0, 0, 0x4e), (0, 0, 0x33), (0, 0, 0x48)],
    );
    mid.init.metadata_hex = rml32::hex(&17i32.to_le_bytes());
    let mut root = image(&[], 3, &[]);
    root.max_functions = 3;
    root.constitution_count = Some(3);
    inherit(&mut root, &mid, b"middle", [1, 1, 1]);
    set_init(
        &mut root,
        &[
            (1, 0, 0x3d),
            (2, 0, 0x3d),
            (0, 0, 0x01),
            (0, 0, 0x4e),
            (0, 0, 0x33),
            (0, 0, 0x48),
        ],
    );
    let mut c = rml32::ProgramCatalog::new();
    c.insert("base", base).unwrap();
    c.insert("mid", mid).unwrap();
    c.insert("root", root).unwrap();
    c.bind_inherit("mid", b"base", "base");
    c.bind_inherit("root", b"middle", "mid");
    let (mut vm, id) = init_vm(&c, "root");
    let o = vm.begin_initialize(id).unwrap();
    let mut ticks = 0;
    loop {
        ticks += 1;
        if vm.run(1).unwrap() != RunOutcome::BudgetExhausted {
            break;
        }
        assert!(ticks < 40);
    }
    assert_eq!(
        object_values(&vm, o),
        vec![Value::Int(100), Value::Int(17), Value::Int(83)]
    );
    assert_eq!(ticks, 18);
}
#[test]
fn initialization_failure_in_each_constitution_cleans_frames_and_preserves_source() {
    for (key, index) in [("a", 1), ("b", 2), ("root", 0)] {
        // Rebuild explicitly to change one source's code; catalog remains opaque.
        let mut a = image(&[], 0, &[]);
        set_init(&mut a, &[(0, 0, 0x2e), (0, 0, 0x48)]);
        let mut b = a.clone();
        let mut root = a.clone();
        root.constitution_count = Some(3);
        inherit(&mut root, &a, b"a", [1, 0, 0]);
        inherit(&mut root, &b, b"b", [2, 0, 0]);
        let failing = match key {
            "a" => &mut a,
            "b" => &mut b,
            _ => &mut root,
        };
        set_init(failing, &[(0, 0, 0x33), (0, 0, 0x48)]);
        let mut c = rml32::ProgramCatalog::new();
        for (k, p) in [("a", a), ("b", b), ("root", root)] {
            c.insert(k, p).unwrap();
        }
        c.bind_inherit("root", b"a", "a");
        c.bind_inherit("root", b"b", "b");
        let (mut vm, id) = init_vm(&c, "root");
        let o = vm.begin_initialize(id).unwrap();
        let e = vm.run(100).unwrap_err();
        assert_eq!(e.kind, VmErrorKind::StackUnderflow);
        assert_eq!(e.source, key);
        assert_eq!(e.initialization, Some(index));
        assert_eq!(e.function, None);
        assert_eq!(e.pc, Some(0));
        assert_eq!(e.offset, Some(9000));
        assert!(vm.objects().get(o).is_err());
        assert_eq!(vm.frame_depth(), 0);
    }
}
#[test]
fn initialization_calls_reference_and_optional_functions_then_restores_context() {
    let mut defaults = optional_fun(1, 1, 0, 255, &[vec![(7, 0, 0x3b)]], &[(0, 0, 0x37)]);
    defaults.meta = vec![];
    let mut p = image(
        &[
            fun(0, 1, 0, 1, &[(9, 0, 0x3b), (0, 0, 0x4d), (0, 0, 0x33)]),
            defaults,
        ],
        2,
        &[],
    );
    ref_arg(&mut p, 0, 0, 2, 0);
    optional_meta(&mut p, 1, 0, false, &[1]);
    set_init(
        &mut p,
        &[
            (2, 0, 0x3b),
            (0, 0, 0x4e),
            (0, 0, 0x33),
            (0, 0, 0x3e),
            (0, 1, 0x07),
            (0, 0, 0x33),
            (1, 0, 0x07),
            (1, 0, 0x4e),
            (0, 0, 0x33),
            (0, 0, 0x48),
        ],
    );
    let (mut vm, id) = init_vm(&init_catalog(p), "root");
    let o = vm.begin_initialize(id).unwrap();
    assert_eq!(
        vm.run(100).unwrap(),
        RunOutcome::Finished(Completion::Initialized(o))
    );
    assert_eq!(object_values(&vm, o), vec![Value::Int(9), Value::Int(7)]);
}
#[test]
fn initialization_nested_error_retains_function_diagnostic_and_task_constitution() {
    let mut p = image(&[fun(0, 0, 0, 255, &[(0, 0, 0x33)])], 0, &[]);
    set_init(&mut p, &[(0, 0, 0x07), (0, 0, 0x48)]);
    let mut root = image(&[], 0, &[]);
    root.max_functions = 1;
    root.constitution_count = Some(2);
    inherit(&mut root, &p, b"parent\0", [1, 0, 0]);
    set_init(&mut root, &[(0, 0, 0x48)]);
    let c = linked_catalog(p, root);
    let (mut vm, id) = init_vm(&c, "child");
    let o = vm.begin_initialize(id).unwrap();
    let e = vm.run(100).unwrap_err();
    assert_eq!(e.source, "parent");
    assert_eq!(e.function, Some(0));
    assert_eq!(e.initialization, Some(1));
    assert_eq!(e.pc, Some(2));
    assert!(e.offset.unwrap() < 9000);
    assert!(vm.objects().get(o).is_err());
}
#[test]
fn initialization_preparation_reports_source_operand_metadata_and_branch_bounds() {
    for case in 0..4 {
        let mut p = image(&[], 0, &[]);
        set_init(
            &mut p,
            &[
                (
                    0,
                    0,
                    match case {
                        0 => 0x40,
                        1 => 0x3d,
                        2 => 0x3c,
                        _ => 0x1f,
                    },
                ),
                (0, 0, 0x48),
            ],
        );
        if case == 3 {
            p.init.nodes[0].0 = 9;
        }
        let e = prepare_init(&init_catalog(p), "root").unwrap_err();
        assert_eq!(e.kind, VmErrorKind::InvalidProgram);
        assert_eq!(e.source, "root");
        assert_eq!(e.initialization, Some(0));
        assert_eq!(e.pc, Some(0));
        assert_eq!(e.offset, Some(9000));
    }
}
#[test]
fn initialization_frame_and_stack_limits_fail_without_leaving_owned_objects() {
    let mut p = image(&[], 0, &[]);
    set_init(&mut p, &[(1, 0, 0x3b), (0, 0, 0x33), (0, 0, 0x48)]);
    let prepared = prepare_init(&init_catalog(p), "root").unwrap();
    let mut store = ObjectStore::new();
    let id = store.register_program(prepared).unwrap();
    let mut vm = Vm::with_limits(
        store,
        VmLimits {
            max_frames: 0,
            ..Default::default()
        },
    );
    assert_eq!(
        vm.begin_initialize(id).unwrap_err().kind,
        VmErrorKind::Limit
    );
    let mut vm = Vm::with_limits(
        vm.into_objects(),
        VmLimits {
            max_stack: 0,
            ..Default::default()
        },
    );
    let o = vm.begin_initialize(id).unwrap();
    assert_eq!(vm.run(1).unwrap_err().kind, VmErrorKind::Limit);
    assert!(vm.objects().get(o).is_err());
}

#[test]
fn initialization_never_runs_lifecycle_functions_or_allows_explicit_lifecycle_call() {
    let mut p = image(
        &[
            fun(0, 0, 0, 1, &[(99, 0, 0x3b), (0, 0, 0x4e), (0, 0, 0x33)]),
            fun(1, 0, 0, 1, &[]),
        ],
        1,
        &[],
    );
    p.functions[0].name = Some(ByteString::new(b"create\0"));
    p.functions[1].name = Some(ByteString::new(b"destruct\0"));
    set_init(
        &mut p,
        &[(7, 0, 0x3b), (0, 0, 0x4e), (0, 0, 0x33), (0, 0, 0x48)],
    );
    let (mut vm, id) = init_vm(&init_catalog(p), "root");
    let o = vm.begin_initialize(id).unwrap();
    vm.run(100).unwrap();
    assert_eq!(object_values(&vm, o), vec![Value::Int(7)]);
    for slot in [0, 1] {
        assert_eq!(
            vm.start(o, slot, vec![]).unwrap_err().kind,
            VmErrorKind::UnsupportedFeature
        );
    }
    assert_eq!(
        vm.run(0).unwrap(),
        RunOutcome::Finished(Completion::Initialized(o))
    );
}
#[test]
fn initialization_variadic_call_and_numeric_comparison_keep_task_context() {
    let mut p = image(&[optional_fun(0, 0, 0, 255, &[], &[(0, 0, 0x34)])], 1, &[]);
    optional_meta(&mut p, 0, 0, true, &[]);
    set_init(
        &mut p,
        &[
            (3, 0, 0x3b),
            (4, 0, 0x3b),
            (0, 2, 0x07),
            (2, 0, 0x3b),
            (0, 0, 0x1c),
            (0, 0, 0x4e),
            (0, 0, 0x33),
            (0, 0, 0x48),
        ],
    );
    let (mut vm, id) = init_vm(&init_catalog(p), "root");
    let o = vm.begin_initialize(id).unwrap();
    assert_eq!(
        vm.run(100).unwrap(),
        RunOutcome::Finished(Completion::Initialized(o))
    );
    assert_eq!(object_values(&vm, o), vec![Value::Int(1)]);
}
#[test]
fn initialization_failure_releases_temporary_reference_quota_for_new_tasks() {
    let mut p = image(
        &[fun(
            0,
            0,
            0,
            255,
            &[(0, 0, 0x16), (0, 0, 0x13), (0, 0, 0x33), (7, 0, 0x3b)],
        )],
        0,
        &[],
    );
    set_init(&mut p, &[(0, 0, 0x16), (0, 0, 0x13), (0, 0, 0x48)]);
    let (vm, id) = init_vm(&init_catalog(p), "root");
    let mut vm = Vm::with_limits(
        vm.into_objects(),
        VmLimits {
            max_reference_cells: 1,
            ..Default::default()
        },
    );
    for _ in 0..2 {
        let o = vm.begin_initialize(id).unwrap();
        assert_eq!(vm.run(100).unwrap_err().kind, VmErrorKind::InvalidProgram);
        assert!(vm.objects().get(o).is_err());
    }
    let manual = vm.objects_mut().create(id, vec![]).unwrap();
    vm.start(manual, 0, vec![]).unwrap();
    assert_eq!(
        vm.run(100).unwrap(),
        RunOutcome::Finished(Completion::Returned(Value::Int(7)))
    );
}
#[test]
fn initialization_nested_call_frame_limit_cleans_owned_object() {
    let mut p = image(&[fun(0, 0, 0, 255, &[])], 0, &[]);
    set_init(&mut p, &[(0, 0, 0x07), (0, 0, 0x33), (0, 0, 0x48)]);
    let (vm, id) = init_vm(&init_catalog(p), "root");
    let mut vm = Vm::with_limits(
        vm.into_objects(),
        VmLimits {
            max_frames: 1,
            ..Default::default()
        },
    );
    let o = vm.begin_initialize(id).unwrap();
    let e = vm.run(1).unwrap_err();
    assert_eq!(e.kind, VmErrorKind::Limit);
    assert_eq!(e.initialization, Some(0));
    assert_eq!(e.function, None);
    assert!(vm.objects().get(o).is_err());
    assert_eq!(vm.frame_depth(), 0);
}
#[test]
fn initialization_loops_obey_budget_and_global_input_is_rejected() {
    let mut p = image(&[], 0, &[]);
    set_init(&mut p, &[(-1, 0, 0x1f), (0, 0, 0x48)]);
    let (mut vm, id) = init_vm(&init_catalog(p.clone()), "root");
    let o = vm.begin_initialize(id).unwrap();
    for _ in 0..3 {
        assert_eq!(vm.run(5).unwrap(), RunOutcome::BudgetExhausted);
        assert_eq!(
            vm.objects().get(o).unwrap().initialization_state(),
            rml32::InitializationState::Initializing
        );
    }
    assert!(vm.into_objects().get(o).is_err());
    p.mode = ProgramMode::Global;
    assert_eq!(
        prepare_init(&init_catalog(p), "root").unwrap_err().kind,
        VmErrorKind::UnsupportedFeature
    );
}

// Stage 2O lifecycle fixtures: shared mapping is an observable synthetic trace.
fn lifecycle_name(p: &mut ProgramImage, position: usize, name: &[u8]) {
    p.functions[position].name = Some(ByteString::new(name));
}
fn lifecycle_log(marker: i16) -> Vec<Node> {
    vec![
        (0, 0, 0x3e),
        (0, 0, 0x3b),
        (0, 0, 0x44),
        (1, 0, 0x22),
        (0, 0, 0x3e),
        (marker, 0, 0x3b),
        (0, 0, 0x44),
        (0, 0, 0x3d),
        (0, 0, 0x3b),
        (0, 0, 0x43),
        (0, 0, 0x04),
        (0, 0, 0x33),
    ]
}
fn lifecycle_image(
    create_index: i16,
    destruct_index: i16,
    marker: i16,
    vars: usize,
    fail_create: bool,
    fail_destruct: bool,
) -> ProgramImage {
    let mut create = vec![(0, 0, 0x37), (0, 0, 0x4e), (0, 0, 0x33)];
    create.extend(lifecycle_log(marker));
    if fail_create {
        create.push((0, 0, 0x33));
    }
    let mut destruct = lifecycle_log(marker + 10);
    if fail_destruct {
        destruct.push((0, 0, 0x33));
    }
    let mut p = image(
        &[
            fun(create_index, 1, 0, 1, &create),
            fun(destruct_index, 0, 0, 1, &destruct),
        ],
        vars,
        &[],
    );
    lifecycle_name(&mut p, 0, b"create\0");
    lifecycle_name(&mut p, 1, b"destruct\0");
    set_init(
        &mut p,
        &[(7, 0, 0x3b), (0, 0, 0x4e), (0, 0, 0x33), (0, 0, 0x48)],
    );
    p
}
fn construct_family(fail: Option<usize>, destruct_fail: bool) -> rml32::ProgramCatalog {
    let a = lifecycle_image(0, 1, 1, 1, fail == Some(0), destruct_fail);
    let b = lifecycle_image(0, 1, 2, 1, fail == Some(1), destruct_fail);
    let mut root = lifecycle_image(4, 5, 3, 3, fail == Some(2), destruct_fail);
    root.constitution_count = Some(3);
    inherit(&mut root, &a, b"a", [1, 0, 1]);
    inherit(&mut root, &b, b"b", [2, 2, 2]);
    let mut c = rml32::ProgramCatalog::new();
    for (key, p, create, destruct) in [("a", a, 0, 1), ("b", b, 0, 1), ("root", root, 4, 5)] {
        c.insert(key, p).unwrap();
        c.bind_lifecycle(
            key,
            rml32::LifecycleBinding {
                create: Some(create),
                destruct: Some(destruct),
            },
        )
        .unwrap();
    }
    c.bind_inherit("root", b"a", "a");
    c.bind_inherit("root", b"b", "b");
    c
}
fn construct_vm(c: &rml32::ProgramCatalog, root: &str) -> (Vm, rml32::ProgramId) {
    let p = rml32::prepare_constructible_program(c, root, Default::default()).unwrap();
    assert!(p.is_constructible());
    assert!(p.is_initializable());
    let mut s = ObjectStore::new();
    let id = s.register_program(p).unwrap();
    (Vm::new(s), id)
}
fn trace_mapping() -> Value {
    Value::mapping(vec![(Value::Int(0), Value::Int(0))]).unwrap()
}
fn trace_value(trace: &Value, key: i32) -> Option<Value> {
    let Value::Mapping(m) = trace else {
        unreachable!()
    };
    m.get(&Value::Int(key)).unwrap()
}
#[test]
fn construction_initializes_all_sources_then_runs_creates_in_order() {
    let (mut vm, id) = construct_vm(&construct_family(None, false), "root");
    let trace = trace_mapping();
    let o = vm.begin_construct(id, Some(trace.clone())).unwrap();
    assert_eq!(
        vm.objects().get(o).unwrap().lifecycle_state(),
        rml32::LifecycleState::Constructing
    );
    assert_eq!(vm.run(12).unwrap(), RunOutcome::BudgetExhausted);
    assert_eq!(object_values(&vm, o), vec![Value::Int(7); 3]);
    assert_eq!(
        vm.objects().get(o).unwrap().initialization_state(),
        rml32::InitializationState::Initialized
    );
    assert_eq!(trace_value(&trace, 0), Some(Value::Int(0)));
    loop {
        if vm.run(1).unwrap() != RunOutcome::BudgetExhausted {
            break;
        }
    }
    assert_eq!(
        vm.run(0).unwrap(),
        RunOutcome::Finished(Completion::Constructed(o))
    );
    assert_eq!(
        vm.objects().get(o).unwrap().lifecycle_state(),
        rml32::LifecycleState::Constructed
    );
    for marker in 1..=3 {
        assert_eq!(trace_value(&trace, marker), Some(Value::Int(marker)));
        assert_eq!(trace_value(&trace, marker + 10), None);
    }
    assert!(vm.rollback_errors().is_empty());
    assert!(vm.stack().unwrap().is_empty());
    assert_eq!(
        vm.start(o, 4, vec![trace]).unwrap_err().kind,
        VmErrorKind::UnsupportedFeature
    );
}
#[test]
fn construction_failure_rolls_back_failing_layer_and_previous_layers_in_reverse() {
    for failed in 0..3 {
        let (mut vm, id) = construct_vm(&construct_family(Some(failed), false), "root");
        let trace = trace_mapping();
        let o = vm.begin_construct(id, Some(trace.clone())).unwrap();
        let e = vm.run(1000).unwrap_err();
        assert_eq!(e.kind, VmErrorKind::StackUnderflow);
        assert_eq!(
            e.lifecycle.as_ref().unwrap().phase,
            rml32::LifecyclePhase::Create
        );
        assert_eq!(
            e.lifecycle.as_ref().unwrap().constitution,
            [1, 2, 0][failed]
        );
        assert_eq!(e.source, ["a", "b", "root"][failed]);
        assert_eq!(e.function, Some([0, 2, 4][failed]));
        assert!(vm.objects().get(o).is_err());
        for k in 0..=failed {
            assert_eq!(
                trace_value(&trace, k as i32 + 1),
                Some(Value::Int(k as i32 + 1))
            );
            assert_eq!(
                trace_value(&trace, k as i32 + 11),
                Some(Value::Int((failed + 1 + failed - k + 1) as i32))
            );
        }
        assert!(vm.rollback_errors().is_empty());
        assert_eq!(vm.run(0).unwrap_err(), e);
        assert_eq!(vm.frame_depth(), 0);
    }
}
#[test]
fn construction_preserves_primary_error_and_continues_after_multiple_destruct_failures() {
    let (mut vm, id) = construct_vm(&construct_family(Some(2), true), "root");
    let trace = trace_mapping();
    let o = vm.begin_construct(id, Some(trace.clone())).unwrap();
    let e = vm.run(1000).unwrap_err();
    assert_eq!(
        e.lifecycle.as_ref().unwrap().phase,
        rml32::LifecyclePhase::Create
    );
    assert_eq!(vm.rollback_errors().len(), 3);
    for (err, source) in vm.rollback_errors().iter().zip(["root", "b", "a"]) {
        assert_eq!(err.source, source);
        assert_eq!(
            err.lifecycle.as_ref().unwrap().phase,
            rml32::LifecyclePhase::RollbackDestruct
        );
        assert_eq!(err.kind, VmErrorKind::StackUnderflow);
    }
    assert_eq!(trace_value(&trace, 13), Some(Value::Int(4)));
    assert_eq!(trace_value(&trace, 12), Some(Value::Int(5)));
    assert_eq!(trace_value(&trace, 11), Some(Value::Int(6)));
    assert!(vm.objects().get(o).is_err());
    let again = vm.begin_construct(id, Some(trace_mapping())).unwrap();
    assert_ne!(again, o);
    assert!(vm.rollback_errors().is_empty());
}
#[test]
fn construction_rollback_can_pause_and_host_cancellation_does_not_run_more_destructors() {
    let (mut vm, id) = construct_vm(&construct_family(Some(1), false), "root");
    let trace = trace_mapping();
    let o = vm.begin_construct(id, Some(trace.clone())).unwrap();
    loop {
        assert_eq!(vm.run(1).unwrap(), RunOutcome::BudgetExhausted);
        if vm.objects().get(o).unwrap().lifecycle_state() == rml32::LifecycleState::RollingBack {
            break;
        }
    }
    assert_eq!(vm.run(0).unwrap(), RunOutcome::BudgetExhausted);
    assert_eq!(trace_value(&trace, 12), None);
    assert_eq!(
        vm.start(o, 4, vec![]).unwrap_err().kind,
        VmErrorKind::InvalidState
    );
    assert_eq!(
        vm.begin_construct(id, None).unwrap_err().kind,
        VmErrorKind::InvalidState
    );
    let s = vm.into_objects();
    assert!(s.get(o).is_err());
    assert_eq!(trace_value(&trace, 12), None);
}
#[test]
fn construction_external_delete_preserves_primary_during_rollback() {
    let (mut vm, id) = construct_vm(&construct_family(Some(0), false), "root");
    let o = vm.begin_construct(id, Some(trace_mapping())).unwrap();
    while vm.objects().get(o).unwrap().lifecycle_state() != rml32::LifecycleState::RollingBack {
        vm.run(1).unwrap();
    }
    vm.objects_mut().remove(o).unwrap();
    let e = vm.run(1).unwrap_err();
    assert_eq!(e.kind, VmErrorKind::StackUnderflow);
    assert_eq!(
        e.lifecycle.as_ref().unwrap().phase,
        rml32::LifecyclePhase::Create
    );
    assert_eq!(
        vm.rollback_errors().last().unwrap().kind,
        VmErrorKind::InvalidObject
    );
    assert_eq!(vm.frame_depth(), 0);
}
#[test]
fn lifecycle_bindings_are_required_and_none_cannot_hide_local_functions() {
    let p = lifecycle_image(0, 1, 1, 1, false, false);
    let mut c = init_catalog(p.clone());
    assert!(
        rml32::prepare_constructible_program(&c, "root", Default::default())
            .unwrap_err()
            .message
            .contains("missing")
    );
    c.bind_lifecycle("root", Default::default()).unwrap();
    assert!(c.bind_lifecycle("root", Default::default()).is_err());
    assert!(
        rml32::prepare_constructible_program(&c, "root", Default::default())
            .unwrap_err()
            .message
            .contains("hides")
    );
    for binding in [
        rml32::LifecycleBinding {
            create: Some(1),
            destruct: Some(1),
        },
        rml32::LifecycleBinding {
            create: Some(-1),
            destruct: Some(1),
        },
        rml32::LifecycleBinding {
            create: Some(7),
            destruct: Some(1),
        },
    ] {
        let mut c = init_catalog(p.clone());
        c.bind_lifecycle("root", binding).unwrap();
        assert!(rml32::prepare_constructible_program(&c, "root", Default::default()).is_err());
    }
}
#[test]
fn lifecycle_preparation_rejects_declarations_atomic_and_unsupported_prototypes() {
    for case in 0..6 {
        let mut p = lifecycle_image(0, 1, 1, 1, false, false);
        match case {
            0 => p.functions[0].code.nodes.clear(),
            1 => p.functions[0].attrib |= 2,
            2 => p.functions[0].return_type = 255,
            3 => ref_arg(&mut p, 0, 0, 255, 0),
            4 => p.functions[0].arguments[0].r#type = 2,
            _ => p.functions[1].max_args = 1,
        };
        let mut c = init_catalog(p);
        c.bind_lifecycle(
            "root",
            rml32::LifecycleBinding {
                create: Some(0),
                destruct: Some(1),
            },
        )
        .unwrap();
        assert!(
            rml32::prepare_constructible_program(&c, "root", Default::default()).is_err(),
            "case {case}"
        );
    }
}

fn single_construct_catalog(
    p: ProgramImage,
    binding: rml32::LifecycleBinding,
) -> rml32::ProgramCatalog {
    let mut c = init_catalog(p);
    c.bind_lifecycle("root", binding).unwrap();
    c
}
#[test]
fn construction_without_lifecycle_methods_finishes_after_initialization() {
    let mut p = image(&[], 1, &[]);
    set_init(&mut p, &[(0, 0, 0x48)]);
    let c = single_construct_catalog(p, Default::default());
    let (mut vm, id) = construct_vm(&c, "root");
    let o = vm.begin_construct(id, None).unwrap();
    assert_eq!(vm.run(0).unwrap(), RunOutcome::BudgetExhausted);
    assert_eq!(
        vm.run(1).unwrap(),
        RunOutcome::Finished(Completion::Constructed(o))
    );
    assert_eq!(object_values(&vm, o), vec![Value::Undefined]);
    let store = vm.into_objects();
    assert_eq!(
        store.get(o).unwrap().lifecycle_state(),
        rml32::LifecycleState::Constructed
    );
}
#[test]
fn construction_skipped_create_layer_still_participates_in_failure_rollback() {
    let mut parent = image(&[fun(0, 0, 0, 1, &[(0, 0, 0x33)])], 0, &[]);
    lifecycle_name(&mut parent, 0, b"destruct");
    set_init(&mut parent, &[(0, 0, 0x48)]);
    let mut root = image(&[fun(1, 0, 0, 1, &[(0, 0, 0x33)])], 0, &[]);
    lifecycle_name(&mut root, 0, b"create");
    set_init(&mut root, &[(0, 0, 0x48)]);
    root.constitution_count = Some(2);
    inherit(&mut root, &parent, b"parent\0", [1, 0, 0]);
    let mut c = linked_catalog(parent, root);
    c.bind_lifecycle(
        "parent",
        rml32::LifecycleBinding {
            create: None,
            destruct: Some(0),
        },
    )
    .unwrap();
    c.bind_lifecycle(
        "child",
        rml32::LifecycleBinding {
            create: Some(1),
            destruct: None,
        },
    )
    .unwrap();
    let (mut vm, id) = construct_vm(&c, "child");
    let o = vm.begin_construct(id, None).unwrap();
    let e = vm.run(100).unwrap_err();
    assert_eq!(e.source, "child");
    assert_eq!(vm.rollback_errors().len(), 1);
    assert_eq!(vm.rollback_errors()[0].source, "parent");
    assert_eq!(vm.rollback_errors()[0].function, Some(0));
    assert_eq!(
        vm.rollback_errors()[0]
            .lifecycle
            .as_ref()
            .unwrap()
            .constitution,
        1
    );
    assert!(vm.objects().get(o).is_err());
}
#[test]
fn construction_zero_declared_parameters_accept_shared_mapping_as_native_extra() {
    let mut p = image(
        &[fun(0, 0, 0, 1, &[(0, 0, 0x34), (0, 0, 0x4e), (0, 0, 0x33)])],
        1,
        &[],
    );
    lifecycle_name(&mut p, 0, b"create");
    set_init(&mut p, &[(0, 0, 0x48)]);
    let c = single_construct_catalog(
        p,
        rml32::LifecycleBinding {
            create: Some(0),
            destruct: None,
        },
    );
    let (mut vm, id) = construct_vm(&c, "root");
    for (parameter, count) in [(None, 0), (Some(trace_mapping()), 1)] {
        let o = vm.begin_construct(id, parameter).unwrap();
        assert_eq!(
            vm.run(100).unwrap(),
            RunOutcome::Finished(Completion::Constructed(o))
        );
        assert_eq!(object_values(&vm, o), vec![Value::Int(count)]);
    }
}
#[test]
fn construction_optional_mapping_defaults_and_required_argument_entry_failure() {
    let mut p = image(
        &[optional_fun(
            0,
            1,
            0,
            1,
            &[vec![(0, 0, 0x18)]],
            &[(0, 0, 0x37), (0, 0, 0x4e), (0, 0, 0x33)],
        )],
        1,
        &[],
    );
    optional_meta(&mut p, 0, 0, false, &[1]);
    p.functions[0].arguments[0].r#type = 7;
    lifecycle_name(&mut p, 0, b"create");
    set_init(&mut p, &[(0, 0, 0x48)]);
    let c = single_construct_catalog(
        p,
        rml32::LifecycleBinding {
            create: Some(0),
            destruct: None,
        },
    );
    let (mut vm, id) = construct_vm(&c, "root");
    let o = vm.begin_construct(id, None).unwrap();
    assert_eq!(
        vm.run(100).unwrap(),
        RunOutcome::Finished(Completion::Constructed(o))
    );
    let Value::Mapping(m) = &object_values(&vm, o)[0] else {
        panic!("mapping expected")
    };
    assert_eq!(m.entries().unwrap().len(), 0);
    let mut p = lifecycle_image(0, 1, 1, 1, false, false);
    p.functions[1].code.nodes = vec![(0, 0, 0x0f), (0, 0, 0x1b), (0, 0, 0x33), (0, 0, 0x49)];
    let c = single_construct_catalog(
        p,
        rml32::LifecycleBinding {
            create: Some(0),
            destruct: Some(1),
        },
    );
    let (mut vm, id) = construct_vm(&c, "root");
    let o = vm.begin_construct(id, None).unwrap();
    let e = vm.run(100).unwrap_err();
    assert_eq!(e.kind, VmErrorKind::ArgumentCount);
    assert_eq!(e.function, Some(0));
    assert_eq!(e.pc, None);
    assert_eq!(e.offset, None);
    assert_eq!(
        e.lifecycle.as_ref().unwrap().phase,
        rml32::LifecyclePhase::Create
    );
    assert_eq!(vm.rollback_errors().len(), 1);
    assert!(vm.objects().get(o).is_err());
}
#[test]
fn construction_initialization_failure_never_runs_destruct_and_retains_context() {
    let mut p = lifecycle_image(0, 1, 1, 1, false, true);
    set_init(&mut p, &[(0, 0, 0x33), (0, 0, 0x48)]);
    let c = single_construct_catalog(
        p,
        rml32::LifecycleBinding {
            create: Some(0),
            destruct: Some(1),
        },
    );
    let (mut vm, id) = construct_vm(&c, "root");
    let manual = vm.objects_mut().create(id, vec![Value::Int(42)]).unwrap();
    let trace = trace_mapping();
    let o = vm.begin_construct(id, Some(trace.clone())).unwrap();
    let e = vm.run(1).unwrap_err();
    assert_eq!(
        e.lifecycle.as_ref().unwrap().phase,
        rml32::LifecyclePhase::Initialize
    );
    assert_eq!(e.initialization, Some(0));
    assert!(vm.rollback_errors().is_empty());
    assert_eq!(trace_value(&trace, 0), Some(Value::Int(0)));
    assert!(vm.objects().get(o).is_err());
    assert_eq!(object_values(&vm, manual), vec![Value::Int(42)]);
}
#[test]
fn construction_user_calls_cannot_enter_lifecycle_functions() {
    for target in [0, 1] {
        let mut p = lifecycle_image(0, 1, 1, 1, false, false);
        let wrapper = fun(2, 0, 0, 1, &[(target, 0, 0x07), (0, 0, 0x33)]);
        let mut extra = image(&[wrapper], 1, &[]);
        p.functions.push(extra.functions.remove(0));
        p.max_functions = 3;
        let c = single_construct_catalog(
            p,
            rml32::LifecycleBinding {
                create: Some(0),
                destruct: Some(1),
            },
        );
        let (mut vm, id) = construct_vm(&c, "root");
        let o = vm.objects_mut().create(id, vec![trace_mapping()]).unwrap();
        vm.start(o, 2, vec![]).unwrap();
        assert_eq!(
            vm.run(100).unwrap_err().kind,
            VmErrorKind::UnsupportedFeature
        );
    }
}
#[test]
fn construction_preflight_errors_do_not_change_prior_completion_or_objects() {
    let (mut vm, id) = construct_vm(&construct_family(None, false), "root");
    let o = vm.begin_construct(id, Some(trace_mapping())).unwrap();
    vm.run(1000).unwrap();
    assert_eq!(
        vm.begin_construct(id, Some(Value::Int(3)))
            .unwrap_err()
            .kind,
        VmErrorKind::UnsupportedType
    );
    assert_eq!(
        vm.run(0).unwrap(),
        RunOutcome::Finished(Completion::Constructed(o))
    );
    let plain = vm
        .objects_mut()
        .register_program(prepare_program(image(&[], 0, &[])).unwrap())
        .unwrap();
    assert_eq!(
        vm.begin_construct(plain, None).unwrap_err().kind,
        VmErrorKind::UnsupportedFeature
    );
    assert_eq!(
        vm.objects().get(o).unwrap().lifecycle_state(),
        rml32::LifecycleState::Constructed
    );
}

#[test]
fn construction_nested_failure_unwinds_temporaries_and_allows_rollback_with_same_quota() {
    let mut p = image(
        &[
            fun(0, 0, 0, 1, &[(0, 0, 0x16), (0, 0, 0x13), (2, 0, 0x07)]),
            fun(1, 0, 0, 1, &[(0, 0, 0x16), (0, 0, 0x13), (0, 0, 0x33)]),
            fun(2, 0, 0, 1, &[(0, 0, 0x33)]),
        ],
        0,
        &[],
    );
    lifecycle_name(&mut p, 0, b"create");
    lifecycle_name(&mut p, 1, b"destruct");
    set_init(&mut p, &[(0, 0, 0x48)]);
    let c = single_construct_catalog(
        p,
        rml32::LifecycleBinding {
            create: Some(0),
            destruct: Some(1),
        },
    );
    let (vm, id) = construct_vm(&c, "root");
    let mut vm = Vm::with_limits(
        vm.into_objects(),
        VmLimits {
            max_reference_cells: 1,
            ..Default::default()
        },
    );
    for _ in 0..2 {
        let o = vm.begin_construct(id, None).unwrap();
        let e = vm.run(100).unwrap_err();
        assert_eq!(e.kind, VmErrorKind::StackUnderflow);
        assert_eq!(e.function, Some(2));
        assert_eq!(
            e.lifecycle.as_ref().unwrap().phase,
            rml32::LifecyclePhase::Create
        );
        assert!(vm.rollback_errors().is_empty());
        assert!(vm.objects().get(o).is_err());
        assert_eq!(vm.frame_depth(), 0);
        assert!(vm.stack().unwrap().is_empty());
    }
}
#[test]
fn construction_frame_limit_failure_still_executes_root_rollback() {
    let mut p = image(
        &[
            fun(0, 0, 0, 1, &[(2, 0, 0x07)]),
            fun(1, 0, 0, 1, &[]),
            fun(2, 0, 0, 1, &[]),
        ],
        0,
        &[],
    );
    lifecycle_name(&mut p, 0, b"create");
    lifecycle_name(&mut p, 1, b"destruct");
    set_init(&mut p, &[(0, 0, 0x48)]);
    let c = single_construct_catalog(
        p,
        rml32::LifecycleBinding {
            create: Some(0),
            destruct: Some(1),
        },
    );
    let (vm, id) = construct_vm(&c, "root");
    let mut vm = Vm::with_limits(
        vm.into_objects(),
        VmLimits {
            max_frames: 1,
            ..Default::default()
        },
    );
    let o = vm.begin_construct(id, None).unwrap();
    let e = vm.run(100).unwrap_err();
    assert_eq!(e.kind, VmErrorKind::Limit);
    assert_eq!(e.function, Some(0));
    assert!(vm.rollback_errors().is_empty());
    assert!(vm.objects().get(o).is_err());
}
#[test]
fn construction_rollback_resume_finishes_with_original_error_and_retains_shared_writes() {
    let (mut vm, id) = construct_vm(&construct_family(Some(1), false), "root");
    let trace = trace_mapping();
    let o = vm.begin_construct(id, Some(trace.clone())).unwrap();
    while vm.objects().get(o).unwrap().lifecycle_state() != rml32::LifecycleState::RollingBack {
        assert_eq!(vm.run(1).unwrap(), RunOutcome::BudgetExhausted);
    }
    assert_eq!(vm.run(0).unwrap(), RunOutcome::BudgetExhausted);
    let e = loop {
        match vm.run(1) {
            Ok(RunOutcome::BudgetExhausted) => {}
            Err(e) => break e,
            other => panic!("unexpected {other:?}"),
        }
    };
    assert_eq!(e.source, "b");
    assert_eq!(trace_value(&trace, 12), Some(Value::Int(3)));
    assert_eq!(trace_value(&trace, 11), Some(Value::Int(4)));
    assert!(vm.objects().get(o).is_err());
    assert!(vm.rollback_errors().is_empty());
}
#[test]
fn construction_multilevel_uses_each_source_pool_and_relocated_lifecycle_slots() {
    let mut base = image(
        &[fun(0, 0, 0, 1, &[(0, 0, 0x40), (0, 0, 0x4e), (0, 0, 0x33)])],
        1,
        &[b"base\0"],
    );
    lifecycle_name(&mut base, 0, b"create");
    set_init(&mut base, &[(0, 0, 0x48)]);
    let mut mid = image(
        &[fun(1, 0, 0, 1, &[(0, 0, 0x40), (0, 0, 0x4e), (0, 0, 0x33)])],
        2,
        &[b"mid\0"],
    );
    lifecycle_name(&mut mid, 0, b"create");
    mid.constitution_count = Some(2);
    inherit(&mut mid, &base, b"base", [1, 0, 1]);
    set_init(&mut mid, &[(0, 0, 0x48)]);
    let mut root = image(
        &[fun(2, 0, 0, 1, &[(0, 0, 0x40), (0, 0, 0x4e), (0, 0, 0x33)])],
        3,
        &[b"root\0"],
    );
    lifecycle_name(&mut root, 0, b"create");
    root.constitution_count = Some(3);
    inherit(&mut root, &mid, b"mid", [1, 0, 1]);
    set_init(&mut root, &[(0, 0, 0x48)]);
    let mut c = rml32::ProgramCatalog::new();
    for (key, p, slot) in [("base", base, 0), ("mid", mid, 1), ("root", root, 2)] {
        c.insert(key, p).unwrap();
        c.bind_lifecycle(
            key,
            rml32::LifecycleBinding {
                create: Some(slot),
                destruct: None,
            },
        )
        .unwrap();
    }
    c.bind_inherit("mid", b"base", "base");
    c.bind_inherit("root", b"mid", "mid");
    let (mut vm, id) = construct_vm(&c, "root");
    let o = vm.begin_construct(id, None).unwrap();
    assert_eq!(
        vm.run(100).unwrap(),
        RunOutcome::Finished(Completion::Constructed(o))
    );
    assert_eq!(
        object_values(&vm, o),
        vec![
            Value::bytes(b"root"),
            Value::bytes(b"mid"),
            Value::bytes(b"base")
        ]
    );
}
#[test]
fn construction_external_delete_during_initialization_and_create_is_terminal() {
    for ticks in [0, 12] {
        let (mut vm, id) = construct_vm(&construct_family(None, false), "root");
        let o = vm.begin_construct(id, Some(trace_mapping())).unwrap();
        vm.run(ticks).unwrap();
        vm.objects_mut().remove(o).unwrap();
        let e = vm.run(1).unwrap_err();
        assert_eq!(e.kind, VmErrorKind::InvalidObject);
        assert_eq!(
            e.lifecycle.as_ref().unwrap().phase,
            if ticks == 0 {
                rml32::LifecyclePhase::Initialize
            } else {
                rml32::LifecyclePhase::Create
            }
        );
        assert!(vm.rollback_errors().is_empty());
        assert_eq!(vm.frame_depth(), 0);
    }
}
#[test]
fn construction_opt_in_does_not_change_initialize_only_tasks() {
    let (mut vm, id) = construct_vm(&construct_family(Some(0), true), "root");
    let o = vm.begin_initialize(id).unwrap();
    assert_eq!(
        vm.run(100).unwrap(),
        RunOutcome::Finished(Completion::Initialized(o))
    );
    assert_eq!(
        vm.objects().get(o).unwrap().lifecycle_state(),
        rml32::LifecycleState::Unmanaged
    );
    assert!(vm.rollback_errors().is_empty());
    assert_eq!(object_values(&vm, o), vec![Value::Int(7); 3]);
}
#[test]
fn construction_rejects_missing_dependency_binding_and_inherited_slot_selection() {
    let mut p = lifecycle_image(0, 1, 1, 1, false, false);
    set_init(&mut p, &[(0, 0, 0x48)]);
    let mut root = image(&[], 1, &[]);
    root.max_functions = 2;
    root.constitution_count = Some(2);
    inherit(&mut root, &p, b"parent\0", [1, 0, 0]);
    set_init(&mut root, &[(0, 0, 0x48)]);
    let mut c = linked_catalog(p, root);
    c.bind_lifecycle("child", Default::default()).unwrap();
    let e = rml32::prepare_constructible_program(&c, "child", Default::default()).unwrap_err();
    assert_eq!(e.source, "parent");
    assert!(e.message.contains("missing"));
    c.bind_lifecycle(
        "parent",
        rml32::LifecycleBinding {
            create: Some(0),
            destruct: Some(1),
        },
    )
    .unwrap();
    assert!(rml32::prepare_constructible_program(&c, "child", Default::default()).is_ok());
    let mut root = image(&[], 0, &[]);
    root.max_functions = 2;
    root.constitution_count = Some(2);
    let mut p = lifecycle_image(0, 1, 1, 0, false, false);
    set_init(&mut p, &[(0, 0, 0x48)]);
    inherit(&mut root, &p, b"parent\0", [1, 0, 0]);
    set_init(&mut root, &[(0, 0, 0x48)]);
    let mut c = linked_catalog(p, root);
    c.bind_lifecycle(
        "parent",
        rml32::LifecycleBinding {
            create: Some(0),
            destruct: Some(1),
        },
    )
    .unwrap();
    c.bind_lifecycle(
        "child",
        rml32::LifecycleBinding {
            create: Some(0),
            destruct: None,
        },
    )
    .unwrap();
    assert!(rml32::prepare_constructible_program(&c, "child", Default::default()).is_err());
}

// Stage 2Q: complete method namespaces and all executable inputs are synthetic.
fn method(name: &[u8], slot: i16) -> rml32::MethodBinding {
    rml32::MethodBinding {
        name: name.to_vec(),
        slot,
    }
}
fn other_wrapper(argc: usize, flags: i16) -> Fun {
    let mut nodes: Vec<_> = (0..argc).rev().map(|i| ((i + 2) as i16, 0, 0x37)).collect();
    nodes.extend([(1, 0, 0x37), (0, 0, 0x37), (flags, argc as i8, 0x08)]);
    fun(0, argc + 2, 0, 255, &nodes)
}
fn other_machine(
    caller: ProgramImage,
    target: rml32::PreparedProgram,
    caller_values: Vec<Value>,
    target_values: Vec<Value>,
) -> (Vm, rml32::ObjectId, rml32::ObjectId) {
    let mut store = ObjectStore::new();
    let a = store
        .register_program(prepare_program(caller).unwrap().with_source("caller"))
        .unwrap();
    let b = store
        .register_program(target.with_source("target"))
        .unwrap();
    let a = store.create(a, caller_values).unwrap();
    let b = store.create(b, target_values).unwrap();
    (Vm::new(store), a, b)
}
fn other_target(p: ProgramImage, bindings: Vec<rml32::MethodBinding>) -> rml32::PreparedProgram {
    prepare_program(p).unwrap().with_methods(bindings).unwrap()
}

#[test]
fn other_methods_validate_explicit_names_slots_and_lifecycle_entries() {
    let p = image(&[fun(0, 0, 0, 255, &[])], 0, &[]);
    for bindings in [
        vec![method(b"", 0)],
        vec![method(b"a\0b", 0)],
        vec![method(b"a", -1)],
        vec![method(b"a", 8)],
        vec![method(b"a", 0), method(b"a", 0)],
    ] {
        assert_eq!(
            prepare_program(p.clone())
                .unwrap()
                .with_methods(bindings)
                .unwrap_err()
                .kind,
            VmErrorKind::InvalidProgram
        );
    }
    assert!(
        prepare_program(p.clone())
            .unwrap()
            .with_methods(vec![method(b"a", 0), method(b"A", 0), method(b"\xff", 0)])
            .is_ok()
    );
    for name in [b"create".as_slice(), b"destruct"] {
        assert_eq!(
            prepare_program(p.clone())
                .unwrap()
                .with_methods(vec![method(name, 0)])
                .unwrap_err()
                .kind,
            VmErrorKind::UnsupportedFeature
        );
        let mut named = p.clone();
        lifecycle_name(&mut named, 0, &[name, b"\0"].concat());
        assert_eq!(
            prepare_program(named)
                .unwrap()
                .with_methods(vec![method(b"alias", 0)])
                .unwrap_err()
                .kind,
            VmErrorKind::UnsupportedFeature
        );
    }
    let (mut parent, child) = inherited_pair(&[], &[]);
    parent.functions[0].code.nodes.clear();
    let linked = link(&linked_catalog(parent, child)).unwrap();
    assert_eq!(
        linked
            .with_methods(vec![method(b"missing", 0)])
            .unwrap_err()
            .kind,
        VmErrorKind::InvalidProgram
    );
}

#[test]
fn other_call_changes_this_object_and_restores_caller() {
    let caller = image(
        &[fun(
            0,
            1,
            0,
            255,
            &[
                (0, 0, 0x40),
                (0, 0, 0x37),
                (0, 0, 0x08),
                (0, 0, 0x41),
                (2, 0, 0x16),
            ],
        )],
        0,
        &[b"who\0"],
    );
    let target = other_target(
        image(
            &[
                fun(0, 0, 0, 255, &[(1, 0, 0x07)]),
                fun(1, 0, 0, 255, &[(0, 0, 0x41)]),
            ],
            0,
            &[],
        ),
        vec![method(b"who", 0)],
    );
    let (mut vm, a, b) = other_machine(caller, target, vec![], vec![]);
    vm.start(a, 0, vec![Value::Object(b)]).unwrap();
    assert_eq!(
        returned_array(&mut vm),
        vec![Value::Object(a), Value::Object(b)]
    );
    assert_eq!(vm.frame_depth(), 0);
    assert!(vm.stack().unwrap().is_empty());
}

#[test]
fn other_call_same_program_keeps_instances_and_variable_banks_separate() {
    let p = image(
        &[
            other_wrapper(0, 0),
            fun(1, 0, 0, 255, &[(0, 0, 0x3d), (0, 0, 0x41), (2, 0, 0x16)]),
        ],
        1,
        &[],
    );
    let mut store = ObjectStore::new();
    let id = store
        .register_program(other_target(p, vec![method(b"read", 1)]))
        .unwrap();
    let a = store.create(id, vec![Value::Int(11)]).unwrap();
    let b = store.create(id, vec![Value::Int(22)]).unwrap();
    let mut vm = Vm::new(store);
    vm.start(a, 0, vec![Value::Object(b), Value::bytes(b"read")])
        .unwrap();
    assert_eq!(
        returned_array(&mut vm),
        vec![Value::Object(b), Value::Int(22)]
    );
    assert_eq!(object_values(&vm, a), vec![Value::Int(11)]);
}

#[test]
fn other_call_round_trip_restores_program_pool_and_object() {
    let a = image(
        &[
            fun(
                0,
                1,
                0,
                255,
                &[
                    (0, 0, 0x41),
                    (0, 0, 0x40),
                    (0, 0, 0x37),
                    (0, 1, 0x08),
                    (1, 0, 0x40),
                    (0, 0, 0x41),
                    (3, 0, 0x16),
                ],
            ),
            fun(1, 0, 0, 255, &[(0, 0, 0x41), (1, 0, 0x40), (2, 0, 0x16)]),
        ],
        0,
        &[b"bounce\0", b"A\0"],
    );
    let b = image(
        &[fun(
            0,
            1,
            0,
            255,
            &[
                (0, 0, 0x40),
                (0, 0, 0x37),
                (0, 0, 0x08),
                (1, 0, 0x40),
                (0, 0, 0x41),
                (3, 0, 0x16),
            ],
        )],
        0,
        &[b"back\0", b"B\0"],
    );
    let mut store = ObjectStore::new();
    let pa = store
        .register_program(other_target(a, vec![method(b"back", 1)]))
        .unwrap();
    let pb = store
        .register_program(other_target(b, vec![method(b"bounce", 0)]))
        .unwrap();
    let a = store.create(pa, vec![]).unwrap();
    let b = store.create(pb, vec![]).unwrap();
    let mut vm = Vm::new(store);
    vm.start(a, 0, vec![Value::Object(b)]).unwrap();
    let result = returned_array(&mut vm);
    let Value::Array(mid) = &result[2] else {
        panic!("middle result")
    };
    let mid = mid.elements().unwrap();
    let Value::Array(inner) = &mid[2] else {
        panic!("inner result")
    };
    assert_eq!(
        inner.elements().unwrap(),
        vec![Value::bytes(b"A"), Value::Object(a)]
    );
    assert_eq!(mid[..2], [Value::Object(b), Value::bytes(b"B")]);
    assert_eq!(result[..2], [Value::Object(a), Value::bytes(b"A")]);
}

#[test]
fn other_call_inherited_body_uses_target_bases_and_source_pool() {
    let (parent, child) = inherited_pair(
        &[(0, 0, 0x3d), (0, 0, 0x40), (0, 0, 0x41), (3, 0, 0x16)],
        &[],
    );
    let target = link(&linked_catalog(parent, child))
        .unwrap()
        .with_methods(vec![method(b"read", 0)])
        .unwrap();
    let caller = image(&[other_wrapper(0, 0)], 0, &[]);
    let (mut vm, a, b) = other_machine(caller, target, vec![], vec![Value::Int(1), Value::Int(9)]);
    vm.start(a, 0, vec![Value::Object(b), Value::bytes(b"read")])
        .unwrap();
    assert_eq!(
        returned_array(&mut vm),
        vec![Value::Object(b), Value::bytes(b"parent"), Value::Int(9)]
    );
}

#[test]
fn other_call_private_lookup_and_fallback_preserve_method_argument() {
    for flags in [0, 2, 4, 6] {
        let mut p = image(
            &[
                fun(0, 0, 0, 255, &[(10, 0, 0x3b)]),
                fun(1, 1, 0, 255, &[(0, 0, 0x37)]),
            ],
            0,
            &[],
        );
        p.functions[0].attrib |= 4;
        let target = other_target(
            p,
            vec![method(b"secret", 0), method(b"default_interface", 1)],
        );
        let (mut vm, a, b) = other_machine(
            image(&[other_wrapper(0, flags)], 0, &[]),
            target,
            vec![],
            vec![],
        );
        vm.start(a, 0, vec![Value::Object(b), Value::bytes(b"secret")])
            .unwrap();
        returns(
            &mut vm,
            if flags & 2 != 0 {
                Value::Int(10)
            } else {
                Value::bytes(b"secret")
            },
        );
    }
}

#[test]
fn other_call_private_fallback_obeys_same_access_flag() {
    for flags in [0, 2, 4, 6] {
        let mut p = image(&[fun(0, 1, 0, 255, &[(0, 0, 0x37)])], 0, &[]);
        p.functions[0].attrib |= 4;
        let target = other_target(p, vec![method(b"default_interface", 0)]);
        let (mut vm, a, b) = other_machine(
            image(&[other_wrapper(0, flags)], 0, &[]),
            target,
            vec![],
            vec![],
        );
        vm.start(a, 0, vec![Value::Object(b), Value::bytes(b"absent")])
            .unwrap();
        if flags & 2 != 0 {
            returns(&mut vm, Value::bytes(b"absent"));
        } else if flags & 4 != 0 {
            returns(&mut vm, Value::Void);
        } else {
            assert_eq!(vm.run(100).unwrap_err().kind, VmErrorKind::InvalidProgram);
        }
    }
}

#[test]
fn other_call_argument_and_fallback_orders_match_native_stack() {
    for fallback in [false, true] {
        let f = fun(0, if fallback { 3 } else { 2 }, 0, 255, &[(0, 0, 0x15)]);
        let target = other_target(
            image(&[f], 0, &[]),
            vec![method(
                if fallback {
                    b"default_interface"
                } else {
                    b"method"
                },
                0,
            )],
        );
        let (mut vm, a, b) = other_machine(
            image(&[other_wrapper(2, 0)], 0, &[]),
            target,
            vec![],
            vec![],
        );
        vm.start(
            a,
            0,
            vec![
                Value::Object(b),
                Value::bytes(b"method"),
                Value::Int(7),
                Value::Int(8),
            ],
        )
        .unwrap();
        let mut expected = vec![Value::Int(7), Value::Int(8)];
        if fallback {
            expected.insert(0, Value::bytes(b"method"));
        }
        assert_eq!(returned_array(&mut vm), expected);
        assert_eq!(vm.frame_depth(), 0);
    }
}

#[test]
fn other_call_missing_method_cleans_only_call_operands_and_pushes_void() {
    let caller = image(
        &[fun(
            0,
            1,
            0,
            255,
            &[
                (99, 0, 0x3b),
                (7, 0, 0x3b),
                (0, 0, 0x40),
                (0, 0, 0x37),
                (4, 1, 0x08),
                (2, 0, 0x16),
            ],
        )],
        0,
        &[b"absent\0"],
    );
    let target = other_target(image(&[], 0, &[]), vec![]);
    let (mut vm, a, b) = other_machine(caller, target, vec![], vec![]);
    vm.start(a, 0, vec![Value::Object(b)]).unwrap();
    assert_eq!(returned_array(&mut vm), vec![Value::Void, Value::Int(99)]);
}

#[test]
fn other_call_unprovided_namespace_is_not_suppressed_missing_method() {
    for explicit in [false, true] {
        let p = prepare_program(image(&[], 0, &[])).unwrap();
        let target = if explicit {
            p.with_methods(vec![]).unwrap()
        } else {
            p
        };
        let (mut vm, a, b) = other_machine(
            image(&[other_wrapper(0, 4)], 0, &[]),
            target,
            vec![],
            vec![],
        );
        vm.start(a, 0, vec![Value::Object(b), Value::bytes(b"missing")])
            .unwrap();
        if explicit {
            returns(&mut vm, Value::Void);
        } else {
            vm.run(4).unwrap();
            let before = vm.stack().unwrap();
            assert_eq!(vm.run(1).unwrap_err().kind, VmErrorKind::UnsupportedFeature);
            assert_eq!(vm.stack().unwrap(), before);
            assert_eq!(vm.frame_depth(), 1);
        }
    }
}

#[test]
fn other_call_names_are_case_sensitive_raw_bytes_and_nul_is_rejected() {
    for name in [b"name".as_slice(), b"NAME", b"\xff", b"name\0tail"] {
        let target = other_target(
            image(&[fun(0, 0, 0, 255, &[(6, 0, 0x3b)])], 0, &[]),
            vec![method(b"name", 0), method(b"\xff", 0)],
        );
        let (mut vm, a, b) = other_machine(
            image(&[other_wrapper(0, 4)], 0, &[]),
            target,
            vec![],
            vec![],
        );
        vm.start(a, 0, vec![Value::Object(b), Value::bytes(name)])
            .unwrap();
        if name.contains(&0) {
            assert_eq!(
                vm.run(100).unwrap_err().kind,
                VmErrorKind::UnsupportedFeature
            );
        } else {
            returns(
                &mut vm,
                if name == b"NAME" {
                    Value::Void
                } else {
                    Value::Int(6)
                },
            );
        }
    }
}

#[test]
fn other_call_bad_targets_and_names_are_not_suppressed() {
    for (target_value, name, kind) in [
        (
            Value::Int(0),
            Value::bytes(b"m"),
            VmErrorKind::UnsupportedType,
        ),
        (
            Value::bytes(b"object"),
            Value::bytes(b"m"),
            VmErrorKind::UnsupportedFeature,
        ),
        (
            Value::array(vec![]).unwrap(),
            Value::bytes(b"m"),
            VmErrorKind::UnsupportedFeature,
        ),
    ] {
        let target = other_target(image(&[], 0, &[]), vec![]);
        let (mut vm, a, _) = other_machine(
            image(&[other_wrapper(0, 4)], 0, &[]),
            target,
            vec![],
            vec![],
        );
        vm.start(a, 0, vec![target_value, name]).unwrap();
        vm.run(4).unwrap();
        let before = vm.stack().unwrap();
        let e = vm.run(1).unwrap_err();
        assert_eq!((e.kind, e.source.as_str(), e.pc), (kind, "caller", Some(4)));
        assert_eq!(vm.stack().unwrap(), before);
    }
    let target = other_target(image(&[], 0, &[]), vec![]);
    let (mut vm, a, b) = other_machine(
        image(&[other_wrapper(0, 4)], 0, &[]),
        target,
        vec![],
        vec![],
    );
    vm.start(a, 0, vec![Value::Object(b), Value::Int(1)])
        .unwrap();
    assert_eq!(vm.run(100).unwrap_err().kind, VmErrorKind::UnsupportedType);
}

#[test]
fn other_call_deleted_target_fails_before_consuming_operands() {
    let target = other_target(image(&[], 0, &[]), vec![]);
    let (mut vm, a, b) = other_machine(
        image(&[other_wrapper(0, 4)], 0, &[]),
        target,
        vec![],
        vec![],
    );
    vm.start(a, 0, vec![Value::Object(b), Value::bytes(b"m")])
        .unwrap();
    vm.run(4).unwrap();
    let before = vm.stack().unwrap();
    vm.objects_mut().remove(b).unwrap();
    assert_eq!(vm.run(1).unwrap_err().kind, VmErrorKind::InvalidObject);
    assert_eq!(vm.stack().unwrap(), before);
}

#[test]
fn other_call_rejects_bad_flags_negative_argc_and_underflow() {
    for (flags, argc) in [(1, 0), (3, 0), (8, 0), (-1, 0), (0x800, 0), (0, -1)] {
        let p = image(&[fun(0, 0, 0, 255, &[(flags, argc, 0x08)])], 0, &[]);
        let e = prepare_program(p).unwrap_err();
        assert_eq!((e.kind, e.pc), (VmErrorKind::UnsupportedFeature, Some(2)));
    }
    for nodes in [vec![(0, 0, 0x08)], vec![(0, 0, 0x41), (0, 0, 0x08)]] {
        let (mut vm, a) = machine(image(&[fun(0, 0, 1, 255, &nodes)], 0, &[]), vec![]);
        vm.start(a, 0, vec![]).unwrap();
        assert_eq!(vm.run(100).unwrap_err().kind, VmErrorKind::StackUnderflow);
    }
}

#[test]
fn other_call_reference_owner_survives_cross_program_and_nested_forwarding() {
    for local in [false, true] {
        let mut nodes = if local {
            vec![(7, 0, 0x3b), (-1, 0, 0x4d), (0, 0, 0x33)]
        } else {
            vec![]
        };
        nodes.extend([
            (
                if local { -1 } else { 0 },
                0,
                if local { 0x39 } else { 0x3e },
            ),
            (1, 0, 0x37),
            (0, 0, 0x37),
            (0, 1, 0x08),
            (0, 0, 0x33),
            (
                if local { -1 } else { 0 },
                0,
                if local { 0x37 } else { 0x3d },
            ),
        ]);
        let caller = image(&[fun(0, 2, usize::from(local), 255, &nodes)], 1, &[]);
        let mut target = image(
            &[
                fun(0, 1, 0, 255, &[(0, 0, 0x39), (1, 1, 0x07)]),
                fun(1, 1, 0, 255, &[(21, 0, 0x3b), (0, 0, 0x39), (0, 0, 0x03)]),
            ],
            1,
            &[],
        );
        ref_arg(&mut target, 0, 0, 2, 0);
        ref_arg(&mut target, 1, 0, 2, 0);
        let target = other_target(target, vec![method(b"write", 0)]);
        let (mut vm, a, b) =
            other_machine(caller, target, vec![Value::Int(7)], vec![Value::Int(4)]);
        vm.start(a, 0, vec![Value::Object(b), Value::bytes(b"write")])
            .unwrap();
        returns(&mut vm, Value::Int(21));
        assert_eq!(
            object_values(&vm, a),
            vec![Value::Int(if local { 7 } else { 21 })]
        );
        assert_eq!(object_values(&vm, b), vec![Value::Int(4)]);
        assert_eq!(vm.frame_depth(), 0);
        assert!(vm.stack().unwrap().is_empty());
    }
}

#[test]
fn other_call_member_reference_quota_is_released_after_return() {
    let mut nodes = Vec::new();
    for _ in 0..2 {
        nodes.extend([
            (0, 0, 0x3e),
            (0, 0, 0x3b),
            (0, 0, 0x44),
            (1, 0, 0x37),
            (0, 0, 0x37),
            (0, 1, 0x08),
            (0, 0, 0x33),
        ]);
    }
    nodes.extend([(0, 0, 0x3d), (0, 0, 0x3b), (0, 0, 0x43)]);
    let caller = image(&[fun(0, 2, 0, 255, &nodes)], 1, &[]);
    let mut target = image(
        &[fun(
            0,
            1,
            0,
            255,
            &[(0, 0, 0x39), (1, 0, 0x22), (0, 0, 0x37)],
        )],
        0,
        &[],
    );
    ref_arg(&mut target, 0, 0, 2, 0);
    let target = other_target(target, vec![method(b"inc", 0)]);
    let (vm, a, b) = other_machine(
        caller,
        target,
        vec![Value::array(vec![Value::Int(5)]).unwrap()],
        vec![],
    );
    let mut vm = Vm::with_limits(
        vm.into_objects(),
        VmLimits {
            max_reference_cells: 1,
            ..Default::default()
        },
    );
    vm.start(a, 0, vec![Value::Object(b), Value::bytes(b"inc")])
        .unwrap();
    returns(&mut vm, Value::Int(7));
}

#[test]
fn other_call_entry_failures_preserve_operands_and_reference_targets() {
    for scenario in 0..4 {
        let caller = image(&[other_wrapper(1, 0)], 0, &[]);
        let mut p = image(
            &[fun(0, if scenario == 0 { 0 } else { 1 }, 0, 255, &[])],
            1,
            &[],
        );
        if scenario == 1 {
            ref_arg(&mut p, 0, 0, 2, 0);
        }
        if scenario == 2 {
            p.functions[0].arguments[0].r#type = 2;
        }
        let target = other_target(
            p,
            if scenario == 3 {
                vec![]
            } else {
                vec![method(b"m", 0)]
            },
        );
        let (mut vm, a, b) = other_machine(caller, target, vec![], vec![Value::Int(9)]);
        vm.start(
            a,
            0,
            vec![Value::Object(b), Value::bytes(b"m"), Value::bytes(b"bad")],
        )
        .unwrap();
        vm.run(5).unwrap();
        let before = vm.stack().unwrap();
        let e = vm.run(1).unwrap_err();
        assert_eq!(
            e.kind,
            match scenario {
                0 => VmErrorKind::ArgumentCount,
                1 => VmErrorKind::UnsupportedFeature,
                2 => VmErrorKind::UnsupportedType,
                _ => VmErrorKind::InvalidProgram,
            }
        );
        assert_eq!(
            (e.object, e.source.as_str(), e.pc),
            (Some(a), "caller", Some(5))
        );
        assert_eq!(vm.stack().unwrap(), before);
        assert_eq!(vm.frame_depth(), 1);
        assert_eq!(object_values(&vm, b), vec![Value::Int(9)]);
        assert_eq!(vm.run(10).unwrap_err(), e);
    }
}

#[test]
fn other_call_optional_variadic_and_numeric_entry_contracts_are_reused() {
    for provided in 0..=3 {
        let f = optional_fun(
            0,
            2,
            0,
            255,
            &[vec![(10, 0, 0x3b)], vec![(20, 0, 0x3b)]],
            &[(0, 0, 0x15)],
        );
        let mut p = image(&[f], 0, &[]);
        optional_meta(&mut p, 0, 0, true, &[1, 1]);
        p.functions[0].arguments[0].r#type = 3;
        let target = other_target(p, vec![method(b"m", 0)]);
        let (mut vm, a, b) = other_machine(
            image(&[other_wrapper(provided, 0)], 0, &[]),
            target,
            vec![],
            vec![],
        );
        let mut args = vec![Value::Object(b), Value::bytes(b"m")];
        args.extend((0..provided).map(|i| Value::Int(100 + i as i32)));
        vm.start(a, 0, args).unwrap();
        let expected = if provided == 0 {
            vec![Value::Int(10), Value::Int(20)]
        } else {
            let mut values: Vec<_> = (0..provided).map(|i| Value::Int(100 + i as i32)).collect();
            values[0] = real(100.0);
            if provided == 1 {
                values.push(Value::Int(20));
            }
            values
        };
        assert_eq!(returned_array(&mut vm), expected);
    }
}

#[test]
fn other_call_budget_pause_preserves_callee_and_restores_return_context() {
    let target = other_target(
        image(&[fun(0, 0, 0, 255, &[(0, 0, 0x41)])], 0, &[]),
        vec![method(b"m", 0)],
    );
    let (mut vm, a, b) = other_machine(
        image(&[other_wrapper(0, 0)], 0, &[]),
        target,
        vec![],
        vec![],
    );
    vm.start(a, 0, vec![Value::Object(b), Value::bytes(b"m")])
        .unwrap();
    vm.run(5).unwrap(); // call has installed target frame, no target instruction has run
    assert_eq!(vm.frame_depth(), 2);
    let before = vm.stack().unwrap();
    assert_eq!(vm.run(0).unwrap(), RunOutcome::BudgetExhausted);
    assert_eq!(vm.stack().unwrap(), before);
    for _ in 0..3 {
        assert_eq!(vm.run(1).unwrap(), RunOutcome::BudgetExhausted);
    }
    assert_eq!(vm.frame_depth(), 2);
    assert_eq!(vm.run(1).unwrap(), RunOutcome::BudgetExhausted); // target return
    assert_eq!(vm.frame_depth(), 1);
    returns(&mut vm, Value::Object(b));
}

#[test]
fn other_call_frame_limit_is_atomic_and_local_limit_is_a_callee_error() {
    for frame_limit in [1, 2] {
        let target = other_target(
            image(&[fun(0, 0, 10, 255, &[])], 0, &[]),
            vec![method(b"m", 0)],
        );
        let (vm, a, b) = other_machine(
            image(&[other_wrapper(0, 0)], 0, &[]),
            target,
            vec![],
            vec![],
        );
        let mut vm = Vm::with_limits(
            vm.into_objects(),
            VmLimits {
                max_frames: frame_limit,
                max_stack: 4,
                ..Default::default()
            },
        );
        vm.start(a, 0, vec![Value::Object(b), Value::bytes(b"m")])
            .unwrap();
        vm.run(4).unwrap();
        let before = vm.stack().unwrap();
        let e = vm.run(10).unwrap_err();
        assert_eq!(e.kind, VmErrorKind::Limit);
        if frame_limit == 1 {
            assert_eq!(vm.stack().unwrap(), before);
            assert_eq!(e.object, Some(a));
            assert_eq!(vm.frame_depth(), 1);
        } else {
            assert_eq!(
                (e.object, e.source.as_str(), e.pc),
                (Some(b), "target", Some(1))
            );
            assert_eq!(vm.frame_depth(), 2);
        }
    }
}

#[test]
fn other_call_return_type_errors_retain_target_diagnostic_and_frames() {
    let target = other_target(
        image(&[fun(0, 0, 0, 2, &[(0, 0, 0x40)])], 0, &[b"bad\0"]),
        vec![method(b"m", 0)],
    );
    let (mut vm, a, b) = other_machine(
        image(&[other_wrapper(0, 0)], 0, &[]),
        target,
        vec![],
        vec![],
    );
    vm.start(a, 0, vec![Value::Object(b), Value::bytes(b"m")])
        .unwrap();
    let e = vm.run(100).unwrap_err();
    assert_eq!(
        (e.kind, e.object, e.source.as_str(), e.pc),
        (VmErrorKind::UnsupportedType, Some(b), "target", Some(3))
    );
    assert_eq!(vm.frame_depth(), 2);
    assert_eq!(vm.run(100).unwrap_err(), e);
}

#[test]
fn other_call_lifecycle_preparation_rejects_direct_and_helper_calls() {
    for helper in [false, true] {
        let mut p = image(
            &[fun(
                0,
                0,
                0,
                255,
                if helper { &[(0, 0, 0x08)] } else { &[] },
            )],
            0,
            &[],
        );
        set_init(
            &mut p,
            if helper {
                &[(0, 0, 0x48)]
            } else {
                &[(0, 0, 0x08), (0, 0, 0x48)]
            },
        );
        let mut catalog = init_catalog(p);
        catalog
            .bind_lifecycle("root", rml32::LifecycleBinding::default())
            .unwrap();
        for construct in [false, true] {
            let result = if construct {
                rml32::prepare_constructible_program(&catalog, "root", Default::default())
            } else {
                prepare_init(&catalog, "root")
            };
            let e = result.unwrap_err();
            assert_eq!(
                (e.kind, e.source.as_str()),
                (VmErrorKind::UnsupportedFeature, "root")
            );
            assert_eq!(e.pc, Some(if helper { 2 } else { 0 }));
            if helper {
                assert_eq!(e.function, Some(0));
            }
        }
    }
    let (parent, child) = inherited_pair(&[(0, 0, 0x08)], &[]);
    let mut parent = parent;
    let mut child = child;
    set_init(&mut parent, &[(0, 0, 0x48)]);
    set_init(&mut child, &[(0, 0, 0x48)]);
    let e = prepare_init(&linked_catalog(parent, child), "child").unwrap_err();
    assert_eq!(
        (e.kind, e.source.as_str(), e.function),
        (VmErrorKind::UnsupportedFeature, "parent", Some(0))
    );
}

#[test]
fn this_object_works_during_initialization_and_obeys_stack_limit() {
    let mut p = image(&[], 1, &[]);
    set_init(
        &mut p,
        &[(0, 0, 0x41), (0, 0, 0x4e), (0, 0, 0x33), (0, 0, 0x48)],
    );
    let (mut vm, id) = init_vm(&init_catalog(p), "root");
    let o = vm.begin_initialize(id).unwrap();
    assert!(matches!(vm.run(100).unwrap(), RunOutcome::Finished(_)));
    assert_eq!(object_values(&vm, o), vec![Value::Object(o)]);
    let (vm, o) = machine(image(&[fun(0, 0, 0, 255, &[(0, 0, 0x41)])], 0, &[]), vec![]);
    let mut vm = Vm::with_limits(
        vm.into_objects(),
        VmLimits {
            max_stack: 0,
            ..Default::default()
        },
    );
    vm.start(o, 0, vec![]).unwrap();
    assert_eq!(vm.run(100).unwrap_err().kind, VmErrorKind::Limit);
}

#[test]
fn other_method_binding_requires_checked_entry_for_numeric_coercion() {
    let mut f = fun(0, 1, 0, 255, &[(0, 0, 0x37)]);
    f.nodes.remove(0); // fixed value-only functions can otherwise omit 0f
    let p = prepare_program(image(&[f], 0, &[])).unwrap();
    assert_eq!(
        p.with_methods(vec![method(b"m", 0)]).unwrap_err().kind,
        VmErrorKind::UnsupportedFeature
    );
}

#[test]
fn other_call_inherited_private_body_and_recursive_frame_limit_are_bounded() {
    for flags in [0, 2] {
        let (mut parent, child) = inherited_pair(&[(42, 0, 0x3b)], &[]);
        parent.functions[0].attrib |= 4;
        let target = link(&linked_catalog(parent, child))
            .unwrap()
            .with_methods(vec![method(b"m", 0)])
            .unwrap();
        let (mut vm, a, b) = other_machine(
            image(&[other_wrapper(0, flags)], 0, &[]),
            target,
            vec![],
            vec![Value::Int(0); 2],
        );
        vm.start(a, 0, vec![Value::Object(b), Value::bytes(b"m")])
            .unwrap();
        if flags == 2 {
            returns(&mut vm, Value::Int(42));
        } else {
            assert_eq!(vm.run(100).unwrap_err().kind, VmErrorKind::InvalidProgram);
        }
    }
    let p = other_target(
        image(
            &[fun(
                0,
                0,
                0,
                255,
                &[(0, 0, 0x40), (0, 0, 0x41), (0, 0, 0x08)],
            )],
            0,
            &[b"recur\0"],
        ),
        vec![method(b"recur", 0)],
    );
    let mut store = ObjectStore::new();
    let p = store.register_program(p).unwrap();
    let o = store.create(p, vec![]).unwrap();
    let mut vm = Vm::with_limits(
        store,
        VmLimits {
            max_frames: 2,
            ..Default::default()
        },
    );
    vm.start(o, 0, vec![]).unwrap();
    assert_eq!(vm.run(100).unwrap_err().kind, VmErrorKind::Limit);
    assert_eq!(vm.frame_depth(), 2);
}

// Stage 2S: case instructions execute only synthetic code and metadata.
fn switch_image(op: u8, literal: i32, strings: &[&[u8]]) -> ProgramImage {
    let mut f = fun(
        0,
        1,
        0,
        255,
        &[
            (0, 0, 0x37),
            (0, 0, op),
            (0, 99, 0x0c),
            (10, 0, 0x3b),
            (0, 0, 0x49),
            (20, 0, 0x3b),
        ],
    );
    f.meta.extend(3i32.to_le_bytes()); // pc3 -> pc7; a miss reaches default at pc4
    f.meta.extend(literal.to_le_bytes());
    image(&[f], 0, strings)
}

#[test]
fn switch_number_cases_match_only_exact_ints_and_default_consumes_selector() {
    for literal in [i32::MIN, -3, 0, i32::MAX] {
        for (value, matched) in [
            (Value::Int(literal), true),
            (Value::Int(literal.wrapping_add(1)), false),
            (real(literal as f32), false),
            (Value::bytes(literal.to_string()), false),
            (Value::Void, false),
            (Value::Undefined, false),
            (Value::array(vec![]).unwrap(), false),
            (Value::mapping(vec![]).unwrap(), false),
        ] {
            let (mut vm, o) = machine(switch_image(0x0a, literal, &[]), vec![]);
            vm.start(o, 0, vec![value]).unwrap();
            returns(&mut vm, Value::Int(if matched { 20 } else { 10 }));
            assert!(vm.stack().unwrap().is_empty());
        }
    }
}

#[test]
fn switch_string_cases_compare_length_and_raw_bytes_without_coercion() {
    for (pool, selector, matched) in [
        (b"abc\0".as_slice(), Value::bytes(b"abc"), true),
        (b"abc\0", Value::bytes(b"ABC"), false),
        (b"abc\0", Value::bytes(b"abcd"), false),
        (b"abc\0", Value::bytes(b"abc\0"), false),
        (b"\0", Value::bytes(b""), true),
        (b"\xff\xfe\0", Value::bytes(b"\xff\xfe"), true),
        (b"\xff\0ignored\0", Value::bytes(b"\xff"), true),
        (b"3\0", Value::Int(3), false),
        (b"3\0", real(3.0), false),
        (b"abc\0", Value::Void, false),
    ] {
        let (mut vm, o) = machine(switch_image(0x0b, 0, &[pool]), vec![]);
        vm.start(o, 0, vec![selector]).unwrap();
        returns(&mut vm, Value::Int(if matched { 20 } else { 10 }));
    }
}

#[test]
fn switch_miss_preserves_selector_and_hit_pops_exactly_one_value() {
    for matched in [false, true] {
        let mut f = fun(
            0,
            1,
            0,
            255,
            &[
                (77, 0, 0x3b),
                (0, 0, 0x37),
                (0, 0, 0x0a),
                (0, 0, 0x33),
                (0, 0, 0x49),
            ],
        );
        f.meta.extend(1i32.to_le_bytes()); // case pc4 -> pc6 return; miss pop at pc5
        f.meta.extend(3i32.to_le_bytes());
        let (mut vm, o) = machine(image(&[f], 0, &[]), vec![]);
        vm.start(o, 0, vec![Value::Int(if matched { 3 } else { 4 })])
            .unwrap();
        vm.run(4).unwrap();
        let before = vm.stack().unwrap();
        vm.run(1).unwrap();
        if matched {
            assert_eq!(vm.stack().unwrap().len(), before.len() - 1);
        } else {
            assert_eq!(vm.stack().unwrap(), before);
        }
        returns(&mut vm, Value::Int(77));
    }
}

#[test]
fn switch_sequential_mixed_cases_and_default_choose_first_matching_case() {
    let mut f = fun(
        0,
        1,
        0,
        255,
        &[
            (0, 0, 0x37),
            (0, 0, 0x0a),
            (8, 0, 0x0b),
            (4, -7, 0x0c),
            (11, 0, 0x3b),
            (0, 0, 0x49),
            (22, 0, 0x3b),
            (0, 0, 0x49),
            (33, 0, 0x3b),
        ],
    );
    for pair in [[2i32, 7], [3, 0]] {
        for v in pair {
            f.meta.extend(v.to_le_bytes());
        }
    }
    for (selector, expected) in [
        (Value::Int(7), 11),
        (Value::bytes(b"yes"), 22),
        (Value::Int(8), 33),
    ] {
        let (mut vm, o) = machine(image(&[f.clone()], 0, &[b"yes\0"]), vec![]);
        vm.start(o, 0, vec![selector]).unwrap();
        returns(&mut vm, Value::Int(expected));
    }
}

#[test]
fn switch_without_default_and_nested_selectors_need_no_hidden_state() {
    let mut f = fun(
        0,
        0,
        0,
        255,
        &[
            (90, 0, 0x3b),
            (3, 0, 0x3b),
            (0, 0, 0x0a),
            (0, 0, 0x33),
            (0, 0, 0x0c),
            (0, 0, 0x49),
        ],
    );
    f.meta.extend(0i32.to_le_bytes());
    f.meta.extend(8i32.to_le_bytes());
    // Inner miss keeps3, explicit33 drops it; default pops outer90. ReturnVoid.
    let (mut vm, o) = machine(image(&[f], 0, &[]), vec![]);
    vm.start(o, 0, vec![]).unwrap();
    returns(&mut vm, Value::Void);
}

#[test]
fn switch_budget_pause_commits_case_only_once_and_zero_budget_is_inert() {
    let (mut vm, o) = machine(switch_image(0x0a, 3, &[]), vec![]);
    vm.start(o, 0, vec![Value::Int(3)]).unwrap();
    vm.run(3).unwrap(); // immediately before case
    let before = vm.stack().unwrap();
    assert_eq!(vm.run(0).unwrap(), RunOutcome::BudgetExhausted);
    assert_eq!(vm.stack().unwrap(), before);
    assert_eq!(vm.run(1).unwrap(), RunOutcome::BudgetExhausted);
    assert_eq!(vm.stack().unwrap().len(), before.len() - 1);
    assert_eq!(vm.run(0).unwrap(), RunOutcome::BudgetExhausted);
    returns(&mut vm, Value::Int(20));
}

#[test]
fn switch_empty_calculation_region_and_reference_selectors_fail_without_consumption() {
    for op in [0x0a, 0x0b, 0x0c] {
        for reference in [false, true] {
            let mut body = if reference {
                vec![(-1, 0, 0x39)]
            } else {
                vec![]
            };
            body.push((0, 0, op));
            let mut f = fun(0, 0, 1, 255, &body);
            f.meta.extend([0; 8]);
            let (mut vm, o) = machine(image(&[f], 0, &[b"x\0"]), vec![]);
            vm.start(o, 0, vec![]).unwrap();
            vm.run(if reference { 3 } else { 2 }).unwrap();
            let before = vm.stack().unwrap();
            let e = vm.run(1).unwrap_err();
            assert_eq!(
                e.kind,
                if reference {
                    VmErrorKind::UnsupportedFeature
                } else {
                    VmErrorKind::StackUnderflow
                }
            );
            assert_eq!(vm.stack().unwrap(), before);
            assert_eq!(vm.run(1).unwrap_err(), e);
        }
    }
}

#[test]
fn switch_preparation_checks_metadata_sign_extension_truncation_and_offsets() {
    for (a, b, len) in [
        (-1, 0, 8),
        (0, -1, 8),
        (0, 0, 7),
        (8, 0, 8),
        (-32768, 1, 65544),
    ] {
        for op in [0x0a, 0x0b] {
            let mut p = switch_image(op, 0, &[b"x\0"]);
            let f = &mut p.functions[0];
            f.code.nodes[3] = (a, b, op);
            f.code.metadata_hex = rml32::hex(&vec![0; len]);
            let e = prepare_program(p).unwrap_err();
            assert_eq!((e.kind, e.pc), (VmErrorKind::InvalidProgram, Some(3)));
            assert!(e.offset.is_some());
        }
    }
}

#[test]
fn switch_preparation_rejects_bad_string_pool_before_selector_type() {
    for index in [-1, 1, i32::MAX, 0] {
        let mut p = switch_image(0x0b, index, &[b"x\0"]);
        if index == 0 {
            p.strings[0] = None;
        }
        assert_eq!(
            prepare_program(p).unwrap_err().kind,
            VmErrorKind::InvalidProgram
        );
    }
    // Unreachable case still has its pool checked during preparation.
    let mut p = switch_image(0x0b, -1, &[b"x\0"]);
    p.functions[0].code.nodes[2] = (1, 0, 0x3b); // an Int selector cannot hide invalid pool metadata
    assert_eq!(prepare_program(p).unwrap_err().pc, Some(3));
}

#[test]
fn switch_preparation_validates_all_targets_including_unreachable_cases() {
    for delta in [i32::MIN, -5, 100, i32::MAX] {
        let mut p = switch_image(0x0a, 3, &[]);
        let mut meta = Vec::new();
        meta.extend(delta.to_le_bytes());
        meta.extend(3i32.to_le_bytes());
        p.functions[0].code.metadata_hex = rml32::hex(&meta);
        assert_eq!(prepare_program(p).unwrap_err().pc, Some(3));
    }
    for a in [-32768, 32767] {
        let p = image(&[fun(0, 0, 0, 255, &[(a, 0, 0x0c)])], 0, &[]);
        assert_eq!(prepare_program(p).unwrap_err().pc, Some(2));
    }
}

#[test]
fn switch_metadata_allows_unaligned_shared_and_overlapping_records() {
    let mut f = fun(
        0,
        0,
        0,
        255,
        &[(0, 0, 0x3b), (1, 0, 0x0a), (1, 0, 0x0a), (5, 0, 0x0a)],
    );
    // Records at1 and5 overlap by4 bytes: both decode [0,0].
    f.meta = vec![0; 13];
    let p = prepare_program(image(&[f], 0, &[])).unwrap();
    for i in &p.function(0).unwrap().instructions()[3..6] {
        assert_eq!(i.operands(), [0, 0]);
    }
    // Positive high byte participates in the signed address expression.
    let mut f = fun(0, 0, 0, 255, &[(0, 1, 0x0a)]);
    f.meta = vec![0; 65544];
    assert_eq!(
        prepare_program(image(&[f], 0, &[]))
            .unwrap()
            .function(0)
            .unwrap()
            .instructions()[2]
            .operands(),
        [0, 0]
    );
}

#[test]
fn switch_accepts_first_and_last_node_targets_without_cfg_inference() {
    for (op, delta) in [(0x0a, -4i32), (0x0a, 4), (0x0b, -4), (0x0b, 4)] {
        let mut p = switch_image(op, 0, &[b"x\0"]);
        let mut meta = Vec::new();
        meta.extend(delta.to_le_bytes());
        meta.extend(0i32.to_le_bytes());
        p.functions[0].code.metadata_hex = rml32::hex(&meta);
        assert!(prepare_program(p).is_ok()); // length9: targets0 or8
    }
}

#[test]
fn switch_inherited_bodies_validate_and_use_their_own_string_pools() {
    let (mut p, mut c) = inherited_pair(
        &[
            (0, 0, 0x40),
            (0, 0, 0x2e),
            (0, 0, 0x0c),
            (10, 0, 0x3b),
            (0, 0, 0x49),
            (20, 0, 0x3b),
        ],
        &[
            (0, 0, 0x07),
            (0, 0, 0x40),
            (0, 0, 0x2e),
            (0, 0, 0x0c),
            (30, 0, 0x3b),
            (1, 0, 0x21),
            (40, 0, 0x3b),
            (2, 0, 0x16),
        ],
    );
    p.functions[0].code.nodes[3] = (0, 0, 0x0b);
    c.functions[0].code.nodes[4] = (0, 0, 0x0b);
    for f in [&mut p.functions[0], &mut c.functions[0]] {
        let mut meta = Vec::new();
        meta.extend(3i32.to_le_bytes());
        meta.extend(0i32.to_le_bytes());
        f.code.metadata_hex = rml32::hex(&meta);
    }
    let (mut vm, o) = linked_vm(&linked_catalog(p.clone(), c.clone()));
    vm.start(o, 1, vec![]).unwrap();
    assert_eq!(
        returned_array(&mut vm),
        vec![Value::Int(40), Value::Int(20)]
    );
    p.strings[0] = None;
    let e = link(&linked_catalog(p, c)).unwrap_err();
    assert_eq!(
        (e.kind, e.source.as_str(), e.function, e.pc),
        (VmErrorKind::InvalidProgram, "parent", Some(0), Some(3))
    );
}

#[test]
fn switch_cross_object_calls_use_target_pool_and_restore_caller_case_context() {
    let target = other_target(
        switch_image(0x0b, 0, &[b"target\0"]),
        vec![method(b"choose", 0)],
    );
    let mut caller = fun(
        0,
        1,
        0,
        255,
        &[
            (0, 0, 0x40),
            (1, 0, 0x40),
            (0, 0, 0x37),
            (0, 1, 0x08),
            (0, 0, 0x0a),
            (0, 0, 0x0c),
            (10, 0, 0x3b),
            (0, 0, 0x49),
            (30, 0, 0x3b),
        ],
    );
    caller.meta.extend(3i32.to_le_bytes());
    caller.meta.extend(20i32.to_le_bytes());
    let (mut vm, a, b) = other_machine(
        image(&[caller], 0, &[b"target\0", b"choose\0"]),
        target,
        vec![],
        vec![],
    );
    vm.start(a, 0, vec![Value::Object(b)]).unwrap();
    returns(&mut vm, Value::Int(30));
    assert_eq!(vm.frame_depth(), 0);
}

#[test]
fn switch_initialization_rejects_direct_cases_but_can_call_ordinary_helpers() {
    for op in [0x0a, 0x0b, 0x0c] {
        let mut p = image(&[], 0, &[b"x\0"]);
        set_init(&mut p, &[(0, 0, op), (0, 0, 0x48)]);
        p.init.metadata_hex = "0000000000000000".into();
        let e = prepare_init(&init_catalog(p), "root").unwrap_err();
        assert_eq!(
            (e.kind, e.initialization, e.pc),
            (VmErrorKind::UnsupportedFeature, Some(0), Some(0))
        );
    }
    let mut p = switch_image(0x0a, 3, &[]);
    p.max_vars = 1;
    set_init(
        &mut p,
        &[
            (3, 0, 0x3b),
            (0, 1, 0x07),
            (0, 0, 0x4e),
            (0, 0, 0x33),
            (0, 0, 0x48),
        ],
    );
    let (mut vm, id) = init_vm(&init_catalog(p), "root");
    let o = vm.begin_initialize(id).unwrap();
    assert!(matches!(vm.run(100).unwrap(), RunOutcome::Finished(_)));
    assert_eq!(object_values(&vm, o), vec![Value::Int(20)]);
}

#[test]
fn switch_construction_helpers_and_failure_rollback_keep_existing_task_contract() {
    for fail in [false, true] {
        let mut p = switch_image(0x0a, 3, &[]);
        let create = image(
            &[fun(
                1,
                0,
                0,
                1,
                &[(3, 0, 0x3b), (0, 1, 0x07), (0, 0, 0x4e), (0, 0, 0x33)],
            )],
            1,
            &[],
        )
        .functions
        .remove(0);
        p.functions.push(create);
        p.max_functions = 2;
        p.max_vars = 1;
        lifecycle_name(&mut p, 1, b"create\0");
        if fail {
            p.functions[1].code.nodes.insert(6, (0, 0, 0x33));
        }
        set_init(&mut p, &[(0, 0, 0x48)]);
        let mut c = init_catalog(p);
        c.bind_lifecycle(
            "root",
            rml32::LifecycleBinding {
                create: Some(1),
                destruct: None,
            },
        )
        .unwrap();
        let (mut vm, id) = construct_vm(&c, "root");
        let o = vm.begin_construct(id, None).unwrap();
        if fail {
            let e = vm.run(100).unwrap_err();
            assert_eq!(e.kind, VmErrorKind::StackUnderflow);
            assert!(vm.objects().get(o).is_err());
            assert_eq!(vm.frame_depth(), 0);
        } else {
            assert!(matches!(vm.run(100).unwrap(), RunOutcome::Finished(_)));
            assert_eq!(object_values(&vm, o), vec![Value::Int(20)]);
        }
    }
}

#[test]
fn switch_does_not_expand_default_expression_whitelist() {
    let mut f = optional_fun(0, 1, 0, 255, &[vec![(0, 0, 0x0a)]], &[]);
    f.meta.extend(0i32.to_le_bytes());
    f.meta.extend(3i32.to_le_bytes());
    let mut p = image(&[f], 0, &[]);
    optional_meta(&mut p, 0, 0, false, &[1]);
    assert_eq!(
        prepare_program(p).unwrap_err().kind,
        VmErrorKind::UnsupportedFeature
    );
}

// Stage2T: all loop bytecode below is synthesized; no shipped program executes.
fn numeric_loop(start: i16, end: i16, step: i16, body: &[Node]) -> Fun {
    let mut nodes = vec![
        (start, 0, 0x3b),
        (-1, 0, 0x4d),
        (0, 0, 0x33),
        (-1, 0, 0x39),
        (end, 0, 0x3b),
        (step, 0, 0x3b),
        (-2, 0, 0x23),
    ];
    // cond is pc9, body starts pc10; step back to body, not to cond.
    nodes.push(((body.len() + 1) as i16, -2, 0x12));
    nodes.extend(body);
    nodes.push((-((body.len() + 1) as i16), -2, 0x50));
    nodes.push((-1, 0, 0x37));
    fun(0, 0, 2, 255, &nodes)
}
fn array_loop(body: &[Node]) -> Fun {
    let mut nodes = vec![
        (-1, 0, 0x39),
        (0, 0, 0x37),
        (997, 0, 0x3b),
        (-2, 0, 0x23),
        ((body.len() + 1) as i16, -2, 0x12),
    ];
    nodes.extend(body);
    nodes.extend([(-((body.len() + 1) as i16), -2, 0x50), (-1, 0, 0x37)]);
    fun(0, 1, 2, 255, &nodes)
}
#[test]
fn loops_numeric_closed_ranges_include_end_and_write_failed_check_value() {
    for (start, end, step, count, last) in [
        (1, 3, 1, 3, 4),
        (3, 1, -1, 3, 0),
        (3, 3, 1, 1, 4),
        (3, 3, -1, 1, 2),
        (4, 3, 1, 0, 4),
        (2, 3, -1, 0, 2),
    ] {
        let f = numeric_loop(start, end, step, &[(0, 0, 0x3e), (1, 0, 0x22)]);
        let (mut vm, o) = machine(image(&[f], 1, &[]), vec![Value::Int(0)]);
        vm.start(o, 0, vec![]).unwrap();
        returns(&mut vm, Value::Int(last));
        assert_eq!(object_values(&vm, o), vec![Value::Int(count)]);
    }
}
#[test]
fn loops_numeric_i32_endpoints_wrap_once_without_reentering() {
    for (start, end, step, last) in [
        (i32::MAX, i32::MAX, 1, i32::MIN),
        (i32::MIN, i32::MIN, -1, i32::MAX),
    ] {
        let mut p = image(&[numeric_loop(0, 0, step, &[])], 0, &[]);
        p.functions[0].code.nodes[2] = (0, 0, 0x37); // initial value from arg0
        p.functions[0].code.nodes[6] = (1, 0, 0x37); // end from arg1
        let args = image(&[fun(0, 2, 0, 255, &[])], 0, &[]).functions.remove(0);
        p.functions[0].arguments = args.arguments;
        p.functions[0].min_args = 2;
        p.functions[0].max_args = 2;
        let (mut vm, o) = machine(p, vec![]);
        vm.start(o, 0, vec![Value::Int(start), Value::Int(end)])
            .unwrap();
        returns(&mut vm, Value::Int(last));
    }
}
#[test]
fn loops_numeric_body_write_does_not_change_internal_counter() {
    let f = numeric_loop(
        1,
        3,
        1,
        &[
            (99, 0, 0x3b),
            (-1, 0, 0x4d),
            (0, 0, 0x33),
            (0, 0, 0x3e),
            (1, 0, 0x22),
        ],
    );
    let (mut vm, o) = machine(image(&[f], 1, &[]), vec![Value::Int(0)]);
    vm.start(o, 0, vec![]).unwrap();
    returns(&mut vm, Value::Int(4));
    assert_eq!(object_values(&vm, o), vec![Value::Int(3)]);
}
#[test]
fn loops_array_empty_single_and_multiple_copy_current_values() {
    for values in [
        vec![],
        vec![Value::Int(7)],
        vec![Value::Int(7), Value::bytes(b"last")],
    ] {
        let f = array_loop(&[(0, 0, 0x3e), (1, 0, 0x22)]);
        let (mut vm, o) = machine(image(&[f], 1, &[]), vec![Value::Int(0)]);
        vm.start(o, 0, vec![Value::array(values.clone()).unwrap()])
            .unwrap();
        returns(&mut vm, values.last().cloned().unwrap_or(Value::Undefined));
        assert_eq!(object_values(&vm, o), vec![Value::Int(values.len() as i32)]);
    }
}
#[test]
fn loops_array_reads_live_cells_but_retains_original_header_after_rebinding() {
    let f = array_loop(&[
        (0, 0, 0x39),
        (1, 0, 0x3b),
        (0, 0, 0x44),
        (99, 0, 0x3b),
        (0, 0, 0x04),
        (0, 0, 0x33),
    ]);
    let source = Value::array(vec![Value::Int(1), Value::Int(2)]).unwrap();
    let (mut vm, o) = machine(image(&[f], 0, &[]), vec![]);
    vm.start(o, 0, vec![source]).unwrap();
    returns(&mut vm, Value::Int(99));
    let f = array_loop(&[(0, 0, 0x42), (0, 0, 0x4d), (0, 0, 0x33)]); // arg0 becomes Undefined
    let (mut vm, o) = machine(image(&[f], 0, &[]), vec![]);
    vm.start(
        o,
        0,
        vec![Value::array(vec![Value::Int(1), Value::Int(2)]).unwrap()],
    )
    .unwrap();
    returns(&mut vm, Value::Int(2));
}
#[test]
fn loops_control_reads_project_undefined_without_destroying_iteration() {
    let f = numeric_loop(
        1,
        2,
        1,
        &[
            (-2, 0, 0x37),
            (0, 0, 0x33),
            (-2, 0, 0x39),
            (0, 0, 0x14),
            (0, 0, 0x33),
        ],
    );
    let (mut vm, o) = machine(image(&[f], 0, &[]), vec![]);
    vm.start(o, 0, vec![]).unwrap();
    returns(&mut vm, Value::Int(3));
}
#[test]
fn loops_control_overwrite_invalidates_next_check_and_failure_is_sticky() {
    let f = numeric_loop(1, 2, 1, &[(0, 0, 0x42), (-2, 0, 0x4d), (0, 0, 0x33)]);
    let (mut vm, o) = machine(image(&[f], 0, &[]), vec![]);
    vm.start(o, 0, vec![]).unwrap();
    let e = vm.run(100).unwrap_err();
    assert_eq!(e.kind, VmErrorKind::InvalidState);
    assert_eq!(vm.run(100).unwrap_err(), e);
    assert_eq!(vm.frame_depth(), 1);
    vm.start(o, 0, vec![]).unwrap();
    assert_eq!(vm.run(0).unwrap(), RunOutcome::BudgetExhausted);
}
#[test]
fn loops_budget_single_step_matches_uninterrupted_execution() {
    let p = image(
        &[numeric_loop(-2, 2, 1, &[(0, 0, 0x3e), (1, 0, 0x22)])],
        1,
        &[],
    );
    let (mut vm, o) = machine(p, vec![Value::Int(0)]);
    vm.start(o, 0, vec![]).unwrap();
    for _ in 0..100 {
        let before = vm.stack().unwrap();
        assert_eq!(vm.run(0).unwrap(), RunOutcome::BudgetExhausted);
        assert_eq!(vm.stack().unwrap(), before);
        if let RunOutcome::Finished(c) = vm.run(1).unwrap() {
            assert_eq!(c, Completion::Returned(Value::Int(3)));
            assert_eq!(object_values(&vm, o), vec![Value::Int(5)]);
            return;
        }
    }
    panic!("loop failed to finish");
}
#[test]
fn loops_bad_modes_sources_aliases_and_missing_state_preserve_operands() {
    for mode in [0, 2, 997] {
        let mut f = numeric_loop(1, 2, 1, &[]);
        f.nodes[7] = (mode, 0, 0x3b);
        let (mut vm, o) = machine(image(&[f], 0, &[]), vec![]);
        vm.start(o, 0, vec![]).unwrap();
        vm.run(8).unwrap();
        let before = vm.stack().unwrap();
        assert!(vm.run(1).is_err());
        assert_eq!(vm.stack().unwrap(), before);
    }
    let mut f = numeric_loop(1, 2, 1, &[]);
    f.nodes[5] = (-2, 0, 0x39);
    let (mut vm, o) = machine(image(&[f], 0, &[]), vec![]);
    vm.start(o, 0, vec![]).unwrap();
    vm.run(8).unwrap();
    let before = vm.stack().unwrap();
    assert_eq!(vm.run(1).unwrap_err().kind, VmErrorKind::UnsupportedFeature);
    assert_eq!(vm.stack().unwrap(), before);
    let (mut vm, o) = machine(
        image(&[fun(0, 0, 1, 255, &[(0, -1, 0x12)])], 0, &[]),
        vec![],
    );
    vm.start(o, 0, vec![]).unwrap();
    assert_eq!(vm.run(100).unwrap_err().kind, VmErrorKind::InvalidState);
}
#[test]
fn loops_mapping_mode_and_member_binding_remain_explicitly_unsupported() {
    let (mut vm, o) = machine(image(&[array_loop(&[])], 0, &[]), vec![]);
    vm.start(o, 0, vec![Value::mapping(vec![]).unwrap()])
        .unwrap();
    assert_eq!(
        vm.run(100).unwrap_err().kind,
        VmErrorKind::UnsupportedFeature
    );
    let f = fun(
        0,
        1,
        1,
        255,
        &[
            (0, 0, 0x39),
            (0, 0, 0x3b),
            (0, 0, 0x44),
            (3, 0, 0x3b),
            (1, 0, 0x3b),
            (-1, 0, 0x23),
        ],
    );
    let (mut vm, o) = machine(image(&[f], 0, &[]), vec![]);
    vm.start(o, 0, vec![Value::array(vec![Value::Int(1)]).unwrap()])
        .unwrap();
    assert_eq!(
        vm.run(100).unwrap_err().kind,
        VmErrorKind::UnsupportedFeature
    );
}
#[test]
fn loops_preparation_checks_slots_targets_and_lifecycle_helpers() {
    for node in [
        (0, 0, 0x23),
        (-2, 0, 0x23),
        (0, 0, 0x12),
        (0, -2, 0x50),
        (32767, -1, 0x12),
    ] {
        let p = image(&[fun(0, 0, 1, 255, &[node])], 0, &[]);
        assert_eq!(
            prepare_program(p).unwrap_err().kind,
            VmErrorKind::InvalidProgram
        );
    }
    for op in [0x12, 0x23, 0x50] {
        let mut p = image(
            &[fun(
                0,
                0,
                1,
                255,
                &[(
                    if op == 0x23 { -1 } else { 0 },
                    if op == 0x23 { 0 } else { -1 },
                    op,
                )],
            )],
            0,
            &[],
        );
        set_init(&mut p, &[(0, 0, 0x48)]);
        let mut c = init_catalog(p.clone());
        c.bind_lifecycle("root", Default::default()).unwrap();
        assert_eq!(
            prepare_init(&c, "root").unwrap_err().kind,
            VmErrorKind::UnsupportedFeature
        );
        assert_eq!(
            rml32::prepare_constructible_program(&c, "root", Default::default())
                .unwrap_err()
                .kind,
            VmErrorKind::UnsupportedFeature
        );
        p.functions.clear();
        p.init.nodes.insert(0, (-1, 0, op));
        assert_eq!(
            prepare_init(&init_catalog(p), "root").unwrap_err().kind,
            VmErrorKind::UnsupportedFeature
        );
    }
}
#[test]
fn loops_object_binding_rechecks_deleted_owner_before_committing() {
    let caller = fun(
        0,
        0,
        1,
        255,
        &[
            (0, 0, 0x3e),
            (3, 0, 0x3b),
            (1, 0, 0x3b),
            (-1, 0, 0x23),
            (1, -1, 0x12),
            (0, -1, 0x50),
        ],
    );
    let (mut vm, o) = machine(image(&[caller], 1, &[]), vec![Value::Int(1)]);
    vm.start(o, 0, vec![]).unwrap();
    vm.run(6).unwrap();
    vm.objects_mut().remove(o).unwrap();
    assert_eq!(vm.run(1).unwrap_err().kind, VmErrorKind::InvalidObject);
}
#[test]
fn loops_hidden_control_cannot_bind_function_reference_parameters() {
    let caller = numeric_loop(1, 2, 1, &[(-2, 0, 0x39), (1, 1, 0x07), (0, 0, 0x33)]);
    let mut p = image(&[caller, fun(1, 1, 0, 255, &[])], 0, &[]);
    ref_arg(&mut p, 1, 0, 255, 0);
    let (mut vm, o) = machine(p, vec![]);
    vm.start(o, 0, vec![]).unwrap();
    assert_eq!(
        vm.run(100).unwrap_err().kind,
        VmErrorKind::UnsupportedFeature
    );
}

#[test]
fn loops_nested_cursors_and_calls_do_not_share_state() {
    // Outer 1..2, inner1..3; each inner body calls an ordinary helper.
    let mut nodes = vec![
        (1, 0, 0x3b),
        (-1, 0, 0x4d),
        (0, 0, 0x33),
        (-1, 0, 0x39),
        (2, 0, 0x3b),
        (1, 0, 0x3b),
        (-2, 0, 0x23),
    ];
    let cond_outer = nodes.len();
    nodes.push((0, -2, 0x12));
    let outer_body = nodes.len();
    nodes.extend([
        (1, 0, 0x3b),
        (-3, 0, 0x4d),
        (0, 0, 0x33),
        (-3, 0, 0x39),
        (3, 0, 0x3b),
        (1, 0, 0x3b),
        (-4, 0, 0x23),
    ]);
    let cond_inner = nodes.len();
    nodes.push((0, -4, 0x12));
    let inner_body = nodes.len();
    nodes.extend([(1, 0, 0x07), (0, 0, 0x33)]);
    let step_inner = nodes.len();
    nodes.push(((inner_body as i16) - (step_inner as i16) - 1, -4, 0x50));
    let step_outer = nodes.len();
    nodes.push(((outer_body as i16) - (step_outer as i16) - 1, -2, 0x50));
    nodes[cond_inner].0 = (step_outer - cond_inner - 1) as i16;
    nodes[cond_outer].0 = (nodes.len() - cond_outer - 1) as i16;
    nodes.push((0, 0, 0x3d));
    let p = image(
        &[
            fun(0, 0, 4, 255, &nodes),
            fun(1, 0, 0, 255, &[(0, 0, 0x3e), (1, 0, 0x22)]),
        ],
        1,
        &[],
    );
    let (mut vm, o) = machine(p, vec![Value::Int(0)]);
    vm.start(o, 0, vec![]).unwrap();
    returns(&mut vm, Value::Int(6));
}
#[test]
fn loops_cross_object_call_and_default_in_body_preserve_local_control() {
    let mut loopfun = numeric_loop(
        1,
        3,
        1,
        &[(0, 0, 0x40), (0, 0, 0x3d), (0, 0, 0x08), (0, 0, 0x0c)],
    );
    loopfun.index = 1;
    let caller = image(
        &[
            fun(
                0,
                1,
                0,
                255,
                &[(0, 0, 0x37), (0, 0, 0x4e), (0, 0, 0x33), (1, 0, 0x07)],
            ),
            loopfun,
        ],
        1,
        &[b"inc\0"],
    );
    let target = other_target(
        image(&[fun(0, 0, 0, 255, &[(0, 0, 0x3e), (1, 0, 0x22)])], 1, &[]),
        vec![method(b"inc", 0)],
    );
    let (mut next, a, b) =
        other_machine(caller, target, vec![Value::Undefined], vec![Value::Int(0)]);
    next.start(a, 0, vec![Value::Object(b)]).unwrap();
    returns(&mut next, Value::Int(4));
    assert_eq!(object_values(&next, b), vec![Value::Int(3)]);
}

#[test]
fn loops_recursive_calls_hit_frame_limit_and_restart_discards_old_controls() {
    let p = image(
        &[numeric_loop(1, 2, 1, &[(0, 0, 0x07), (0, 0, 0x33)])],
        0,
        &[],
    );
    let (vm, o) = machine(p, vec![]);
    let mut vm = Vm::with_limits(
        vm.into_objects(),
        VmLimits {
            max_frames: 3,
            ..Default::default()
        },
    );
    vm.start(o, 0, vec![]).unwrap();
    assert_eq!(vm.run(100).unwrap_err().kind, VmErrorKind::Limit);
    assert_eq!(vm.frame_depth(), 3);
    vm.start(o, 0, vec![]).unwrap();
    assert_eq!(vm.frame_depth(), 1);
    assert_eq!(vm.run(0).unwrap(), RunOutcome::BudgetExhausted);
}
#[test]
fn loops_early_break_and_return_do_not_require_exhaustion() {
    for early_return in [false, true] {
        let body = if early_return {
            vec![(-1, 0, 0x37), (0, 0, 0x49)]
        } else {
            vec![(1, 0, 0x1f)]
        };
        let (mut vm, o) = machine(image(&[array_loop(&body)], 0, &[]), vec![]);
        vm.start(
            o,
            0,
            vec![Value::array(vec![Value::Int(8), Value::Int(9)]).unwrap()],
        )
        .unwrap();
        returns(&mut vm, Value::Int(8));
        assert_eq!(vm.frame_depth(), 0);
        assert!(vm.stack().unwrap().is_empty());
    }
}
#[test]
fn loops_bound_object_deleted_during_cross_call_does_not_advance_controller() {
    let caller = image(
        &[fun(
            0,
            1,
            0,
            255,
            &[(0, 0, 0x3e), (0, 0, 0x40), (0, 0, 0x37), (0, 1, 0x08)],
        )],
        1,
        &[b"loop\0"],
    );
    let mut target = image(
        &[fun(
            0,
            1,
            1,
            255,
            &[
                (0, 0, 0x39),
                (3, 0, 0x3b),
                (1, 0, 0x3b),
                (-1, 0, 0x23),
                (1, -1, 0x12),
                (0, -1, 0x50),
            ],
        )],
        0,
        &[],
    );
    ref_arg(&mut target, 0, 0, 2, 0);
    let (mut vm, a, b) = other_machine(
        caller,
        other_target(target, vec![method(b"loop", 0)]),
        vec![Value::Int(1)],
        vec![],
    );
    vm.start(a, 0, vec![Value::Object(b)]).unwrap();
    vm.run(12).unwrap();
    assert_eq!(vm.frame_depth(), 2);
    vm.objects_mut().remove(a).unwrap();
    let before = vm.stack().unwrap();
    let e = vm.run(1).unwrap_err();
    assert_eq!((e.kind, e.object), (VmErrorKind::InvalidObject, Some(b)));
    assert_eq!(vm.stack().unwrap(), before);
    assert_eq!(vm.run(1).unwrap_err(), e);
}
#[test]
fn loops_stack_and_local_allocation_limits_fail_without_bypassing_guards() {
    for stack_limit in [1, 4] {
        let (vm, o) = machine(image(&[numeric_loop(1, 2, 1, &[])], 0, &[]), vec![]);
        let mut vm = Vm::with_limits(
            vm.into_objects(),
            VmLimits {
                max_stack: stack_limit,
                ..Default::default()
            },
        );
        vm.start(o, 0, vec![]).unwrap();
        assert_eq!(vm.run(100).unwrap_err().kind, VmErrorKind::Limit);
    }
}

#[test]
fn loops_array_copy_normalizes_dead_object_source_atomically() {
    let (mut vm, o) = machine(image(&[array_loop(&[])], 0, &[]), vec![]);
    let p = vm.objects().get(o).unwrap().program();
    let victim = vm.objects_mut().create(p, vec![]).unwrap();
    let source = Value::array(vec![Value::Object(victim)]).unwrap();
    vm.start(o, 0, vec![source.clone()]).unwrap();
    vm.objects_mut().remove(victim).unwrap();
    returns(&mut vm, Value::Undefined);
    let Value::Array(a) = source else {
        unreachable!()
    };
    assert_eq!(a.elements().unwrap(), vec![Value::Undefined]);
}

#[test]
fn loops_inherited_body_keeps_local_control_and_relocated_object_binding() {
    let parent = image(
        &[numeric_loop(1, 3, 1, &[(0, 0, 0x3e), (1, 0, 0x22)])],
        1,
        &[],
    );
    let mut child = image(&[fun(1, 0, 0, 255, &[(0, 0, 0x07)])], 2, &[]);
    child.constitution_count = Some(2);
    inherit(&mut child, &parent, b"parent\0", [1, 0, 1]);
    let p = link(&linked_catalog(parent, child)).unwrap();
    let mut store = ObjectStore::new();
    let p = store.register_program(p).unwrap();
    let o = store
        .create(p, vec![Value::Int(100), Value::Int(0)])
        .unwrap();
    let mut vm = Vm::new(store);
    vm.start(o, 1, vec![]).unwrap();
    returns(&mut vm, Value::Int(4));
    assert_eq!(object_values(&vm, o), vec![Value::Int(100), Value::Int(3)]);
}

// Stage2V: synthetic script catches only; never run packaged bytecode.
fn catch_fun(index: i16, args: usize, locals: usize, ret: u8, body: &[Node]) -> Fun {
    let mut nodes = vec![((body.len() + 2) as i16, 0, 0x2c)];
    nodes.extend(body);
    nodes.extend([(0, 0, 0x1a), (1, 0, 0x1f), (0, 0, 0x0e)]);
    fun(index, args, locals, ret, &nodes)
}
const INT_ZERO: &[Node] = &[(9, 0, 0x3b), (0, 0, 0x3b), (0, 0, 0x19)];

#[test]
fn catches_normal_end_preserves_values_and_marker_is_an_independent_primitive() {
    for f in [
        catch_fun(0, 0, 0, 255, &[(7, 0, 0x3b)]),
        fun(0, 0, 0, 255, &[(0, 0, 0x0e)]),
    ] {
        let expected = if f.nodes[2].2 == 0x2c { 7 } else { 1 };
        let (mut vm, o) = machine(image(&[f], 0, &[]), vec![]);
        vm.start(o, 0, vec![]).unwrap();
        returns(&mut vm, Value::Int(expected));
    }
}

#[test]
fn catches_integer_zero_discards_temporaries_but_keeps_committed_object_writes() {
    let mut body = vec![(17, 0, 0x3b), (0, 0, 0x4e), (0, 0, 0x33), (99, 0, 0x3b)];
    body.extend(INT_ZERO);
    let (mut vm, o) = machine(
        image(&[catch_fun(0, 0, 1, 255, &body)], 1, &[]),
        vec![Value::Int(0)],
    );
    vm.start(o, 0, vec![]).unwrap();
    returns(&mut vm, Value::Int(1));
    assert_eq!(object_values(&vm, o), vec![Value::Int(17)]);
    assert_eq!(vm.frame_depth(), 0);
}

#[test]
fn catches_unwind_multiple_local_calls_and_restore_saved_caller_prefix() {
    let caller = catch_fun(0, 0, 0, 255, &[(1, 0, 0x07)]);
    let middle = fun(1, 0, 2, 255, &[(2, 0, 0x07)]);
    let leaf = fun(2, 0, 1, 255, INT_ZERO);
    let (mut vm, o) = machine(image(&[caller, middle, leaf], 0, &[]), vec![]);
    vm.start(o, 0, vec![]).unwrap();
    returns(&mut vm, Value::Int(1));
    assert_eq!(vm.frame_depth(), 0);
}

#[test]
fn catches_nested_handler_failure_reaches_outer_handler() {
    let inner = catch_fun(0, 0, 0, 255, INT_ZERO);
    let mut body = inner.nodes[2..inner.nodes.len() - 1].to_vec();
    body.push((0, 0, 0x33));
    body.extend(INT_ZERO);
    let (mut vm, o) = machine(image(&[catch_fun(0, 0, 0, 255, &body)], 0, &[]), vec![]);
    vm.start(o, 0, vec![]).unwrap();
    returns(&mut vm, Value::Int(1));
}

#[test]
fn catches_normal_inner_end_leaves_outer_handler_active() {
    let inner = catch_fun(0, 0, 0, 255, &[(7, 0, 0x3b)]);
    let mut body = inner.nodes[2..inner.nodes.len() - 1].to_vec();
    body.push((0, 0, 0x33));
    body.extend(INT_ZERO);
    let (mut vm, o) = machine(image(&[catch_fun(0, 0, 0, 255, &body)], 0, &[]), vec![]);
    vm.start(o, 0, vec![]).unwrap();
    returns(&mut vm, Value::Int(1));
}

#[test]
fn catches_early_return_retires_callee_handler_before_caller_error() {
    let leaf = catch_fun(1, 0, 0, 255, &[(7, 0, 0x3b), (0, 0, 0x49)]);
    let mut body = vec![(1, 0, 0x07), (0, 0, 0x33)];
    body.extend(INT_ZERO);
    let (mut vm, o) = machine(
        image(&[catch_fun(0, 0, 0, 255, &body), leaf], 0, &[]),
        vec![],
    );
    vm.start(o, 0, vec![]).unwrap();
    returns(&mut vm, Value::Int(1));
}

#[test]
fn catches_return_validation_errors_remain_fatal_with_real_failure_location() {
    for ret in [1, 4] {
        let leaf = catch_fun(1, 0, 0, ret, &[(7, 0, 0x3b), (0, 0, 0x49)]);
        let (mut vm, o) = machine(
            image(&[catch_fun(0, 0, 0, 255, &[(1, 0, 0x07)]), leaf], 0, &[]),
            vec![],
        );
        vm.start(o, 0, vec![]).unwrap();
        let e = vm.run(100).unwrap_err();
        assert_eq!((e.function, e.pc), (Some(1), Some(4)));
        assert_eq!(
            e.kind,
            if ret == 1 {
                VmErrorKind::InvalidProgram
            } else {
                VmErrorKind::UnsupportedType
            }
        );
        assert_eq!(vm.run(100).unwrap_err(), e);
    }
}

#[test]
fn catches_do_not_classify_other_arithmetic_errors_by_kind_or_message() {
    for (op, left, right) in [
        (0x19, Value::Int(i32::MIN), Value::Int(-1)),
        (0x28, Value::Int(i32::MIN), Value::Int(-1)),
        (0x19, real(1.0), Value::bytes(b"bad")),
        (0x19, Value::bytes(b"x"), Value::Int(0)),
    ] {
        let body = [(0, 0, 0x37), (1, 0, 0x37), (0, 0, op)];
        let (mut vm, o) = machine(image(&[catch_fun(0, 2, 0, 255, &body)], 0, &[]), vec![]);
        vm.start(o, 0, vec![left, right]).unwrap();
        let e = vm.run(100).unwrap_err();
        assert_eq!((e.function, e.pc), (Some(0), Some(5)));
        assert_eq!(vm.frame_depth(), 1);
        assert_eq!(vm.run(100).unwrap_err(), e);
    }
}

#[test]
fn catches_unhandled_integer_zero_preserves_original_diagnostic_and_operands() {
    let (mut vm, o) = machine(image(&[fun(0, 0, 0, 255, INT_ZERO)], 0, &[]), vec![]);
    vm.start(o, 0, vec![]).unwrap();
    let e = vm.run(100).unwrap_err();
    assert_eq!((e.kind, e.pc), (VmErrorKind::Arithmetic, Some(4)));
    assert_eq!(
        vm.stack().unwrap(),
        vec![
            rml32::StackSnapshot::Value(Value::Int(9)),
            rml32::StackSnapshot::Value(Value::Int(0))
        ]
    );
    assert_eq!(vm.run(0).unwrap_err(), e);
}

#[test]
fn catches_empty_and_cross_owner_end_fail_without_consuming_caller_stack() {
    for cross in [false, true] {
        let f = if cross {
            catch_fun(0, 0, 0, 255, &[(1, 0, 0x07)])
        } else {
            fun(0, 0, 0, 255, &[(0, 0, 0x1a)])
        };
        let leaf = fun(1, 0, 0, 255, &[(0, 0, 0x1a)]);
        let (mut vm, o) = machine(image(&[f, leaf], 0, &[]), vec![]);
        vm.start(o, 0, vec![]).unwrap();
        let e = vm.run(100).unwrap_err();
        assert_eq!(e.kind, VmErrorKind::InvalidProgram);
        assert_eq!(e.function, Some(i16::from(cross)));
        assert!(vm.stack().unwrap().is_empty());
    }
}

#[test]
fn catches_registration_requires_entered_frame_and_honors_context_limit() {
    let mut f = fun(0, 0, 0, 255, &[(0, 0, 0x2c), (0, 0, 0x0e)]);
    f.nodes.remove(1);
    let (mut vm, o) = machine(image(&[f], 0, &[]), vec![]);
    vm.start(o, 0, vec![]).unwrap();
    assert_eq!(vm.run(100).unwrap_err().kind, VmErrorKind::InvalidProgram);
    let f = fun(0, 0, 0, 255, &[(1, 0, 0x2c), (-2, 0, 0x1f), (0, 0, 0x0e)]);
    let (vm, o) = machine(image(&[f], 0, &[]), vec![]);
    let mut vm = Vm::with_limits(
        vm.into_objects(),
        VmLimits {
            max_stack: 2,
            ..Default::default()
        },
    );
    vm.start(o, 0, vec![]).unwrap();
    assert_eq!(vm.run(100).unwrap_err().kind, VmErrorKind::Limit);
}

#[test]
fn catches_budget_pause_before_marker_matches_uninterrupted_execution() {
    let p = image(&[catch_fun(0, 0, 0, 255, INT_ZERO)], 0, &[]);
    let (mut vm, o) = machine(p.clone(), vec![]);
    vm.start(o, 0, vec![]).unwrap();
    let before = vm.stack().unwrap();
    assert_eq!(vm.run(0).unwrap(), RunOutcome::BudgetExhausted);
    assert_eq!(vm.stack().unwrap(), before);
    assert_eq!(vm.run(6).unwrap(), RunOutcome::BudgetExhausted);
    assert!(vm.stack().unwrap().is_empty()); // routing has happened, marker has not
    assert_eq!(vm.run(1).unwrap(), RunOutcome::BudgetExhausted);
    assert_eq!(
        vm.stack().unwrap(),
        vec![rml32::StackSnapshot::Value(Value::Int(1))]
    );
    returns(&mut vm, Value::Int(1));
    let (mut direct, o) = machine(p, vec![]);
    direct.start(o, 0, vec![]).unwrap();
    returns(&mut direct, Value::Int(1));
}

#[test]
fn catches_preparation_validates_targets_and_preserves_default_whitelist() {
    for offset in [-32768, 32767] {
        let p = image(&[fun(0, 0, 0, 255, &[(offset, 0, 0x2c)])], 0, &[]);
        assert_eq!(
            prepare_program(p).unwrap_err().kind,
            VmErrorKind::InvalidProgram
        );
    }
    let f = optional_fun(0, 1, 0, 255, &[vec![]], &[(-2, 0, 0x2c)]);
    let mut p = image(&[f], 0, &[]);
    optional_meta(&mut p, 0, 0, false, &[0]);
    assert_eq!(
        prepare_program(p).unwrap_err().kind,
        VmErrorKind::InvalidProgram
    );
    for op in [0x0e, 0x1a, 0x2c] {
        let f = optional_fun(0, 1, 0, 255, &[vec![(0, 0, op)]], &[]);
        let mut p = image(&[f], 0, &[]);
        optional_meta(&mut p, 0, 0, false, &[1]);
        assert_eq!(
            prepare_program(p).unwrap_err().kind,
            VmErrorKind::UnsupportedFeature
        );
    }
}

#[test]
fn catches_lifecycle_preparation_rejects_direct_and_all_linked_helper_bodies() {
    for op in [0x0e, 0x1a, 0x2c] {
        let mut p = image(&[fun(0, 0, 0, 255, &[(0, 0, op)])], 0, &[]);
        set_init(&mut p, &[(0, 0, 0x48)]);
        let mut c = init_catalog(p.clone());
        c.bind_lifecycle("root", Default::default()).unwrap();
        assert_eq!(
            prepare_init(&c, "root").unwrap_err().kind,
            VmErrorKind::UnsupportedFeature
        );
        assert_eq!(
            rml32::prepare_constructible_program(&c, "root", Default::default())
                .unwrap_err()
                .kind,
            VmErrorKind::UnsupportedFeature
        );
        p.functions.clear();
        set_init(&mut p, &[(0, 0, op), (0, 0, 0x48)]);
        assert_eq!(
            prepare_init(&init_catalog(p), "root").unwrap_err().kind,
            VmErrorKind::UnsupportedFeature
        );
    }
}

#[test]
fn catches_inside_numeric_loop_preserve_cursor_and_continue_after_each_error() {
    let trap = catch_fun(0, 0, 0, 255, INT_ZERO);
    let mut body = trap.nodes[2..trap.nodes.len() - 1].to_vec();
    body.extend([(0, 0, 0x33), (0, 0, 0x3e), (1, 0, 0x22)]);
    let (mut vm, o) = machine(
        image(&[numeric_loop(1, 3, 1, &body)], 1, &[]),
        vec![Value::Int(0)],
    );
    vm.start(o, 0, vec![]).unwrap();
    returns(&mut vm, Value::Int(4));
    assert_eq!(object_values(&vm, o), vec![Value::Int(3)]);
}

#[test]
fn catches_unwind_array_loop_helper_and_keep_shared_array_values() {
    let mut leaf = array_loop(INT_ZERO);
    leaf.index = 1;
    let caller = catch_fun(0, 1, 0, 255, &[(0, 0, 0x37), (1, 1, 0x07)]);
    let array = Value::array(vec![Value::Int(4), Value::Int(5)]).unwrap();
    let (mut vm, o) = machine(image(&[caller, leaf], 0, &[]), vec![]);
    vm.start(o, 0, vec![array.clone()]).unwrap();
    returns(&mut vm, Value::Int(1));
    let Value::Array(a) = array else {
        unreachable!()
    };
    assert_eq!(a.elements().unwrap(), vec![Value::Int(4), Value::Int(5)]);
}

#[test]
fn catches_cross_object_error_restores_caller_program_and_object_context() {
    let caller = image(
        &[catch_fun(
            0,
            1,
            0,
            255,
            &[(0, 0, 0x40), (0, 0, 0x37), (0, 0, 0x08)],
        )],
        0,
        &[b"fail\0"],
    );
    let target = other_target(
        image(&[fun(0, 0, 0, 255, INT_ZERO)], 0, &[]),
        vec![method(b"fail", 0)],
    );
    let (mut vm, a, b) = other_machine(caller, target, vec![], vec![]);
    vm.start(a, 0, vec![Value::Object(b)]).unwrap();
    returns(&mut vm, Value::Int(1));
}

#[test]
fn catches_inherited_body_uses_its_own_target_and_relocated_variables() {
    let mut body = vec![(23, 0, 0x3b), (0, 0, 0x4e), (0, 0, 0x33)];
    body.extend(INT_ZERO);
    let parent = image(&[catch_fun(0, 0, 0, 255, &body)], 1, &[]);
    let mut child = image(&[fun(1, 0, 0, 255, &[(0, 0, 0x07)])], 2, &[]);
    child.constitution_count = Some(2);
    inherit(&mut child, &parent, b"parent\0", [1, 0, 1]);
    let mut store = ObjectStore::new();
    let p = store
        .register_program(link(&linked_catalog(parent, child)).unwrap())
        .unwrap();
    let o = store
        .create(p, vec![Value::Int(100), Value::Int(0)])
        .unwrap();
    let mut vm = Vm::new(store);
    vm.start(o, 1, vec![]).unwrap();
    returns(&mut vm, Value::Int(1));
    assert_eq!(object_values(&vm, o), vec![Value::Int(100), Value::Int(23)]);
}

#[test]
fn catches_preserve_saved_evaluation_prefix_and_optional_argument_storage() {
    let trap = catch_fun(0, 0, 0, 255, INT_ZERO);
    let mut nodes = vec![(42, 0, 0x3b)];
    nodes.extend_from_slice(&trap.nodes[2..trap.nodes.len() - 1]);
    nodes.push((0, 0, 0x33));
    let (mut vm, o) = machine(image(&[fun(0, 0, 0, 255, &nodes)], 0, &[]), vec![]);
    vm.start(o, 0, vec![]).unwrap();
    returns(&mut vm, Value::Int(42));
    let mut body = trap.nodes[2..trap.nodes.len() - 1].to_vec();
    body.extend([(0, 0, 0x33), (0, 0, 0x37)]);
    let optional = optional_fun(0, 1, 0, 255, &[vec![(7, 0, 0x3b)]], &body);
    let mut p = image(&[optional], 0, &[]);
    optional_meta(&mut p, 0, 0, false, &[1]);
    for args in [vec![], vec![Value::Int(12)]] {
        let expected = if args.is_empty() { 7 } else { 12 };
        let (mut vm, o) = machine(p.clone(), vec![]);
        vm.start(o, 0, args).unwrap();
        returns(&mut vm, Value::Int(expected));
    }
}

#[test]
fn catches_marker_limit_remains_fatal_and_compound_zero_uses_handler() {
    let (vm, o) = machine(image(&[fun(0, 0, 1, 255, &[(0, 0, 0x0e)])], 0, &[]), vec![]);
    let mut vm = Vm::with_limits(
        vm.into_objects(),
        VmLimits {
            max_stack: 1,
            ..Default::default()
        },
    );
    vm.start(o, 0, vec![]).unwrap();
    assert_eq!(vm.run(100).unwrap_err().kind, VmErrorKind::Limit);
    let f = catch_fun(0, 0, 0, 255, &[(0, 0, 0x3e), (0, 0, 0x3b), (5, 0, 0x2f)]);
    let (mut vm, o) = machine(image(&[f], 1, &[]), vec![Value::Int(9)]);
    vm.start(o, 0, vec![]).unwrap();
    returns(&mut vm, Value::Int(1));
    assert_eq!(object_values(&vm, o), vec![Value::Int(9)]);
}

// Stage2W: per-site numeric zero faults; all programs remain synthetic.
fn zero_binary(opcode: u8, left: Value, right: Value) -> (Vm, rml32::ObjectId) {
    let body = [(0, 0, 0x37), (1, 0, 0x37), (0, 0, opcode)];
    let (mut vm, o) = machine(image(&[catch_fun(0, 2, 0, 255, &body)], 0, &[]), vec![]);
    vm.start(o, 0, vec![left, right]).unwrap();
    (vm, o)
}

#[test]
fn numeric_catch_division_accepts_numeric_pairs_and_both_real_zero_signs() {
    for left in [
        Value::Int(9),
        real(9.0),
        real(f32::NAN),
        real(f32::INFINITY),
    ] {
        for right in [Value::Int(0), real(0.0), real(-0.0)] {
            let (mut vm, _) = zero_binary(0x19, left.clone(), right);
            returns(&mut vm, Value::Int(1));
        }
    }
}

#[test]
fn numeric_catch_integer_remainder_zero_accepts_negative_and_minimum_dividends() {
    for left in [0, -9, i32::MIN, i32::MAX] {
        let (mut vm, _) = zero_binary(0x28, Value::Int(left), Value::Int(0));
        returns(&mut vm, Value::Int(1));
    }
}

#[test]
fn numeric_catch_nan_divisors_do_not_enter_zero_handlers() {
    for left in [Value::Int(7), real(7.0)] {
        let integer = matches!(left, Value::Int(_));
        let (mut vm, _) = zero_binary(0x19, left, real(f32::NAN));
        match vm.run(100).unwrap() {
            RunOutcome::Finished(Completion::Returned(Value::Int(v))) if integer => {
                assert_eq!(v, i32::MIN)
            }
            RunOutcome::Finished(Completion::Returned(Value::Real(v))) if !integer => {
                assert!(v.to_f32().is_nan())
            }
            other => panic!("NaN must follow arithmetic path: {other:?}"),
        }
    }
}

#[test]
fn numeric_catch_float_remainder_zero_keeps_fmod_nan_result() {
    for (left, right) in [
        (Value::Int(7), real(0.0)),
        (Value::Int(7), real(-0.0)),
        (real(7.0), Value::Int(0)),
        (real(7.0), real(-0.0)),
    ] {
        let (mut vm, _) = zero_binary(0x28, left, right);
        let RunOutcome::Finished(Completion::Returned(Value::Real(v))) = vm.run(100).unwrap()
        else {
            panic!("fmod result must be Real")
        };
        assert!(v.to_f32().is_nan());
    }
}

#[test]
fn numeric_catch_nonzero_divisors_keep_normal_numeric_results() {
    for (left, right, expected) in [
        (Value::Int(9), real(2.0), Value::Int(4)),
        (real(9.0), Value::Int(2), real(4.5)),
        (real(9.0), real(f32::INFINITY), real(0.0)),
    ] {
        let (mut vm, _) = zero_binary(0x19, left, right);
        returns(&mut vm, expected);
    }
}

#[test]
fn numeric_catch_compound_object_zero_preserves_every_supported_target_type() {
    for left in [
        Value::Int(9),
        real(9.0),
        real(f32::NAN),
        real(f32::INFINITY),
    ] {
        for right in [Value::Int(0), real(0.0), real(-0.0)] {
            let body = [(0, 0, 0x3e), (0, 0, 0x37), (5, 0, 0x2f)];
            let (mut vm, o) = machine(
                image(&[catch_fun(0, 1, 0, 255, &body)], 1, &[]),
                vec![left.clone()],
            );
            vm.start(o, 0, vec![right]).unwrap();
            returns(&mut vm, Value::Int(1));
            assert_eq!(object_values(&vm, o), vec![left.clone()]);
        }
    }
    let body = [(0, 0, 0x3e), (0, 0, 0x3b), (11, 0, 0x2f)];
    let (mut vm, o) = machine(
        image(&[catch_fun(0, 0, 0, 255, &body)], 1, &[]),
        vec![Value::Int(-9)],
    );
    vm.start(o, 0, vec![]).unwrap();
    returns(&mut vm, Value::Int(1));
    assert_eq!(object_values(&vm, o), vec![Value::Int(-9)]);
}

#[test]
fn numeric_catch_compound_local_keeps_value_and_does_not_commit_a_result() {
    for selector in [5, 11] {
        let trap = catch_fun(
            0,
            0,
            0,
            255,
            &[(-1, 0, 0x39), (0, 0, 0x3b), (selector, 0, 0x2f)],
        );
        let mut nodes = vec![(27, 0, 0x3b), (-1, 0, 0x4d), (0, 0, 0x33)];
        nodes.extend_from_slice(&trap.nodes[2..trap.nodes.len() - 1]);
        nodes.extend([(0, 0, 0x33), (-1, 0, 0x37)]);
        let (mut vm, o) = machine(image(&[fun(0, 0, 1, 255, &nodes)], 0, &[]), vec![]);
        vm.start(o, 0, vec![]).unwrap();
        returns(&mut vm, Value::Int(27));
    }
}

#[test]
fn numeric_catch_compound_members_release_discarded_leases_before_next_query() {
    for mapping in [false, true] {
        for selector in [5, 11] {
            let value = if mapping {
                Value::mapping(vec![
                    (Value::Int(0), Value::Int(9)),
                    (Value::Int(1), Value::Int(5)),
                ])
                .unwrap()
            } else {
                Value::array(vec![Value::Int(9), Value::Int(5)]).unwrap()
            };
            let trap = catch_fun(
                0,
                1,
                0,
                255,
                &[
                    (0, 0, 0x39),
                    (0, 0, 0x3b),
                    (0, 0, 0x44),
                    (0, 0, 0x3b),
                    (selector, 0, 0x2f),
                ],
            );
            let mut f = trap;
            f.nodes.pop();
            f.nodes.extend([
                (0, 0, 0x33),
                (0, 0, 0x39),
                (1, 0, 0x3b),
                (0, 0, 0x44),
                (0, 0, 0x33),
                (7, 0, 0x3b),
                (0, 0, 0x49),
            ]);
            let (vm, o) = machine(image(&[f], 0, &[]), vec![]);
            let mut vm = Vm::with_limits(
                vm.into_objects(),
                VmLimits {
                    max_reference_cells: 1,
                    ..Default::default()
                },
            );
            vm.start(o, 0, vec![value.clone()]).unwrap();
            returns(&mut vm, Value::Int(7));
            match value {
                Value::Array(a) => {
                    assert_eq!(a.elements().unwrap(), vec![Value::Int(9), Value::Int(5)])
                }
                Value::Mapping(m) => {
                    assert_eq!(m.get(&Value::Int(0)).unwrap(), Some(Value::Int(9)))
                }
                _ => unreachable!(),
            }
        }
    }
}

#[test]
fn numeric_catch_keeps_mapping_insertions_committed_before_later_arithmetic_error() {
    let mapping = Value::mapping(vec![(Value::Int(0), Value::Int(9))]).unwrap();
    let body = [
        (0, 0, 0x39),
        (99, 0, 0x3b),
        (0, 0, 0x44),
        (0, 0, 0x33),
        (0, 0, 0x39),
        (0, 0, 0x3b),
        (0, 0, 0x44),
        (0, 0, 0x3b),
        (11, 0, 0x2f),
    ];
    let (mut vm, o) = machine(image(&[catch_fun(0, 1, 0, 255, &body)], 0, &[]), vec![]);
    vm.start(o, 0, vec![mapping.clone()]).unwrap();
    returns(&mut vm, Value::Int(1));
    let Value::Mapping(m) = mapping else {
        unreachable!()
    };
    assert_eq!(m.get(&Value::Int(99)).unwrap(), Some(Value::Undefined));
    assert_eq!(m.get(&Value::Int(0)).unwrap(), Some(Value::Int(9)));
}

#[test]
fn numeric_catch_compound_integer_overflow_is_still_fatal_for_division_and_remainder() {
    for selector in [5, 11] {
        let body = [(0, 0, 0x3e), (-1, 0, 0x3b), (selector, 0, 0x2f)];
        let (mut vm, o) = machine(
            image(&[catch_fun(0, 0, 0, 255, &body)], 1, &[]),
            vec![Value::Int(i32::MIN)],
        );
        vm.start(o, 0, vec![]).unwrap();
        let e = vm.run(100).unwrap_err();
        assert_eq!(
            (e.kind, e.function, e.pc),
            (VmErrorKind::Arithmetic, Some(0), Some(5))
        );
        assert_eq!(vm.run(100).unwrap_err(), e);
        assert_eq!(object_values(&vm, o), vec![Value::Int(i32::MIN)]);
    }
}

#[test]
fn numeric_catch_unhandled_compound_zero_preserves_operands_and_location() {
    for selector in [5, 11] {
        let body = [(0, 0, 0x3e), (0, 0, 0x3b), (selector, 0, 0x2f)];
        let (mut vm, o) = machine(
            image(&[fun(0, 0, 0, 255, &body)], 1, &[]),
            vec![Value::Int(9)],
        );
        vm.start(o, 0, vec![]).unwrap();
        let e = vm.run(100).unwrap_err();
        assert_eq!((e.kind, e.pc), (VmErrorKind::Arithmetic, Some(4)));
        assert!(e.offset.is_some());
        let stack = vm.stack().unwrap();
        assert!(matches!(
            stack[0],
            rml32::StackSnapshot::Reference { valid: true, .. }
        ));
        assert_eq!(stack[1], rml32::StackSnapshot::Value(Value::Int(0)));
        assert_eq!(object_values(&vm, o), vec![Value::Int(9)]);
        assert_eq!(vm.run(0).unwrap_err(), e);
    }
}

#[test]
fn numeric_catch_nested_mixed_zero_then_remainder_matches_single_step_budget() {
    let inner = catch_fun(0, 2, 0, 255, &[(0, 0, 0x37), (1, 0, 0x37), (0, 0, 0x19)]);
    let mut body = inner.nodes[2..inner.nodes.len() - 1].to_vec();
    body.extend([(0, 0, 0x33), (7, 0, 0x3b), (0, 0, 0x3b), (0, 0, 0x28)]);
    let p = image(&[catch_fun(0, 2, 0, 255, &body)], 0, &[]);
    let (mut vm, o) = machine(p.clone(), vec![]);
    vm.start(o, 0, vec![real(8.0), real(-0.0)]).unwrap();
    returns(&mut vm, Value::Int(1));
    let (mut stepped, o) = machine(p, vec![]);
    stepped.start(o, 0, vec![real(8.0), real(-0.0)]).unwrap();
    let mut result = None;
    for _ in 0..100 {
        assert_eq!(stepped.run(0).unwrap(), RunOutcome::BudgetExhausted);
        if let RunOutcome::Finished(c) = stepped.run(1).unwrap() {
            result = Some(c);
            break;
        }
    }
    assert_eq!(result, Some(Completion::Returned(Value::Int(1))));
}

#[test]
fn numeric_catch_cross_object_compound_remainder_restores_caller_without_target_write() {
    let caller = image(
        &[catch_fun(
            0,
            1,
            0,
            255,
            &[(0, 0, 0x40), (0, 0, 0x37), (0, 0, 0x08)],
        )],
        0,
        &[b"fail\0"],
    );
    let target = other_target(
        image(
            &[fun(
                0,
                0,
                0,
                255,
                &[(0, 0, 0x3e), (0, 0, 0x3b), (11, 0, 0x2f)],
            )],
            1,
            &[],
        ),
        vec![method(b"fail", 0)],
    );
    let (mut vm, a, b) = other_machine(caller, target, vec![], vec![Value::Int(13)]);
    vm.start(a, 0, vec![Value::Object(b)]).unwrap();
    returns(&mut vm, Value::Int(1));
    assert_eq!(object_values(&vm, b), vec![Value::Int(13)]);
}

#[test]
fn numeric_catch_handled_source_does_not_make_later_type_error_catchable() {
    let inner = catch_fun(0, 0, 0, 255, &[(9, 0, 0x3b), (0, 0, 0x3b), (0, 0, 0x28)]);
    let mut body = inner.nodes[2..inner.nodes.len() - 1].to_vec();
    body.extend([(0, 0, 0x33), (0, 0, 0x40), (0, 0, 0x3b), (0, 0, 0x19)]);
    let (mut vm, o) = machine(
        image(&[catch_fun(0, 0, 0, 255, &body)], 0, &[b"not numeric\0"]),
        vec![],
    );
    vm.start(o, 0, vec![]).unwrap();
    let e = vm.run(100).unwrap_err();
    assert_eq!(e.kind, VmErrorKind::UnsupportedType);
    assert_eq!(vm.run(1).unwrap_err(), e);
}

#[test]
fn numeric_catch_compound_float_remainder_zero_and_nan_divisor_follow_normal_path() {
    for (selector, left, right, integer_result) in [
        (11, real(7.0), Value::Int(0), false),
        (11, Value::Int(7), real(-0.0), false),
        (5, Value::Int(7), real(f32::NAN), true),
        (5, real(7.0), real(f32::NAN), false),
    ] {
        let body = [(0, 0, 0x3e), (0, 0, 0x37), (selector, 0, 0x2f)];
        let (mut vm, o) = machine(image(&[catch_fun(0, 1, 0, 255, &body)], 1, &[]), vec![left]);
        vm.start(o, 0, vec![right]).unwrap();
        let RunOutcome::Finished(Completion::Returned(result)) = vm.run(100).unwrap() else {
            panic!("expected return")
        };
        if integer_result {
            assert_eq!(result, Value::Int(i32::MIN));
        } else {
            let Value::Real(r) = &result else {
                panic!("expected floating result")
            };
            assert!(r.to_f32().is_nan());
        }
        assert_eq!(object_values(&vm, o), vec![result]);
    }
}

#[test]
fn numeric_catch_unhandled_real_zero_preserves_diagnostic_and_negative_zero_bits() {
    let body = [(0, 0, 0x37), (1, 0, 0x37), (0, 0, 0x19)];
    let (mut vm, o) = machine(image(&[fun(0, 2, 0, 255, &body)], 0, &[]), vec![]);
    vm.start(o, 0, vec![real(7.0), real(-0.0)]).unwrap();
    let e = vm.run(100).unwrap_err();
    assert_eq!(
        (e.kind, e.function, e.pc),
        (VmErrorKind::Arithmetic, Some(0), Some(4))
    );
    assert_eq!(&*e.message, "divide by zero");
    let stack = vm.stack().unwrap();
    assert_eq!(
        stack[stack.len() - 1],
        rml32::StackSnapshot::Value(real(-0.0))
    );
    assert_eq!(vm.run(100).unwrap_err(), e);
}

#[test]
fn numeric_catch_reference_quota_failure_before_zero_remains_fatal() {
    let body = [
        (0, 0, 0x39),
        (0, 0, 0x3b),
        (0, 0, 0x44),
        (0, 0, 0x3b),
        (5, 0, 0x2f),
    ];
    let (vm, o) = machine(image(&[catch_fun(0, 1, 0, 255, &body)], 0, &[]), vec![]);
    let mut vm = Vm::with_limits(
        vm.into_objects(),
        VmLimits {
            max_reference_cells: 0,
            ..Default::default()
        },
    );
    vm.start(o, 0, vec![Value::array(vec![Value::Int(9)]).unwrap()])
        .unwrap();
    let e = vm.run(100).unwrap_err();
    assert_eq!((e.kind, e.pc), (VmErrorKind::Limit, Some(5)));
    assert_eq!(vm.run(100).unwrap_err(), e);
}

// Stage 2Z: direct reference shape checks, using synthetic programs only.
#[test]
fn ref_checks_plain_values_extreme_indices_and_ignored_par8() {
    for index in [0, 1, i16::MAX, i16::MIN, -1, -2] {
        let (mut vm, o) = machine(
            image(&[fun(0, 1, 0, 255, &[(index, -128, 0x10)])], 0, &[]),
            vec![],
        );
        vm.start(o, 0, vec![Value::Int(9)]).unwrap();
        returns(&mut vm, Value::Int(0));
    }
}

#[test]
fn ref_checks_nested_forwarded_reference_preserves_source() {
    let caller = fun(0, 1, 0, 255, &[(0, 0, 0x39), (1, 1, 0x07)]);
    let middle = fun(1, 1, 0, 255, &[(0, 0, 0x39), (2, 1, 0x07)]);
    let leaf = fun(2, 1, 0, 255, &[(0, 127, 0x10)]);
    let mut p = image(&[caller, middle, leaf], 0, &[]);
    ref_arg(&mut p, 1, 0, 255, 0);
    ref_arg(&mut p, 2, 0, 255, 0);
    let (mut vm, o) = machine(p, vec![]);
    vm.start(o, 0, vec![Value::Int(9)]).unwrap();
    returns(&mut vm, Value::Int(1));
    assert_eq!(vm.frame_depth(), 0);
}

#[test]
fn ref_checks_negative_local_and_calculation_slots_before_result_push() {
    for (locals, body, expected) in [
        (1, vec![(-1, 0, 0x10)], 0),
        (0, vec![(0, 0, 0x39), (-1, 0, 0x10)], 1),
        (1, vec![(0, 0, 0x39), (-2, 0, 0x10)], 1),
        (0, vec![(0, 0, 0x39), (-2, 0, 0x10)], 0),
        (0, vec![(9, 0, 0x3b), (-1, 0, 0x10)], 0),
    ] {
        let (mut vm, o) = machine(image(&[fun(0, 1, locals, 255, &body)], 0, &[]), vec![]);
        vm.start(o, 0, vec![Value::Int(7)]).unwrap();
        vm.run(2 + body.len()).unwrap();
        assert_eq!(
            vm.stack().unwrap().last(),
            Some(&rml32::StackSnapshot::Value(Value::Int(expected)))
        );
    }
}

#[test]
fn ref_checks_optional_default_and_variadic_slots() {
    for (defaults, lengths, variadic, argc) in [
        (vec![], vec![0], false, 0),
        (vec![vec![(7, 0, 0x3b)]], vec![1], false, 0),
        (vec![], vec![0], false, 1),
        (vec![], vec![0], true, 3),
    ] {
        let f = optional_fun(
            0,
            1,
            0,
            255,
            &defaults,
            &[(0, 0, 0x10), (2, 0, 0x10), (2, 0, 0x16)],
        );
        let mut p = image(&[f], 0, &[]);
        optional_meta(&mut p, 0, 0, variadic, &lengths);
        let (mut vm, o) = machine(p, vec![]);
        vm.start(o, 0, vec![Value::Int(9); argc]).unwrap();
        assert_eq!(returned_array(&mut vm), vec![Value::Int(0), Value::Int(0)]);
    }
    let caller = fun(0, 1, 0, 255, &[(0, 0, 0x39), (1, 1, 0x07)]);
    let leaf = optional_fun(
        1,
        2,
        0,
        255,
        &[vec![], vec![(8, 0, 0x3b)]],
        &[(0, 0, 0x10), (1, 0, 0x10), (2, 0, 0x16)],
    );
    let mut p = image(&[caller, leaf], 0, &[]);
    optional_meta(&mut p, 1, 1, false, &[0, 1]);
    ref_arg(&mut p, 1, 0, 255, 0);
    let (mut vm, o) = machine(p, vec![]);
    vm.start(o, 0, vec![Value::Int(9)]).unwrap();
    assert_eq!(returned_array(&mut vm), vec![Value::Int(0), Value::Int(1)]);
}

#[test]
fn ref_checks_budget_and_stack_limit_preserve_operand_identity() {
    let p = image(&[fun(0, 1, 0, 255, &[(0, 0, 0x39), (-1, 0, 0x10)])], 0, &[]);
    let (mut vm, o) = machine(p.clone(), vec![]);
    vm.start(o, 0, vec![Value::Int(8)]).unwrap();
    vm.run(3).unwrap();
    let before = vm.stack().unwrap();
    assert_eq!(vm.run(0).unwrap(), RunOutcome::BudgetExhausted);
    assert_eq!(vm.stack().unwrap(), before);
    assert_eq!(vm.run(1).unwrap(), RunOutcome::BudgetExhausted);
    let after = vm.stack().unwrap();
    assert_eq!(&after[..before.len()], &before);
    assert_eq!(
        after.last(),
        Some(&rml32::StackSnapshot::Value(Value::Int(1)))
    );
    let (vm, o) = machine(p, vec![]);
    let mut vm = Vm::with_limits(
        vm.into_objects(),
        VmLimits {
            max_stack: 2,
            ..Default::default()
        },
    );
    vm.start(o, 0, vec![Value::Int(8)]).unwrap();
    vm.run(3).unwrap();
    let before = vm.stack().unwrap();
    let e = vm.run(1).unwrap_err();
    assert_eq!(
        (e.kind, e.object, e.function, e.pc),
        (VmErrorKind::Limit, Some(o), Some(0), Some(3))
    );
    assert_eq!(vm.stack().unwrap(), before);
    assert_eq!(vm.run(100).unwrap_err(), e);
}

#[test]
fn ref_checks_inherited_and_cross_object_frames() {
    let (parent, child) = inherited_pair(&[(-1, 0, 0x10)], &[(0, 0, 0x07)]);
    let (mut vm, o) = linked_vm(&linked_catalog(parent, child));
    vm.start(o, 0, vec![]).unwrap();
    returns(&mut vm, Value::Int(0));
    let target = other_target(
        image(&[fun(0, 1, 0, 255, &[(0, 0, 0x10)])], 0, &[]),
        vec![method(b"check", 0)],
    );
    let (mut vm, a, b) = other_machine(
        image(&[other_wrapper(1, 0)], 0, &[]),
        target,
        vec![],
        vec![],
    );
    vm.start(
        a,
        0,
        vec![Value::Object(b), Value::bytes(b"check"), Value::Int(5)],
    )
    .unwrap();
    loop {
        if let RunOutcome::Finished(result) = vm.run(1).unwrap() {
            assert_eq!(result, Completion::Returned(Value::Int(0)));
            break;
        }
    }
    assert_eq!(vm.frame_depth(), 0);
    let mut parent = image(&[fun(0, 1, 0, 255, &[(0, 0, 0x10)])], 1, &[]);
    ref_arg(&mut parent, 0, 0, 255, 0);
    let mut child = image(&[fun(1, 1, 0, 255, &[(0, 0, 0x39), (0, 1, 0x07)])], 2, &[]);
    child.constitution_count = Some(2);
    inherit(&mut child, &parent, b"parent\0", [1, 0, 1]);
    let (mut vm, o) = linked_vm(&linked_catalog(parent, child));
    vm.start(o, 1, vec![Value::Int(8)]).unwrap();
    returns(&mut vm, Value::Int(1));

    let caller = image(
        &[fun(
            0,
            2,
            0,
            255,
            &[(1, 0, 0x39), (0, 0, 0x40), (0, 0, 0x37), (0, 1, 0x08)],
        )],
        0,
        &[b"check\0"],
    );
    let mut target = image(&[fun(0, 1, 0, 255, &[(0, 0, 0x10)])], 0, &[]);
    ref_arg(&mut target, 0, 0, 255, 0);
    let (mut vm, a, b) = other_machine(
        caller,
        other_target(target, vec![method(b"check", 0)]),
        vec![],
        vec![],
    );
    vm.start(a, 0, vec![Value::Object(b), Value::Int(8)])
        .unwrap();
    returns(&mut vm, Value::Int(1));
}

#[test]
fn ref_checks_stack_limit_inside_catch_remains_fatal() {
    let body = [(8, 0, 0x3b), (0, 0, 0x10)];
    let (vm, o) = machine(image(&[catch_fun(0, 0, 0, 255, &body)], 0, &[]), vec![]);
    let mut vm = Vm::with_limits(
        vm.into_objects(),
        VmLimits {
            max_stack: 1,
            ..Default::default()
        },
    );
    vm.start(o, 0, vec![]).unwrap();
    let e = vm.run(100).unwrap_err();
    assert_eq!(e.kind, VmErrorKind::Limit);
    assert_eq!(e.pc, Some(4));
    assert_eq!(
        vm.stack().unwrap(),
        vec![rml32::StackSnapshot::Value(Value::Int(8))]
    );
    assert_eq!(vm.run(0).unwrap_err(), e);
}

#[test]
fn ref_checks_preparation_rejects_init_defaults_and_dynamic_variant() {
    let mut p = image(&[], 0, &[]);
    set_init(&mut p, &[(0, 0, 0x10), (0, 0, 0x48)]);
    let e = prepare_init(&init_catalog(p), "root").unwrap_err();
    assert_eq!(
        (e.kind, e.offset),
        (VmErrorKind::UnsupportedFeature, Some(9000))
    );
    let mut p = image(
        &[optional_fun(0, 1, 0, 255, &[vec![(0, 0, 0x10)]], &[])],
        0,
        &[],
    );
    optional_meta(&mut p, 0, 0, false, &[1]);
    assert_eq!(
        prepare_program(p).unwrap_err().kind,
        VmErrorKind::UnsupportedFeature
    );
    assert_eq!(
        prepare_program(image(&[fun(0, 0, 0, 255, &[(0, 0, 0x11)])], 0, &[]))
            .unwrap_err()
            .kind,
        VmErrorKind::UnsupportedFeature
    );
}

#[test]
fn ref_checks_lifecycle_helper_rejection_cleans_initialization_and_construction() {
    let helper = fun(
        2,
        0,
        0,
        1,
        &[
            (0, 0, 0x40),
            (0, 0, 0x3b),
            (0, 0, 0x3b),
            (0, 0, 0x10),
            (0, 0, 0x33),
        ],
    );
    let mut p = image(
        &[
            fun(0, 0, 0, 1, &[(2, 0, 0x07)]),
            fun(1, 0, 0, 1, &[]),
            helper,
        ],
        0,
        &[b"x"],
    );
    set_init(&mut p, &[(2, 0, 0x07), (0, 0, 0x48)]);
    let (mut vm, id) = init_vm(&init_catalog(p.clone()), "root");
    let o = vm.begin_initialize(id).unwrap();
    let e = vm.run(100).unwrap_err();
    assert_eq!(e.kind, VmErrorKind::UnsupportedFeature);
    assert_eq!(e.function, Some(2));
    assert!(vm.objects().get(o).is_err());
    assert!(vm.stack().unwrap().is_empty());
    lifecycle_name(&mut p, 0, b"create");
    lifecycle_name(&mut p, 1, b"destruct");
    set_init(&mut p, &[(0, 0, 0x48)]);
    let c = single_construct_catalog(
        p,
        rml32::LifecycleBinding {
            create: Some(0),
            destruct: Some(1),
        },
    );
    let (mut vm, id) = construct_vm(&c, "root");
    let o = vm.begin_construct(id, None).unwrap();
    let e = vm.run(100).unwrap_err();
    assert_eq!(e.kind, VmErrorKind::UnsupportedFeature);
    assert_eq!(
        e.lifecycle.as_ref().unwrap().phase,
        rml32::LifecyclePhase::Create
    );
    assert!(vm.rollback_errors().is_empty());
    assert!(vm.objects().get(o).is_err());
    assert!(vm.stack().unwrap().is_empty());
}
#[test]
fn ref_checks_rollback_helper_rejection_preserves_primary_error_and_cleans_task() {
    let mut p = image(
        &[
            fun(0, 0, 0, 1, &[(0, 0, 0x33)]),
            fun(1, 0, 0, 1, &[(2, 0, 0x07)]),
            fun(
                2,
                0,
                0,
                1,
                &[(0, 0, 0x40), (0, 0, 0x3b), (0, 0, 0x3b), (0, 0, 0x10)],
            ),
        ],
        0,
        &[b"x"],
    );
    lifecycle_name(&mut p, 0, b"create");
    lifecycle_name(&mut p, 1, b"destruct");
    set_init(&mut p, &[(0, 0, 0x48)]);
    let c = single_construct_catalog(
        p,
        rml32::LifecycleBinding {
            create: Some(0),
            destruct: Some(1),
        },
    );
    let (mut vm, id) = construct_vm(&c, "root");
    let o = vm.begin_construct(id, None).unwrap();
    assert_eq!(vm.run(100).unwrap_err().kind, VmErrorKind::StackUnderflow);
    assert_eq!(vm.rollback_errors().len(), 1);
    let e = &vm.rollback_errors()[0];
    assert_eq!(e.kind, VmErrorKind::UnsupportedFeature);
    assert_eq!(e.function, Some(2));
    assert_eq!(
        e.lifecycle.as_ref().unwrap().phase,
        rml32::LifecyclePhase::RollbackDestruct
    );
    assert!(vm.objects().get(o).is_err());
    assert!(vm.stack().unwrap().is_empty());
}

// Stage 2AB: explicit synthetic banks only; no real global programs are executed.
fn global_machine(p: ProgramImage, values: Vec<Value>, slots: usize) -> (Vm, rml32::ObjectId) {
    let mut store = ObjectStore::new();
    let vars = p.max_vars as usize;
    let prepared =
        rml32::prepare_program_with_globals(p, rml32::GlobalBankSchema { slots }).unwrap();
    let program = store.register_program(prepared).unwrap();
    let object = store.create(program, vec![Value::Undefined; vars]).unwrap();
    store.register_globals(values).unwrap();
    (Vm::new(store), object)
}

#[test]
fn globals_preparation_is_explicit_and_checks_signed_slot_boundaries() {
    use rml32::{GlobalBankSchema, prepare_program_with_globals};
    for op in [0x35, 0x36] {
        for (slots, slot, expected) in [
            (0, 0, Some(VmErrorKind::InvalidProgram)),
            (1, -1, Some(VmErrorKind::InvalidProgram)),
            (1, 1, Some(VmErrorKind::InvalidProgram)),
            (32768, 32767, None),
            (32769, 0, Some(VmErrorKind::Limit)),
        ] {
            let p = image(&[fun(0, 0, 0, 255, &[(slot, -128, op)])], 0, &[]);
            assert_eq!(
                prepare_program(p.clone()).unwrap_err().kind,
                VmErrorKind::UnsupportedFeature
            );
            let result = prepare_program_with_globals(p, GlobalBankSchema { slots });
            if let Some(kind) = expected {
                let error = result.unwrap_err();
                assert_eq!(error.kind, kind);
                if kind == VmErrorKind::InvalidProgram {
                    assert_eq!(error.pc, Some(2));
                    assert!(error.offset.is_some());
                }
            } else {
                assert!(result.is_ok());
            }
        }
    }
    assert!(
        prepare_program_with_globals(
            image(&[fun(0, 0, 0, 1, &[])], 0, &[]),
            GlobalBankSchema { slots: 0 }
        )
        .is_ok()
    );
}

#[test]
fn globals_preparation_does_not_enable_other_global_features_or_init() {
    {
        let p = image(&[fun(0, 0, 0, 255, &[(0, 0, 0x06)])], 0, &[]);
        assert_eq!(
            rml32::prepare_program_with_globals(p, rml32::GlobalBankSchema { slots: 1 })
                .unwrap_err()
                .kind,
            VmErrorKind::UnsupportedFeature
        );
    }
    let mut p = image(&[fun(0, 0, 0, 255, &[(4, 0, 0x3b)])], 0, &[]);
    p.mode = ProgramMode::Global;
    assert_eq!(
        rml32::prepare_program_with_globals(p.clone(), rml32::GlobalBankSchema { slots: 1 })
            .unwrap_err()
            .kind,
        VmErrorKind::UnsupportedFeature
    );
    p.mode = ProgramMode::Object;
    p.init.nodes = vec![(0, 0, 0x35)]; // Ignored by this preparation path.
    let (mut vm, o) = global_machine(p, vec![Value::Int(8)], 1);
    vm.start(o, 0, vec![]).unwrap();
    returns(&mut vm, Value::Int(4));
}

#[test]
fn globals_store_install_remove_empty_and_capacity_contracts() {
    let mut s = ObjectStore::new();
    assert_eq!(
        s.global_values().unwrap_err().kind,
        VmErrorKind::InvalidState
    );
    assert_eq!(
        s.remove_globals().unwrap_err().kind,
        VmErrorKind::InvalidState
    );
    assert_eq!(
        s.register_globals(vec![Value::Undefined; 32769])
            .unwrap_err()
            .kind,
        VmErrorKind::Limit
    );
    assert!(s.global_values().is_err());
    s.register_globals(vec![]).unwrap();
    assert_eq!(s.global_values().unwrap(), vec![]);
    assert_eq!(
        s.register_globals(vec![]).unwrap_err().kind,
        VmErrorKind::InvalidState
    );
    s.remove_globals().unwrap();
    s.register_globals(vec![Value::Undefined; 32768]).unwrap();
    assert_eq!(s.global_values().unwrap().len(), 32768);
    s.remove_globals().unwrap();
    let mut nested = Value::Int(1);
    for _ in 0..64 {
        nested = Value::array(vec![nested]).unwrap();
    }
    s.register_globals(vec![nested]).unwrap(); // No artificial 65th bank level.
}

#[test]
fn globals_missing_and_mismatched_banks_report_current_instruction() {
    for values in [None, Some(vec![]), Some(vec![Value::Int(1), Value::Int(2)])] {
        let p = rml32::prepare_program_with_globals(
            image(&[fun(0, 0, 0, 255, &[(0, 0, 0x35)])], 0, &[]),
            rml32::GlobalBankSchema { slots: 1 },
        )
        .unwrap();
        let mut s = ObjectStore::new();
        let id = s.register_program(p).unwrap();
        let o = s.create(id, vec![]).unwrap();
        if let Some(values) = values {
            s.register_globals(values).unwrap();
        }
        let mut vm = Vm::new(s);
        vm.start(o, 0, vec![]).unwrap();
        let e = vm.run(100).unwrap_err();
        assert_eq!(e.kind, VmErrorKind::InvalidState);
        assert_eq!(e.pc, Some(2));
        assert!(e.message.contains("global slot 0"));
        assert_eq!(e.object, Some(o));
    }
}

#[test]
fn globals_read_is_shallow_and_normalizes_only_direct_deleted_objects() {
    let shared = [
        Value::bytes(b"shared"),
        Value::array(vec![Value::Int(7)]).unwrap(),
        Value::mapping(vec![(Value::Int(1), Value::Int(9))]).unwrap(),
    ];
    for value in shared {
        let (mut vm, o) = global_machine(
            image(&[fun(0, 0, 0, 255, &[(0, 127, 0x35)])], 0, &[]),
            vec![value.clone()],
            1,
        );
        vm.start(o, 0, vec![]).unwrap();
        returns(&mut vm, value.clone());
        assert_eq!(vm.objects().global_values().unwrap(), vec![value]);
    }
    let dead = deleted_global_test_object();
    for (op, expected) in [(0x35, Value::Undefined), (0x36, dead.clone())] {
        let (mut vm, o) = global_machine(
            image(
                &[fun(0, 0, 0, 255, &[(0, 0, op), (0, 0, 0x33), (7, 0, 0x3b)])],
                0,
                &[],
            ),
            vec![dead.clone()],
            1,
        );
        assert_eq!(vm.objects().global_values().unwrap(), vec![dead.clone()]);
        vm.start(o, 0, vec![]).unwrap();
        returns(&mut vm, Value::Int(7));
        assert_eq!(vm.objects().global_values().unwrap(), vec![expected]);
    }
    let nested = Value::array(vec![dead]).unwrap();
    let (mut vm, o) = global_machine(
        image(&[fun(0, 0, 0, 255, &[(0, 0, 0x35)])], 0, &[]),
        vec![nested.clone()],
        1,
    );
    vm.start(o, 0, vec![]).unwrap();
    returns(&mut vm, nested);
}

#[test]
fn globals_full_stack_failure_preserves_dead_source_and_operands() {
    let dead = deleted_global_test_object();
    let (vm, o) = global_machine(
        image(&[fun(0, 0, 0, 255, &[(1, 0, 0x3b), (0, 0, 0x35)])], 0, &[]),
        vec![dead.clone()],
        1,
    );
    let mut vm = Vm::with_limits(
        vm.into_objects(),
        VmLimits {
            max_stack: 1,
            ..VmLimits::default()
        },
    );
    vm.start(o, 0, vec![]).unwrap();
    assert_eq!(vm.run(3).unwrap(), RunOutcome::BudgetExhausted);
    let before = vm.stack().unwrap();
    assert_eq!(vm.run(1).unwrap_err().kind, VmErrorKind::Limit);
    assert_eq!(vm.stack().unwrap(), before);
    assert_eq!(vm.objects().global_values().unwrap(), vec![dead]);
}

#[test]
fn globals_root_write_and_compound_update_do_not_consume_member_quota() {
    let nodes = [
        (0, 0, 0x36),
        (9, 0, 0x3b),
        (0, 0, 0x04),
        (0, 0, 0x33),
        (0, 0, 0x36),
        (1, 0, 0x22),
        (0, 0, 0x35),
    ];
    let (vm, o) = global_machine(
        image(&[fun(0, 0, 0, 255, &nodes)], 0, &[]),
        vec![Value::Int(1)],
        1,
    );
    let mut vm = Vm::with_limits(
        vm.into_objects(),
        VmLimits {
            max_reference_cells: 0,
            ..VmLimits::default()
        },
    );
    vm.start(o, 0, vec![]).unwrap();
    returns(&mut vm, Value::Int(10));
    assert_eq!(vm.objects().global_values().unwrap(), vec![Value::Int(10)]);
}

#[test]
fn globals_nested_reference_parameters_write_through_without_bank_checks() {
    let caller = fun(
        0,
        0,
        0,
        255,
        &[(0, 0, 0x36), (1, 1, 0x07), (0, 0, 0x33), (0, 0, 0x35)],
    );
    let forward = fun(1, 1, 0, 255, &[(0, 0, 0x39), (2, 1, 0x07)]);
    let callee = fun(2, 1, 0, 1, &[(9, 0, 0x3b), (0, 0, 0x4d), (0, 0, 0x33)]);
    let mut p = image(&[caller, forward, callee], 0, &[]);
    ref_arg(&mut p, 1, 0, 2, 0);
    ref_arg(&mut p, 2, 0, 2, 0);
    let (mut vm, o) = global_machine(p, vec![Value::Int(3)], 1);
    vm.start(o, 0, vec![]).unwrap();
    returns(&mut vm, Value::Int(9));
}

#[test]
fn globals_old_root_survives_removal_and_new_bank_has_distinct_identity() {
    use rml32::{ReferenceKind, StackSnapshot};
    let nodes = [
        (0, 0, 0x36),
        (0, 0, 0x14),
        (0, 0, 0x33),
        (0, 0, 0x36),
        (0, 0, 0x33),
        (0, 0, 0x35),
    ];
    let (mut vm, o) = global_machine(
        image(&[fun(0, 0, 0, 255, &nodes)], 0, &[]),
        vec![Value::Int(3)],
        1,
    );
    vm.start(o, 0, vec![]).unwrap();
    vm.run(3).unwrap();
    let old = match vm.stack().unwrap()[0] {
        StackSnapshot::Reference {
            kind: ReferenceKind::Global,
            target_id,
            valid: true,
        } => target_id,
        _ => panic!(),
    };
    vm.objects_mut().remove_globals().unwrap();
    vm.run(1).unwrap();
    // 14 consumes the detached root and reads its retained value.
    assert_eq!(
        vm.stack().unwrap(),
        vec![StackSnapshot::Value(Value::Int(3))]
    );
    vm.objects_mut()
        .register_globals(vec![Value::Int(8)])
        .unwrap();
    vm.run(2).unwrap();
    let new = match vm.stack().unwrap()[0] {
        StackSnapshot::Reference {
            kind: ReferenceKind::Global,
            target_id,
            valid: true,
        } => target_id,
        _ => panic!(),
    };
    assert_ne!(old, new);
    returns(&mut vm, Value::Int(8));
}

#[test]
fn globals_detached_root_writes_do_not_alias_reinstalled_bank() {
    let nodes = [
        (0, 0, 0x36),
        (9, 0, 0x3b),
        (0, 0, 0x04),
        (0, 0, 0x33),
        (0, 0, 0x35),
    ];
    let (mut vm, o) = global_machine(
        image(&[fun(0, 0, 0, 255, &nodes)], 0, &[]),
        vec![Value::Int(3)],
        1,
    );
    vm.start(o, 0, vec![]).unwrap();
    vm.run(3).unwrap();
    vm.objects_mut().remove_globals().unwrap();
    vm.objects_mut()
        .register_globals(vec![Value::Int(8)])
        .unwrap();
    vm.run(2).unwrap();
    assert_eq!(
        vm.stack().unwrap(),
        vec![rml32::StackSnapshot::Value(Value::Int(9))]
    );
    assert_eq!(vm.objects().global_values().unwrap(), vec![Value::Int(8)]);
    returns(&mut vm, Value::Int(8));
}

#[test]
fn globals_detached_container_members_keep_existing_mapping_array_lifetimes() {
    for (container, expected) in [
        (Value::array(vec![Value::Int(7)]).unwrap(), Value::Int(7)),
        (
            Value::mapping(vec![(Value::Int(0), Value::Int(7))]).unwrap(),
            Value::Undefined,
        ),
    ] {
        let nodes = [(0, 0, 0x36), (0, 0, 0x3b), (0, 0, 0x44), (0, 0, 0x14)];
        let (mut vm, o) = global_machine(
            image(&[fun(0, 0, 0, 255, &nodes)], 0, &[]),
            vec![container],
            1,
        );
        vm.start(o, 0, vec![]).unwrap();
        vm.run(3).unwrap();
        vm.objects_mut().remove_globals().unwrap();
        returns(&mut vm, expected);
    }
}

#[test]
fn globals_member_queries_still_obey_member_quota() {
    let nodes = [(0, 0, 0x36), (0, 0, 0x3b), (0, 0, 0x44)];
    let (vm, o) = global_machine(
        image(&[fun(0, 0, 0, 255, &nodes)], 0, &[]),
        vec![Value::array(vec![Value::Int(7)]).unwrap()],
        1,
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
    assert_eq!(vm.objects().global_values().unwrap().len(), 1);
}

#[test]
fn globals_objects_share_bank_but_keep_independent_object_variables() {
    let nodes = [(0, 0, 0x36), (1, 0, 0x22), (0, 0, 0x35), (0, 0, 0x4e)];
    let (mut vm, a) = global_machine(
        image(&[fun(0, 0, 0, 255, &nodes)], 1, &[]),
        vec![Value::Int(0)],
        1,
    );
    let program = vm.objects().get(a).unwrap().program();
    let b = vm
        .objects_mut()
        .create(program, vec![Value::Int(99)])
        .unwrap();
    vm.start(a, 0, vec![]).unwrap();
    returns(&mut vm, Value::Int(1));
    vm.start(b, 0, vec![]).unwrap();
    returns(&mut vm, Value::Int(2));
    assert_eq!(
        vm.objects().get(a).unwrap().variables().unwrap(),
        vec![Value::Int(1)]
    );
    assert_eq!(
        vm.objects().get(b).unwrap().variables().unwrap(),
        vec![Value::Int(2)]
    );
    assert_eq!(
        vm.into_objects().global_values().unwrap(),
        vec![Value::Int(2)]
    );
}

#[test]
fn globals_zero_and_single_step_budgets_match_continuous_execution() {
    for step in [false, true] {
        let (mut vm, o) = global_machine(
            image(
                &[fun(
                    0,
                    0,
                    0,
                    255,
                    &[(0, 0, 0x36), (1, 0, 0x22), (0, 0, 0x35)],
                )],
                0,
                &[],
            ),
            vec![Value::Int(1)],
            1,
        );
        vm.start(o, 0, vec![]).unwrap();
        let before = vm.stack().unwrap();
        assert_eq!(vm.run(0).unwrap(), RunOutcome::BudgetExhausted);
        assert_eq!(vm.stack().unwrap(), before);
        assert_eq!(vm.objects().global_values().unwrap(), vec![Value::Int(1)]);
        if step {
            loop {
                if let RunOutcome::Finished(result) = vm.run(1).unwrap() {
                    assert_eq!(result, Completion::Returned(Value::Int(2)));
                    break;
                }
            }
        } else {
            returns(&mut vm, Value::Int(2));
        }
    }
}

fn deleted_global_test_object() -> Value {
    let (mut vm, id) = machine(image(&[fun(0, 0, 0, 1, &[])], 0, &[]), vec![]);
    vm.objects_mut().remove(id).unwrap();
    // Use a second identity so it cannot alias the first object in the test VM.
    let p = vm
        .objects_mut()
        .register_program(prepare_program(image(&[fun(0, 0, 0, 1, &[])], 0, &[])).unwrap())
        .unwrap();
    let id = vm.objects_mut().create(p, vec![]).unwrap();
    vm.objects_mut().remove(id).unwrap();
    Value::Object(id)
}

#[test]
fn globals_reference_shape_and_loop_binding_reuse_existing_rules() {
    let caller = fun(0, 0, 0, 255, &[(0, 0, 0x36), (1, 1, 0x07)]);
    let leaf = fun(1, 1, 0, 255, &[(0, 127, 0x10)]);
    let mut p = image(&[caller, leaf], 0, &[]);
    ref_arg(&mut p, 1, 0, 255, 0);
    let (mut vm, o) = global_machine(p, vec![Value::Int(7)], 1);
    vm.start(o, 0, vec![]).unwrap();
    returns(&mut vm, Value::Int(1));
    let mut f = numeric_loop(1, 3, 1, &[]);
    f.nodes[5] = (0, 0, 0x36); // Loop variable binds the global root.
    let last = f.nodes.len() - 2;
    f.nodes[last] = (0, 0, 0x35);
    let (mut vm, o) = global_machine(image(&[f], 0, &[]), vec![Value::Int(1)], 1);
    vm.start(o, 0, vec![]).unwrap();
    returns(&mut vm, Value::Int(4));
    assert_eq!(vm.objects().global_values().unwrap(), vec![Value::Int(4)]);
}

#[test]
fn globals_failures_inside_catch_remain_fatal_and_do_not_expand_catch_sources() {
    let (mut vm, o) = global_machine(
        image(&[catch_fun(0, 0, 0, 255, &[(0, 0, 0x35)])], 0, &[]),
        vec![Value::Int(1)],
        1,
    );
    vm.objects_mut().remove_globals().unwrap();
    vm.start(o, 0, vec![]).unwrap();
    let e = vm.run(100).unwrap_err();
    assert_eq!(e.kind, VmErrorKind::InvalidState);
    assert_eq!(vm.run(100).unwrap_err(), e);
}

#[test]
fn globals_default_and_lifecycle_preparation_stay_closed() {
    let f = optional_fun(0, 1, 0, 255, &[vec![(0, 0, 0x35)]], &[(0, 0, 0x37)]);
    let mut p = image(&[f], 0, &[]);
    optional_meta(&mut p, 0, 0, false, &[1]);
    assert_eq!(
        rml32::prepare_program_with_globals(p, rml32::GlobalBankSchema { slots: 1 })
            .unwrap_err()
            .kind,
        VmErrorKind::UnsupportedFeature
    );
    for op in [0x35, 0x36] {
        let mut p = image(&[fun(0, 0, 0, 1, &[])], 0, &[]);
        set_init(&mut p, &[(0, 0, op), (0, 0, 0x48)]);
        let c = init_catalog(p);
        assert_eq!(
            rml32::prepare_initializable_program(&c, "root", rml32::LinkLimits::default())
                .unwrap_err()
                .kind,
            VmErrorKind::UnsupportedFeature
        );
    }
}

#[test]
fn globals_cross_object_frames_check_the_callee_schema() {
    use rml32::{GlobalBankSchema, MethodBinding, prepare_program_with_globals};
    let caller = image(
        &[fun(
            0,
            1,
            0,
            255,
            &[(0, 0, 0x40), (0, 0, 0x37), (0, 0, 0x08)],
        )],
        0,
        &[b"read\0"],
    );
    for slots in [1, 2] {
        let target = prepare_program_with_globals(
            image(&[fun(0, 0, 0, 255, &[(0, 0, 0x35)])], 0, &[]),
            GlobalBankSchema { slots },
        )
        .unwrap()
        .with_methods(vec![MethodBinding {
            name: b"read".to_vec(),
            slot: 0,
        }])
        .unwrap();
        let (mut vm, o) = global_machine(caller.clone(), vec![Value::Int(7)], 1);
        let p = vm.objects_mut().register_program(target).unwrap();
        let target = vm.objects_mut().create(p, vec![]).unwrap();
        vm.start(o, 0, vec![Value::Object(target)]).unwrap();
        if slots == 1 {
            returns(&mut vm, Value::Int(7));
        } else {
            let e = vm.run(100).unwrap_err();
            assert_eq!(e.kind, VmErrorKind::InvalidState);
            assert_eq!(e.object, Some(target));
        }
    }
}

#[test]
fn globals_highest_signed_slot_executes_and_new_reads_require_current_bank() {
    let p = image(&[fun(0, 0, 0, 255, &[(32767, -128, 0x35)])], 0, &[]);
    let mut values = vec![Value::Undefined; 32768];
    values[32767] = Value::Int(42);
    let (mut vm, o) = global_machine(p, values, 32768);
    vm.start(o, 0, vec![]).unwrap();
    returns(&mut vm, Value::Int(42));
    let (mut vm, o) = global_machine(
        image(
            &[fun(
                0,
                0,
                0,
                255,
                &[(0, 0, 0x36), (0, 0, 0x14), (0, 0, 0x33), (0, 0, 0x35)],
            )],
            0,
            &[],
        ),
        vec![Value::Int(3)],
        1,
    );
    vm.start(o, 0, vec![]).unwrap();
    vm.run(3).unwrap();
    vm.objects_mut().remove_globals().unwrap();
    vm.run(2).unwrap();
    assert_eq!(vm.run(1).unwrap_err().kind, VmErrorKind::InvalidState);
}

#[test]
fn globals_do_not_change_legacy_programs_that_do_not_access_the_bank() {
    let (mut vm, o) = machine(image(&[fun(0, 0, 0, 255, &[(9, 0, 0x3b)])], 0, &[]), vec![]);
    for values in [vec![], vec![Value::Int(7)]] {
        vm.objects_mut().register_globals(values).unwrap();
        vm.start(o, 0, vec![]).unwrap();
        returns(&mut vm, Value::Int(9));
        vm.objects_mut().remove_globals().unwrap();
    }
    vm.start(o, 0, vec![]).unwrap();
    returns(&mut vm, Value::Int(9));
}

// Stage 2AC: all Global images below are serialized synthetic Magic559 programs.
fn bind_synthetic_globals(vm: &mut Vm, p: ProgramImage, slots: usize) -> rml32::ProgramId {
    let p = rml32::prepare_global_program(p, rml32::GlobalBankSchema { slots }).unwrap();
    let id = vm.objects_mut().register_program(p).unwrap();
    vm.objects_mut().bind_global_program(id).unwrap();
    id
}
fn global_call_machine(global: ProgramImage) -> (Vm, rml32::ObjectId, rml32::ProgramId) {
    let caller = image(&[fun(0, 0, 0, 255, &[(559, -128, 0x07)])], 0, &[]);
    let slots = global.max_vars as usize;
    let (mut vm, o) = global_machine(caller, vec![Value::Undefined; slots], slots);
    let id = bind_synthetic_globals(&mut vm, global, slots);
    (vm, o, id)
}

#[test]
fn global_calls_preparation_checks_kind_schema_function_interval_and_duplicates() {
    use rml32::{GlobalBankSchema, prepare_global_program};
    let good = global_image(&[fun(559, 0, 0, 255, &[(0, 0, 0x41)])], 1, &[]);
    assert!(prepare_global_program(good.clone(), GlobalBankSchema { slots: 1 }).is_ok());
    for (case, kind) in [
        (0, VmErrorKind::UnsupportedFeature),
        (1, VmErrorKind::InvalidProgram),
        (2, VmErrorKind::InvalidProgram),
        (3, VmErrorKind::InvalidProgram),
        (4, VmErrorKind::InvalidProgram),
        (5, VmErrorKind::InvalidProgram),
    ] {
        let mut p = good.clone();
        match case {
            0 => p.mode = ProgramMode::Object,
            1 => p.function_base = 558,
            2 => p.function_base = 561,
            3 => p.functions[0].index = Some(558),
            4 => p.functions.push(p.functions[0].clone()),
            _ => p.max_vars = 2,
        }
        assert_eq!(
            prepare_global_program(p, GlobalBankSchema { slots: 1 })
                .unwrap_err()
                .kind,
            kind
        );
    }
    let mut p = good;
    p.efuns_count = 558;
    assert_eq!(
        prepare_global_program(p, GlobalBankSchema { slots: 1 })
            .unwrap_err()
            .kind,
        VmErrorKind::InvalidProgram
    );
}

#[test]
fn global_calls_binding_is_explicit_and_global_code_cannot_create_objects() {
    let mut s = ObjectStore::new();
    assert_eq!(
        s.clear_global_program().unwrap_err().kind,
        VmErrorKind::InvalidState
    );
    let object = s
        .register_program(prepare_program(image(&[fun(0, 0, 0, 1, &[])], 0, &[])).unwrap())
        .unwrap();
    assert_eq!(
        s.bind_global_program(object).unwrap_err().kind,
        VmErrorKind::UnsupportedFeature
    );
    let p = rml32::prepare_global_program(
        global_image(&[fun(559, 0, 0, 1, &[])], 0, &[]),
        rml32::GlobalBankSchema { slots: 0 },
    )
    .unwrap();
    let global = s.register_program(p).unwrap();
    assert_eq!(
        s.create(global, vec![]).unwrap_err().kind,
        VmErrorKind::UnsupportedFeature
    );
    s.bind_global_program(global).unwrap();
    assert_eq!(
        s.bind_global_program(global).unwrap_err().kind,
        VmErrorKind::InvalidState
    );
    s.clear_global_program().unwrap();
    s.bind_global_program(global).unwrap(); // No variable bank required.
}

#[test]
fn global_calls_legacy_and_global_local_calls_stay_rejected() {
    let p = image(&[fun(0, 0, 0, 255, &[(559, -128, 0x07)])], 0, &[]);
    assert_eq!(
        prepare_program(p.clone()).unwrap_err().kind,
        VmErrorKind::UnsupportedFeature
    );
    assert!(rml32::prepare_program_with_globals(p, rml32::GlobalBankSchema { slots: 0 }).is_ok());
    for op in [(559, 0, 0x07), (0, 0, 0x06)] {
        let e = rml32::prepare_global_program(
            global_image(&[fun(559, 0, 0, 255, &[op])], 0, &[]),
            rml32::GlobalBankSchema { slots: 0 },
        )
        .unwrap_err();
        assert_eq!(e.kind, VmErrorKind::UnsupportedFeature);
        assert_eq!(e.pc, Some(2));
    }
}

#[test]
fn global_calls_absolute_slots_flags_and_missing_bindings_have_distinct_errors() {
    for (slot, expected) in [
        (-1, Some(VmErrorKind::Bounds)),
        (560, Some(VmErrorKind::Bounds)),
        (558, Some(VmErrorKind::UnsupportedFeature)),
        (559, None),
    ] {
        for flags in [-128, -64] {
            // bit6 ignored; argc remains zero.
            let caller = image(&[fun(0, 0, 0, 255, &[(slot, flags, 0x07)])], 0, &[]);
            let (mut vm, o) = global_machine(caller, vec![], 0);
            bind_synthetic_globals(
                &mut vm,
                global_image(&[fun(559, 0, 0, 255, &[(9, 0, 0x3b)])], 0, &[]),
                0,
            );
            vm.start(o, 0, vec![]).unwrap();
            if let Some(kind) = expected {
                let e = vm.run(100).unwrap_err();
                assert_eq!(e.kind, kind);
                assert_eq!(e.object, Some(o));
                assert_eq!(e.pc, Some(2));
            } else {
                returns(&mut vm, Value::Int(9));
            }
        }
    }
    let (mut vm, o) = global_machine(
        image(&[fun(0, 0, 0, 255, &[(559, -128, 0x07)])], 0, &[]),
        vec![],
        0,
    );
    vm.start(o, 0, vec![]).unwrap();
    assert_eq!(vm.run(100).unwrap_err().kind, VmErrorKind::InvalidState);
    let mut p = global_image(&[fun(572, 0, 0, 255, &[(9, 0, 0x3b)])], 0, &[]);
    p.function_base = 572;
    let (mut vm, o, _) = global_call_machine(p);
    vm.start(o, 0, vec![]).unwrap();
    assert_eq!(
        vm.run(100).unwrap_err().kind,
        VmErrorKind::UnsupportedFeature
    );
}

#[test]
fn global_calls_clear_object_context_and_restore_object_on_return() {
    let caller = image(
        &[fun(
            0,
            0,
            0,
            255,
            &[(559, -128, 0x07), (0, 0, 0x33), (0, 0, 0x41)],
        )],
        0,
        &[],
    );
    let (mut vm, o) = global_machine(caller, vec![], 0);
    bind_synthetic_globals(
        &mut vm,
        global_image(&[fun(559, 0, 0, 2, &[(0, 0, 0x41)])], 0, &[]),
        0,
    );
    vm.start(o, 0, vec![]).unwrap();
    vm.run(6).unwrap();
    assert_eq!(
        vm.stack().unwrap(),
        vec![rml32::StackSnapshot::Value(Value::Int(0))]
    );
    returns(&mut vm, Value::Object(o));
}

#[test]
fn global_calls_nested_global_functions_use_absolute_slots_and_own_string_pool() {
    let global = global_image(
        &[
            fun(559, 0, 0, 255, &[(560, -128, 0x07)]),
            fun(560, 0, 0, 255, &[(0, 0, 0x40)]),
        ],
        0,
        &[b"global\0"],
    );
    let caller = image(
        &[fun(
            0,
            0,
            0,
            255,
            &[(559, -128, 0x07), (0, 0, 0x40), (2, 0, 0x16)],
        )],
        0,
        &[b"object\0"],
    );
    let (mut vm, o) = global_machine(caller, vec![], 0);
    bind_synthetic_globals(&mut vm, global, 0);
    vm.start(o, 0, vec![]).unwrap();
    let RunOutcome::Finished(Completion::Returned(Value::Array(values))) = vm.run(100).unwrap()
    else {
        panic!()
    };
    assert_eq!(
        values.elements().unwrap(),
        vec![Value::bytes(b"object"), Value::bytes(b"global")]
    );
}

#[test]
fn global_calls_object_variable_instructions_fail_without_fake_object_identity() {
    for opcode in [0x3d, 0x3e, 0x4e] {
        let mut nodes = vec![];
        if opcode == 0x4e {
            nodes.push((7, 0, 0x3b));
        }
        nodes.push((0, 0, opcode));
        let (mut vm, o, id) =
            global_call_machine(global_image(&[fun(559, 0, 0, 255, &nodes)], 1, &[]));
        vm.start(o, 0, vec![]).unwrap();
        let e = vm.run(100).unwrap_err();
        assert_eq!(e.kind, VmErrorKind::UnsupportedFeature);
        assert_eq!(e.object, None);
        assert_eq!(e.program, Some(id));
        assert_eq!(e.function, Some(559));
        assert!(e.offset.is_some());
    }
}

#[test]
fn global_calls_can_call_object_methods_and_restore_global_none_context() {
    let global = global_image(
        &[fun(
            559,
            1,
            0,
            255,
            &[
                (0, 0, 0x40),
                (0, 0, 0x37),
                (0, 0, 0x08),
                (0, 0, 0x33),
                (0, 0, 0x41),
            ],
        )],
        0,
        &[b"who\0"],
    );
    let caller = image(
        &[fun(0, 1, 0, 255, &[(0, 0, 0x37), (559, -127, 0x07)])],
        0,
        &[],
    );
    let (mut vm, o) = global_machine(caller, vec![], 0);
    bind_synthetic_globals(&mut vm, global, 0);
    let p = other_target(
        image(&[fun(0, 0, 0, 255, &[(0, 0, 0x41)])], 0, &[]),
        vec![rml32::MethodBinding {
            name: b"who".to_vec(),
            slot: 0,
        }],
    );
    let p = vm.objects_mut().register_program(p).unwrap();
    let target = vm.objects_mut().create(p, vec![]).unwrap();
    vm.start(o, 0, vec![Value::Object(target)]).unwrap();
    assert_eq!(vm.run(13).unwrap(), RunOutcome::BudgetExhausted);
    assert_eq!(
        vm.stack().unwrap().last(),
        Some(&rml32::StackSnapshot::Value(Value::Object(target)))
    );
    returns(&mut vm, Value::Int(0));
}

#[test]
fn global_calls_active_frames_survive_clear_and_new_calls_use_rebinding() {
    let old = global_image(
        &[
            fun(559, 0, 0, 255, &[(1, 0, 0x3b)]),
            fun(560, 0, 0, 255, &[(2, 0, 0x3b)]),
        ],
        0,
        &[],
    );
    let new = global_image(
        &[
            fun(559, 0, 0, 255, &[(8, 0, 0x3b)]),
            fun(560, 0, 0, 255, &[(9, 0, 0x3b)]),
        ],
        0,
        &[],
    );
    let caller = image(
        &[fun(
            0,
            0,
            0,
            255,
            &[(559, -128, 0x07), (560, -128, 0x07), (0, 0, 0x01)],
        )],
        0,
        &[],
    );
    let (mut vm, o) = global_machine(caller, vec![], 0);
    bind_synthetic_globals(&mut vm, old, 0);
    vm.start(o, 0, vec![]).unwrap();
    vm.run(3).unwrap();
    vm.objects_mut().clear_global_program().unwrap();
    vm.run(2).unwrap(); // Already-entered code remains executable without binding.
    bind_synthetic_globals(&mut vm, new, 0);
    returns(&mut vm, Value::Int(10));
}

#[test]
fn global_calls_deleted_caller_does_not_invalidate_active_global_code() {
    let (mut vm, o, _) = global_call_machine(global_image(
        &[fun(559, 0, 0, 255, &[(7, 0, 0x3b)])],
        0,
        &[],
    ));
    vm.start(o, 0, vec![]).unwrap();
    vm.run(3).unwrap();
    vm.objects_mut().remove(o).unwrap();
    assert_eq!(vm.run(4).unwrap(), RunOutcome::BudgetExhausted); // 0f,1b,push,return all use Global code.
    let e = vm.run(1).unwrap_err();
    assert_eq!(e.kind, VmErrorKind::InvalidObject);
    assert_eq!(e.object, Some(o));
    assert_eq!(e.function, Some(0));
}

#[test]
fn global_calls_global_errors_have_actual_program_pool_and_no_object() {
    let (mut vm, o, id) = global_call_machine(global_image(
        &[fun(
            559,
            0,
            0,
            255,
            &[(0, 0, 0x40), (1, 0, 0x3b), (0, 0, 0x01)],
        )],
        0,
        &[b"text\0"],
    ));
    vm.start(o, 0, vec![]).unwrap();
    let e = vm.run(100).unwrap_err();
    assert_eq!(e.kind, VmErrorKind::UnsupportedType);
    assert_eq!(e.program, Some(id));
    assert_eq!(e.object, None);
    assert_eq!(e.function, Some(559));
    assert_eq!(e.pc, Some(4));
}

#[test]
fn global_calls_catch_unwinds_both_execution_sources_and_keeps_correct_context() {
    // Object catches a Global arithmetic fault; Global catches an Object fault.
    let caller = image(&[catch_fun(0, 0, 0, 255, &[(559, -128, 0x07)])], 0, &[]);
    let (mut vm, o) = global_machine(caller, vec![], 0);
    bind_synthetic_globals(
        &mut vm,
        global_image(&[fun(559, 0, 0, 255, INT_ZERO)], 0, &[]),
        0,
    );
    vm.start(o, 0, vec![]).unwrap();
    returns(&mut vm, Value::Int(1));
    let caller = image(
        &[fun(0, 1, 0, 255, &[(0, 0, 0x37), (559, -127, 0x07)])],
        0,
        &[],
    );
    let global = global_image(
        &[catch_fun(
            559,
            1,
            0,
            255,
            &[(0, 0, 0x40), (0, 0, 0x37), (0, 0, 0x08)],
        )],
        0,
        &[b"fail\0"],
    );
    let (mut vm, o) = global_machine(caller, vec![], 0);
    bind_synthetic_globals(&mut vm, global, 0);
    let p = other_target(
        image(&[fun(0, 0, 0, 255, INT_ZERO)], 0, &[]),
        vec![rml32::MethodBinding {
            name: b"fail".to_vec(),
            slot: 0,
        }],
    );
    let id = vm.objects_mut().register_program(p).unwrap();
    let target = vm.objects_mut().create(id, vec![]).unwrap();
    vm.start(o, 0, vec![Value::Object(target)]).unwrap();
    returns(&mut vm, Value::Int(1));
}

#[test]
fn global_calls_reference_parameters_forward_global_roots_and_object_locals() {
    let caller = fun(
        0,
        0,
        1,
        255,
        &[
            (0, 0, 0x36),
            (559, -127, 0x07),
            (0, 0, 0x33),
            (-1, 0, 0x39),
            (559, -127, 0x07),
            (0, 0, 0x33),
            (-1, 0, 0x37),
        ],
    );
    let forward = fun(559, 1, 0, 255, &[(0, 0, 0x39), (560, -127, 0x07)]);
    let callee = fun(560, 1, 0, 1, &[(9, 0, 0x3b), (0, 0, 0x4d), (0, 0, 0x33)]);
    let mut global = global_image(&[forward, callee], 1, &[]);
    ref_arg(&mut global, 0, 0, 255, 0);
    ref_arg(&mut global, 1, 0, 255, 0);
    let (mut vm, o) = global_machine(image(&[caller], 0, &[]), vec![Value::Int(1)], 1);
    bind_synthetic_globals(&mut vm, global, 1);
    vm.start(o, 0, vec![]).unwrap();
    returns(&mut vm, Value::Int(9));
    assert_eq!(vm.objects().global_values().unwrap(), vec![Value::Int(9)]);
}

#[test]
fn global_calls_parameter_preflight_failure_preserves_caller_operands_and_frames() {
    for reference in [false, true] {
        let nodes = if reference {
            vec![(0, 0, 0x36), (559, -127, 0x07)]
        } else {
            vec![(0, 0, 0x40), (559, -127, 0x07)]
        };
        let caller = image(&[fun(0, 0, 0, 255, &nodes)], 0, &[b"wrong\0"]);
        let (mut vm, o) = global_machine(caller, vec![Value::bytes(b"wrong")], 1);
        let mut global = global_image(&[fun(559, 1, 0, 1, &[])], 1, &[]);
        if reference {
            ref_arg(&mut global, 0, 0, 2, 0);
        } else {
            global.functions[0].arguments[0].r#type = 2;
        }
        bind_synthetic_globals(&mut vm, global, 1);
        vm.start(o, 0, vec![]).unwrap();
        vm.run(3).unwrap();
        let before = vm.stack().unwrap();
        let e = vm.run(1).unwrap_err();
        assert_eq!(e.kind, VmErrorKind::UnsupportedType);
        assert_eq!(e.object, Some(o));
        assert_eq!(vm.stack().unwrap(), before);
        assert_eq!(vm.frame_depth(), 1);
        assert_eq!(
            vm.objects().global_values().unwrap(),
            vec![Value::bytes(b"wrong")]
        );
    }
}

#[test]
fn global_calls_prototype_limits_return_failures_and_catch_sources_remain_bounded() {
    for (argc, ret, body, kind) in [
        (1, 255, vec![(7, 0, 0x3b)], VmErrorKind::ArgumentCount),
        (0, 2, vec![(0, 0, 0x40)], VmErrorKind::UnsupportedType),
    ] {
        let (mut vm, o, id) = global_call_machine(global_image(
            &[fun(559, argc, 0, ret, &body)],
            0,
            &[b"wrong\0"],
        ));
        vm.start(o, 0, vec![]).unwrap();
        let e = vm.run(100).unwrap_err();
        assert_eq!(e.kind, kind);
        assert_eq!(e.object, if argc == 1 { Some(o) } else { None });
        if argc == 0 {
            assert_eq!(e.program, Some(id));
        }
    }
    let (vm, o, _) = global_call_machine(global_image(
        &[fun(559, 0, 0, 255, &[(7, 0, 0x3b)])],
        0,
        &[],
    ));
    let mut vm = Vm::with_limits(
        vm.into_objects(),
        VmLimits {
            max_frames: 1,
            ..VmLimits::default()
        },
    );
    vm.start(o, 0, vec![]).unwrap();
    assert_eq!(vm.run(100).unwrap_err().kind, VmErrorKind::Limit);
    let (mut vm, o) = global_machine(
        image(&[catch_fun(0, 0, 0, 255, &[(559, -128, 0x07)])], 0, &[]),
        vec![],
        0,
    );
    vm.start(o, 0, vec![]).unwrap();
    assert_eq!(vm.run(100).unwrap_err().kind, VmErrorKind::InvalidState);
}

#[test]
fn global_calls_variable_access_uses_global_schema_and_current_bank() {
    let global = global_image(
        &[fun(
            559,
            0,
            0,
            255,
            &[(0, 0, 0x36), (1, 0, 0x22), (0, 0, 0x35)],
        )],
        1,
        &[],
    );
    let (mut vm, o, _) = global_call_machine(global);
    vm.objects_mut().remove_globals().unwrap();
    vm.objects_mut()
        .register_globals(vec![Value::Int(6)])
        .unwrap();
    vm.start(o, 0, vec![]).unwrap();
    returns(&mut vm, Value::Int(7));
    let (mut vm, o) = global_machine(
        image(&[fun(0, 0, 0, 255, &[(559, -128, 0x07)])], 0, &[]),
        vec![],
        0,
    );
    bind_synthetic_globals(
        &mut vm,
        global_image(&[fun(559, 0, 0, 255, &[(0, 0, 0x35)])], 1, &[]),
        1,
    );
    vm.start(o, 0, vec![]).unwrap();
    let e = vm.run(100).unwrap_err();
    assert_eq!(e.kind, VmErrorKind::InvalidState);
    assert_eq!(e.object, None);
}

#[test]
fn global_calls_global_init_is_ignored_and_default_whitelist_does_not_expand() {
    let mut global = global_image(&[fun(559, 0, 0, 255, &[(7, 0, 0x3b)])], 0, &[]);
    global.init.nodes = vec![(559, 0, 0x06)];
    let (mut vm, o, _) = global_call_machine(global);
    vm.start(o, 0, vec![]).unwrap();
    returns(&mut vm, Value::Int(7));
    let f = optional_fun(559, 1, 0, 255, &[vec![(7, 0, 0x3b)]], &[(0, 0, 0x37)]);
    let mut global = global_image(&[f], 0, &[]);
    optional_meta(&mut global, 0, 0, false, &[1]);
    let (mut vm, o, _) = global_call_machine(global.clone());
    vm.start(o, 0, vec![]).unwrap();
    returns(&mut vm, Value::Int(7));
    global.max_vars = 1;
    global.functions[0].code.nodes[1] = (0, 0, 0x35);
    assert_eq!(
        rml32::prepare_global_program(global, rml32::GlobalBankSchema { slots: 1 })
            .unwrap_err()
            .kind,
        VmErrorKind::UnsupportedFeature
    );
}

#[test]
fn global_calls_zero_and_single_step_budgets_match_continuous_execution() {
    for step in [false, true] {
        let global = global_image(
            &[
                fun(559, 0, 0, 255, &[(560, -128, 0x07)]),
                fun(560, 0, 0, 255, &[(0, 0, 0x41)]),
            ],
            0,
            &[],
        );
        let (mut vm, o, _) = global_call_machine(global);
        vm.start(o, 0, vec![]).unwrap();
        let before = vm.stack().unwrap();
        assert_eq!(vm.run(0).unwrap(), RunOutcome::BudgetExhausted);
        assert_eq!(vm.stack().unwrap(), before);
        if step {
            loop {
                if let RunOutcome::Finished(result) = vm.run(1).unwrap() {
                    assert_eq!(result, Completion::Returned(Value::Int(0)));
                    break;
                }
            }
        } else {
            returns(&mut vm, Value::Int(0));
        }
    }
}

#[test]
fn global_calls_preflight_does_not_normalize_earlier_deleted_object_before_later_type_error() {
    let caller = image(
        &[fun(
            0,
            2,
            0,
            255,
            &[(1, 0, 0x37), (0, 0, 0x37), (559, -126, 0x07)],
        )],
        0,
        &[],
    );
    let (mut vm, o) = global_machine(caller, vec![], 0);
    let mut global = global_image(&[fun(559, 2, 0, 1, &[])], 0, &[]);
    global.functions[0].arguments[1].r#type = 2;
    bind_synthetic_globals(&mut vm, global, 0);
    let program = vm.objects().get(o).unwrap().program();
    let target = vm.objects_mut().create(program, vec![]).unwrap();
    let dead = Value::Object(target);
    vm.start(o, 0, vec![dead.clone(), Value::bytes(b"wrong")])
        .unwrap();
    vm.run(4).unwrap();
    vm.objects_mut().remove(target).unwrap();
    let before = vm.stack().unwrap();
    assert_eq!(vm.run(1).unwrap_err().kind, VmErrorKind::UnsupportedType);
    assert_eq!(vm.stack().unwrap(), before);
    assert_eq!(vm.frame_depth(), 1);
    assert!(before.contains(&rml32::StackSnapshot::Value(dead)));
}

#[test]
fn global_calls_code_binding_and_variable_bank_lifetimes_are_independent() {
    let global = global_image(
        &[
            fun(559, 0, 0, 255, &[(560, -128, 0x07)]),
            fun(560, 0, 0, 255, &[(7, 0, 0x3b)]),
        ],
        0,
        &[],
    );
    let (mut vm, o, _) = global_call_machine(global);
    vm.objects_mut().remove_globals().unwrap();
    vm.start(o, 0, vec![]).unwrap();
    vm.run(3).unwrap();
    vm.objects_mut().clear_global_program().unwrap(); // Active Global code survives; its next call cannot resolve a binding.
    let e = vm.run(100).unwrap_err();
    assert_eq!(e.kind, VmErrorKind::InvalidState);
    assert_eq!(e.object, None);
    assert_eq!(e.function, Some(559));
    let (mut vm, o, _) = global_call_machine(global_image(
        &[fun(559, 0, 0, 255, &[(7, 0, 0x3b)])],
        0,
        &[],
    ));
    vm.objects_mut().remove_globals().unwrap();
    vm.start(o, 0, vec![]).unwrap();
    returns(&mut vm, Value::Int(7));
    let mut s = vm.into_objects();
    assert!(s.global_values().is_err());
    s.clear_global_program().unwrap();
}

// Stage 2AE: synthetic synchronous callbacks only; no native efun is executed.
use rml32::{HostCallContext, HostError, HostFunction, HostReturn, HostSignature, HostType};
use std::{cell::Cell as HostCounter, rc::Rc as HostRc};

fn host_binding(
    required: Vec<HostType>,
    returns: HostReturn,
    callback: impl Fn(&HostCallContext, &[Value]) -> Result<Option<Value>, HostError> + 'static,
) -> HostFunction {
    HostFunction {
        name: "synthetic".into(),
        signature: HostSignature {
            required,
            optional: vec![],
            rest: None,
            returns,
        },
        callback: HostRc::new(callback),
    }
}
fn host_machine(p: ProgramImage, binding: HostFunction) -> (Vm, rml32::ObjectId) {
    let vars = p.max_vars as usize;
    let p = rml32::prepare_program_with_host_calls(p, None).unwrap();
    let mut store = ObjectStore::new();
    let p = store.register_program(p).unwrap();
    let o = store.create(p, vec![Value::Undefined; vars]).unwrap();
    store.register_host_function(0, binding).unwrap();
    (Vm::new(store), o)
}
fn counted_host(counter: &HostRc<HostCounter<usize>>, returns: HostReturn) -> HostFunction {
    let counter = counter.clone();
    host_binding(vec![], returns, move |_, _| {
        counter.set(counter.get() + 1);
        Ok(Some(Value::Int(9)))
    })
}
#[test]
fn host_registry_validates_slots_names_signatures_and_preserves_registration_on_failure() {
    let make = || host_binding(vec![], HostReturn::Mixed, |_, _| Ok(None));
    let mut store = ObjectStore::new();
    for slot in [0, 558] {
        store.register_host_function(slot, make()).unwrap();
    }
    assert_eq!(
        store.register_host_function(0, make()).unwrap_err().kind,
        VmErrorKind::InvalidState
    );
    for slot in [559, u16::MAX] {
        assert_eq!(
            store.register_host_function(slot, make()).unwrap_err().kind,
            VmErrorKind::Bounds
        );
        assert_eq!(
            store.remove_host_function(slot).unwrap_err().kind,
            VmErrorKind::Bounds
        );
    }
    for name in ["", "bad\0name"] {
        let mut f = make();
        f.name = name.into();
        assert_eq!(
            store.register_host_function(1, f).unwrap_err().kind,
            VmErrorKind::InvalidProgram
        );
    }
    for returns in [
        HostReturn::Exact(HostType::Any),
        HostReturn::Exact(HostType::Void),
    ] {
        let mut f = make();
        f.signature.returns = returns;
        assert_eq!(
            store.register_host_function(1, f).unwrap_err().kind,
            VmErrorKind::InvalidProgram
        );
    }
    let mut f = make();
    f.signature.required = vec![HostType::Any; 127];
    f.signature.optional.push(HostType::Any);
    assert_eq!(
        store.register_host_function(1, f).unwrap_err().kind,
        VmErrorKind::Limit
    );
    store.register_host_function(1, make()).unwrap();
    assert!(!format!("{store:?}").contains("callback"));
    store.remove_host_function(0).unwrap();
    assert_eq!(
        store.remove_host_function(0).unwrap_err().kind,
        VmErrorKind::InvalidState
    );
}
#[test]
fn host_preparation_requires_opt_in_and_validates_signed_operands() {
    use rml32::{
        GlobalBankSchema, prepare_global_program_with_host_calls, prepare_program_with_host_calls,
    };
    for (slot, argc) in [(0, 0), (558, 127), (0, 64)] {
        let p = image(&[fun(0, 0, 0, 255, &[(slot, argc, 6)])], 0, &[]);
        assert_eq!(
            prepare_program(p.clone()).unwrap_err().kind,
            VmErrorKind::UnsupportedFeature
        );
        assert_eq!(
            rml32::prepare_program_with_globals(p.clone(), GlobalBankSchema { slots: 0 })
                .unwrap_err()
                .kind,
            VmErrorKind::UnsupportedFeature
        );
        assert!(prepare_program_with_host_calls(p, None).is_ok());
    }
    for (slot, argc) in [(-1, 0), (559, 0), (0, -1), (0, -128)] {
        // The read-only parser already rejects bad efun slots; mutate a parsed
        // synthetic image to exercise the preparation layer independently.
        let mut p = image(&[fun(0, 0, 0, 255, &[(0, 0, 6)])], 0, &[]);
        p.functions[0].code.nodes[2] = (slot, argc, 6);
        let offset = p.functions[0].code.nodes_offset + 8;
        let e = prepare_program_with_host_calls(p, None).unwrap_err();
        assert_eq!(
            (e.kind, e.function, e.pc, e.offset),
            (VmErrorKind::InvalidProgram, Some(0), Some(2), Some(offset))
        );
    }
    let global = global_image(&[fun(559, 0, 0, 255, &[(0, 0, 6)])], 0, &[]);
    assert_eq!(
        rml32::prepare_global_program(global.clone(), GlobalBankSchema { slots: 0 })
            .unwrap_err()
            .kind,
        VmErrorKind::UnsupportedFeature
    );
    assert!(prepare_global_program_with_host_calls(global, GlobalBankSchema { slots: 0 }).is_ok());
}
#[test]
fn host_preparation_keeps_init_defaults_and_catalog_paths_isolated() {
    let mut p = image(&[fun(0, 0, 0, 255, &[(0, 0, 6)])], 0, &[]);
    set_init(&mut p, &[(0, 0, 6), (0, 0, 0x48)]);
    let prepared = rml32::prepare_program_with_host_calls(p.clone(), None).unwrap();
    assert!(!prepared.is_initializable());
    let catalog = init_catalog(p);
    for result in [
        rml32::prepare_linked_program(&catalog, "root", Default::default()),
        prepare_init(&catalog, "root"),
    ] {
        assert_eq!(result.unwrap_err().kind, VmErrorKind::UnsupportedFeature);
    }
    let mut p = image(
        &[optional_fun(
            0,
            1,
            0,
            255,
            &[vec![(0, 0, 6)]],
            &[(0, 0, 0x37)],
        )],
        0,
        &[],
    );
    optional_meta(&mut p, 0, 0, false, &[1]);
    assert_eq!(
        rml32::prepare_program_with_host_calls(p, None)
            .unwrap_err()
            .kind,
        VmErrorKind::UnsupportedFeature
    );
}
#[test]
fn host_arguments_follow_stack_order_without_numeric_conversion() {
    let f = host_binding(
        vec![HostType::Int, HostType::Int],
        HostReturn::Exact(HostType::Int),
        |_, args| {
            let [Value::Int(a), Value::Int(b)] = args else {
                panic!()
            };
            Ok(Some(Value::Int(a - b)))
        },
    );
    let (mut vm, o) = host_machine(
        image(
            &[fun(0, 0, 0, 2, &[(3, 0, 0x3b), (10, 0, 0x3b), (0, 2, 6)])],
            0,
            &[],
        ),
        f,
    );
    vm.start(o, 0, vec![]).unwrap();
    returns(&mut vm, Value::Int(7));
    let calls = HostRc::new(HostCounter::new(0));
    let mut binding = counted_host(&calls, HostReturn::Mixed);
    binding.signature.required = vec![HostType::Real];
    let (mut vm, o) = host_machine(
        image(&[fun(0, 0, 0, 255, &[(1, 0, 0x3b), (0, 1, 6)])], 0, &[]),
        binding,
    );
    vm.start(o, 0, vec![]).unwrap();
    assert_eq!(vm.run(10).unwrap_err().kind, VmErrorKind::UnsupportedType);
    assert_eq!(calls.get(), 0);
}
#[test]
fn host_optional_and_rest_pass_actual_arguments_without_defaults() {
    for argc in 1..=4 {
        let mut binding = host_binding(
            vec![HostType::Int],
            HostReturn::Exact(HostType::Int),
            |_, args| Ok(Some(Value::Int(args.len() as i32))),
        );
        binding.signature.optional = vec![HostType::Int];
        binding.signature.rest = Some(HostType::Int);
        let mut nodes = vec![(1, 0, 0x3b); argc];
        nodes.push((0, argc as i8, 6));
        let (mut vm, o) = host_machine(image(&[fun(0, 0, 0, 2, &nodes)], 0, &[]), binding);
        vm.start(o, 0, vec![]).unwrap();
        returns(&mut vm, Value::Int(argc as i32));
    }
}
#[test]
fn host_argument_count_and_underflow_fail_before_callback() {
    for (required, pushed, argc, kind) in [
        (1, 0, 0, VmErrorKind::ArgumentCount),
        (0, 1, 1, VmErrorKind::ArgumentCount),
        (0, 0, 1, VmErrorKind::StackUnderflow),
    ] {
        let calls = HostRc::new(HostCounter::new(0));
        let mut binding = counted_host(&calls, HostReturn::Mixed);
        binding.signature.required = vec![HostType::Any; required];
        let mut nodes = vec![(1, 0, 0x3b); pushed];
        nodes.push((0, argc, 6));
        let (mut vm, o) = host_machine(image(&[fun(0, 0, 0, 255, &nodes)], 0, &[]), binding);
        vm.start(o, 0, vec![]).unwrap();
        assert_eq!(vm.run(100).unwrap_err().kind, kind);
        assert_eq!(calls.get(), 0);
    }
}
#[test]
fn host_argc_64_and_127_are_not_masked_and_charge_one_instruction() {
    for argc in [0, 64, 127] {
        let calls = HostRc::new(HostCounter::new(0));
        let counted = calls.clone();
        let mut binding = host_binding(vec![], HostReturn::Exact(HostType::Int), move |_, args| {
            counted.set(counted.get() + 1);
            Ok(Some(Value::Int(args.len() as i32)))
        });
        binding.signature.rest = Some(HostType::Int);
        let mut nodes = vec![(1, 0, 0x3b); argc];
        nodes.push((0, argc as i8, 6));
        let (mut vm, o) = host_machine(image(&[fun(0, 0, 0, 2, &nodes)], 0, &[]), binding);
        vm.start(o, 0, vec![]).unwrap();
        assert_eq!(vm.run(2 + argc).unwrap(), RunOutcome::BudgetExhausted);
        assert_eq!(calls.get(), 0);
        assert_eq!(vm.run(0).unwrap(), RunOutcome::BudgetExhausted);
        assert_eq!(calls.get(), 0);
        assert_eq!(vm.run(1).unwrap(), RunOutcome::BudgetExhausted);
        assert_eq!(calls.get(), 1);
        returns(&mut vm, Value::Int(argc as i32));
        assert_eq!(calls.get(), 1);
    }
}
#[test]
fn host_return_policies_validate_void_mixed_and_exact_values() {
    let values = [
        (HostType::Undefined, Value::Undefined),
        (HostType::Int, Value::Int(4)),
        (HostType::Real, real(1.5)),
        (HostType::Bytes, Value::bytes(b"a\0b")),
        (HostType::Array, Value::array(vec![Value::Int(1)]).unwrap()),
        (
            HostType::Mapping,
            Value::mapping(vec![(Value::Int(1), Value::Int(2))]).unwrap(),
        ),
    ];
    for (ty, value) in values {
        let expected = value.clone();
        let f = host_binding(vec![], HostReturn::Exact(ty), move |_, _| {
            Ok(Some(value.clone()))
        });
        let (mut vm, o) = host_machine(image(&[fun(0, 0, 0, 255, &[(0, 0, 6)])], 0, &[]), f);
        vm.start(o, 0, vec![]).unwrap();
        returns(&mut vm, expected);
    }
    for (policy, result, expected) in [
        (HostReturn::Void, None, Value::Void),
        (HostReturn::Mixed, None, Value::Void),
        (HostReturn::Mixed, Some(Value::Void), Value::Void),
        (HostReturn::Mixed, Some(Value::Int(8)), Value::Int(8)),
    ] {
        let f = host_binding(vec![], policy, move |_, _| Ok(result.clone()));
        let (mut vm, o) = host_machine(image(&[fun(0, 0, 0, 255, &[(0, 0, 6)])], 0, &[]), f);
        vm.start(o, 0, vec![]).unwrap();
        returns(&mut vm, expected);
    }
}
#[test]
fn host_wrong_results_preserve_operands_and_are_not_replayed() {
    for (policy, result) in [
        (HostReturn::Void, Some(Value::Void)),
        (HostReturn::Exact(HostType::Int), None),
        (HostReturn::Exact(HostType::Int), Some(real(1.0))),
    ] {
        let calls = HostRc::new(HostCounter::new(0));
        let counted = calls.clone();
        let f = host_binding(vec![HostType::Any], policy, move |_, _| {
            counted.set(counted.get() + 1);
            Ok(result.clone())
        });
        let (mut vm, o) = host_machine(
            image(&[fun(0, 0, 0, 255, &[(7, 0, 0x3b), (0, 1, 6)])], 0, &[]),
            f,
        );
        vm.start(o, 0, vec![]).unwrap();
        vm.run(3).unwrap();
        let before = vm.stack().unwrap();
        let e = vm.run(1).unwrap_err();
        assert_eq!(e.kind, VmErrorKind::UnsupportedType);
        assert_eq!(e.pc, Some(3));
        assert!(e.message.contains("host slot 0 (synthetic)"));
        assert_eq!(vm.stack().unwrap(), before);
        assert_eq!(vm.run(10).unwrap_err(), e);
        assert_eq!(calls.get(), 1);
    }
}
#[test]
fn host_callback_error_preserves_side_effect_and_bypasses_script_catch() {
    let calls = HostRc::new(HostCounter::new(0));
    let counted = calls.clone();
    let f = host_binding(vec![], HostReturn::Mixed, move |_, _| {
        counted.set(counted.get() + 1);
        Err(HostError {
            kind: VmErrorKind::Arithmetic,
            message: "synthetic arithmetic".into(),
        })
    });
    // catch target is body PC 6; the host error must never reach it.
    let (mut vm, o) = host_machine(
        image(
            &[fun(
                0,
                0,
                0,
                255,
                &[
                    (3, 0, 0x2c),
                    (0, 0, 6),
                    (0, 0, 0x1a),
                    (1, 0, 0x32),
                    (0, 0, 0x0e),
                ],
            )],
            0,
            &[],
        ),
        f,
    );
    vm.start(o, 0, vec![]).unwrap();
    let e = vm.run(100).unwrap_err();
    assert_eq!(
        (e.kind, e.pc, e.object, e.function),
        (VmErrorKind::Arithmetic, Some(3), Some(o), Some(0))
    );
    assert_eq!(vm.run(100).unwrap_err(), e);
    assert_eq!(calls.get(), 1);
}
#[test]
fn host_missing_registration_reports_actual_callsite_without_global_binding() {
    let f = host_binding(vec![], HostReturn::Mixed, |_, _| Ok(None));
    let p = image(&[fun(0, 0, 0, 255, &[(0, 0, 6)])], 0, &[]);
    let offset = p.functions[0].code.nodes_offset + 8;
    let (mut vm, o) = host_machine(p, f);
    vm.objects_mut().remove_host_function(0).unwrap();
    vm.start(o, 0, vec![]).unwrap();
    let e = vm.run(10).unwrap_err();
    assert_eq!(
        (e.kind, e.object, e.function, e.pc, e.offset),
        (
            VmErrorKind::UnsupportedFeature,
            Some(o),
            Some(0),
            Some(2),
            Some(offset)
        )
    );
    assert_eq!(e.program, Some(vm.objects().get(o).unwrap().program()));
}
#[test]
fn host_stack_and_frame_limits_prevent_callback_and_preserve_stack() {
    for frames in [false, true] {
        let calls = HostRc::new(HostCounter::new(0));
        let binding = counted_host(&calls, HostReturn::Mixed);
        let (vm, o) = host_machine(image(&[fun(0, 0, 0, 255, &[(0, 0, 6)])], 0, &[]), binding);
        let mut vm = Vm::with_limits(
            vm.into_objects(),
            VmLimits {
                max_stack: if frames { 8 } else { 0 },
                max_frames: if frames { 1 } else { 8 },
                ..Default::default()
            },
        );
        vm.start(o, 0, vec![]).unwrap();
        vm.run(2).unwrap();
        let before = vm.stack().unwrap();
        assert_eq!(vm.run(1).unwrap_err().kind, VmErrorKind::Limit);
        assert_eq!(calls.get(), 0);
        assert_eq!(vm.stack().unwrap(), before);
    }
}
#[test]
fn host_rejects_references_even_for_any_parameters() {
    let calls = HostRc::new(HostCounter::new(0));
    let mut f = counted_host(&calls, HostReturn::Mixed);
    f.signature.required.push(HostType::Any);
    let (mut vm, o) = host_machine(
        image(&[fun(0, 0, 1, 255, &[(-1, 0, 0x39), (0, 1, 6)])], 0, &[]),
        f,
    );
    vm.start(o, 0, vec![]).unwrap();
    vm.run(3).unwrap();
    let before = vm.stack().unwrap();
    assert_eq!(vm.run(1).unwrap_err().kind, VmErrorKind::UnsupportedFeature);
    assert_eq!(calls.get(), 0);
    assert_eq!(vm.stack().unwrap(), before);
}
#[test]
fn host_container_roundtrip_preserves_identity_and_original_object_slots() {
    let f = host_binding(vec![HostType::Any], HostReturn::Mixed, |_, a| {
        Ok(Some(a[0].clone()))
    });
    let (mut vm, o) = host_machine(
        image(&[fun(0, 1, 0, 255, &[(0, 0, 0x37), (0, 1, 6)])], 0, &[]),
        f,
    );
    for value in [
        Value::array(vec![Value::Int(8)]).unwrap(),
        Value::mapping(vec![(Value::Int(1), Value::Int(9))]).unwrap(),
    ] {
        vm.start(o, 0, vec![value.clone()]).unwrap();
        returns(&mut vm, value);
    }
}
#[test]
fn host_returned_object_and_nested_object_handles_are_opaque() {
    let f = host_binding(vec![], HostReturn::Exact(HostType::Object), |c, _| {
        Ok(Some(Value::Object(c.object.unwrap())))
    });
    let (mut vm, o) = host_machine(image(&[fun(0, 0, 0, 255, &[(0, 0, 6)])], 0, &[]), f);
    vm.start(o, 0, vec![]).unwrap();
    returns(&mut vm, Value::Object(o));
    let (mut vm, o) = host_machine(
        image(&[fun(0, 1, 0, 255, &[(0, 0, 0x37), (0, 1, 6)])], 0, &[]),
        host_binding(vec![HostType::Any], HostReturn::Mixed, |_, a| {
            Ok(Some(a[0].clone()))
        }),
    );
    let nested = Value::array(vec![Value::Object(o)]).unwrap();
    vm.start(o, 0, vec![nested.clone()]).unwrap();
    vm.run(3).unwrap();
    // Object is still alive here; deleted raw handles are exercised with private cells.
    returns(&mut vm, nested);
}
#[test]
fn host_pause_rebinding_and_into_objects_keep_registry_and_never_replay() {
    let calls = HostRc::new(HostCounter::new(0));
    let (mut vm, o) = host_machine(
        image(
            &[fun(0, 0, 0, 255, &[(0, 0, 6), (1, 0, 0x32), (0, 0, 6)])],
            0,
            &[],
        ),
        counted_host(&calls, HostReturn::Mixed),
    );
    vm.start(o, 0, vec![]).unwrap();
    vm.run(3).unwrap();
    assert_eq!(calls.get(), 1);
    vm.objects_mut().remove_host_function(0).unwrap();
    vm.objects_mut()
        .register_host_function(
            0,
            host_binding(vec![], HostReturn::Mixed, |_, _| Ok(Some(Value::Int(42)))),
        )
        .unwrap();
    returns(&mut vm, Value::Int(42));
    assert_eq!(calls.get(), 1);
    let mut vm = Vm::new(vm.into_objects());
    vm.start(o, 0, vec![]).unwrap();
    returns(&mut vm, Value::Int(42));
}
#[test]
fn host_global_calls_restore_none_context_and_object_caller_code_pool() {
    use rml32::{
        GlobalBankSchema, prepare_global_program_with_host_calls, prepare_program_with_host_calls,
    };
    let schema = GlobalBankSchema { slots: 0 };
    let seen = HostRc::new(std::cell::RefCell::new(vec![]));
    let captured = seen.clone();
    let binding = host_binding(vec![], HostReturn::Mixed, move |ctx, _| {
        captured.borrow_mut().push(ctx.clone());
        Ok(Some(Value::Int(7)))
    });
    let caller = image(
        &[fun(
            0,
            0,
            0,
            255,
            &[
                (0, 0, 6),
                (1, 0, 0x32),
                (559, -128, 7),
                (1, 0, 0x32),
                (0, 0, 0x40),
            ],
        )],
        0,
        &[b"object\0"],
    );
    let global = global_image(
        &[fun(
            559,
            0,
            0,
            255,
            &[(0, 0, 6), (1, 0, 0x32), (0, 0, 0x41)],
        )],
        0,
        &[b"global\0"],
    );
    let mut store = ObjectStore::new();
    let p = store
        .register_program(
            prepare_program_with_host_calls(caller, Some(schema))
                .unwrap()
                .with_source("object-pool"),
        )
        .unwrap();
    let o = store.create(p, vec![]).unwrap();
    let g = store
        .register_program(
            prepare_global_program_with_host_calls(global, schema)
                .unwrap()
                .with_source("global-pool"),
        )
        .unwrap();
    store.bind_global_program(g).unwrap();
    store.register_host_function(0, binding).unwrap();
    let mut vm = Vm::new(store);
    vm.start(o, 0, vec![]).unwrap();
    returns(&mut vm, Value::bytes(b"object"));
    let seen = seen.borrow();
    assert_eq!(seen.len(), 2);
    assert_eq!(
        (seen[0].program, seen[0].object, seen[0].source.as_str()),
        (p, Some(o), "object-pool")
    );
    assert_eq!(
        (
            seen[1].program,
            seen[1].object,
            seen[1].source.as_str(),
            seen[1].function
        ),
        (g, None, "global-pool", 559)
    );
}
#[test]
fn host_global_errors_have_no_object_and_point_to_global_06() {
    use rml32::{GlobalBankSchema, prepare_global_program_with_host_calls};
    let schema = GlobalBankSchema { slots: 0 };
    let (mut vm, o) = host_machine(
        image(&[fun(0, 0, 0, 255, &[(0, 0, 6)])], 0, &[]),
        host_binding(vec![], HostReturn::Mixed, |_, _| {
            Err(HostError {
                kind: VmErrorKind::InvalidState,
                message: "global failure".into(),
            })
        }),
    );
    // Replace the Object program with a global-opted-in caller.
    let mut store = vm.into_objects();
    let p = store
        .register_program(
            rml32::prepare_program_with_host_calls(
                image(&[fun(0, 0, 0, 255, &[(559, -128, 7)])], 0, &[]),
                Some(schema),
            )
            .unwrap(),
        )
        .unwrap();
    store.remove(o).unwrap();
    let o = store.create(p, vec![]).unwrap();
    let g = store
        .register_program(
            prepare_global_program_with_host_calls(
                global_image(&[fun(559, 0, 0, 255, &[(0, 0, 6)])], 0, &[]),
                schema,
            )
            .unwrap()
            .with_source("global-failure"),
        )
        .unwrap();
    store.bind_global_program(g).unwrap();
    vm = Vm::new(store);
    vm.start(o, 0, vec![]).unwrap();
    let e = vm.run(100).unwrap_err();
    assert_eq!(
        (e.program, e.object, e.function, e.pc),
        (Some(g), None, Some(559), Some(2))
    );
    assert_eq!(e.source, "global-failure");
}

#[test]
fn host_exact_argument_types_and_optional_rest_failures_are_checked_before_calling() {
    let (_, object) = machine(image(&[fun(0, 0, 0, 255, &[])], 0, &[]), vec![]);
    for (ty, value) in [
        (HostType::Undefined, Value::Undefined),
        (HostType::Void, Value::Void),
        (HostType::Int, Value::Int(1)),
        (HostType::Real, real(1.5)),
        (HostType::Bytes, Value::bytes(b"bytes")),
        (HostType::Object, Value::Object(object)),
        (HostType::Array, Value::array(vec![]).unwrap()),
        (HostType::Mapping, Value::mapping(vec![]).unwrap()),
    ] {
        let expected = value.clone();
        let f = host_binding(vec![ty], HostReturn::Mixed, |_, a| Ok(Some(a[0].clone())));
        let (mut vm, o) = host_machine(
            image(&[fun(0, 1, 0, 255, &[(0, 0, 0x37), (0, 1, 6)])], 0, &[]),
            f,
        );
        vm.start(o, 0, vec![value]).unwrap();
        returns(&mut vm, expected);
    }
    for rest in [false, true] {
        let calls = HostRc::new(HostCounter::new(0));
        let mut f = counted_host(&calls, HostReturn::Mixed);
        if rest {
            f.signature.rest = Some(HostType::Real)
        } else {
            f.signature.optional = vec![HostType::Real]
        }
        let (mut vm, o) = host_machine(
            image(&[fun(0, 0, 0, 255, &[(1, 0, 0x3b), (0, 1, 6)])], 0, &[]),
            f,
        );
        vm.start(o, 0, vec![]).unwrap();
        assert_eq!(vm.run(100).unwrap_err().kind, VmErrorKind::UnsupportedType);
        assert_eq!(calls.get(), 0);
    }
}

#[test]
fn host_binding_at_last_slot_works_without_a_global_program_or_bank() {
    let f = host_binding(vec![], HostReturn::Exact(HostType::Int), |ctx, _| {
        assert_eq!(ctx.slot, 558);
        Ok(Some(Value::Int(18)))
    });
    let (mut vm, o) = host_machine(image(&[fun(0, 0, 0, 2, &[(558, 0, 6)])], 0, &[]), f.clone());
    vm.objects_mut().remove_host_function(0).unwrap();
    vm.objects_mut().register_host_function(558, f).unwrap();
    vm.start(o, 0, vec![]).unwrap();
    returns(&mut vm, Value::Int(18));
}
