use super::Error;
use crate::{
    ast::{Identifier, Literal, RightAssigmentValue},
    lexer::{CharReader, Token},
};
use std::io::Read;

pub fn parse_right_assignment_value<R: Read>(
    reader: &mut CharReader<R>,
) -> Result<RightAssigmentValue, Error> {
    match Token::get_token(reader)? {
        Token::Number(val) => Ok(RightAssigmentValue::Literal(Literal::Number(val))),
        Token::String(val) => Ok(RightAssigmentValue::Literal(Literal::String(val))),
        Token::Ident(name) => Ok(RightAssigmentValue::Identifier(Identifier { name })),
        token => Err(Error::UnexpectedToken(token)),
    }
}
