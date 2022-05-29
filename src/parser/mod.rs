use crate::{
    ast::{AssigmentExpression, Expression, Program, VariableDeclaration},
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

pub trait Parser: Sized {
    fn parse<R: Read>(cur_token: Token, reader: &mut CharReader<R>) -> Result<Self, Error>;
}

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{AssigmentExpression, Identifier, Literal, RightAssigmentValue};

    #[test]
    fn parse_program_from_file() {
        let file = std::fs::File::open("test_scripts/basic.js").unwrap();
        let mut reader = CharReader::new(file);

        let program = Program::parse(Token::get_token(&mut reader).unwrap(), &mut reader).unwrap();

        assert_eq!(program.body.len(), 5);
        let mut iter = program.body.iter();
        // var a = 5;
        assert_eq!(
            *iter.next().unwrap(),
            Expression::VariableDeclaration(VariableDeclaration {
                id: Identifier {
                    name: "a".to_string()
                },
                init: RightAssigmentValue::Literal(Literal::Number(5_f64))
            })
        );
        // var b = 6;
        assert_eq!(
            *iter.next().unwrap(),
            Expression::VariableDeclaration(VariableDeclaration {
                id: Identifier {
                    name: "b".to_string()
                },
                init: RightAssigmentValue::Literal(Literal::Number(6_f64))
            })
        );
        // a = b;
        assert_eq!(
            *iter.next().unwrap(),
            Expression::AssigmentExpression(AssigmentExpression {
                left: Identifier {
                    name: "a".to_string()
                },
                right: RightAssigmentValue::Identifier(Identifier {
                    name: "b".to_string()
                })
            })
        );
        // b = 7;
        assert_eq!(
            *iter.next().unwrap(),
            Expression::AssigmentExpression(AssigmentExpression {
                left: Identifier {
                    name: "b".to_string()
                },
                right: RightAssigmentValue::Literal(Literal::Number(7_f64))
            })
        );
        // var b = 6;
        assert_eq!(
            *iter.next().unwrap(),
            Expression::VariableDeclaration(VariableDeclaration {
                id: Identifier {
                    name: "c".to_string()
                },
                init: RightAssigmentValue::Literal(Literal::String("hello".to_string()))
            })
        );
    }
}
