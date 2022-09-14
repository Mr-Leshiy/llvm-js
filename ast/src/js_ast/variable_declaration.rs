use super::{Identifier, VariableAssigment};
use crate::{llvm_ast, Error};
use lexer::{Keyword, Token, TokenReader};
use precompiler::{self, Precompiler};
use std::io::Read;

/// VariableDeclaration - Expression type for variable assigment, like "var a = 4" or "let a = 4"
#[derive(Clone, Debug, PartialEq)]
pub struct VariableDeclaration(pub VariableAssigment);

impl VariableDeclaration {
    pub fn parse<R: Read>(cur_token: Token, reader: &mut TokenReader<R>) -> Result<Self, Error> {
        match cur_token {
            Token::Keyword(Keyword::Var) => Ok(Self(VariableAssigment::parse(
                reader.next_token()?,
                reader,
            )?)),
            Token::Keyword(Keyword::Let) => Ok(Self(VariableAssigment::parse(
                reader.next_token()?,
                reader,
            )?)),
            token => Err(Error::UnexpectedToken(token)),
        }
    }
}

impl VariableDeclaration {
    pub fn precompile(
        self,
        precompiler: &mut Precompiler<Identifier, llvm_ast::FunctionDeclaration>,
    ) -> Result<llvm_ast::VariableDeclaration, precompiler::Error<Identifier>> {
        let value = match self.0.right {
            Some(expr) => Some(expr.precompile(precompiler)?),
            None => None,
        };
        let index = precompiler.variables.insert(self.0.left.clone());
        let res = llvm_ast::VariableAssigment {
            name: llvm_ast::Identifier::new(self.0.left.name, index),
            value,
        };
        Ok(llvm_ast::VariableDeclaration(res))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::js_ast::{VariableExpression, VariableValue};

    #[test]
    fn parse_variable_declaration_test1() {
        let mut reader = TokenReader::new("var name = 12;".as_bytes());
        assert_eq!(
            VariableDeclaration::parse(reader.next_token().unwrap(), &mut reader),
            Ok(VariableDeclaration(VariableAssigment {
                left: "name".to_string().into(),
                right: Some(VariableExpression::VariableValue(VariableValue::Number(
                    12_f64
                )))
            }))
        );

        let mut reader = TokenReader::new("var name1 = name2;".as_bytes());
        assert_eq!(
            VariableDeclaration::parse(reader.next_token().unwrap(), &mut reader),
            Ok(VariableDeclaration(VariableAssigment {
                left: "name1".to_string().into(),
                right: Some(VariableExpression::VariableValue(
                    VariableValue::Identifier("name2".to_string().into())
                ))
            }))
        );

        let mut reader = TokenReader::new("var name1;".as_bytes());
        assert_eq!(
            VariableDeclaration::parse(reader.next_token().unwrap(), &mut reader),
            Ok(VariableDeclaration(VariableAssigment {
                left: "name1".to_string().into(),
                right: None
            }))
        );
    }

    #[test]
    fn parse_variable_declaration_test2() {
        let mut reader = TokenReader::new("let name = 12;".as_bytes());
        assert_eq!(
            VariableDeclaration::parse(reader.next_token().unwrap(), &mut reader),
            Ok(VariableDeclaration(VariableAssigment {
                left: "name".to_string().into(),
                right: Some(VariableExpression::VariableValue(VariableValue::Number(
                    12_f64
                )))
            }))
        );

        let mut reader = TokenReader::new("let name1 = name2;".as_bytes());
        assert_eq!(
            VariableDeclaration::parse(reader.next_token().unwrap(), &mut reader),
            Ok(VariableDeclaration(VariableAssigment {
                left: "name1".to_string().into(),
                right: Some(VariableExpression::VariableValue(
                    VariableValue::Identifier("name2".to_string().into())
                ))
            }))
        );

        let mut reader = TokenReader::new("let name1;".as_bytes());
        assert_eq!(
            VariableDeclaration::parse(reader.next_token().unwrap(), &mut reader),
            Ok(VariableDeclaration(VariableAssigment {
                left: "name1".to_string().into(),
                right: None
            }))
        );
    }

    #[test]
    fn precompile_variable_declaration_test_1() {
        let mut precompiler = Precompiler::new(Vec::new().into_iter());

        let variable_declaration = VariableDeclaration(VariableAssigment {
            left: "name_1".to_string().into(),
            right: Some(VariableExpression::VariableValue(VariableValue::Number(
                64_f64,
            ))),
        });

        assert_eq!(
            variable_declaration.precompile(&mut precompiler),
            Ok(llvm_ast::VariableDeclaration(llvm_ast::VariableAssigment {
                name: llvm_ast::Identifier::new("name_1".to_string(), 0),
                value: Some(llvm_ast::VariableExpression::VariableValue(
                    llvm_ast::VariableValue::FloatNumber(64_f64)
                )),
            }))
        );
        assert_eq!(
            precompiler.variables.get(&"name_1".to_string().into()),
            Some(0)
        );
    }

    #[test]
    fn precompile_variable_declaration_test_2() {
        let mut precompiler = Precompiler::new(Vec::new().into_iter());
        precompiler.variables.insert("name_2".to_string().into());

        let variable_declaration = VariableDeclaration(VariableAssigment {
            left: "name_1".to_string().into(),
            right: Some(VariableExpression::VariableValue(
                VariableValue::Identifier("name_2".to_string().into()),
            )),
        });

        assert_eq!(
            variable_declaration.precompile(&mut precompiler),
            Ok(llvm_ast::VariableDeclaration(llvm_ast::VariableAssigment {
                name: llvm_ast::Identifier::new("name_1".to_string(), 0),
                value: Some(llvm_ast::VariableExpression::VariableValue(
                    llvm_ast::VariableValue::Identifier(llvm_ast::Identifier::new(
                        "name_2".to_string(),
                        0
                    ))
                )),
            }))
        );
        assert_eq!(
            precompiler.variables.get(&"name_1".to_string().into()),
            Some(0)
        );
    }

    #[test]
    fn precompile_variable_declaration_test_3() {
        let mut precompiler = Precompiler::new(Vec::new().into_iter());
        precompiler.variables.insert("name_1".to_string().into());

        let variable_declaration = VariableDeclaration(VariableAssigment {
            left: "name_1".to_string().into(),
            right: Some(VariableExpression::VariableValue(VariableValue::Number(
                64_f64,
            ))),
        });

        assert_eq!(
            variable_declaration.precompile(&mut precompiler),
            Ok(llvm_ast::VariableDeclaration(llvm_ast::VariableAssigment {
                name: llvm_ast::Identifier::new("name_1".to_string(), 1),
                value: Some(llvm_ast::VariableExpression::VariableValue(
                    llvm_ast::VariableValue::FloatNumber(64_f64)
                )),
            }))
        );
    }

    #[test]
    fn precompile_variable_declaration_error_test_2() {
        let mut precompiler = Precompiler::new(Vec::new().into_iter());

        let variable_declaration = VariableDeclaration(VariableAssigment {
            left: "name_1".to_string().into(),
            right: Some(VariableExpression::VariableValue(
                VariableValue::Identifier("name_2".to_string().into()),
            )),
        });

        assert_eq!(
            variable_declaration.precompile(&mut precompiler),
            Err(precompiler::Error::UndefinedVariable(
                "name_2".to_string().into(),
            ))
        );
    }
}
