use crate::{
    ast::{AssigmentExpression, Expr, Program, VariableDeclaration},
    lexer::{self, CharReader, Token},
};
use std::io::Read;
use thiserror::Error;

mod assigment_expression;
mod identifier;
mod literal;
mod right_assignment_value;
mod variable_declaration;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Unexpected token provided: {0}")]
    UnexpectedToken(Token),
    #[error("Parsing token error: {0}")]
    ParseTokenError(#[from] lexer::Error),
}

trait Parser: Sized {
    fn parse<R: Read>(cur_token: Token, reader: &mut CharReader<R>) -> Result<Self, Error>;
}

impl Parser for Program {
    fn parse<R: Read>(mut cur_token: Token, reader: &mut CharReader<R>) -> Result<Self, Error> {
        let mut body = Vec::new();

        loop {
            let expr: Box<dyn Expr> = match cur_token {
                Token::Var => Box::new(VariableDeclaration::parse(cur_token, reader)?),
                Token::Ident(_) => Box::new(AssigmentExpression::parse(cur_token, reader)?),
                Token::Eof => break,
                token => return Err(Error::UnexpectedToken(token)),
            };

            cur_token = Token::get_token(reader)?;
            body.push(expr);
        }

        Ok(Program { body })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_program_from_file() {
        let file = std::fs::File::open("test_scripts/basic.js").unwrap();
        let mut reader = CharReader::new(file);

        let _ = Program::parse(Token::get_token(&mut reader).unwrap(), &mut reader).unwrap();
    }
}
