//! PAK layout: fm_readCatalog @0x080b51ec, fm_unpack @0x080b5500.
use crate::{ByteString, ErrorKind, ParseError, lzss};
use std::collections::HashSet;

#[derive(Debug, Clone, serde::Serialize)]
pub struct Member {
    pub path: ByteString,
    pub record_offset: usize,
    pub attrib: u32,
    pub data_offset: usize,
    pub stored_bytes: usize,
    pub original_bytes: usize,
    pub modified: u32,
    pub directory: bool,
}

#[derive(Debug)]
pub struct PakArchive<'a> {
    bytes: &'a [u8],
    pub header: [u32; 4],
    pub members: Vec<Member>,
}
fn range(bytes: &[u8], offset: usize, length: usize) -> Result<&[u8], ParseError> {
    let end = offset
        .checked_add(length)
        .ok_or_else(|| ParseError::new(offset, ErrorKind::Bounds, "offset overflow"))?;
    bytes
        .get(offset..end)
        .ok_or_else(|| ParseError::new(offset, ErrorKind::Bounds, "range exceeds package"))
}
fn word(bytes: &[u8], i: usize) -> u32 {
    u32::from_le_bytes(bytes[i..i + 4].try_into().unwrap())
}
impl<'a> PakArchive<'a> {
    pub fn parse(bytes: &'a [u8]) -> Result<Self, ParseError> {
        let raw = range(bytes, 0, 16)?;
        let header = [word(raw, 0), word(raw, 4), word(raw, 8), word(raw, 12)];
        // Strict offline-tool policy; not a claimed original driver check.
        if header[0] != 0x11223344 {
            return Err(ParseError::new(
                0,
                ErrorKind::InvalidFormat,
                "unexpected PAK signature",
            ));
        }
        let mut seen = HashSet::new();
        let mut pending = vec![(16usize, header[1] as usize, Vec::<u8>::new())];
        let mut members = Vec::new();
        while let Some((offset, size, prefix)) = pending.pop() {
            if offset < 16 || size % 64 != 0 {
                return Err(ParseError::new(
                    offset,
                    ErrorKind::InvalidFormat,
                    "invalid directory bounds",
                ));
            }
            let directory = range(bytes, offset, size)?;
            if !seen.insert((offset, size)) {
                return Err(ParseError::new(
                    offset,
                    ErrorKind::InvalidFormat,
                    "repeated/cyclic directory",
                ));
            }
            for (i, item) in directory.as_chunks::<64>().0.iter().enumerate() {
                let record = offset + i * 64;
                let name = item[20..64].split(|b| *b == 0).next().unwrap_or_default();
                if name.is_empty()
                    || name == b"."
                    || name == b".."
                    || name.contains(&b'/')
                    || name.contains(&b'\\')
                {
                    return Err(ParseError::new(
                        record,
                        ErrorKind::InvalidFormat,
                        "invalid member name",
                    ));
                }
                let mut path = prefix.clone();
                path.extend_from_slice(name);
                let attrib = word(item, 0);
                let entry = Member {
                    path: ByteString::new(&path),
                    record_offset: record,
                    attrib,
                    data_offset: word(item, 4) as usize,
                    stored_bytes: word(item, 8) as usize,
                    original_bytes: word(item, 12) as usize,
                    modified: word(item, 16),
                    directory: attrib & 1 != 0,
                };
                range(bytes, entry.data_offset, entry.stored_bytes)?;
                if entry.directory {
                    path.push(b'/');
                    pending.push((entry.data_offset, entry.stored_bytes, path));
                }
                members.push(entry);
            }
        }
        members.sort_by(|a, b| a.path.bytes.cmp(&b.path.bytes));
        Ok(Self {
            bytes,
            header,
            members,
        })
    }
    pub fn read_member(&self, path: impl AsRef<[u8]>) -> Result<Vec<u8>, ParseError> {
        let path = path.as_ref();
        let path = path.strip_prefix(b"/").unwrap_or(path);
        let matches: Vec<_> = self
            .members
            .iter()
            .filter(|m| m.path.bytes.eq_ignore_ascii_case(path))
            .collect();
        if matches.len() != 1 {
            return Err(ParseError::new(
                0,
                if matches.is_empty() {
                    ErrorKind::InvalidFormat
                } else {
                    ErrorKind::Ambiguous
                },
                "member missing or ambiguous",
            ));
        }
        self.read_entry(matches[0])
    }
    pub fn read_entry(&self, entry: &Member) -> Result<Vec<u8>, ParseError> {
        if entry.directory {
            return Err(ParseError::new(
                entry.record_offset,
                ErrorKind::InvalidFormat,
                "cannot read directory as a file",
            ));
        }
        if entry.attrib & 0x80000000 != 0 {
            Ok(range(self.bytes, entry.data_offset, entry.original_bytes)?.to_vec())
        } else {
            let payload = range(self.bytes, entry.data_offset, entry.stored_bytes)?;
            lzss::decompress(payload, entry.original_bytes).map_err(|mut e| {
                e.offset += entry.data_offset;
                e
            })
        }
    }
}
