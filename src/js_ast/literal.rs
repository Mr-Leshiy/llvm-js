use crate::{
    lexer::{CharReader, Literal as LiteralToken, Token},
    parser::{self, Parser},
};
use std::io::Read;

/// Literal - Expression type for numeric literals like "1.0" or string literals "hello".
#[derive(Debug, PartialEq)]
pub enum Literal {
    Number(f64),
    String(String),
}

impl Parser for Literal {
    fn parse<R: Read>(cur_token: Token, _: &mut CharReader<R>) -> Result<Self, parser::Error> {
        match cur_token {
            Token::Literal(LiteralToken::Number(val)) => Ok(Self::Number(val)),
            Token::Literal(LiteralToken::String(val)) => Ok(Self::String(val)),
            token => Err(parser::Error::UnexpectedToken(token)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer;

    #[test]
    fn parse_literal_test() {
        let mut reader = CharReader::new("12;".as_bytes());
        assert_eq!(
            Literal::parse(lexer::get_token(&mut reader).unwrap(), &mut reader).unwrap(),
            Literal::Number(12_f64)
        );

        let mut reader = CharReader::new(r#""name""#.as_bytes());
        assert_eq!(
            Literal::parse(lexer::get_token(&mut reader).unwrap(), &mut reader).unwrap(),
            Literal::String("name".to_string())
        );
    }
}
