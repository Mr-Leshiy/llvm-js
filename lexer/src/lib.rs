use char_reader::CharReader;
pub use position::Position;
use std::io::Read;
use thiserror::Error;
pub use tokens::{Keyword, Literal, Separator, Token};

mod char_reader;
mod position;
mod tokens;

#[derive(Debug, Error, PartialEq)]
pub enum Error {
    #[error("Reader error: {0}")]
    ReaderError(char_reader::Error),
    #[error("Unected symbol: {0}, position: {1}")]
    UnexpectedSymbol(char, Position),
    #[error("Unexpected token provided: {0}")]
    UnexpectedToken(Token),
}

pub trait Parser: Sized {
    fn parse<R: Read>(cur_token: Token, reader: &mut TokenReader<R>) -> Result<Self, Error>;
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
}

enum TokenResult<T> {
    Result(T),
    Token(Token),
}

impl<T> TokenResult<T> {
    fn token_or_continue<F>(self, f: F) -> Result<Token, Error>
    where
        F: FnOnce(T) -> Result<Token, Error>,
    {
        match self {
            TokenResult::Result(val) => f(val),
            TokenResult::Token(token) => Ok(token),
        }
    }
}

impl<R: Read> TokenReader<R> {
    // Skip any whitespaces
    fn try_skip_whitespaces(&mut self, mut char: char) -> Result<TokenResult<char>, Error> {
        while is_skip(&char) {
            char = match self.char_reader.get_char() {
                Ok(char) => char,
                Err(e) if e == char_reader::Error::Eof => {
                    return Ok(TokenResult::Token(Token::Eof))
                }
                Err(e) => return Err(Error::ReaderError(e)),
            };
        }
        Ok(TokenResult::Result(char))
    }

    // try read identifier: [a-zA-Z][a-zA-Z0-9_]*
    fn try_read_identifier(&mut self, mut char: char) -> Result<TokenResult<()>, Error> {
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
                return Ok(TokenResult::Token(Token::Keyword(Keyword::Var)));
            }

            if ident == "function" {
                return Ok(TokenResult::Token(Token::Keyword(Keyword::Function)));
            }

            if ident == "true" {
                return Ok(TokenResult::Token(Token::Literal(Literal::True)));
            }

            if ident == "false" {
                return Ok(TokenResult::Token(Token::Literal(Literal::False)));
            }

            return Ok(TokenResult::Token(Token::Ident(ident)));
        }
        Ok(TokenResult::Result(()))
    }

    // try read number: [0-9.]+
    fn try_read_number(&mut self, mut char: char) -> Result<TokenResult<()>, Error> {
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

            return Ok(TokenResult::Token(Token::Literal(Literal::Number(
                number.parse().expect("string should be f64 number"),
            ))));
        }
        Ok(TokenResult::Result(()))
    }

    // try read assign operator: '='
    fn try_read_assign_operator(&mut self, char: char) -> Result<TokenResult<()>, Error> {
        if char == '=' {
            return Ok(TokenResult::Token(Token::Assign));
        }
        Ok(TokenResult::Result(()))
    }

    // try read separator: '(',')','{','}','[',']'
    fn try_read_separator(&mut self, char: char) -> Result<TokenResult<()>, Error> {
        match char {
            '(' => Ok(TokenResult::Token(Token::Separator(Separator::OpenBrace))),
            ')' => Ok(TokenResult::Token(Token::Separator(Separator::CloseBrace))),
            '{' => Ok(TokenResult::Token(Token::Separator(
                Separator::OpenCurlyBrace,
            ))),
            '}' => Ok(TokenResult::Token(Token::Separator(
                Separator::CloseCurlyBrace,
            ))),
            '[' => Ok(TokenResult::Token(Token::Separator(
                Separator::OpenSquareBracket,
            ))),
            ']' => Ok(TokenResult::Token(Token::Separator(
                Separator::CloseSquareBracket,
            ))),
            ',' => Ok(TokenResult::Token(Token::Separator(Separator::Comma))),
            _ => Ok(TokenResult::Result(())),
        }
    }

    // try read string: "<any symbol>"
    fn try_read_string(&mut self, mut char: char) -> Result<TokenResult<()>, Error> {
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

            return Ok(TokenResult::Token(Token::Literal(Literal::String(string))));
        }
        Ok(TokenResult::Result(()))
    }
}

impl<R: Read> TokenReader<R> {
    pub fn start_saving(&mut self) {
        self.saved_flag = true;
    }

    pub fn stop_saving(&mut self) {
        self.saved_flag = false;
    }

    pub fn reset_saving(&mut self) {
        self.saved_flag = false;
        self.saved_tokens.clear();
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
            Ok(char) => self.try_skip_whitespaces(char)?.token_or_continue(|char| {
                self.try_read_identifier(char)?.token_or_continue(|_| {
                    self.try_read_number(char)?.token_or_continue(|_| {
                        self.try_read_assign_operator(char)?.token_or_continue(|_| {
                            self.try_read_separator(char)?.token_or_continue(|_| {
                                self.try_read_string(char)?.token_or_continue(|_| {
                                    Err(Error::UnexpectedSymbol(
                                        char,
                                        self.char_reader.get_position().clone(),
                                    ))
                                })
                            })
                        })
                    })
                })
            }),
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
    fn token_from_file_test() {
        let file = std::fs::File::open("../test_scripts/basic.js").unwrap();
        let mut reader = TokenReader::new(file);

        // line: "function foo(arg1, arg2) { arg1 = 12; }"
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
        assert_eq!(reader.read_token(), Ok(Token::Ident("arg1".to_string())));
        assert_eq!(reader.read_token(), Ok(Token::Assign));
        assert_eq!(
            reader.read_token(),
            Ok(Token::Literal(Literal::Number(12_f64)))
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
