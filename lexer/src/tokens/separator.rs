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
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Token, TokenReader};

    #[test]
    fn separator_test_1() {
        let mut reader = TokenReader::new(r#" )({[]]  }})[],,, "#.as_bytes());

        assert_eq!(
            reader.read_token(),
            Ok(Token::Separator(Separator::CloseBrace))
        );
        assert_eq!(
            reader.read_token(),
            Ok(Token::Separator(Separator::OpenBrace))
        );
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
        assert_eq!(
            reader.read_token(),
            Ok(Token::Separator(Separator::CloseCurlyBrace))
        );
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
        assert_eq!(reader.read_token(), Ok(Token::Separator(Separator::Comma)));
        assert_eq!(reader.read_token(), Ok(Token::Separator(Separator::Comma)));
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
    }
}
