//! Isolated execution of explicitly selected Global initialization slices.
//!
//! This is an execution capability, unlike `rml32-inspect`. It cannot resume
//! beyond the prefix, install callbacks, or initialize a real service.
use crate::{
    HostError, HostFunction, HostResult, HostReturn, HostSignature, HostType, LifecycleLimits,
    ObjectStore, PakArchive, ProgramImage, ProgramMode, ValueInput, ValueLimits, ValueSnapshot, Vm,
    VmError, VmErrorKind, VmLimits, parse_program,
};
use serde::Serialize;
use std::{cell::RefCell, fs, path::Path, rc::Rc};
#[path = "startup_memory_fs.rs"]
mod memory_fs;

pub const PREFIX_END_PC: usize = 25;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum StartupSlice {
    AllocationPrefix,
    LogDirectory,
}
impl StartupSlice {
    pub const fn end_pc(self) -> usize {
        match self {
            Self::AllocationPrefix => PREFIX_END_PC,
            Self::LogDirectory => 28,
        }
    }
    pub fn parse(name: &str) -> Option<Self> {
        match name {
            "allocate" => Some(Self::AllocationPrefix),
            "log-dir" => Some(Self::LogDirectory),
            _ => None,
        }
    }
}

/// Named, reproducible probe configuration; no implicit lifecycle defaults.
#[derive(Debug, Clone, Copy)]
pub struct StartupProbeLimits {
    pub values: ValueLimits,
    pub vm: VmLimits,
    pub lifecycle: LifecycleLimits,
}
impl StartupProbeLimits {
    pub const fn isolated() -> Self {
        Self {
            values: ValueLimits {
                max_host_roots: 32,
                max_graph_nodes: 8192,
                max_depth: 16,
                max_items: 4096,
            },
            vm: VmLimits {
                max_stack: 64,
                max_frames: 2,
                max_reference_cells: 64,
            },
            lifecycle: LifecycleLimits {
                max_objects: 0,
                max_function_heads: 0,
                max_ordinary_refs_per_object: 2,
                max_exec_refs_per_object: 1,
                max_owners_per_header: 8,
            },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum StartupService {
    Aaa,
    Ccs,
    Csa,
    Dba,
    Gs,
}
impl StartupService {
    pub fn parse(name: &str) -> Option<Self> {
        match name {
            "aaa" => Some(Self::Aaa),
            "ccs" => Some(Self::Ccs),
            "csa" => Some(Self::Csa),
            "dba" => Some(Self::Dba),
            "gs" => Some(Self::Gs),
            _ => None,
        }
    }
    pub fn source(self) -> &'static str {
        match self {
            Self::Aaa => "aaa/pack_data/lib_aaa32.pak/aaa/start_aaa.o",
            Self::Ccs => "ccs/pack_data/lib_ccs32.pak/ccs/start_ccs.o",
            Self::Csa => "csa/pack_data/lib_csa32.pak/csa/start_csa.o",
            Self::Dba => "dba/pack_data/lib_dba32.pak/dba/start_dba.o",
            Self::Gs => "gs/pack_data/lib_gs32.pak/gs/start_gs.o",
        }
    }
    pub fn sha256(self) -> &'static str {
        match self {
            Self::Aaa => "f9896828c87399be4977107fbfbe3f67fd71caebd3008bda84be16cc722d0a0e",
            Self::Ccs => "256301c54d7c2b8bdf50aeded93c27325dad80690a2f488249d04acfe3f59b47",
            Self::Csa => "9d73f41b25887221e015373f262945bd907a33af16785e5ea26fcdbad7df38a1",
            Self::Dba => "5fd2005c124be25643ebe19169ce2a3d798fbbde106efe1aa8b36287e0aebcd8",
            Self::Gs => "2f3ce58bb54584d2ce7bb3d90852893b97fcd5fb55d455b8eccff856fc696174",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StartupProbeOutcome {
    BudgetExhausted,
    PrefixReached,
}

/// Identity-free, owned data. This type conveys no live VM/store authority.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ProbeValue {
    Undefined,
    Int {
        value: i32,
    },
    Bytes {
        bytes_hex: String,
    },
    Array {
        elements: Vec<ProbeValue>,
    },
    Mapping {
        entries: Vec<(ProbeValue, ProbeValue)>,
    },
}
impl ProbeValue {
    fn from_snapshot(v: ValueSnapshot) -> Result<Self, VmError> {
        Ok(match v {
            ValueSnapshot::Undefined => Self::Undefined,
            ValueSnapshot::Int(value) => Self::Int { value },
            ValueSnapshot::Bytes(bytes) => Self::Bytes {
                bytes_hex: crate::hex(&bytes),
            },
            ValueSnapshot::Array { elements, .. } => Self::Array {
                elements: elements
                    .into_iter()
                    .map(Self::from_snapshot)
                    .collect::<Result<_, _>>()?,
            },
            ValueSnapshot::Mapping { entries, .. } => Self::Mapping {
                entries: entries
                    .into_iter()
                    .map(|(k, v)| Ok((Self::from_snapshot(k)?, Self::from_snapshot(v)?)))
                    .collect::<Result<_, VmError>>()?,
            },
            _ => {
                return Err(error(
                    VmErrorKind::UnsupportedType,
                    "value outside startup probe snapshot domain",
                ));
            }
        })
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ProbeHostCall {
    pub slot: u16,
    pub name: String,
    pub pc: usize,
    pub offset: usize,
    pub argument: i32,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ProbeDependency {
    pub pc: usize,
    pub offset: usize,
    pub slot: u16,
    pub name: String,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ProbeDependencyKind {
    GlobalFunction,
    Efun,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ProbeMkdirCall {
    pub slot: u16,
    pub name: String,
    pub pc: usize,
    pub offset: usize,
    pub argument: crate::ByteString,
    pub normalized: Option<crate::ByteString>,
    /// The backend result; VM publication may subsequently fail.
    pub result: Option<i32>,
    pub error: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(tag = "operation", content = "call", rename_all = "snake_case")]
pub enum ProbeHostEvent {
    AllocateArray(ProbeHostCall),
    Mkdir(ProbeMkdirCall),
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ProbeFilesystemEntry {
    pub path: crate::ByteString,
    pub directory: bool,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ProbeFilesystemSnapshot {
    pub entries: Vec<ProbeFilesystemEntry>,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct StartupProbeReport {
    pub source: String,
    pub sha256: String,
    pub function_base: i32,
    pub original_init_instructions: usize,
    pub executed_instructions: usize,
    pub stop_pc: usize,
    pub stop_offset: usize,
    pub stop_reason: String,
    pub initialization_complete: bool,
    pub host_calls: Vec<ProbeHostCall>,
    pub globals: Vec<ProbeValue>,
    pub next_dependency: Option<ProbeDependency>,
    pub configuration: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slice: Option<StartupSlice>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_events: Option<Vec<ProbeHostEvent>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filesystem: Option<ProbeFilesystemSnapshot>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_dependency_kind: Option<ProbeDependencyKind>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_dependency_reason: Option<String>,
}

fn error(kind: VmErrorKind, message: impl Into<String>) -> VmError {
    VmError::new(kind, message)
}
fn host_error(e: VmError) -> HostError {
    HostError {
        kind: e.kind,
        message: e.message.into(),
    }
}

/// Owns its VM. No store, callback registration or arbitrary resume API escapes.
pub struct StartupProbe {
    vm: Vm,
    source: String,
    sha256: String,
    nodes_offset: usize,
    function_base: i32,
    original_init_instructions: usize,
    next_dependency: Option<ProbeDependency>,
    calls: Rc<RefCell<Vec<ProbeHostCall>>>,
    limits: StartupProbeLimits,
    failure: Option<VmError>,
    slice: StartupSlice,
    filesystem: Option<Rc<RefCell<memory_fs::MemoryDirectories>>>,
    host_events: Option<Rc<RefCell<Vec<ProbeHostEvent>>>>,
    next_dependency_kind: Option<ProbeDependencyKind>,
    next_dependency_reason: Option<String>,
}
impl StartupProbe {
    /// Execute only the admitted prefix of a caller-supplied parsed image.
    /// This does not authenticate a mutable ProgramImage; `load` authenticates
    /// the five repository service members from their actual decompressed bytes.
    pub fn new(image: ProgramImage, limits: StartupProbeLimits) -> Result<Self, VmError> {
        Self::new_with_slice(image, limits, StartupSlice::AllocationPrefix)
    }
    pub fn new_with_slice(
        image: ProgramImage,
        limits: StartupProbeLimits,
        slice: StartupSlice,
    ) -> Result<Self, VmError> {
        let source = format!("sha256:{}", image.sha256);
        Self::prepare(image, limits, source, slice)
    }
    pub fn load(
        root: &Path,
        service: StartupService,
        limits: StartupProbeLimits,
    ) -> Result<Self, VmError> {
        Self::load_with_slice(root, service, limits, StartupSlice::AllocationPrefix)
    }
    pub fn load_with_slice(
        root: &Path,
        service: StartupService,
        limits: StartupProbeLimits,
        slice: StartupSlice,
    ) -> Result<Self, VmError> {
        let source = service.source();
        let (package, member) = source.split_once(".pak/").unwrap();
        let bytes = fs::read(root.join(format!("{package}.pak")))
            .map_err(|e| error(VmErrorKind::InvalidProgram, format!("read package: {e}")))?;
        let bytes = PakArchive::parse(&bytes)
            .and_then(|p| p.read_member(member))
            .map_err(|e| error(VmErrorKind::InvalidProgram, e.to_string()))?;
        if crate::sha256(&bytes) != service.sha256() {
            return Err(error(
                VmErrorKind::InvalidProgram,
                "startup member SHA-256 mismatch",
            ));
        }
        let image = parse_program(&bytes, ProgramMode::Global)
            .map_err(|e| error(VmErrorKind::InvalidProgram, e.to_string()))?;
        Self::prepare(image, limits, source.into(), slice)
    }
    fn prepare(
        image: ProgramImage,
        limits: StartupProbeLimits,
        source: String,
        slice: StartupSlice,
    ) -> Result<Self, VmError> {
        let prepared = match slice {
            StartupSlice::AllocationPrefix => {
                crate::prepared::prepare_startup_prefix(&image, &source)?
            }
            StartupSlice::LogDirectory => {
                crate::prepared::prepare_startup_slice(&image, &source, slice)?
            }
        };
        let nodes_offset = image.init.nodes_offset;
        nodes_offset
            .checked_add(slice.end_pc() * 4)
            .ok_or_else(|| {
                error(
                    VmErrorKind::InvalidProgram,
                    "startup boundary offset overflow",
                )
            })?;
        let mut store = ObjectStore::with_bounded_lifecycle(limits.values, limits.lifecycle)?;
        let mut globals = Vec::new();
        globals
            .try_reserve_exact(image.max_vars as usize)
            .map_err(|_| error(VmErrorKind::Limit, "global bank allocation failed"))?;
        globals.resize(image.max_vars as usize, ValueInput::Undefined);
        store.register_globals(&globals)?;
        let calls = Rc::new(RefCell::new(Vec::new()));
        let events = calls.clone();
        let filesystem = if slice == StartupSlice::LogDirectory {
            Some(Rc::new(RefCell::new(memory_fs::MemoryDirectories::new()?)))
        } else {
            None
        };
        let host_events = filesystem
            .as_ref()
            .map(|_| Rc::new(RefCell::new(Vec::new())));
        let complete_events = host_events.clone();
        store.register_host_function(
            0,
            HostFunction {
                name: "allocate_array".into(),
                signature: HostSignature {
                    required: vec![HostType::Int],
                    optional: vec![],
                    rest: None,
                    returns: HostReturn::Exact(HostType::Array),
                },
                callback: Rc::new(move |context, scope, args| {
                    let ValueSnapshot::Int(size) = args[0].snapshot().map_err(host_error)? else {
                        return Err(host_error(error(
                            VmErrorKind::UnsupportedType,
                            "allocate_array expects Int",
                        )));
                    };
                    let mut events = events.borrow_mut();
                    events.try_reserve(1).map_err(|_| {
                        host_error(error(VmErrorKind::Limit, "probe event allocation failed"))
                    })?;
                    events.push(ProbeHostCall {
                        slot: context.slot,
                        name: "allocate_array".into(),
                        pc: context.pc,
                        offset: context.offset,
                        argument: size,
                    });
                    if let Some(all) = &complete_events {
                        let mut all = all.borrow_mut();
                        crate::copy_plan::reserve(&mut all, 1).map_err(host_error)?;
                        all.push(ProbeHostEvent::AllocateArray(
                            events.last().unwrap().clone(),
                        ));
                    }
                    drop(events);
                    let count = usize::try_from(size).map_err(|_| {
                        host_error(error(VmErrorKind::Bounds, "negative array size"))
                    })?;
                    if count
                        > limits
                            .values
                            .max_items
                            .min(limits.values.max_graph_nodes)
                            .min(65536)
                    {
                        return Err(host_error(error(
                            VmErrorKind::Limit,
                            "allocate_array exceeds Rust value limits",
                        )));
                    }
                    let mut values = Vec::new();
                    values.try_reserve_exact(count).map_err(|_| {
                        host_error(error(VmErrorKind::Limit, "array allocation failed"))
                    })?;
                    values.resize(count, ValueInput::Undefined);
                    Ok(HostResult::Owned(scope.array(&values).map_err(host_error)?))
                }),
            },
        )?;
        if let (Some(directories), Some(all)) = (&filesystem, &host_events) {
            let directories = directories.clone();
            let all = all.clone();
            store.register_host_function(
                336,
                HostFunction {
                    name: "mkdir".into(),
                    signature: HostSignature {
                        required: vec![HostType::Bytes],
                        optional: vec![],
                        rest: None,
                        returns: HostReturn::Exact(HostType::Int),
                    },
                    callback: Rc::new(move |context, scope, args| {
                        let ValueSnapshot::Bytes(path) = args[0].snapshot().map_err(host_error)?
                        else {
                            return Err(host_error(error(
                                VmErrorKind::UnsupportedType,
                                "mkdir expects Bytes",
                            )));
                        };
                        let index = {
                            let mut events = all.borrow_mut();
                            crate::copy_plan::reserve(&mut events, 1).map_err(host_error)?;
                            let i = events.len();
                            events.push(ProbeHostEvent::Mkdir(ProbeMkdirCall {
                                slot: context.slot,
                                name: "mkdir".into(),
                                pc: context.pc,
                                offset: context.offset,
                                argument: crate::ByteString::new(&path),
                                normalized: None,
                                result: None,
                                error: None,
                            }));
                            i
                        };
                        let invoke = || -> Result<HostResult, VmError> {
                            let normalized = memory_fs::MemoryDirectories::normalize(&path)?;
                            if let ProbeHostEvent::Mkdir(call) = &mut all.borrow_mut()[index] {
                                call.normalized = Some(crate::ByteString::new(&normalized));
                            }
                            let result = directories.borrow_mut().mkdir(&normalized)?;
                            if let ProbeHostEvent::Mkdir(call) = &mut all.borrow_mut()[index] {
                                call.result = Some(result);
                            }
                            Ok(HostResult::Owned(scope.value(ValueInput::Int(result))?))
                        };
                        invoke().map_err(|e| {
                            if let ProbeHostEvent::Mkdir(call) = &mut all.borrow_mut()[index] {
                                call.error = Some(format!("{:?}", e.kind));
                            }
                            host_error(e)
                        })
                    }),
                },
            )?;
        }
        let program = store.register_program(prepared)?;
        let mut vm = Vm::with_limits(store, limits.vm);
        vm.begin_startup_prefix(program)?;
        let mut next_dependency_kind = None;
        let mut next_dependency_reason = None;
        let next_dependency = if slice == StartupSlice::AllocationPrefix {
            image
                .init
                .nodes
                .iter()
                .enumerate()
                .skip(PREFIX_END_PC)
                .find(|(_, n)| n.2 == 0x06)
                .and_then(|(pc, n)| (n.0 == 336).then_some((pc, n)))
                .map(|(pc, _)| {
                    Ok(ProbeDependency {
                        pc,
                        offset: nodes_offset.checked_add(pc * 4).ok_or_else(|| {
                            error(VmErrorKind::InvalidProgram, "dependency offset overflow")
                        })?,
                        slot: 336,
                        name: "mkdir".into(),
                    })
                })
                .transpose()?
        } else {
            image
                .init
                .nodes
                .iter()
                .enumerate()
                .skip(slice.end_pc())
                .find(|(_, n)| matches!(n.2, 0x06 | 0x07))
                .and_then(|(pc, n)| (n.0 >= 0).then_some((pc, n)))
                .map(|(pc, n)| {
                    next_dependency_kind = Some(if n.2 == 0x07 {
                        ProbeDependencyKind::GlobalFunction
                    } else {
                        ProbeDependencyKind::Efun
                    });
                    next_dependency_reason = Some(
                        if n.2 == 0x07
                            && image
                                .functions
                                .iter()
                                .any(|f| f.index == Some(n.0) && f.attrib & 2 != 0)
                        {
                            "atomic_function_not_enabled"
                        } else {
                            "call_not_prepared"
                        }
                        .into(),
                    );
                    let name = if n.2 == 0x07 {
                        image
                            .functions
                            .iter()
                            .find(|f| f.index == Some(n.0))
                            .and_then(|f| f.name.as_ref())
                            .map(|s| String::from_utf8_lossy(s.c_bytes()).into_owned())
                            .unwrap_or_else(|| "unprepared_global_function".into())
                    } else {
                        format!("unprepared_efun_{}", n.0)
                    };
                    Ok(ProbeDependency {
                        pc,
                        offset: nodes_offset.checked_add(pc * 4).ok_or_else(|| {
                            error(VmErrorKind::InvalidProgram, "dependency offset overflow")
                        })?,
                        slot: n.0 as u16,
                        name,
                    })
                })
                .transpose()?
        };
        Ok(Self {
            vm,
            source,
            sha256: image.sha256,
            nodes_offset,
            function_base: image.function_base,
            original_init_instructions: image.init.nodes.len(),
            next_dependency,
            calls,
            limits,
            failure: None,
            slice,
            filesystem,
            host_events,
            next_dependency_kind,
            next_dependency_reason,
        })
    }
    pub fn run(&mut self, budget: usize) -> Result<StartupProbeOutcome, VmError> {
        match self.vm.run_startup_prefix(budget) {
            Ok(true) => Ok(StartupProbeOutcome::PrefixReached),
            Ok(false) => Ok(StartupProbeOutcome::BudgetExhausted),
            Err(e) => {
                self.failure = Some(e.clone());
                Err(e)
            }
        }
    }
    /// A frozen observation, including partial state while budget-paused.
    pub fn report(&self) -> Result<StartupProbeReport, VmError> {
        // Some legacy instructions increment their internal PC before failing.
        // The diagnostic is the authoritative location of the failed attempt.
        let pc = self
            .failure
            .as_ref()
            .and_then(|e| e.pc)
            .unwrap_or(self.vm.startup_pc()?);
        let v = self.limits.values;
        let m = self.limits.vm;
        let l = self.limits.lifecycle;
        let mut configuration = serde_json::json!({
            "values": {"max_host_roots":v.max_host_roots,"max_graph_nodes":v.max_graph_nodes,"max_depth":v.max_depth,"max_items":v.max_items},
            "vm": {"max_stack":m.max_stack,"max_frames":m.max_frames,"max_reference_cells":m.max_reference_cells},
            "lifecycle": {"max_objects":l.max_objects,"max_function_heads":l.max_function_heads,
                "max_ordinary_refs_per_object":l.max_ordinary_refs_per_object,"max_exec_refs_per_object":l.max_exec_refs_per_object,"max_owners_per_header":l.max_owners_per_header},
        });
        if let Some(fs) = &self.filesystem {
            configuration["filesystem"] = serde_json::json!({"backend":"memory","max_nodes":fs.borrow().max_nodes,"max_path_bytes":memory_fs::MAX_PATH_BYTES});
        }
        Ok(StartupProbeReport {
            source: self.source.clone(),
            sha256: self.sha256.clone(),
            function_base: self.function_base,
            original_init_instructions: self.original_init_instructions,
            executed_instructions: pc,
            stop_pc: pc,
            stop_offset: self.nodes_offset + pc * 4,
            stop_reason: if self.failure.is_some() {
                "failed"
            } else if pc == self.slice.end_pc() {
                "prefix_boundary"
            } else {
                "budget_exhausted"
            }
            .into(),
            initialization_complete: false,
            host_calls: self.calls.borrow().clone(),
            globals: self
                .vm
                .objects()
                .global_values()?
                .into_iter()
                .map(ProbeValue::from_snapshot)
                .collect::<Result<_, _>>()?,
            next_dependency: self.next_dependency.clone(),
            configuration,
            slice: (self.slice == StartupSlice::LogDirectory).then_some(self.slice),
            host_events: self.host_events.as_ref().map(|v| v.borrow().clone()),
            filesystem: self.filesystem.as_ref().map(|v| v.borrow().snapshot()),
            next_dependency_kind: self.next_dependency_kind,
            next_dependency_reason: self.next_dependency_reason.clone(),
        })
    }
}

#[cfg(test)]
#[path = "startup_mkdir_tests.rs"]
mod mkdir_tests;
#[cfg(test)]
#[path = "startup_probe_tests.rs"]
mod tests;
