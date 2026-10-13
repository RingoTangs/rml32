//! Numeric compatibility adapters, based on Ghidra instruction evidence.
use crate::{RealBits, Value, VmError, VmErrorKind};
fn real(x: f32) -> Value {
    Value::Real(RealBits::from_f32(x))
}
fn unsupported() -> VmError {
    VmError::new(
        VmErrorKind::UnsupportedType,
        "operation requires supported numeric operands",
    )
}
/// CVTTSS2SI: truncate; indefinite INT_MIN for NaN/infinite/out-of-range.
pub(crate) fn trunc_i32(x: f32) -> i32 {
    if !x.is_finite() || !(-2147483648.0..2147483648.0).contains(&f64::from(x)) {
        i32::MIN
    } else {
        x as i32
    }
}
pub(crate) fn coerce(value: Value, tag: u8) -> Value {
    match (tag, value) {
        (2, Value::Real(x)) => Value::Int(trunc_i32(x.to_f32())),
        (3, Value::Int(n)) => real(n as f32),
        (_, value) => value,
    }
}
pub(crate) fn cast(value: &Value, tag: i32) -> Result<Value, VmError> {
    match (tag, value) {
        (2, Value::Undefined | Value::Void) => Ok(Value::Int(0)),
        (3, Value::Undefined | Value::Void) => Ok(real(0.0)),
        (2 | 3, Value::Int(_) | Value::Real(_)) => Ok(coerce(value.clone(), tag as u8)),
        _ => Err(unsupported()),
    }
}
pub(crate) fn zero(value: &Value) -> bool {
    match value {
        Value::Undefined | Value::Void => true,
        Value::Int(n) => *n == 0,
        Value::Real(r) => r.to_f32() == 0.0,
        _ => false,
    }
}
// Exact i32 and f32 inputs can land at a binary32 midpoint after an intermediate
// binary64 rounding. FMA / two-sum residuals select its correct side instead of
// accidentally rounding twice. No floating-point environment is modified.
fn round32(wide: f64, residual: f64) -> f32 {
    let rounded = wide as f32;
    if !wide.is_finite() || residual == 0.0 || residual.is_nan() {
        return rounded;
    }
    if rounded.is_infinite() {
        let max = f64::from(f32::MAX);
        let threshold = max + (max - f64::from(f32::MAX.next_down())) * 0.5;
        if wide.abs() == threshold && wide.signum() != residual.signum() {
            return f32::MAX.copysign(rounded);
        }
        return rounded;
    }
    let center = f64::from(rounded);
    let neighbor = if wide > center {
        rounded.next_up()
    } else {
        rounded.next_down()
    };
    if neighbor.is_finite() && wide == (center + f64::from(neighbor)) * 0.5 {
        if residual > 0.0 {
            rounded.max(neighbor)
        } else {
            rounded.min(neighbor)
        }
    } else {
        rounded
    }
}
fn wide_arithmetic(op: u8, a: f64, b: f64) -> f32 {
    let (wide, error) = match op {
        1 | 0x51 => {
            let b = if op == 0x51 { -b } else { b };
            let s = a + b;
            let t = s - a;
            (s, (a - (s - t)) + (b - t))
        }
        0x29 => {
            let p = a * b;
            (p, a.mul_add(b, -p))
        }
        0x19 => {
            let q = a / b;
            (q, (-q).mul_add(b, a) / b)
        }
        _ => unreachable!(),
    };
    round32(wide, error)
}
fn as32(v: &Value) -> Result<f32, VmError> {
    match v {
        Value::Int(n) => Ok(*n as f32),
        Value::Real(r) => Ok(r.to_f32()),
        _ => Err(unsupported()),
    }
}
fn as64(v: &Value) -> Result<f64, VmError> {
    match v {
        Value::Int(n) => Ok(f64::from(*n)),
        Value::Real(r) => Ok(f64::from(r.to_f32())),
        _ => Err(unsupported()),
    }
}
pub(crate) fn binary(op: u8, left: &Value, right: &Value) -> Result<Value, VmError> {
    if matches!(op, 2 | 0x30 | 0x55 | 0x26 | 0x4b) {
        let (Value::Int(a), Value::Int(b)) = (left, right) else {
            return Err(unsupported());
        };
        return Ok(Value::Int(match op {
            2 => a & b,
            0x30 => a | b,
            0x55 => a ^ b,
            0x26 => {
                if (0..32).contains(b) {
                    a.wrapping_shl(*b as u32)
                } else {
                    0
                }
            }
            0x4b => {
                if (0..32).contains(b) {
                    ((*a as u32) >> (*b as u32)) as i32
                } else {
                    0
                }
            }
            _ => unreachable!(),
        }));
    }
    if let (Value::Int(a), Value::Int(b)) = (left, right) {
        let value = match op {
            1 => a.wrapping_add(*b),
            0x51 => a.wrapping_sub(*b),
            0x29 => a.wrapping_mul(*b),
            0x19 => a.checked_div(*b).ok_or_else(|| {
                VmError::new(VmErrorKind::Arithmetic, "IDIV divide by zero or overflow")
            })?,
            0x28 => a.checked_rem(*b).ok_or_else(|| {
                VmError::new(
                    VmErrorKind::Arithmetic,
                    "IDIV remainder by zero or overflow",
                )
            })?,
            0x1c => i32::from(a == b),
            0x2a => i32::from(a != b),
            0x1e => i32::from(a >= b),
            0x20 => i32::from(a > b),
            0x25 => i32::from(a <= b),
            0x27 => i32::from(a < b),
            _ => return Err(unsupported()),
        };
        return Ok(Value::Int(value));
    }
    if matches!(op, 0x1c | 0x2a | 0x1e | 0x20 | 0x25 | 0x27) {
        // Comparison explicitly rounds integer operands to binary32 first.
        let (a, b) = (as32(left)?, as32(right)?);
        return Ok(Value::Int(i32::from(match op {
            0x1c => a == b,
            0x2a => a != b,
            0x1e => a >= b,
            0x20 => a > b,
            0x25 => a <= b,
            0x27 => a < b,
            _ => unreachable!(),
        })));
    }
    if op == 0x28 {
        // CVTSI2SS before libm fmod(double,double), then store binary32.
        return Ok(real(
            (f64::from(as32(left)?) % f64::from(as32(right)?)) as f32,
        ));
    }
    let (a, b) = (as64(left)?, as64(right)?);
    if op == 0x19 && b == 0.0 {
        return Err(VmError::new(VmErrorKind::Arithmetic, "divide by zero"));
    }
    let result = wide_arithmetic(op, a, b);
    Ok(if matches!(left, Value::Int(_)) {
        Value::Int(trunc_i32(result))
    } else {
        real(result)
    })
}
