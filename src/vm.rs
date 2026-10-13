//! Bounded core interpreter. Only synthetic tests execute in this repository workflow.
use crate::{
    object::ObjectStore,
    prepared::{Instruction, PreparedFunction, PreparedProgram},
    value::{ObjectId, ProgramId, Value},
    vm_error::{VmError, VmErrorKind},
};
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Completion {
    Returned(Value),
    ActionFinished,
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
}
impl Default for VmLimits {
    fn default() -> Self {
        Self {
            max_stack: 65536,
            max_frames: 1024,
        }
    }
}
#[derive(Debug, Clone)]
struct Frame {
    program: ProgramId,
    object: ObjectId,
    function: i16,
    pc: usize,
    // Ascending Vec equivalent of old descending-stack bp: arg 0 is bp-1;
    // slot no is bp-1-no. Negative slots are locals/evaluation values.
    bp: usize,
    argc: usize,
    entered: bool,
}
#[derive(Debug)]
pub struct Vm {
    objects: ObjectStore,
    stack: Vec<Value>,
    frames: Vec<Frame>,
    limits: VmLimits,
    completion: Option<Completion>,
    failure: Option<VmError>,
}
fn err(kind: VmErrorKind, message: impl Into<String>) -> VmError {
    VmError::new(kind, message)
}
impl Vm {
    pub fn new(objects: ObjectStore) -> Self {
        Self {
            objects,
            stack: Vec::new(),
            frames: Vec::new(),
            limits: VmLimits::default(),
            completion: None,
            failure: None,
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
    pub fn into_objects(self) -> ObjectStore {
        self.objects
    }
    pub fn stack(&self) -> &[Value] {
        &self.stack
    }
    pub fn frame_depth(&self) -> usize {
        self.frames.len()
    }
    fn copy_value(&self, value: &Value) -> Value {
        match value {
            Value::Object(id) if self.objects.get(*id).is_err() => Value::Undefined,
            _ => value.clone(),
        }
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
        let o = self.objects.get(object)?;
        let program = o.program();
        let p = self.objects.program(program)?;
        if !p.functions.contains_key(&function_index) {
            return Err(err(
                VmErrorKind::InvalidProgram,
                "entry function does not exist",
            ));
        }
        if self.limits.max_frames == 0 || arguments.len() > self.limits.max_stack {
            return Err(err(VmErrorKind::Limit, "entry exceeds VM limits"));
        }
        let arguments: Vec<_> = arguments.iter().rev().map(|v| self.copy_value(v)).collect();
        self.stack = arguments;
        self.frames = vec![Frame {
            program,
            object,
            function: function_index,
            pc: 0,
            bp: self.stack.len(),
            argc: self.stack.len(),
            entered: false,
        }];
        self.completion = None;
        self.failure = None;
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
            if let Err(mut e) = self.step() {
                e.program = Some(frame.program);
                e.object = Some(frame.object);
                e.function = Some(frame.function);
                e.pc = Some(frame.pc);
                if let Ok(p) = self.objects.program(frame.program) {
                    e.source = p.source.clone();
                    e.offset = p
                        .functions
                        .get(&frame.function)
                        .and_then(|f| f.instructions.get(frame.pc))
                        .map(|i| i.offset);
                }
                self.failure = Some(e.clone());
                return Err(e);
            }
            if let Some(c) = &self.completion {
                return Ok(RunOutcome::Finished(c.clone()));
            }
        }
        Ok(RunOutcome::BudgetExhausted)
    }
    fn push(&mut self, value: Value) -> Result<(), VmError> {
        if self.stack.len() >= self.limits.max_stack {
            return Err(err(VmErrorKind::Limit, "value stack limit exceeded"));
        }
        self.stack.push(value);
        Ok(())
    }
    fn context(&self) -> Result<(Frame, Rc<PreparedProgram>), VmError> {
        let frame = self
            .frames
            .last()
            .ok_or_else(|| err(VmErrorKind::InvalidState, "missing call frame"))?
            .clone();
        self.objects.get(frame.object)?;
        let program = self.objects.program(frame.program)?;
        Ok((frame, program))
    }
    fn eval_floor(frame: &Frame, function: &PreparedFunction) -> usize {
        frame.bp
            + if frame.entered {
                function.local_count
            } else {
                0
            }
    }
    fn need(&self, n: usize, frame: &Frame, function: &PreparedFunction) -> Result<(), VmError> {
        if self
            .stack
            .len()
            .saturating_sub(Self::eval_floor(frame, function))
            < n
        {
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
    fn check_args(&mut self, frame: &Frame, f: &PreparedFunction) -> Result<(), VmError> {
        if f.args.len() != f.argument_count || frame.bp < frame.argc || self.stack.len() < frame.bp
        {
            return Err(err(
                VmErrorKind::InvalidProgram,
                "invalid frame argument storage",
            ));
        }
        if frame.argc != f.argument_count {
            return Err(err(
                VmErrorKind::ArgumentCount,
                "fixed argument count does not match prototype",
            ));
        }
        for (i, arg) in f.args.iter().enumerate() {
            let pos = frame.bp - 1 - i;
            let value = crate::numeric::coerce(self.copy_value(&self.stack[pos]), arg.r#type);
            if arg.r#type != 255
                && value.tag() != arg.r#type
                && !(arg.attrib & 8 != 0 && value == Value::Undefined)
            {
                return Err(err(
                    VmErrorKind::UnsupportedType,
                    format!("argument {i} type does not match prototype"),
                ));
            }
            self.stack[pos] = value;
        }
        Ok(())
    }
    fn binary(&mut self, opcode: u8, frame: &Frame, f: &PreparedFunction) -> Result<(), VmError> {
        self.need(2, frame, f)?;
        let len = self.stack.len();
        let result = crate::numeric::binary(opcode, &self.stack[len - 2], &self.stack[len - 1])?;
        self.stack.pop();
        self.stack[len - 2] = result;
        Ok(())
    }
    fn return_function(&mut self, frame: &Frame, f: &PreparedFunction) -> Result<(), VmError> {
        if f.local_count > 0 && !frame.entered {
            return Err(err(
                VmErrorKind::InvalidProgram,
                "local storage was never entered",
            ));
        }
        let floor = frame.bp + f.local_count;
        let count = self.stack.len().checked_sub(floor).ok_or_else(|| {
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
            (_, 1) => self.copy_value(self.stack.last().unwrap()),
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
        self.stack.truncate(frame.bp - frame.argc);
        self.frames.pop();
        if self.frames.is_empty() {
            self.completion = Some(Completion::Returned(result));
            Ok(())
        } else {
            self.push(result)
        }
    }
    fn step(&mut self) -> Result<(), VmError> {
        let (frame, p) = self.context()?;
        let f = p
            .functions
            .get(&frame.function)
            .ok_or_else(|| err(VmErrorKind::InvalidProgram, "current function is missing"))?;
        let Instruction {
            opcode, operands, ..
        } = f
            .instructions
            .get(frame.pc)
            .ok_or_else(|| err(VmErrorKind::Bounds, "program counter outside function code"))?
            .clone();
        // regularProcessor @0x08097950 increments before dispatch.
        self.frames.last_mut().unwrap().pc += 1;
        let [a, b] = operands;
        match opcode {
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
                self.need(count, &frame, f)?;
                let start = self.stack.len() - count;
                let values: Vec<Value> = self.stack[start..].iter().rev().cloned().collect();
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
                self.stack.truncate(start);
                self.push(value)
            }
            0x43 => {
                self.need(2, &frame, f)?;
                let n = self.stack.len();
                let key = &self.stack[n - 1];
                let selected = match &self.stack[n - 2] {
                    Value::Array(array) => {
                        let index = match key {
                            Value::Int(i) => *i,
                            Value::Real(r) => crate::numeric::trunc_i32(r.to_f32()),
                            _ => {
                                return Err(err(
                                    VmErrorKind::UnsupportedType,
                                    "array index requires a number",
                                ));
                            }
                        };
                        usize::try_from(index)
                            .ok()
                            .and_then(|i| array.elements().get(i))
                            .ok_or_else(|| {
                                err(VmErrorKind::Bounds, "array index outside container")
                            })?
                            .clone()
                    }
                    Value::Mapping(mapping) => {
                        mapping.get(key)?.cloned().unwrap_or(Value::Undefined)
                    }
                    _ => {
                        return Err(err(
                            VmErrorKind::UnsupportedType,
                            "member query requires array or mapping",
                        ));
                    }
                };
                let selected = self.copy_value(&selected);
                self.stack.pop();
                self.stack[n - 2] = selected;
                Ok(())
            }
            0x4f => {
                self.need(1, &frame, f)?;
                let size = match self.stack.last().unwrap() {
                    Value::Array(a) => a.elements().len(),
                    Value::Mapping(m) => m.entries().len(),
                    Value::Bytes(b) => b.len(),
                    _ => 0,
                };
                let size = i32::try_from(size)
                    .map_err(|_| err(VmErrorKind::Limit, "size exceeds legacy integer range"))?;
                *self.stack.last_mut().unwrap() = Value::Int(size);
                Ok(())
            }
            0x01 | 0x19 | 0x28 | 0x29 | 0x51 | 0x1c | 0x1e | 0x20 | 0x25 | 0x27 | 0x02 | 0x30
            | 0x55 | 0x26 | 0x4b | 0x2a => self.binary(opcode, &frame, f),
            0x2b => {
                self.need(1, &frame, f)?;
                let value = self.stack.last_mut().unwrap();
                match value {
                    Value::Int(n) => *n = n.wrapping_neg(),
                    Value::Real(r) => *r = crate::RealBits::from_bits(r.bits() ^ 0x80000000),
                    _ => {
                        return Err(err(
                            VmErrorKind::UnsupportedType,
                            "negation requires numbers",
                        ));
                    }
                }
                Ok(())
            }
            0x3b | 0x3c => self.push(Value::Int(a)),
            0x42 => self.push(Value::Undefined),
            0x3f => self.push(Value::Real(crate::RealBits::from_bits(a as u32))),
            0x0d => {
                self.need(1, &frame, f)?;
                if b & 2 != 0 {
                    return Err(err(
                        VmErrorKind::UnsupportedFeature,
                        "const casts are unsupported",
                    ));
                }
                let value = crate::numeric::cast(self.stack.last().unwrap(), a)?;
                *self.stack.last_mut().unwrap() = value;
                Ok(())
            }
            0x24 | 0x2d | 0x4a => {
                self.need(1, &frame, f)?;
                let value = self.copy_value(self.stack.last().unwrap());
                let result = match opcode {
                    0x24 => i32::from(i32::from(value.tag()) == a),
                    0x2d => i32::from(crate::numeric::zero(&value)),
                    0x4a => {
                        if let Value::Int(n) = value {
                            !n
                        } else {
                            return Err(err(
                                VmErrorKind::UnsupportedType,
                                "bitwise NOT requires integer",
                            ));
                        }
                    }
                    _ => unreachable!(),
                };
                *self.stack.last_mut().unwrap() = Value::Int(result);
                Ok(())
            }
            0x40 => {
                let value = usize::try_from(a)
                    .ok()
                    .and_then(|n| p.strings.get(n))
                    .and_then(Option::as_ref)
                    .ok_or_else(|| {
                        err(VmErrorKind::Bounds, "shared string is absent or invalid")
                    })?;
                self.push(value.clone())
            }
            0x32 | 0x33 => {
                let n = if opcode == 0x33 { 1 } else { a.max(0) as usize };
                self.need(n, &frame, f)?;
                self.stack.truncate(self.stack.len() - n);
                Ok(())
            }
            0x37 => {
                let value = self
                    .slot(a, &frame)
                    .map_or(Value::Undefined, |n| self.copy_value(&self.stack[n]));
                self.push(value)
            }
            0x4d => {
                self.need(1, &frame, f)?;
                let pos = self
                    .slot(a, &frame)
                    .ok_or_else(|| err(VmErrorKind::Bounds, "no such local/argument slot"))?;
                self.stack[pos] = self.copy_value(self.stack.last().unwrap());
                Ok(())
            }
            0x3d => {
                let value = usize::try_from(a)
                    .ok()
                    .and_then(|n| self.objects.get(frame.object).ok()?.variables.get(n))
                    .ok_or_else(|| {
                        err(VmErrorKind::Bounds, "object variable index outside storage")
                    })?;
                self.push(self.copy_value(value))
            }
            0x4e => {
                self.need(1, &frame, f)?;
                let value = self.copy_value(self.stack.last().unwrap());
                let slot = usize::try_from(a)
                    .ok()
                    .and_then(|n| {
                        self.objects
                            .get_mut(frame.object)
                            .ok()?
                            .variables
                            .get_mut(n)
                    })
                    .ok_or_else(|| {
                        err(VmErrorKind::Bounds, "object variable index outside storage")
                    })?;
                *slot = value;
                Ok(())
            }
            0x1f => self.jump(a, f.instructions.len()),
            0x21 => {
                self.need(1, &frame, f)?;
                let value = self.copy_value(self.stack.last().unwrap());
                let zero = crate::numeric::zero(&value);
                *self.stack.last_mut().unwrap() = value;
                let jump_on_true = b & 1 != 0;
                if zero != jump_on_true {
                    self.jump(a, f.instructions.len())?;
                }
                if b & 2 != 0 {
                    self.stack.pop();
                }
                Ok(())
            }
            0x0f => {
                self.check_args(&frame, f)?;
                self.jump(a, f.instructions.len())
            }
            0x1b => {
                if frame.entered {
                    return Err(err(VmErrorKind::InvalidProgram, "function entered twice"));
                }
                if self.stack.len() != frame.bp {
                    return Err(err(
                        VmErrorKind::InvalidProgram,
                        "function entry has unexpected evaluation values",
                    ));
                }
                if self
                    .stack
                    .len()
                    .checked_add(f.local_count)
                    .is_none_or(|n| n > self.limits.max_stack)
                {
                    return Err(err(
                        VmErrorKind::Limit,
                        "local allocation exceeds stack limit",
                    ));
                }
                self.stack
                    .resize(self.stack.len() + f.local_count, Value::Undefined);
                self.frames.last_mut().unwrap().entered = true;
                Ok(())
            }
            0x07 => {
                if b & 0x80 != 0 {
                    return Err(err(
                        VmErrorKind::UnsupportedFeature,
                        "global calls are unsupported",
                    ));
                }
                let argc = (b & 0x3f) as usize;
                self.need(argc, &frame, f)?;
                if self.frames.len() >= self.limits.max_frames {
                    return Err(err(VmErrorKind::Limit, "call frame limit exceeded"));
                }
                let index = a as i16;
                if !p.functions.contains_key(&index) {
                    return Err(err(
                        VmErrorKind::InvalidProgram,
                        "local function index does not exist",
                    ));
                }
                self.frames.push(Frame {
                    program: frame.program,
                    object: frame.object,
                    function: index,
                    pc: 0,
                    bp: self.stack.len(),
                    argc,
                    entered: false,
                });
                Ok(())
            }
            0x49 => self.return_function(&frame, f),
            0x1d => {
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
