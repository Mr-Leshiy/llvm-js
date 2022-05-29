use super::{Error, Parser};
use crate::{
    ast::Literal,
    lexer::{CharReader, Token},
};
use std::io::Read;

impl Parser for Literal {
    fn parse<R: Read>(cur_token: Token, _: &mut CharReader<R>) -> Result<Self, Error> {
        match cur_token {
            Token::Number(val) => Ok(Literal::Number(val)),
            Token::String(val) => Ok(Literal::String(val)),
            token => Err(Error::UnexpectedToken(token)),
        }
    }
}
