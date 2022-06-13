use super::{Error, Parser};
use crate::{
    ast::{Expression, Program},
    lexer::{CharReader, Token},
};
use std::io::Read;

impl Parser for Program {
    fn parse<R: Read>(mut cur_token: Token, reader: &mut CharReader<R>) -> Result<Self, Error> {
        let mut body = Vec::new();

        loop {
            let expr = match cur_token {
                Token::Eof => break,
                cur_token => Expression::parse(cur_token, reader)?,
            };

            cur_token = Token::get_token(reader)?;
            body.push(expr);
        }

        Ok(Program { body })
    }
}
