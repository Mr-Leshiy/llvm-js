use super::{Error, Parser};
use crate::{
    ast::{AssigmentExpression, Expression, VariableDeclaration},
    lexer::{CharReader, Separator, Token},
};
use std::io::Read;

impl Parser for Expression {
    fn parse<R: Read>(cur_token: Token, reader: &mut CharReader<R>) -> Result<Self, Error> {
        match cur_token {
            Token::Var => Ok(Expression::VariableDeclaration(VariableDeclaration(
                AssigmentExpression::parse(Token::get_token(reader)?, reader)?,
            ))),
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{Identifier, Literal, RightAssigmentValue};

    #[test]
    fn expression_variable_declaration_test() {
        let mut reader = CharReader::new("var name = 12;".as_bytes());
        assert_eq!(
            Expression::parse(Token::get_token(&mut reader).unwrap(), &mut reader).unwrap(),
            Expression::VariableDeclaration(VariableDeclaration(AssigmentExpression {
                left: Identifier {
                    name: "name".to_string()
                },
                right: RightAssigmentValue::Literal(Literal::Number(12_f64))
            }))
        );
    }

    #[test]
    fn expression_assigment_test() {
        let mut reader = CharReader::new("name = 12;".as_bytes());
        assert_eq!(
            Expression::parse(Token::get_token(&mut reader).unwrap(), &mut reader).unwrap(),
            Expression::Assigment(AssigmentExpression {
                left: Identifier {
                    name: "name".to_string()
                },
                right: RightAssigmentValue::Literal(Literal::Number(12_f64))
            })
        );
    }

    #[test]
    fn expression_block_statement_test() {
        let mut reader = CharReader::new("{ }".as_bytes());
        assert_eq!(
            Expression::parse(Token::get_token(&mut reader).unwrap(), &mut reader).unwrap(),
            Expression::BlockStatement { body: vec![] }
        );

        let mut reader = CharReader::new("{ name1 = name2; }".as_bytes());
        assert_eq!(
            Expression::parse(Token::get_token(&mut reader).unwrap(), &mut reader).unwrap(),
            Expression::BlockStatement {
                body: vec![Expression::Assigment(AssigmentExpression {
                    left: Identifier {
                        name: "name1".to_string()
                    },
                    right: RightAssigmentValue::Identifier(Identifier {
                        name: "name2".to_string()
                    })
                })]
            }
        );

        let mut reader =
            CharReader::new("{ name1 = name2; { name1 = name2; name1 = name2; } }".as_bytes());

        assert_eq!(
            Expression::parse(Token::get_token(&mut reader).unwrap(), &mut reader).unwrap(),
            Expression::BlockStatement {
                body: vec![
                    Expression::Assigment(AssigmentExpression {
                        left: Identifier {
                            name: "name1".to_string()
                        },
                        right: RightAssigmentValue::Identifier(Identifier {
                            name: "name2".to_string()
                        })
                    }),
                    Expression::BlockStatement {
                        body: vec![
                            Expression::Assigment(AssigmentExpression {
                                left: Identifier {
                                    name: "name1".to_string()
                                },
                                right: RightAssigmentValue::Identifier(Identifier {
                                    name: "name2".to_string()
                                })
                            }),
                            Expression::Assigment(AssigmentExpression {
                                left: Identifier {
                                    name: "name1".to_string()
                                },
                                right: RightAssigmentValue::Identifier(Identifier {
                                    name: "name2".to_string()
                                })
                            }),
                        ]
                    }
                ]
            }
        );
    }
}
