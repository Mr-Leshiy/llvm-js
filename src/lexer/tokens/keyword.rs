use std::fmt::Display;

#[derive(Clone, Debug, PartialEq)]
pub enum Keyword {
    /// "var"
    Var,
    /// "function"
    Function,
}

impl Display for Keyword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Var => write!(f, "Keyword var token"),
            Self::Function => write!(f, "Keyword function token"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::{get_token, CharReader, Token};

    #[test]
    fn token_var() {
        let mut reader = CharReader::new("var".as_bytes());

        assert_eq!(get_token(&mut reader), Ok(Token::Keyword(Keyword::Var)));
        assert_eq!(get_token(&mut reader), Ok(Token::Eof));
    }

    #[test]
    fn token_function() {
        let mut reader = CharReader::new("function".as_bytes());

        assert_eq!(
            get_token(&mut reader),
            Ok(Token::Keyword(Keyword::Function))
        );
        assert_eq!(get_token(&mut reader), Ok(Token::Eof));
    }
}
