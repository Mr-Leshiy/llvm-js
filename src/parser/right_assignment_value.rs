use super::{Error, Parser};
use crate::{
    ast::{Identifier, Literal, RightAssigmentValue},
    lexer::{CharReader, Token},
};
use std::io::Read;

impl Parser for RightAssigmentValue {
    fn parse<R: Read>(cur_token: Token, _: &mut CharReader<R>) -> Result<Self, Error> {
        match cur_token {
            Token::Number(val) => Ok(RightAssigmentValue::Literal(Literal::Number(val))),
            Token::String(val) => Ok(RightAssigmentValue::Literal(Literal::String(val))),
            Token::Ident(name) => Ok(RightAssigmentValue::Identifier(Identifier { name })),
            token => Err(Error::UnexpectedToken(token)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn right_assigment_value_test() {
        let mut reader = CharReader::new("12".as_bytes());
        assert_eq!(
            RightAssigmentValue::parse(Token::get_token(&mut reader).unwrap(), &mut reader)
                .unwrap(),
            RightAssigmentValue::Literal(Literal::Number(12_f64)),
        );

        let mut reader = CharReader::new(r#""name""#.as_bytes());
        assert_eq!(
            RightAssigmentValue::parse(Token::get_token(&mut reader).unwrap(), &mut reader)
                .unwrap(),
            RightAssigmentValue::Literal(Literal::String("name".to_string())),
        );

        let mut reader = CharReader::new("name".as_bytes());
        assert_eq!(
            RightAssigmentValue::parse(Token::get_token(&mut reader).unwrap(), &mut reader)
                .unwrap(),
            RightAssigmentValue::Identifier(Identifier {
                name: "name".to_string()
            }),
        );
    }
}
