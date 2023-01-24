use super::Expression;
use crate::{llvm_ast, LexerError, Precompiler, PrecompilerError};
use lexer::{Separator, Token, TokenReader};
use std::io::Read;

#[derive(Clone, Debug, PartialEq)]
pub struct BlockStatement {
    pub body: Vec<Expression>,
}

impl BlockStatement {
    pub fn parse<R: Read>(
        cur_token: Token,
        reader: &mut TokenReader<R>,
    ) -> Result<Self, LexerError> {
        match cur_token {
            Token::Separator(Separator::OpenCurlyBrace) => {
                let mut body = Vec::new();
                let mut cur_token = reader.next_token()?;
                loop {
                    let expr = match cur_token {
                        Token::Separator(Separator::CloseCurlyBrace) => break,
                        cur_token => Expression::parse(cur_token, reader)?,
                    };

                    cur_token = reader.next_token()?;
                    body.push(expr);
                }

                Ok(Self { body })
            }
            token => Err(LexerError::UnexpectedToken(token)),
        }
    }
}

impl BlockStatement {
    pub fn precompile(
        self,
        precompiler: &mut Precompiler,
    ) -> Result<Vec<llvm_ast::Expression>, PrecompilerError> {
        let mut res = Vec::with_capacity(self.body.len());
        let variables_len = precompiler.variables_len();
        let functions_len = precompiler.functions_len();
        for expr in self.body {
            expr.precompile(precompiler)?
                .into_iter()
                .for_each(|expr| res.push(expr));
        }
        let vars =
            precompiler.remove_last_added_variables(precompiler.variables_len() - variables_len);
        for (var, index) in vars {
            res.push(llvm_ast::Expression::DeallocateExpression(
                llvm_ast::DeallocateExpression {
                    name: llvm_ast::Identifier::new(var.name, index),
                },
            ));
        }
        precompiler.remove_last_added_functions(precompiler.functions_len() - functions_len);
        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::js_ast::{
        FunctionDeclaration, MemberExpression, VariableAssigment, VariableDeclaration,
        VariableExpression, VariableValue,
    };

    #[test]
    fn parse_block_statement_test() {
        let mut reader = TokenReader::new("{ }".as_bytes());
        assert_eq!(
            BlockStatement::parse(reader.next_token().unwrap(), &mut reader),
            Ok(BlockStatement { body: vec![] })
        );

        let mut reader = TokenReader::new("{ name1 = name2; }".as_bytes());
        assert_eq!(
            BlockStatement::parse(reader.next_token().unwrap(), &mut reader),
            Ok(BlockStatement {
                body: vec![Expression::VariableAssigment(VariableAssigment {
                    left: MemberExpression {
                        variable_name: "name1".to_string().into(),
                        property: None
                    },
                    right: Some(VariableExpression::VariableValue(
                        VariableValue::MemberExpression(MemberExpression {
                            variable_name: "name2".to_string().into(),
                            property: None
                        })
                    ))
                })]
            })
        );

        let mut reader =
            TokenReader::new("{ name1 = name2; { name1 = name2; name1 = name2; } }".as_bytes());

        assert_eq!(
            BlockStatement::parse(reader.next_token().unwrap(), &mut reader),
            Ok(BlockStatement {
                body: vec![
                    Expression::VariableAssigment(VariableAssigment {
                        left: MemberExpression {
                            variable_name: "name1".to_string().into(),
                            property: None
                        },
                        right: Some(VariableExpression::VariableValue(
                            VariableValue::MemberExpression(MemberExpression {
                                variable_name: "name2".to_string().into(),
                                property: None
                            })
                        ))
                    }),
                    Expression::BlockStatement(BlockStatement {
                        body: vec![
                            Expression::VariableAssigment(VariableAssigment {
                                left: MemberExpression {
                                    variable_name: "name1".to_string().into(),
                                    property: None
                                },
                                right: Some(VariableExpression::VariableValue(
                                    VariableValue::MemberExpression(MemberExpression {
                                        variable_name: "name2".to_string().into(),
                                        property: None
                                    })
                                ))
                            }),
                            Expression::VariableAssigment(VariableAssigment {
                                left: MemberExpression {
                                    variable_name: "name1".to_string().into(),
                                    property: None
                                },
                                right: Some(VariableExpression::VariableValue(
                                    VariableValue::MemberExpression(MemberExpression {
                                        variable_name: "name2".to_string().into(),
                                        property: None
                                    })
                                ))
                            }),
                        ]
                    })
                ]
            })
        );
    }

    #[test]
    fn precompile_block_statement_test_1() {
        let mut precompiler = Precompiler::new(std::iter::empty());
        assert_eq!(precompiler.variables_len(), 0);
        let block_statement = BlockStatement {
            body: vec![Expression::VariableDeclaration(VariableDeclaration {
                name: "name_1".to_string().into(),
                value: Some(VariableExpression::VariableValue(VariableValue::Number(
                    64_f64,
                ))),
            })],
        };

        assert_eq!(
            block_statement.precompile(&mut precompiler),
            Ok(vec![
                llvm_ast::Expression::VariableDeclaration(llvm_ast::VariableDeclaration {
                    name: llvm_ast::Identifier::new("name_1".to_string(), 0),
                    value: Some(llvm_ast::VariableExpression::VariableValue(
                        llvm_ast::VariableValue::FloatNumber(64_f64)
                    )),
                }),
                llvm_ast::Expression::DeallocateExpression(llvm_ast::DeallocateExpression {
                    name: llvm_ast::Identifier::new("name_1".to_string(), 0)
                })
            ])
        );
        assert_eq!(precompiler.variables_len(), 0);
    }

    #[test]
    fn precompile_block_statement_test_2() {
        let mut precompiler = Precompiler::new(std::iter::empty());
        assert_eq!(precompiler.functions_len(), 0);

        let block_statement = BlockStatement {
            body: vec![Expression::FunctionDeclaration(FunctionDeclaration {
                name: "name_1".to_string().into(),
                args: vec![],
                body: BlockStatement { body: vec![] },
            })],
        };

        assert_eq!(
            block_statement.precompile(&mut precompiler),
            Ok(vec![llvm_ast::Expression::VariableFunctionDeclaration(
                llvm_ast::VariableFunctionDeclaration {
                    name: llvm_ast::Identifier::new("name_1".to_string(), 0),
                    args_num: 0
                }
            )])
        );
        assert_eq!(precompiler.functions_len(), 0);
        assert_eq!(precompiler.get_function_declarations().len(), 1);
    }
}
