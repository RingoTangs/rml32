//! Magic559 loader structure, based on vm_loadBinary @0x080b9264.
//! Raw records are retained; serialized pointers are never dereferenced.
use crate::{ByteString, ErrorKind, ParseError, hex, reader::Reader, sha256};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AbiProfile {
    Magic559,
    LegacyAuth389,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ProgramMode {
    Auto,
    Object,
    Global,
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct Define {
    pub nargs: i32,
    pub name: Option<ByteString>,
    pub expansion: Option<ByteString>,
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct Variable {
    pub raw_hex: String,
    pub name: Option<ByteString>,
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct Argument {
    pub raw_hex: String,
    pub name: Option<ByteString>,
    pub flag: u8,
    pub r#type: u8,
    pub attrib: u8,
    pub index: i16,
    /// Serialized instruction count at +0x0a; retained signed, validated by preparation.
    pub default_action_count: i16,
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct Inherit {
    pub name: Option<ByteString>,
    pub offsets: [i16; 6],
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct Dlt {
    pub offset: usize,
    pub name: ByteString,
    pub attrib: u32,
    pub index: i16,
    pub raw_hex: String,
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct CodeSegment {
    pub offset: usize,
    pub nodes_offset: usize,
    pub nodes: Vec<(i16, i8, u8)>,
    pub metadata_offset: usize,
    pub metadata_hex: String,
    pub symbols: Vec<(Option<ByteString>, i32)>,
    pub runtime_count: usize,
    pub runtime_hex: String,
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct Function {
    pub offset: usize,
    pub raw_hex: String,
    pub name: Option<ByteString>,
    pub token: u32,
    pub attrib: i16,
    pub local_count: i16,
    pub return_type: u8,
    pub max_args: u8,
    pub min_args: u8,
    pub arguments: Vec<Argument>,
    pub locals: Vec<Argument>,
    pub code: CodeSegment,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dlt_name: Option<ByteString>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<i16>,
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct ProgramImage {
    pub mode: ProgramMode,
    pub opaque_header: i32,
    pub function_base: i32,
    pub defines: Vec<Define>,
    pub strings: Vec<Option<ByteString>>,
    pub names_hex: String,
    pub max_vars: i16,
    pub vars_count: i16,
    pub variables: Vec<Variable>,
    pub efuns_count: i16,
    pub max_functions: i16,
    pub hidden: Vec<Function>,
    pub constitution_count: Option<i16>,
    pub inherits: Vec<Inherit>,
    pub dlts: Vec<Dlt>,
    pub functions: Vec<Function>,
    pub init: CodeSegment,
    pub bytes_consumed: usize,
    pub sha256: String,
}

fn short(raw: &[u8], i: usize) -> i16 {
    i16::from_le_bytes(raw[i..i + 2].try_into().unwrap())
}
fn word(raw: &[u8], i: usize) -> u32 {
    u32::from_le_bytes(raw[i..i + 4].try_into().unwrap())
}
fn code(r: &mut Reader<'_>) -> Result<CodeSegment, ParseError> {
    let offset = r.pos;
    let mut symbols = Vec::new();
    let (mut filename, mut line) = (None::<ByteString>, 0);
    for _ in 0..r.count()? {
        let flag = r.u8()?;
        if flag & 1 != 0 {
            filename = r.string()?;
        }
        if flag & 2 != 0 {
            line = r.i32()?;
        }
        if filename.as_ref().is_none_or(|s| s.c_bytes().is_empty()) {
            return Err(r.error("missing symbol filename"));
        }
        symbols.push((filename.clone(), line));
    }
    let count = r.count()?;
    let nodes_offset = r.pos;
    let raw = r.take(count * 4)?;
    let nodes: Vec<_> = raw
        .as_chunks::<4>()
        .0
        .iter()
        .map(|b| (short(b, 0), b[2] as i8, b[3]))
        .collect();
    if nodes.iter().any(|n| n.2 == 0 || n.2 >= 86) {
        return Err(ParseError::new(
            nodes_offset,
            ErrorKind::UnsupportedFormat,
            "opcode outside Magic559 action table",
        ));
    }
    let meta_len = r.count()?;
    let metadata_offset = r.pos;
    let metadata = r.take(meta_len)?;
    for (pc, &(a, b, op)) in nodes.iter().enumerate() {
        let (params, attrib) = ACTIONS[op as usize];
        if attrib & 0x80000000 != 0 {
            let offset = (i32::from(b) << 16) | i32::from(a);
            if offset < 0
                || (offset as usize)
                    .checked_add(params as usize * 4)
                    .is_none_or(|end| end > metadata.len())
            {
                return Err(ParseError::new(
                    nodes_offset + pc * 4,
                    ErrorKind::Bounds,
                    "metadata operand outside segment",
                ));
            }
        }
    }
    let runtime_count = r.count()?;
    if runtime_count != 0 && runtime_count != count {
        return Err(r.error("runtime count does not match nodes"));
    }
    let runtime = r.take(runtime_count * 4)?;
    Ok(CodeSegment {
        offset,
        nodes_offset,
        nodes,
        metadata_offset,
        metadata_hex: hex(metadata),
        symbols,
        runtime_count,
        runtime_hex: hex(runtime),
    })
}
fn arguments(r: &mut Reader<'_>) -> Result<Vec<Argument>, ParseError> {
    let mut out = Vec::new();
    for _ in 0..r.short_count()? {
        let raw = r.take(20)?;
        out.push(Argument {
            raw_hex: hex(raw),
            name: r.string()?,
            flag: raw[4],
            r#type: raw[5],
            attrib: raw[6],
            index: short(raw, 8),
            default_action_count: short(raw, 10),
        });
    }
    Ok(out)
}
fn function(r: &mut Reader<'_>) -> Result<Function, ParseError> {
    let offset = r.pos;
    let raw = r.take(64)?;
    let name = r.string()?;
    let args = arguments(r)?;
    let locals = arguments(r)?;
    Ok(Function {
        offset,
        raw_hex: hex(raw),
        name,
        token: word(raw, 4),
        attrib: short(raw, 8),
        local_count: short(raw, 10),
        return_type: raw[13],
        max_args: raw[14],
        min_args: raw[15],
        arguments: args,
        locals,
        code: code(r)?,
        dlt_name: None,
        index: None,
    })
}
fn parse_mode(bytes: &[u8], mode: ProgramMode) -> Result<ProgramImage, ParseError> {
    let mut r = Reader::new(bytes);
    if r.i32()? != 0x10001 {
        return Err(ParseError::new(
            0,
            ErrorKind::InvalidFormat,
            "not VM bytecode",
        ));
    }
    let opaque_header = r.i32()?;
    let function_base = r.i32()?;
    let mut defines = Vec::new();
    for _ in 0..r.count()? {
        defines.push(Define {
            nargs: r.i32()?,
            name: r.string()?,
            expansion: r.string()?,
        });
    }
    let mut strings = Vec::new();
    for _ in 0..r.count()? {
        strings.push(r.string()?);
    }
    let names_count = r.count()?;
    let names_size = r.count()?;
    let names = r.take(names_size)?;
    let max_vars = r.i16()?;
    let vars_count = r.i16()?;
    let mut variables = Vec::new();
    if mode == ProgramMode::Object {
        if vars_count < 0 {
            return Err(r.error("negative variable count"));
        }
        for _ in 0..vars_count {
            let raw = r.take(16)?;
            variables.push(Variable {
                raw_hex: hex(raw),
                name: r.string()?,
            });
        }
    }
    let efuns_count = r.i16()?;
    let max_functions = r.i16()?;
    if efuns_count != 559 {
        return Err(r.error("Magic559 requires 559 efun slots"));
    }
    if max_functions < 0 || max_vars < 0 {
        return Err(r.error("negative function/variable capacity"));
    }
    let mut hidden = Vec::new();
    for _ in 0..r.count()? {
        hidden.push(function(&mut r)?);
    }
    let mut inherits = Vec::new();
    let mut constitution_count = None;
    if mode == ProgramMode::Object {
        let count = r.i16()?;
        if count < 1 {
            return Err(r.error("empty constitution table"));
        }
        constitution_count = Some(count);
        for _ in 0..r.count()? {
            let name = r.string()?;
            let mut offsets = [0; 6];
            for n in &mut offsets {
                *n = r.i16()?;
            }
            if offsets.iter().any(|n| *n < 0) {
                return Err(r.error("negative inheritance offset/count"));
            }
            inherits.push(Inherit { name, offsets });
        }
    }
    let dlt_count = r.count()?;
    if dlt_count != names_count {
        return Err(r.error("name/DLT count mismatch"));
    }
    let mut dlts = Vec::new();
    let mut functions = Vec::new();
    for _ in 0..dlt_count {
        let offset = r.pos;
        let raw = r.take(20)?;
        let name_offset = word(raw, 0) as usize;
        let attrib = word(raw, 4);
        let rest = names
            .get(name_offset..)
            .filter(|s| !s.is_empty())
            .ok_or_else(|| r.error("DLT name offset outside pool"))?;
        let len = rest
            .iter()
            .position(|b| *b == 0)
            .ok_or_else(|| r.error("unterminated DLT name"))?;
        let name = ByteString::new(&rest[..len]);
        let index = short(raw, 16);
        if attrib & 0xc0000 == 0 {
            if attrib & 0x8000000 == 0 {
                return Err(r.error("unknown DLT kind"));
            }
            let mut f = function(&mut r)?;
            f.dlt_name = Some(name.clone());
            f.index = Some(index);
            functions.push(f);
        }
        dlts.push(Dlt {
            offset,
            name,
            attrib,
            index,
            raw_hex: hex(raw),
        });
    }
    let init = code(&mut r)?;
    if r.pos != bytes.len() {
        return Err(r.error("trailing bytes"));
    }
    if init.nodes.last().is_none_or(|n| n.2 != 0x1d) {
        return Err(r.error("missing final finish instruction"));
    }
    for segment in hidden
        .iter()
        .chain(functions.iter())
        .map(|f| &f.code)
        .chain(std::iter::once(&init))
    {
        for (pc, &(a, _, op)) in segment.nodes.iter().enumerate() {
            if op == 64 && (a < 0 || a as usize >= strings.len()) {
                return Err(ParseError::new(
                    segment.nodes_offset + pc * 4,
                    ErrorKind::Bounds,
                    "string index outside pool",
                ));
            }
            if op == 6 && (a < 0 || a >= efuns_count) {
                return Err(ParseError::new(
                    segment.nodes_offset + pc * 4,
                    ErrorKind::Bounds,
                    "efun index outside Magic559 table",
                ));
            }
        }
    }
    Ok(ProgramImage {
        mode,
        opaque_header,
        function_base,
        defines,
        strings,
        names_hex: hex(names),
        max_vars,
        vars_count,
        variables,
        efuns_count,
        max_functions,
        hidden,
        constitution_count,
        inherits,
        dlts,
        functions,
        init,
        bytes_consumed: r.pos,
        sha256: sha256(bytes),
    })
}

pub fn parse_program(bytes: &[u8], mode: ProgramMode) -> Result<ProgramImage, ParseError> {
    parse_program_with_profile(bytes, mode, AbiProfile::Magic559)
}
pub fn parse_program_with_profile(
    bytes: &[u8],
    mode: ProgramMode,
    profile: AbiProfile,
) -> Result<ProgramImage, ParseError> {
    // Shared magic is not an ABI discriminator. Only exact known artifact hashes are recognized automatically.
    if profile == AbiProfile::LegacyAuth389 || LEGACY_HASHES.contains(&sha256(bytes).as_str()) {
        return Err(ParseError::new(
            0,
            ErrorKind::UnsupportedFormat,
            "LegacyAuth389 is not supported by the stage-1 loader",
        ));
    }
    if mode != ProgramMode::Auto {
        return parse_mode(bytes, mode);
    }
    match (
        parse_mode(bytes, ProgramMode::Global),
        parse_mode(bytes, ProgramMode::Object),
    ) {
        (Ok(p), Err(_)) | (Err(_), Ok(p)) => Ok(p),
        (Ok(_), Ok(_)) => Err(ParseError::new(
            0,
            ErrorKind::Ambiguous,
            "both global and object modes accept this input",
        )),
        (Err(a), Err(b)) => Err(ParseError::new(
            a.offset.max(b.offset),
            ErrorKind::InvalidFormat,
            format!("global: {a}; object: {b}"),
        )),
    }
}

include!("baseline.rs");

#[cfg(test)]
mod argument_tests {
    use super::*;

    #[test]
    fn default_instruction_counts_are_decoded_as_signed_little_endian() {
        let values = [i16::MIN, -1, 0, 1, i16::MAX];
        let mut input = (values.len() as i16).to_le_bytes().to_vec();
        for value in values {
            let mut record = [0; 20];
            record[10..12].copy_from_slice(&value.to_le_bytes());
            input.extend(record);
            input.extend(0i32.to_le_bytes()); // absent name
        }
        let args = arguments(&mut Reader::new(&input)).unwrap();
        assert_eq!(
            args.iter()
                .map(|arg| arg.default_action_count)
                .collect::<Vec<_>>(),
            values
        );
    }
}
