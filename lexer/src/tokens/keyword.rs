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
    use crate::{Token, TokenReader};

    #[test]
    fn token_var() {
        let mut reader = TokenReader::new("var".as_bytes());

        assert_eq!(reader.read_token(), Ok(Token::Keyword(Keyword::Var)));
        assert_eq!(reader.read_token(), Ok(Token::Eof));
    }

    #[test]
    fn token_function() {
        let mut reader = TokenReader::new("function".as_bytes());

        assert_eq!(reader.read_token(), Ok(Token::Keyword(Keyword::Function)));
        assert_eq!(reader.read_token(), Ok(Token::Eof));
    }
}
