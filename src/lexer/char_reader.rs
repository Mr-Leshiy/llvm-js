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
}

impl<R: Read> CharReader<R> {
    pub fn new(reader: R) -> Self {
        Self {
            buf: [0],
            reader,
            position: Position { line: 0, column: 0 },
        }
    }

    pub fn get_char(&mut self) -> Result<char, Error> {
        match self.reader.read(&mut self.buf) {
            Ok(read_bytes) if read_bytes == 0 => Err(Error::Eof),
            Ok(_) => {
                self.position.inc_line();
                if self.buf[0] as char == '\n' {
                    self.position.inc_column();
                }
                Ok(self.buf[0].into())
            }
            Err(e) => Err(Error::ReadError(e.to_string(), self.position.clone())),
        }
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
        let mut reader = CharReader::new("hello world! \n hello world!".as_bytes());

        assert_eq!(reader.get_char(), Ok('h'));
        assert_eq!(reader.get_position(), &Position { line: 1, column: 0 });

        assert_eq!(reader.get_char(), Ok('e'));
        assert_eq!(reader.get_position(), &Position { line: 2, column: 0 });

        assert_eq!(reader.get_char(), Ok('l'));
        assert_eq!(reader.get_position(), &Position { line: 3, column: 0 });

        assert_eq!(reader.get_char(), Ok('l'));
        assert_eq!(reader.get_position(), &Position { line: 4, column: 0 });

        assert_eq!(reader.get_char(), Ok('o'));
        assert_eq!(reader.get_position(), &Position { line: 5, column: 0 });

        assert_eq!(reader.get_char(), Ok(' '));
        assert_eq!(reader.get_position(), &Position { line: 6, column: 0 });

        assert_eq!(reader.get_char(), Ok('w'));
        assert_eq!(reader.get_position(), &Position { line: 7, column: 0 });

        assert_eq!(reader.get_char(), Ok('o'));
        assert_eq!(reader.get_position(), &Position { line: 8, column: 0 });

        assert_eq!(reader.get_char(), Ok('r'));
        assert_eq!(reader.get_position(), &Position { line: 9, column: 0 });

        assert_eq!(reader.get_char(), Ok('l'));
        assert_eq!(
            reader.get_position(),
            &Position {
                line: 10,
                column: 0
            }
        );

        assert_eq!(reader.get_char(), Ok('d'));
        assert_eq!(
            reader.get_position(),
            &Position {
                line: 11,
                column: 0
            }
        );

        assert_eq!(reader.get_char(), Ok('!'));
        assert_eq!(
            reader.get_position(),
            &Position {
                line: 12,
                column: 0
            }
        );

        assert_eq!(reader.get_char(), Ok(' '));
        assert_eq!(
            reader.get_position(),
            &Position {
                line: 13,
                column: 0
            }
        );

        assert_eq!(reader.get_char(), Ok('\n'));
        assert_eq!(reader.get_position(), &Position { line: 0, column: 1 });

        // new line

        assert_eq!(reader.get_char(), Ok(' '));
        assert_eq!(reader.get_position(), &Position { line: 1, column: 1 });

        assert_eq!(reader.get_char(), Ok('h'));
        assert_eq!(reader.get_position(), &Position { line: 2, column: 1 });

        assert_eq!(reader.get_char(), Ok('e'));
        assert_eq!(reader.get_position(), &Position { line: 3, column: 1 });

        assert_eq!(reader.get_char(), Ok('l'));
        assert_eq!(reader.get_position(), &Position { line: 4, column: 1 });

        assert_eq!(reader.get_char(), Ok('l'));
        assert_eq!(reader.get_position(), &Position { line: 5, column: 1 });

        assert_eq!(reader.get_char(), Ok('o'));
        assert_eq!(reader.get_position(), &Position { line: 6, column: 1 });

        assert_eq!(reader.get_char(), Ok(' '));
        assert_eq!(reader.get_position(), &Position { line: 7, column: 1 });

        assert_eq!(reader.get_char(), Ok('w'));
        assert_eq!(reader.get_position(), &Position { line: 8, column: 1 });

        assert_eq!(reader.get_char(), Ok('o'));
        assert_eq!(reader.get_position(), &Position { line: 9, column: 1 });

        assert_eq!(reader.get_char(), Ok('r'));
        assert_eq!(
            reader.get_position(),
            &Position {
                line: 10,
                column: 1
            }
        );

        assert_eq!(reader.get_char(), Ok('l'));
        assert_eq!(
            reader.get_position(),
            &Position {
                line: 11,
                column: 1
            }
        );

        assert_eq!(reader.get_char(), Ok('d'));
        assert_eq!(
            reader.get_position(),
            &Position {
                line: 12,
                column: 1
            }
        );

        assert_eq!(reader.get_char(), Ok('!'));
        assert_eq!(
            reader.get_position(),
            &Position {
                line: 13,
                column: 1
            }
        );

        assert_eq!(reader.get_char(), Err(Error::Eof));
        assert_eq!(reader.get_char(), Err(Error::Eof));
        assert_eq!(reader.get_char(), Err(Error::Eof));
    }
}
