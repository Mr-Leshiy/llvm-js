use crate::{
    ast::{ModuleUnit, Program},
    lexer::{self, CharReader, Token},
};
use std::io::Read;
use thiserror::Error;

mod assigment_expression;
mod expression;
mod identifier;
mod literal;
mod program;
mod right_assignment_value;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Unexpected token provided: {0}")]
    UnexpectedToken(Token),
    #[error("Parsing token error: {0}")]
    ParseTokenError(#[from] lexer::Error),
}

impl ModuleUnit {
    pub fn new<R: Read>(name: String, input: R) -> Result<Self, Error> {
        let mut reader = CharReader::new(input);
        let program = Program::parse(Token::get_token(&mut reader)?, &mut reader)?;
        Ok(Self { name, program })
    }
}

pub trait Parser: Sized {
    fn parse<R: Read>(cur_token: Token, reader: &mut CharReader<R>) -> Result<Self, Error>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{AssigmentExpression, Expression, Identifier, Literal, RightAssigmentValue};

    #[test]
    fn parse_program_from_file() {
        let file = std::fs::File::open("test_scripts/basic.js").unwrap();
        let module = ModuleUnit::new("".to_string(), file).unwrap();
        let program = module.program;

        assert_eq!(
            program.body,
            vec![Expression::BlockStatement {
                body: vec![
                    Expression::VariableDeclaration(AssigmentExpression {
                        left: Identifier {
                            name: "a".to_string()
                        },
                        right: RightAssigmentValue::Literal(Literal::Number(5_f64))
                    }),
                    Expression::VariableDeclaration(AssigmentExpression {
                        left: Identifier {
                            name: "b".to_string()
                        },
                        right: RightAssigmentValue::Literal(Literal::Number(6_f64))
                    }),
                    Expression::BlockStatement {
                        body: vec![
                            Expression::Assigment(AssigmentExpression {
                                left: Identifier {
                                    name: "a".to_string()
                                },
                                right: RightAssigmentValue::Identifier(Identifier {
                                    name: "b".to_string()
                                })
                            }),
                            Expression::Assigment(AssigmentExpression {
                                left: Identifier {
                                    name: "b".to_string()
                                },
                                right: RightAssigmentValue::Literal(Literal::Number(7_f64))
                            }),
                            Expression::VariableDeclaration(AssigmentExpression {
                                left: Identifier {
                                    name: "c".to_string()
                                },
                                right: RightAssigmentValue::Literal(Literal::String(
                                    "hello".to_string()
                                ))
                            })
                        ]
                    }
                ]
            }]
        );
    }
}
