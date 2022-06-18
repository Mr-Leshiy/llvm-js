use super::{AssigmentExpression, BlockStatement, VariableDeclaration};
use crate::{
    lexer::{CharReader, Keyword, Separator, Token},
    parser::{self, Parser},
};
use std::io::Read;

#[derive(Debug, PartialEq)]
pub enum Expression {
    VariableDeclaration(VariableDeclaration),
    Assigment(AssigmentExpression),
    BlockStatement(BlockStatement),
}

impl Parser for Expression {
    fn parse<R: Read>(cur_token: Token, reader: &mut CharReader<R>) -> Result<Self, parser::Error> {
        match cur_token {
            Token::Keyword(Keyword::Var) => Ok(Self::VariableDeclaration(
                VariableDeclaration::parse(cur_token, reader)?,
            )),
            Token::Ident(_) => Ok(Self::Assigment(AssigmentExpression::parse(
                cur_token, reader,
            )?)),
            Token::Separator(Separator::OpenCurlyBrace) => Ok(Self::BlockStatement(
                BlockStatement::parse(cur_token, reader)?,
            )),
            token => Err(parser::Error::UnexpectedToken(token)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        ast::{Identifier, Literal, RightAssigmentValue},
        lexer,
    };

    #[test]
    fn expression_variable_declaration_test() {
        let mut reader = CharReader::new("var name = 12;".as_bytes());
        assert_eq!(
            Expression::parse(lexer::get_token(&mut reader).unwrap(), &mut reader).unwrap(),
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
            Expression::parse(lexer::get_token(&mut reader).unwrap(), &mut reader).unwrap(),
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
            Expression::parse(lexer::get_token(&mut reader).unwrap(), &mut reader).unwrap(),
            Expression::BlockStatement(BlockStatement { body: vec![] })
        );

        let mut reader = CharReader::new("{ name1 = name2; }".as_bytes());
        assert_eq!(
            Expression::parse(lexer::get_token(&mut reader).unwrap(), &mut reader).unwrap(),
            Expression::BlockStatement(BlockStatement {
                body: vec![Expression::Assigment(AssigmentExpression {
                    left: Identifier {
                        name: "name1".to_string()
                    },
                    right: RightAssigmentValue::Identifier(Identifier {
                        name: "name2".to_string()
                    })
                })]
            })
        );

        let mut reader =
            CharReader::new("{ name1 = name2; { name1 = name2; name1 = name2; } }".as_bytes());

        assert_eq!(
            Expression::parse(lexer::get_token(&mut reader).unwrap(), &mut reader).unwrap(),
            Expression::BlockStatement(BlockStatement {
                body: vec![
                    Expression::Assigment(AssigmentExpression {
                        left: Identifier {
                            name: "name1".to_string()
                        },
                        right: RightAssigmentValue::Identifier(Identifier {
                            name: "name2".to_string()
                        })
                    }),
                    Expression::BlockStatement(BlockStatement {
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
                    })
                ]
            })
        );
    }
}
