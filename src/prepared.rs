//! Runtime preparation for the explicitly bounded Magic559 core.
use crate::{
    ProgramImage, ProgramMode,
    program::{Argument, CodeSegment, Function},
    value::Value,
    vm_error::{VmError, VmErrorKind},
};
use std::collections::BTreeMap;

pub const SUPPORTED_OPCODES: &[u8] = &[
    0x1, 0x2, 0x3, 0x4, 0x7, 0xd, 0xf, 0x13, 0x14, 0x16, 0x18, 0x19, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f,
    0x20, 0x21, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2d, 0x30, 0x32, 0x33, 0x37, 0x39,
    0x3b, 0x3c, 0x3d, 0x3e, 0x3f, 0x40, 0x42, 0x43, 0x44, 0x49, 0x4a, 0x4b, 0x4d, 0x4e, 0x4f, 0x51,
    0x55,
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
pub struct PreparedFunction {
    pub(crate) index: i16,
    pub(crate) name: Option<Vec<u8>>,
    pub(crate) instructions: Vec<Instruction>,
    pub(crate) local_count: usize,
    pub(crate) args: Vec<Argument>,
    pub(crate) argument_count: usize,
    pub(crate) return_type: u8,
    pub(crate) return_attrib: u8,
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
#[derive(Debug)]
pub struct PreparedProgram {
    pub(crate) source: String,
    pub(crate) variable_count: usize,
    pub(crate) strings: Vec<Option<Value>>,
    pub(crate) functions: BTreeMap<i16, PreparedFunction>,
}
impl PreparedProgram {
    pub fn source(&self) -> &str {
        &self.source
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
fn instructions(
    segment: &CodeSegment,
    strings: usize,
    vars: usize,
) -> Result<Vec<Instruction>, VmError> {
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
        if matches!(op, 0x3c | 0x3f) {
            let pos = (i32::from(b) << 16) | i32::from(a);
            let raw = usize::try_from(pos)
                .ok()
                .and_then(|pos| {
                    pos.checked_add(if op == 0x3f { 8 } else { 4 })
                        .and_then(|end| metadata.get(pos..end))
                })
                .ok_or_else(|| {
                    let mut e = err("numeric metadata operand is truncated or outside segment");
                    e.pc = Some(pc);
                    e.offset = Some(offset);
                    e
                })?;
            operands[0] = i32::from_le_bytes(raw[..4].try_into().unwrap());
            if op == 0x3f {
                operands[1] = i32::from_le_bytes(raw[4..8].try_into().unwrap());
            }
        }
        let check = if op == 0x0d && (b as u8 & 2 != 0 || ![2, 3].contains(&a)) {
            Err(unsupported("only non-const numeric casts are supported"))
        } else if matches!(op, 0x43 | 0x44) && ![0, 1].contains(&a) {
            Err(unsupported(
                "only member query modes zero and one are supported",
            ))
        } else if !SUPPORTED_OPCODES.contains(&op) {
            Err(unsupported(format!(
                "opcode 0x{op:02x} is not supported by the bounded VM"
            )))
        } else if op == 0x40 && (a < 0 || a as usize >= strings) {
            Err(err("shared string index outside pool"))
        } else if matches!(op, 0x3d | 0x3e | 0x4e) && (a < 0 || a as usize >= vars) {
            Err(err("object variable index outside capacity"))
        } else if op == 0x07 && (b as u8 & 0x80 != 0) {
            Err(unsupported(
                "global/bootstrap function calls are not supported",
            ))
        } else {
            Ok(())
        };
        if let Err(mut e) = check {
            e.pc = Some(pc);
            e.offset = Some(offset);
            return Err(e);
        }
        if matches!(op, 0x1f | 0x21 | 0x0f) {
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
fn prepare_function(f: Function, strings: usize, vars: usize) -> Result<PreparedFunction, VmError> {
    let index = f
        .index
        .ok_or_else(|| unsupported("hidden functions are not supported"))?;
    if index < 0 || f.local_count < 0 {
        return Err(err("negative function index or local count"));
    }
    if f.attrib & 3 != 0 || f.min_args != f.max_args {
        return Err(unsupported(
            "variadic, atomic and default-argument functions are not supported",
        ));
    }
    let raw = bytes(&f.raw_hex)?;
    if raw.len() != 64 {
        return Err(err("function raw record is not 64 bytes"));
    }
    if raw[12] & 2 != 0 {
        return Err(unsupported("const return semantics are not supported"));
    }
    if ![0, 1, 2, 3, 4, 6, 7, 10, 255].contains(&f.return_type) {
        return Err(VmError::new(
            VmErrorKind::UnsupportedType,
            "return type is not supported",
        ));
    }
    if f.arguments.len() != usize::from(f.max_args) {
        return Err(err("fixed argument descriptors do not match prototype"));
    }
    for (i, a) in f.arguments.iter().enumerate() {
        if a.flag & 1 != 0 || a.attrib & 2 != 0 {
            return Err(unsupported("reference/const arguments are not supported"));
        }
        if ![0, 1, 2, 3, 4, 6, 7, 10, 255].contains(&a.r#type) {
            return Err(VmError::new(
                VmErrorKind::UnsupportedType,
                "argument type is not supported",
            ));
        }
        if a.index != i as i16 {
            return Err(err("argument descriptor index does not match position"));
        }
    }
    let instructions = instructions(&f.code, strings, vars)?;
    Ok(PreparedFunction {
        index,
        name: f.name.map(|s| s.bytes),
        instructions,
        local_count: f.local_count as usize,
        args: f.arguments,
        argument_count: usize::from(f.max_args),
        return_type: f.return_type,
        return_attrib: raw[12],
    })
}
pub fn prepare_program(image: ProgramImage) -> Result<PreparedProgram, VmError> {
    let source = format!("sha256:{}", image.sha256);
    let build = || {
        if image.mode != ProgramMode::Object
            || !image.inherits.is_empty()
            || image.constitution_count != Some(1)
        {
            return Err(unsupported(
                "stage 2A executes only single-constitution objects without inheritance",
            ));
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
            let prepared = prepare_function(function, image.strings.len(), image.max_vars as usize)
                .map_err(|mut e| {
                    e.function = index;
                    e
                })?;
            if prepared.index >= image.max_functions || functions.contains_key(&prepared.index) {
                return Err(err("function index is duplicated or outside capacity"));
            }
            functions.insert(prepared.index, prepared);
        }
        Ok(PreparedProgram {
            source: source.clone(),
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
        })
    };
    build().map_err(|mut e| {
        e.source = source;
        e
    })
}
