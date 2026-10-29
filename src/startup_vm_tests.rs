//! Deliberately bypass preparation to exercise the runtime permission boundary.
use super::*;
use crate::{ByteString, ProgramImage, ProgramMode, program::CodeSegment};

fn prepared() -> PreparedProgram {
    let nodes = vec![
        (3, 0, 0x3b),
        (0, 1, 6),
        (2, 0, 0x36),
        (0, 0, 3),
        (0, 0, 0x33),
        (2, 0, 0x3b),
        (3, 0, 0x36),
        (0, 0, 3),
        (0, 0, 0x33),
        (0, 0, 0x3b),
        (4, 0, 0x36),
        (0, 0, 3),
        (0, 0, 0x33),
        (0, 0, 0x40),
        (5, 0, 0x36),
        (0, 0, 3),
        (0, 0, 0x33),
        (0, 0, 0x18),
        (6, 0, 0x36),
        (0, 0, 3),
        (0, 0, 0x33),
        (0, 0, 0x3b),
        (7, 0, 0x36),
        (0, 0, 3),
        (0, 0, 0x33),
    ];
    let image = ProgramImage {
        mode: ProgramMode::Global,
        opaque_header: 0,
        function_base: 572,
        defines: vec![],
        strings: vec![Some(ByteString::new(b"/xdbc/\0"))],
        names_hex: String::new(),
        max_vars: 8,
        vars_count: 0,
        variables: vec![],
        efuns_count: 559,
        max_functions: 572,
        hidden: vec![],
        constitution_count: None,
        inherits: vec![],
        dlts: vec![],
        functions: vec![],
        init: CodeSegment {
            offset: 80,
            nodes_offset: 100,
            nodes,
            metadata_offset: 200,
            metadata_hex: String::new(),
            symbols: vec![],
            runtime_count: 0,
            runtime_hex: String::new(),
        },
        bytes_consumed: 0,
        sha256: "synthetic-runtime-guard".into(),
    };
    crate::prepared::prepare_startup_prefix(&image, "synthetic-prefix").unwrap()
}
fn vm(prepared: PreparedProgram) -> (Vm, Rc<std::cell::Cell<usize>>) {
    let mut store = ObjectStore::new();
    store.register_globals(vec![Value::Undefined; 8]).unwrap();
    let calls = Rc::new(std::cell::Cell::new(0));
    let count = calls.clone();
    store
        .register_host_function(
            0,
            crate::host::HostFunction {
                name: "test_allocate_array".into(),
                signature: crate::HostSignature {
                    required: vec![crate::HostType::Int],
                    optional: vec![],
                    rest: None,
                    returns: crate::HostReturn::Exact(crate::HostType::Array),
                },
                callback: Rc::new(move |context, args| {
                    assert_eq!(context.function, -1);
                    assert_eq!(context.object, None);
                    count.set(count.get() + 1);
                    let Value::Int(size) = args[0] else { panic!() };
                    Value::array(vec![Value::Undefined; size as usize])
                        .and_then(crate::ownership::OwnedValue::new)
                        .map(Some)
                        .map_err(|e| crate::HostError {
                            kind: e.kind,
                            message: e.message.into(),
                        })
                }),
            },
        )
        .unwrap();
    let id = store.register_program(prepared).unwrap();
    let mut vm = Vm::new(store);
    vm.begin_startup_prefix(id).unwrap();
    (vm, calls)
}
fn code(p: &mut PreparedProgram) -> &mut Vec<Instruction> {
    Rc::make_mut(&mut p.initializations.as_mut().unwrap()[0].instructions)
}

#[test]
fn runtime_denies_forbidden_calls_and_control_transfer_after_preparation() {
    for (opcode, operands) in [
        (6, [336, 1]),
        (6, [0, 2]),
        (7, [572, -128]),
        (0x48, [0, 0]),
        (0x1f, [0, 0]),
    ] {
        let mut p = prepared();
        code(&mut p)[0] = Instruction {
            opcode,
            operands,
            offset: 100,
        };
        let (mut vm, calls) = vm(p);
        let e = vm.run_startup_prefix(25).unwrap_err();
        assert_eq!(
            (e.kind, e.pc, e.offset, e.function, e.initialization),
            (
                VmErrorKind::UnsupportedFeature,
                Some(0),
                Some(100),
                None,
                Some(0)
            )
        );
        assert_eq!(calls.get(), 0);
        assert_eq!(vm.frames[0].pc, 0);
    }
}
#[test]
fn runtime_fence_blocks_even_an_appended_prepared_instruction() {
    let mut p = prepared();
    code(&mut p).push(Instruction {
        opcode: 6,
        operands: [336, 1],
        offset: 200,
    });
    let (mut vm, calls) = vm(p);
    assert!(vm.run_startup_prefix(usize::MAX).unwrap());
    assert_eq!(calls.get(), 1);
    assert_eq!(vm.frames[0].pc, 25);
    assert!(vm.completion.is_none());
    assert!(vm.initialization.is_none());
    assert!(vm.stack.is_empty());
    let e = vm.run(100).unwrap_err();
    assert_eq!((e.kind, e.pc), (VmErrorKind::Bounds, Some(25)));
    assert_eq!(calls.get(), 1);
}
#[test]
fn runtime_global_bounds_remain_checked_after_preparation() {
    let mut p = prepared();
    code(&mut p)[2].operands[0] = 8;
    let (mut vm, calls) = vm(p);
    let e = vm.run_startup_prefix(25).unwrap_err();
    assert_eq!((e.kind, e.pc), (VmErrorKind::Bounds, Some(2)));
    assert_eq!(calls.get(), 1);
    assert!(
        vm.objects
            .global_values()
            .unwrap()
            .iter()
            .all(|v| *v == Value::Undefined)
    );
}
#[test]
fn cancellation_releases_actual_array_storage_in_paused_finished_and_failed_states() {
    for (budget, fail) in [(2, false), (3, false), (25, false), (3, true)] {
        let mut p = prepared();
        if fail {
            code(&mut p)[2].operands[0] = 8;
        }
        let (mut vm, _) = vm(p);
        let result = vm.run_startup_prefix(budget);
        assert_eq!(result.is_err(), fail);
        let value = if budget == 25 {
            vm.objects.global_values().unwrap()[2].clone()
        } else {
            vm.value(0).unwrap()
        };
        let Value::Array(array) = value else { panic!() };
        let weak = Rc::downgrade(&array.0);
        drop(array);
        assert!(weak.upgrade().is_some());
        drop(vm);
        assert!(weak.upgrade().is_none());
    }
}
