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
