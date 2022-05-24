use self::char_reader::CharReader;
use thiserror::Error;

mod char_reader;

#[derive(Debug, Error, PartialEq)]
enum Error {
    #[error("Reader error: {0}")]
    ReaderError(char_reader::Error),
    #[error("Unected symbol: {0}")]
    UnexpectedSymbol(char),
}

#[derive(Debug, PartialEq)]
enum Token {
    /// "var"
    Var,
    /// assign token, "="
    Assign,
    /// ident token, e.g. "val1", "car_type"
    Ident(String),
    /// number token, e.g. 5, 6, 6.12
    Number(f64),
    /// end of file token
    Eof,
}

impl Token {
    fn get_token<R: std::io::Read>(reader: &mut CharReader<R>) -> Result<Self, Error> {
        match reader.get_char() {
            Ok(mut char) => {
                // Skip any whitespaces
                while char.is_ascii_whitespace() {
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

                // identifier: [a-zA-Z][a-zA-Z0-9]*
                if char.is_ascii_alphabetic() {
                    let mut ident = char.to_string();
                    loop {
                        char = match reader.get_char() {
                            Ok(char) => char,
                            Err(e) if e == char_reader::Error::Eof => break,
                            Err(e) => return Err(Error::ReaderError(e)),
                        };
                        if !char.is_ascii_alphanumeric() {
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
                            break;
                        }

                        number.push(char);
                    }

                    return Ok(Self::Number(
                        number.parse().expect("string should be f64 number"),
                    ));
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
    }

    #[test]
    fn token_number() {
        let mut reader = CharReader::new("12".as_bytes());

        assert_eq!(Token::get_token(&mut reader), Ok(Token::Number(12_f64)));
        assert_eq!(Token::get_token(&mut reader), Ok(Token::Eof));

        let mut reader = CharReader::new("12.145".as_bytes());

        assert_eq!(Token::get_token(&mut reader), Ok(Token::Number(12.145)));
        assert_eq!(Token::get_token(&mut reader), Ok(Token::Eof));
    }
}
