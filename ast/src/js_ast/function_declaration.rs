use super::{BlockStatement, Identifier};
use crate::{
    llvm_ast,
    precompiler::{self, Precompile, Precompiler},
};
use lexer::{Keyword, Parser, Separator, Token, TokenReader};
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

impl Precompile for FunctionDeclaration {
    type Output = llvm_ast::FunctionDeclaration;
    fn precompile(self, precompiler: &mut Precompiler) -> Result<Self::Output, precompiler::Error> {
        precompiler
            .functions
            .insert(self.name.clone())
            .map_err(|_| precompiler::Error::AlreadyDeclaredFunction(self.name.clone()))?;

        let variables_len = precompiler.variables.len();
        for arg in &self.args {
            // argument initialization hides the previous variable declaration with the same name
            let _ = precompiler.variables.insert(arg.clone());
        }

        let function_declaration = llvm_ast::FunctionDeclaration {
            name: self.name.name,
            args: self.args.into_iter().map(|name| name.name).collect(),
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
    use crate::js_ast::{Expression, RightAssigmentValue, VariableAssigment};

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
                        right: RightAssigmentValue::Identifier("b".to_string().into()),
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
                    right: RightAssigmentValue::Identifier("b".to_string().into()),
                })],
            },
        };

        assert_eq!(
            function_declaration.precompile(&mut precompiler),
            Ok(llvm_ast::FunctionDeclaration {
                name: "name_1".to_string(),
                args: vec!["a".to_string(), "b".to_string()],
                body: vec![llvm_ast::Expression::VariableAssigment(
                    llvm_ast::VariableAssigment {
                        name: "a".to_string(),
                        value: llvm_ast::VariableValue::Identifier("b".to_string())
                    }
                )]
            })
        );
        assert!(precompiler.variables.is_empty());
        assert!(precompiler
            .functions
            .contains(&"name_1".to_string().into(),));
    }

    #[test]
    fn precompile_function_declaration_test2() {
        let mut precompiler = Precompiler::new(Vec::new().into_iter());
        precompiler
            .variables
            .insert("a".to_string().into())
            .unwrap();
        precompiler
            .variables
            .insert("b".to_string().into())
            .unwrap();

        let function_declaration = FunctionDeclaration {
            name: "name_1".to_string().into(),
            args: vec!["a".to_string().into(), "b".to_string().into()],
            body: BlockStatement {
                body: vec![Expression::VariableAssigment(VariableAssigment {
                    left: "a".to_string().into(),
                    right: RightAssigmentValue::Identifier("b".to_string().into()),
                })],
            },
        };

        assert_eq!(
            function_declaration.precompile(&mut precompiler),
            Ok(llvm_ast::FunctionDeclaration {
                name: "name_1".to_string(),
                args: vec!["a".to_string(), "b".to_string()],
                body: vec![llvm_ast::Expression::VariableAssigment(
                    llvm_ast::VariableAssigment {
                        name: "a".to_string(),
                        value: llvm_ast::VariableValue::Identifier("b".to_string())
                    }
                )]
            })
        );
        assert_eq!(precompiler.variables.len(), 2);
        assert!(precompiler
            .functions
            .contains(&"name_1".to_string().into(),));
    }

    #[test]
    fn precompile_function_declaration_error_test() {
        let mut precompiler = Precompiler::new(Vec::new().into_iter());
        precompiler
            .functions
            .insert("name_1".to_string().into())
            .unwrap();

        let function_declaration = FunctionDeclaration {
            name: "name_1".to_string().into(),

            args: vec![],
            body: BlockStatement { body: vec![] },
        };

        assert_eq!(
            function_declaration.precompile(&mut precompiler),
            Err(precompiler::Error::AlreadyDeclaredFunction(
                "name_1".to_string().into(),
            ))
        );
    }
}
