//! Mirrors lzss_decompress @0x080cb47c; 4096-byte ring, LSB-first flags.
use crate::{ErrorKind, ParseError};

pub fn decompress(payload: &[u8], expected: usize) -> Result<Vec<u8>, ParseError> {
    let mut ring = [0u8; 4096];
    ring[..0xfee].fill(b' ');
    let (mut cursor, mut flags, mut pos) = (0xfee, 0u16, 0usize);
    // Allocate as output grows, not from an untrusted declared length.
    let mut out = Vec::new();
    while pos < payload.len() {
        flags >>= 1;
        if flags & 0x100 == 0 {
            flags = u16::from(payload[pos]) | 0xff00;
            pos += 1;
        }
        let (source, count, literal) = if flags & 1 != 0 {
            let value = *payload
                .get(pos)
                .ok_or_else(|| ParseError::new(pos, ErrorKind::Compression, "truncated literal"))?;
            pos += 1;
            (0, 1, Some(value))
        } else {
            let pair = payload.get(pos..pos.saturating_add(2)).ok_or_else(|| {
                ParseError::new(pos, ErrorKind::Compression, "truncated backreference")
            })?;
            pos += 2;
            (
                usize::from(pair[0]) | ((usize::from(pair[1]) & 0xf0) << 4),
                usize::from(pair[1] & 15) + 3,
                None,
            )
        };
        if out
            .len()
            .checked_add(count)
            .is_none_or(|end| end > expected)
        {
            return Err(ParseError::new(
                pos,
                ErrorKind::Compression,
                "output exceeds declared length",
            ));
        }
        for i in 0..count {
            let value = literal.unwrap_or(ring[(source + i) & 4095]);
            out.push(value);
            ring[cursor] = value;
            cursor = (cursor + 1) & 4095;
        }
    }
    if out.len() != expected {
        return Err(ParseError::new(
            pos,
            ErrorKind::Compression,
            format!("LZSS size mismatch {} != {expected}", out.len()),
        ));
    }
    Ok(out)
}
