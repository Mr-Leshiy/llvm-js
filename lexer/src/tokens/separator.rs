use super::IsToken;
use std::fmt::Display;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Separator {
    /// "("
    OpenBrace,
    /// ")"
    CloseBrace,
    /// "{"
    OpenCurlyBrace,
    /// "}"
    CloseCurlyBrace,
    /// "["
    OpenSquareBracket,
    /// "]"
    CloseSquareBracket,
    /// ","
    Comma,
    /// ","
    Dot,
    /// ":"
    Colon,
    /// ";"
    SemiColon,
}

impl Display for Separator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::OpenBrace => write!(f, r#"Separator token, "(""#),
            Self::CloseBrace => write!(f, r#"Separator token, ")""#),
            Self::OpenCurlyBrace => write!(f, r#"Separator token, "{{""#),
            Self::CloseCurlyBrace => write!(f, r#"Separator token, "}}""#),
            Self::OpenSquareBracket => write!(f, r#"Separator token, "[""#),
            Self::CloseSquareBracket => write!(f, r#"Separator token, "]""#),
            Self::Comma => write!(f, r#"Separator token, ",""#),
            Self::Dot => write!(f, r#"Separator token, ".""#),
            Self::Colon => write!(f, r#"Separator token, ":""#),
            Self::SemiColon => write!(f, r#"Separator token, ";""#),
        }
    }
}

impl Separator {
    fn is<Res>(self, expected: Separator, fun: impl FnOnce(()) -> Res) -> IsToken<Res, Separator> {
        if self == expected {
            IsToken::True(fun(()))
        } else {
            IsToken::False(self)
        }
    }

    pub fn is_open_brace<Res>(self, fun: impl FnOnce(()) -> Res) -> IsToken<Res, Separator> {
        self.is(Separator::OpenBrace, fun)
    }

    pub fn is_close_brace<Res>(self, fun: impl FnOnce(()) -> Res) -> IsToken<Res, Separator> {
        self.is(Separator::CloseBrace, fun)
    }

    pub fn is_open_curly_brace<Res>(self, fun: impl FnOnce(()) -> Res) -> IsToken<Res, Separator> {
        self.is(Separator::OpenCurlyBrace, fun)
    }

    pub fn is_close_curly_brace<Res>(self, fun: impl FnOnce(()) -> Res) -> IsToken<Res, Separator> {
        self.is(Separator::CloseCurlyBrace, fun)
    }

    pub fn is_comma<Res>(self, fun: impl FnOnce(()) -> Res) -> IsToken<Res, Separator> {
        self.is(Separator::Comma, fun)
    }

    pub fn is_dot<Res>(self, fun: impl FnOnce(()) -> Res) -> IsToken<Res, Separator> {
        self.is(Separator::Dot, fun)
    }

    pub fn is_colon<Res>(self, fun: impl FnOnce(()) -> Res) -> IsToken<Res, Separator> {
        self.is(Separator::Colon, fun)
    }

    pub fn is_semi_colon<Res>(self, fun: impl FnOnce(()) -> Res) -> IsToken<Res, Separator> {
        self.is(Separator::SemiColon, fun)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Token, TokenReader};

    #[test]
    fn separator_test_1() {
        let mut reader = TokenReader::new(r#" )(:{[]]:  }:})[],.,.,. "#.as_bytes());

        assert_eq!(
            reader.read_token(),
            Ok(Token::Separator(Separator::CloseBrace))
        );
        assert_eq!(
            reader.read_token(),
            Ok(Token::Separator(Separator::OpenBrace))
        );
        assert_eq!(reader.read_token(), Ok(Token::Separator(Separator::Colon)));
        assert_eq!(
            reader.read_token(),
            Ok(Token::Separator(Separator::OpenCurlyBrace))
        );
        assert_eq!(
            reader.read_token(),
            Ok(Token::Separator(Separator::OpenSquareBracket))
        );
        assert_eq!(
            reader.read_token(),
            Ok(Token::Separator(Separator::CloseSquareBracket))
        );
        assert_eq!(
            reader.read_token(),
            Ok(Token::Separator(Separator::CloseSquareBracket))
        );
        assert_eq!(reader.read_token(), Ok(Token::Separator(Separator::Colon)));
        assert_eq!(
            reader.read_token(),
            Ok(Token::Separator(Separator::CloseCurlyBrace))
        );
        assert_eq!(reader.read_token(), Ok(Token::Separator(Separator::Colon)));
        assert_eq!(
            reader.read_token(),
            Ok(Token::Separator(Separator::CloseCurlyBrace))
        );
        assert_eq!(
            reader.read_token(),
            Ok(Token::Separator(Separator::CloseBrace))
        );
        assert_eq!(
            reader.read_token(),
            Ok(Token::Separator(Separator::OpenSquareBracket))
        );
        assert_eq!(
            reader.read_token(),
            Ok(Token::Separator(Separator::CloseSquareBracket))
        );
        assert_eq!(reader.read_token(), Ok(Token::Separator(Separator::Comma)));
        assert_eq!(reader.read_token(), Ok(Token::Separator(Separator::Dot)));
        assert_eq!(reader.read_token(), Ok(Token::Separator(Separator::Comma)));
        assert_eq!(reader.read_token(), Ok(Token::Separator(Separator::Dot)));
        assert_eq!(reader.read_token(), Ok(Token::Separator(Separator::Comma)));
        assert_eq!(reader.read_token(), Ok(Token::Separator(Separator::Dot)));
        assert_eq!(reader.read_token(), Ok(Token::Eof));
    }

    #[test]
    fn separator_test_2() {
        let mut reader = TokenReader::new(r#"name()"#.as_bytes());
        assert_eq!(reader.read_token(), Ok(Token::Ident("name".to_string())));
        assert_eq!(
            reader.read_token(),
            Ok(Token::Separator(Separator::OpenBrace))
        );
        assert_eq!(
            reader.read_token(),
            Ok(Token::Separator(Separator::CloseBrace))
        );
        assert_eq!(reader.read_token(), Ok(Token::Eof));

        let mut reader = TokenReader::new(r#"name{}"#.as_bytes());
        assert_eq!(reader.read_token(), Ok(Token::Ident("name".to_string())));
        assert_eq!(
            reader.read_token(),
            Ok(Token::Separator(Separator::OpenCurlyBrace))
        );
        assert_eq!(
            reader.read_token(),
            Ok(Token::Separator(Separator::CloseCurlyBrace))
        );
        assert_eq!(reader.read_token(), Ok(Token::Eof));

        let mut reader = TokenReader::new(r#"name[]"#.as_bytes());
        assert_eq!(reader.read_token(), Ok(Token::Ident("name".to_string())));
        assert_eq!(
            reader.read_token(),
            Ok(Token::Separator(Separator::OpenSquareBracket))
        );
        assert_eq!(
            reader.read_token(),
            Ok(Token::Separator(Separator::CloseSquareBracket))
        );
        assert_eq!(reader.read_token(), Ok(Token::Eof));

        let mut reader = TokenReader::new(r#"name,"#.as_bytes());
        assert_eq!(reader.read_token(), Ok(Token::Ident("name".to_string())));
        assert_eq!(reader.read_token(), Ok(Token::Separator(Separator::Comma)));
        assert_eq!(reader.read_token(), Ok(Token::Eof));

        let mut reader = TokenReader::new(r#"name:"#.as_bytes());
        assert_eq!(reader.read_token(), Ok(Token::Ident("name".to_string())));
        assert_eq!(reader.read_token(), Ok(Token::Separator(Separator::Colon)));
        assert_eq!(reader.read_token(), Ok(Token::Eof));
    }
}
