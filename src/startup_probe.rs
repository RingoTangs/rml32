//! Isolated execution of PC 0..25 of a Global initialization segment.
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

pub const PREFIX_END_PC: usize = 25;

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
}
impl StartupProbe {
    /// Execute only the admitted prefix of a caller-supplied parsed image.
    /// This does not authenticate a mutable ProgramImage; `load` authenticates
    /// the five repository service members from their actual decompressed bytes.
    pub fn new(image: ProgramImage, limits: StartupProbeLimits) -> Result<Self, VmError> {
        let source = format!("sha256:{}", image.sha256);
        Self::prepare(image, limits, source)
    }
    pub fn load(
        root: &Path,
        service: StartupService,
        limits: StartupProbeLimits,
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
        Self::prepare(image, limits, source.into())
    }
    fn prepare(
        image: ProgramImage,
        limits: StartupProbeLimits,
        source: String,
    ) -> Result<Self, VmError> {
        let prepared = crate::prepared::prepare_startup_prefix(&image, &source)?;
        let nodes_offset = image.init.nodes_offset;
        nodes_offset.checked_add(PREFIX_END_PC * 4).ok_or_else(|| {
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
        let program = store.register_program(prepared)?;
        let mut vm = Vm::with_limits(store, limits.vm);
        vm.begin_startup_prefix(program)?;
        let next_dependency = image
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
            .transpose()?;
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
            } else if pc == PREFIX_END_PC {
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
            configuration: serde_json::json!({
                "values": {"max_host_roots":v.max_host_roots,"max_graph_nodes":v.max_graph_nodes,"max_depth":v.max_depth,"max_items":v.max_items},
                "vm": {"max_stack":m.max_stack,"max_frames":m.max_frames,"max_reference_cells":m.max_reference_cells},
                "lifecycle": {"max_objects":l.max_objects,"max_function_heads":l.max_function_heads,
                    "max_ordinary_refs_per_object":l.max_ordinary_refs_per_object,"max_exec_refs_per_object":l.max_exec_refs_per_object,"max_owners_per_header":l.max_owners_per_header},
            }),
        })
    }
}

#[cfg(test)]
#[path = "startup_probe_tests.rs"]
mod tests;
