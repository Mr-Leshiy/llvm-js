use super::{Error, Parser};
use crate::{
    ast::{Identifier, RightAssigmentValue, VariableDeclaration},
    lexer::{CharReader, Token},
};
use std::io::Read;

impl Parser for VariableDeclaration {
    fn parse<R: Read>(cur_token: Token, reader: &mut CharReader<R>) -> Result<Self, Error> {
        match cur_token {
            Token::Var => {
                let id = Identifier::parse(Token::get_token(reader)?, reader)?;

                match Token::get_token(reader)? {
                    Token::Assign => {}
                    token => return Err(Error::UnexpectedToken(token)),
                }

                let init = RightAssigmentValue::parse(Token::get_token(reader)?, reader)?;
                Ok(VariableDeclaration { id, init })
            }
            token => Err(Error::UnexpectedToken(token)),
        }
    }
}
