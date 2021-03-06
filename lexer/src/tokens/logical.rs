use std::fmt::Display;

#[derive(Clone, Debug, PartialEq)]
pub enum Logical {
    /// logical negate, e.g. !
    Not,
    /// logical negate, e.g. &&
    And,
    /// logical negate, e.g. ||
    Or,
}

impl Display for Logical {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Not => write!(f, "Logical NOT operator"),
            Self::And => write!(f, "Logical AND operator"),
            Self::Or => write!(f, "Logical OR operator"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Token, TokenReader};

    #[test]
    fn logical_negate_test() {
        let mut reader = TokenReader::new("!".as_bytes());

        assert_eq!(reader.read_token(), Ok(Token::Logical(Logical::Not)));
        assert_eq!(reader.read_token(), Ok(Token::Eof));

        let mut reader = TokenReader::new("!a".as_bytes());

        assert_eq!(reader.read_token(), Ok(Token::Logical(Logical::Not)));
        assert_eq!(reader.read_token(), Ok(Token::Ident("a".to_string())));
        assert_eq!(reader.read_token(), Ok(Token::Eof));
    }

    #[test]
    fn logical_and_test() {
        let mut reader = TokenReader::new("&&".as_bytes());

        assert_eq!(reader.read_token(), Ok(Token::Logical(Logical::And)));
        assert_eq!(reader.read_token(), Ok(Token::Eof));

        let mut reader = TokenReader::new("a && b".as_bytes());

        assert_eq!(reader.read_token(), Ok(Token::Ident("a".to_string())));
        assert_eq!(reader.read_token(), Ok(Token::Logical(Logical::And)));
        assert_eq!(reader.read_token(), Ok(Token::Ident("b".to_string())));
        assert_eq!(reader.read_token(), Ok(Token::Eof));
    }

    #[test]
    fn logical_or_test() {
        let mut reader = TokenReader::new("||".as_bytes());

        assert_eq!(reader.read_token(), Ok(Token::Logical(Logical::Or)));
        assert_eq!(reader.read_token(), Ok(Token::Eof));

        let mut reader = TokenReader::new("a || b".as_bytes());

        assert_eq!(reader.read_token(), Ok(Token::Ident("a".to_string())));
        assert_eq!(reader.read_token(), Ok(Token::Logical(Logical::Or)));
        assert_eq!(reader.read_token(), Ok(Token::Ident("b".to_string())));
        assert_eq!(reader.read_token(), Ok(Token::Eof));
    }
}
