use super::IsToken;
use crate::Error;
use std::fmt::Display;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Arithmetic {
    /// arithmetic addition, e.g. +
    Add,
    /// arithmetic substraction, e.g. -
    Sub,
    /// arithmetic division, e.g. /
    Div,
    /// arithmetic multiplication, e.g. *
    Mul,
}

impl Display for Arithmetic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Add => write!(f, "Arithmetic ADD operator"),
            Self::Sub => write!(f, "Arithmetic SUB operator"),
            Self::Div => write!(f, "Arithmetic DIV operator"),
            Self::Mul => write!(f, "Arithmetic MUL operator"),
        }
    }
}

impl Arithmetic {
    pub fn is_add<Res>(
        self,
        fun: impl FnOnce(()) -> Result<Res, Error>,
    ) -> Result<IsToken<Res, Arithmetic>, Error> {
        IsToken::<Res, Arithmetic>::is(self, Arithmetic::Add, fun)
    }

    pub fn is_sub<Res>(
        self,
        fun: impl FnOnce(()) -> Result<Res, Error>,
    ) -> Result<IsToken<Res, Arithmetic>, Error> {
        IsToken::<Res, Arithmetic>::is(self, Arithmetic::Sub, fun)
    }

    pub fn is_div<Res>(
        self,
        fun: impl FnOnce(()) -> Result<Res, Error>,
    ) -> Result<IsToken<Res, Arithmetic>, Error> {
        IsToken::<Res, Arithmetic>::is(self, Arithmetic::Div, fun)
    }

    pub fn is_mul<Res>(
        self,
        fun: impl FnOnce(()) -> Result<Res, Error>,
    ) -> Result<IsToken<Res, Arithmetic>, Error> {
        IsToken::<Res, Arithmetic>::is(self, Arithmetic::Mul, fun)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Token, TokenReader};

    #[test]
    fn arithmetic_add_test() {
        let mut reader = TokenReader::new("+".as_bytes());

        assert_eq!(reader.read_token(), Ok(Token::Arithmetic(Arithmetic::Add)));
        assert_eq!(reader.read_token(), Ok(Token::Eof));

        let mut reader = TokenReader::new("a + b".as_bytes());

        assert_eq!(reader.read_token(), Ok(Token::Ident("a".to_string())));
        assert_eq!(reader.read_token(), Ok(Token::Arithmetic(Arithmetic::Add)));
        assert_eq!(reader.read_token(), Ok(Token::Ident("b".to_string())));
        assert_eq!(reader.read_token(), Ok(Token::Eof));

        let mut reader = TokenReader::new("a+b".as_bytes());

        assert_eq!(reader.read_token(), Ok(Token::Ident("a".to_string())));
        assert_eq!(reader.read_token(), Ok(Token::Arithmetic(Arithmetic::Add)));
        assert_eq!(reader.read_token(), Ok(Token::Ident("b".to_string())));
        assert_eq!(reader.read_token(), Ok(Token::Eof));
    }

    #[test]
    fn arithmetic_sub_test() {
        let mut reader = TokenReader::new("-".as_bytes());

        assert_eq!(reader.read_token(), Ok(Token::Arithmetic(Arithmetic::Sub)));
        assert_eq!(reader.read_token(), Ok(Token::Eof));

        let mut reader = TokenReader::new("a - b".as_bytes());

        assert_eq!(reader.read_token(), Ok(Token::Ident("a".to_string())));
        assert_eq!(reader.read_token(), Ok(Token::Arithmetic(Arithmetic::Sub)));
        assert_eq!(reader.read_token(), Ok(Token::Ident("b".to_string())));
        assert_eq!(reader.read_token(), Ok(Token::Eof));

        let mut reader = TokenReader::new("a-b".as_bytes());

        assert_eq!(reader.read_token(), Ok(Token::Ident("a".to_string())));
        assert_eq!(reader.read_token(), Ok(Token::Arithmetic(Arithmetic::Sub)));
        assert_eq!(reader.read_token(), Ok(Token::Ident("b".to_string())));
        assert_eq!(reader.read_token(), Ok(Token::Eof));
    }

    #[test]
    fn arithmetic_div_test() {
        let mut reader = TokenReader::new("/".as_bytes());

        assert_eq!(reader.read_token(), Ok(Token::Arithmetic(Arithmetic::Div)));
        assert_eq!(reader.read_token(), Ok(Token::Eof));

        let mut reader = TokenReader::new("a / b".as_bytes());

        assert_eq!(reader.read_token(), Ok(Token::Ident("a".to_string())));
        assert_eq!(reader.read_token(), Ok(Token::Arithmetic(Arithmetic::Div)));
        assert_eq!(reader.read_token(), Ok(Token::Ident("b".to_string())));
        assert_eq!(reader.read_token(), Ok(Token::Eof));

        let mut reader = TokenReader::new("a/b".as_bytes());

        assert_eq!(reader.read_token(), Ok(Token::Ident("a".to_string())));
        assert_eq!(reader.read_token(), Ok(Token::Arithmetic(Arithmetic::Div)));
        assert_eq!(reader.read_token(), Ok(Token::Ident("b".to_string())));
        assert_eq!(reader.read_token(), Ok(Token::Eof));
    }

    #[test]
    fn arithmetic_mul_test() {
        let mut reader = TokenReader::new("*".as_bytes());

        assert_eq!(reader.read_token(), Ok(Token::Arithmetic(Arithmetic::Mul)));
        assert_eq!(reader.read_token(), Ok(Token::Eof));

        let mut reader = TokenReader::new("a * b".as_bytes());

        assert_eq!(reader.read_token(), Ok(Token::Ident("a".to_string())));
        assert_eq!(reader.read_token(), Ok(Token::Arithmetic(Arithmetic::Mul)));
        assert_eq!(reader.read_token(), Ok(Token::Ident("b".to_string())));
        assert_eq!(reader.read_token(), Ok(Token::Eof));

        let mut reader = TokenReader::new("a*b".as_bytes());

        assert_eq!(reader.read_token(), Ok(Token::Ident("a".to_string())));
        assert_eq!(reader.read_token(), Ok(Token::Arithmetic(Arithmetic::Mul)));
        assert_eq!(reader.read_token(), Ok(Token::Ident("b".to_string())));
        assert_eq!(reader.read_token(), Ok(Token::Eof));
    }
}
