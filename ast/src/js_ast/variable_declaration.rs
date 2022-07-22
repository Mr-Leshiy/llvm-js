use super::{VariableAssigment, VariableValue};
use crate::{
    llvm_ast,
    precompiler::{self, Precompile, Precompiler},
};
use lexer::{Keyword, Parser, Token, TokenReader};
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

impl Precompile for VariableDeclaration {
    type Output = llvm_ast::VariableDeclaration;
    fn precompile(self, precompiler: &mut Precompiler) -> Result<Self::Output, precompiler::Error> {
        let res = match self.0.right {
            VariableValue::Number(value) => llvm_ast::VariableAssigment {
                name: self.0.left.name.clone(),
                value: llvm_ast::VariableValue::FloatNumber(value),
            },
            VariableValue::String(value) => llvm_ast::VariableAssigment {
                name: self.0.left.name.clone(),
                value: llvm_ast::VariableValue::String(value),
            },
            VariableValue::Identifier(identifier) => {
                precompiler
                    .variables
                    .contains(&identifier)
                    .then(|| ())
                    .ok_or_else(|| precompiler::Error::UndefinedVariable(identifier.clone()))?;
                llvm_ast::VariableAssigment {
                    name: self.0.left.name.clone(),
                    value: llvm_ast::VariableValue::Identifier(identifier.name),
                }
            }
        };
        precompiler
            .variables
            .insert(self.0.left.clone())
            .map_err(|_| precompiler::Error::AlreadyDeclaredVariable(self.0.left))?;
        Ok(llvm_ast::VariableDeclaration(res))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::js_ast::VariableValue;

    #[test]
    fn parse_variable_declaration_test() {
        let mut reader = TokenReader::new("var name = 12;".as_bytes());
        assert_eq!(
            VariableDeclaration::parse(reader.next_token().unwrap(), &mut reader),
            Ok(VariableDeclaration(VariableAssigment {
                left: "name".to_string().into(),
                right: VariableValue::Number(12_f64)
            }))
        );

        let mut reader = TokenReader::new("var name1 = name2;".as_bytes());
        assert_eq!(
            VariableDeclaration::parse(reader.next_token().unwrap(), &mut reader),
            Ok(VariableDeclaration(VariableAssigment {
                left: "name1".to_string().into(),
                right: VariableValue::Identifier("name2".to_string().into())
            }))
        );
    }

    #[test]
    fn precompile_variable_declaration_test_1() {
        let mut precompiler = Precompiler::new(Vec::new().into_iter());

        let variable_declaration = VariableDeclaration(VariableAssigment {
            left: "name_1".to_string().into(),
            right: VariableValue::Number(64_f64),
        });

        assert_eq!(
            variable_declaration.precompile(&mut precompiler),
            Ok(llvm_ast::VariableDeclaration(llvm_ast::VariableAssigment {
                name: "name_1".to_string(),
                value: llvm_ast::VariableValue::FloatNumber(64_f64),
            }))
        );
        assert!(precompiler
            .variables
            .contains(&"name_1".to_string().into(),));
    }

    #[test]
    fn precompile_variable_declaration_test_2() {
        let mut precompiler = Precompiler::new(Vec::new().into_iter());
        precompiler
            .variables
            .insert("name_2".to_string().into())
            .unwrap();

        let variable_declaration = VariableDeclaration(VariableAssigment {
            left: "name_1".to_string().into(),
            right: VariableValue::Identifier("name_2".to_string().into()),
        });

        assert_eq!(
            variable_declaration.precompile(&mut precompiler),
            Ok(llvm_ast::VariableDeclaration(llvm_ast::VariableAssigment {
                name: "name_1".to_string(),
                value: llvm_ast::VariableValue::Identifier("name_2".to_string()),
            }))
        );
        assert!(precompiler.variables.contains(&"name_1".to_string().into()));
    }

    #[test]
    fn precompile_variable_declaration_error_test_1() {
        let mut precompiler = Precompiler::new(Vec::new().into_iter());
        precompiler
            .variables
            .insert("name_1".to_string().into())
            .unwrap();

        let variable_declaration = VariableDeclaration(VariableAssigment {
            left: "name_1".to_string().into(),
            right: VariableValue::Number(64_f64),
        });

        assert_eq!(
            variable_declaration.precompile(&mut precompiler),
            Err(precompiler::Error::AlreadyDeclaredVariable(
                "name_1".to_string().into(),
            ))
        );
    }

    #[test]
    fn precompile_variable_declaration_error_test_2() {
        let mut precompiler = Precompiler::new(Vec::new().into_iter());

        let variable_declaration = VariableDeclaration(VariableAssigment {
            left: "name_1".to_string().into(),
            right: VariableValue::Identifier("name_2".to_string().into()),
        });

        assert_eq!(
            variable_declaration.precompile(&mut precompiler),
            Err(precompiler::Error::UndefinedVariable(
                "name_2".to_string().into(),
            ))
        );
    }
}