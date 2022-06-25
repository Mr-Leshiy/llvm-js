use super::{Identifier, Literal};
use crate::{
    lexer::{CharReader, Literal as LiteralToken, Token},
    parser::{self, Parser},
};
use std::io::Read;

/// RightAssigmentValue
#[derive(Clone, Debug, PartialEq)]
pub enum RightAssigmentValue {
    Literal(Literal),
    Identifier(Identifier),
}

impl Parser for RightAssigmentValue {
    fn parse<R: Read>(cur_token: Token, _: &mut CharReader<R>) -> Result<Self, parser::Error> {
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
    use crate::lexer;

    #[test]
    fn parse_right_assigment_value_test() {
        let mut reader = CharReader::new("12".as_bytes());
        assert_eq!(
            RightAssigmentValue::parse(lexer::get_token(&mut reader).unwrap(), &mut reader)
                .unwrap(),
            RightAssigmentValue::Literal(Literal::Number(12_f64)),
        );

        let mut reader = CharReader::new(r#""name""#.as_bytes());
        assert_eq!(
            RightAssigmentValue::parse(lexer::get_token(&mut reader).unwrap(), &mut reader)
                .unwrap(),
            RightAssigmentValue::Literal(Literal::String("name".to_string())),
        );

        let mut reader = CharReader::new("name".as_bytes());
        assert_eq!(
            RightAssigmentValue::parse(lexer::get_token(&mut reader).unwrap(), &mut reader)
                .unwrap(),
            RightAssigmentValue::Identifier(Identifier {
                name: "name".to_string()
            }),
        );
    }
}
