use std::fmt::Display;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Logical {
    /// logical negate, e.g. !
    Not,
    /// logical and, e.g. &&
    And,
    /// logical or, e.g. ||
    Or,
    /// logical equality, e.g. ==
    Eq,
    /// logical inequality, e.g. !=
    Ne,
    /// logical strict equality, e.g. ===
    SEq,
    /// logical strict inequality, e.g. !==
    SNe,
}

impl Display for Logical {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Not => write!(f, "Logical NOT operator"),
            Self::And => write!(f, "Logical AND operator"),
            Self::Or => write!(f, "Logical OR operator"),
            Self::Eq => write!(f, "Logical EQ operator"),
            Self::Ne => write!(f, "Logical NE operator"),
            Self::SEq => write!(f, "Logical SEQ operator"),
            Self::SNe => write!(f, "Logical SNE operator"),
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

    #[test]
    fn logical_eq_test() {
        let mut reader = TokenReader::new("==".as_bytes());

        assert_eq!(reader.read_token(), Ok(Token::Logical(Logical::Eq)));
        assert_eq!(reader.read_token(), Ok(Token::Eof));

        let mut reader = TokenReader::new("a == b".as_bytes());

        assert_eq!(reader.read_token(), Ok(Token::Ident("a".to_string())));
        assert_eq!(reader.read_token(), Ok(Token::Logical(Logical::Eq)));
        assert_eq!(reader.read_token(), Ok(Token::Ident("b".to_string())));
        assert_eq!(reader.read_token(), Ok(Token::Eof));
    }

    #[test]
    fn logical_seq_test() {
        let mut reader = TokenReader::new("===".as_bytes());

        assert_eq!(reader.read_token(), Ok(Token::Logical(Logical::SEq)));
        assert_eq!(reader.read_token(), Ok(Token::Eof));

        let mut reader = TokenReader::new("a === b".as_bytes());

        assert_eq!(reader.read_token(), Ok(Token::Ident("a".to_string())));
        assert_eq!(reader.read_token(), Ok(Token::Logical(Logical::SEq)));
        assert_eq!(reader.read_token(), Ok(Token::Ident("b".to_string())));
        assert_eq!(reader.read_token(), Ok(Token::Eof));
    }

    #[test]
    fn logical_ne_test() {
        let mut reader = TokenReader::new("!=".as_bytes());

        assert_eq!(reader.read_token(), Ok(Token::Logical(Logical::Ne)));
        assert_eq!(reader.read_token(), Ok(Token::Eof));

        let mut reader = TokenReader::new("a != b".as_bytes());

        assert_eq!(reader.read_token(), Ok(Token::Ident("a".to_string())));
        assert_eq!(reader.read_token(), Ok(Token::Logical(Logical::Ne)));
        assert_eq!(reader.read_token(), Ok(Token::Ident("b".to_string())));
        assert_eq!(reader.read_token(), Ok(Token::Eof));
    }

    #[test]
    fn logical_sne_test() {
        let mut reader = TokenReader::new("!==".as_bytes());

        assert_eq!(reader.read_token(), Ok(Token::Logical(Logical::SNe)));
        assert_eq!(reader.read_token(), Ok(Token::Eof));

        let mut reader = TokenReader::new("a !== b".as_bytes());

        assert_eq!(reader.read_token(), Ok(Token::Ident("a".to_string())));
        assert_eq!(reader.read_token(), Ok(Token::Logical(Logical::SNe)));
        assert_eq!(reader.read_token(), Ok(Token::Ident("b".to_string())));
        assert_eq!(reader.read_token(), Ok(Token::Eof));
    }
}
