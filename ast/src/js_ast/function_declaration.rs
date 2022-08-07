use super::{BlockStatement, Identifier};
use crate::llvm_ast;
use lexer::{Keyword, Parser, Separator, Token, TokenReader};
use precompiler::{self, Precompiler};
use std::io::Read;

#[derive(Clone, Debug, PartialEq)]
pub struct FunctionDeclaration {
    pub name: Identifier,
    pub args: Vec<Identifier>,
    pub body: BlockStatement,
}

impl Parser for FunctionDeclaration {
    fn parse<R: Read>(
        mut cur_token: Token,
        reader: &mut TokenReader<R>,
    ) -> Result<Self, lexer::Error> {
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
                                token => return Err(lexer::Error::UnexpectedToken(token)),
                            };
                        }
                        Ok(args)
                    }
                    token => Err(lexer::Error::UnexpectedToken(token)),
                }?;

                // parse function body
                let body = BlockStatement::parse(reader.next_token()?, reader)?;

                Ok(Self { name, args, body })
            }
            token => Err(lexer::Error::UnexpectedToken(token)),
        }
    }
}

impl FunctionDeclaration {
    pub fn precompile(
        self,
        precompiler: &mut Precompiler<Identifier, llvm_ast::FunctionDeclaration>,
    ) -> Result<llvm_ast::FunctionDeclaration, precompiler::Error<Identifier>> {
        let index = precompiler.functions.insert(self.name.clone());

        let variables_len = precompiler.variables.len();

        let function_declaration = llvm_ast::FunctionDeclaration {
            name: llvm_ast::Identifier::new(self.name.name, index),
            args: self
                .args
                .into_iter()
                .map(|name| {
                    // argument initialization hides the previous variable declaration with the same name
                    let index = precompiler.variables.insert(name.clone());
                    llvm_ast::Identifier::new(name.name, index)
                })
                .collect(),
            body: self.body.precompile(precompiler)?,
        };
        precompiler
            .variables
            .remove_last_added(precompiler.variables.len() - variables_len);

        Ok(function_declaration)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::js_ast::{Expression, VariableAssigment, VariableExpression, VariableValue};

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
                        left: "a".to_string().into(),
                        right: VariableExpression::VariableValue(VariableValue::Identifier(
                            "b".to_string().into()
                        )),
                    })]
                }
            })
        );
    }

    #[test]
    fn precompile_function_declaration_test() {
        let mut precompiler = Precompiler::new(Vec::new().into_iter());

        let function_declaration = FunctionDeclaration {
            name: "name_1".to_string().into(),
            args: vec!["a".to_string().into(), "b".to_string().into()],
            body: BlockStatement {
                body: vec![Expression::VariableAssigment(VariableAssigment {
                    left: "a".to_string().into(),
                    right: VariableExpression::VariableValue(VariableValue::Identifier(
                        "b".to_string().into(),
                    )),
                })],
            },
        };

        assert_eq!(
            function_declaration.precompile(&mut precompiler),
            Ok(llvm_ast::FunctionDeclaration {
                name: llvm_ast::Identifier::new("name_1".to_string(), 0),
                args: vec![
                    llvm_ast::Identifier::new("a".to_string(), 0),
                    llvm_ast::Identifier::new("b".to_string(), 0)
                ],
                body: vec![llvm_ast::Expression::VariableAssigment(
                    llvm_ast::VariableAssigment {
                        name: llvm_ast::Identifier::new("a".to_string(), 0),
                        value: llvm_ast::VariableValue::Identifier(llvm_ast::Identifier::new(
                            "b".to_string(),
                            0
                        ))
                    }
                )]
            })
        );
        assert!(precompiler.variables.is_empty());
        assert_eq!(
            precompiler.functions.get(&"name_1".to_string().into(),),
            Some(0)
        );
    }

    #[test]
    fn precompile_function_declaration_test_2() {
        let mut precompiler = Precompiler::new(Vec::new().into_iter());
        precompiler.variables.insert("a".to_string().into());
        precompiler.variables.insert("b".to_string().into());

        let function_declaration = FunctionDeclaration {
            name: "name_1".to_string().into(),
            args: vec!["a".to_string().into(), "b".to_string().into()],
            body: BlockStatement {
                body: vec![Expression::VariableAssigment(VariableAssigment {
                    left: "a".to_string().into(),
                    right: VariableExpression::VariableValue(VariableValue::Identifier(
                        "b".to_string().into(),
                    )),
                })],
            },
        };

        assert_eq!(
            function_declaration.precompile(&mut precompiler),
            Ok(llvm_ast::FunctionDeclaration {
                name: llvm_ast::Identifier::new("name_1".to_string(), 0),
                args: vec![
                    llvm_ast::Identifier::new("a".to_string(), 1),
                    llvm_ast::Identifier::new("b".to_string(), 1)
                ],
                body: vec![llvm_ast::Expression::VariableAssigment(
                    llvm_ast::VariableAssigment {
                        name: llvm_ast::Identifier::new("a".to_string(), 1),
                        value: llvm_ast::VariableValue::Identifier(llvm_ast::Identifier::new(
                            "b".to_string(),
                            1
                        ))
                    }
                )]
            })
        );
        assert_eq!(precompiler.variables.len(), 2);
        assert_eq!(
            precompiler.functions.get(&"name_1".to_string().into(),),
            Some(0)
        );
    }

    #[test]
    fn precompile_function_declaration_test_3() {
        let mut precompiler = Precompiler::new(Vec::new().into_iter());
        precompiler.functions.insert("name_1".to_string().into());

        let function_declaration = FunctionDeclaration {
            name: "name_1".to_string().into(),

            args: vec![],
            body: BlockStatement { body: vec![] },
        };

        assert_eq!(
            function_declaration.precompile(&mut precompiler),
            Ok(llvm_ast::FunctionDeclaration {
                name: llvm_ast::Identifier::new("name_1".to_string(), 1),
                args: vec![],
                body: vec![],
            })
        );
    }
}
