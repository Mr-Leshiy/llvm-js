use super::Error;
use crate::{
    ast::Identifier,
    lexer::{CharReader, Token},
};
use std::io::Read;

pub fn parse_identifier<R: Read>(reader: &mut CharReader<R>) -> Result<Identifier, Error> {
    match Token::get_token(reader)? {
        Token::Ident(name) => Ok(Identifier { name }),
        token => Err(Error::UnexpectedToken(token)),
    }
}
