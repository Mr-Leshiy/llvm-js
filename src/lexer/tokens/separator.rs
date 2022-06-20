use std::fmt::Display;

#[derive(Clone, Debug, PartialEq)]
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
    use crate::lexer::{get_token, CharReader, Token};

    #[test]
    fn separator_test_1() {
        let mut reader = CharReader::new(r#" )({[]]  }})[],,, "#.as_bytes());

        assert_eq!(
            get_token(&mut reader),
            Ok(Token::Separator(Separator::CloseBrace))
        );
        assert_eq!(
            get_token(&mut reader),
            Ok(Token::Separator(Separator::OpenBrace))
        );
        assert_eq!(
            get_token(&mut reader),
            Ok(Token::Separator(Separator::OpenCurlyBrace))
        );
        assert_eq!(
            get_token(&mut reader),
            Ok(Token::Separator(Separator::OpenSquareBracket))
        );
        assert_eq!(
            get_token(&mut reader),
            Ok(Token::Separator(Separator::CloseSquareBracket))
        );
        assert_eq!(
            get_token(&mut reader),
            Ok(Token::Separator(Separator::CloseSquareBracket))
        );
        assert_eq!(
            get_token(&mut reader),
            Ok(Token::Separator(Separator::CloseCurlyBrace))
        );
        assert_eq!(
            get_token(&mut reader),
            Ok(Token::Separator(Separator::CloseCurlyBrace))
        );
        assert_eq!(
            get_token(&mut reader),
            Ok(Token::Separator(Separator::CloseBrace))
        );
        assert_eq!(
            get_token(&mut reader),
            Ok(Token::Separator(Separator::OpenSquareBracket))
        );
        assert_eq!(
            get_token(&mut reader),
            Ok(Token::Separator(Separator::CloseSquareBracket))
        );
        assert_eq!(
            get_token(&mut reader),
            Ok(Token::Separator(Separator::Comma))
        );
        assert_eq!(
            get_token(&mut reader),
            Ok(Token::Separator(Separator::Comma))
        );
        assert_eq!(
            get_token(&mut reader),
            Ok(Token::Separator(Separator::Comma))
        );
        assert_eq!(get_token(&mut reader), Ok(Token::Eof));
    }

    #[test]
    fn separator_test_2() {
        let mut reader = CharReader::new(r#"name()"#.as_bytes());
        assert_eq!(get_token(&mut reader), Ok(Token::Ident("name".to_string())));
        assert_eq!(
            get_token(&mut reader),
            Ok(Token::Separator(Separator::OpenBrace))
        );
        assert_eq!(
            get_token(&mut reader),
            Ok(Token::Separator(Separator::CloseBrace))
        );
        assert_eq!(get_token(&mut reader), Ok(Token::Eof));

        let mut reader = CharReader::new(r#"name{}"#.as_bytes());
        assert_eq!(get_token(&mut reader), Ok(Token::Ident("name".to_string())));
        assert_eq!(
            get_token(&mut reader),
            Ok(Token::Separator(Separator::OpenCurlyBrace))
        );
        assert_eq!(
            get_token(&mut reader),
            Ok(Token::Separator(Separator::CloseCurlyBrace))
        );
        assert_eq!(get_token(&mut reader), Ok(Token::Eof));

        let mut reader = CharReader::new(r#"name[]"#.as_bytes());
        assert_eq!(get_token(&mut reader), Ok(Token::Ident("name".to_string())));
        assert_eq!(
            get_token(&mut reader),
            Ok(Token::Separator(Separator::OpenSquareBracket))
        );
        assert_eq!(
            get_token(&mut reader),
            Ok(Token::Separator(Separator::CloseSquareBracket))
        );
        assert_eq!(get_token(&mut reader), Ok(Token::Eof));

        let mut reader = CharReader::new(r#"name,"#.as_bytes());
        assert_eq!(get_token(&mut reader), Ok(Token::Ident("name".to_string())));
        assert_eq!(
            get_token(&mut reader),
            Ok(Token::Separator(Separator::Comma))
        );
        assert_eq!(get_token(&mut reader), Ok(Token::Eof));
    }
}
