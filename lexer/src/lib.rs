use char_reader::CharReader;
pub use position::Position;
use std::io::Read;
use thiserror::Error;
pub use tokens::{Arithmetic, Keyword, Literal, Logical, Separator, Token};

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
        || char.eq(&'&')
        || char.eq(&'|')
        || char.eq(&'!')
        || char.eq(&'+')
        || char.eq(&'-')
        || char.eq(&'*')
        || char.eq(&'/')
}

pub struct TokenReader<R: Read> {
    char_reader: CharReader<R>,
    // used as FIFO collection
    saved_tokens: Vec<Vec<Token>>,
    next_to_read_tokens: Vec<Vec<Token>>,
    saved_flag: u8,
}

impl<R: Read> TokenReader<R> {
    pub fn new(reader: R) -> Self {
        Self {
            char_reader: CharReader::new(reader),
            saved_tokens: Vec::new(),
            next_to_read_tokens: Vec::new(),
            saved_flag: 0,
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

    // Skip comments as "// ... ", "/* ... */"
    fn try_skip_comments(&mut self, char: char) -> Result<TokenResult<char>, Error> {
        if char == '/' {
            match self.char_reader.get_char() {
                Ok(mut char) => match char {
                    // handle "// ... " case
                    '/' => {
                        while char != '\n' {
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
                    // handle "/* ... */" case
                    '*' => {
                        loop {
                            char = match self.char_reader.get_char() {
                                Ok(char) => char,
                                Err(e) if e == char_reader::Error::Eof => {
                                    return Ok(TokenResult::Token(Token::Eof))
                                }
                                Err(e) => return Err(Error::ReaderError(e)),
                            };
                            if char == '*' {
                                break;
                            }
                        }
                        let position = self.char_reader.get_position().clone();
                        match self.char_reader.get_char() {
                            Ok('/') => {
                                char = match self.char_reader.get_char() {
                                    Ok(char) => char,
                                    Err(e) if e == char_reader::Error::Eof => {
                                        return Ok(TokenResult::Token(Token::Eof))
                                    }
                                    Err(e) => return Err(Error::ReaderError(e)),
                                };
                            }
                            Ok(char) => {
                                return Err(Error::UnexpectedSymbol(
                                    char,
                                    self.char_reader.get_position().clone(),
                                ))
                            }
                            Err(e) if e == char_reader::Error::Eof => {
                                return Err(Error::UnexpectedSymbol('*', position))
                            }
                            Err(e) => return Err(Error::ReaderError(e)),
                        };
                        Ok(TokenResult::Result(char))
                    }
                    char => {
                        self.char_reader.save(char);
                        Ok(TokenResult::Result('/'))
                    }
                },
                Err(e) if e == char_reader::Error::Eof => Ok(TokenResult::Result('/')),
                Err(e) => Err(Error::ReaderError(e)),
            }
        } else {
            Ok(TokenResult::Result(char))
        }
    }

    fn try_skip(&mut self, mut char: char) -> Result<TokenResult<char>, Error> {
        let mut res;
        loop {
            let cur_position = self.char_reader.get_position().clone();
            res = match self.try_skip_whitespaces(char)? {
                TokenResult::Result(char) => self.try_skip_comments(char),
                result => Ok(result),
            };
            char = match res {
                Ok(TokenResult::Result(char)) => char,
                result => return result,
            };
            if cur_position == self.char_reader.get_position().clone() {
                break;
            }
        }
        res
    }

    // try read identifier: [a-zA-Z][a-zA-Z0-9_]*
    fn try_read_identifier(&mut self, mut char: char) -> Result<TokenResult<()>, Error> {
        if char.is_ascii_alphabetic() || char == '_' {
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

            if ident == "let" {
                return Ok(TokenResult::Token(Token::Keyword(Keyword::Let)));
            }

            if ident == "function" {
                return Ok(TokenResult::Token(Token::Keyword(Keyword::Function)));
            }

            if ident == "return" {
                return Ok(TokenResult::Token(Token::Keyword(Keyword::Return)));
            }

            if ident == "if" {
                return Ok(TokenResult::Token(Token::Keyword(Keyword::If)));
            }

            if ident == "else" {
                return Ok(TokenResult::Token(Token::Keyword(Keyword::Else)));
            }

            if ident == "while" {
                return Ok(TokenResult::Token(Token::Keyword(Keyword::While)));
            }

            if ident == "true" {
                return Ok(TokenResult::Token(Token::Literal(Literal::Boolean(true))));
            }

            if ident == "false" {
                return Ok(TokenResult::Token(Token::Literal(Literal::Boolean(false))));
            }

            if ident == "undefined" {
                return Ok(TokenResult::Token(Token::Literal(Literal::Undefined)));
            }

            if ident == "null" {
                return Ok(TokenResult::Token(Token::Literal(Literal::Null)));
            }

            if ident == "NaN" {
                return Ok(TokenResult::Token(Token::Literal(Literal::NaN)));
            }

            if ident == "Infinity" {
                return Ok(TokenResult::Token(Token::Literal(Literal::Infinity)));
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

    // try read assign operator
    fn try_read_assign_operator(&mut self, char: char) -> Result<TokenResult<()>, Error> {
        if char == '=' {
            return Ok(TokenResult::Token(Token::Assign));
        }
        Ok(TokenResult::Result(()))
    }

    // try read logical
    fn try_read_logical(&mut self, mut char: char) -> Result<TokenResult<()>, Error> {
        if char == '=' {
            char = match self.char_reader.get_char() {
                Ok(char) => char,
                Err(e) if e == char_reader::Error::Eof => return Ok(TokenResult::Result(())),
                Err(e) => return Err(Error::ReaderError(e)),
            };
            if char == '=' {
                char = match self.char_reader.get_char() {
                    Ok(char) => char,
                    Err(e) if e == char_reader::Error::Eof => {
                        return Ok(TokenResult::Token(Token::Logical(Logical::Eq)))
                    }
                    Err(e) => return Err(Error::ReaderError(e)),
                };
                if char == '=' {
                    return Ok(TokenResult::Token(Token::Logical(Logical::SEq)));
                }
                self.char_reader.save(char);
                return Ok(TokenResult::Token(Token::Logical(Logical::Eq)));
            }
            self.char_reader.save(char);
        }

        if char == '!' {
            char = match self.char_reader.get_char() {
                Ok(char) => char,
                Err(e) if e == char_reader::Error::Eof => {
                    return Ok(TokenResult::Token(Token::Logical(Logical::Not)))
                }
                Err(e) => return Err(Error::ReaderError(e)),
            };
            if char == '=' {
                char = match self.char_reader.get_char() {
                    Ok(char) => char,
                    Err(e) if e == char_reader::Error::Eof => {
                        return Ok(TokenResult::Token(Token::Logical(Logical::Ne)))
                    }
                    Err(e) => return Err(Error::ReaderError(e)),
                };
                if char == '=' {
                    return Ok(TokenResult::Token(Token::Logical(Logical::SNe)));
                }
                self.char_reader.save(char);
                return Ok(TokenResult::Token(Token::Logical(Logical::Ne)));
            }

            self.char_reader.save(char);
            return Ok(TokenResult::Token(Token::Logical(Logical::Not)));
        }
        if char == '&' {
            let postion = self.char_reader.get_position().clone();
            char = match self.char_reader.get_char() {
                Ok(char) => char,
                Err(e) if e == char_reader::Error::Eof => {
                    return Err(Error::UnexpectedSymbol('&', postion))
                }
                Err(e) => return Err(Error::ReaderError(e)),
            };
            if char == '&' {
                return Ok(TokenResult::Token(Token::Logical(Logical::And)));
            } else {
                return Err(Error::UnexpectedSymbol('&', postion));
            }
        }
        if char == '|' {
            let postion = self.char_reader.get_position().clone();
            char = match self.char_reader.get_char() {
                Ok(char) => char,
                Err(e) if e == char_reader::Error::Eof => {
                    return Err(Error::UnexpectedSymbol('|', postion))
                }
                Err(e) => return Err(Error::ReaderError(e)),
            };
            if char == '|' {
                return Ok(TokenResult::Token(Token::Logical(Logical::Or)));
            } else {
                return Err(Error::UnexpectedSymbol('|', postion));
            }
        }
        Ok(TokenResult::Result(()))
    }

    // try read arithmetic
    fn try_read_arithmetic(&mut self, char: char) -> Result<TokenResult<()>, Error> {
        if char == '+' {
            return Ok(TokenResult::Token(Token::Arithmetic(Arithmetic::Add)));
        }
        if char == '-' {
            return Ok(TokenResult::Token(Token::Arithmetic(Arithmetic::Sub)));
        }
        if char == '*' {
            return Ok(TokenResult::Token(Token::Arithmetic(Arithmetic::Mul)));
        }
        if char == '/' {
            return Ok(TokenResult::Token(Token::Arithmetic(Arithmetic::Div)));
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
    pub fn start_saving(&mut self) -> u8 {
        self.saved_flag += 1;
        self.saved_tokens.push(Vec::new());
        self.saved_flag
    }

    pub fn stop_saving(&mut self) {
        if self.saved_flag != 0 {
            self.saved_flag -= 1;
            let saved = self.saved_tokens.pop().unwrap();
            if !saved.is_empty() {
                self.next_to_read_tokens.push(saved);
            }
        }
    }

    pub fn reset_saving(&mut self) {
        if self.saved_flag != 0 {
            self.saved_flag -= 1;
        }
        self.saved_tokens.pop();
    }

    pub fn next_token(&mut self) -> Result<Token, Error> {
        let token = if let Some(next_to_read) = self.next_to_read_tokens.first_mut() {
            if next_to_read.is_empty() {
                self.next_to_read_tokens.remove(0);
                self.read_token()?
            } else {
                // remove first element
                let token = next_to_read.remove(0);
                if next_to_read.is_empty() {
                    self.next_to_read_tokens.remove(0);
                }
                token
            }
        } else {
            self.read_token()?
        };
        if self.saved_flag > 0 {
            self.saved_tokens
                .last_mut()
                .expect("saved tokens should not be empty")
                .push(token.clone());
        }
        Ok(token)
    }

    fn read_token(&mut self) -> Result<Token, Error> {
        match self.char_reader.get_char() {
            Ok(char) => self.try_skip(char)?.token_or_continue(|char| {
                self.try_read_identifier(char)?.token_or_continue(|_| {
                    self.try_read_number(char)?.token_or_continue(|_| {
                        self.try_read_logical(char)?.token_or_continue(|_| {
                            self.try_read_arithmetic(char)?.token_or_continue(|_| {
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
    fn token_reader_saving_test1() {
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
    fn token_reader_saving_test2() {
        let mut reader = TokenReader::new("name1 name2 name3 name4".as_bytes());

        assert_eq!(reader.next_token(), Ok(Token::Ident("name1".to_string())));

        reader.start_saving();

        assert_eq!(reader.next_token(), Ok(Token::Ident("name2".to_string())));

        reader.start_saving();

        assert_eq!(reader.next_token(), Ok(Token::Ident("name3".to_string())));
        assert_eq!(reader.next_token(), Ok(Token::Ident("name4".to_string())));

        reader.stop_saving();

        assert_eq!(reader.next_token(), Ok(Token::Ident("name3".to_string())));
        assert_eq!(reader.next_token(), Ok(Token::Ident("name4".to_string())));

        assert_eq!(reader.next_token(), Ok(Token::Eof));

        reader.stop_saving();

        assert_eq!(reader.next_token(), Ok(Token::Ident("name2".to_string())));
        assert_eq!(reader.next_token(), Ok(Token::Ident("name3".to_string())));
        assert_eq!(reader.next_token(), Ok(Token::Ident("name4".to_string())));

        assert_eq!(reader.next_token(), Ok(Token::Eof));
    }

    #[test]
    fn token_reader_saving_test3() {
        let mut reader = TokenReader::new("name1 name2 name3 name4".as_bytes());

        assert_eq!(reader.next_token(), Ok(Token::Ident("name1".to_string())));

        reader.start_saving();

        assert_eq!(reader.next_token(), Ok(Token::Ident("name2".to_string())));
        assert_eq!(reader.next_token(), Ok(Token::Ident("name3".to_string())));
        assert_eq!(reader.next_token(), Ok(Token::Ident("name4".to_string())));

        reader.stop_saving();

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
    fn try_skip_comments_test() {
        let mut reader = TokenReader::new("name1 \n // name2 name3 \n name1".as_bytes());
        assert_eq!(reader.read_token(), Ok(Token::Ident("name1".to_string())));
        assert_eq!(reader.read_token(), Ok(Token::Ident("name1".to_string())));
        assert_eq!(reader.read_token(), Ok(Token::Eof));

        let mut reader = TokenReader::new("name1 /* name2 name3 */ name1".as_bytes());
        assert_eq!(reader.read_token(), Ok(Token::Ident("name1".to_string())));
        assert_eq!(reader.read_token(), Ok(Token::Ident("name1".to_string())));
        assert_eq!(reader.read_token(), Ok(Token::Eof));

        let mut reader = TokenReader::new("// name1 \n /* name2 name3 */".as_bytes());
        assert_eq!(reader.read_token(), Ok(Token::Eof));
    }
}
