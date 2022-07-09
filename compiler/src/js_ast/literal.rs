use crate::parser::{self, Parser};
use lexer::{Literal as LiteralToken, Token, TokenReader};
use std::io::Read;

/// Literal - Expression type for numeric literals like "1.0" or string literals "hello".
#[derive(Clone, Debug, PartialEq)]
pub enum Literal {
    Number(f64),
    String(String),
}

impl Parser for Literal {
    fn parse<R: Read>(cur_token: Token, _: &mut TokenReader<R>) -> Result<Self, parser::Error> {
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

    #[test]
    fn parse_literal_test() {
        let mut reader = TokenReader::new("12;".as_bytes());
        assert_eq!(
            Literal::parse(reader.next_token().unwrap(), &mut reader),
            Ok(Literal::Number(12_f64))
        );

        let mut reader = TokenReader::new(r#""name""#.as_bytes());
        assert_eq!(
            Literal::parse(reader.next_token().unwrap(), &mut reader),
            Ok(Literal::String("name".to_string()))
        );
    }
}
