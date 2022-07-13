use super::{Identifier, Literal};
use crate::parser::{self, Parser};
use lexer::{Literal as LiteralToken, Token, TokenReader};
use std::io::Read;

/// RightAssigmentValue
#[derive(Clone, Debug, PartialEq)]
pub enum RightAssigmentValue {
    Literal(Literal),
    Identifier(Identifier),
}

impl Parser for RightAssigmentValue {
    fn parse<R: Read>(cur_token: Token, _: &mut TokenReader<R>) -> Result<Self, parser::Error> {
        match cur_token {
            Token::Literal(LiteralToken::Number(val)) => Ok(Self::Literal(Literal::Number(val))),
            Token::Literal(LiteralToken::String(val)) => Ok(Self::Literal(Literal::String(val))),
            Token::Ident(name) => Ok(Self::Identifier(Identifier { name })),
            token => Err(parser::Error::UnexpectedToken(token)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_right_assigment_value_test() {
        let mut reader = TokenReader::new("12".as_bytes());
        assert_eq!(
            RightAssigmentValue::parse(reader.next_token().unwrap(), &mut reader),
            Ok(RightAssigmentValue::Literal(Literal::Number(12_f64))),
        );

        let mut reader = TokenReader::new(r#""name""#.as_bytes());
        assert_eq!(
            RightAssigmentValue::parse(reader.next_token().unwrap(), &mut reader),
            Ok(RightAssigmentValue::Literal(Literal::String(
                "name".to_string()
            ))),
        );

        let mut reader = TokenReader::new("name".as_bytes());
        assert_eq!(
            RightAssigmentValue::parse(reader.next_token().unwrap(), &mut reader),
            Ok(RightAssigmentValue::Identifier(Identifier {
                name: "name".to_string()
            })),
        );
    }
}
