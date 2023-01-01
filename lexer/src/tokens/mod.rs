use crate::Error;
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
            Self::Ident(val) => write!(f, "Ident token, val: {}", val),
            Self::Literal(val) => val.fmt(f),
            Self::Separator(val) => val.fmt(f),
            Self::Eof => write!(f, "Eof token"),
        }
    }
}

pub enum IsToken<Res, T> {
    True(Res),
    False(T),
}

impl<Res, T> IsToken<Res, T> {
    pub fn or<T1, FunT: FnOnce(T1) -> Result<Res, Error>>(
        self,
        is: impl FnOnce(T, FunT) -> Result<IsToken<Res, T>, Error>,
        fun: FunT,
    ) -> Result<IsToken<Res, T>, Error> {
        match self {
            IsToken::True(val) => Ok(IsToken::True(val)),
            IsToken::False(val) => is(val, fun),
        }
    }
}

impl<Res> IsToken<Res, Token> {
    pub fn result(self) -> Result<Res, Error> {
        match self {
            IsToken::True(val) => Ok(val),
            IsToken::False(token) => Err(Error::UnexpectedToken(token)),
        }
    }
}

impl<Res> IsToken<Res, Keyword> {
    pub fn result(self) -> Result<Res, Error> {
        match self {
            IsToken::True(val) => Ok(val),
            IsToken::False(keyword) => Err(Error::UnexpectedToken(Token::Keyword(keyword))),
        }
    }
}

impl Token {
    fn is<Res>(self, expected: Token, fun: impl FnOnce(()) -> Res) -> IsToken<Res, Token> {
        if self == expected {
            IsToken::True(fun(()))
        } else {
            IsToken::False(self)
        }
    }

    pub fn is_keyword<Res>(
        self,
        fun: impl FnOnce(Keyword) -> Result<Res, Error>,
    ) -> Result<IsToken<Res, Token>, Error> {
        if let Token::Keyword(val) = self {
            Ok(IsToken::True(fun(val)?))
        } else {
            Ok(IsToken::False(self))
        }
    }

    pub fn is_assign<Res>(self, fun: impl FnOnce(()) -> Res) -> IsToken<Res, Token> {
        self.is(Token::Assign, fun)
    }

    pub fn is_eof<Res>(self, fun: impl FnOnce(()) -> Res) -> IsToken<Res, Token> {
        self.is(Token::Eof, fun)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Error, Position, TokenReader};

    #[test]
    fn test() {
        // let res = Token::Assign
        //     .is_assign(|_| 0)
        //     .or(Token::is_keyword, |keyword: Keyword| {
        //         keyword.is_var(|_| 0).result()
        //     })
        //     .unwrap();
    }

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
