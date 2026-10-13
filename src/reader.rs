use crate::{ByteString, ErrorKind, ParseError};

pub(crate) struct Reader<'a> {
    pub data: &'a [u8],
    pub pos: usize,
}
impl<'a> Reader<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self { data, pos: 0 }
    }
    pub fn error(&self, message: impl Into<String>) -> ParseError {
        ParseError::new(self.pos, ErrorKind::InvalidFormat, message)
    }
    pub fn take(&mut self, n: usize) -> Result<&'a [u8], ParseError> {
        let end = self
            .pos
            .checked_add(n)
            .ok_or_else(|| self.error("read length overflow"))?;
        let bytes = self
            .data
            .get(self.pos..end)
            .ok_or_else(|| ParseError::new(self.pos, ErrorKind::Bounds, "read exceeds input"))?;
        self.pos = end;
        Ok(bytes)
    }
    pub fn u8(&mut self) -> Result<u8, ParseError> {
        Ok(self.take(1)?[0])
    }
    pub fn i16(&mut self) -> Result<i16, ParseError> {
        Ok(i16::from_le_bytes(self.take(2)?.try_into().unwrap()))
    }
    pub fn i32(&mut self) -> Result<i32, ParseError> {
        Ok(i32::from_le_bytes(self.take(4)?.try_into().unwrap()))
    }
    pub fn count(&mut self) -> Result<usize, ParseError> {
        let offset = self.pos;
        let n = self.i32()?;
        if !(0..=1_000_000).contains(&n) {
            return Err(ParseError::new(
                offset,
                ErrorKind::Limit,
                "count outside offline tool limit 0..1000000",
            ));
        }
        Ok(n as usize)
    }
    pub fn short_count(&mut self) -> Result<usize, ParseError> {
        let n = self.i16()?;
        if n < 0 {
            return Err(self.error("negative short count"));
        }
        Ok(n as usize)
    }
    pub fn string(&mut self) -> Result<Option<ByteString>, ParseError> {
        let n = self.i32()?;
        // The supplied loader uses nonpositive lengths for absent strings.
        if n <= 0 {
            return Ok(None);
        }
        Ok(Some(ByteString::new(self.take(n as usize)?)))
    }
}
