use super::{Error, Parser};
use crate::{
    ast::{AssigmentExpression, Identifier, RightAssigmentValue},
    lexer::{CharReader, Token},
};
use std::io::Read;

impl Parser for AssigmentExpression {
    fn parse<R: Read>(cur_token: Token, reader: &mut CharReader<R>) -> Result<Self, Error> {
        let left = Identifier::parse(cur_token, reader)?;

        match Token::get_token(reader)? {
            Token::Assign => {}
            token => return Err(Error::UnexpectedToken(token)),
        }

        let right = RightAssigmentValue::parse(Token::get_token(reader)?, reader)?;
        Ok(AssigmentExpression { left, right })
    }
}
