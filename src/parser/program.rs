use super::{Error, Parser};
use crate::{
    ast::{AssigmentExpression, Expression, Program, VariableDeclaration},
    lexer::{CharReader, Token},
};
use std::io::Read;

impl Parser for Program {
    fn parse<R: Read>(mut cur_token: Token, reader: &mut CharReader<R>) -> Result<Self, Error> {
        let mut body = Vec::new();

        loop {
            let expr = match cur_token {
                Token::Var => {
                    Expression::VariableDeclaration(VariableDeclaration::parse(cur_token, reader)?)
                }
                Token::Ident(_) => {
                    Expression::AssigmentExpression(AssigmentExpression::parse(cur_token, reader)?)
                }
                Token::Eof => break,
                token => return Err(Error::UnexpectedToken(token)),
            };

            cur_token = Token::get_token(reader)?;
            body.push(expr);
        }

        Ok(Program { body })
    }
}
