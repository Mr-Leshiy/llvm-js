use super::{Error, Parser};
use crate::{
    ast::Identifier,
    lexer::{CharReader, Token},
};
use std::io::Read;

impl Parser for Identifier {
    fn parse<R: Read>(cur_token: Token, _: &mut CharReader<R>) -> Result<Self, Error> {
        match cur_token {
            Token::Ident(name) => Ok(Identifier { name }),
            token => Err(Error::UnexpectedToken(token)),
        }
    }
}
