//! Bounded Magic559 interpreter; reference artifacts are never executed by tests.
use crate::{
    cell::{Cell, SlotPayload},
    object::{InitializationState, LifecycleState, ObjectStore},
    prepared::{EntryLayout, Instruction, PreparedFunction, PreparedProgram, ProgramKind},
    value::{ObjectId, ProgramId, Value},
    vm_error::{LifecycleContext, LifecyclePhase, VmError, VmErrorKind},
};
use std::{
    collections::BTreeMap,
    rc::{Rc, Weak},
};
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Completion {
    Returned(Value),
    ActionFinished,
    /// All initialization segments returned; no create/destruct functions were run.
    Initialized(ObjectId),
    Constructed(ObjectId),
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RunOutcome {
    Finished(Completion),
    BudgetExhausted,
}
#[derive(Debug, Clone, Copy)]
pub struct VmLimits {
    pub max_stack: usize,
    pub max_frames: usize,
    pub max_reference_cells: usize,
}
impl Default for VmLimits {
    fn default() -> Self {
        Self {
            max_stack: 65536,
            max_frames: 1024,
            max_reference_cells: 65536,
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReferenceKind {
    Local,
    Object,
    Global,
    ArrayMember,
    MappingMember,
    Temporary,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StackSnapshot {
    Value(Value),
    Reference {
        kind: ReferenceKind,
        target_id: u64,
        valid: bool,
    },
}
#[derive(Debug, Clone)]
enum Target {
    Global { cell: Rc<Cell> },
    Local { frame: u64, cell: Weak<Cell> },
    Object { object: ObjectId, cell: Weak<Cell> },
    Owned { cell: Rc<Cell>, kind: ReferenceKind },
}
#[derive(Debug, Clone)]
struct Reference {
    id: u64,
    target: Target,
    readonly: bool,
    _lease: Option<Rc<()>>,
}
impl Reference {
    fn kind(&self) -> ReferenceKind {
        match &self.target {
            Target::Global { .. } => ReferenceKind::Global,
            Target::Local { .. } => ReferenceKind::Local,
            Target::Object { .. } => ReferenceKind::Object,
            Target::Owned { kind, .. } => *kind,
        }
    }
    fn id(&self) -> u64 {
        self.id
    }
}
/// Private native tag-20 payload; never represented by a public Value.
#[derive(Debug, Clone)]
pub(crate) struct LoopControl {
    bound: Reference,
    state: LoopState,
}
#[cfg(test)]
impl LoopControl {
    pub(crate) fn for_test(owner: crate::container::ArrayValue) -> Self {
        Self {
            bound: Reference {
                id: 0,
                target: Target::Local {
                    frame: 0,
                    cell: Weak::new(),
                },
                readonly: false,
                _lease: None,
            },
            state: LoopState::Array { owner, index: 0 },
        }
    }
}
#[derive(Debug, Clone)]
enum LoopState {
    Numeric {
        next: i32,
        end: i32,
        step: i32,
        finished: bool,
    },
    Array {
        owner: crate::container::ArrayValue,
        index: u32,
    },
}
#[derive(Debug, Clone)]
enum StackEntry {
    Value(Rc<Cell>),
    Reference(Reference),
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum EntryPhase {
    Unchecked,
    Defaults { expected: usize },
    Ready,
    Body,
}
#[derive(Debug, Clone, Copy)]
enum FrameBody {
    Function(i16),
    Initialization(usize),
}
impl FrameBody {
    fn function(self) -> Option<i16> {
        match self {
            Self::Function(i) => Some(i),
            Self::Initialization(_) => None,
        }
    }
}
struct CodeView<'a> {
    instructions: &'a [Instruction],
    source: &'a str,
    pool: Option<&'a Vec<Option<Value>>>,
    function_base: i16,
    variable_base: i16,
    local_count: usize,
    entry: Option<&'a EntryLayout>,
    function: Option<&'a PreparedFunction>,
}
impl<'a> CodeView<'a> {
    fn function(&self) -> Result<&'a PreparedFunction, VmError> {
        self.function.ok_or_else(|| {
            err(
                VmErrorKind::UnsupportedFeature,
                "ordinary function operation in initialization",
            )
        })
    }
    fn get(program: &'a PreparedProgram, body: FrameBody) -> Result<Self, VmError> {
        match body {
            FrameBody::Function(index) => {
                let f = program.functions.get(&index).ok_or_else(|| {
                    err(VmErrorKind::InvalidProgram, "current function is missing")
                })?;
                Ok(Self {
                    instructions: &f.instructions,
                    source: f.origin.as_deref().unwrap_or(&program.source),
                    pool: f.pool.as_deref(),
                    function_base: f.function_base,
                    variable_base: f.variable_base,
                    local_count: f.local_count,
                    entry: f.entry.as_ref(),
                    function: Some(f),
                })
            }
            FrameBody::Initialization(index) => {
                let init = program
                    .initializations
                    .as_ref()
                    .and_then(|v| v.get(index))
                    .ok_or_else(|| {
                        err(
                            VmErrorKind::InvalidProgram,
                            "initialization body is missing",
                        )
                    })?;
                Ok(Self {
                    instructions: &init.instructions,
                    source: &init.source,
                    pool: Some(&init.pool),
                    function_base: init.function_base,
                    variable_base: init.variable_base,
                    local_count: 0,
                    entry: None,
                    function: None,
                })
            }
        }
    }
}
#[derive(Debug)]
struct InitializationTask {
    object: ObjectId,
    position: usize,
    constitution: usize,
}
#[derive(Debug)]
struct ConstructionTask {
    object: ObjectId,
    program: ProgramId,
    phase: LifecyclePhase,
    position: usize,
    constitution: usize,
    parameter: Option<Value>,
    primary: Option<VmError>,
}
#[derive(Debug, Clone, Copy)]
enum ExecutionSource {
    ObjectProgram(ProgramId),
    GlobalProgram(ProgramId),
}
impl ExecutionSource {
    fn program(self) -> ProgramId {
        match self {
            Self::ObjectProgram(p) | Self::GlobalProgram(p) => p,
        }
    }
}
#[derive(Debug, Clone)]
struct Frame {
    id: u64,
    execution: ExecutionSource,
    active_object: Option<ObjectId>,
    body: FrameBody,
    pc: usize,
    bp: usize,
    argc: usize,
    entered: bool,
    phase: EntryPhase,
}
impl Frame {
    fn program(&self) -> ProgramId {
        self.execution.program()
    }
    fn object(&self) -> Result<ObjectId, VmError> {
        self.active_object.ok_or_else(|| {
            err(
                VmErrorKind::UnsupportedFeature,
                "operation requires an active Object",
            )
        })
    }
}
#[derive(Debug)]
struct ScriptCatch {
    frame_id: u64,
    frame_depth: usize,
    stack_len: usize,
    target_pc: usize,
}
#[derive(Debug, Clone, Copy)]
// Keep the zero qualifier explicit: overflow/type faults at the same arithmetic
// operations are deliberately excluded from these catchable provenance sites.
#[allow(clippy::enum_variant_names)]
enum RuntimeFaultSite {
    IntDivZero,
    NumericDivZero,
    IntRemZero,
    CompoundDivZero,
    CompoundIntRemZero,
}
impl RuntimeFaultSite {
    // Only called at a failed arithmetic operation, after operand/ref validation.
    // This classifies the exact native vm_error site, never diagnostic text/kind.
    fn binary(opcode: u8, left: &Value, right: &Value) -> Option<Self> {
        match (opcode, left, right) {
            (0x19, Value::Int(_), Value::Int(0)) => Some(Self::IntDivZero),
            (0x19, Value::Real(_), Value::Int(0)) => Some(Self::NumericDivZero),
            (0x19, Value::Int(_) | Value::Real(_), Value::Real(r)) if r.to_f32() == 0.0 => {
                Some(Self::NumericDivZero)
            }
            (0x28, Value::Int(_), Value::Int(0)) => Some(Self::IntRemZero),
            _ => None,
        }
    }
    fn compound(selector: i32, left: &Value, right: &Value) -> Option<Self> {
        match selector {
            5 => Self::binary(0x19, left, right).map(|_| Self::CompoundDivZero),
            11 => Self::binary(0x28, left, right).map(|_| Self::CompoundIntRemZero),
            _ => None,
        }
    }
}
#[derive(Debug)]
struct StepFault {
    diagnostic: Box<VmError>,
    site: Option<RuntimeFaultSite>,
}
#[derive(Debug)]
pub struct Vm {
    objects: ObjectStore,
    stack: Vec<StackEntry>,
    frames: Vec<Frame>,
    catches: Vec<ScriptCatch>,
    limits: VmLimits,
    next_frame: u64,
    reference_cells: BTreeMap<u64, Weak<()>>,
    completion: Option<Completion>,
    failure: Option<VmError>,
    initialization: Option<InitializationTask>,
    construction: Option<ConstructionTask>,
    rollback_errors: Vec<VmError>,
}
fn err(kind: VmErrorKind, message: impl Into<String>) -> VmError {
    VmError::new(kind, message)
}
impl Vm {
    pub fn new(objects: ObjectStore) -> Self {
        Self {
            objects,
            stack: vec![],
            frames: vec![],
            catches: vec![],
            limits: VmLimits::default(),
            next_frame: 0,
            reference_cells: BTreeMap::new(),
            completion: None,
            failure: None,
            initialization: None,
            construction: None,
            rollback_errors: vec![],
        }
    }
    pub fn with_limits(objects: ObjectStore, limits: VmLimits) -> Self {
        Self {
            limits,
            ..Self::new(objects)
        }
    }
    pub fn objects(&self) -> &ObjectStore {
        &self.objects
    }
    pub fn objects_mut(&mut self) -> &mut ObjectStore {
        &mut self.objects
    }
    /// Extract storage, cancelling and deleting any unfinished managed lifecycle object.
    pub fn into_objects(mut self) -> ObjectStore {
        self.cleanup_initialization();
        self.objects
    }
    fn cleanup_initialization(&mut self) {
        let init = self.initialization.take();
        let construct = self.construction.take();
        if let Some(object) = construct
            .map(|t| t.object)
            .or_else(|| init.map(|t| t.object))
        {
            self.clear_task_frames();
            let _ = self.objects.remove(object);
        }
    }
    /// Allocate a new object and start only its bounded initialization segments.
    pub fn begin_initialize(&mut self, program: ProgramId) -> Result<ObjectId, VmError> {
        if self.initialization.is_some()
            || self.construction.is_some()
            || (!self.frames.is_empty() && self.completion.is_none() && self.failure.is_none())
        {
            return Err(err(
                VmErrorKind::InvalidState,
                "an execution is already active",
            ));
        }
        let p = self.objects.program(program)?;
        if !p.is_initializable() {
            return Err(err(
                VmErrorKind::UnsupportedFeature,
                "program was not prepared for initialization",
            ));
        }
        if self.limits.max_frames == 0 {
            return Err(err(
                VmErrorKind::Limit,
                "initialization exceeds frame limit",
            ));
        }
        let constitution = *p
            .init_order
            .first()
            .ok_or_else(|| err(VmErrorKind::InvalidProgram, "missing initialization order"))?;
        let id = self.frame_id()?;
        let object = self
            .objects
            .create(program, vec![Value::Undefined; p.variable_count])?;
        self.objects
            .set_initialization(object, InitializationState::Initializing)?;
        self.stack.clear();
        self.catches.clear();
        self.reference_cells.clear();
        self.frames = vec![Self::initialization_frame(
            id,
            program,
            object,
            constitution,
        )];
        self.initialization = Some(InitializationTask {
            object,
            position: 0,
            constitution,
        });
        self.completion = None;
        self.failure = None;
        self.rollback_errors.clear();
        Ok(object)
    }
    fn initialization_frame(
        id: u64,
        program: ProgramId,
        object: ObjectId,
        constitution: usize,
    ) -> Frame {
        Frame {
            id,
            execution: ExecutionSource::ObjectProgram(program),
            active_object: Some(object),
            body: FrameBody::Initialization(constitution),
            pc: 0,
            bp: 0,
            argc: 0,
            entered: false,
            phase: EntryPhase::Body,
        }
    }
    fn return_initialization(&mut self, frame: &Frame, p: &PreparedProgram) -> Result<(), VmError> {
        if !matches!(frame.body, FrameBody::Initialization(_))
            || self.frames.len() != 1
            || self.stack.len() != frame.bp
        {
            return Err(err(
                VmErrorKind::InvalidProgram,
                "initialization return requires a balanced root initialization frame",
            ));
        }
        let task = self
            .initialization
            .as_ref()
            .ok_or_else(|| err(VmErrorKind::InvalidState, "missing initialization task"))?;
        let position = task.position + 1;
        if let Some(&constitution) = p.init_order.get(position) {
            let id = self.frame_id()?;
            self.frames[0] =
                Self::initialization_frame(id, frame.program(), frame.object()?, constitution);
            let task = self.initialization.as_mut().unwrap();
            task.position = position;
            task.constitution = constitution;
        } else {
            self.objects
                .set_initialization(frame.object()?, InitializationState::Initialized)?;
            self.frames.clear();
            self.reference_cells.clear();
            self.initialization = None;
            if let Some(task) = self.construction.as_mut() {
                task.phase = LifecyclePhase::Create;
                task.position = 0;
                self.advance_construction();
            } else {
                self.completion = Some(Completion::Initialized(frame.object()?));
            }
        }
        Ok(())
    }
    /// Errors suppressed while attempting the remaining rollback destructors.
    pub fn rollback_errors(&self) -> &[VmError] {
        &self.rollback_errors
    }
    /// Allocate, initialize and construct a new object; only mapping parameters are accepted.
    pub fn begin_construct(
        &mut self,
        program: ProgramId,
        parameter: Option<Value>,
    ) -> Result<ObjectId, VmError> {
        let p = self.objects.program(program)?;
        if !p.is_constructible() {
            return Err(err(
                VmErrorKind::UnsupportedFeature,
                "program was not prepared for construction",
            ));
        }
        if parameter
            .as_ref()
            .is_some_and(|v| !matches!(v, Value::Mapping(_)))
        {
            return Err(err(
                VmErrorKind::UnsupportedType,
                "construction parameter must be a mapping",
            ));
        }
        let object = self.begin_initialize(program)?;
        self.objects
            .set_lifecycle(object, LifecycleState::Constructing)?;
        let constitution = self.initialization.as_ref().unwrap().constitution;
        self.construction = Some(ConstructionTask {
            object,
            program,
            phase: LifecyclePhase::Initialize,
            position: 0,
            constitution,
            parameter,
            primary: None,
        });
        Ok(object)
    }
    fn clear_task_frames(&mut self) {
        self.catches.clear();
        self.frames.clear();
        self.stack.clear();
        self.reference_cells.clear();
    }
    fn lifecycle_context(&self) -> Option<Box<LifecycleContext>> {
        self.construction.as_ref().map(|t| {
            Box::new(LifecycleContext {
                phase: t.phase,
                constitution: if t.phase == LifecyclePhase::Initialize {
                    self.initialization
                        .as_ref()
                        .map_or(t.constitution, |i| i.constitution)
                } else {
                    t.constitution
                },
            })
        })
    }
    fn finish_construction_failure(&mut self) {
        let task = self.construction.take().unwrap();
        let primary = task
            .primary
            .expect("failed construction has a primary error");
        self.initialization = None;
        self.clear_task_frames();
        let _ = self.objects.remove(task.object);
        self.failure = Some(primary);
    }
    fn construction_error(&mut self, error: VmError) {
        let task = self.construction.as_mut().unwrap();
        let missing = self.objects.get(task.object).is_err();
        if task.primary.is_none() {
            task.primary = Some(error);
        } else {
            self.rollback_errors.push(error);
        }
        self.clear_task_frames();
        if missing || self.construction.as_ref().unwrap().phase == LifecyclePhase::Initialize {
            self.finish_construction_failure();
        } else {
            self.construction.as_mut().unwrap().phase = LifecyclePhase::RollbackDestruct;
            let object = self.construction.as_ref().unwrap().object;
            let _ = self
                .objects
                .set_lifecycle(object, LifecycleState::RollingBack);
        }
    }
    fn lifecycle_entry_error(&self, mut e: VmError, p: &PreparedProgram, slot: i16) -> VmError {
        let task = self.construction.as_ref().unwrap();
        e.program = Some(task.program);
        e.object = Some(task.object);
        e.function = Some(slot);
        e.lifecycle = self.lifecycle_context();
        e.source = p
            .functions
            .get(&slot)
            .and_then(|f| f.origin.clone())
            .unwrap_or_else(|| p.source.clone());
        e.pc = None;
        e.offset = None;
        e
    }
    fn enter_lifecycle(
        &mut self,
        p: &PreparedProgram,
        slot: i16,
        parameter: Option<Value>,
    ) -> Result<(), VmError> {
        let f = p.functions.get(&slot).ok_or_else(|| {
            err(
                VmErrorKind::InvalidProgram,
                "missing bound lifecycle function",
            )
        })?;
        let argc = usize::from(parameter.is_some());
        Self::argument_plan(f, argc)?;
        if self.limits.max_frames == 0 || argc > self.limits.max_stack {
            return Err(err(VmErrorKind::Limit, "lifecycle entry exceeds VM limits"));
        }
        let arguments = parameter
            .into_iter()
            .map(|v| Cell::new(v).map(StackEntry::Value))
            .collect::<Result<Vec<_>, _>>()?;
        let id = self.frame_id()?;
        let task = self.construction.as_ref().unwrap();
        self.frames = vec![Frame {
            id,
            execution: ExecutionSource::ObjectProgram(task.program),
            active_object: Some(task.object),
            body: FrameBody::Function(slot),
            pc: 0,
            bp: argc,
            argc,
            entered: false,
            phase: EntryPhase::Unchecked,
        }];
        self.stack = arguments;
        self.catches.clear();
        Ok(())
    }
    // Metadata transitions are bounded by the already validated constitution count.
    fn advance_construction(&mut self) {
        while self.construction.is_some() && self.frames.is_empty() {
            let task = self.construction.as_ref().unwrap();
            let p = self
                .objects
                .program(task.program)
                .expect("registered task program remains present");
            if self.objects.get(task.object).is_err() {
                let mut e = err(
                    VmErrorKind::InvalidObject,
                    "construction object was removed",
                );
                e.program = Some(task.program);
                e.object = Some(task.object);
                e.lifecycle = self.lifecycle_context();
                e.source = p.source.clone();
                self.construction_error(e);
                return;
            }
            let phase = task.phase;
            let position = task.position;
            if phase == LifecyclePhase::Create && position == p.init_order.len() {
                let object = task.object;
                self.objects
                    .set_lifecycle(object, LifecycleState::Constructed)
                    .expect("task object is present");
                self.construction = None;
                self.completion = Some(Completion::Constructed(object));
                return;
            }
            if phase == LifecyclePhase::RollbackDestruct && position == 0 {
                self.finish_construction_failure();
                return;
            }
            let next = if phase == LifecyclePhase::Create {
                position
            } else {
                position - 1
            };
            let constitution = p.init_order[next];
            let task = self.construction.as_mut().unwrap();
            task.constitution = constitution;
            task.position = if phase == LifecyclePhase::Create {
                position + 1
            } else {
                next
            };
            let binding = p.lifecycles.as_ref().unwrap()[constitution];
            let (slot, parameter) = if phase == LifecyclePhase::Create {
                (binding.create, task.parameter.clone())
            } else {
                (binding.destruct, None)
            };
            let Some(slot) = slot else {
                continue;
            };
            if let Err(e) = self.enter_lifecycle(&p, slot, parameter) {
                let e = self.lifecycle_entry_error(e, &p, slot);
                self.construction_error(e);
            }
        }
    }
    pub fn frame_depth(&self) -> usize {
        self.frames.len()
    }
    pub fn stack(&self) -> Result<Vec<StackSnapshot>, VmError> {
        self.stack
            .iter()
            .map(|e| match e {
                StackEntry::Value(c) => Ok(StackSnapshot::Value(c.read()?)),
                StackEntry::Reference(r) => Ok(StackSnapshot::Reference {
                    kind: r.kind(),
                    target_id: r.id(),
                    valid: self.resolve(r).is_ok(),
                }),
            })
            .collect()
    }
    fn frame_id(&mut self) -> Result<u64, VmError> {
        let id = self.next_frame;
        self.next_frame = id
            .checked_add(1)
            .ok_or_else(|| err(VmErrorKind::Limit, "frame ID space exhausted"))?;
        Ok(id)
    }
    fn copy_value(&self, value: &Value) -> Value {
        match value {
            Value::Object(id) if self.objects.get(*id).is_err() => Value::Undefined,
            Value::Function(f)
                if Rc::ptr_eq(&f.0.origin, &self.objects.function_origin)
                    && self.objects.get(f.owner()).is_err() =>
            {
                Value::Undefined
            }
            _ => value.clone(),
        }
    }
    fn value(&self, pos: usize) -> Result<Value, VmError> {
        match &self.stack[pos] {
            StackEntry::Value(c) => c.read(),
            StackEntry::Reference(_) => Err(err(
                VmErrorKind::UnsupportedFeature,
                "reference requires explicit dereference; reference escape is unsupported",
            )),
        }
    }
    fn set(&mut self, pos: usize, value: Value) -> Result<(), VmError> {
        match &self.stack[pos] {
            StackEntry::Value(c) => c.replace(value),
            StackEntry::Reference(_) => {
                self.stack[pos] = StackEntry::Value(Cell::new(value)?);
                Ok(())
            }
        }
    }
    fn push_entry(&mut self, e: StackEntry) -> Result<(), VmError> {
        if self.stack.len() >= self.limits.max_stack {
            return Err(err(VmErrorKind::Limit, "value stack limit exceeded"));
        }
        self.stack.push(e);
        Ok(())
    }
    fn push(&mut self, v: Value) -> Result<(), VmError> {
        self.push_entry(StackEntry::Value(Cell::new(v)?))
    }
    fn reference(&self, pos: usize) -> Result<Reference, VmError> {
        match &self.stack[pos] {
            StackEntry::Reference(r) => Ok(r.clone()),
            _ => Err(err(
                VmErrorKind::UnsupportedType,
                "instruction requires a reference",
            )),
        }
    }
    fn resolve(&self, r: &Reference) -> Result<Rc<Cell>, VmError> {
        match &r.target {
            Target::Local { frame, cell } => {
                if !self.frames.iter().any(|f| f.id == *frame) {
                    return Err(err(VmErrorKind::InvalidState, "reference frame has exited"));
                }
                cell.upgrade()
                    .ok_or_else(|| err(VmErrorKind::InvalidState, "reference slot was consumed"))
            }
            Target::Object { object, cell } => {
                self.objects.get(*object)?;
                cell.upgrade()
                    .ok_or_else(|| err(VmErrorKind::InvalidObject, "object slot was removed"))
            }
            Target::Global { cell } | Target::Owned { cell, .. } => Ok(cell.clone()),
        }
    }
    fn writable(&self, r: &Reference, value: &Value) -> Result<Rc<Cell>, VmError> {
        if r.readonly {
            return Err(err(
                VmErrorKind::UnsupportedFeature,
                "cannot modify const reference",
            ));
        }
        let cell = self.resolve(r)?;
        if matches!(
            r.kind(),
            ReferenceKind::ArrayMember | ReferenceKind::MappingMember
        ) && matches!(value, Value::Array(_) | Value::Mapping(_))
        {
            return Err(err(
                VmErrorKind::UnsupportedType,
                "container member writes require scalar values",
            ));
        }
        Ok(cell)
    }
    fn validate_binding(&self, r: &Reference) -> Result<(), VmError> {
        let cell = self.resolve(r)?;
        if cell.is_loop()? {
            return Err(err(
                VmErrorKind::UnsupportedFeature,
                "hidden loop controls cannot bind reference parameters or loop variables",
            ));
        }
        if let Target::Local { frame, .. } = r.target {
            let owner = self
                .frames
                .iter()
                .find(|f| f.id == frame)
                .ok_or_else(|| err(VmErrorKind::InvalidState, "reference frame has exited"))?;
            let program = self.objects.program(owner.program())?;
            let locals = if owner.entered {
                CodeView::get(&program, owner.body)?.local_count
            } else {
                0
            };
            let declared = &self.stack[owner.bp - owner.argc..owner.bp + locals];
            if !declared
                .iter()
                .any(|entry| matches!(entry, StackEntry::Value(c) if c.id() == cell.id()))
            {
                return Err(err(
                    VmErrorKind::UnsupportedFeature,
                    "reference argument originates in an evaluation slot",
                ));
            }
        }
        Ok(())
    }
    // Allocate the result before normalizing a dead Object in its source Cell.
    fn read_reference(&self, r: &Reference) -> Result<StackEntry, VmError> {
        let cell = self.resolve(r)?;
        let original = cell.read()?;
        let value = self.copy_value(&original);
        let entry = StackEntry::Value(Cell::new(value.clone())?);
        if value != original {
            cell.replace(value)?;
        }
        Ok(entry)
    }
    fn reserve_reference(&mut self, id: Option<u64>) -> Result<(), VmError> {
        // Bound registry memory without scanning all live slots on every query.
        if self.reference_cells.len() >= self.limits.max_reference_cells {
            self.reference_cells.retain(|_, w| w.strong_count() != 0);
        }
        if id.is_none_or(|id| !self.reference_cells.contains_key(&id))
            && self.reference_cells.len() >= self.limits.max_reference_cells
        {
            return Err(err(
                VmErrorKind::Limit,
                "live temporary/member reference cell limit exceeded",
            ));
        }
        Ok(())
    }
    fn owned_reference(
        &mut self,
        cell: Rc<Cell>,
        kind: ReferenceKind,
    ) -> Result<Reference, VmError> {
        self.reserve_reference(Some(cell.id()))?;
        let lease =
            if let Some(token) = self.reference_cells.get(&cell.id()).and_then(Weak::upgrade) {
                token
            } else {
                let t = Rc::new(());
                self.reference_cells.insert(cell.id(), Rc::downgrade(&t));
                t
            };
        Ok(Reference {
            id: cell.id(),
            target: Target::Owned { cell, kind },
            readonly: false,
            _lease: Some(lease),
        })
    }
    pub fn start(
        &mut self,
        object: ObjectId,
        function_index: i16,
        arguments: Vec<Value>,
    ) -> Result<(), VmError> {
        if !self.frames.is_empty() && self.completion.is_none() && self.failure.is_none() {
            return Err(err(
                VmErrorKind::InvalidState,
                "an execution is already active",
            ));
        }
        if self.objects.get(object)?.initialization_state() == InitializationState::Initializing
            || matches!(
                self.objects.get(object)?.lifecycle_state(),
                LifecycleState::Constructing | LifecycleState::RollingBack
            )
        {
            return Err(err(
                VmErrorKind::InvalidState,
                "object initialization is incomplete",
            ));
        }
        let program = self.objects.get(object)?.program();
        let p = self.objects.program(program)?;
        if p.unimplemented.contains(&function_index) {
            return Err(err(
                VmErrorKind::UnsupportedFeature,
                "entry declaration is unimplemented",
            ));
        }
        if !p.functions.contains_key(&function_index) {
            return Err(err(
                VmErrorKind::InvalidProgram,
                "entry function does not exist",
            ));
        }
        if p.functions[&function_index]
            .args
            .iter()
            .any(|a| a.flag == 1)
        {
            return Err(err(
                VmErrorKind::UnsupportedFeature,
                "reference entry requires an internal wrapper call",
            ));
        }
        Self::check_user_entry(&p.functions[&function_index])?;
        Self::argument_plan(&p.functions[&function_index], arguments.len())?;
        if self.limits.max_frames == 0 || arguments.len() > self.limits.max_stack {
            return Err(err(VmErrorKind::Limit, "entry exceeds VM limits"));
        }
        let arguments = arguments
            .iter()
            .rev()
            .map(|v| Cell::new(self.copy_value(v)).map(StackEntry::Value))
            .collect::<Result<Vec<_>, _>>()?;
        let id = self.frame_id()?;
        self.stack = arguments;
        self.catches.clear();
        self.frames = vec![Frame {
            id,
            execution: ExecutionSource::ObjectProgram(program),
            active_object: Some(object),
            body: FrameBody::Function(function_index),
            pc: 0,
            bp: self.stack.len(),
            argc: self.stack.len(),
            entered: false,
            phase: EntryPhase::Unchecked,
        }];
        self.reference_cells.clear();
        self.completion = None;
        self.failure = None;
        self.rollback_errors.clear();
        Ok(())
    }
    pub fn run(&mut self, instruction_budget: usize) -> Result<RunOutcome, VmError> {
        if let Some(e) = &self.failure {
            return Err(e.clone());
        }
        if let Some(c) = &self.completion {
            return Ok(RunOutcome::Finished(c.clone()));
        }
        if self.frames.is_empty() {
            return Err(err(
                VmErrorKind::InvalidState,
                "no execution has been started",
            ));
        }
        for _ in 0..instruction_budget {
            let frame = self.frames.last().unwrap().clone();
            let initialization = self.initialization.as_ref().map(|t| t.constitution);
            if let Err(fault) = self.step() {
                let mut e = *fault.diagnostic;
                e.program = Some(frame.program());
                e.object = frame.active_object;
                e.function = frame.body.function();
                e.initialization = initialization;
                e.lifecycle = self.lifecycle_context();
                e.pc = Some(frame.pc);
                if let Ok(p) = self.objects.program(frame.program())
                    && let Ok(code) = CodeView::get(&p, frame.body)
                {
                    e.source = code.source.to_owned();
                    e.offset = code.instructions.get(frame.pc).map(|i| i.offset);
                }
                // Capture the actual failing instruction before changing frame/stack state.
                if fault.site.is_some() {
                    match self.route_script_fault() {
                        Ok(true) => continue,
                        Ok(false) => {}
                        Err(invalid) => {
                            e.kind = invalid.kind;
                            e.message = invalid.message;
                        }
                    }
                }
                if self.construction.is_some() {
                    self.construction_error(e);
                    self.advance_construction();
                    if let Some(e) = &self.failure {
                        return Err(e.clone());
                    }
                } else {
                    self.cleanup_initialization();
                    self.failure = Some(e.clone());
                    return Err(e);
                }
            }
            if let Some(e) = &self.failure {
                return Err(e.clone());
            }
            if let Some(c) = &self.completion {
                return Ok(RunOutcome::Finished(c.clone()));
            }
        }
        Ok(RunOutcome::BudgetExhausted)
    }
    fn context(&self) -> Result<(Frame, Rc<PreparedProgram>), VmError> {
        let frame = self
            .frames
            .last()
            .ok_or_else(|| err(VmErrorKind::InvalidState, "missing call frame"))?
            .clone();
        let p = self.objects.program(frame.program())?;
        match (frame.execution, frame.active_object, p.kind) {
            (ExecutionSource::ObjectProgram(_), Some(object), ProgramKind::Object) => {
                self.objects.get(object)?;
            }
            (ExecutionSource::GlobalProgram(_), None, ProgramKind::Global) => {}
            _ => {
                return Err(err(
                    VmErrorKind::InvalidState,
                    "frame code and object contexts disagree",
                ));
            }
        }
        Ok((frame, p))
    }
    fn need(&self, n: usize, frame: &Frame, local_count: usize) -> Result<(), VmError> {
        let floor = frame.bp + if frame.entered { local_count } else { 0 };
        if self.stack.len().saturating_sub(floor) < n {
            return Err(err(
                VmErrorKind::StackUnderflow,
                "instruction would consume argument/local storage",
            ));
        }
        Ok(())
    }
    fn jump(&mut self, delta: i32, code_len: usize) -> Result<(), VmError> {
        let frame = self.frames.last_mut().unwrap();
        let target = frame.pc as i64 + i64::from(delta);
        if target < 0 || target >= code_len as i64 {
            return Err(err(
                VmErrorKind::Bounds,
                "branch target outside function code",
            ));
        }
        frame.pc = target as usize;
        Ok(())
    }
    fn slot(&self, no: i32, frame: &Frame) -> Option<usize> {
        if i64::from(no) >= frame.argc as i64 {
            return None;
        }
        let pos = frame.bp as i64 - 1 - i64::from(no);
        (pos >= 0 && pos < self.stack.len() as i64).then_some(pos as usize)
    }
    fn object_cell(&self, frame: &Frame, index: i32) -> Result<Rc<Cell>, VmError> {
        let object = frame.object()?;
        let p = self.objects.program(frame.program())?;
        let base = CodeView::get(&p, frame.body)?.variable_base;
        usize::try_from(index + i32::from(base))
            .ok()
            .and_then(|i| self.objects.get(object).ok()?.variables.get(i))
            .cloned()
            .ok_or_else(|| err(VmErrorKind::Bounds, "object variable index outside storage"))
    }
    // Returns the skipped default instruction count and number of new slots.
    fn check_user_entry(f: &PreparedFunction) -> Result<(), VmError> {
        if f.origin.is_some()
            && f.name.as_ref().is_some_and(|n| {
                matches!(n.split(|b| *b == 0).next(), Some(b"create" | b"destruct"))
            })
        {
            return Err(err(
                VmErrorKind::UnsupportedFeature,
                "linked lifecycle execution is unsupported",
            ));
        }
        Ok(())
    }
    fn argument_plan(f: &PreparedFunction, argc: usize) -> Result<(usize, usize), VmError> {
        if argc < f.min_args || (!f.variadic && argc > f.max_args) {
            return Err(err(
                VmErrorKind::ArgumentCount,
                "argument count outside prototype range",
            ));
        }
        let mut gap = false;
        let mut expected = 0;
        for arg in f.args.iter().skip(argc) {
            if arg.flag == 1 {
                return Err(err(
                    VmErrorKind::UnsupportedFeature,
                    "omitted reference parameter",
                ));
            }
            if arg.default_action_count == 0 {
                if arg.r#type != 255 {
                    return Err(err(
                        VmErrorKind::InvalidProgram,
                        "missing typed argument has no default",
                    ));
                }
                gap = true;
            } else {
                if gap {
                    return Err(err(
                        VmErrorKind::UnsupportedFeature,
                        "default results would leave a parameter hole",
                    ));
                }
                expected += 1;
            }
        }
        let skip = f
            .args
            .iter()
            .take(argc)
            .map(|a| a.default_action_count as usize)
            .sum();
        Ok((skip, expected))
    }
    fn argument_array(&mut self, frame: &Frame) -> Result<(), VmError> {
        if self.stack.len() >= self.limits.max_stack {
            return Err(err(
                VmErrorKind::Limit,
                "argument array result exceeds stack limit",
            ));
        }
        let mut values = Vec::with_capacity(frame.argc);
        let mut updates = BTreeMap::new();
        for i in 0..frame.argc {
            let pos = frame.bp - 1 - i;
            let value = match &self.stack[pos] {
                StackEntry::Value(cell) => self.copy_value(&cell.read()?),
                StackEntry::Reference(r) => {
                    let cell = self.resolve(r)?;
                    let original = cell.read()?;
                    let value = self.copy_value(&original);
                    if value != original {
                        updates.insert(cell.id(), (cell, value.clone()));
                    }
                    value
                }
            };
            values.push(value);
        }
        let result = StackEntry::Value(Cell::new(Value::array(values)?)?);
        Cell::replace_batch(&updates.into_values().collect::<Vec<_>>())?;
        self.stack.push(result);
        Ok(())
    }
    fn argument_updates(
        &self,
        frame: &Frame,
        f: &PreparedFunction,
    ) -> Result<BTreeMap<u64, (Rc<Cell>, Value)>, VmError> {
        if f.args.len() != f.max_args || frame.bp < frame.argc || self.stack.len() < frame.bp {
            return Err(err(
                VmErrorKind::InvalidProgram,
                "invalid frame argument storage",
            ));
        }
        Self::argument_plan(f, frame.argc)?;
        let mut updates = BTreeMap::new();
        for (i, arg) in f.args.iter().enumerate().take(frame.argc) {
            let pos = frame.bp - 1 - i;
            let (cell, value) = if arg.flag == 1 {
                let reference = self.reference(pos)?;
                let cell = self.resolve(&reference)?;
                if arg.r#type == 255 {
                    continue;
                }
                let value = self.copy_value(&cell.read()?);
                (cell, value)
            } else {
                let StackEntry::Value(cell) = &self.stack[pos] else {
                    return Err(err(
                        VmErrorKind::UnsupportedFeature,
                        "reference passed to a value parameter",
                    ));
                };
                (
                    cell.clone(),
                    crate::numeric::coerce(self.copy_value(&cell.read()?), arg.r#type),
                )
            };
            if arg.r#type != 255
                && value.tag() != arg.r#type
                && !(arg.attrib & 8 != 0 && value == Value::Undefined)
            {
                return Err(err(
                    VmErrorKind::UnsupportedType,
                    format!("argument {i} type does not match prototype"),
                ));
            }
            updates.insert(cell.id(), (cell, value));
        }
        Ok(updates)
    }
    fn check_args(&mut self, frame: &Frame, f: &PreparedFunction) -> Result<(), VmError> {
        let updates = self.argument_updates(frame, f)?;
        Cell::replace_batch(&updates.into_values().collect::<Vec<_>>())
    }
    fn call_other(
        &mut self,
        frame: &Frame,
        local_count: usize,
        flags: i32,
        argc: i32,
    ) -> Result<(), VmError> {
        // vm_callInOtherFun @0x08094cd8, vm_callInOtherFunEx @0x080bd0fc.
        // Only the scalar, non-efun path is enabled; its return callback is a no-op.
        if self.initialization.is_some() || self.construction.is_some() {
            return Err(err(
                VmErrorKind::UnsupportedFeature,
                "cross-object lifecycle calls are unsupported",
            ));
        }
        if !matches!(flags, 0 | 2 | 4 | 6) || argc < 0 {
            return Err(err(
                VmErrorKind::UnsupportedFeature,
                "unsupported cross-object flags or argc",
            ));
        }
        let argc = argc as usize;
        self.need(argc + 2, frame, local_count)?;
        let n = self.stack.len();
        let object = match self.value(n - 1)? {
            Value::Object(object) => object,
            Value::Bytes(_) | Value::Array(_) => {
                return Err(err(
                    VmErrorKind::UnsupportedFeature,
                    "string and array call targets are unsupported",
                ));
            }
            _ => {
                return Err(err(
                    VmErrorKind::UnsupportedType,
                    "call target requires an Object",
                ));
            }
        };
        let target_object = self.objects.get(object)?;
        if target_object.initialization_state() == InitializationState::Initializing
            || matches!(
                target_object.lifecycle_state(),
                LifecycleState::Constructing | LifecycleState::RollingBack
            )
        {
            return Err(err(
                VmErrorKind::InvalidState,
                "call target initialization is incomplete",
            ));
        }
        let program = target_object.program();
        let p = self.objects.program(program)?;
        let Value::Bytes(method) = self.value(n - 2)? else {
            return Err(err(
                VmErrorKind::UnsupportedType,
                "method name requires a String",
            ));
        };
        if method.contains(&0) {
            return Err(err(
                VmErrorKind::UnsupportedFeature,
                "method names containing NUL are unsupported",
            ));
        }
        let methods = p.methods.as_ref().ok_or_else(|| {
            err(
                VmErrorKind::UnsupportedFeature,
                "call target has no explicit complete method namespace",
            )
        })?;
        let lookup = |name: &[u8]| {
            methods
                .get(name)
                .copied()
                .filter(|slot| flags & 2 != 0 || !p.functions[slot].private)
        };
        let (index, removed, argc) = if let Some(slot) = lookup(&method) {
            (slot, 2, argc)
        } else if let Some(slot) = lookup(b"default_interface") {
            // Pop only target: the original method cell becomes the new arg0.
            (slot, 1, argc + 1)
        } else if flags & 4 != 0 {
            let result = StackEntry::Value(Cell::new(Value::Void)?);
            self.stack.truncate(n - argc - 2);
            return self.push_entry(result);
        } else {
            return Err(err(
                VmErrorKind::InvalidProgram,
                "method and default_interface not found",
            ));
        };
        if self.frames.len() >= self.limits.max_frames {
            return Err(err(VmErrorKind::Limit, "call frame limit exceeded"));
        }
        let target = &p.functions[&index];
        Self::check_user_entry(target)?;
        Self::argument_plan(target, argc)?;
        let bp = n - removed;
        for i in 0..argc {
            let flag = target.args.get(i).map_or(0, |a| a.flag);
            match (&self.stack[bp - 1 - i], flag) {
                (StackEntry::Reference(r), 1) => self.validate_binding(r)?,
                (StackEntry::Value(_), 0) => {}
                _ => {
                    return Err(err(
                        VmErrorKind::UnsupportedFeature,
                        "argument binding does not match value/reference parameter",
                    ));
                }
            }
        }
        let next = Frame {
            id: self.next_frame,
            execution: ExecutionSource::ObjectProgram(program),
            active_object: Some(object),
            body: FrameBody::Function(index),
            pc: 0,
            bp,
            argc,
            entered: false,
            phase: EntryPhase::Unchecked,
        };
        // Preflight types without consuming operands or normalizing caller-owned slots.
        // The ordinary 0f entry performs the batch commit after the frame is installed.
        self.argument_updates(&next, target)?;
        self.frame_id()?;
        self.stack.truncate(bp);
        self.frames.push(next);
        Ok(())
    }
    fn call_user(
        &mut self,
        frame: &Frame,
        current: &PreparedProgram,
        local_count: usize,
        base: i16,
        slot: i32,
        flags: i32,
    ) -> Result<(), VmError> {
        let global = flags & 0x80 != 0;
        if global
            && (current.global_schema.is_none()
                || self.initialization.is_some()
                || self.construction.is_some())
        {
            return Err(err(
                VmErrorKind::UnsupportedFeature,
                "global calls require ordinary opt-in execution",
            ));
        }
        if !global && matches!(frame.execution, ExecutionSource::GlobalProgram(_)) {
            return Err(err(
                VmErrorKind::UnsupportedFeature,
                "non-global07 requires Object code",
            ));
        }
        let argc = (flags & 0x3f) as usize;
        self.need(argc, frame, local_count)?;
        if self.frames.len() >= self.limits.max_frames {
            return Err(err(VmErrorKind::Limit, "call frame limit exceeded"));
        }
        let program = if global {
            self.objects.global_program()?
        } else {
            frame.program()
        };
        let p = self.objects.program(program)?;
        let index = if global {
            if slot < 0 || slot as usize >= p.function_capacity {
                return Err(err(
                    VmErrorKind::Bounds,
                    format!("global function slot {slot} outside capacity"),
                ));
            }
            slot as i16
        } else {
            i16::try_from(slot + i32::from(base))
                .map_err(|_| err(VmErrorKind::Bounds, "relocated function index overflow"))?
        };
        if p.unimplemented.contains(&index) || global && !p.functions.contains_key(&index) {
            return Err(err(
                VmErrorKind::UnsupportedFeature,
                format!("function slot {index} is unimplemented"),
            ));
        }
        let target = p.functions.get(&index).ok_or_else(|| {
            err(
                VmErrorKind::InvalidProgram,
                "local function index does not exist",
            )
        })?;
        Self::check_user_entry(target)?;
        Self::argument_plan(target, argc)?;
        let bp = self.stack.len();
        for i in 0..argc {
            let flag = target.args.get(i).map_or(0, |arg| arg.flag);
            match (&self.stack[bp - 1 - i], flag) {
                (StackEntry::Reference(r), 1) => self.validate_binding(r)?,
                (StackEntry::Value(_), 0) => {}
                _ => {
                    return Err(err(
                        VmErrorKind::UnsupportedFeature,
                        "argument binding does not match value/reference parameter",
                    ));
                }
            }
        }
        let next = Frame {
            id: self.next_frame,
            execution: if global {
                ExecutionSource::GlobalProgram(program)
            } else {
                frame.execution
            },
            active_object: if global { None } else { frame.active_object },
            body: FrameBody::Function(index),
            pc: 0,
            bp,
            argc,
            entered: false,
            phase: EntryPhase::Unchecked,
        };
        // As with 08, preflight types before installing a Global frame. 0f commits later.
        if global {
            self.argument_updates(&next, target)?;
        }
        self.frame_id()?;
        self.frames.push(next);
        Ok(())
    }
    fn check_function_context(&self, program: &PreparedProgram) -> Result<(), VmError> {
        if !program.allow_function_values
            || self.initialization.is_some()
            || self.construction.is_some()
        {
            return Err(err(
                VmErrorKind::UnsupportedFeature,
                "function values require ordinary opt-in execution",
            ));
        }
        Ok(())
    }
    /// Inspect raw storage: Cell::read alone would hide native loop-control payloads.
    fn ordinary_value(&self, pos: usize) -> Result<Value, VmError> {
        match &self.stack[pos] {
            StackEntry::Value(cell) if !cell.is_loop()? => cell.read(),
            _ => Err(err(
                VmErrorKind::UnsupportedFeature,
                "function values require ordinary value operands",
            )),
        }
    }
    fn function_target(program: &PreparedProgram, slot: i32) -> Result<&PreparedFunction, VmError> {
        if slot < 0 || slot as usize >= program.function_capacity {
            return Err(err(
                VmErrorKind::Bounds,
                "function value slot outside capacity",
            ));
        }
        let target = program.functions.get(&(slot as i16)).ok_or_else(|| {
            err(
                VmErrorKind::UnsupportedFeature,
                "function value body is unavailable",
            )
        })?;
        if target.args.iter().any(|a| a.flag != 0) {
            return Err(err(
                VmErrorKind::UnsupportedFeature,
                "function value reference parameters are unsupported",
            ));
        }
        Self::check_user_entry(target)?;
        Ok(target)
    }
    fn create_function_value(
        &mut self,
        frame: &Frame,
        program: &PreparedProgram,
        code: &CodeView<'_>,
        slot: i32,
        flags: i32,
    ) -> Result<(), VmError> {
        self.check_function_context(program)?;
        if program.kind != ProgramKind::Object || !(0..64).contains(&flags) {
            return Err(err(
                VmErrorKind::UnsupportedFeature,
                "unsupported function value creation flags or code kind",
            ));
        }
        let owner = frame.object()?;
        if self.objects.get(owner)?.program() != frame.program() {
            return Err(err(
                VmErrorKind::InvalidState,
                "function owner code identity mismatch",
            ));
        }
        Self::function_target(program, slot)?;
        let count = flags as usize;
        self.need(count, frame, code.local_count)?;
        let n = self.stack.len();
        let result_len = (n - count)
            .checked_add(1)
            .filter(|len| *len <= self.limits.max_stack)
            .ok_or_else(|| {
                err(
                    VmErrorKind::Limit,
                    "function value result exceeds stack limit",
                )
            })?;
        let mut bound = Vec::new();
        bound
            .try_reserve(count)
            .map_err(|_| err(VmErrorKind::Limit, "capture allocation failed"))?;
        for pos in (n - count..n).rev() {
            let value = self.ordinary_value(pos)?;
            if !matches!(
                value,
                Value::Undefined | Value::Void | Value::Int(_) | Value::Real(_) | Value::Bytes(_)
            ) {
                return Err(err(
                    VmErrorKind::UnsupportedType,
                    "only scalar function captures are supported",
                ));
            }
            bound.push(value);
        }
        self.stack
            .try_reserve(result_len.saturating_sub(n))
            .map_err(|_| {
                err(
                    VmErrorKind::Limit,
                    "function value result allocation failed",
                )
            })?;
        let cell = Cell::new(Value::Undefined)?;
        let function = crate::FunctionValue(Rc::new(crate::value::FunctionData {
            origin: self.objects.function_origin.clone(),
            owner,
            program: frame.program(),
            function: slot as i16,
            bound,
        }));
        cell.replace(Value::Function(function))?;
        self.stack.truncate(n - count);
        self.stack.push(StackEntry::Value(cell));
        self.frames.last_mut().unwrap().pc += 1;
        Ok(())
    }
    fn call_function_value(
        &mut self,
        frame: &Frame,
        program: &PreparedProgram,
        code: &CodeView<'_>,
        flags: i32,
        argn: i32,
    ) -> Result<(), VmError> {
        self.check_function_context(program)?;
        if !matches!(flags, 0 | 4) || !(1..128).contains(&argn) {
            return Err(err(
                VmErrorKind::Bounds,
                "indirect call flags or argument count outside supported range",
            ));
        }
        let argn = argn as usize;
        self.need(argn, frame, code.local_count)?;
        let n = self.stack.len();
        let Value::Function(function) = self.ordinary_value(n - 1)? else {
            return Err(err(
                VmErrorKind::UnsupportedType,
                "indirect target requires a Function",
            ));
        };
        if !Rc::ptr_eq(&function.0.origin, &self.objects.function_origin) {
            return Err(err(
                VmErrorKind::InvalidState,
                "function value belongs to another ObjectStore",
            ));
        }
        let owner = self.objects.get(function.owner())?;
        if owner.program() != function.program() {
            return Err(err(
                VmErrorKind::InvalidState,
                "saved function code identity mismatch",
            ));
        }
        let target_program = self.objects.program(function.program())?;
        if target_program.kind != ProgramKind::Object {
            return Err(err(
                VmErrorKind::InvalidState,
                "saved function requires Object code",
            ));
        }
        let target = Self::function_target(&target_program, i32::from(function.function()))?;
        // Shape errors precede argument counts/types, without copying or normalizing sources.
        for pos in (n - argn..n - 1).rev() {
            self.ordinary_value(pos)?;
        }
        let argc = function
            .bound_count()
            .checked_add(argn - 1)
            .filter(|argc| *argc <= 127)
            .ok_or_else(|| err(VmErrorKind::Limit, "indirect call exceeds 127 arguments"))?;
        let (_, defaults) = Self::argument_plan(target, argc)?;
        let prefix = n - argn;
        let bp = prefix
            .checked_add(argc)
            .ok_or_else(|| err(VmErrorKind::Limit, "indirect argument stack overflow"))?;
        let mut arguments = Vec::new();
        arguments
            .try_reserve(argc)
            .map_err(|_| err(VmErrorKind::Limit, "indirect argument allocation failed"))?;
        for i in 0..argc {
            let raw = if i < function.bound_count() {
                function.0.bound[i].clone()
            } else {
                self.ordinary_value(n - 2 - (i - function.bound_count()))?
            };
            let mut value = self.copy_value(&raw);
            if let Some(arg) = target.args.get(i) {
                value = crate::numeric::coerce(value, arg.r#type);
                if arg.r#type != 255
                    && value.tag() != arg.r#type
                    && !(arg.attrib & 8 != 0 && value == Value::Undefined)
                {
                    return Err(err(
                        VmErrorKind::UnsupportedType,
                        format!("argument {i} type does not match prototype"),
                    ));
                }
            }
            crate::host::validate_value(&value)?;
            arguments.push(value);
        }
        if bp
            .checked_add(defaults)
            .and_then(|len| len.checked_add(target.local_count))
            .is_none_or(|len| len > self.limits.max_stack)
            || self.frames.len() >= self.limits.max_frames
        {
            return Err(err(
                VmErrorKind::Limit,
                "indirect call exceeds stack or frame limit",
            ));
        }
        let next_id = self
            .next_frame
            .checked_add(1)
            .ok_or_else(|| err(VmErrorKind::Limit, "frame ID space exhausted"))?;
        let mut cells = Vec::new();
        cells
            .try_reserve(argc)
            .map_err(|_| err(VmErrorKind::Limit, "indirect cell allocation failed"))?;
        for value in arguments {
            cells.push(StackEntry::Value(Cell::new(value)?));
        }
        self.stack
            .try_reserve(bp.saturating_sub(n))
            .map_err(|_| err(VmErrorKind::Limit, "indirect stack allocation failed"))?;
        self.frames
            .try_reserve(1)
            .map_err(|_| err(VmErrorKind::Limit, "indirect frame allocation failed"))?;
        let next = Frame {
            id: self.next_frame,
            execution: ExecutionSource::ObjectProgram(function.program()),
            active_object: Some(function.owner()),
            body: FrameBody::Function(function.function()),
            pc: 0,
            bp,
            argc,
            entered: false,
            phase: EntryPhase::Unchecked,
        };
        // Commit only after every fallible check/allocation. Argument zero is nearest bp.
        self.next_frame = next_id;
        self.stack.truncate(prefix);
        self.stack.extend(cells.into_iter().rev());
        self.frames.last_mut().unwrap().pc += 1;
        self.frames.push(next);
        Ok(())
    }
    fn call_host(
        &mut self,
        frame: &Frame,
        program: &PreparedProgram,
        code: &CodeView<'_>,
        slot: i32,
        argc: i32,
    ) -> Result<(), VmError> {
        use crate::host::{HostActivation, HostCallContext, HostReturn, validate_value};
        if !program.allow_host_calls || self.initialization.is_some() || self.construction.is_some()
        {
            return Err(err(
                VmErrorKind::UnsupportedFeature,
                "host calls require ordinary opt-in execution",
            ));
        }
        if !(0..559).contains(&slot) || !(0..128).contains(&argc) {
            return Err(err(
                VmErrorKind::Bounds,
                "host slot or argc outside supported range",
            ));
        }
        let activation = HostActivation {
            context: HostCallContext {
                slot: slot as u16,
                source: code.source.to_owned(),
                program: frame.program(),
                object: frame.active_object,
                function: code.function()?.index,
                pc: frame.pc,
                offset: code.instructions[frame.pc].offset,
            },
            function: self.objects.host_function(slot as u16)?,
        };
        let invoke = |vm: &mut Self| -> Result<(), VmError> {
            let argc = argc as usize;
            vm.need(argc, frame, code.local_count)?;
            if vm.frames.len() >= vm.limits.max_frames {
                return Err(err(
                    VmErrorKind::Limit,
                    "host activation exceeds frame limit",
                ));
            }
            activation.function.signature.check_count(argc)?;
            let n = vm.stack.len();
            // Inspect raw shape first. Cell::read intentionally hides Loop payloads.
            for entry in &vm.stack[n - argc..] {
                match entry {
                    StackEntry::Value(cell) if !cell.is_loop()? => {}
                    _ => {
                        return Err(err(
                            VmErrorKind::UnsupportedFeature,
                            "host arguments require ordinary values",
                        ));
                    }
                }
            }
            let mut arguments = Vec::new();
            arguments
                .try_reserve(argc)
                .map_err(|_| err(VmErrorKind::Limit, "host argument allocation failed"))?;
            for i in 0..argc {
                let value = vm.value(n - 1 - i)?;
                let expected = activation.function.signature.argument(i);
                if !expected.accepts(&value) {
                    return Err(err(
                        VmErrorKind::UnsupportedType,
                        format!("host argument {i} expected {expected:?}"),
                    ));
                }
                validate_value(&value)?;
                arguments.push(value);
            }
            let result_len = (n - argc)
                .checked_add(1)
                .ok_or_else(|| err(VmErrorKind::Limit, "host result stack overflow"))?;
            if result_len > vm.limits.max_stack {
                return Err(err(VmErrorKind::Limit, "host result exceeds stack limit"));
            }
            let cell = Cell::new(Value::Void)?;
            vm.stack
                .try_reserve(result_len.saturating_sub(n))
                .map_err(|_| err(VmErrorKind::Limit, "host result allocation failed"))?;
            let result = (activation.function.callback)(&activation.context, &arguments)
                .map_err(|e| err(e.kind, e.message))?;
            let value = match (activation.function.signature.returns, result) {
                (HostReturn::Void, None) | (HostReturn::Mixed, None) => Value::Void,
                (HostReturn::Mixed, Some(value)) => value,
                (HostReturn::Exact(expected), Some(value)) if expected.accepts(&value) => value,
                (expected, _) => {
                    return Err(err(
                        VmErrorKind::UnsupportedType,
                        format!("host result expected {expected:?}"),
                    ));
                }
            };
            validate_value(&value)?;
            cell.replace(value)?;
            drop(arguments);
            vm.stack.truncate(n - argc);
            vm.stack.push(StackEntry::Value(cell));
            vm.frames.last_mut().unwrap().pc += 1;
            Ok(())
        };
        invoke(self).map_err(|mut e| {
            e.message = format!(
                "host slot {} ({}): {}",
                slot, activation.function.name, e.message
            )
            .into();
            e
        })
    }
    fn return_function(&mut self, frame: &Frame, f: &PreparedFunction) -> Result<(), VmError> {
        // vm_returnFromFun retires this frame's handlers before return validation.
        self.catches.retain(|c| c.frame_depth < self.frames.len());
        if f.local_count > 0 && !frame.entered {
            return Err(err(
                VmErrorKind::InvalidProgram,
                "local storage was never entered",
            ));
        }
        let count = self
            .stack
            .len()
            .checked_sub(frame.bp + f.local_count)
            .ok_or_else(|| {
                err(
                    VmErrorKind::StackUnderflow,
                    "function frame storage was damaged",
                )
            })?;
        let result = match (f.return_type, count) {
            (1, 0) | (255, 0) => Value::Void,
            (1, _) => {
                return Err(err(
                    VmErrorKind::InvalidProgram,
                    "void function returned a value",
                ));
            }
            (_, 1) => self.copy_value(&self.value(self.stack.len() - 1)?),
            (_, 0) => {
                return Err(err(
                    VmErrorKind::InvalidProgram,
                    "typed function did not return a value",
                ));
            }
            _ => {
                return Err(err(
                    VmErrorKind::InvalidProgram,
                    "function must return at most one value",
                ));
            }
        };
        let result = crate::numeric::coerce(result, f.return_type);
        if f.return_type != 255
            && result.tag() != f.return_type
            && !(f.return_attrib & 8 != 0 && result == Value::Undefined)
        {
            return Err(err(
                VmErrorKind::UnsupportedType,
                "return type does not match prototype",
            ));
        }
        let floor = frame.bp - frame.argc;
        let result_entry = if self.frames.len() > 1 {
            if floor >= self.limits.max_stack {
                return Err(err(VmErrorKind::Limit, "return result exceeds stack limit"));
            }
            Some(StackEntry::Value(Cell::new(result.clone())?))
        } else {
            None
        };
        self.stack.truncate(floor);
        self.frames.pop();
        if let Some(entry) = result_entry {
            self.stack.push(entry);
        } else if self.construction.is_some() {
            self.reference_cells.clear();
            self.advance_construction();
        } else {
            self.completion = Some(Completion::Returned(result));
        }
        Ok(())
    }

    fn array_index(
        array: &crate::container::ArrayValue,
        key: &Value,
        mode: i32,
    ) -> Result<usize, VmError> {
        let i = match key {
            Value::Int(i) => *i,
            Value::Real(r) => crate::numeric::trunc_i32(r.to_f32()),
            _ => {
                return Err(err(
                    VmErrorKind::UnsupportedType,
                    "array index requires a number",
                ));
            }
        };
        let i = if mode == 1 {
            array.len() as i64 - i64::from(i)
        } else {
            i64::from(i)
        };
        if i < 0 || i >= array.len() as i64 {
            return Err(err(VmErrorKind::Bounds, "array index outside container"));
        }
        Ok(i as usize)
    }
    fn numeric_update(
        &mut self,
        opcode: u8,
        operand: i32,
        frame: &Frame,
        local_count: usize,
        fault_site: &mut Option<RuntimeFaultSite>,
    ) -> Result<(), VmError> {
        let count = if opcode == 0x22 { 1 } else { 2 };
        self.need(count, frame, local_count)?;
        let n = self.stack.len();
        let reference = self.reference(n - count)?;
        if reference.readonly {
            return Err(err(
                VmErrorKind::UnsupportedFeature,
                "cannot modify const reference",
            ));
        }
        let cell = self.resolve(&reference)?;
        let left = cell.read()?;
        let result = if opcode == 0x22 {
            crate::numeric::increment(&left, operand)?
        } else {
            let right = self.value(n - 1)?;
            crate::numeric::compound(operand, &left, &right).inspect_err(|_| {
                *fault_site = RuntimeFaultSite::compound(operand, &left, &right);
            })?
        };
        // All computation and result allocation precede the commit. A failure
        // leaves both the target and operand stack intact (IP still advances).
        let entry = if opcode == 0x2f {
            Some(StackEntry::Value(Cell::new(result.clone())?))
        } else {
            None
        };
        self.writable(&reference, &result)?.replace(result)?;
        self.stack.truncate(n - count);
        if let Some(entry) = entry {
            self.push_entry(entry)?;
        }
        Ok(())
    }
    fn control_cell(
        &self,
        frame: &Frame,
        local_count: usize,
        slot: i32,
    ) -> Result<Rc<Cell>, VmError> {
        if !frame.entered || slot >= 0 || i64::from(slot) < -(local_count as i64) {
            return Err(err(
                VmErrorKind::Bounds,
                "loop control requires an allocated local slot",
            ));
        }
        let pos = self
            .slot(slot, frame)
            .ok_or_else(|| err(VmErrorKind::Bounds, "invalid loop slot"))?;
        match &self.stack[pos] {
            StackEntry::Value(c) => Ok(c.clone()),
            _ => Err(err(
                VmErrorKind::UnsupportedFeature,
                "loop control requires a value slot",
            )),
        }
    }
    fn loop_instruction(
        &mut self,
        frame: &Frame,
        code: &CodeView<'_>,
        opcode: u8,
        a: i32,
        b: i32,
    ) -> Result<(), VmError> {
        if self.initialization.is_some() || self.construction.is_some() {
            return Err(err(
                VmErrorKind::UnsupportedFeature,
                "loop lifecycle execution is unsupported",
            ));
        }
        let cell =
            self.control_cell(frame, code.local_count, if opcode == 0x23 { a } else { b })?;
        if opcode == 0x23 {
            self.need(3, frame, code.local_count)?;
            let n = self.stack.len();
            let bound = self.reference(n - 3)?;
            if matches!(bound.target, Target::Owned { .. }) || bound.id == cell.id() {
                return Err(err(
                    VmErrorKind::UnsupportedFeature,
                    "unsupported loop binding or control alias",
                ));
            }
            self.validate_binding(&bound)?;
            let target = self.resolve(&bound)?;
            let source = self.value(n - 2)?;
            let mode = self.value(n - 1)?;
            self.writable(&bound, &Value::Undefined)?;
            let state = match (mode, source) {
                (Value::Int(step @ (1 | -1)), Value::Int(end)) => {
                    let Value::Int(next) = target.read()? else {
                        return Err(err(
                            VmErrorKind::UnsupportedType,
                            "integer loop requires an Int initial value",
                        ));
                    };
                    LoopState::Numeric {
                        next,
                        end,
                        step,
                        finished: false,
                    }
                }
                (Value::Int(997), Value::Array(owner)) => LoopState::Array { owner, index: 0 },
                (Value::Int(997), Value::Mapping(_)) => {
                    return Err(err(
                        VmErrorKind::UnsupportedFeature,
                        "mapping loop order is unsupported",
                    ));
                }
                (Value::Int(1 | -1 | 997), _) => {
                    return Err(err(
                        VmErrorKind::UnsupportedType,
                        "invalid loop source/end type",
                    ));
                }
                _ => {
                    return Err(err(
                        VmErrorKind::UnsupportedFeature,
                        "loop mode requires +/-1 or Array mode997",
                    ));
                }
            };
            Cell::replace_payload_batch(vec![(
                cell,
                SlotPayload::Loop(Box::new(LoopControl { bound, state })),
            )])?;
            self.stack.truncate(n - 3);
            return Ok(());
        }
        let mut control = cell.loop_control()?;
        self.validate_binding(&control.bound)?;
        let mut output = None;
        let mut source_update = None;
        let valid = match &mut control.state {
            LoopState::Numeric {
                next,
                end,
                step,
                finished,
            } => {
                let value = *next;
                *next = next.wrapping_add(*step);
                let valid = !*finished
                    && if *step > 0 {
                        value <= *end
                    } else {
                        value >= *end
                    };
                if value == *end {
                    *finished = true;
                }
                output = Some(Value::Int(value));
                valid
            }
            LoopState::Array { owner, index } => {
                let i = *index;
                *index = index.wrapping_add(1);
                if (i as usize) < owner.len() {
                    let source = owner.cell(i as usize)?;
                    let original = source.read()?;
                    let value = self.copy_value(&original);
                    if original != value {
                        source_update = Some((source, SlotPayload::Value(value.clone())));
                    }
                    output = Some(value);
                    true
                } else {
                    false
                }
            }
        };
        let delta = if (opcode == 0x12 && !valid) || (opcode == 0x50 && valid) {
            a
        } else {
            0
        };
        let pc = self.frames.last().unwrap().pc as i64 + i64::from(delta);
        if pc < 0 || pc >= code.instructions.len() as i64 {
            return Err(err(VmErrorKind::Bounds, "loop branch outside code"));
        }
        // Recheck owner and readonly even on exhaustion, without retaining guards.
        let target = self.writable(&control.bound, output.as_ref().unwrap_or(&Value::Undefined))?;
        let mut updates = vec![(cell, SlotPayload::Loop(Box::new(control)))];
        if let Some(value) = output {
            updates.push((target, SlotPayload::Value(value)));
        }
        if let Some(update) = source_update {
            updates.push(update);
        }
        Cell::replace_payload_batch(updates)?;
        self.frames.last_mut().unwrap().pc = pc as usize;
        Ok(())
    }
    // vm_checkIsRefType @0x08091750 inspects the raw tag before pushing.
    // In particular, a dangling reference still has tag 8; do not resolve it.
    fn check_reference_type(&mut self, frame: &Frame, index: i32) -> Result<(), VmError> {
        if self.initialization.is_some() || self.construction.is_some() {
            return Err(err(
                VmErrorKind::UnsupportedFeature,
                "reference type check during lifecycle task",
            ));
        }
        if self.stack.len() >= self.limits.max_stack {
            return Err(err(VmErrorKind::Limit, "value stack limit exceeded"));
        }
        let is_reference = self
            .slot(index, frame)
            .is_some_and(|pos| matches!(self.stack[pos], StackEntry::Reference(_)));
        self.push(Value::Int(i32::from(is_reference)))
    }
    // vm_queryRange @0x0808fd04: inclusive endpoints, bit0=end-relative,
    // bit1=begin-relative. Wide temporaries avoid native signed overflow.
    fn range_bounds(
        len: usize,
        begin: &Value,
        end: &Value,
        mode: i32,
    ) -> Result<std::ops::Range<usize>, VmError> {
        let endpoint = |v: &Value| match v {
            Value::Int(i) => Ok(i64::from(*i)),
            Value::Real(r) => Ok(i64::from(crate::numeric::trunc_i32(r.to_f32()))),
            _ => Err(err(
                VmErrorKind::UnsupportedType,
                "range endpoints require numbers",
            )),
        };
        let mut begin = endpoint(begin)?;
        let mut end = endpoint(end)?;
        let len = i32::try_from(len).map_err(|_| {
            err(
                VmErrorKind::Limit,
                "range source exceeds legacy integer length",
            )
        })?;
        let len = i64::from(len);
        if mode & 1 != 0 {
            end = len - end.max(1);
        }
        if mode & 2 != 0 {
            begin = len - begin.max(1);
        }
        begin = begin.max(0);
        end = end.min(len - 1);
        if begin > end {
            Ok(0..0)
        } else {
            Ok(begin as usize..end as usize + 1)
        }
    }
    fn query_range(&mut self, frame: &Frame, locals: usize, mode: i32) -> Result<(), VmError> {
        if self.initialization.is_some() || self.construction.is_some() {
            return Err(err(
                VmErrorKind::UnsupportedFeature,
                "range query during lifecycle task",
            ));
        }
        self.need(3, frame, locals)?;
        let n = self.stack.len();
        let source = self.value(n - 3)?;
        let begin = self.value(n - 2)?;
        let end = self.value(n - 1)?;
        let mut updates = std::collections::BTreeMap::new();
        let result = match source {
            Value::Bytes(bytes) => {
                let range = Self::range_bounds(bytes.len(), &begin, &end, mode)?;
                if range.len() > 65536 {
                    return Err(err(
                        VmErrorKind::Limit,
                        "range byte result exceeds 65536 bytes",
                    ));
                }
                let mut result = bytes[range].to_vec();
                // Native String uses strncpy, not Buffer's memcpy. Preserve length.
                if let Some(nul) = result.iter().position(|&b| b == 0) {
                    result[nul..].fill(0);
                }
                Value::Bytes(Rc::from(result))
            }
            Value::Array(array) => {
                let range = Self::range_bounds(array.len(), &begin, &end, mode)?;
                let mut values = Vec::with_capacity(range.len());
                for index in range {
                    let cell = array.cell(index)?;
                    let original = cell.read()?;
                    let copied = self.copy_value(&original);
                    if copied != original {
                        updates.insert(cell.id(), (cell, copied.clone()));
                    }
                    values.push(copied);
                }
                Value::array(values)?
            }
            _ => {
                return Err(err(
                    VmErrorKind::UnsupportedType,
                    "range query requires bytes or array",
                ));
            }
        };
        let StackEntry::Value(cell) = &self.stack[n - 3] else {
            unreachable!()
        };
        updates.insert(cell.id(), (cell.clone(), result));
        // All fallible reads/allocations precede the atomic commit. Source dead-object
        // normalization is visible through aliases, as in native vm_copyValue.
        Cell::replace_batch(&updates.into_values().collect::<Vec<_>>())?;
        self.stack.truncate(n - 2);
        Ok(())
    }
    fn check_script_context(&self) -> Result<(), VmError> {
        if self.initialization.is_some() || self.construction.is_some() {
            return Err(err(
                VmErrorKind::UnsupportedFeature,
                "catch during lifecycle task",
            ));
        }
        Ok(())
    }
    fn register_script_catch(
        &mut self,
        frame: &Frame,
        code: &CodeView<'_>,
        offset: i32,
    ) -> Result<(), VmError> {
        self.check_script_context()?;
        code.function()?;
        if !frame.entered
            || frame
                .bp
                .checked_add(code.local_count)
                .is_none_or(|floor| floor > self.stack.len())
        {
            return Err(err(
                VmErrorKind::InvalidProgram,
                "catch requires an entered function frame",
            ));
        }
        let target_pc = frame
            .pc
            .checked_add(1)
            .and_then(|pc| pc.checked_add_signed(offset as isize))
            .filter(|&pc| pc < code.instructions.len())
            .ok_or_else(|| err(VmErrorKind::InvalidProgram, "catch target outside function"))?;
        if code.entry.is_some_and(|entry| target_pc <= entry.enter) {
            return Err(err(
                VmErrorKind::InvalidProgram,
                "catch targets function initialization",
            ));
        }
        if self.catches.len() >= self.limits.max_stack {
            return Err(err(VmErrorKind::Limit, "catch context limit exceeded"));
        }
        self.catches.push(ScriptCatch {
            frame_id: frame.id,
            frame_depth: self.frames.len(),
            stack_len: self.stack.len(),
            target_pc,
        });
        Ok(())
    }
    fn end_script_catch(&mut self, frame: &Frame) -> Result<(), VmError> {
        self.check_script_context()?;
        if self
            .catches
            .last()
            .is_none_or(|c| c.frame_id != frame.id || c.frame_depth != self.frames.len())
        {
            return Err(err(
                VmErrorKind::InvalidProgram,
                "catch end has no handler owned by this frame",
            ));
        }
        self.catches.pop();
        Ok(())
    }
    fn route_script_fault(&mut self) -> Result<bool, VmError> {
        // No task error is ever turned into an ordinary script recovery.
        if self.initialization.is_some() || self.construction.is_some() {
            return Ok(false);
        }
        let Some(handler) = self.catches.last() else {
            return Ok(false);
        };
        let invalid = || {
            err(
                VmErrorKind::InvalidProgram,
                "catch recovery context is invalid",
            )
        };
        let owner_index = handler.frame_depth.checked_sub(1).ok_or_else(invalid)?;
        let owner = self.frames.get(owner_index).ok_or_else(invalid)?;
        if owner.id != handler.frame_id || !owner.entered {
            return Err(invalid());
        }
        if let Some(object) = owner.active_object {
            self.objects.get(object)?;
        }
        let program = self.objects.program(owner.program())?;
        let code = CodeView::get(&program, owner.body)?;
        code.function()?;
        let floor = owner.bp.checked_add(code.local_count).ok_or_else(invalid)?;
        if handler.stack_len < floor
            || handler.stack_len > self.stack.len()
            || handler.target_pc >= code.instructions.len()
            || code
                .entry
                .is_some_and(|entry| handler.target_pc <= entry.enter)
        {
            return Err(invalid());
        }
        let child_floor = self
            .frames
            .get(handler.frame_depth)
            .map(|child| child.bp.checked_sub(child.argc).ok_or_else(invalid))
            .transpose()?;
        if child_floor.is_some_and(|floor| floor < handler.stack_len || floor > self.stack.len()) {
            return Err(invalid());
        }
        let (depth, saved_len, target) =
            (handler.frame_depth, handler.stack_len, handler.target_pc);
        // All checks precede destruction. No borrow guard, callback, allocation,
        // or budget yield may occur between these two native-corresponding phases.
        if let Some(floor) = child_floor {
            self.stack.truncate(floor);
            self.frames.truncate(depth);
        }
        self.catches.pop();
        self.stack.truncate(saved_len);
        self.frames[owner_index].pc = target;
        self.reference_cells
            .retain(|_, lease| lease.strong_count() != 0);
        Ok(true)
    }
    fn step(&mut self) -> Result<(), StepFault> {
        // Provenance lives only for this instruction, never in sticky VM state.
        let mut site = None;
        self.step_instruction(&mut site)
            .map_err(|diagnostic| StepFault {
                diagnostic: Box::new(diagnostic),
                site,
            })
    }
    fn step_instruction(
        &mut self,
        fault_site: &mut Option<RuntimeFaultSite>,
    ) -> Result<(), VmError> {
        let (frame, p) = self.context()?;
        let f = CodeView::get(&p, frame.body)?;
        let Instruction {
            opcode, operands, ..
        } = f
            .instructions
            .get(frame.pc)
            .ok_or_else(|| err(VmErrorKind::Bounds, "program counter outside function code"))?
            .clone();
        if let Some(entry) = &f.entry {
            let valid = match frame.phase {
                EntryPhase::Unchecked => frame.pc == 0,
                EntryPhase::Defaults { .. } => {
                    frame.pc > 0 && entry.resort.is_some_and(|pc| frame.pc <= pc)
                }
                EntryPhase::Ready => frame.pc == entry.enter,
                EntryPhase::Body => frame.pc > entry.enter,
            };
            if !valid {
                return Err(err(
                    VmErrorKind::InvalidProgram,
                    "invalid function initialization state",
                ));
            }
        }
        // Synthetic 06 commits PC only after a complete synchronous call.
        // Other instructions retain the native pre-dispatch increment.
        if opcode == 0x06 {
            return self.call_host(&frame, &p, &f, operands[0], operands[1]);
        }
        if opcode == 0x17 {
            return self.create_function_value(&frame, &p, &f, operands[0], operands[1]);
        }
        if opcode == 0x05 {
            return self.call_function_value(&frame, &p, &f, operands[0], operands[1]);
        }
        // Bytes preflight and commit preserve the original PC on recoverable failure.
        if matches!(opcode, 1 | 0x1c | 0x2a | 0x1e | 0x20 | 0x25 | 0x27) {
            let n = self.stack.len();
            let has_bytes = self.stack[n.saturating_sub(2)..].iter().any(|entry| {
                matches!(entry, StackEntry::Value(c) if matches!(c.read(), Ok(Value::Bytes(_))))
            });
            if has_bytes {
                self.need(2, &frame, f.local_count)?;
                for entry in &self.stack[n - 2..] {
                    match entry {
                        StackEntry::Value(c) if !c.is_loop()? => (),
                        _ => {
                            return Err(err(
                                VmErrorKind::UnsupportedFeature,
                                "Bytes operands require ordinary values",
                            ));
                        }
                    }
                }
                let left = self.value(n - 2)?;
                let right = self.value(n - 1)?;
                if crate::bytes::selected(opcode, &left, &right) {
                    if self.initialization.is_some() || self.construction.is_some() {
                        return Err(err(
                            VmErrorKind::UnsupportedFeature,
                            "Bytes operations during lifecycle tasks are unsupported",
                        ));
                    }
                    let result = crate::bytes::binary(opcode, &left, &right)?;
                    self.set(n - 2, result)?;
                    self.stack.pop();
                    self.frames.last_mut().unwrap().pc += 1;
                    return Ok(());
                }
            }
        }
        // vm_regularProcessor @0x080978c4 increments IP before dispatch.
        self.frames.last_mut().unwrap().pc += 1;
        let [a, b] = operands;
        let n = self.stack.len();
        match opcode {
            0x10 => self.check_reference_type(&frame, a),
            0x45 => self.query_range(&frame, f.local_count, a),
            0x2c => self.register_script_catch(&frame, &f, a),
            0x1a => self.end_script_catch(&frame),
            0x0e => {
                self.check_script_context()?;
                self.push(Value::Int(1))
            }
            0x12 | 0x23 | 0x50 => self.loop_instruction(&frame, &f, opcode, a, b),
            0x08 => self.call_other(&frame, f.local_count, a, b),
            // vm_caseCheckNumber/String/Default @0x080974c0/4fc/584.
            0x0a..=0x0c => {
                self.need(1, &frame, f.local_count)?;
                // Native 0b checks the body-specific pool before testing the selector tag.
                let literal = if opcode == 0x0b {
                    Some(
                        f.pool
                            .unwrap_or(&p.strings)
                            .get(b as usize)
                            .and_then(Option::as_ref)
                            .ok_or_else(|| {
                                err(VmErrorKind::InvalidProgram, "case string is unavailable")
                            })?,
                    )
                } else {
                    None
                };
                let selector = self.value(n - 1)?; // Reference selectors remain unsupported.
                let matched = match opcode {
                    0x0a => matches!(selector, Value::Int(value) if value == b),
                    0x0b => {
                        matches!((&selector, literal), (Value::Bytes(value), Some(Value::Bytes(expected))) if value == expected)
                    }
                    _ => true,
                };
                if matched {
                    self.jump(a, f.instructions.len())?;
                    self.stack.pop();
                }
                Ok(())
            }
            // vm_pushThisObject @0x080975d8. All currently accepted frames own an Object.
            0x41 => self.push(
                frame
                    .active_object
                    .map(Value::Object)
                    .unwrap_or(Value::Int(0)),
            ),
            0x22 | 0x2f => self.numeric_update(opcode, a, &frame, f.local_count, fault_site),
            0x03 | 0x04 => {
                self.need(2, &frame, f.local_count)?;
                let (target, source) = if opcode == 3 {
                    (n - 1, n - 2)
                } else {
                    (n - 2, n - 1)
                };
                let reference = self.reference(target)?;
                let value = self.copy_value(&self.value(source)?);
                let cell = self.writable(&reference, &value)?;
                // Allocate the result before committing the write.
                let result = StackEntry::Value(Cell::new(value.clone())?);
                cell.replace(value)?;
                self.stack.truncate(n - 2);
                self.push_entry(result)
            }
            0x39 => {
                let pos = self
                    .slot(a, &frame)
                    .ok_or_else(|| err(VmErrorKind::Bounds, "no such local/argument slot"))?;
                let r = match &self.stack[pos] {
                    StackEntry::Reference(r) => {
                        if pos >= frame.bp {
                            return Err(err(
                                VmErrorKind::UnsupportedFeature,
                                "only reference parameters may be forwarded",
                            ));
                        }
                        self.resolve(r)?;
                        r.clone()
                    }
                    StackEntry::Value(c) => Reference {
                        id: c.id(),
                        target: Target::Local {
                            frame: frame.id,
                            cell: Rc::downgrade(c),
                        },
                        readonly: false,
                        _lease: None,
                    },
                };
                self.push_entry(StackEntry::Reference(r))
            }
            0x3e => {
                let cell = self.object_cell(&frame, a)?;
                let r = Reference {
                    id: cell.id(),
                    target: Target::Object {
                        object: frame.object()?,
                        cell: Rc::downgrade(&cell),
                    },
                    readonly: false,
                    _lease: None,
                };
                self.push_entry(StackEntry::Reference(r))
            }
            0x13 => {
                self.need(1, &frame, f.local_count)?;
                if matches!(self.stack[n - 1], StackEntry::Reference(_)) {
                    return Ok(());
                }
                let value = self.value(n - 1)?;
                if !matches!(value, Value::Array(_) | Value::Mapping(_)) {
                    return Err(err(
                        VmErrorKind::UnsupportedType,
                        "pointer conversion requires array or mapping",
                    ));
                }
                self.reserve_reference(None)?;
                let cell = Cell::new(value)?;
                let r = self.owned_reference(cell, ReferenceKind::Temporary)?;
                self.stack[n - 1] = StackEntry::Reference(r);
                Ok(())
            }
            0x14 => {
                self.need(1, &frame, f.local_count)?;
                let r = self.reference(n - 1)?;
                let entry = self.read_reference(&r)?;
                self.stack[n - 1] = entry;
                Ok(())
            }
            0x44 => {
                self.need(2, &frame, f.local_count)?;
                let old = self.reference(n - 2)?;
                if old.readonly {
                    return Err(err(
                        VmErrorKind::UnsupportedFeature,
                        "cannot modify const reference",
                    ));
                }
                let key = self.value(n - 1)?;
                // This scope must release the temporary owner Value/header before
                // the original reference is consumed, matching native lifetimes.
                let (cell, kind) = {
                    let owner = self.resolve(&old)?.read()?;
                    match owner {
                        Value::Array(array) => {
                            let i = Self::array_index(&array, &key, a)?;
                            let cell = array.cell(i)?;
                            self.reserve_reference(Some(cell.id()))?;
                            (cell, ReferenceKind::ArrayMember)
                        }
                        Value::Mapping(mapping) => {
                            let missing = mapping.insertion_needed(&key)?;
                            if missing {
                                self.reserve_reference(None)?;
                            } else {
                                let c = mapping.member(&key)?;
                                self.reserve_reference(Some(c.id()))?;
                            }
                            (mapping.member(&key)?, ReferenceKind::MappingMember)
                        }
                        _ => {
                            return Err(err(
                                VmErrorKind::UnsupportedType,
                                "member reference requires array or mapping",
                            ));
                        }
                    }
                };
                let reference = self.owned_reference(cell, kind)?;
                self.stack.pop();
                self.stack[n - 2] = StackEntry::Reference(reference);
                Ok(())
            }
            0x16 | 0x18 => {
                let count = usize::try_from(a).map_err(|_| {
                    err(
                        VmErrorKind::InvalidProgram,
                        "negative container input count",
                    )
                })?;
                if opcode == 0x18 && count % 2 != 0 {
                    return Err(err(
                        VmErrorKind::InvalidProgram,
                        "mapping input count must be even",
                    ));
                }
                self.need(count, &frame, f.local_count)?;
                if count == 0 && n >= self.limits.max_stack {
                    return Err(err(VmErrorKind::Limit, "value stack limit exceeded"));
                }
                let values = (n - count..n)
                    .rev()
                    .map(|i| self.value(i))
                    .collect::<Result<Vec<_>, _>>()?;
                let value = if opcode == 0x16 {
                    Value::array(values)?
                } else {
                    Value::mapping(
                        values
                            .as_chunks::<2>()
                            .0
                            .iter()
                            .map(|p| (p[0].clone(), p[1].clone()))
                            .collect(),
                    )?
                };
                self.stack.truncate(n - count);
                self.push(value)
            }
            0x43 => {
                self.need(2, &frame, f.local_count)?;
                let key = self.value(n - 1)?;
                let selected = match self.value(n - 2)? {
                    Value::Array(array) => {
                        array.cell(Self::array_index(&array, &key, a)?)?.read()?
                    }
                    Value::Mapping(mapping) => mapping.get(&key)?.unwrap_or(Value::Undefined),
                    _ => {
                        return Err(err(
                            VmErrorKind::UnsupportedType,
                            "member query requires array or mapping",
                        ));
                    }
                };
                let selected = self.copy_value(&selected);
                self.stack.pop();
                self.set(n - 2, selected)
            }
            0x4f => {
                self.need(1, &frame, f.local_count)?;
                let size = match self.value(n - 1)? {
                    Value::Array(a) => a.len(),
                    Value::Mapping(m) => m.len()?,
                    Value::Bytes(b) => b.len(),
                    _ => 0,
                };
                let size = i32::try_from(size)
                    .map_err(|_| err(VmErrorKind::Limit, "size exceeds legacy integer range"))?;
                self.set(n - 1, Value::Int(size))
            }
            0x01 | 0x19 | 0x28 | 0x29 | 0x51 | 0x1c | 0x1e | 0x20 | 0x25 | 0x27 | 0x02 | 0x30
            | 0x55 | 0x26 | 0x4b | 0x2a => {
                self.need(2, &frame, f.local_count)?;
                let left = self.value(n - 2)?;
                let right = self.value(n - 1)?;
                let result = crate::numeric::binary(opcode, &left, &right).inspect_err(|_| {
                    *fault_site = RuntimeFaultSite::binary(opcode, &left, &right);
                })?;
                self.stack.pop();
                self.set(n - 2, result)
            }
            0x2b => {
                self.need(1, &frame, f.local_count)?;
                let result = match self.value(n - 1)? {
                    Value::Int(i) => Value::Int(i.wrapping_neg()),
                    Value::Real(r) => {
                        Value::Real(crate::RealBits::from_bits(r.bits() ^ 0x80000000))
                    }
                    _ => {
                        return Err(err(
                            VmErrorKind::UnsupportedType,
                            "negation requires numbers",
                        ));
                    }
                };
                self.set(n - 1, result)
            }
            0x3b | 0x3c => self.push(Value::Int(a)),
            0x42 => self.push(Value::Undefined),
            0x3f => self.push(Value::Real(crate::RealBits::from_bits(a as u32))),
            0x0d => {
                self.need(1, &frame, f.local_count)?;
                let v = crate::numeric::cast(&self.value(n - 1)?, a)?;
                self.set(n - 1, v)
            }
            0x24 | 0x2d | 0x4a => {
                self.need(1, &frame, f.local_count)?;
                let raw = self.value(n - 1)?;
                let value = if opcode == 0x2d && matches!(raw, Value::Function(_)) {
                    raw
                } else {
                    self.copy_value(&raw)
                };
                let result = match opcode {
                    0x24 => i32::from(i32::from(value.tag()) == a),
                    0x2d => i32::from(crate::numeric::zero(&value)),
                    _ => {
                        if let Value::Int(i) = value {
                            !i
                        } else {
                            return Err(err(
                                VmErrorKind::UnsupportedType,
                                "bitwise NOT requires integer",
                            ));
                        }
                    }
                };
                self.set(n - 1, Value::Int(result))
            }
            0x40 => {
                let value = usize::try_from(a)
                    .ok()
                    .and_then(|n| f.pool.unwrap_or(&p.strings).get(n))
                    .and_then(Option::as_ref)
                    .ok_or_else(|| {
                        err(VmErrorKind::Bounds, "shared string is absent or invalid")
                    })?;
                self.push(value.clone())
            }
            0x32 | 0x33 => {
                let count = if opcode == 0x33 { 1 } else { a.max(0) as usize };
                self.need(count, &frame, f.local_count)?;
                self.stack.truncate(n - count);
                Ok(())
            }
            0x37 => {
                if n >= self.limits.max_stack {
                    return Err(err(VmErrorKind::Limit, "value stack limit exceeded"));
                }
                let entry = if let Some(pos) = self.slot(a, &frame) {
                    match &self.stack[pos] {
                        StackEntry::Reference(r) => {
                            if pos >= frame.bp {
                                return Err(err(
                                    VmErrorKind::UnsupportedFeature,
                                    "only reference parameters may be read as local references",
                                ));
                            }
                            self.read_reference(r)?
                        }
                        StackEntry::Value(c) => {
                            StackEntry::Value(Cell::new(self.copy_value(&c.read()?))?)
                        }
                    }
                } else {
                    StackEntry::Value(Cell::new(Value::Undefined)?)
                };
                self.push_entry(entry)
            }
            0x4d => {
                self.need(1, &frame, f.local_count)?;
                let pos = self
                    .slot(a, &frame)
                    .ok_or_else(|| err(VmErrorKind::Bounds, "no such local/argument slot"))?;
                let value = self.copy_value(&self.value(n - 1)?);
                match &self.stack[pos] {
                    StackEntry::Reference(r) => {
                        if pos >= frame.bp {
                            return Err(err(
                                VmErrorKind::UnsupportedFeature,
                                "only reference parameters may be assigned through local slots",
                            ));
                        }
                        self.writable(r, &value)?.replace(value)
                    }
                    StackEntry::Value(c) => c.replace(value),
                }
            }
            0x35 | 0x36 => {
                if self.initialization.is_some()
                    || self.construction.is_some()
                    || !matches!(frame.body, FrameBody::Function(_))
                {
                    return Err(err(
                        VmErrorKind::UnsupportedFeature,
                        format!("global slot {a} is unavailable during lifecycle execution"),
                    ));
                }
                if self.stack.len() >= self.limits.max_stack {
                    return Err(err(
                        VmErrorKind::Limit,
                        format!("global slot {a}: value stack limit exceeded"),
                    ));
                }
                let schema = p.global_schema.ok_or_else(|| {
                    err(
                        VmErrorKind::UnsupportedFeature,
                        format!("global slot {a} requires explicit schema"),
                    )
                })?;
                let cells = self.objects.global_cells().map_err(|mut e| {
                    e.message = format!("global slot {a}: {}", e.message).into();
                    e
                })?;
                if cells.len() != schema.slots {
                    return Err(err(
                        VmErrorKind::InvalidState,
                        format!("global slot {a}: bank capacity differs from program schema"),
                    ));
                }
                let cell = usize::try_from(a)
                    .ok()
                    .and_then(|i| cells.get(i))
                    .cloned()
                    .ok_or_else(|| {
                        err(VmErrorKind::Bounds, format!("global slot {a} outside bank"))
                    })?;
                let reference = Reference {
                    id: cell.id(),
                    target: Target::Global { cell },
                    readonly: false,
                    _lease: None,
                };
                let entry = if opcode == 0x35 {
                    self.read_reference(&reference).map_err(|mut e| {
                        e.message = format!("global slot {a}: {}", e.message).into();
                        e
                    })?
                } else {
                    StackEntry::Reference(reference)
                };
                self.stack.push(entry);
                Ok(())
            }
            0x3d => {
                let value = self.copy_value(&self.object_cell(&frame, a)?.read()?);
                self.push(value)
            }
            0x4e => {
                self.need(1, &frame, f.local_count)?;
                let cell = self.object_cell(&frame, a)?;
                let value = self.copy_value(&self.value(n - 1)?);
                cell.replace(value)
            }
            0x1f => self.jump(a, f.instructions.len()),
            0x21 => {
                self.need(1, &frame, f.local_count)?;
                let raw = self.value(n - 1)?;
                let value = if matches!(raw, Value::Function(_)) {
                    raw
                } else {
                    self.copy_value(&raw)
                };
                let zero = crate::numeric::zero(&value);
                self.set(n - 1, value)?;
                if zero != (b & 1 != 0) {
                    self.jump(a, f.instructions.len())?;
                }
                if b & 2 != 0 {
                    self.stack.pop();
                }
                Ok(())
            }
            0x34 => self.push(Value::Int(i32::try_from(frame.argc).map_err(|_| {
                err(VmErrorKind::Limit, "argument count exceeds integer range")
            })?)),
            0x15 => self.argument_array(&frame),
            0x47 => {
                let entry = f.entry.as_ref().ok_or_else(|| {
                    err(VmErrorKind::InvalidProgram, "resort outside default entry")
                })?;
                let EntryPhase::Defaults { expected } = frame.phase else {
                    return Err(err(
                        VmErrorKind::InvalidProgram,
                        "duplicate or unprepared resort",
                    ));
                };
                if frame.entered
                    || entry.resort != Some(frame.pc)
                    || n.checked_sub(frame.bp) != Some(expected)
                    || self.stack[frame.bp..]
                        .iter()
                        .any(|v| matches!(v, StackEntry::Reference(_)))
                {
                    return Err(err(VmErrorKind::InvalidProgram, "invalid default results"));
                }
                let floor = frame.bp - frame.argc;
                // Reverse default evaluation order, then place it below old args.
                self.stack[frame.bp..].reverse();
                self.stack[floor..].rotate_right(expected);
                let current = self.frames.last_mut().unwrap();
                current.argc += expected;
                current.bp = n;
                current.phase = EntryPhase::Ready;
                Ok(())
            }
            0x0f => {
                let (skip, expected) = Self::argument_plan(f.function()?, frame.argc)?;
                let target = if let Some(entry) = &f.entry {
                    if frame.phase != EntryPhase::Unchecked || n != frame.bp {
                        return Err(err(
                            VmErrorKind::InvalidProgram,
                            "duplicate or damaged argument entry",
                        ));
                    }
                    if frame.argc < f.function()?.max_args {
                        1 + skip
                    } else {
                        entry.enter
                    }
                } else {
                    usize::try_from(frame.pc as i64 + 1 + i64::from(a)).map_err(|_| {
                        err(VmErrorKind::Bounds, "branch target outside function code")
                    })?
                };
                if target >= f.instructions.len() {
                    return Err(err(
                        VmErrorKind::Bounds,
                        "branch target outside function code",
                    ));
                }
                self.check_args(&frame, f.function()?)?;
                let current = self.frames.last_mut().unwrap();
                current.pc = target;
                if let Some(entry) = &f.entry {
                    current.phase = if target == entry.enter {
                        EntryPhase::Ready
                    } else {
                        EntryPhase::Defaults { expected }
                    };
                }
                Ok(())
            }
            0x1b => {
                if f.entry.is_some() && frame.phase != EntryPhase::Ready {
                    return Err(err(
                        VmErrorKind::InvalidProgram,
                        "initialization was not completed",
                    ));
                }
                if frame.entered {
                    return Err(err(VmErrorKind::InvalidProgram, "function entered twice"));
                }
                if n != frame.bp {
                    return Err(err(
                        VmErrorKind::InvalidProgram,
                        "function entry has unexpected evaluation values",
                    ));
                }
                if n.checked_add(f.local_count)
                    .is_none_or(|n| n > self.limits.max_stack)
                {
                    return Err(err(
                        VmErrorKind::Limit,
                        "local allocation exceeds stack limit",
                    ));
                }
                let locals = (0..f.local_count)
                    .map(|_| Cell::new(Value::Undefined).map(StackEntry::Value))
                    .collect::<Result<Vec<_>, _>>()?;
                self.stack.extend(locals);
                self.frames.last_mut().unwrap().entered = true;
                self.frames.last_mut().unwrap().phase = EntryPhase::Body;
                Ok(())
            }
            0x07 => self.call_user(&frame, &p, f.local_count, f.function_base, a, b),
            0x2e => Ok(()),
            0x48 => self.return_initialization(&frame, &p),
            0x49 => self.return_function(&frame, f.function()?),
            0x1d => {
                if self.initialization.is_some() || self.construction.is_some() {
                    return Err(err(
                        VmErrorKind::UnsupportedFeature,
                        "finishAction cannot complete an initialization task",
                    ));
                }
                self.completion = Some(Completion::ActionFinished);
                self.catches.clear();
                Ok(())
            }
            _ => Err(err(
                VmErrorKind::UnsupportedFeature,
                format!("opcode {opcode:#x} is unsupported"),
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn local_reference_cannot_revive_after_frame_exit_or_slot_reuse() {
        let mut vm = Vm::new(ObjectStore::new());
        let id = vm.frame_id().unwrap();
        let cell = Cell::new(Value::Int(7)).unwrap();
        let reference = Reference {
            id: cell.id(),
            target: Target::Local {
                frame: id,
                cell: Rc::downgrade(&cell),
            },
            readonly: false,
            _lease: None,
        };
        vm.frames.push(Frame {
            id,
            execution: ExecutionSource::ObjectProgram(ProgramId(0)),
            active_object: Some(ObjectId(0)),
            body: FrameBody::Function(0),
            pc: 0,
            bp: 0,
            argc: 0,
            entered: true,
            phase: EntryPhase::Body,
        });
        assert_eq!(
            vm.resolve(&reference).unwrap().read().unwrap(),
            Value::Int(7)
        );
        vm.frames.clear();
        let new_id = vm.frame_id().unwrap();
        assert_ne!(id, new_id);
        vm.frames.push(Frame {
            id: new_id,
            execution: ExecutionSource::ObjectProgram(ProgramId(0)),
            active_object: Some(ObjectId(0)),
            body: FrameBody::Function(0),
            pc: 0,
            bp: 0,
            argc: 0,
            entered: true,
            phase: EntryPhase::Body,
        });
        assert_eq!(
            vm.resolve(&reference).unwrap_err().kind,
            VmErrorKind::InvalidState
        );
        vm.frames[0].id = id;
        drop(cell);
        assert_eq!(
            vm.resolve(&reference).unwrap_err().kind,
            VmErrorKind::InvalidState
        );
        assert_ne!(reference.id(), 0);
    }
    #[test]
    fn object_reference_resolve_checks_the_owner_even_if_cell_is_retained() {
        let vm = Vm::new(ObjectStore::new());
        let cell = Cell::new(Value::Int(7)).unwrap();
        let reference = Reference {
            id: cell.id(),
            target: Target::Object {
                object: ObjectId(0),
                cell: Rc::downgrade(&cell),
            },
            readonly: false,
            _lease: None,
        };
        assert_eq!(
            vm.resolve(&reference).unwrap_err().kind,
            VmErrorKind::InvalidObject
        );
    }
    #[test]
    fn numeric_updates_reject_readonly_and_invalid_targets_before_mutation() {
        let frame = Frame {
            id: 0,
            execution: ExecutionSource::ObjectProgram(ProgramId(0)),
            active_object: Some(ObjectId(0)),
            body: FrameBody::Function(0),
            pc: 0,
            bp: 0,
            argc: 0,
            entered: true,
            phase: EntryPhase::Body,
        };
        let function = PreparedFunction {
            private: false,
            origin: None,
            pool: None,
            function_base: 0,
            variable_base: 0,
            index: 0,
            name: None,
            instructions: Rc::new(vec![]),
            local_count: 0,
            args: vec![],
            min_args: 0,
            max_args: 0,
            variadic: false,
            entry: None,
            return_type: 255,
            return_attrib: 0,
        };
        for opcode in [0x22, 0x2f] {
            for scenario in 0..3 {
                let cell = Cell::new(Value::Int(7)).unwrap();
                let (target, kind) = match scenario {
                    0 => (
                        Target::Owned {
                            cell: cell.clone(),
                            kind: ReferenceKind::Temporary,
                        },
                        VmErrorKind::UnsupportedFeature,
                    ),
                    1 => (
                        Target::Local {
                            frame: 0,
                            cell: Rc::downgrade(&cell),
                        },
                        VmErrorKind::InvalidState,
                    ),
                    _ => (
                        Target::Object {
                            object: ObjectId(0),
                            cell: Rc::downgrade(&cell),
                        },
                        VmErrorKind::InvalidObject,
                    ),
                };
                let reference = Reference {
                    id: cell.id(),
                    target,
                    readonly: scenario == 0,
                    _lease: None,
                };
                let mut vm = Vm::new(ObjectStore::new());
                vm.stack.push(StackEntry::Reference(reference));
                if opcode == 0x2f {
                    vm.push(Value::Int(2)).unwrap();
                }
                let before = vm.stack().unwrap();
                let error = vm
                    .numeric_update(
                        opcode,
                        if opcode == 0x22 { 1 } else { 2 },
                        &frame,
                        function.local_count,
                        &mut None,
                    )
                    .unwrap_err();
                assert_eq!(error.kind, kind);
                assert_eq!(cell.read().unwrap(), Value::Int(7));
                assert_eq!(vm.stack().unwrap(), before);
            }
        }
    }
    #[test]
    fn cloned_reference_permissions_cannot_be_weakened() {
        let vm = Vm::new(ObjectStore::new());
        let cell = Cell::new(Value::Int(5)).unwrap();
        for kind in [
            ReferenceKind::Temporary,
            ReferenceKind::ArrayMember,
            ReferenceKind::MappingMember,
        ] {
            let r = Reference {
                id: cell.id(),
                target: Target::Owned {
                    cell: cell.clone(),
                    kind,
                },
                readonly: true,
                _lease: None,
            };
            let forwarded = r.clone();
            assert_eq!(
                vm.writable(&forwarded, &Value::Int(9)).unwrap_err().kind,
                VmErrorKind::UnsupportedFeature
            );
            let mut mutable = forwarded;
            mutable.readonly = false;
            if kind != ReferenceKind::Temporary {
                assert_eq!(
                    vm.writable(&mutable, &Value::array(vec![]).unwrap())
                        .unwrap_err()
                        .kind,
                    VmErrorKind::UnsupportedType
                );
            }
            assert_eq!(cell.read().unwrap(), Value::Int(5));
        }
    }
    #[test]
    fn loop_readonly_binding_and_invalidated_owner_preserve_control_state() {
        let function = PreparedFunction {
            private: false,
            origin: None,
            pool: None,
            function_base: 0,
            variable_base: 0,
            index: 0,
            name: None,
            instructions: Rc::new(vec![
                Instruction {
                    opcode: 0x23,
                    operands: [-1, 0],
                    offset: 0,
                },
                Instruction {
                    opcode: 0x12,
                    operands: [0, -1],
                    offset: 4,
                },
            ]),
            local_count: 2,
            args: vec![],
            min_args: 0,
            max_args: 0,
            variadic: false,
            entry: None,
            return_type: 255,
            return_attrib: 0,
        };
        let program = PreparedProgram {
            global_schema: None,
            allow_host_calls: false,
            allow_function_values: false,
            kind: ProgramKind::Object,
            function_capacity: 0,
            source: "synthetic-loop".into(),
            variable_count: 0,
            strings: vec![],
            functions: BTreeMap::from([(0, function)]),
            unimplemented: Default::default(),
            initializations: None,
            init_order: vec![],
            lifecycles: None,
            methods: None,
        };
        let mut objects = ObjectStore::new();
        let program = objects.register_program(program).unwrap();
        let object = objects.create(program, vec![]).unwrap();
        let frame = Frame {
            id: 0,
            execution: ExecutionSource::ObjectProgram(program),
            active_object: Some(object),
            body: FrameBody::Function(0),
            pc: 0,
            bp: 0,
            argc: 0,
            entered: true,
            phase: EntryPhase::Body,
        };
        let mut vm = Vm::new(objects);
        vm.frames.push(frame.clone());
        let control = Cell::new(Value::Undefined).unwrap();
        let bound = Cell::new(Value::Int(1)).unwrap();
        vm.stack.extend([
            StackEntry::Value(control.clone()),
            StackEntry::Value(bound.clone()),
            StackEntry::Reference(Reference {
                id: bound.id(),
                target: Target::Local {
                    frame: 0,
                    cell: Rc::downgrade(&bound),
                },
                readonly: true,
                _lease: None,
            }),
        ]);
        vm.push(Value::Int(2)).unwrap();
        vm.push(Value::Int(1)).unwrap();
        let p = vm.objects.program(program).unwrap();
        let view = CodeView::get(&p, frame.body).unwrap();
        let before = vm.stack().unwrap();
        assert_eq!(
            vm.loop_instruction(&frame, &view, 0x23, -1, 0)
                .unwrap_err()
                .kind,
            VmErrorKind::UnsupportedFeature
        );
        assert_eq!(vm.stack().unwrap(), before);
        assert!(!control.is_loop().unwrap());
        let StackEntry::Reference(r) = &mut vm.stack[2] else {
            unreachable!()
        };
        r.readonly = false;
        vm.loop_instruction(&frame, &view, 0x23, -1, 0).unwrap();
        vm.frames[0].id = 1;
        let before = vm.stack().unwrap();
        assert_eq!(
            vm.loop_instruction(&frame, &view, 0x12, 0, -1)
                .unwrap_err()
                .kind,
            VmErrorKind::InvalidState
        );
        assert_eq!(vm.stack().unwrap(), before);
        assert_eq!(bound.read().unwrap(), Value::Int(1));
        assert!(matches!(
            control.loop_control().unwrap().state,
            LoopState::Numeric { next: 1, .. }
        ));
    }
}

#[cfg(test)]
mod range_tests {
    use super::*;

    fn source_vm(values: Vec<Value>) -> (Vm, Frame, crate::container::ArrayValue) {
        let mut vm = super::catch_tests::fixture(0);
        let Value::Array(array) = Value::array(values).unwrap() else {
            unreachable!()
        };
        vm.push(Value::Array(array.clone())).unwrap();
        vm.push(Value::Int(0)).unwrap();
        vm.push(Value::Int(1)).unwrap();
        let frame = vm.frames[0].clone();
        (vm, frame, array)
    }

    #[test]
    fn ranges_borrow_failure_does_not_partially_normalize_source_or_replace_stack() {
        // ObjectId(999) is deliberately absent from the synthetic store.
        let dead = Value::Object(ObjectId(999));
        let (mut vm, frame, array) = source_vm(vec![dead.clone(), dead.clone()]);
        let before = vm.stack().unwrap();
        let cell = array.cell(1).unwrap();
        cell.with_read_borrow_for_test(|| {
            assert_eq!(
                vm.query_range(&frame, 0, 0).unwrap_err().kind,
                VmErrorKind::InvalidState
            );
            assert_eq!(vm.stack().unwrap(), before);
            assert_eq!(array.elements().unwrap(), vec![dead.clone(), dead.clone()]);
        });
        vm.query_range(&frame, 0, 0).unwrap();
        assert_eq!(array.elements().unwrap(), vec![Value::Undefined; 2]);
        assert_eq!(vm.stack.len(), 1);
    }

    #[test]
    fn ranges_source_stack_borrow_failure_leaves_dead_elements_unchanged() {
        let dead = Value::Object(ObjectId(999));
        let (mut vm, frame, array) = source_vm(vec![dead.clone(), Value::Int(2)]);
        let StackEntry::Value(cell) = vm.stack[0].clone() else {
            unreachable!()
        };
        cell.with_read_borrow_for_test(|| {
            assert_eq!(
                vm.query_range(&frame, 0, 0).unwrap_err().kind,
                VmErrorKind::InvalidState
            );
            assert_eq!(array.elements().unwrap()[0], dead);
            assert_eq!(vm.stack.len(), 3);
        });
    }

    #[test]
    fn ranges_result_validation_failure_keeps_prior_normalizations_uncommitted() {
        let dead = Value::Object(ObjectId(999));
        let (mut vm, frame, array) = source_vm(vec![dead.clone(), Value::Int(2)]);
        let mut deep = Value::Int(1);
        for _ in 0..64 {
            deep = Value::array(vec![deep]).unwrap();
        }
        // Corrupted internal graph, bypassing the ordinary reference write depth checks.
        array.cell(1).unwrap().replace(deep).unwrap();
        assert_eq!(
            vm.query_range(&frame, 0, 0).unwrap_err().kind,
            VmErrorKind::Limit
        );
        assert_eq!(array.cell(0).unwrap().read().unwrap(), dead);
        assert_eq!(vm.stack.len(), 3);
    }

    #[test]
    fn ranges_source_length_guard_and_extreme_interval_math_are_bounded() {
        assert_eq!(
            Vm::range_bounds(i32::MAX as usize + 1, &Value::Int(0), &Value::Int(0), 0)
                .unwrap_err()
                .kind,
            VmErrorKind::Limit
        );
        assert_eq!(
            Vm::range_bounds(
                i32::MAX as usize,
                &Value::Int(i32::MAX),
                &Value::Int(i32::MIN),
                3
            )
            .unwrap(),
            0..i32::MAX as usize
        );
        assert_eq!(
            Vm::range_bounds(0, &Value::Int(i32::MIN), &Value::Int(i32::MAX), 0).unwrap(),
            0..0
        );
    }
}

#[cfg(test)]
mod reference_type_tests {
    use super::*;

    #[test]
    fn ref_checks_dangling_and_readonly_references_keep_raw_identity() {
        for target in [
            Target::Local {
                frame: 999,
                cell: Weak::new(),
            },
            Target::Local {
                frame: 10,
                cell: Weak::new(),
            },
            Target::Object {
                object: ObjectId(999),
                cell: Weak::new(),
            },
        ] {
            let mut vm = super::catch_tests::fixture(0);
            let frame = vm.frames[0].clone();
            vm.stack.push(StackEntry::Reference(Reference {
                id: 123,
                target,
                readonly: true,
                _lease: None,
            }));
            let before = vm.stack().unwrap();
            vm.check_reference_type(&frame, -1).unwrap();
            assert_eq!(vm.stack().unwrap()[0], before[0]);
            assert_eq!(vm.value(1).unwrap(), Value::Int(1));
        }
    }

    #[test]
    fn ref_checks_hidden_loop_and_dead_object_values_are_not_read_or_normalized() {
        let mut vm = super::catch_tests::fixture(2);
        let frame = vm.frames[0].clone();
        let Value::Array(array) = Value::array(vec![Value::Int(7)]).unwrap() else {
            unreachable!()
        };
        let StackEntry::Value(control) = vm.stack[0].clone() else {
            unreachable!()
        };
        Cell::replace_payload_batch(vec![(
            control.clone(),
            SlotPayload::Loop(Box::new(LoopControl::for_test(array.clone()))),
        )])
        .unwrap();
        let StackEntry::Value(dead) = vm.stack[1].clone() else {
            unreachable!()
        };
        dead.replace(Value::Object(ObjectId(999))).unwrap();
        let refs = array.header_refs();
        control.with_read_borrow_for_test(|| {
            vm.check_reference_type(&frame, -1).unwrap();
        });
        vm.check_reference_type(&frame, -2).unwrap();
        assert_eq!(vm.value(2).unwrap(), Value::Int(0));
        assert_eq!(vm.value(3).unwrap(), Value::Int(0));
        assert!(control.is_loop().unwrap());
        assert_eq!(array.header_refs(), refs);
        assert_eq!(dead.read().unwrap(), Value::Object(ObjectId(999)));
    }

    #[test]
    fn ref_checks_full_reference_quota_neither_reserves_nor_clones_a_lease() {
        let mut vm = super::catch_tests::fixture(0);
        let frame = vm.frames[0].clone();
        vm.limits.max_reference_cells = 1;
        let cell = Cell::new(Value::Object(ObjectId(999))).unwrap();
        let reference = vm
            .owned_reference(cell.clone(), ReferenceKind::Temporary)
            .unwrap();
        let lease_count = Rc::strong_count(reference._lease.as_ref().unwrap());
        vm.stack.push(StackEntry::Reference(reference));
        let cell_refs = Rc::strong_count(&cell);
        vm.check_reference_type(&frame, -1).unwrap();
        let StackEntry::Reference(reference) = &vm.stack[0] else {
            unreachable!()
        };
        assert_eq!(
            Rc::strong_count(reference._lease.as_ref().unwrap()),
            lease_count
        );
        assert_eq!(Rc::strong_count(&cell), cell_refs);
        assert_eq!(cell.read().unwrap(), Value::Object(ObjectId(999)));
        assert_eq!(vm.reference_cells.len(), 1);
        assert_eq!(vm.value(1).unwrap(), Value::Int(1));
        let another = Cell::new(Value::Int(8)).unwrap();
        assert_eq!(
            vm.owned_reference(another.clone(), ReferenceKind::Temporary)
                .unwrap_err()
                .kind,
            VmErrorKind::Limit
        );
        vm.stack.clear();
        assert!(
            vm.owned_reference(another, ReferenceKind::Temporary)
                .is_ok()
        );
    }

    #[test]
    fn ref_checks_lookup_uses_actual_frame_bounds_and_signed_indices() {
        let mut vm = super::catch_tests::fixture(0);
        vm.stack.push(StackEntry::Reference(Reference {
            id: 1,
            target: Target::Local {
                frame: 999,
                cell: Weak::new(),
            },
            readonly: false,
            _lease: None,
        }));
        vm.push(Value::Int(8)).unwrap();
        let mut frame = vm.frames[0].clone();
        frame.bp = 2;
        frame.argc = 1; // First stack entry is caller prefix, never an argument here.
        for (index, expected) in [(0, 0), (1, 0), (i32::MAX, 0), (i32::MIN, 0)] {
            vm.check_reference_type(&frame, index).unwrap();
            assert_eq!(vm.value(vm.stack.len() - 1).unwrap(), Value::Int(expected));
            vm.stack.pop();
        }
        frame.argc = 2;
        vm.check_reference_type(&frame, 1).unwrap();
        assert_eq!(vm.value(2).unwrap(), Value::Int(1));
    }

    #[test]
    fn ref_checks_limit_failure_preserves_slots_and_reference_registry() {
        let mut vm = super::catch_tests::fixture(1);
        let frame = vm.frames[0].clone();
        vm.limits.max_stack = 1;
        let before = vm.stack().unwrap();
        assert_eq!(
            vm.check_reference_type(&frame, -1).unwrap_err().kind,
            VmErrorKind::Limit
        );
        assert_eq!(vm.stack().unwrap(), before);
        assert!(vm.reference_cells.is_empty());
    }
}

#[cfg(test)]
mod catch_tests {
    use super::*;

    // A synthetic prepared body permits direct inspection of lifetime and corrupted-state
    // boundaries which cannot be exposed through the public Value/API.
    pub(super) fn fixture(locals: usize) -> Vm {
        let function = PreparedFunction {
            index: 0,
            name: None,
            private: false,
            instructions: Rc::new(vec![Instruction {
                opcode: 0x1d,
                operands: [0, 0],
                offset: 0,
            }]),
            origin: None,
            pool: None,
            function_base: 0,
            variable_base: 0,
            local_count: locals,
            args: vec![],
            min_args: 0,
            max_args: 0,
            variadic: false,
            entry: None,
            return_type: 1,
            return_attrib: 0,
        };
        let program = PreparedProgram {
            global_schema: None,
            allow_host_calls: false,
            allow_function_values: false,
            kind: ProgramKind::Object,
            function_capacity: 0,
            source: "synthetic-catch".into(),
            variable_count: 0,
            strings: vec![],
            functions: BTreeMap::from([(0, function)]),
            unimplemented: Default::default(),
            initializations: None,
            init_order: vec![],
            lifecycles: None,
            methods: None,
        };
        let mut objects = ObjectStore::new();
        let program = objects.register_program(program).unwrap();
        let object = objects.create(program, vec![]).unwrap();
        let mut vm = Vm::new(objects);
        vm.frames.push(Frame {
            id: 10,
            execution: ExecutionSource::ObjectProgram(program),
            active_object: Some(object),
            body: FrameBody::Function(0),
            pc: 0,
            bp: 0,
            argc: 0,
            entered: true,
            phase: EntryPhase::Body,
        });
        for _ in 0..locals {
            vm.push(Value::Undefined).unwrap();
        }
        vm
    }
    fn save(vm: &mut Vm) {
        vm.catches.push(ScriptCatch {
            frame_id: 10,
            frame_depth: 1,
            stack_len: vm.stack.len(),
            target_pc: 0,
        });
    }

    #[test]
    fn catches_unwind_preserves_owner_slots_and_releases_young_loop_headers() {
        let mut vm = fixture(1);
        let Value::Array(array) = Value::array(vec![Value::Int(8)]).unwrap() else {
            unreachable!()
        };
        let StackEntry::Value(owner_cell) = &vm.stack[0] else {
            unreachable!()
        };
        let owner_cell = owner_cell.clone();
        Cell::replace_payload_batch(vec![(
            owner_cell.clone(),
            SlotPayload::Loop(Box::new(LoopControl::for_test(array.clone()))),
        )])
        .unwrap();
        save(&mut vm);
        let mut child = vm.frames[0].clone();
        child.id = 11;
        child.bp = 1;
        vm.frames.push(child);
        let young = Cell::new(Value::Undefined).unwrap();
        let reference = Reference {
            id: young.id(),
            target: Target::Local {
                frame: 11,
                cell: Rc::downgrade(&young),
            },
            readonly: false,
            _lease: None,
        };
        Cell::replace_payload_batch(vec![(
            young.clone(),
            SlotPayload::Loop(Box::new(LoopControl::for_test(array.clone()))),
        )])
        .unwrap();
        vm.stack.push(StackEntry::Value(young));
        assert_eq!(array.header_refs(), 3);
        assert!(vm.route_script_fault().unwrap());
        assert_eq!(array.header_refs(), 2);
        assert_eq!(vm.frames.len(), 1);
        let StackEntry::Value(after) = &vm.stack[0] else {
            unreachable!()
        };
        assert!(Rc::ptr_eq(after, &owner_cell));
        assert!(after.is_loop().unwrap());
        assert!(vm.resolve(&reference).is_err());
        drop(owner_cell);
        vm.stack.clear();
        assert_eq!(array.header_refs(), 1);
    }

    #[test]
    fn catches_mapping_final_header_clear_keeps_surviving_reference_lease_counted() {
        let mut vm = fixture(0);
        vm.limits.max_reference_cells = 1;
        save(&mut vm);
        let Value::Mapping(mapping) = Value::mapping(vec![(Value::Int(2), Value::Int(9))]).unwrap()
        else {
            unreachable!()
        };
        let member = mapping.member(&Value::Int(2)).unwrap();
        let reference = vm
            .owned_reference(member.clone(), ReferenceKind::MappingMember)
            .unwrap();
        vm.push(Value::Mapping(mapping)).unwrap(); // sole header is discarded by recovery
        assert!(vm.route_script_fault().unwrap());
        assert_eq!(member.read().unwrap(), Value::Undefined);
        assert_eq!(vm.resolve(&reference).unwrap().id(), member.id());
        let another = Cell::new(Value::Int(0)).unwrap();
        assert_eq!(
            vm.owned_reference(another.clone(), ReferenceKind::Temporary)
                .unwrap_err()
                .kind,
            VmErrorKind::Limit
        );
        drop(reference);
        assert!(
            vm.owned_reference(another, ReferenceKind::Temporary)
                .is_ok()
        );
    }

    #[test]
    fn catches_invalid_recovery_preflight_preserves_frames_stack_and_handlers() {
        for corruption in 0..4 {
            let mut vm = fixture(0);
            save(&mut vm);
            match corruption {
                0 => vm.catches[0].frame_id = 999,
                1 => vm.catches[0].stack_len = 1,
                2 => vm.catches[0].target_pc = 1,
                _ => {
                    vm.push(Value::Int(7)).unwrap();
                    vm.catches[0].stack_len = 1;
                    let mut child = vm.frames[0].clone();
                    child.id = 11;
                    vm.frames.push(child); // child floor0 is below saved1
                }
            }
            let before = vm.stack().unwrap();
            let depth = vm.frames.len();
            assert_eq!(
                vm.route_script_fault().unwrap_err().kind,
                VmErrorKind::InvalidProgram
            );
            assert_eq!(vm.stack().unwrap(), before);
            assert_eq!(vm.frames.len(), depth);
            assert_eq!(vm.catches.len(), 1);
        }
    }

    #[test]
    fn catches_retire_before_return_validation_and_do_not_retire_callers() {
        let mut vm = fixture(0);
        save(&mut vm);
        let mut child = vm.frames[0].clone();
        child.id = 11;
        vm.frames.push(child.clone());
        vm.catches.push(ScriptCatch {
            frame_id: 11,
            frame_depth: 2,
            stack_len: 0,
            target_pc: 0,
        });
        vm.push(Value::Int(7)).unwrap(); // void return fails after handler retirement
        let program = vm.objects.program(child.program()).unwrap();
        assert_eq!(
            vm.return_function(&child, &program.functions[&0])
                .unwrap_err()
                .kind,
            VmErrorKind::InvalidProgram
        );
        assert_eq!(vm.catches.len(), 1);
        assert_eq!(vm.catches[0].frame_id, 10);
    }

    #[test]
    fn catches_restart_preflight_completion_and_task_cleanup_have_correct_boundaries() {
        let mut vm = fixture(0);
        save(&mut vm);
        let object = vm.frames[0].object().unwrap();
        vm.failure = Some(err(VmErrorKind::Arithmetic, "synthetic failure"));
        assert!(vm.start(object, 99, vec![]).is_err());
        assert_eq!(vm.catches.len(), 1);
        vm.start(object, 0, vec![]).unwrap();
        assert!(vm.catches.is_empty());
        vm.catches.push(ScriptCatch {
            frame_id: vm.frames[0].id,
            frame_depth: 1,
            stack_len: 0,
            target_pc: 0,
        });
        assert_eq!(
            vm.run(1).unwrap(),
            RunOutcome::Finished(Completion::ActionFinished)
        );
        assert!(vm.catches.is_empty());
        save(&mut vm);
        vm.clear_task_frames();
        assert!(vm.catches.is_empty());
    }
}

#[cfg(test)]
mod numeric_fault_tests {
    use super::*;

    #[test]
    fn numeric_catch_reference_guard_failures_never_tag_arithmetic_sources() {
        for selector in [5, 11] {
            for invalid in 0..4 {
                let mut vm = super::catch_tests::fixture(0);
                let frame = vm.frames[0].clone();
                let cell = Cell::new(if invalid == 3 {
                    Value::Undefined
                } else {
                    Value::Int(9)
                })
                .unwrap();
                let reference = Reference {
                    id: cell.id(),
                    readonly: invalid == 0,
                    _lease: None,
                    target: Target::Local {
                        frame: if invalid == 1 { 999 } else { frame.id },
                        cell: Rc::downgrade(&cell),
                    },
                };
                vm.stack.push(StackEntry::Reference(reference.clone()));
                if invalid == 2 {
                    vm.stack.push(StackEntry::Reference(reference));
                } else {
                    vm.push(Value::Int(0)).unwrap();
                }
                let before = vm.stack().unwrap();
                let value_before = cell.read().unwrap();
                let mut source = None;
                let e = vm
                    .numeric_update(0x2f, selector, &frame, 0, &mut source)
                    .unwrap_err();
                assert_eq!(
                    e.kind,
                    match invalid {
                        0 | 2 => VmErrorKind::UnsupportedFeature,
                        1 => VmErrorKind::InvalidState,
                        _ => VmErrorKind::UnsupportedType,
                    }
                );
                assert!(source.is_none());
                assert_eq!(vm.stack().unwrap(), before);
                assert_eq!(cell.read().unwrap(), value_before);
            }
        }
    }

    #[test]
    fn numeric_catch_compound_zero_tags_only_after_resolving_valid_target_and_rhs() {
        for (selector, expected) in [
            (5, RuntimeFaultSite::CompoundDivZero),
            (11, RuntimeFaultSite::CompoundIntRemZero),
        ] {
            let mut vm = super::catch_tests::fixture(0);
            let frame = vm.frames[0].clone();
            let cell = Cell::new(Value::Int(9)).unwrap();
            let reference = vm
                .owned_reference(cell.clone(), ReferenceKind::ArrayMember)
                .unwrap();
            vm.stack.push(StackEntry::Reference(reference));
            vm.push(Value::Int(0)).unwrap();
            let before = vm.stack().unwrap();
            let mut source = None;
            assert_eq!(
                vm.numeric_update(0x2f, selector, &frame, 0, &mut source)
                    .unwrap_err()
                    .kind,
                VmErrorKind::Arithmetic
            );
            assert_eq!(
                std::mem::discriminant(&source.unwrap()),
                std::mem::discriminant(&expected)
            );
            assert_eq!(vm.stack().unwrap(), before);
            assert_eq!(cell.read().unwrap(), Value::Int(9));
        }
    }

    #[test]
    fn numeric_catch_deleted_object_guard_precedes_compound_zero_classification() {
        let mut vm = super::catch_tests::fixture(0);
        let frame = vm.frames[0].clone();
        let cell = Cell::new(Value::Int(9)).unwrap();
        let reference = Reference {
            id: cell.id(),
            readonly: false,
            _lease: None,
            target: Target::Object {
                object: frame.object().unwrap(),
                cell: Rc::downgrade(&cell),
            },
        };
        vm.stack.push(StackEntry::Reference(reference));
        vm.push(Value::Int(0)).unwrap();
        vm.objects.remove(frame.object().unwrap()).unwrap();
        let before = vm.stack().unwrap();
        let mut source = None;
        assert_eq!(
            vm.numeric_update(0x2f, 5, &frame, 0, &mut source)
                .unwrap_err()
                .kind,
            VmErrorKind::InvalidObject
        );
        assert!(source.is_none());
        assert_eq!(vm.stack().unwrap(), before);
        assert_eq!(cell.read().unwrap(), Value::Int(9));
    }
}

#[cfg(test)]
mod global_tests {
    use super::*;
    use crate::prepared::GlobalBankSchema;

    #[test]
    fn bytes_commit_borrow_failure_keeps_pc_slots_and_operands() {
        let (mut vm, _) = fixture(1, None);
        vm.push(Value::bytes(b"left")).unwrap();
        vm.push(Value::bytes(b"right")).unwrap();
        let StackEntry::Value(left) = vm.stack[0].clone() else {
            panic!()
        };
        let before = vm.stack().unwrap();
        left.with_read_borrow_for_test(|| {
            let fault = vm.step().unwrap_err();
            assert_eq!(fault.diagnostic.kind, VmErrorKind::InvalidState);
            assert!(fault.site.is_none());
        });
        assert_eq!(vm.frames[0].pc, 0);
        assert_eq!(vm.stack().unwrap(), before);
        // This path reuses the left cell and requires no Cell allocation.
        crate::cell::with_allocation_failure(0, || vm.step().unwrap());
        assert_eq!(vm.frames[0].pc, 1);
        assert_eq!(left.read().unwrap(), Value::bytes(b"leftright"));
        assert_eq!(vm.stack.len(), 1);
        assert!(matches!(&vm.stack[0], StackEntry::Value(c) if c.id() == left.id()));
    }

    #[test]
    fn bytes_hidden_loop_is_rejected_without_projection_or_fault_provenance() {
        let (mut vm, _) = fixture(0x1c, None);
        vm.push(Value::Undefined).unwrap();
        let StackEntry::Value(cell) = &vm.stack[0] else {
            panic!()
        };
        let Value::Array(array) = Value::array(vec![]).unwrap() else {
            panic!()
        };
        Cell::replace_payload_batch(vec![(
            cell.clone(),
            SlotPayload::Loop(Box::new(LoopControl::for_test(array))),
        )])
        .unwrap();
        vm.push(Value::bytes(b"x")).unwrap();
        let fault = vm.step().unwrap_err();
        assert_eq!(fault.diagnostic.kind, VmErrorKind::UnsupportedFeature);
        assert!(fault.site.is_none());
        assert_eq!(vm.frames[0].pc, 0);
        assert_eq!(vm.stack.len(), 2);
        let StackEntry::Value(cell) = &vm.stack[0] else {
            panic!()
        };
        assert!(cell.is_loop().unwrap());
    }

    #[test]
    fn bytes_lifecycle_task_guard_applies_to_ordinary_helper_frames() {
        for opcode in [1, 0x1c, 0x2a, 0x1e, 0x20, 0x25, 0x27] {
            for phase in [
                None,
                Some(LifecyclePhase::Create),
                Some(LifecyclePhase::RollbackDestruct),
            ] {
                let (mut vm, object) = fixture(opcode, None);
                if let Some(phase) = phase {
                    vm.construction = Some(ConstructionTask {
                        object,
                        program: vm.frames[0].program(),
                        phase,
                        position: 0,
                        constitution: 0,
                        parameter: None,
                        primary: None,
                    });
                } else {
                    vm.initialization = Some(InitializationTask {
                        object,
                        position: 0,
                        constitution: 0,
                    });
                }
                vm.push(Value::bytes(b"a")).unwrap();
                vm.push(Value::bytes(b"b")).unwrap();
                let before = vm.stack().unwrap();
                let fault = vm.step().unwrap_err();
                assert_eq!(fault.diagnostic.kind, VmErrorKind::UnsupportedFeature);
                assert!(fault.site.is_none());
                assert_eq!(vm.frames[0].pc, 0);
                assert_eq!(vm.stack().unwrap(), before);
            }
        }
    }

    fn fixture(opcode: u8, schema: Option<GlobalBankSchema>) -> (Vm, ObjectId) {
        let function = PreparedFunction {
            index: 0,
            name: None,
            private: false,
            instructions: Rc::new(vec![Instruction {
                opcode,
                operands: if opcode == 0x07 { [559, -128] } else { [0, 0] },
                offset: 42,
            }]),
            origin: None,
            pool: None,
            function_base: 0,
            variable_base: 0,
            local_count: 0,
            args: vec![],
            min_args: 0,
            max_args: 0,
            variadic: false,
            entry: None,
            return_type: 255,
            return_attrib: 0,
        };
        let program = PreparedProgram {
            source: "synthetic-global".into(),
            global_schema: schema,
            allow_host_calls: false,
            allow_function_values: false,
            kind: ProgramKind::Object,
            function_capacity: 0,
            variable_count: 0,
            strings: vec![],
            functions: BTreeMap::from([(0, function)]),
            unimplemented: Default::default(),
            initializations: None,
            init_order: vec![],
            lifecycles: None,
            methods: None,
        };
        let mut objects = ObjectStore::new();
        let p = objects.register_program(program).unwrap();
        let o = objects.create(p, vec![]).unwrap();
        objects
            .register_globals(vec![Value::Object(ObjectId(999))])
            .unwrap();
        let mut vm = Vm::new(objects);
        vm.frames.push(Frame {
            id: 1,
            execution: ExecutionSource::ObjectProgram(p),
            active_object: Some(o),
            body: FrameBody::Function(0),
            pc: 0,
            bp: 0,
            argc: 0,
            entered: true,
            phase: EntryPhase::Body,
        });
        (vm, o)
    }
    #[test]
    fn globals_copy_allocation_and_borrow_failures_preserve_source_and_stack() {
        for allocation in [false, true] {
            let (mut vm, _) = fixture(0x35, Some(GlobalBankSchema { slots: 1 }));
            let source = vm.objects.global_cells().unwrap()[0].clone();
            let fail = || {
                let error = vm.run(1).unwrap_err();
                assert_eq!(
                    error.kind,
                    if allocation {
                        VmErrorKind::Limit
                    } else {
                        VmErrorKind::InvalidState
                    }
                );
                assert!(vm.stack.is_empty());
            };
            if allocation {
                crate::cell::with_allocation_failure(0, fail);
            } else {
                source.with_read_borrow_for_test(fail);
            }
            assert_eq!(source.read().unwrap(), Value::Object(ObjectId(999)));
        }
    }
    #[test]
    fn globals_install_allocation_failure_does_not_publish_partial_bank() {
        let mut objects = ObjectStore::new();
        crate::cell::with_allocation_failure(1, || {
            assert_eq!(
                objects
                    .register_globals(vec![Value::Int(1), Value::Int(2)])
                    .unwrap_err()
                    .kind,
                VmErrorKind::Limit
            )
        });
        assert!(objects.global_values().is_err());
        objects.register_globals(vec![Value::Int(3)]).unwrap();
        assert_eq!(objects.global_values().unwrap(), vec![Value::Int(3)]);
    }
    #[test]
    fn globals_reference_creation_does_not_read_or_normalize_borrowed_source() {
        let (mut vm, _) = fixture(0x36, Some(GlobalBankSchema { slots: 1 }));
        let source = vm.objects.global_cells().unwrap()[0].clone();
        source.with_read_borrow_for_test(|| {
            vm.run(1).unwrap();
        });
        assert_eq!(source.read().unwrap(), Value::Object(ObjectId(999)));
        assert!(matches!(vm.stack[0], StackEntry::Reference(_)));
    }
    #[test]
    fn globals_runtime_requires_schema_even_for_internal_instruction() {
        let (mut vm, _) = fixture(0x35, None);
        assert_eq!(vm.run(1).unwrap_err().kind, VmErrorKind::UnsupportedFeature);
    }
    #[test]
    fn globals_lifecycle_helpers_reject_access_and_clean_up_managed_objects() {
        for opcode in [0x35, 0x36] {
            for phase in [
                None,
                Some(LifecyclePhase::Create),
                Some(LifecyclePhase::RollbackDestruct),
            ] {
                let (mut vm, o) = fixture(opcode, Some(GlobalBankSchema { slots: 1 }));
                if let Some(phase) = phase {
                    let program = vm.frames[0].program();
                    vm.construction = Some(ConstructionTask {
                        object: o,
                        program,
                        phase,
                        position: 0,
                        constitution: 0,
                        parameter: None,
                        primary: None,
                    });
                } else {
                    vm.initialization = Some(InitializationTask {
                        object: o,
                        position: 0,
                        constitution: 0,
                    });
                }
                assert_eq!(vm.run(1).unwrap_err().kind, VmErrorKind::UnsupportedFeature);
                assert!(vm.objects.get(o).is_err());
                assert!(vm.stack.is_empty());
                assert!(vm.frames.is_empty());
                assert_eq!(
                    vm.objects.global_values().unwrap(),
                    vec![Value::Object(ObjectId(999))]
                );
            }
        }
    }
    #[test]
    fn global_calls_internal_lifecycle_helper_rejects_and_preserves_primary_rollback_error() {
        for phase in [
            LifecyclePhase::Initialize,
            LifecyclePhase::Create,
            LifecyclePhase::RollbackDestruct,
        ] {
            let (mut vm, o) = fixture(0x07, Some(GlobalBankSchema { slots: 1 }));
            let primary = (phase == LifecyclePhase::RollbackDestruct)
                .then(|| err(VmErrorKind::Arithmetic, "primary"));
            vm.construction = Some(ConstructionTask {
                object: o,
                program: vm.frames[0].program(),
                phase,
                position: 0,
                constitution: 0,
                parameter: None,
                primary,
            });
            let e = vm.run(1).unwrap_err();
            assert_eq!(
                e.kind,
                if phase == LifecyclePhase::RollbackDestruct {
                    VmErrorKind::Arithmetic
                } else {
                    VmErrorKind::UnsupportedFeature
                }
            );
            if phase == LifecyclePhase::RollbackDestruct {
                assert_eq!(
                    vm.rollback_errors()[0].kind,
                    VmErrorKind::UnsupportedFeature
                );
            }
            assert!(vm.objects.get(o).is_err());
            assert!(vm.frames.is_empty());
        }
    }
    #[test]
    fn global_calls_inconsistent_frame_context_is_rejected_without_fake_identity() {
        let (mut vm, _) = fixture(0x41, None);
        vm.frames[0].active_object = None;
        let e = vm.run(1).unwrap_err();
        assert_eq!(e.kind, VmErrorKind::InvalidState);
        assert_eq!(e.object, None);
        assert!(vm.stack.is_empty());
    }
}

#[cfg(test)]
mod host_tests {
    use super::*;
    use crate::{HostError, HostFunction, HostReturn, HostSignature, HostType};
    use std::cell::Cell as Counter;

    fn fixture(slot: i32, argc: i32, allow: bool) -> (Vm, Rc<Counter<usize>>) {
        let mut vm = super::catch_tests::fixture(0);
        let mut function = vm
            .objects
            .program(vm.frames[0].program())
            .unwrap()
            .functions[&0]
            .clone();
        function.return_type = 255;
        function.instructions = Rc::new(vec![Instruction {
            opcode: 6,
            operands: [slot, argc],
            offset: 42,
        }]);
        let program = PreparedProgram {
            source: "synthetic-host".into(),
            global_schema: None,
            allow_host_calls: allow,
            allow_function_values: false,
            kind: ProgramKind::Object,
            function_capacity: 1,
            variable_count: 0,
            strings: vec![],
            functions: BTreeMap::from([(0, function)]),
            unimplemented: Default::default(),
            initializations: None,
            init_order: vec![],
            lifecycles: None,
            methods: None,
        };
        let p = vm.objects.register_program(program).unwrap();
        let o = vm.objects.create(p, vec![]).unwrap();
        vm.frames[0].execution = ExecutionSource::ObjectProgram(p);
        vm.frames[0].active_object = Some(o);
        let calls = Rc::new(Counter::new(0));
        let counter = calls.clone();
        vm.objects
            .register_host_function(
                0,
                HostFunction {
                    name: "internal".into(),
                    signature: HostSignature {
                        required: vec![],
                        optional: vec![],
                        rest: Some(HostType::Any),
                        returns: HostReturn::Mixed,
                    },
                    callback: Rc::new(move |_, a| {
                        counter.set(counter.get() + 1);
                        Ok(a.first().cloned())
                    }),
                },
            )
            .unwrap();
        (vm, calls)
    }
    fn fail_step(vm: &mut Vm, kind: VmErrorKind) {
        let before = vm.stack().unwrap();
        let depth = vm.frames.len();
        let pc = vm.frames[0].pc;
        let fault = vm.step().unwrap_err();
        assert_eq!(fault.diagnostic.kind, kind);
        assert!(fault.site.is_none());
        assert_eq!(vm.frames[0].pc, pc);
        assert_eq!(vm.frames.len(), depth);
        assert_eq!(vm.stack().unwrap(), before);
    }
    #[test]
    fn host_allocation_and_borrow_failures_preserve_pc_operands_and_callback_count() {
        let (mut vm, calls) = fixture(0, 1, true);
        vm.push(Value::Int(7)).unwrap();
        crate::cell::with_allocation_failure(0, || fail_step(&mut vm, VmErrorKind::Limit));
        assert_eq!(calls.get(), 0);
        let StackEntry::Value(cell) = vm.stack[0].clone() else {
            panic!()
        };
        let before = vm.stack().unwrap();
        cell.with_write_borrow_for_test(|| {
            let fault = vm.step().unwrap_err();
            assert_eq!(fault.diagnostic.kind, VmErrorKind::InvalidState);
            assert!(fault.site.is_none());
        });
        assert_eq!(vm.stack().unwrap(), before);
        assert_eq!(vm.frames[0].pc, 0);
        assert_eq!(calls.get(), 0);
        let (mut vm, calls) = fixture(0, 1, true);
        let Value::Array(a) = Value::array(vec![Value::Int(1)]).unwrap() else {
            panic!()
        };
        vm.push(Value::Array(a.clone())).unwrap();
        let before = vm.stack().unwrap();
        a.cell(0).unwrap().with_write_borrow_for_test(|| {
            let fault = vm.step().unwrap_err();
            assert_eq!(fault.diagnostic.kind, VmErrorKind::InvalidState);
            assert!(fault.site.is_none());
        });
        assert_eq!(vm.stack().unwrap(), before);
        assert_eq!(vm.frames[0].pc, 0);
        assert_eq!(calls.get(), 0);
    }
    #[test]
    fn host_hidden_loop_payload_is_rejected_before_value_projection() {
        let (mut vm, calls) = fixture(0, 1, true);
        let Value::Array(array) = Value::array(vec![Value::Int(1)]).unwrap() else {
            panic!()
        };
        let cell = Cell::new(Value::Undefined).unwrap();
        Cell::replace_payload_batch(vec![(
            cell.clone(),
            SlotPayload::Loop(Box::new(LoopControl::for_test(array.clone()))),
        )])
        .unwrap();
        vm.stack.push(StackEntry::Value(cell.clone()));
        fail_step(&mut vm, VmErrorKind::UnsupportedFeature);
        assert!(cell.is_loop().unwrap());
        assert_eq!(array.header_refs(), 2);
        assert_eq!(calls.get(), 0);
    }
    #[test]
    fn host_defensive_capability_bounds_and_task_checks_do_not_advance_pc() {
        for (slot, argc, allow, kind) in [
            (0, 0, false, VmErrorKind::UnsupportedFeature),
            (-1, 0, true, VmErrorKind::Bounds),
            (559, 0, true, VmErrorKind::Bounds),
            (0, -1, true, VmErrorKind::Bounds),
            (0, 128, true, VmErrorKind::Bounds),
        ] {
            let (mut vm, calls) = fixture(slot, argc, allow);
            fail_step(&mut vm, kind);
            assert_eq!(calls.get(), 0);
        }
        let (mut vm, calls) = fixture(0, 0, true);
        vm.initialization = Some(InitializationTask {
            object: vm.frames[0].active_object.unwrap(),
            position: 0,
            constitution: 0,
        });
        fail_step(&mut vm, VmErrorKind::UnsupportedFeature);
        assert_eq!(calls.get(), 0);
        vm.initialization = None;
        vm.construction = Some(ConstructionTask {
            object: vm.frames[0].active_object.unwrap(),
            program: vm.frames[0].program(),
            phase: LifecyclePhase::Create,
            position: 0,
            constitution: 0,
            parameter: None,
            primary: None,
        });
        fail_step(&mut vm, VmErrorKind::UnsupportedFeature);
        assert_eq!(calls.get(), 0);
    }
    #[test]
    fn host_deleted_object_arguments_and_results_are_not_normalized() {
        let (mut vm, calls) = fixture(0, 1, true);
        let deleted = vm.objects.create(vm.frames[0].program(), vec![]).unwrap();
        vm.objects.remove(deleted).unwrap();
        let source = Cell::new(Value::Object(deleted)).unwrap();
        vm.stack.push(StackEntry::Value(source.clone()));
        vm.run(1).unwrap();
        assert_eq!(calls.get(), 1);
        assert_eq!(source.read().unwrap(), Value::Object(deleted));
        assert_eq!(vm.value(0).unwrap(), Value::Object(deleted));
        assert_eq!(vm.frames[0].pc, 1);
    }
    #[test]
    fn host_deep_container_rejection_and_return_validation_release_temporary_headers() {
        let mut nested = Value::Int(1);
        for _ in 0..64 {
            nested = Value::array(vec![nested]).unwrap();
        }
        let Value::Array(outer) = Value::array(vec![Value::Undefined]).unwrap() else {
            panic!()
        };
        outer.cell(0).unwrap().replace(nested).unwrap();
        let (mut vm, calls) = fixture(0, 1, true);
        vm.push(Value::Array(outer.clone())).unwrap();
        fail_step(&mut vm, VmErrorKind::Limit);
        assert_eq!(calls.get(), 0);
        assert_eq!(outer.header_refs(), 2);
        let (mut vm, calls) = fixture(0, 0, true);
        vm.objects.remove_host_function(0).unwrap();
        let counter = calls.clone();
        let retained = outer.clone();
        vm.objects
            .register_host_function(
                0,
                HostFunction {
                    name: "deep-result".into(),
                    signature: HostSignature {
                        required: vec![],
                        optional: vec![],
                        rest: None,
                        returns: HostReturn::Mixed,
                    },
                    callback: Rc::new(move |_, _| {
                        counter.set(counter.get() + 1);
                        Ok(Some(Value::Array(retained.clone())))
                    }),
                },
            )
            .unwrap();
        let before = outer.header_refs();
        fail_step(&mut vm, VmErrorKind::Limit);
        assert_eq!(calls.get(), 1);
        assert_eq!(outer.header_refs(), before);
    }
    #[test]
    fn host_callback_failure_releases_temporaries_and_restart_clears_old_operands() {
        let (mut vm, calls) = fixture(0, 1, true);
        vm.objects.remove_host_function(0).unwrap();
        let counter = calls.clone();
        vm.objects
            .register_host_function(
                0,
                HostFunction {
                    name: "fail".into(),
                    signature: HostSignature {
                        required: vec![HostType::Any],
                        optional: vec![],
                        rest: None,
                        returns: HostReturn::Mixed,
                    },
                    callback: Rc::new(move |_, _| {
                        counter.set(counter.get() + 1);
                        Err(HostError {
                            kind: VmErrorKind::Arithmetic,
                            message: "failure".into(),
                        })
                    }),
                },
            )
            .unwrap();
        let Value::Array(a) = Value::array(vec![Value::Int(9)]).unwrap() else {
            panic!()
        };
        vm.push(Value::Array(a.clone())).unwrap();
        let frame = vm.frames[0].clone();
        let e = vm.run(1).unwrap_err();
        assert_eq!(e.pc, Some(0));
        assert_eq!(vm.frames[0].pc, 0);
        assert_eq!(calls.get(), 1);
        assert_eq!(a.header_refs(), 2);
        assert_eq!(vm.run(1).unwrap_err(), e);
        vm.start(frame.active_object.unwrap(), 0, vec![]).unwrap();
        assert_eq!(a.header_refs(), 1);
    }
}

#[cfg(test)]
mod function_value_tests {
    use super::*;
    use crate::cell::with_allocation_failure;
    fn fixture(opcode: u8, operands: [i32; 2], argc: usize) -> Vm {
        let mut vm = super::catch_tests::fixture(0);
        let mut caller = vm
            .objects
            .program(vm.frames[0].program())
            .unwrap()
            .functions[&0]
            .clone();
        caller.return_type = 255;
        caller.instructions = Rc::new(vec![Instruction {
            opcode,
            operands,
            offset: 84,
        }]);
        let mut target = caller.clone();
        target.index = 1;
        target.args = (0..argc)
            .map(|i| crate::program::Argument {
                raw_hex: String::new(),
                name: None,
                flag: 0,
                r#type: 255,
                attrib: 0,
                index: i as i16,
                default_action_count: 0,
            })
            .collect();
        target.min_args = argc;
        target.max_args = argc;
        target.instructions = Rc::new(vec![Instruction {
            opcode: 0x1d,
            operands: [0, 0],
            offset: 88,
        }]);
        let program = PreparedProgram {
            source: "synthetic-function-values".into(),
            global_schema: None,
            allow_host_calls: false,
            allow_function_values: true,
            kind: ProgramKind::Object,
            function_capacity: 2,
            variable_count: 0,
            strings: vec![],
            functions: BTreeMap::from([(0, caller), (1, target)]),
            unimplemented: Default::default(),
            initializations: None,
            init_order: vec![],
            lifecycles: None,
            methods: None,
        };
        let p = vm.objects.register_program(program).unwrap();
        let o = vm.objects.create(p, vec![]).unwrap();
        vm.frames[0].execution = ExecutionSource::ObjectProgram(p);
        vm.frames[0].active_object = Some(o);
        vm
    }
    fn value(vm: &Vm, bound: Vec<Value>) -> crate::FunctionValue {
        crate::FunctionValue(Rc::new(crate::value::FunctionData {
            origin: vm.objects.function_origin.clone(),
            owner: vm.frames[0].object().unwrap(),
            program: vm.frames[0].program(),
            function: 1,
            bound,
        }))
    }
    fn assert_atomic(vm: &mut Vm, kind: VmErrorKind) {
        let before = vm.stack().unwrap();
        let pc = vm.frames[0].pc;
        let depth = vm.frames.len();
        let id = vm.next_frame;
        let e = vm.run(1).unwrap_err();
        assert_eq!(e.kind, kind);
        assert_eq!(e.pc, Some(pc));
        assert_eq!(e.offset, Some(84));
        assert_eq!(vm.frames[0].pc, pc);
        assert_eq!(vm.frames.len(), depth);
        assert_eq!(vm.next_frame, id);
        assert_eq!(vm.stack().unwrap(), before);
        assert_eq!(vm.run(1).unwrap_err(), e);
    }
    #[test]
    fn function_values_create_allocation_failure_preserves_capture_and_pc() {
        let mut vm = fixture(0x17, [1, 1], 1);
        vm.push(Value::bytes(b"bound")).unwrap();
        with_allocation_failure(0, || assert_atomic(&mut vm, VmErrorKind::Limit));
    }
    #[test]
    fn function_values_call_every_argument_allocation_failure_is_atomic() {
        for after in 0..3 {
            let mut vm = fixture(5, [0, 2], 3);
            let f = value(&vm, vec![Value::Int(1), Value::Int(2)]);
            vm.push(Value::Int(3)).unwrap();
            vm.push(Value::Function(f)).unwrap();
            with_allocation_failure(after, || assert_atomic(&mut vm, VmErrorKind::Limit));
        }
    }
    #[test]
    fn function_values_creation_zero_capture_checks_result_stack_capacity() {
        let mut vm = fixture(0x17, [1, 0], 0);
        vm.limits.max_stack = 0;
        assert_atomic(&mut vm, VmErrorKind::Limit);
    }
    #[test]
    fn function_values_raw_loop_capture_target_and_new_argument_are_rejected() {
        for (op, operands) in [(0x17, [1, 1]), (5, [0, 1]), (5, [0, 2])] {
            let mut vm = fixture(op, operands, 0);
            let Value::Array(owner) = Value::array(vec![]).unwrap() else {
                unreachable!()
            };
            let c = Cell::new(Value::Undefined).unwrap();
            Cell::replace_payload_batch(vec![(
                c.clone(),
                SlotPayload::Loop(Box::new(LoopControl::for_test(owner))),
            )])
            .unwrap();
            vm.stack.push(StackEntry::Value(c.clone()));
            if operands[1] == 2 {
                let f = value(&vm, vec![]);
                vm.push(Value::Function(f)).unwrap();
            }
            assert_atomic(&mut vm, VmErrorKind::UnsupportedFeature);
            assert!(c.is_loop().unwrap());
        }
    }
    #[test]
    fn function_values_identity_token_default_new_and_foreign_copy() {
        let a = ObjectStore::default();
        let b = ObjectStore::new();
        assert!(!Rc::ptr_eq(&a.function_origin, &b.function_origin));
        let vm = fixture(5, [0, 1], 0);
        let mut f = value(&vm, vec![]);
        let other = Rc::new(());
        Rc::get_mut(&mut f.0).unwrap().origin = other;
        Rc::get_mut(&mut f.0).unwrap().owner = ObjectId(999);
        assert_eq!(
            vm.copy_value(&Value::Function(f.clone())),
            Value::Function(f)
        );
    }
    #[test]
    fn function_values_dead_owner_reference_normalization_is_allocation_atomic() {
        let mut vm = fixture(5, [0, 1], 0);
        let f = value(&vm, vec![]);
        vm.objects.remove(f.owner()).unwrap();
        let c = Cell::new(Value::Function(f.clone())).unwrap();
        let r = Reference {
            id: c.id(),
            target: Target::Global { cell: c.clone() },
            readonly: false,
            _lease: None,
        };
        with_allocation_failure(0, || {
            assert_eq!(vm.read_reference(&r).unwrap_err().kind, VmErrorKind::Limit)
        });
        assert_eq!(c.read().unwrap(), Value::Function(f));
        let entry = vm.read_reference(&r).unwrap();
        assert!(matches!(entry,StackEntry::Value(cell) if cell.read().unwrap()==Value::Undefined));
        assert_eq!(c.read().unwrap(), Value::Undefined);
    }
    #[test]
    fn function_values_header_and_bytes_release_after_last_owner() {
        let vm = fixture(5, [0, 1], 0);
        let bytes: Rc<[u8]> = Rc::from(&b"capture"[..]);
        let f = value(&vm, vec![Value::Bytes(bytes.clone())]);
        let weak = Rc::downgrade(&f.0);
        let a = Value::array(vec![Value::Function(f.clone())]).unwrap();
        let m = Value::mapping(vec![(Value::Int(0), Value::Function(f.clone()))]).unwrap();
        let copy = f.clone();
        drop(f);
        drop(a);
        drop(m);
        assert!(weak.upgrade().is_some());
        assert_eq!(Rc::strong_count(&bytes), 2);
        drop(copy);
        assert!(weak.upgrade().is_none());
        assert_eq!(Rc::strong_count(&bytes), 1);
    }
    #[test]
    fn function_values_defensive_flags_slots_code_identity_and_frame_id() {
        for (op, operands, expected) in [
            (5, [2, 1], VmErrorKind::Bounds),
            (5, [0, 0], VmErrorKind::Bounds),
            (0x17, [-1, 0], VmErrorKind::Bounds),
            (0x17, [2, 0], VmErrorKind::Bounds),
        ] {
            let mut vm = fixture(op, operands, 0);
            assert_atomic(&mut vm, expected);
        }
        for bad in 0..3 {
            let mut vm = fixture(5, [0, 1], 0);
            let mut f = value(&vm, vec![]);
            if bad == 0 {
                Rc::get_mut(&mut f.0).unwrap().program = ProgramId(999);
            } else if bad == 1 {
                Rc::get_mut(&mut f.0).unwrap().function = 2;
            } else {
                vm.next_frame = u64::MAX;
            }
            vm.push(Value::Function(f)).unwrap();
            assert_atomic(
                &mut vm,
                match bad {
                    0 => VmErrorKind::InvalidState,
                    1 => VmErrorKind::Bounds,
                    _ => VmErrorKind::Limit,
                },
            );
        }
    }
    #[test]
    fn function_values_lifecycle_context_guards_precede_operand_access() {
        for op in [5, 0x17] {
            let mut vm = fixture(op, [-1, -1], 0);
            let frame = vm.frames[0].clone();
            let p = vm.objects.program(frame.program()).unwrap();
            let code = CodeView::get(&p, frame.body).unwrap();
            let old = vm.objects.program(ProgramId(0)).unwrap();
            let e = if op == 5 {
                vm.call_function_value(&frame, &old, &code, -1, -1)
            } else {
                vm.create_function_value(&frame, &old, &code, -1, -1)
            };
            assert_eq!(e.unwrap_err().kind, VmErrorKind::UnsupportedFeature);
            vm.initialization = Some(InitializationTask {
                object: frame.object().unwrap(),
                position: 0,
                constitution: 0,
            });
            let call = |vm: &mut Vm| {
                if op == 5 {
                    vm.call_function_value(&frame, &p, &code, -1, -1)
                } else {
                    vm.create_function_value(&frame, &p, &code, -1, -1)
                }
            };
            assert_eq!(
                call(&mut vm).unwrap_err().kind,
                VmErrorKind::UnsupportedFeature
            );
            vm.initialization = None;
            for phase in [
                LifecyclePhase::Initialize,
                LifecyclePhase::Create,
                LifecyclePhase::RollbackDestruct,
            ] {
                vm.construction = Some(ConstructionTask {
                    object: frame.object().unwrap(),
                    program: frame.program(),
                    phase,
                    position: 0,
                    constitution: 0,
                    parameter: None,
                    primary: None,
                });
                assert_eq!(
                    call(&mut vm).unwrap_err().kind,
                    VmErrorKind::UnsupportedFeature
                );
                vm.construction = None;
            }
        }
    }
    #[test]
    fn function_values_zero_budget_does_not_allocate_or_consume() {
        for op in [5, 0x17] {
            let mut vm = fixture(op, [1, 0], 0);
            let before = vm.stack().unwrap();
            with_allocation_failure(0, || {
                assert_eq!(vm.run(0).unwrap(), RunOutcome::BudgetExhausted)
            });
            assert_eq!(vm.stack().unwrap(), before);
            assert_eq!(vm.frames[0].pc, 0);
        }
    }
    #[test]
    fn function_values_call_preserves_reference_lease_prefix() {
        let mut vm = fixture(5, [0, 1], 0);
        let c = Cell::new(Value::Int(9)).unwrap();
        let r = vm.owned_reference(c, ReferenceKind::Temporary).unwrap();
        let id = r.id;
        let lease = vm.reference_cells[&id].clone();
        vm.stack.push(StackEntry::Reference(r));
        let f = value(&vm, vec![]);
        vm.push(Value::Function(f)).unwrap();
        vm.run(1).unwrap();
        assert!(lease.upgrade().is_some());
        assert!(matches!(&vm.stack[0],StackEntry::Reference(r) if r.id==id));
        assert_eq!(vm.frames.len(), 2);
        assert_eq!(vm.frames[1].bp, 1);
        assert_eq!(vm.frames[0].pc, 1);
    }
}
