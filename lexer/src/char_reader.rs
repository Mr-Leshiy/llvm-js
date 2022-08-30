use super::Position;
use std::io::Read;
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum Error {
    #[error("Read error: {0}, position: {1}")]
    ReadError(String, Position),
    #[error("End of file")]
    Eof,
}

pub struct CharReader<R: Read> {
    buf: [u8; 1],
    reader: R,

    position: Position,
    saved: Option<char>,
}

impl<R: Read> CharReader<R> {
    pub fn new(reader: R) -> Self {
        Self {
            buf: [0],
            reader,
            position: Position::new(1, 1),
            saved: None,
        }
    }

    pub fn get_char(&mut self) -> Result<char, Error> {
        match self.saved {
            Some(char) => {
                self.saved = None;
                Ok(char)
            }
            None => match self.reader.read(&mut self.buf) {
                Ok(read_bytes) if read_bytes == 0 => Err(Error::Eof),
                Ok(_) => {
                    self.position.inc_line();
                    if self.buf[0] as char == '\n' {
                        self.position.inc_column();
                    }
                    Ok(self.buf[0].into())
                }
                Err(e) => Err(Error::ReadError(e.to_string(), self.position.clone())),
            },
        }
    }

    pub fn save(&mut self, char: char) {
        self.saved = Some(char);
    }

    pub fn get_position(&self) -> &Position {
        &self.position
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
        assert_eq!(reader.get_char(), Err(Error::Eof));
        assert_eq!(reader.get_char(), Err(Error::Eof));
    }

    #[test]
    fn reader_get_position_test() {
        let mut reader = CharReader::new("a \n b".as_bytes());

        assert_eq!(reader.get_char(), Ok('a'));
        assert_eq!(reader.get_position(), &Position::new(2, 1));

        assert_eq!(reader.get_char(), Ok(' '));
        assert_eq!(reader.get_position(), &Position::new(3, 1));

        assert_eq!(reader.get_char(), Ok('\n'));
        assert_eq!(reader.get_position(), &Position::new(1, 2));

        assert_eq!(reader.get_char(), Ok(' '));
        assert_eq!(reader.get_position(), &Position::new(2, 2));

        assert_eq!(reader.get_char(), Ok('b'));
        assert_eq!(reader.get_position(), &Position::new(3, 2));
    }

    #[test]
    fn reader_save_test() {
        let mut reader = CharReader::new("a b".as_bytes());

        assert_eq!(reader.get_char(), Ok('a'));
        reader.save('a');
        assert_eq!(reader.get_char(), Ok('a'));
        reader.save('b');
        assert_eq!(reader.get_char(), Ok('b'));
        assert_eq!(reader.get_char(), Ok(' '));
        assert_eq!(reader.get_char(), Ok('b'));
    }
}
