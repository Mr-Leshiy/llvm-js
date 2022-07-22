use super::{Identifier, VariableValue};
use crate::{
    llvm_ast,
    precompiler::{self, Precompile, Precompiler},
};
use lexer::{Parser, Token, TokenReader};
use std::io::Read;

/// VariableAssigment - Expression type for variable assigment, like "a = 4"
#[derive(Clone, Debug, PartialEq)]
pub struct VariableAssigment {
    pub left: Identifier,
    pub right: VariableValue,
}

impl Parser for VariableAssigment {
    fn parse<R: Read>(cur_token: Token, reader: &mut TokenReader<R>) -> Result<Self, lexer::Error> {
        let left = Identifier::parse(cur_token, reader)?;

        match reader.next_token()? {
            Token::Assign => {}
            token => return Err(lexer::Error::UnexpectedToken(token)),
        }

        let right = VariableValue::parse(reader.next_token()?, reader)?;
        Ok(Self { left, right })
    }
}

impl Precompile for VariableAssigment {
    type Output = llvm_ast::VariableAssigment;
    fn precompile(self, precompiler: &mut Precompiler) -> Result<Self::Output, precompiler::Error> {
        if precompiler.variables.contains(&self.left) {
            match self.right {
                VariableValue::Number(value) => Ok(llvm_ast::VariableAssigment {
                    name: self.left.name.clone(),
                    value: llvm_ast::VariableValue::FloatNumber(value),
                }),
                VariableValue::String(value) => Ok(llvm_ast::VariableAssigment {
                    name: self.left.name.clone(),
                    value: llvm_ast::VariableValue::String(value),
                }),
                VariableValue::Identifier(identifier) => {
                    precompiler
                        .variables
                        .contains(&identifier)
                        .then(|| ())
                        .ok_or_else(|| precompiler::Error::UndefinedVariable(identifier.clone()))?;
                    Ok(llvm_ast::VariableAssigment {
                        name: self.left.name.clone(),
                        value: llvm_ast::VariableValue::Identifier(identifier.name),
                    })
                }
            }
        } else {
            Err(precompiler::Error::UndefinedVariable(self.left))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::js_ast::VariableValue;

    #[test]
    fn parse_assigment_expression_test() {
        let mut reader = TokenReader::new("name = 12;".as_bytes());
        assert_eq!(
            VariableAssigment::parse(reader.next_token().unwrap(), &mut reader),
            Ok(VariableAssigment {
                left: "name".to_string().into(),
                right: VariableValue::Number(12_f64)
            })
        );

        let mut reader = TokenReader::new("name1 = name2;".as_bytes());
        assert_eq!(
            VariableAssigment::parse(reader.next_token().unwrap(), &mut reader),
            Ok(VariableAssigment {
                left: "name1".to_string().into(),
                right: VariableValue::Identifier("name2".to_string().into())
            })
        );
    }

    #[test]
    fn precompile_variable_assigment_test_1() {
        let mut precompiler = Precompiler::new(Vec::new().into_iter());
        precompiler
            .variables
            .insert("name_1".to_string().into())
            .unwrap();

        let variable_assigment = VariableAssigment {
            left: "name_1".to_string().into(),
            right: VariableValue::Number(64_f64),
        };

        assert_eq!(
            variable_assigment.precompile(&mut precompiler),
            Ok(llvm_ast::VariableAssigment {
                name: "name_1".to_string(),
                value: llvm_ast::VariableValue::FloatNumber(64_f64),
            })
        );
        assert!(precompiler.variables.contains(&"name_1".to_string().into()),);
    }

    #[test]
    fn precompile_variable_assigment_test_2() {
        let mut precompiler = Precompiler::new(Vec::new().into_iter());
        precompiler
            .variables
            .insert("name_2".to_string().into())
            .unwrap();
        precompiler
            .variables
            .insert("name_1".to_string().into())
            .unwrap();

        let variable_assigment = VariableAssigment {
            left: "name_1".to_string().into(),
            right: VariableValue::Identifier("name_2".to_string().into()),
        };

        assert_eq!(
            variable_assigment.precompile(&mut precompiler),
            Ok(llvm_ast::VariableAssigment {
                name: "name_1".to_string(),
                value: llvm_ast::VariableValue::Identifier("name_2".to_string()),
            })
        );
        assert!(precompiler.variables.contains(&"name_1".to_string().into()));
    }

    #[test]
    fn precompile_variable_assigment_error_test_1() {
        let mut precompiler = Precompiler::new(Vec::new().into_iter());

        let variable_assigment = VariableAssigment {
            left: "name_1".to_string().into(),
            right: VariableValue::Number(64_f64),
        };

        assert_eq!(
            variable_assigment.precompile(&mut precompiler),
            Err(precompiler::Error::UndefinedVariable(
                "name_1".to_string().into(),
            ))
        );
    }

    #[test]
    fn precompile_variable_assigment_error_test_2() {
        let mut precompiler = Precompiler::new(Vec::new().into_iter());
        precompiler
            .variables
            .insert("name_1".to_string().into())
            .unwrap();

        let variable_assigment = VariableAssigment {
            left: "name_1".to_string().into(),
            right: VariableValue::Identifier("name_2".to_string().into()),
        };

        assert_eq!(
            variable_assigment.precompile(&mut precompiler),
            Err(precompiler::Error::UndefinedVariable(
                "name_2".to_string().into(),
            ))
        );
    }
}