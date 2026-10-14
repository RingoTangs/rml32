//! Bounded Magic559 interpreter; reference artifacts are never executed by tests.
use crate::{
    cell::Cell,
    object::{InitializationState, LifecycleState, ObjectStore},
    prepared::{EntryLayout, Instruction, PreparedFunction, PreparedProgram},
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
            Target::Local { .. } => ReferenceKind::Local,
            Target::Object { .. } => ReferenceKind::Object,
            Target::Owned { kind, .. } => *kind,
        }
    }
    fn id(&self) -> u64 {
        self.id
    }
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
#[derive(Debug, Clone)]
struct Frame {
    id: u64,
    program: ProgramId,
    object: ObjectId,
    body: FrameBody,
    pc: usize,
    bp: usize,
    argc: usize,
    entered: bool,
    phase: EntryPhase,
}
#[derive(Debug)]
pub struct Vm {
    objects: ObjectStore,
    stack: Vec<StackEntry>,
    frames: Vec<Frame>,
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
            program,
            object,
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
                Self::initialization_frame(id, frame.program, frame.object, constitution);
            let task = self.initialization.as_mut().unwrap();
            task.position = position;
            task.constitution = constitution;
        } else {
            self.objects
                .set_initialization(frame.object, InitializationState::Initialized)?;
            self.frames.clear();
            self.reference_cells.clear();
            self.initialization = None;
            if let Some(task) = self.construction.as_mut() {
                task.phase = LifecyclePhase::Create;
                task.position = 0;
                self.advance_construction();
            } else {
                self.completion = Some(Completion::Initialized(frame.object));
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
            program: task.program,
            object: task.object,
            body: FrameBody::Function(slot),
            pc: 0,
            bp: argc,
            argc,
            entered: false,
            phase: EntryPhase::Unchecked,
        }];
        self.stack = arguments;
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
            Target::Owned { cell, .. } => Ok(cell.clone()),
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
        if let Target::Local { frame, .. } = r.target {
            let owner = self
                .frames
                .iter()
                .find(|f| f.id == frame)
                .ok_or_else(|| err(VmErrorKind::InvalidState, "reference frame has exited"))?;
            let program = self.objects.program(owner.program)?;
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
        self.frames = vec![Frame {
            id,
            program,
            object,
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
            if let Err(mut e) = self.step() {
                e.program = Some(frame.program);
                e.object = Some(frame.object);
                e.function = frame.body.function();
                e.initialization = initialization;
                e.lifecycle = self.lifecycle_context();
                e.pc = Some(frame.pc);
                if let Ok(p) = self.objects.program(frame.program)
                    && let Ok(code) = CodeView::get(&p, frame.body)
                {
                    e.source = code.source.to_owned();
                    e.offset = code.instructions.get(frame.pc).map(|i| i.offset);
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
        self.objects.get(frame.object)?;
        let p = self.objects.program(frame.program)?;
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
        let p = self.objects.program(frame.program)?;
        let base = CodeView::get(&p, frame.body)?.variable_base;
        usize::try_from(index + i32::from(base))
            .ok()
            .and_then(|i| self.objects.get(frame.object).ok()?.variables.get(i))
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
            program,
            object,
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
    fn return_function(&mut self, frame: &Frame, f: &PreparedFunction) -> Result<(), VmError> {
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
            crate::numeric::compound(operand, &left, &self.value(n - 1)?)?
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
    fn step(&mut self) -> Result<(), VmError> {
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
        // vm_regularProcessor @0x080978c4 increments IP before dispatch.
        self.frames.last_mut().unwrap().pc += 1;
        let [a, b] = operands;
        let n = self.stack.len();
        match opcode {
            0x08 => self.call_other(&frame, f.local_count, a, b),
            // vm_pushThisObject @0x080975d8. All currently accepted frames own an Object.
            0x41 => self.push(Value::Object(frame.object)),
            0x22 | 0x2f => self.numeric_update(opcode, a, &frame, f.local_count),
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
                        object: frame.object,
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
                let result =
                    crate::numeric::binary(opcode, &self.value(n - 2)?, &self.value(n - 1)?)?;
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
                let value = self.copy_value(&self.value(n - 1)?);
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
                let value = self.copy_value(&self.value(n - 1)?);
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
            0x07 => {
                let argc = (b & 0x3f) as usize;
                self.need(argc, &frame, f.local_count)?;
                if self.frames.len() >= self.limits.max_frames {
                    return Err(err(VmErrorKind::Limit, "call frame limit exceeded"));
                }
                let index = i16::try_from(a + i32::from(f.function_base))
                    .map_err(|_| err(VmErrorKind::Bounds, "relocated function index overflow"))?;
                if p.unimplemented.contains(&index) {
                    return Err(err(
                        VmErrorKind::UnsupportedFeature,
                        "function declaration is unimplemented",
                    ));
                }
                if !p.functions.contains_key(&index) {
                    return Err(err(
                        VmErrorKind::InvalidProgram,
                        "local function index does not exist",
                    ));
                }
                let target = &p.functions[&index];
                Self::check_user_entry(target)?;
                Self::argument_plan(target, argc)?;
                for i in 0..argc {
                    let flag = target.args.get(i).map_or(0, |arg| arg.flag);
                    match (&self.stack[n - 1 - i], flag) {
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
                let id = self.frame_id()?;
                self.frames.push(Frame {
                    id,
                    program: frame.program,
                    object: frame.object,
                    body: FrameBody::Function(index),
                    pc: 0,
                    bp: n,
                    argc,
                    entered: false,
                    phase: EntryPhase::Unchecked,
                });
                Ok(())
            }
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
            program: ProgramId(0),
            object: ObjectId(0),
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
            program: ProgramId(0),
            object: ObjectId(0),
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
            program: ProgramId(0),
            object: ObjectId(0),
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
}
