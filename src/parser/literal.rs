use super::Error;
use crate::{
    ast::Literal,
    lexer::{CharReader, Token},
};
use std::io::Read;

pub fn parse_literal<R: Read>(reader: &mut CharReader<R>) -> Result<Literal, Error> {
    match Token::get_token(reader)? {
        Token::Number(val) => Ok(Literal::Number(val)),
        Token::String(val) => Ok(Literal::String(val)),
        token => Err(Error::UnexpectedToken(token)),
    }
}
