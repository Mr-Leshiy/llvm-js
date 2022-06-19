pub use char_reader::CharReader;
pub use position::Position;
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
}

pub fn get_token<R: std::io::Read>(reader: &mut CharReader<R>) -> Result<Token, Error> {
    match reader.get_char() {
        Ok(mut char) => {
            // Skip any whitespaces
            while is_skip(&char) {
                char = match reader.get_char() {
                    Ok(char) => char,
                    Err(e) if e == char_reader::Error::Eof => return Ok(Token::Eof),
                    Err(e) => return Err(Error::ReaderError(e)),
                };
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
                        if !can_stop(&char) {
                            return Err(Error::UnexpectedSymbol(
                                char,
                                reader.get_position().clone(),
                            ));
                        }
                        reader.save(char);
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
                    char = match reader.get_char() {
                        Ok(char) => char,
                        Err(e) if e == char_reader::Error::Eof => break,
                        Err(e) => return Err(Error::ReaderError(e)),
                    };
                    if !char.is_ascii_digit() && char != '.' {
                        if !can_stop(&char) {
                            return Err(Error::UnexpectedSymbol(
                                char,
                                reader.get_position().clone(),
                            ));
                        }
                        reader.save(char);
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
                _ => {}
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
                        if !can_stop(&char) {
                            return Err(Error::UnexpectedSymbol(
                                char,
                                reader.get_position().clone(),
                            ));
                        }
                        reader.save(char);
                        break;
                    }

                    string.push(char);
                }

                return Ok(Token::Literal(Literal::String(string)));
            }

            Err(Error::UnexpectedSymbol(char, reader.get_position().clone()))
        }
        Err(e) if e == char_reader::Error::Eof => Ok(Token::Eof),
        Err(e) => Err(Error::ReaderError(e)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn token_ident_test() {
        let mut reader = CharReader::new("name1".as_bytes());
        assert_eq!(
            get_token(&mut reader),
            Ok(Token::Ident("name1".to_string()))
        );
        assert_eq!(get_token(&mut reader), Ok(Token::Eof));

        let mut reader = CharReader::new("name12name".as_bytes());
        assert_eq!(
            get_token(&mut reader),
            Ok(Token::Ident("name12name".to_string()))
        );
        assert_eq!(get_token(&mut reader), Ok(Token::Eof));

        let mut reader = CharReader::new("name_1".as_bytes());
        assert_eq!(
            get_token(&mut reader),
            Ok(Token::Ident("name_1".to_string()))
        );
        assert_eq!(get_token(&mut reader), Ok(Token::Eof));

        let mut reader = CharReader::new("name^2name".as_bytes());
        assert_eq!(
            get_token(&mut reader),
            Err(Error::UnexpectedSymbol(
                '^',
                Position { line: 5, column: 0 }
            ))
        );
    }

    #[test]
    fn token_assign_test() {
        let mut reader = CharReader::new("=".as_bytes());

        assert_eq!(get_token(&mut reader), Ok(Token::Assign));
        assert_eq!(get_token(&mut reader), Ok(Token::Eof));

        let mut reader = CharReader::new("name_1=name_2".as_bytes());
        assert_eq!(
            get_token(&mut reader),
            Ok(Token::Ident("name_1".to_string()))
        );
        assert_eq!(get_token(&mut reader), Ok(Token::Assign));
        assert_eq!(
            get_token(&mut reader),
            Ok(Token::Ident("name_2".to_string()))
        );
        assert_eq!(get_token(&mut reader), Ok(Token::Eof));
    }

    #[test]
    fn token_unexpected_symbol_test() {
        let mut reader = CharReader::new("^".as_bytes());

        assert_eq!(
            get_token(&mut reader),
            Err(Error::UnexpectedSymbol(
                '^',
                Position { line: 1, column: 0 }
            ))
        );
        assert_eq!(get_token(&mut reader), Ok(Token::Eof));
    }

    #[test]
    fn token_from_file_test() {
        let file = std::fs::File::open("test_scripts/basic.js").unwrap();
        let mut reader = CharReader::new(file);

        // line: "function foo() {}"
        assert_eq!(
            get_token(&mut reader),
            Ok(Token::Keyword(Keyword::Function))
        );
        assert_eq!(get_token(&mut reader), Ok(Token::Ident("foo".to_string())));
        assert_eq!(
            get_token(&mut reader),
            Ok(Token::Separator(Separator::OpenBrace))
        );
        assert_eq!(
            get_token(&mut reader),
            Ok(Token::Separator(Separator::CloseBrace))
        );
        assert_eq!(
            get_token(&mut reader),
            Ok(Token::Separator(Separator::OpenCurlyBrace))
        );
        assert_eq!(
            get_token(&mut reader),
            Ok(Token::Separator(Separator::CloseCurlyBrace))
        );

        //line: "{"
        assert_eq!(
            get_token(&mut reader),
            Ok(Token::Separator(Separator::OpenCurlyBrace))
        );

        //line: "var a = 5;"
        assert_eq!(get_token(&mut reader), Ok(Token::Keyword(Keyword::Var)));
        assert_eq!(get_token(&mut reader), Ok(Token::Ident("a".to_string())));
        assert_eq!(get_token(&mut reader), Ok(Token::Assign));
        assert_eq!(
            get_token(&mut reader),
            Ok(Token::Literal(Literal::Number(5_f64)))
        );

        //line: "var b = 6;"
        assert_eq!(get_token(&mut reader), Ok(Token::Keyword(Keyword::Var)));
        assert_eq!(get_token(&mut reader), Ok(Token::Ident("b".to_string())));
        assert_eq!(get_token(&mut reader), Ok(Token::Assign));
        assert_eq!(
            get_token(&mut reader),
            Ok(Token::Literal(Literal::Number(6_f64)))
        );

        //line: "{"
        assert_eq!(
            get_token(&mut reader),
            Ok(Token::Separator(Separator::OpenCurlyBrace))
        );

        //line: "a = b;"
        assert_eq!(get_token(&mut reader), Ok(Token::Ident("a".to_string())));
        assert_eq!(get_token(&mut reader), Ok(Token::Assign));
        assert_eq!(get_token(&mut reader), Ok(Token::Ident("b".to_string())));

        //line: "b = 7;"
        assert_eq!(get_token(&mut reader), Ok(Token::Ident("b".to_string())));
        assert_eq!(get_token(&mut reader), Ok(Token::Assign));
        assert_eq!(
            get_token(&mut reader),
            Ok(Token::Literal(Literal::Number(7_f64)))
        );

        //line: "var c = "hello";"
        assert_eq!(get_token(&mut reader), Ok(Token::Keyword(Keyword::Var)));
        assert_eq!(get_token(&mut reader), Ok(Token::Ident("c".to_string())));
        assert_eq!(get_token(&mut reader), Ok(Token::Assign));
        assert_eq!(
            get_token(&mut reader),
            Ok(Token::Literal(Literal::String("hello".to_string())))
        );

        //line: "}"
        assert_eq!(
            get_token(&mut reader),
            Ok(Token::Separator(Separator::CloseCurlyBrace))
        );

        //line: "}"
        assert_eq!(
            get_token(&mut reader),
            Ok(Token::Separator(Separator::CloseCurlyBrace))
        );

        assert_eq!(get_token(&mut reader), Ok(Token::Eof));
    }
}
