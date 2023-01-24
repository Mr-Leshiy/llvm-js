use super::{BlockStatement, Identifier};
use crate::{llvm_ast, LexerError, Precompiler, PrecompilerError};
use lexer::{Keyword, Separator, Token, TokenReader};
use std::io::Read;

#[derive(Clone, Debug, PartialEq)]
pub struct FunctionDeclaration {
    pub name: Identifier,
    pub args: Vec<Identifier>,
    pub body: BlockStatement,
}

impl FunctionDeclaration {
    pub fn parse<R: Read>(
        mut cur_token: Token,
        reader: &mut TokenReader<R>,
    ) -> Result<Self, LexerError> {
        match cur_token {
            Token::Keyword(Keyword::Function) => {
                // parse function name
                let name = Identifier::parse(reader.next_token()?, reader)?;

                // parse function args
                let args = match reader.next_token()? {
                    Token::Separator(Separator::OpenBrace) => {
                        let mut args = Vec::new();
                        cur_token = reader.next_token()?;
                        loop {
                            let arg = match cur_token {
                                Token::Separator(Separator::CloseBrace) => break,
                                cur_token => Identifier::parse(cur_token, reader)?,
                            };
                            args.push(arg);

                            cur_token = match reader.next_token()? {
                                Token::Separator(Separator::CloseBrace) => break,
                                Token::Separator(Separator::Comma) => reader.next_token()?,
                                token => return Err(LexerError::UnexpectedToken(token)),
                            };
                        }
                        Ok(args)
                    }
                    token => Err(LexerError::UnexpectedToken(token)),
                }?;

                // parse function body
                let body = BlockStatement::parse(reader.next_token()?, reader)?;

                Ok(Self { name, args, body })
            }
            token => Err(LexerError::UnexpectedToken(token)),
        }
    }
}

impl FunctionDeclaration {
    pub fn precompile(
        self,
        precompiler: &mut Precompiler,
    ) -> Result<
        (
            llvm_ast::FunctionDeclaration,
            llvm_ast::VariableFunctionDeclaration,
        ),
        PrecompilerError,
    > {
        let index = precompiler.insert_function(self.name.clone());

        let variables_len = precompiler.variables_len();
        let name = llvm_ast::Identifier::new(self.name.name, index);
        let args_num = self.args.len().try_into().expect("");

        let function_declaration = llvm_ast::FunctionDeclaration {
            name: name.clone(),
            args: self
                .args
                .into_iter()
                .map(|name| {
                    // argument initialization hides the previous variable declaration with the same name
                    let index = precompiler.insert_variable(name.clone());
                    llvm_ast::Identifier::new(name.name, index)
                })
                .collect(),
            body: self.body.precompile(precompiler)?,
        };
        precompiler.remove_last_added_variables(precompiler.variables_len() - variables_len);

        Ok((
            function_declaration,
            llvm_ast::VariableFunctionDeclaration { name, args_num },
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::js_ast::{
        Expression, MemberExpression, VariableAssigment, VariableExpression, VariableValue,
    };

    #[test]
    fn parse_function_declaration_test() {
        let mut reader = TokenReader::new("function foo(a, b) { a = b; }".as_bytes());
        assert_eq!(
            FunctionDeclaration::parse(reader.next_token().unwrap(), &mut reader),
            Ok(FunctionDeclaration {
                name: "foo".to_string().into(),
                args: vec!["a".to_string().into(), "b".to_string().into()],
                body: BlockStatement {
                    body: vec![Expression::VariableAssigment(VariableAssigment {
                        left: MemberExpression {
                            variable_name: "a".to_string().into(),
                            property: None
                        },
                        right: Some(VariableExpression::VariableValue(
                            VariableValue::MemberExpression(MemberExpression {
                                variable_name: "b".to_string().into(),
                                property: None
                            })
                        )),
                    })]
                }
            })
        );
    }

    #[test]
    fn precompile_function_declaration_test() {
        let mut precompiler = Precompiler::new(std::iter::empty());

        let function_declaration = FunctionDeclaration {
            name: "name_1".to_string().into(),
            args: vec!["a".to_string().into(), "b".to_string().into()],
            body: BlockStatement {
                body: vec![Expression::VariableAssigment(VariableAssigment {
                    left: MemberExpression {
                        variable_name: "a".to_string().into(),
                        property: None,
                    },
                    right: Some(VariableExpression::VariableValue(
                        VariableValue::MemberExpression(MemberExpression {
                            variable_name: "b".to_string().into(),
                            property: None,
                        }),
                    )),
                })],
            },
        };

        assert_eq!(
            function_declaration.precompile(&mut precompiler),
            Ok((
                llvm_ast::FunctionDeclaration {
                    name: llvm_ast::Identifier::new("name_1".to_string(), 0),
                    args: vec![
                        llvm_ast::Identifier::new("a".to_string(), 0),
                        llvm_ast::Identifier::new("b".to_string(), 0)
                    ],
                    body: vec![llvm_ast::Expression::VariableAssigment(
                        llvm_ast::VariableAssigment {
                            left: llvm_ast::MemberExpression {
                                variable_name: llvm_ast::Identifier::new("a".to_string(), 0),
                                property: None
                            },
                            right: Some(llvm_ast::VariableExpression::VariableValue(
                                llvm_ast::VariableValue::MemberExpression(
                                    llvm_ast::MemberExpression {
                                        variable_name: llvm_ast::Identifier::new(
                                            "b".to_string(),
                                            0
                                        ),
                                        property: None
                                    }
                                )
                            ))
                        }
                    )]
                },
                llvm_ast::VariableFunctionDeclaration {
                    name: llvm_ast::Identifier::new("name_1".to_string(), 0),
                    args_num: 2
                }
            ))
        );
        assert_eq!(precompiler.variables_len(), 0);
        assert_eq!(precompiler.get_function("name_1".to_string().into()), Ok(0));
    }

    #[test]
    fn precompile_function_declaration_test_2() {
        let mut precompiler = Precompiler::new(std::iter::empty());
        precompiler.insert_variable("a".to_string().into());
        precompiler.insert_variable("b".to_string().into());

        let function_declaration = FunctionDeclaration {
            name: "name_1".to_string().into(),
            args: vec!["a".to_string().into(), "b".to_string().into()],
            body: BlockStatement {
                body: vec![Expression::VariableAssigment(VariableAssigment {
                    left: MemberExpression {
                        variable_name: "a".to_string().into(),
                        property: None,
                    },
                    right: Some(VariableExpression::VariableValue(
                        VariableValue::MemberExpression(MemberExpression {
                            variable_name: "b".to_string().into(),
                            property: None,
                        }),
                    )),
                })],
            },
        };

        assert_eq!(
            function_declaration.precompile(&mut precompiler),
            Ok((
                llvm_ast::FunctionDeclaration {
                    name: llvm_ast::Identifier::new("name_1".to_string(), 0),
                    args: vec![
                        llvm_ast::Identifier::new("a".to_string(), 1),
                        llvm_ast::Identifier::new("b".to_string(), 1)
                    ],
                    body: vec![llvm_ast::Expression::VariableAssigment(
                        llvm_ast::VariableAssigment {
                            left: llvm_ast::MemberExpression {
                                variable_name: llvm_ast::Identifier::new("a".to_string(), 1),
                                property: None
                            },
                            right: Some(llvm_ast::VariableExpression::VariableValue(
                                llvm_ast::VariableValue::MemberExpression(
                                    llvm_ast::MemberExpression {
                                        variable_name: llvm_ast::Identifier::new(
                                            "b".to_string(),
                                            1
                                        ),
                                        property: None
                                    }
                                )
                            ))
                        }
                    )]
                },
                llvm_ast::VariableFunctionDeclaration {
                    name: llvm_ast::Identifier::new("name_1".to_string(), 0),
                    args_num: 2
                }
            ))
        );
        assert_eq!(precompiler.variables_len(), 2);
        assert_eq!(precompiler.get_function("name_1".to_string().into()), Ok(0));
    }

    #[test]
    fn precompile_function_declaration_test_3() {
        let mut precompiler = Precompiler::new(std::iter::empty());
        precompiler.insert_function("name_1".to_string().into());

        let function_declaration = FunctionDeclaration {
            name: "name_1".to_string().into(),

            args: vec![],
            body: BlockStatement { body: vec![] },
        };

        assert_eq!(
            function_declaration.precompile(&mut precompiler),
            Ok((
                llvm_ast::FunctionDeclaration {
                    name: llvm_ast::Identifier::new("name_1".to_string(), 1),
                    args: vec![],
                    body: vec![],
                },
                llvm_ast::VariableFunctionDeclaration {
                    name: llvm_ast::Identifier::new("name_1".to_string(), 1),
                    args_num: 0
                }
            ))
        );
    }
}
