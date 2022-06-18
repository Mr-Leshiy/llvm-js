use super::Expression;
use crate::{
    lexer::{CharReader, Separator, Token},
    parser::{self, Parser},
};
use std::io::Read;

#[derive(Debug, PartialEq)]
pub struct BlockStatement {
    pub body: Vec<Expression>,
}

impl Parser for BlockStatement {
    fn parse<R: Read>(cur_token: Token, reader: &mut CharReader<R>) -> Result<Self, parser::Error> {
        match cur_token {
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

                Ok(Self { body })
            }
            token => Err(parser::Error::UnexpectedToken(token)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{AssigmentExpression, Identifier, RightAssigmentValue};

    #[test]
    fn block_statement_test() {
        let mut reader = CharReader::new("{ }".as_bytes());
        assert_eq!(
            BlockStatement::parse(Token::get_token(&mut reader).unwrap(), &mut reader).unwrap(),
            BlockStatement { body: vec![] }
        );

        let mut reader = CharReader::new("{ name1 = name2; }".as_bytes());
        assert_eq!(
            BlockStatement::parse(Token::get_token(&mut reader).unwrap(), &mut reader).unwrap(),
            BlockStatement {
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
            BlockStatement::parse(Token::get_token(&mut reader).unwrap(), &mut reader).unwrap(),
            BlockStatement {
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
            }
        );
    }
}