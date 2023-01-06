use super::IsToken;
use crate::Error;
use std::fmt::Display;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Keyword {
    /// "var"
    Var,
    /// "let"
    Let,
    /// "function"
    Function,
    /// "return",
    Return,
    /// "if"
    If,
    /// "else"
    Else,
    /// "while"
    While,
    /// "do"
    Do,
}

impl Display for Keyword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Var => write!(f, "Keyword var token"),
            Self::Let => write!(f, "Keyword let token"),
            Self::Function => write!(f, "Keyword function token"),
            Self::Return => write!(f, "Keyword return token"),
            Self::If => write!(f, "Keyword if token"),
            Self::Else => write!(f, "Keyword else token"),
            Self::While => write!(f, "Keyword while token"),
            Self::Do => write!(f, "Keyword do token"),
        }
    }
}

impl Keyword {
    pub fn is_var<Res>(
        self,
        fun: impl FnOnce(()) -> Result<Res, Error>,
    ) -> Result<IsToken<Res, Keyword>, Error> {
        IsToken::<Res, Keyword>::is(self, Keyword::Var, fun)
    }

    pub fn is_let<Res>(
        self,
        fun: impl FnOnce(()) -> Result<Res, Error>,
    ) -> Result<IsToken<Res, Keyword>, Error> {
        IsToken::<Res, Keyword>::is(self, Keyword::Let, fun)
    }

    pub fn is_function<Res>(
        self,
        fun: impl FnOnce(()) -> Result<Res, Error>,
    ) -> Result<IsToken<Res, Keyword>, Error> {
        IsToken::<Res, Keyword>::is(self, Keyword::Function, fun)
    }

    pub fn is_return<Res>(
        self,
        fun: impl FnOnce(()) -> Result<Res, Error>,
    ) -> Result<IsToken<Res, Keyword>, Error> {
        IsToken::<Res, Keyword>::is(self, Keyword::Return, fun)
    }

    pub fn is_if<Res>(
        self,
        fun: impl FnOnce(()) -> Result<Res, Error>,
    ) -> Result<IsToken<Res, Keyword>, Error> {
        IsToken::<Res, Keyword>::is(self, Keyword::If, fun)
    }

    pub fn is_else<Res>(
        self,
        fun: impl FnOnce(()) -> Result<Res, Error>,
    ) -> Result<IsToken<Res, Keyword>, Error> {
        IsToken::<Res, Keyword>::is(self, Keyword::Else, fun)
    }

    pub fn is_while<Res>(
        self,
        fun: impl FnOnce(()) -> Result<Res, Error>,
    ) -> Result<IsToken<Res, Keyword>, Error> {
        IsToken::<Res, Keyword>::is(self, Keyword::While, fun)
    }

    pub fn is_do<Res>(
        self,
        fun: impl FnOnce(()) -> Result<Res, Error>,
    ) -> Result<IsToken<Res, Keyword>, Error> {
        IsToken::<Res, Keyword>::is(self, Keyword::Do, fun)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Token, TokenReader};

    #[test]
    fn keyword_var_test() {
        let mut reader = TokenReader::new("var".as_bytes());

        assert_eq!(reader.read_token(), Ok(Token::Keyword(Keyword::Var)));
        assert_eq!(reader.read_token(), Ok(Token::Eof));
    }

    #[test]
    fn keyword_let_test() {
        let mut reader = TokenReader::new("let".as_bytes());

        assert_eq!(reader.read_token(), Ok(Token::Keyword(Keyword::Let)));
        assert_eq!(reader.read_token(), Ok(Token::Eof));
    }

    #[test]
    fn keyword_function_test() {
        let mut reader = TokenReader::new("function".as_bytes());

        assert_eq!(reader.read_token(), Ok(Token::Keyword(Keyword::Function)));
        assert_eq!(reader.read_token(), Ok(Token::Eof));
    }

    #[test]
    fn keyword_return_test() {
        let mut reader = TokenReader::new("return".as_bytes());

        assert_eq!(reader.read_token(), Ok(Token::Keyword(Keyword::Return)));
        assert_eq!(reader.read_token(), Ok(Token::Eof));
    }

    #[test]
    fn keyword_if_test() {
        let mut reader = TokenReader::new("if".as_bytes());

        assert_eq!(reader.read_token(), Ok(Token::Keyword(Keyword::If)));
        assert_eq!(reader.read_token(), Ok(Token::Eof));
    }

    #[test]
    fn keyword_else_test() {
        let mut reader = TokenReader::new("else".as_bytes());

        assert_eq!(reader.read_token(), Ok(Token::Keyword(Keyword::Else)));
        assert_eq!(reader.read_token(), Ok(Token::Eof));
    }

    #[test]
    fn keyword_while_test() {
        let mut reader = TokenReader::new("while".as_bytes());

        assert_eq!(reader.read_token(), Ok(Token::Keyword(Keyword::While)));
        assert_eq!(reader.read_token(), Ok(Token::Eof));
    }

    #[test]
    fn keyword_do_test() {
        let mut reader = TokenReader::new("do".as_bytes());

        assert_eq!(reader.read_token(), Ok(Token::Keyword(Keyword::Do)));
        assert_eq!(reader.read_token(), Ok(Token::Eof));
    }
}
