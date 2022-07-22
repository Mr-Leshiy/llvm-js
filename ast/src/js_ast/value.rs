use super::{Identifier, Literal};
use lexer::{Literal as LiteralToken, Parser, Token, TokenReader};
use std::io::Read;

/// Value
#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Literal(Literal),
    Identifier(Identifier),
}

impl Parser for Value {
    fn parse<R: Read>(cur_token: Token, _: &mut TokenReader<R>) -> Result<Self, lexer::Error> {
        match cur_token {
            Token::Literal(LiteralToken::Number(val)) => Ok(Self::Literal(Literal::Number(val))),
            Token::Literal(LiteralToken::String(val)) => Ok(Self::Literal(Literal::String(val))),
            Token::Ident(name) => Ok(Self::Identifier(Identifier { name })),
            token => Err(lexer::Error::UnexpectedToken(token)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_value_test() {
        let mut reader = TokenReader::new("12".as_bytes());
        assert_eq!(
            Value::parse(reader.next_token().unwrap(), &mut reader),
            Ok(Value::Literal(Literal::Number(12_f64))),
        );

        let mut reader = TokenReader::new(r#""name""#.as_bytes());
        assert_eq!(
            Value::parse(reader.next_token().unwrap(), &mut reader),
            Ok(Value::Literal(Literal::String(
                "name".to_string()
            ))),
        );

        let mut reader = TokenReader::new("name".as_bytes());
        assert_eq!(
            Value::parse(reader.next_token().unwrap(), &mut reader),
            Ok(Value::Identifier(Identifier {
                name: "name".to_string()
            })),
        );
    }
}
