use super::{BlockStatement, Identifier};
use crate::{
    lexer::{CharReader, Token},
    parser::{self, Parser},
};
use std::io::Read;

#[derive(Debug, PartialEq)]
pub struct FunctionDeclaration {
    pub name: Identifier,
    pub args: Vec<Identifier>,
    pub body: BlockStatement,
}

impl Parser for FunctionDeclaration {
    fn parse<R: Read>(cur_token: Token, _: &mut CharReader<R>) -> Result<Self, parser::Error> {
        match cur_token {
            token => Err(parser::Error::UnexpectedToken(token)),
        }
    }
}
