//! Selected Bytes semantics from stage2ai evidence; limits are Rust policy.
use crate::{Value, VmError, VmErrorKind};

pub(crate) fn selected(op: u8, left: &Value, right: &Value) -> bool {
    matches!(op, 1 | 0x1c | 0x2a | 0x1e | 0x20 | 0x25 | 0x27)
        && (matches!(left, Value::Bytes(_)) || matches!(right, Value::Bytes(_)))
        && !(matches!(op, 0x1c | 0x2a)
            && (matches!(left, Value::Function(_)) || matches!(right, Value::Function(_))))
}

fn limit() -> VmError {
    VmError::new(
        VmErrorKind::Limit,
        "Bytes concatenation exceeds resource limit",
    )
}
fn result_len(left: usize, right: usize) -> Result<usize, VmError> {
    left.checked_add(right)
        .filter(|n| *n <= 65_536)
        .ok_or_else(limit)
}
fn prefix(bytes: &[u8]) -> &[u8] {
    &bytes[..bytes.iter().position(|b| *b == 0).unwrap_or(bytes.len())]
}
pub(crate) fn binary(op: u8, left: &Value, right: &Value) -> Result<Value, VmError> {
    if matches!(op, 0x1c | 0x2a) {
        let equal = matches!((left, right), (Value::Bytes(a), Value::Bytes(b)) if a == b);
        return Ok(Value::Int(i32::from(if op == 0x1c {
            equal
        } else {
            !equal
        })));
    }
    if op == 1 {
        if let Value::Bytes(left) = left {
            let mut digits = [0u8; 11];
            let append: &[u8] = match right {
                Value::Bytes(b) => b,
                Value::Int(n) => {
                    let mut magnitude = n.unsigned_abs();
                    let mut start = digits.len();
                    loop {
                        start -= 1;
                        digits[start] = b'0' + (magnitude % 10) as u8;
                        magnitude /= 10;
                        if magnitude == 0 {
                            break;
                        }
                    }
                    if *n < 0 {
                        start -= 1;
                        digits[start] = b'-';
                    }
                    &digits[start..]
                }
                _ => return Err(unsupported()),
            };
            let length = result_len(left.len(), append.len())?;
            if append.is_empty() {
                return Ok(Value::Bytes(left.clone()));
            }
            let mut result = Vec::new();
            result.try_reserve_exact(length).map_err(|_| limit())?;
            result.extend_from_slice(left);
            result.extend_from_slice(append);
            return Ok(Value::Bytes(result.into()));
        }
    } else if let (Value::Bytes(a), Value::Bytes(b)) = (left, right) {
        let order = prefix(a).cmp(prefix(b));
        let yes = match op {
            0x1e => order.is_ge(),
            0x20 => order.is_gt(),
            0x25 => order.is_le(),
            0x27 => order.is_lt(),
            _ => return Err(unsupported()),
        };
        return Ok(Value::Int(i32::from(yes)));
    }
    Err(unsupported())
}
fn unsupported() -> VmError {
    VmError::new(VmErrorKind::UnsupportedType, "unsupported Bytes operands")
}

#[cfg(test)]
mod tests {
    #[test]
    fn checked_length_includes_empty_append_and_overflow() {
        assert_eq!(super::result_len(65_536, 0).unwrap(), 65_536);
        for (a, b) in [(65_537, 0), (usize::MAX, 1), (65_536, 1)] {
            assert_eq!(
                super::result_len(a, b).unwrap_err().kind,
                crate::VmErrorKind::Limit
            );
        }
    }
}
