//! Runtime preparation for the explicitly bounded Magic559 core.
use crate::{
    ProgramImage, ProgramMode,
    program::{Argument, CodeSegment, Function},
    value::Value,
    vm_error::{VmError, VmErrorKind},
};
use std::{
    collections::{BTreeMap, BTreeSet},
    rc::Rc,
};

pub const SUPPORTED_OPCODES: &[u8] = &[
    0x1, 0x2, 0x3, 0x4, 0x5, 0x6, 0x7, 0x8, 0xa, 0xb, 0xc, 0xd, 0xe, 0xf, 0x10, 0x12, 0x13, 0x14,
    0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24,
    0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x32, 0x33, 0x34, 0x35,
    0x36, 0x37, 0x39, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x47, 0x48,
    0x49, 0x4a, 0x4b, 0x4d, 0x4e, 0x4f, 0x50, 0x51, 0x55,
];
#[derive(Debug, Clone)]
pub struct Instruction {
    pub(crate) opcode: u8,
    pub(crate) operands: [i32; 2],
    pub(crate) offset: usize,
}
impl Instruction {
    pub fn opcode(&self) -> u8 {
        self.opcode
    }
    pub fn operands(&self) -> [i32; 2] {
        self.operands
    }
    pub fn offset(&self) -> usize {
        self.offset
    }
}
#[derive(Debug, Clone)]
pub(crate) struct EntryLayout {
    pub enter: usize,
    pub resort: Option<usize>,
}
#[derive(Debug, Clone)]
pub struct PreparedFunction {
    pub(crate) index: i16,
    pub(crate) name: Option<Vec<u8>>,
    pub(crate) private: bool,
    pub(crate) instructions: Rc<Vec<Instruction>>,
    // Immutable dispatch context: the caller frame retains its own slot on return.
    pub(crate) origin: Option<String>,
    pub(crate) pool: Option<Rc<Vec<Option<Value>>>>,
    pub(crate) function_base: i16,
    pub(crate) variable_base: i16,
    pub(crate) local_count: usize,
    pub(crate) args: Vec<Argument>,
    pub(crate) min_args: usize,
    pub(crate) max_args: usize,
    pub(crate) variadic: bool,
    pub(crate) entry: Option<EntryLayout>,
    pub(crate) return_type: u8,
    pub(crate) return_attrib: u8,
}
/// An explicit entry in a complete, synthetic method namespace, not native lookup reconstruction.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MethodBinding {
    pub name: Vec<u8>,
    pub slot: i16,
}
impl PreparedFunction {
    pub fn index(&self) -> i16 {
        self.index
    }
    pub fn name(&self) -> Option<&[u8]> {
        self.name.as_deref()
    }
    pub fn instructions(&self) -> &[Instruction] {
        &self.instructions
    }
}
/// A real initialization code segment, independent of function dispatch slots.
#[derive(Debug)]
pub(crate) struct PreparedInitialization {
    pub instructions: Rc<Vec<Instruction>>,
    pub source: String,
    pub pool: Rc<Vec<Option<Value>>>,
    pub function_base: i16,
    pub variable_base: i16,
}
/// Explicit capacity of a synthetic, store-wide global variable bank.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GlobalBankSchema {
    pub slots: usize,
}

pub(crate) const MAX_GLOBAL_SLOTS: usize = 32768;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ProgramKind {
    Object,
    Global,
}

#[derive(Debug)]
pub struct PreparedProgram {
    pub(crate) source: String,
    pub(crate) global_schema: Option<GlobalBankSchema>,
    pub(crate) allow_host_calls: bool,
    pub(crate) allow_function_values: bool,
    pub(crate) allow_global_function_values: bool,
    pub(crate) allow_host_function_values: bool,
    pub(crate) allow_container_captures: bool,
    pub(crate) kind: ProgramKind,
    pub(crate) function_capacity: usize,
    pub(crate) variable_count: usize,
    pub(crate) strings: Vec<Option<Value>>,
    pub(crate) functions: BTreeMap<i16, PreparedFunction>,
    pub(crate) unimplemented: BTreeSet<i16>,
    pub(crate) initializations: Option<Vec<PreparedInitialization>>,
    pub(crate) init_order: Vec<usize>,
    pub(crate) lifecycles: Option<Vec<LifecycleBinding>>,
    pub(crate) methods: Option<BTreeMap<Vec<u8>, i16>>,
}
impl PreparedProgram {
    pub fn source(&self) -> &str {
        &self.source
    }
    /// True only when all source-local lifecycle bindings were explicitly prepared.
    pub fn is_constructible(&self) -> bool {
        self.lifecycles.is_some()
    }
    /// True only when all inherited initialization bodies were explicitly prepared.
    pub fn is_initializable(&self) -> bool {
        self.initializations.is_some()
    }
    pub fn variable_count(&self) -> usize {
        self.variable_count
    }
    pub fn function(&self, index: i16) -> Option<&PreparedFunction> {
        self.functions.get(&index)
    }
    pub fn with_source(mut self, source: impl Into<String>) -> Self {
        self.source = source.into();
        self
    }
    /// Supply the complete method namespace. An empty list deliberately means no methods.
    /// Names are case-sensitive raw bytes without NUL; private comes from the function body.
    pub fn with_methods(mut self, bindings: Vec<MethodBinding>) -> Result<Self, VmError> {
        let mut methods = BTreeMap::new();
        for binding in bindings {
            if binding.name.is_empty() || binding.name.contains(&0) {
                return Err(err("method name must be nonempty and contain no NUL"));
            }
            let f = self.functions.get(&binding.slot).ok_or_else(|| {
                err("method binding must select an implemented final function slot")
            })?;
            if f.instructions.first().is_none_or(|i| i.opcode != 0x0f) {
                return Err(unsupported(
                    "method functions must start with check_args (0f)",
                ));
            }
            if matches!(binding.name.as_slice(), b"create" | b"destruct")
                || f.name.as_ref().is_some_and(|n| {
                    matches!(n.split(|b| *b == 0).next(), Some(b"create" | b"destruct"))
                })
                || self.lifecycles.as_ref().is_some_and(|bindings| {
                    bindings
                        .iter()
                        .any(|b| b.create == Some(binding.slot) || b.destruct == Some(binding.slot))
                })
            {
                return Err(unsupported("lifecycle method binding is unsupported"));
            }
            if methods.insert(binding.name, binding.slot).is_some() {
                return Err(err("duplicate method name"));
            }
        }
        self.methods = Some(methods);
        Ok(self)
    }
}
fn err(message: impl Into<String>) -> VmError {
    VmError::new(VmErrorKind::InvalidProgram, message)
}
fn unsupported(message: impl Into<String>) -> VmError {
    VmError::new(VmErrorKind::UnsupportedFeature, message)
}
fn bytes(hex: &str) -> Result<Vec<u8>, VmError> {
    if !hex.len().is_multiple_of(2) || !hex.is_ascii() {
        return Err(err("invalid hex record"));
    }
    (0..hex.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&hex[i..i + 2], 16).map_err(|_| err("invalid hex record")))
        .collect()
}
#[derive(Clone, Copy, Default)]
struct PreparationOptions {
    globals: Option<GlobalBankSchema>,
    global_code: bool,
    host_calls: bool,
    function_values: bool,
    global_function_values: bool,
    host_function_values: bool,
    container_captures: bool,
}
fn instructions(
    segment: &CodeSegment,
    strings: &[Option<crate::ByteString>],
    vars: usize,
    initialization: bool,
    options: PreparationOptions,
) -> Result<Vec<Instruction>, VmError> {
    let PreparationOptions {
        globals,
        global_code,
        host_calls,
        function_values,
        global_function_values,
        host_function_values,
        ..
    } = options;
    let metadata = bytes(&segment.metadata_hex)?;
    let mut result = Vec::new();
    for (pc, &(a, b, op)) in segment.nodes.iter().enumerate() {
        let offset = segment
            .nodes_offset
            .checked_add(
                pc.checked_mul(4)
                    .ok_or_else(|| err("node offset overflow"))?,
            )
            .ok_or_else(|| err("node offset overflow"))?;
        let mut operands = [i32::from(a), i32::from(b)];
        if matches!(op, 0x0a | 0x0b | 0x3c | 0x3f) {
            let pos = (i32::from(b) << 16) | i32::from(a);
            let raw = usize::try_from(pos)
                .ok()
                .and_then(|pos| {
                    pos.checked_add(if op == 0x3c { 4 } else { 8 })
                        .and_then(|end| metadata.get(pos..end))
                })
                .ok_or_else(|| {
                    let mut e = err("metadata operand is truncated or outside segment");
                    e.pc = Some(pc);
                    e.offset = Some(offset);
                    e
                })?;
            operands[0] = i32::from_le_bytes(raw[..4].try_into().unwrap());
            if op != 0x3c {
                operands[1] = i32::from_le_bytes(raw[4..8].try_into().unwrap());
            }
        }
        let check = if matches!(op, 0x05 | 0x17) {
            if !function_values || initialization {
                Err(unsupported(
                    "function values require explicit supported ordinary-function preparation",
                ))
            } else if op == 0x17
                && ((b as u8 & 0x40 != 0 && (b as u8 & 0x80 == 0 || !host_function_values))
                    || (b as u8 & 0xc0 == 0x80 && !global_function_values)
                    || (global_code && b as u8 & 0x80 == 0))
            {
                Err(unsupported("function value target kind is not enabled"))
            } else if op == 0x17 && (a < 0 || (b as u8 & 0xc0 == 0xc0 && a >= 559)) {
                Err(err("function value slot outside supported range"))
            } else if op == 0x05 && !matches!(a, 0 | 4) {
                Err(unsupported("indirect call requires flags zero or four"))
            } else if op == 0x05 && b <= 0 {
                Err(err("indirect argument count must include its target"))
            } else {
                Ok(())
            }
        } else if op == 0x06 {
            if !host_calls || initialization {
                Err(unsupported(
                    "host calls require explicit ordinary-function preparation",
                ))
            } else if !(0..559).contains(&a) || b < 0 {
                Err(err(
                    "host slot or signed argument count outside supported range",
                ))
            } else {
                Ok(())
            }
        } else if matches!(op, 0x35 | 0x36) {
            match globals.filter(|_| !initialization) {
                None => Err(unsupported(format!(
                    "global slot {a} requires explicit ordinary-function preparation"
                ))),
                Some(schema) if a < 0 || a as usize >= schema.slots => {
                    Err(err(format!("global slot {a} outside schema capacity")))
                }
                Some(_) => Ok(()),
            }
        } else if op == 0x10 && initialization {
            Err(unsupported(
                "reference type check is unavailable in initialization",
            ))
        } else if op == 0x45 && (initialization || !(0..=3).contains(&a)) {
            Err(unsupported(
                "range query requires an ordinary function and mode zero through three",
            ))
        } else if initialization && matches!(op, 0x0e | 0x1a | 0x2c) {
            Err(unsupported(
                "catch instructions are unavailable in initialization",
            ))
        } else if initialization && matches!(op, 0x12 | 0x23 | 0x50) {
            Err(unsupported(
                "loop instructions are unavailable in initialization",
            ))
        } else if initialization && matches!(op, 0x0a..=0x0c) {
            Err(unsupported(
                "case instructions are unavailable in initialization",
            ))
        } else if op == 0x08 && initialization {
            Err(unsupported(
                "cross-object calls during initialization are unsupported",
            ))
        } else if op == 0x08 && (!matches!(a, 0 | 2 | 4 | 6) || b < 0) {
            Err(unsupported(
                "cross-object call requires flags 0/2/4/6 and nonnegative argc",
            ))
        } else if op == 0x48 && !initialization {
            Err(unsupported("initialization return in ordinary function"))
        } else if initialization
            && matches!(
                op,
                0x0f | 0x1b | 0x47 | 0x49 | 0x34 | 0x15 | 0x37 | 0x39 | 0x4d
            )
        {
            Err(unsupported(
                "function arguments, locals or return are unavailable in initialization",
            ))
        } else if op == 0x0d && (b as u8 & 2 != 0 || ![2, 3].contains(&a)) {
            Err(unsupported("only non-const numeric casts are supported"))
        } else if op == 0x2f && !(2..=11).contains(&a) {
            Err(unsupported("compound operator selector must be in 2..=11"))
        } else if matches!(op, 0x43 | 0x44) && ![0, 1].contains(&a) {
            Err(unsupported(
                "only member query modes zero and one are supported",
            ))
        } else if !SUPPORTED_OPCODES.contains(&op) {
            Err(unsupported(format!(
                "opcode 0x{op:02x} is not supported by the bounded VM"
            )))
        } else if op == 0x40 && (a < 0 || a as usize >= strings.len()) {
            Err(err("shared string index outside pool"))
        } else if op == 0x0b
            && usize::try_from(operands[1])
                .ok()
                .and_then(|i| strings.get(i))
                .is_none_or(Option::is_none)
        {
            Err(err("case string index is outside pool or empty"))
        } else if matches!(op, 0x3d | 0x3e | 0x4e) && (a < 0 || a as usize >= vars) {
            Err(err("object variable index outside capacity"))
        } else if op == 0x07
            && ((b as u8 & 0x80 != 0) && (globals.is_none() || initialization)
                || (b as u8 & 0x80 == 0) && global_code)
        {
            Err(unsupported(
                "function call requires explicit global preparation and global-code calls require bit7",
            ))
        } else {
            Ok(())
        };
        if let Err(mut e) = check {
            e.pc = Some(pc);
            e.offset = Some(offset);
            return Err(e);
        }
        if matches!(op, 0x0a..=0x0c | 0x12 | 0x50 | 0x1f | 0x21 | 0x0f | 0x2c) {
            let target = (pc as i64) + 1 + i64::from(operands[0]);
            if target < 0 || target >= segment.nodes.len() as i64 {
                let mut e = err("relative branch target outside code");
                e.pc = Some(pc);
                e.offset = Some(offset);
                return Err(e);
            }
        }
        result.push(Instruction {
            opcode: op,
            operands,
            offset,
        });
    }
    if result.is_empty() {
        return Err(err("empty function code"));
    }
    Ok(result)
}
// The first implementation accepts only linear, ordinary-value default code.
fn prepare_entry(
    code: &[Instruction],
    args: &[crate::program::Argument],
) -> Result<EntryLayout, VmError> {
    if code.first().is_none_or(|i| i.opcode != 0x0f) {
        return Err(unsupported(
            "optional/variadic functions must start with check_args",
        ));
    }
    let mut cursor = 1usize;
    for arg in args {
        let end = cursor
            .checked_add(arg.default_action_count as usize)
            .filter(|end| *end <= code.len())
            .ok_or_else(|| err("default segment outside code"))?;
        let mut depth = 0usize;
        for (relative, instruction) in code[cursor..end].iter().enumerate() {
            match instruction.opcode {
                0x3b | 0x3c | 0x3f | 0x40 | 0x42 | 0x3d => depth += 1,
                0x16 | 0x18 => {
                    let count = usize::try_from(instruction.operands[0])
                        .map_err(|_| err("negative default container count"))?;
                    if depth < count || (instruction.opcode == 0x18 && count % 2 != 0) {
                        return Err(err("default container consumes outside expression"));
                    }
                    depth = depth - count + 1;
                }
                _ => {
                    let mut e =
                        unsupported("default expression opcode is outside the linear subset");
                    e.pc = Some(cursor + relative);
                    e.offset = Some(instruction.offset);
                    return Err(e);
                }
            }
        }
        if end > cursor && depth != 1 {
            return Err(err("default expression must produce one value"));
        }
        cursor = end;
    }
    let resort = (cursor > 1).then_some(cursor);
    if resort.is_some() {
        if code.get(cursor).is_none_or(|i| i.opcode != 0x47) {
            return Err(err("missing default resort opcode"));
        }
        cursor += 1;
    }
    if code[0].operands[0] != (cursor - 1) as i32
        || code.get(cursor).is_none_or(|i| i.opcode != 0x1b)
    {
        return Err(err("default entry length or enter opcode mismatch"));
    }
    for (pc, instruction) in code.iter().enumerate().skip(cursor + 1) {
        if matches!(instruction.opcode, 0x0f | 0x1b | 0x47) {
            return Err(err("function body re-enters initialization"));
        }
        if matches!(instruction.opcode, 0x1f | 0x21 | 0x2c)
            && pc as i64 + 1 + i64::from(instruction.operands[0]) <= cursor as i64
        {
            return Err(err("body branch targets initialization"));
        }
    }
    Ok(EntryLayout {
        enter: cursor,
        resort,
    })
}

fn prepare_function(
    f: Function,
    strings: &[Option<crate::ByteString>],
    vars: usize,
    options: PreparationOptions,
) -> Result<PreparedFunction, VmError> {
    let index = f
        .index
        .ok_or_else(|| unsupported("hidden functions are not supported"))?;
    if index < 0 || f.local_count < 0 {
        return Err(err("negative function index or local count"));
    }
    if f.attrib & 2 != 0 {
        return Err(unsupported("atomic functions are not supported"));
    }
    if f.min_args > f.max_args || f.max_args > 127 {
        return Err(err("invalid signed native argument range"));
    }
    let raw = bytes(&f.raw_hex)?;
    if raw.len() != 64 {
        return Err(err("function raw record is not 64 bytes"));
    }
    if raw[12] & 2 != 0 {
        return Err(unsupported("const return semantics are not supported"));
    }
    if ![0, 1, 2, 3, 4, 6, 7, 10, 255].contains(&f.return_type)
        && !(options.function_values && f.return_type == 9)
    {
        return Err(VmError::new(
            VmErrorKind::UnsupportedType,
            "return type is not supported",
        ));
    }
    if f.arguments.len() != usize::from(f.max_args) {
        return Err(err("argument descriptors do not match maximum prototype"));
    }
    for (i, a) in f.arguments.iter().enumerate() {
        if a.flag & !1 != 0 || a.attrib & !8 != 0 {
            return Err(unsupported(
                "unknown argument flags or const attributes are not supported",
            ));
        }
        if ![0, 1, 2, 3, 4, 6, 7, 10, 255].contains(&a.r#type)
            && !(options.function_values && a.r#type == 9)
        {
            return Err(VmError::new(
                VmErrorKind::UnsupportedType,
                "argument type is not supported",
            ));
        }
        if a.default_action_count < 0 {
            return Err(err("negative default instruction count"));
        }
        if a.index != i as i16 {
            return Err(err("argument descriptor index does not match position"));
        }
    }
    let instructions = instructions(&f.code, strings, vars, false, options)?;
    for (pc, i) in instructions.iter().enumerate() {
        if matches!(i.opcode, 0x12 | 0x23 | 0x50) {
            let slot = i.operands[usize::from(i.opcode != 0x23)];
            if slot >= 0 || slot < -i32::from(f.local_count) {
                let mut e = err("loop control must select a declared local slot");
                e.pc = Some(pc);
                e.offset = Some(i.offset);
                return Err(e);
            }
        }
    }
    if f.arguments.iter().any(|arg| arg.flag == 1)
        && instructions.first().is_none_or(|i| i.opcode != 0x0f)
    {
        return Err(unsupported(
            "reference functions must start with check_args (0x0f)",
        ));
    }
    let extended = f.min_args != f.max_args
        || f.attrib & 1 != 0
        || f.arguments.iter().any(|a| a.default_action_count != 0);
    let entry = if extended {
        Some(prepare_entry(&instructions, &f.arguments)?)
    } else {
        if instructions.iter().any(|i| i.opcode == 0x47) {
            return Err(unsupported("resort requires a validated default entry"));
        }
        None
    };
    Ok(PreparedFunction {
        index,
        name: f.name.map(|s| s.bytes),
        private: f.attrib & 4 != 0,
        instructions: Rc::new(instructions),
        origin: None,
        pool: None,
        function_base: 0,
        variable_base: 0,
        local_count: f.local_count as usize,
        args: f.arguments,
        min_args: usize::from(f.min_args),
        max_args: usize::from(f.max_args),
        variadic: f.attrib & 1 != 0,
        entry,
        return_type: f.return_type,
        return_attrib: raw[12],
    })
}
pub fn prepare_program(image: ProgramImage) -> Result<PreparedProgram, VmError> {
    prepare_code_program(image, PreparationOptions::default())
}

/// Prepare ordinary single-constitution Object functions with explicit global access.
/// Initialization code is ignored, just as in `prepare_program`.
pub fn prepare_program_with_globals(
    image: ProgramImage,
    schema: GlobalBankSchema,
) -> Result<PreparedProgram, VmError> {
    prepare_code_program(
        image,
        PreparationOptions {
            globals: Some(schema),
            ..PreparationOptions::default()
        },
    )
}

/// Prepare Global functions only; initialization and pre-module bootstrap are excluded.
pub fn prepare_global_program(
    image: ProgramImage,
    schema: GlobalBankSchema,
) -> Result<PreparedProgram, VmError> {
    prepare_code_program(
        image,
        PreparationOptions {
            globals: Some(schema),
            global_code: true,
            ..PreparationOptions::default()
        },
    )
}

/// Prepare ordinary Object functions with synchronous synthetic host calls.
pub fn prepare_program_with_host_calls(
    image: ProgramImage,
    globals: Option<GlobalBankSchema>,
) -> Result<PreparedProgram, VmError> {
    prepare_code_program(
        image,
        PreparationOptions {
            globals,
            host_calls: true,
            ..PreparationOptions::default()
        },
    )
}

/// Prepare ordinary Global functions with synchronous synthetic host calls; init is ignored.
pub fn prepare_global_program_with_host_calls(
    image: ProgramImage,
    globals: GlobalBankSchema,
) -> Result<PreparedProgram, VmError> {
    prepare_code_program(
        image,
        PreparationOptions {
            globals: Some(globals),
            global_code: true,
            host_calls: true,
            ..PreparationOptions::default()
        },
    )
}

/// Prepare ordinary Object functions with scalar-capturing Object function values.
pub fn prepare_program_with_function_values(
    image: ProgramImage,
    globals: Option<GlobalBankSchema>,
) -> Result<PreparedProgram, VmError> {
    prepare_code_program(
        image,
        PreparationOptions {
            globals,
            function_values: true,
            ..PreparationOptions::default()
        },
    )
}
/// Prepare ordinary Global functions with indirect calls; creation and init are excluded.
pub fn prepare_global_program_with_function_values(
    image: ProgramImage,
    globals: GlobalBankSchema,
) -> Result<PreparedProgram, VmError> {
    prepare_code_program(
        image,
        PreparationOptions {
            globals: Some(globals),
            global_code: true,
            function_values: true,
            ..PreparationOptions::default()
        },
    )
}
/// Prepare ordinary Object functions with scalar-capturing Object and Global targets.
pub fn prepare_program_with_global_function_values(
    image: ProgramImage,
    globals: GlobalBankSchema,
) -> Result<PreparedProgram, VmError> {
    prepare_code_program(
        image,
        PreparationOptions {
            globals: Some(globals),
            function_values: true,
            global_function_values: true,
            ..PreparationOptions::default()
        },
    )
}

/// Prepare ordinary Global functions with Global target creation and indirect calls.
pub fn prepare_global_program_with_global_function_values(
    image: ProgramImage,
    globals: GlobalBankSchema,
) -> Result<PreparedProgram, VmError> {
    prepare_code_program(
        image,
        PreparationOptions {
            globals: Some(globals),
            global_code: true,
            function_values: true,
            global_function_values: true,
            ..PreparationOptions::default()
        },
    )
}

/// Prepare ordinary Object code with bytecode function values and synchronous Host/K calls.
pub fn prepare_program_with_callable_values(
    image: ProgramImage,
    globals: GlobalBankSchema,
) -> Result<PreparedProgram, VmError> {
    prepare_code_program(
        image,
        PreparationOptions {
            globals: Some(globals),
            host_calls: true,
            function_values: true,
            global_function_values: true,
            host_function_values: true,
            ..PreparationOptions::default()
        },
    )
}

/// Prepare ordinary Global code with bytecode function values and synchronous Host/K calls.
pub fn prepare_global_program_with_callable_values(
    image: ProgramImage,
    globals: GlobalBankSchema,
) -> Result<PreparedProgram, VmError> {
    prepare_code_program(
        image,
        PreparationOptions {
            globals: Some(globals),
            global_code: true,
            host_calls: true,
            function_values: true,
            global_function_values: true,
            host_function_values: true,
            ..PreparationOptions::default()
        },
    )
}

/// Prepare ordinary Object code with callable values and acyclic Array/Mapping captures.
/// Existing preparation entrypoints remain scalar-only.
pub fn prepare_program_with_container_captures(
    image: ProgramImage,
    globals: GlobalBankSchema,
) -> Result<PreparedProgram, VmError> {
    prepare_container_capture_program(image, globals, false)
}

/// Prepare ordinary Global code with callable values and acyclic Array/Mapping captures.
pub fn prepare_global_program_with_container_captures(
    image: ProgramImage,
    globals: GlobalBankSchema,
) -> Result<PreparedProgram, VmError> {
    prepare_container_capture_program(image, globals, true)
}

fn prepare_container_capture_program(
    image: ProgramImage,
    globals: GlobalBankSchema,
    global_code: bool,
) -> Result<PreparedProgram, VmError> {
    prepare_code_program(
        image,
        PreparationOptions {
            globals: Some(globals),
            global_code,
            host_calls: true,
            function_values: true,
            global_function_values: true,
            host_function_values: true,
            container_captures: true,
        },
    )
}

fn prepare_code_program(
    image: ProgramImage,
    options: PreparationOptions,
) -> Result<PreparedProgram, VmError> {
    let PreparationOptions {
        globals,
        global_code,
        host_calls,
        function_values,
        global_function_values,
        host_function_values,
        container_captures,
    } = options;
    let source = format!("sha256:{}", image.sha256);
    let build = || {
        if globals.is_some_and(|schema| schema.slots > MAX_GLOBAL_SLOTS) {
            return Err(VmError::new(
                VmErrorKind::Limit,
                "global schema exceeds 32768 slots",
            ));
        }
        if !image.inherits.is_empty()
            || (!global_code
                && (image.mode != ProgramMode::Object || image.constitution_count != Some(1)))
            || (global_code && image.mode != ProgramMode::Global)
        {
            return Err(unsupported(
                "preparation requires the selected code kind without inheritance",
            ));
        }
        if global_code
            && (image.function_base < 559
                || image.function_base > i32::from(image.max_functions)
                || globals.is_none_or(|s| s.slots != image.max_vars as usize))
        {
            return Err(err("invalid global function interval or variable schema"));
        }
        if image.efuns_count != 559 || image.max_vars < 0 || image.max_functions < 0 {
            return Err(err("invalid Magic559 capacities"));
        }
        if !image.hidden.is_empty() {
            return Err(unsupported("hidden functions are not supported"));
        }
        let mut functions = BTreeMap::new();
        for function in image.functions {
            let index = function.index;
            let prepared =
                prepare_function(function, &image.strings, image.max_vars as usize, options)
                    .map_err(|mut e| {
                        e.function = index;
                        e
                    })?;
            if prepared.index >= image.max_functions
                || (global_code && i32::from(prepared.index) < image.function_base)
                || functions.contains_key(&prepared.index)
            {
                return Err(err("function index is duplicated or outside capacity"));
            }
            functions.insert(prepared.index, prepared);
        }
        // Resolve direct slots after all bodies have been prepared (forward targets included).
        for f in functions.values() {
            for (pc, i) in f
                .instructions
                .iter()
                .enumerate()
                // Absolute Global targets resolve against the runtime binding,
                // not the creating image's function directory/capacity.
                .filter(|(_, i)| i.opcode == 0x17 && i.operands[1] as u8 & 0x80 == 0)
            {
                let slot = i.operands[0];
                let check = if slot < 0 || slot >= i32::from(image.max_functions) {
                    Err(err("function value slot outside capacity"))
                } else {
                    match functions.get(&(slot as i16)) {
                        Some(target) if !target.args.iter().any(|a| a.flag != 0) => Ok(()),
                        _ => Err(unsupported(
                            "function value target requires a prepared body without reference parameters",
                        )),
                    }
                };
                check.map_err(|mut e| {
                    e.function = Some(f.index);
                    e.pc = Some(pc);
                    e.offset = Some(i.offset);
                    e
                })?;
            }
        }
        Ok(PreparedProgram {
            source: source.clone(),
            global_schema: globals,
            allow_host_calls: host_calls,
            allow_function_values: function_values,
            allow_global_function_values: global_function_values,
            allow_host_function_values: host_function_values,
            allow_container_captures: container_captures,
            kind: if global_code {
                ProgramKind::Global
            } else {
                ProgramKind::Object
            },
            function_capacity: image.max_functions as usize,
            variable_count: image.max_vars as usize,
            strings: image
                .strings
                .into_iter()
                .map(|s| {
                    s.map(|mut s| {
                        // vm_loadBinary forces the final stored byte to NUL;
                        // vm_findOrAddProgramString then uses C-string length.
                        if let Some(last) = s.bytes.last_mut() {
                            *last = 0;
                        }
                        Value::bytes(s.c_bytes())
                    })
                })
                .collect(),
            functions,
            unimplemented: BTreeSet::new(),
            initializations: None,
            init_order: vec![],
            lifecycles: None,
            methods: None,
        })
    };
    build().map_err(|mut e| {
        e.source = source;
        e
    })
}

/// Explicit source-local lifecycle function indices; absence is intentional.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct LifecycleBinding {
    pub create: Option<i16>,
    pub destruct: Option<i16>,
}
/// Explicit dependency namespace. Names are matched as bytes without path inference.
#[derive(Debug, Default)]
pub struct ProgramCatalog {
    programs: BTreeMap<String, ProgramImage>,
    bindings: BTreeMap<(String, Vec<u8>), BTreeSet<String>>,
    lifecycles: BTreeMap<String, LifecycleBinding>,
}
impl ProgramCatalog {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn insert(&mut self, key: impl Into<String>, image: ProgramImage) -> Result<(), VmError> {
        let key = key.into();
        if self.programs.contains_key(&key) {
            return Err(err("duplicate catalog key"));
        }
        self.programs.insert(key, image);
        Ok(())
    }
    pub fn bind_lifecycle(
        &mut self,
        source: impl Into<String>,
        binding: LifecycleBinding,
    ) -> Result<(), VmError> {
        let source = source.into();
        if self.lifecycles.contains_key(&source) {
            return Err(err("duplicate lifecycle binding"));
        }
        self.lifecycles.insert(source, binding);
        Ok(())
    }
    pub fn bind_inherit(
        &mut self,
        child: impl Into<String>,
        name: impl AsRef<[u8]>,
        parent: impl Into<String>,
    ) {
        self.bindings
            .entry((child.into(), name.as_ref().to_vec()))
            .or_default()
            .insert(parent.into());
    }
}
#[derive(Debug, Clone, Copy)]
pub struct LinkLimits {
    pub max_dependencies: usize,
    pub max_depth: usize,
    pub max_function_slots: usize,
}
impl Default for LinkLimits {
    fn default() -> Self {
        Self {
            max_dependencies: 1024,
            max_depth: 64,
            max_function_slots: 1_000_000,
        }
    }
}
#[derive(Clone)]
struct Constitution {
    key: String,
    function_base: i16,
    variable_base: i16,
}
#[derive(Clone)]
struct LinkSlot {
    body: Rc<Function>,
    constitution: usize,
    alias: bool,
}
struct Linked {
    constitutions: Vec<Constitution>,
    slots: Vec<Option<LinkSlot>>,
    init_order: Vec<usize>,
}
fn add_base(a: i16, b: i16) -> Result<i16, VmError> {
    a.checked_add(b)
        .filter(|v| *v >= 0)
        .ok_or_else(|| err("signed16 relocation overflow"))
}
fn prototype(a: &Function, b: &Function) -> Result<(), VmError> {
    if a.return_type != b.return_type
        || a.arguments.len() != b.arguments.len()
        || a.arguments
            .iter()
            .zip(&b.arguments)
            .any(|(a, b)| a.r#type != b.r#type || a.attrib != b.attrib)
    {
        return Err(err("declaration prototype conflicts with implementation"));
    }
    if bytes(&a.raw_hex)?.get(12) != bytes(&b.raw_hex)?.get(12)
        || a.arguments
            .iter()
            .zip(&b.arguments)
            .any(|(a, b)| a.flag != b.flag)
    {
        return Err(unsupported(
            "declaration reference/return attributes differ",
        ));
    }
    Ok(())
}
struct Linker<'a> {
    catalog: &'a ProgramCatalog,
    limits: LinkLimits,
    seen: BTreeSet<String>,
    slots: usize,
}
impl Linker<'_> {
    fn link(&mut self, key: &str, depth: usize) -> Result<Linked, VmError> {
        if depth > self.limits.max_depth {
            return Err(VmError::new(VmErrorKind::Limit, "inheritance depth limit"));
        }
        if !self.seen.insert(key.to_owned()) {
            return Err(unsupported("cycle or repeated ancestor"));
        }
        if self.seen.len().saturating_sub(1) > self.limits.max_dependencies {
            return Err(VmError::new(VmErrorKind::Limit, "dependency limit"));
        }
        let image = self
            .catalog
            .programs
            .get(key)
            .ok_or_else(|| err(format!("missing catalog program {key}")))?;
        if image.mode != ProgramMode::Object || image.efuns_count != 559 || !image.hidden.is_empty()
        {
            return Err(unsupported(
                "linking requires Magic559 objects without hidden functions",
            ));
        }
        let count = image
            .constitution_count
            .filter(|v| *v > 0)
            .ok_or_else(|| err("invalid constitution capacity"))? as usize;
        if image.max_functions < 0 || image.max_vars < 0 {
            return Err(err("negative capacity"));
        }
        self.slots = self
            .slots
            .checked_add(image.max_functions as usize)
            .ok_or_else(|| err("slot sum overflow"))?;
        if self.slots > self.limits.max_function_slots {
            return Err(VmError::new(VmErrorKind::Limit, "function slot limit"));
        }
        let mut cons = vec![None; count];
        cons[0] = Some(Constitution {
            key: key.into(),
            function_base: 0,
            variable_base: 0,
        });
        let mut slots = vec![None; image.max_functions as usize];
        for f in &image.functions {
            let i = f
                .index
                .filter(|i| *i >= 0 && (*i as usize) < slots.len())
                .ok_or_else(|| err("invalid local function slot"))? as usize;
            if slots[i].is_some() {
                return Err(err("duplicate local function slot"));
            }
            slots[i] = Some(LinkSlot {
                body: Rc::new(f.clone()),
                constitution: 0,
                alias: false,
            });
        }
        let mut init_order = vec![None; count];
        init_order[count - 1] = Some(0);
        let mut occupied_functions = BTreeSet::new();
        let mut occupied_variables = BTreeSet::new();
        for (ordinal, inherit) in image.inherits.iter().enumerate() {
            (|| -> Result<(), VmError> {
                let build = |this: &mut Self| -> Result<Linked, VmError> {
                    let name = inherit
                        .name
                        .as_ref()
                        .ok_or_else(|| err("missing inheritance name"))?;
                    let candidates = this
                        .catalog
                        .bindings
                        .get(&(key.into(), name.bytes.clone()))
                        .ok_or_else(|| err("missing explicit inheritance binding"))?;
                    if candidates.len() != 1 {
                        return Err(err("ambiguous inheritance binding"));
                    }
                    let parent_key = candidates.first().unwrap();
                    let parent = this
                        .catalog
                        .programs
                        .get(parent_key)
                        .ok_or_else(|| err("missing bound parent"))?;
                    if [
                        parent.constitution_count.unwrap_or(0),
                        parent.max_functions,
                        parent.max_vars,
                    ] != inherit.offsets[3..]
                    {
                        return Err(err("parent capacity mismatch"));
                    }
                    this.link(parent_key, depth + 1)
                };
                let parent = build(self)?;
                let [cb, fb, vb, nc, nf, nv] = inherit.offsets;
                if cb < 1 {
                    return Err(err("inherited constitution overwrites self"));
                }
                for (base, len, capacity) in [
                    (cb, nc, count),
                    (fb, nf, slots.len()),
                    (vb, nv, image.max_vars as usize),
                ] {
                    if base < 0 || len < 0 || i32::from(base) + i32::from(len) > capacity as i32 {
                        return Err(err("inheritance interval outside capacity"));
                    }
                }
                for i in fb..add_base(fb, nf)? {
                    if !occupied_functions.insert(i) || slots[i as usize].is_some() {
                        return Err(err("overlapping function interval"));
                    }
                }
                for i in vb..add_base(vb, nv)? {
                    if !occupied_variables.insert(i) {
                        return Err(err("overlapping variable interval"));
                    }
                }
                for (j, value) in parent.init_order.into_iter().enumerate() {
                    init_order[cb as usize + j - 1] = Some(cb as usize + value);
                }
                for (j, c) in parent.constitutions.into_iter().enumerate() {
                    let target = cb as usize + j;
                    if cons[target].is_some() {
                        return Err(err("overlapping constitution interval"));
                    }
                    cons[target] = Some(Constitution {
                        key: c.key,
                        function_base: add_base(fb, c.function_base)?,
                        variable_base: add_base(vb, c.variable_base)?,
                    });
                }
                for (j, slot) in parent.slots.into_iter().enumerate() {
                    slots[fb as usize + j] = slot.map(|mut s| {
                        s.constitution += cb as usize;
                        s
                    });
                }
                Ok(())
            })()
            .map_err(|mut e| {
                e.message = format!(
                    "{key} inherit[{ordinal}] name={:?}: {}",
                    inherit.name.as_ref().map(|n| &n.bytes_hex),
                    e.message
                )
                .into_boxed_str();
                e
            })?;
        }
        let constitutions = cons
            .into_iter()
            .map(|c| c.ok_or_else(|| err("constitution hole")))
            .collect::<Result<Vec<_>, _>>()?;
        for i in 0..slots.len() {
            let Some(declaration) = slots[i].clone().filter(|s| s.body.code.nodes.is_empty())
            else {
                continue;
            };
            for j in (0..i).rev().chain(i + 1..slots.len()) {
                let Some(candidate) = slots[j].as_ref() else {
                    continue;
                };
                if candidate.alias
                    || candidate.body.code.nodes.is_empty()
                    || candidate.body.attrib & 4 != 0
                {
                    continue;
                }
                let name = declaration.body.name.as_ref().map(|s| &s.bytes);
                if name.is_none() || name != candidate.body.name.as_ref().map(|s| &s.bytes) {
                    continue;
                }
                if name.is_some_and(|n| {
                    matches!(n.split(|b| *b == 0).next(), Some(b"create" | b"destruct"))
                }) {
                    return Err(unsupported("lifecycle declaration alias"));
                }
                prototype(&declaration.body, &candidate.body)?;
                let mut bound = candidate.clone();
                bound.alias = true;
                slots[i] = Some(bound);
                break;
            }
        }
        Ok(Linked {
            constitutions,
            slots,
            init_order: init_order
                .into_iter()
                .collect::<Option<Vec<_>>>()
                .ok_or_else(|| err("initialization order hole"))?,
        })
    }
}
/// Links without registering programs, creating objects, or executing initialization.
pub fn prepare_linked_program(
    catalog: &ProgramCatalog,
    root: &str,
    limits: LinkLimits,
) -> Result<PreparedProgram, VmError> {
    prepare_catalog(catalog, root, limits, false, false)
}
/// Opt in to bounded initialization preparation; never executes any code.
pub fn prepare_initializable_program(
    catalog: &ProgramCatalog,
    root: &str,
    limits: LinkLimits,
) -> Result<PreparedProgram, VmError> {
    prepare_catalog(catalog, root, limits, true, false)
}
/// Prepare initialization plus explicitly bound, bounded construction and rollback.
pub fn prepare_constructible_program(
    catalog: &ProgramCatalog,
    root: &str,
    limits: LinkLimits,
) -> Result<PreparedProgram, VmError> {
    prepare_catalog(catalog, root, limits, true, true)
}
fn prepare_catalog(
    catalog: &ProgramCatalog,
    root: &str,
    limits: LinkLimits,
    initialize: bool,
    construct: bool,
) -> Result<PreparedProgram, VmError> {
    let build = || {
        let linked = Linker {
            catalog,
            limits,
            seen: BTreeSet::new(),
            slots: 0,
        }
        .link(root, 0)?;
        let lifecycles = if construct {
            Some(
                linked
                    .constitutions
                    .iter()
                    .enumerate()
                    .map(|(cons, c)| {
                        validate_lifecycle_binding(catalog, &linked, cons, c).map_err(|mut e| {
                            e.source = c.key.clone();
                            e.message =
                                format!("constitution {cons}: {}", e.message).into_boxed_str();
                            e
                        })
                    })
                    .collect::<Result<Vec<_>, VmError>>()?,
            )
        } else {
            None
        };
        let mut pools = BTreeMap::new();
        for c in &linked.constitutions {
            pools.entry(c.key.clone()).or_insert_with(|| {
                Rc::new(
                    catalog.programs[&c.key]
                        .strings
                        .iter()
                        .map(|s| {
                            s.as_ref().map(|s| {
                                let mut bytes = s.bytes.clone();
                                if let Some(last) = bytes.last_mut() {
                                    *last = 0;
                                }
                                Value::bytes(bytes.split(|b| *b == 0).next().unwrap_or_default())
                            })
                        })
                        .collect::<Vec<_>>(),
                )
            });
        }
        let initializations = if initialize {
            Some(
                linked
                    .constitutions
                    .iter()
                    .enumerate()
                    .map(|(index, c)| {
                        let image = &catalog.programs[&c.key];
                        let prepare = || {
                            let code = instructions(
                                &image.init,
                                &image.strings,
                                image.max_vars as usize,
                                true,
                                PreparationOptions::default(),
                            )?;
                            if !code.iter().any(|i| i.opcode == 0x48) {
                                return Err(err("initialization has no return (48)"));
                            }
                            Ok(PreparedInitialization {
                                instructions: Rc::new(code),
                                source: c.key.clone(),
                                pool: pools[&c.key].clone(),
                                function_base: c.function_base,
                                variable_base: c.variable_base,
                            })
                        };
                        prepare().map_err(|mut e: VmError| {
                            e.source = c.key.clone();
                            e.initialization = Some(index);
                            e
                        })
                    })
                    .collect::<Result<Vec<_>, VmError>>()?,
            )
        } else {
            None
        };
        let mut functions = BTreeMap::new();
        let mut unimplemented = BTreeSet::new();
        let mut bodies = BTreeMap::<(String, i16), PreparedFunction>::new();
        for (i, slot) in linked.slots.into_iter().enumerate() {
            let Some(slot) = slot else {
                continue;
            };
            if slot.body.code.nodes.is_empty() {
                unimplemented.insert(i as i16);
                continue;
            }
            let c = &linked.constitutions[slot.constitution];
            let image = &catalog.programs[&c.key];
            if initialize
                && let Some(pc) = slot
                    .body
                    .code
                    .nodes
                    .iter()
                    .position(|n| matches!(n.2, 0x08 | 0x12 | 0x23 | 0x50 | 0x0e | 0x1a | 0x2c))
            {
                let mut e = unsupported(
                    "cross-object calls, loops and catches in lifecycle-prepared programs are unsupported",
                );
                e.source = c.key.clone();
                e.function = Some(i as i16);
                e.pc = Some(pc);
                e.offset = Some(slot.body.code.nodes_offset + pc * 4);
                return Err(e);
            }
            let id = (c.key.clone(), slot.body.index.unwrap());
            if !bodies.contains_key(&id) {
                let mut body = (*slot.body).clone();
                if construct {
                    let binding = catalog.lifecycles[&c.key];
                    if body.index == binding.create {
                        body.attrib |= 1 | 4;
                    }
                    if body.index == binding.destruct {
                        body.attrib |= 4;
                    }
                }
                let mut f = prepare_function(
                    body,
                    &image.strings,
                    image.max_vars as usize,
                    PreparationOptions::default(),
                )
                .map_err(|mut e| {
                    e.source = c.key.clone();
                    e.function = Some(i as i16);
                    e
                })?;
                f.origin = Some(c.key.clone());
                f.pool = Some(pools[&c.key].clone());
                bodies.insert(id.clone(), f);
            }
            let mut f = bodies[&id].clone();
            f.index = i as i16;
            f.function_base = c.function_base;
            f.variable_base = c.variable_base;
            functions.insert(i as i16, f);
        }
        Ok(PreparedProgram {
            source: root.into(),
            global_schema: None,
            allow_host_calls: false,
            allow_function_values: false,
            allow_global_function_values: false,
            allow_host_function_values: false,
            allow_container_captures: false,
            kind: ProgramKind::Object,
            function_capacity: 0,
            variable_count: catalog.programs[root].max_vars as usize,
            strings: vec![],
            functions,
            unimplemented,
            initializations,
            lifecycles,
            methods: None,
            init_order: if initialize {
                linked.init_order
            } else {
                vec![]
            },
        })
    };
    build().map_err(|mut e: VmError| {
        if e.source.is_empty() {
            e.source = root.into();
        }
        e
    })
}

fn validate_lifecycle_binding(
    catalog: &ProgramCatalog,
    linked: &Linked,
    cons: usize,
    constitution: &Constitution,
) -> Result<LifecycleBinding, VmError> {
    let image = &catalog.programs[&constitution.key];
    let binding = *catalog
        .lifecycles
        .get(&constitution.key)
        .ok_or_else(|| err("missing explicit lifecycle binding"))?;
    for (name, index) in [
        (b"create".as_slice(), binding.create),
        (b"destruct".as_slice(), binding.destruct),
    ] {
        let named: Vec<_> = image
            .functions
            .iter()
            .filter(|f| f.name.as_ref().is_some_and(|n| n.c_bytes() == name))
            .collect();
        if named.iter().any(|f| f.code.nodes.is_empty()) {
            return Err(unsupported("lifecycle declarations are unsupported"));
        }
        let Some(index) = index else {
            if !named.is_empty() {
                return Err(err("None lifecycle binding hides a local lifecycle record"));
            }
            continue;
        };
        let function = named
            .iter()
            .find(|f| f.index == Some(index))
            .ok_or_else(|| err("lifecycle binding must select a named local implementation"))?;
        if function.return_type != 1
            || function.arguments.iter().any(|a| a.flag != 0)
            || (name == b"create"
                && (function.max_args > 1
                    || function
                        .arguments
                        .iter()
                        .any(|a| !matches!(a.r#type, 7 | 255))))
            || (name == b"destruct"
                && (function.max_args != 0
                    || function.min_args != 0
                    || !function.arguments.is_empty()))
        {
            return Err(unsupported(
                "lifecycle prototype outside bounded void/value subset",
            ));
        }
        let target = add_base(constitution.function_base, index)? as usize;
        let slot = linked
            .slots
            .get(target)
            .and_then(Option::as_ref)
            .ok_or_else(|| err("missing lifecycle implementation slot"))?;
        if slot.alias || slot.constitution != cons || slot.body.index != Some(index) {
            return Err(unsupported(
                "lifecycle binding cannot select an inherited alias",
            ));
        }
    }
    Ok(LifecycleBinding {
        create: binding
            .create
            .map(|i| add_base(constitution.function_base, i))
            .transpose()?,
        destruct: binding
            .destruct
            .map(|i| add_base(constitution.function_base, i))
            .transpose()?,
    })
}
