use std::io::Read;
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum Error {
    #[error("Read error: {0}")]
    ReadError(String),
    #[error("End of file")]
    Eof,
}

pub struct CharReader<R: Read> {
    buf: [u8; 1],
    reader: R,
}

impl<R: Read> CharReader<R> {
    fn new(reader: R) -> Self {
        Self { buf: [0], reader }
    }

    pub fn get_char(&mut self) -> Result<char, Error> {
        match self.reader.read(&mut self.buf) {
            Ok(read_bytes) if read_bytes == 0 => Err(Error::Eof),
            Ok(_) => Ok(self.buf[0].into()),
            Err(e) => Err(Error::ReadError(e.to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reader_get_char() {
        let mut reader = CharReader::new("hello world!".as_bytes());

        assert_eq!(reader.get_char(), Ok('h'));
        assert_eq!(reader.get_char(), Ok('e'));
        assert_eq!(reader.get_char(), Ok('l'));
        assert_eq!(reader.get_char(), Ok('l'));
        assert_eq!(reader.get_char(), Ok('o'));
        assert_eq!(reader.get_char(), Ok(' '));
        assert_eq!(reader.get_char(), Ok('w'));
        assert_eq!(reader.get_char(), Ok('o'));
        assert_eq!(reader.get_char(), Ok('r'));
        assert_eq!(reader.get_char(), Ok('l'));
        assert_eq!(reader.get_char(), Ok('d'));
        assert_eq!(reader.get_char(), Ok('!'));
        assert_eq!(reader.get_char(), Err(Error::Eof));
    }
}
