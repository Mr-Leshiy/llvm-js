use std::fmt::Display;

pub use self::char_reader::CharReader;
use thiserror::Error;

mod char_reader;

#[derive(Debug, Error, PartialEq)]
pub enum Error {
    #[error("Reader error: {0}")]
    ReaderError(char_reader::Error),
    #[error("Unected symbol: {0}")]
    UnexpectedSymbol(char),
}

#[derive(Debug, PartialEq)]
pub enum Token {
    /// "var"
    Var,
    /// assign token, "="
    Assign,
    /// ident token, e.g. "val1", "car_type"
    Ident(String),
    /// number token, e.g. 5, 6, 6.12
    Number(f64),
    /// string token, e.g. "hello^world!"
    String(String),
    /// end of file token
    Eof,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Var => write!(f, "Var token"),
            Self::Assign => write!(f, "Assign token"),
            Self::Ident(val) => write!(f, "Ident token, val: {}", val),
            Self::Number(val) => write!(f, "Number token, val: {}", val),
            Self::String(val) => write!(f, "String token, val: {}", val),
            Self::Eof => write!(f, "Eof token"),
        }
    }
}

fn is_skip(char: &char) -> bool {
    char.is_ascii_whitespace() || char.eq(&';')
}

impl Token {
    pub fn get_token<R: std::io::Read>(reader: &mut CharReader<R>) -> Result<Self, Error> {
        match reader.get_char() {
            Ok(mut char) => {
                // Skip any whitespaces
                while is_skip(&char) {
                    char = match reader.get_char() {
                        Ok(char) => char,
                        Err(e) if e == char_reader::Error::Eof => return Ok(Self::Eof),
                        Err(e) => return Err(Error::ReaderError(e)),
                    };
                }

                // assign operator: '='
                if char == '=' {
                    return Ok(Self::Assign);
                }

                // identifier: [a-zA-Z][a-zA-Z0-9_]*
                if char.is_ascii_alphabetic() {
                    let mut ident = char.to_string();
                    loop {
                        char = match reader.get_char() {
                            Ok(char) => char,
                            Err(e) if e == char_reader::Error::Eof => break,
                            Err(e) => return Err(Error::ReaderError(e)),
                        };
                        if !char.is_ascii_alphanumeric() && char != '_' {
                            // next symbol should be skipped symbol
                            if !is_skip(&char) {
                                return Err(Error::UnexpectedSymbol(char));
                            }
                            break;
                        }

                        ident.push(char);
                    }

                    if ident == "var" {
                        return Ok(Self::Var);
                    }
                    return Ok(Self::Ident(ident));
                }

                // Number: [0-9.]+
                if char.is_ascii_digit() {
                    let mut number = char.to_string();
                    loop {
                        char = match reader.get_char() {
                            Ok(char) => char,
                            Err(e) if e == char_reader::Error::Eof => break,
                            Err(e) => return Err(Error::ReaderError(e)),
                        };
                        if !char.is_ascii_digit() && char != '.' {
                            // next symbol should be skipped symbol
                            if !is_skip(&char) {
                                return Err(Error::UnexpectedSymbol(char));
                            }
                            break;
                        }

                        number.push(char);
                    }

                    return Ok(Self::Number(
                        number.parse().expect("string should be f64 number"),
                    ));
                }

                // String: string
                if char == '"' {
                    let mut string = String::new();
                    loop {
                        char = match reader.get_char() {
                            Ok(char) => char,
                            Err(e) if e == char_reader::Error::Eof => break,
                            Err(e) => return Err(Error::ReaderError(e)),
                        };
                        if char == '"' {
                            char = match reader.get_char() {
                                Ok(char) => char,
                                Err(e) if e == char_reader::Error::Eof => break,
                                Err(e) => return Err(Error::ReaderError(e)),
                            };
                            // next symbol should be skipped symbol
                            if !is_skip(&char) {
                                return Err(Error::UnexpectedSymbol(char));
                            }
                            break;
                        }

                        string.push(char);
                    }

                    return Ok(Self::String(string));
                }

                Err(Error::UnexpectedSymbol(char))
            }
            Err(e) if e == char_reader::Error::Eof => Ok(Self::Eof),
            Err(e) => Err(Error::ReaderError(e)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn token_var() {
        let mut reader = CharReader::new("var".as_bytes());

        assert_eq!(Token::get_token(&mut reader), Ok(Token::Var));
        assert_eq!(Token::get_token(&mut reader), Ok(Token::Eof));
    }

    #[test]
    fn token_assign() {
        let mut reader = CharReader::new("=".as_bytes());

        assert_eq!(Token::get_token(&mut reader), Ok(Token::Assign));
        assert_eq!(Token::get_token(&mut reader), Ok(Token::Eof));
    }

    #[test]
    fn token_ident() {
        let mut reader = CharReader::new("name1".as_bytes());

        assert_eq!(
            Token::get_token(&mut reader),
            Ok(Token::Ident("name1".to_string()))
        );
        assert_eq!(Token::get_token(&mut reader), Ok(Token::Eof));

        let mut reader = CharReader::new("name12name".as_bytes());

        assert_eq!(
            Token::get_token(&mut reader),
            Ok(Token::Ident("name12name".to_string()))
        );
        assert_eq!(Token::get_token(&mut reader), Ok(Token::Eof));

        let mut reader = CharReader::new("name_1".as_bytes());

        assert_eq!(
            Token::get_token(&mut reader),
            Ok(Token::Ident("name_1".to_string()))
        );
        assert_eq!(Token::get_token(&mut reader), Ok(Token::Eof));

        let mut reader = CharReader::new("name^2name".as_bytes());

        assert_eq!(
            Token::get_token(&mut reader),
            Err(Error::UnexpectedSymbol('^'))
        );
    }

    #[test]
    fn token_number() {
        let mut reader = CharReader::new("12".as_bytes());

        assert_eq!(Token::get_token(&mut reader), Ok(Token::Number(12_f64)));
        assert_eq!(Token::get_token(&mut reader), Ok(Token::Eof));

        let mut reader = CharReader::new("12.145".as_bytes());

        assert_eq!(Token::get_token(&mut reader), Ok(Token::Number(12.145)));
        assert_eq!(Token::get_token(&mut reader), Ok(Token::Eof));

        let mut reader = CharReader::new("1f2.145".as_bytes());

        assert_eq!(
            Token::get_token(&mut reader),
            Err(Error::UnexpectedSymbol('f'))
        );
    }

    #[test]
    fn token_string() {
        let mut reader = CharReader::new(r#""Hello World__414f$$@#!@$$!%%!""#.as_bytes());

        assert_eq!(
            Token::get_token(&mut reader),
            Ok(Token::String("Hello World__414f$$@#!@$$!%%!".to_string()))
        );
        assert_eq!(Token::get_token(&mut reader), Ok(Token::Eof));
    }

    #[test]
    fn token_unexpected_symbol() {
        let mut reader = CharReader::new("^".as_bytes());

        assert_eq!(
            Token::get_token(&mut reader),
            Err(Error::UnexpectedSymbol('^'))
        );
        assert_eq!(Token::get_token(&mut reader), Ok(Token::Eof));
    }

    #[test]
    fn token_from_file() {
        let file = std::fs::File::open("test_scripts/basic.js").unwrap();
        let mut reader = CharReader::new(file);

        //line: "var a = 5;"
        assert_eq!(Token::get_token(&mut reader), Ok(Token::Var));
        assert_eq!(
            Token::get_token(&mut reader),
            Ok(Token::Ident("a".to_string()))
        );
        assert_eq!(Token::get_token(&mut reader), Ok(Token::Assign));
        assert_eq!(Token::get_token(&mut reader), Ok(Token::Number(5_f64)));

        //line: "var b = 6;"
        assert_eq!(Token::get_token(&mut reader), Ok(Token::Var));
        assert_eq!(
            Token::get_token(&mut reader),
            Ok(Token::Ident("b".to_string()))
        );
        assert_eq!(Token::get_token(&mut reader), Ok(Token::Assign));
        assert_eq!(Token::get_token(&mut reader), Ok(Token::Number(6_f64)));

        //line: "a = b;"
        assert_eq!(
            Token::get_token(&mut reader),
            Ok(Token::Ident("a".to_string()))
        );
        assert_eq!(Token::get_token(&mut reader), Ok(Token::Assign));
        assert_eq!(
            Token::get_token(&mut reader),
            Ok(Token::Ident("b".to_string()))
        );

        //line: "b = 7;"
        assert_eq!(
            Token::get_token(&mut reader),
            Ok(Token::Ident("b".to_string()))
        );
        assert_eq!(Token::get_token(&mut reader), Ok(Token::Assign));
        assert_eq!(Token::get_token(&mut reader), Ok(Token::Number(7_f64)));

        //line: "var c = "hello";"
        assert_eq!(Token::get_token(&mut reader), Ok(Token::Var));
        assert_eq!(
            Token::get_token(&mut reader),
            Ok(Token::Ident("c".to_string()))
        );
        assert_eq!(Token::get_token(&mut reader), Ok(Token::Assign));
        assert_eq!(
            Token::get_token(&mut reader),
            Ok(Token::String("hello".to_string()))
        );

        assert_eq!(Token::get_token(&mut reader), Ok(Token::Eof));
    }
}
