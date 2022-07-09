pub use keyword::Keyword;
pub use literal::Literal;
pub use separator::Separator;
use std::fmt::Display;

mod keyword;
mod literal;
mod separator;

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    /// keyword token
    Keyword(Keyword),
    /// assign token, "="
    Assign,
    /// ident token, e.g. "val1", "car_type"
    Ident(String),
    /// literal token
    Literal(Literal),
    /// separator token,
    Separator(Separator),
    /// end of file token
    Eof,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Keyword(val) => val.fmt(f),
            Self::Assign => write!(f, "Assign token"),
            Self::Ident(val) => write!(f, "Ident token, val: {}", val),
            Self::Literal(val) => val.fmt(f),
            Self::Separator(val) => val.fmt(f),
            Self::Eof => write!(f, "Eof token"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Error, Position, TokenReader};

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
}
