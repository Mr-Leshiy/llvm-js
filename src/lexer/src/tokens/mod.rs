pub use arithmetic::Arithmetic;
pub use keyword::Keyword;
pub use literal::Literal;
pub use logical::Logical;
pub use separator::Separator;
use std::fmt::Display;

mod arithmetic;
mod keyword;
mod literal;
mod logical;
mod separator;

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    /// keyword token
    Keyword(Keyword),
    /// assign token, "="
    Assign,
    // logical token
    Logical(Logical),
    // arithmetic token,
    Arithmetic(Arithmetic),
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
            Self::Logical(val) => val.fmt(f),
            Self::Arithmetic(val) => val.fmt(f),
            Self::Ident(val) => write!(f, "Ident token, val: {val}"),
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

        let mut reader = TokenReader::new("_name_1".as_bytes());
        assert_eq!(reader.read_token(), Ok(Token::Ident("_name_1".to_string())));
        assert_eq!(reader.read_token(), Ok(Token::Eof));

        let mut reader = TokenReader::new("name^2name".as_bytes());
        assert_eq!(
            reader.read_token(),
            Err(Error::UnexpectedSymbol('^', Position::new(6, 1)))
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
            Err(Error::UnexpectedSymbol('^', Position::new(2, 1)))
        );
        assert_eq!(reader.read_token(), Ok(Token::Eof));
    }
}
