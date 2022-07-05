use super::Expression;
use crate::{
    lexer::{self, CharReader, Separator, Token},
    llvm_ast,
    parser::{self, Parser},
    precompiler::{self, Precompile, Precompiler},
};
use std::io::Read;

#[derive(Clone, Debug, PartialEq)]
pub struct BlockStatement {
    pub body: Vec<Expression>,
}

impl Parser for BlockStatement {
    fn parse<R: Read>(cur_token: Token, reader: &mut CharReader<R>) -> Result<Self, parser::Error> {
        match cur_token {
            Token::Separator(Separator::OpenCurlyBrace) => {
                let mut body = Vec::new();
                let mut cur_token = lexer::get_token(reader)?;
                loop {
                    let expr = match cur_token {
                        Token::Separator(Separator::CloseCurlyBrace) => break,
                        cur_token => Expression::parse(cur_token, reader)?,
                    };

                    cur_token = lexer::get_token(reader)?;
                    body.push(expr);
                }

                Ok(Self { body })
            }
            token => Err(parser::Error::UnexpectedToken(token)),
        }
    }
}

impl Precompile for BlockStatement {
    type Output = Vec<llvm_ast::Expression>;
    fn precompile(self, precompiler: &mut Precompiler) -> Result<Self::Output, precompiler::Error> {
        let mut res = Vec::with_capacity(self.body.len());
        let variables_len = precompiler.variables.len();
        let functions_len = precompiler.functions.len();
        for expr in self.body {
            expr.precompile(precompiler)?
                .into_iter()
                .for_each(|expr| res.push(expr))
        }
        precompiler
            .variables
            .remove_last_added(precompiler.variables.len() - variables_len);
        precompiler
            .functions
            .remove_last_added(precompiler.functions.len() - functions_len);
        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::js_ast::{
        FunctionDeclaration, Identifier, Literal, RightAssigmentValue, VariableAssigment,
        VariableDeclaration,
    };

    #[test]
    fn parse_block_statement_test() {
        let mut reader = CharReader::new("{ }".as_bytes());
        assert_eq!(
            BlockStatement::parse(lexer::get_token(&mut reader).unwrap(), &mut reader),
            Ok(BlockStatement { body: vec![] })
        );

        let mut reader = CharReader::new("{ name1 = name2; }".as_bytes());
        assert_eq!(
            BlockStatement::parse(lexer::get_token(&mut reader).unwrap(), &mut reader),
            Ok(BlockStatement {
                body: vec![Expression::VariableAssigment(VariableAssigment {
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
            BlockStatement::parse(lexer::get_token(&mut reader).unwrap(), &mut reader),
            Ok(BlockStatement {
                body: vec![
                    Expression::VariableAssigment(VariableAssigment {
                        left: Identifier {
                            name: "name1".to_string()
                        },
                        right: RightAssigmentValue::Identifier(Identifier {
                            name: "name2".to_string()
                        })
                    }),
                    Expression::BlockStatement(BlockStatement {
                        body: vec![
                            Expression::VariableAssigment(VariableAssigment {
                                left: Identifier {
                                    name: "name1".to_string()
                                },
                                right: RightAssigmentValue::Identifier(Identifier {
                                    name: "name2".to_string()
                                })
                            }),
                            Expression::VariableAssigment(VariableAssigment {
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

    #[test]
    fn precompile_block_statement_test_1() {
        let mut precompiler = Precompiler::new();
        assert_eq!(precompiler.variables.len(), 0);
        let block_statement = BlockStatement {
            body: vec![Expression::VariableDeclaration(VariableDeclaration(
                VariableAssigment {
                    left: Identifier {
                        name: "name_1".to_string(),
                    },
                    right: RightAssigmentValue::Literal(Literal::Number(64_f64)),
                },
            ))],
        };

        assert_eq!(
            block_statement.precompile(&mut precompiler),
            Ok(vec![llvm_ast::Expression::VariableDeclaration(
                llvm_ast::VariableDeclaration(llvm_ast::VariableAssigment {
                    name: "name_1".to_string(),
                    value: llvm_ast::VariableValue::FloatNumber(64_f64),
                })
            )])
        );
        assert_eq!(precompiler.variables.len(), 0);
    }

    #[test]
    fn precompile_block_statement_test_2() {
        let mut precompiler = Precompiler::new();
        assert_eq!(precompiler.functions.len(), 0);
        let block_statement = BlockStatement {
            body: vec![Expression::FunctionDeclaration(FunctionDeclaration {
                name: Identifier {
                    name: "name_1".to_string(),
                },
                args: vec![],
                body: BlockStatement { body: vec![] },
            })],
        };

        assert_eq!(block_statement.precompile(&mut precompiler), Ok(vec![]));
        assert_eq!(precompiler.functions.len(), 0);
        assert_eq!(precompiler.function_declarations.len(), 1);
    }
}
