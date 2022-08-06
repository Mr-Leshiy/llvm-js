use super::{Identifier, VariableAssigment};
use crate::llvm_ast;
use lexer::{Keyword, Parser, Token, TokenReader};
use precompiler::{self, Precompile, Precompiler};
use std::io::Read;

/// VariableDeclaration - Expression type for variable assigment, like "var a = 4"
#[derive(Clone, Debug, PartialEq)]
pub struct VariableDeclaration(pub VariableAssigment);

impl Parser for VariableDeclaration {
    fn parse<R: Read>(cur_token: Token, reader: &mut TokenReader<R>) -> Result<Self, lexer::Error> {
        match cur_token {
            Token::Keyword(Keyword::Var) => Ok(Self(VariableAssigment::parse(
                reader.next_token()?,
                reader,
            )?)),
            token => Err(lexer::Error::UnexpectedToken(token)),
        }
    }
}

impl Precompile<Identifier, llvm_ast::FunctionDeclaration> for VariableDeclaration {
    type Output = llvm_ast::VariableDeclaration;
    fn precompile(
        self,
        precompiler: &mut Precompiler<Identifier, llvm_ast::FunctionDeclaration>,
    ) -> Result<Self::Output, precompiler::Error<Identifier>> {
        let value = self.0.right.precompile(precompiler)?;
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
    fn parse_variable_declaration_test() {
        let mut reader = TokenReader::new("var name = 12;".as_bytes());
        assert_eq!(
            VariableDeclaration::parse(reader.next_token().unwrap(), &mut reader),
            Ok(VariableDeclaration(VariableAssigment {
                left: "name".to_string().into(),
                right: VariableExpression::VariableValue(VariableValue::Number(12_f64))
            }))
        );

        let mut reader = TokenReader::new("var name1 = name2;".as_bytes());
        assert_eq!(
            VariableDeclaration::parse(reader.next_token().unwrap(), &mut reader),
            Ok(VariableDeclaration(VariableAssigment {
                left: "name1".to_string().into(),
                right: VariableExpression::VariableValue(VariableValue::Identifier(
                    "name2".to_string().into()
                ))
            }))
        );
    }

    #[test]
    fn precompile_variable_declaration_test_1() {
        let mut precompiler = Precompiler::new(Vec::new().into_iter());

        let variable_declaration = VariableDeclaration(VariableAssigment {
            left: "name_1".to_string().into(),
            right: VariableExpression::VariableValue(VariableValue::Number(64_f64)),
        });

        assert_eq!(
            variable_declaration.precompile(&mut precompiler),
            Ok(llvm_ast::VariableDeclaration(llvm_ast::VariableAssigment {
                name: llvm_ast::Identifier::new("name_1".to_string(), 0),
                value: llvm_ast::VariableValue::FloatNumber(64_f64),
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
            right: VariableExpression::VariableValue(VariableValue::Identifier(
                "name_2".to_string().into(),
            )),
        });

        assert_eq!(
            variable_declaration.precompile(&mut precompiler),
            Ok(llvm_ast::VariableDeclaration(llvm_ast::VariableAssigment {
                name: llvm_ast::Identifier::new("name_1".to_string(), 0),
                value: llvm_ast::VariableValue::Identifier(llvm_ast::Identifier::new(
                    "name_2".to_string(),
                    0
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
            right: VariableExpression::VariableValue(VariableValue::Number(64_f64)),
        });

        assert_eq!(
            variable_declaration.precompile(&mut precompiler),
            Ok(llvm_ast::VariableDeclaration(llvm_ast::VariableAssigment {
                name: llvm_ast::Identifier::new("name_1".to_string(), 1),
                value: llvm_ast::VariableValue::FloatNumber(64_f64),
            }))
        );
    }

    #[test]
    fn precompile_variable_declaration_error_test_2() {
        let mut precompiler = Precompiler::new(Vec::new().into_iter());

        let variable_declaration = VariableDeclaration(VariableAssigment {
            left: "name_1".to_string().into(),
            right: VariableExpression::VariableValue(VariableValue::Identifier(
                "name_2".to_string().into(),
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
