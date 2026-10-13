use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum ErrorKind {
    Bounds,
    InvalidFormat,
    UnsupportedFormat,
    Ambiguous,
    Compression,
    Limit,
    Io,
    Integrity,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct ParseError {
    pub source: String,
    pub offset: usize,
    pub kind: ErrorKind,
    pub message: String,
}

impl ParseError {
    pub fn new(offset: usize, kind: ErrorKind, message: impl Into<String>) -> Self {
        Self {
            source: String::new(),
            offset,
            kind,
            message: message.into(),
        }
    }
    pub fn with_source(mut self, source: impl Into<String>) -> Self {
        self.source = source.into();
        self
    }
}
impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} @0x{:x}: {:?}: {}",
            self.source, self.offset, self.kind, self.message
        )
    }
}
impl std::error::Error for ParseError {}
