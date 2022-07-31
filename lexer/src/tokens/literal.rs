use std::fmt::Display;

#[derive(Clone, Debug, PartialEq)]
pub enum Literal {
    /// boolean token, e.g. true or false
    Boolean(bool),
    /// number token, e.g. 5, 6, 6.12
    Number(f64),
    /// string token, e.g. "hello^world!"
    String(String),
}

impl Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Boolean(val) => write!(f, "Literal boolean token, val: {}", val),
            Self::Number(val) => write!(f, "Literal number token, val: {}", val),
            Self::String(val) => write!(f, "Literal string token, val: {}", val),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Error, Position, Token, TokenReader};

    #[test]
    fn literal_boolean_test() {
        let mut reader = TokenReader::new("true".as_bytes());

        assert_eq!(
            reader.read_token(),
            Ok(Token::Literal(Literal::Boolean(true)))
        );
        assert_eq!(reader.read_token(), Ok(Token::Eof));

        let mut reader = TokenReader::new("false".as_bytes());

        assert_eq!(
            reader.read_token(),
            Ok(Token::Literal(Literal::Boolean(false)))
        );
        assert_eq!(reader.read_token(), Ok(Token::Eof));
    }

    #[test]
    fn literal_number_test() {
        let mut reader = TokenReader::new("12".as_bytes());
        assert_eq!(
            reader.read_token(),
            Ok(Token::Literal(Literal::Number(12_f64)))
        );
        assert_eq!(reader.read_token(), Ok(Token::Eof));

        let mut reader = TokenReader::new("12.145".as_bytes());
        assert_eq!(
            reader.read_token(),
            Ok(Token::Literal(Literal::Number(12.145)))
        );
        assert_eq!(reader.read_token(), Ok(Token::Eof));

        let mut reader = TokenReader::new("-12.145".as_bytes());
        assert_eq!(
            reader.read_token(),
            Ok(Token::Literal(Literal::Number(-12.145)))
        );
        assert_eq!(reader.read_token(), Ok(Token::Eof));

        let mut reader = TokenReader::new("1f2.145".as_bytes());
        assert_eq!(
            reader.read_token(),
            Err(Error::UnexpectedSymbol(
                'f',
                Position { line: 2, column: 0 }
            ))
        );
    }

    #[test]
    fn literal_string_test() {
        let mut reader = TokenReader::new(r#""Hello World__414f$$@#!@$$!%%!""#.as_bytes());

        assert_eq!(
            reader.read_token(),
            Ok(Token::Literal(Literal::String(
                "Hello World__414f$$@#!@$$!%%!".to_string()
            )))
        );
        assert_eq!(reader.read_token(), Ok(Token::Eof));
    }
}
