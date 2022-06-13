use super::{Error, Parser};
use crate::{
    ast::{AssigmentExpression, Expression, VariableDeclaration},
    lexer::{CharReader, Separator, Token},
};
use std::io::Read;

impl Parser for Expression {
    fn parse<R: Read>(cur_token: Token, reader: &mut CharReader<R>) -> Result<Self, Error> {
        match cur_token {
            Token::Var => Ok(Expression::VariableDeclaration(VariableDeclaration::parse(
                cur_token, reader,
            )?)),
            Token::Ident(_) => Ok(Expression::Assigment(AssigmentExpression::parse(
                cur_token, reader,
            )?)),
            Token::Separator(Separator::OpenCurlyBrace) => {
                let mut body = Vec::new();
                let mut cur_token = Token::get_token(reader)?;
                loop {
                    let expr = match cur_token {
                        Token::Separator(Separator::CloseCurlyBrace) => break,
                        cur_token => Expression::parse(cur_token, reader)?,
                    };

                    cur_token = Token::get_token(reader)?;
                    body.push(expr);
                }

                Ok(Expression::BlockStatement { body })
            }
            token => Err(Error::UnexpectedToken(token)),
        }
    }
}
