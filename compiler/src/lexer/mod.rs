use char_reader::CharReader;
pub use position::Position;
use std::io::Read;
use thiserror::Error;
pub use tokens::{Keyword, Literal, Separator, Token};

mod char_reader;
mod position;
mod tokens;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum Error {
    #[error("Reader error: {0}")]
    ReaderError(char_reader::Error),
    #[error("Unected symbol: {0}, position: {1}")]
    UnexpectedSymbol(char, Position),
}

fn is_skip(char: &char) -> bool {
    char.is_ascii_whitespace() || char.eq(&';')
}

fn can_stop(char: &char) -> bool {
    is_skip(char)
        || char.eq(&'=')
        || char.eq(&'(')
        || char.eq(&')')
        || char.eq(&'{')
        || char.eq(&'}')
        || char.eq(&'[')
        || char.eq(&']')
        || char.eq(&',')
}

pub struct TokenReader<R: Read> {
    char_reader: CharReader<R>,
    // used as FIFO collection
    saved_tokens: Vec<Token>,
    saved_flag: bool,
}

impl<R: Read> TokenReader<R> {
    pub fn new(reader: R) -> Self {
        Self {
            char_reader: CharReader::new(reader),
            saved_tokens: Vec::new(),
            saved_flag: false,
        }
    }

    pub fn start_saving(&mut self) {
        self.saved_flag = true;
    }

    pub fn stop_saving(&mut self) {
        self.saved_flag = false;
    }

    pub fn next_token(&mut self) -> Result<Token, Error> {
        if self.saved_flag {
            let token = self.read_token()?;
            self.saved_tokens.push(token.clone());
            Ok(token)
        } else if self.saved_tokens.is_empty() {
            self.read_token()
        } else {
            // remove first element
            Ok(self.saved_tokens.remove(0))
        }
    }

    fn read_token(&mut self) -> Result<Token, Error> {
        match self.char_reader.get_char() {
            Ok(mut char) => {
                // Skip any whitespaces
                while is_skip(&char) {
                    char = match self.char_reader.get_char() {
                        Ok(char) => char,
                        Err(e) if e == char_reader::Error::Eof => return Ok(Token::Eof),
                        Err(e) => return Err(Error::ReaderError(e)),
                    };
                }

                // identifier: [a-zA-Z][a-zA-Z0-9_]*
                if char.is_ascii_alphabetic() {
                    let mut ident = char.to_string();
                    loop {
                        char = match self.char_reader.get_char() {
                            Ok(char) => char,
                            Err(e) if e == char_reader::Error::Eof => break,
                            Err(e) => return Err(Error::ReaderError(e)),
                        };
                        if !char.is_ascii_alphanumeric() && char != '_' {
                            if !can_stop(&char) {
                                return Err(Error::UnexpectedSymbol(
                                    char,
                                    self.char_reader.get_position().clone(),
                                ));
                            }
                            self.char_reader.save(char);
                            break;
                        }

                        ident.push(char);
                    }

                    if ident == "var" {
                        return Ok(Token::Keyword(Keyword::Var));
                    }

                    if ident == "function" {
                        return Ok(Token::Keyword(Keyword::Function));
                    }

                    return Ok(Token::Ident(ident));
                }

                // Number: [0-9.]+
                if char.is_ascii_digit() {
                    let mut number = char.to_string();
                    loop {
                        char = match self.char_reader.get_char() {
                            Ok(char) => char,
                            Err(e) if e == char_reader::Error::Eof => break,
                            Err(e) => return Err(Error::ReaderError(e)),
                        };
                        if !char.is_ascii_digit() && char != '.' {
                            if !can_stop(&char) {
                                return Err(Error::UnexpectedSymbol(
                                    char,
                                    self.char_reader.get_position().clone(),
                                ));
                            }
                            self.char_reader.save(char);
                            break;
                        }

                        number.push(char);
                    }

                    return Ok(Token::Literal(Literal::Number(
                        number.parse().expect("string should be f64 number"),
                    )));
                }

                // assign operator: '='
                if char == '=' {
                    return Ok(Token::Assign);
                }

                // separator: '(',')','{','}','[',']'
                match char {
                    '(' => return Ok(Token::Separator(Separator::OpenBrace)),
                    ')' => return Ok(Token::Separator(Separator::CloseBrace)),
                    '{' => return Ok(Token::Separator(Separator::OpenCurlyBrace)),
                    '}' => return Ok(Token::Separator(Separator::CloseCurlyBrace)),
                    '[' => return Ok(Token::Separator(Separator::OpenSquareBracket)),
                    ']' => return Ok(Token::Separator(Separator::CloseSquareBracket)),
                    ',' => return Ok(Token::Separator(Separator::Comma)),
                    _ => {}
                }

                // String: string
                if char == '"' {
                    let mut string = String::new();
                    loop {
                        char = match self.char_reader.get_char() {
                            Ok(char) => char,
                            Err(e) if e == char_reader::Error::Eof => break,
                            Err(e) => return Err(Error::ReaderError(e)),
                        };
                        if char == '"' {
                            char = match self.char_reader.get_char() {
                                Ok(char) => char,
                                Err(e) if e == char_reader::Error::Eof => break,
                                Err(e) => return Err(Error::ReaderError(e)),
                            };
                            // next symbol should be skipped symbol
                            if !can_stop(&char) {
                                return Err(Error::UnexpectedSymbol(
                                    char,
                                    self.char_reader.get_position().clone(),
                                ));
                            }
                            self.char_reader.save(char);
                            break;
                        }

                        string.push(char);
                    }

                    return Ok(Token::Literal(Literal::String(string)));
                }

                Err(Error::UnexpectedSymbol(
                    char,
                    self.char_reader.get_position().clone(),
                ))
            }
            Err(e) if e == char_reader::Error::Eof => Ok(Token::Eof),
            Err(e) => Err(Error::ReaderError(e)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn token_reader_test() {
        let mut reader = TokenReader::new("name1 name2 name3 name4".as_bytes());

        assert_eq!(reader.next_token(), Ok(Token::Ident("name1".to_string())));

        reader.start_saving();

        assert_eq!(reader.next_token(), Ok(Token::Ident("name2".to_string())));
        assert_eq!(reader.next_token(), Ok(Token::Ident("name3".to_string())));
        assert_eq!(reader.next_token(), Ok(Token::Ident("name4".to_string())));

        reader.stop_saving();

        assert_eq!(reader.next_token(), Ok(Token::Ident("name2".to_string())));
        assert_eq!(reader.next_token(), Ok(Token::Ident("name3".to_string())));
        assert_eq!(reader.next_token(), Ok(Token::Ident("name4".to_string())));

        assert_eq!(reader.next_token(), Ok(Token::Eof));
    }

    #[test]
    fn token_ident_test() {
        let mut reader = TokenReader::new("name1".as_bytes());
        assert_eq!(reader.read_token(), Ok(Token::Ident("name1".to_string())));
        assert_eq!(reader.read_token(), Ok(Token::Eof));

        let mut reader = TokenReader::new("name12name".as_bytes());
        assert_eq!(
            reader.read_token(),
            Ok(Token::Ident("name12name".to_string()))
        );
        assert_eq!(reader.read_token(), Ok(Token::Eof));

        let mut reader = TokenReader::new("name_1".as_bytes());
        assert_eq!(reader.read_token(), Ok(Token::Ident("name_1".to_string())));
        assert_eq!(reader.read_token(), Ok(Token::Eof));

        let mut reader = TokenReader::new("name^2name".as_bytes());
        assert_eq!(
            reader.read_token(),
            Err(Error::UnexpectedSymbol(
                '^',
                Position { line: 5, column: 0 }
            ))
        );
    }

    #[test]
    fn token_assign_test() {
        let mut reader = TokenReader::new("=".as_bytes());

        assert_eq!(reader.read_token(), Ok(Token::Assign));
        assert_eq!(reader.read_token(), Ok(Token::Eof));

        let mut reader = TokenReader::new("name_1=name_2".as_bytes());
        assert_eq!(reader.read_token(), Ok(Token::Ident("name_1".to_string())));
        assert_eq!(reader.read_token(), Ok(Token::Assign));
        assert_eq!(reader.read_token(), Ok(Token::Ident("name_2".to_string())));
        assert_eq!(reader.read_token(), Ok(Token::Eof));
    }

    #[test]
    fn token_unexpected_symbol_test() {
        let mut reader = TokenReader::new("^".as_bytes());

        assert_eq!(
            reader.read_token(),
            Err(Error::UnexpectedSymbol(
                '^',
                Position { line: 1, column: 0 }
            ))
        );
        assert_eq!(reader.read_token(), Ok(Token::Eof));
    }

    #[test]
    fn token_from_file_test() {
        let file = std::fs::File::open("test_scripts/basic.js").unwrap();
        let mut reader = TokenReader::new(file);

        // line: "function foo(arg1, arg2) {}"
        assert_eq!(reader.read_token(), Ok(Token::Keyword(Keyword::Function)));
        assert_eq!(reader.read_token(), Ok(Token::Ident("foo".to_string())));
        assert_eq!(
            reader.read_token(),
            Ok(Token::Separator(Separator::OpenBrace))
        );
        assert_eq!(reader.read_token(), Ok(Token::Ident("arg1".to_string())));
        assert_eq!(reader.read_token(), Ok(Token::Separator(Separator::Comma)));
        assert_eq!(reader.read_token(), Ok(Token::Ident("arg2".to_string())));
        assert_eq!(
            reader.read_token(),
            Ok(Token::Separator(Separator::CloseBrace))
        );
        assert_eq!(
            reader.read_token(),
            Ok(Token::Separator(Separator::OpenCurlyBrace))
        );
        assert_eq!(
            reader.read_token(),
            Ok(Token::Separator(Separator::CloseCurlyBrace))
        );

        //line: "{"
        assert_eq!(
            reader.read_token(),
            Ok(Token::Separator(Separator::OpenCurlyBrace))
        );

        //line: "var a = 5;"
        assert_eq!(reader.read_token(), Ok(Token::Keyword(Keyword::Var)));
        assert_eq!(reader.read_token(), Ok(Token::Ident("a".to_string())));
        assert_eq!(reader.read_token(), Ok(Token::Assign));
        assert_eq!(
            reader.read_token(),
            Ok(Token::Literal(Literal::Number(5_f64)))
        );

        //line: "var b = 6;"
        assert_eq!(reader.read_token(), Ok(Token::Keyword(Keyword::Var)));
        assert_eq!(reader.read_token(), Ok(Token::Ident("b".to_string())));
        assert_eq!(reader.read_token(), Ok(Token::Assign));
        assert_eq!(
            reader.read_token(),
            Ok(Token::Literal(Literal::Number(6_f64)))
        );

        //line: "{"
        assert_eq!(
            reader.read_token(),
            Ok(Token::Separator(Separator::OpenCurlyBrace))
        );

        //line: "a = b;"
        assert_eq!(reader.read_token(), Ok(Token::Ident("a".to_string())));
        assert_eq!(reader.read_token(), Ok(Token::Assign));
        assert_eq!(reader.read_token(), Ok(Token::Ident("b".to_string())));

        //line: "b = 7;"
        assert_eq!(reader.read_token(), Ok(Token::Ident("b".to_string())));
        assert_eq!(reader.read_token(), Ok(Token::Assign));
        assert_eq!(
            reader.read_token(),
            Ok(Token::Literal(Literal::Number(7_f64)))
        );

        //line: "var c = "hello";"
        assert_eq!(reader.read_token(), Ok(Token::Keyword(Keyword::Var)));
        assert_eq!(reader.read_token(), Ok(Token::Ident("c".to_string())));
        assert_eq!(reader.read_token(), Ok(Token::Assign));
        assert_eq!(
            reader.read_token(),
            Ok(Token::Literal(Literal::String("hello".to_string())))
        );

        //line: "}"
        assert_eq!(
            reader.read_token(),
            Ok(Token::Separator(Separator::CloseCurlyBrace))
        );

        //line: "foo(a, b);"
        assert_eq!(reader.read_token(), Ok(Token::Ident("foo".to_string())));
        assert_eq!(
            reader.read_token(),
            Ok(Token::Separator(Separator::OpenBrace))
        );
        assert_eq!(reader.read_token(), Ok(Token::Ident("a".to_string())));
        assert_eq!(reader.read_token(), Ok(Token::Separator(Separator::Comma)));
        assert_eq!(reader.read_token(), Ok(Token::Ident("b".to_string())));
        assert_eq!(
            reader.read_token(),
            Ok(Token::Separator(Separator::CloseBrace))
        );

        //line: "}"
        assert_eq!(
            reader.read_token(),
            Ok(Token::Separator(Separator::CloseCurlyBrace))
        );

        assert_eq!(reader.read_token(), Ok(Token::Eof));
    }
}
