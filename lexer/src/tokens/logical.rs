use super::IsToken;
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
    /// logical greater than, e.g. >
    Gt,
    /// logical greater than or equal, e.g. >=
    Ge,
    /// logical less than, e.g. <
    Lt,
    /// logical less than or equal, e.g. <=
    Le,
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
            Self::Gt => write!(f, "Logical GT operator"),
            Self::Ge => write!(f, "Logical GE operator"),
            Self::Lt => write!(f, "Logical LT operator"),
            Self::Le => write!(f, "Logical LE operator"),
        }
    }
}

impl Logical {
    fn is<Res>(self, expected: Logical, fun: impl FnOnce(()) -> Res) -> IsToken<Res, Logical> {
        if self == expected {
            IsToken::True(fun(()))
        } else {
            IsToken::False(self)
        }
    }

    pub fn is_not<Res>(self, fun: impl FnOnce(()) -> Res) -> IsToken<Res, Logical> {
        self.is(Logical::Not, fun)
    }

    pub fn is_and<Res>(self, fun: impl FnOnce(()) -> Res) -> IsToken<Res, Logical> {
        self.is(Logical::And, fun)
    }

    pub fn is_or<Res>(self, fun: impl FnOnce(()) -> Res) -> IsToken<Res, Logical> {
        self.is(Logical::Or, fun)
    }

    pub fn is_eq<Res>(self, fun: impl FnOnce(()) -> Res) -> IsToken<Res, Logical> {
        self.is(Logical::Eq, fun)
    }

    pub fn is_ne<Res>(self, fun: impl FnOnce(()) -> Res) -> IsToken<Res, Logical> {
        self.is(Logical::Ne, fun)
    }

    pub fn is_seq<Res>(self, fun: impl FnOnce(()) -> Res) -> IsToken<Res, Logical> {
        self.is(Logical::SEq, fun)
    }

    pub fn is_sne<Res>(self, fun: impl FnOnce(()) -> Res) -> IsToken<Res, Logical> {
        self.is(Logical::SNe, fun)
    }

    pub fn is_gt<Res>(self, fun: impl FnOnce(()) -> Res) -> IsToken<Res, Logical> {
        self.is(Logical::Gt, fun)
    }

    pub fn is_ge<Res>(self, fun: impl FnOnce(()) -> Res) -> IsToken<Res, Logical> {
        self.is(Logical::Ge, fun)
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

        let mut reader = TokenReader::new("a&&b".as_bytes());

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

        let mut reader = TokenReader::new("a||b".as_bytes());

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

        let mut reader = TokenReader::new("a==b".as_bytes());

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

        let mut reader = TokenReader::new("a===b".as_bytes());

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

        let mut reader = TokenReader::new("a!=b".as_bytes());

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

        let mut reader = TokenReader::new("a!==b".as_bytes());

        assert_eq!(reader.read_token(), Ok(Token::Ident("a".to_string())));
        assert_eq!(reader.read_token(), Ok(Token::Logical(Logical::SNe)));
        assert_eq!(reader.read_token(), Ok(Token::Ident("b".to_string())));
        assert_eq!(reader.read_token(), Ok(Token::Eof));
    }

    #[test]
    fn logical_gt_test() {
        // let mut reader = TokenReader::new(">".as_bytes());

        // assert_eq!(reader.read_token(), Ok(Token::Logical(Logical::Gt)));
        // assert_eq!(reader.read_token(), Ok(Token::Eof));

        let mut reader = TokenReader::new("a > b".as_bytes());

        assert_eq!(reader.read_token(), Ok(Token::Ident("a".to_string())));
        assert_eq!(reader.read_token(), Ok(Token::Logical(Logical::Gt)));
        assert_eq!(reader.read_token(), Ok(Token::Ident("b".to_string())));
        assert_eq!(reader.read_token(), Ok(Token::Eof));

        let mut reader = TokenReader::new("a>b".as_bytes());

        assert_eq!(reader.read_token(), Ok(Token::Ident("a".to_string())));
        assert_eq!(reader.read_token(), Ok(Token::Logical(Logical::Gt)));
        assert_eq!(reader.read_token(), Ok(Token::Ident("b".to_string())));
        assert_eq!(reader.read_token(), Ok(Token::Eof));
    }

    #[test]
    fn logical_ge_test() {
        let mut reader = TokenReader::new(">=".as_bytes());

        assert_eq!(reader.read_token(), Ok(Token::Logical(Logical::Ge)));
        assert_eq!(reader.read_token(), Ok(Token::Eof));

        let mut reader = TokenReader::new("a >= b".as_bytes());

        assert_eq!(reader.read_token(), Ok(Token::Ident("a".to_string())));
        assert_eq!(reader.read_token(), Ok(Token::Logical(Logical::Ge)));
        assert_eq!(reader.read_token(), Ok(Token::Ident("b".to_string())));
        assert_eq!(reader.read_token(), Ok(Token::Eof));

        let mut reader = TokenReader::new("a>=b".as_bytes());

        assert_eq!(reader.read_token(), Ok(Token::Ident("a".to_string())));
        assert_eq!(reader.read_token(), Ok(Token::Logical(Logical::Ge)));
        assert_eq!(reader.read_token(), Ok(Token::Ident("b".to_string())));
        assert_eq!(reader.read_token(), Ok(Token::Eof));
    }

    #[test]
    fn logical_lt_test() {
        let mut reader = TokenReader::new("<".as_bytes());

        assert_eq!(reader.read_token(), Ok(Token::Logical(Logical::Lt)));
        assert_eq!(reader.read_token(), Ok(Token::Eof));

        let mut reader = TokenReader::new("a < b".as_bytes());

        assert_eq!(reader.read_token(), Ok(Token::Ident("a".to_string())));
        assert_eq!(reader.read_token(), Ok(Token::Logical(Logical::Lt)));
        assert_eq!(reader.read_token(), Ok(Token::Ident("b".to_string())));
        assert_eq!(reader.read_token(), Ok(Token::Eof));

        let mut reader = TokenReader::new("a<b".as_bytes());

        assert_eq!(reader.read_token(), Ok(Token::Ident("a".to_string())));
        assert_eq!(reader.read_token(), Ok(Token::Logical(Logical::Lt)));
        assert_eq!(reader.read_token(), Ok(Token::Ident("b".to_string())));
        assert_eq!(reader.read_token(), Ok(Token::Eof));
    }

    #[test]
    fn logical_le_test() {
        let mut reader = TokenReader::new("<=".as_bytes());

        assert_eq!(reader.read_token(), Ok(Token::Logical(Logical::Le)));
        assert_eq!(reader.read_token(), Ok(Token::Eof));

        let mut reader = TokenReader::new("a <= b".as_bytes());

        assert_eq!(reader.read_token(), Ok(Token::Ident("a".to_string())));
        assert_eq!(reader.read_token(), Ok(Token::Logical(Logical::Le)));
        assert_eq!(reader.read_token(), Ok(Token::Ident("b".to_string())));
        assert_eq!(reader.read_token(), Ok(Token::Eof));

        let mut reader = TokenReader::new("a<=b".as_bytes());

        assert_eq!(reader.read_token(), Ok(Token::Ident("a".to_string())));
        assert_eq!(reader.read_token(), Ok(Token::Logical(Logical::Le)));
        assert_eq!(reader.read_token(), Ok(Token::Ident("b".to_string())));
        assert_eq!(reader.read_token(), Ok(Token::Eof));
    }
}
