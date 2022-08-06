use super::{Identifier, VariableExpression};
use crate::llvm_ast;
use lexer::{Parser, Token, TokenReader};
use precompiler::{self, Precompile, Precompiler};
use std::io::Read;

/// VariableAssigment - Expression type for variable assigment, like "a = 4"
#[derive(Clone, Debug, PartialEq)]
pub struct VariableAssigment {
    pub left: Identifier,
    pub right: VariableExpression,
}

impl Parser for VariableAssigment {
    fn parse<R: Read>(cur_token: Token, reader: &mut TokenReader<R>) -> Result<Self, lexer::Error> {
        let left = Identifier::parse(cur_token, reader)?;

        match reader.next_token()? {
            Token::Assign => {}
            token => return Err(lexer::Error::UnexpectedToken(token)),
        }

        let right = VariableExpression::parse(reader.next_token()?, reader)?;
        Ok(Self { left, right })
    }
}

impl Precompile<Identifier, llvm_ast::FunctionDeclaration> for VariableAssigment {
    type Output = llvm_ast::VariableAssigment;
    fn precompile(
        self,
        precompiler: &mut Precompiler<Identifier, llvm_ast::FunctionDeclaration>,
    ) -> Result<Self::Output, precompiler::Error<Identifier>> {
        match precompiler.variables.get(&self.left) {
            Some(index) => {
                let value = self.right.precompile(precompiler)?;
                Ok(llvm_ast::VariableAssigment {
                    name: llvm_ast::Identifier::new(self.left.name, index),
                    value,
                })
            }
            None => Err(precompiler::Error::UndefinedVariable(self.left)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::js_ast::{VariableExpression, VariableValue};

    #[test]
    fn parse_assigment_expression_test() {
        let mut reader = TokenReader::new("name = 12;".as_bytes());
        assert_eq!(
            VariableAssigment::parse(reader.next_token().unwrap(), &mut reader),
            Ok(VariableAssigment {
                left: "name".to_string().into(),
                right: VariableExpression::VariableValue(VariableValue::Number(12_f64))
            })
        );

        let mut reader = TokenReader::new("name1 = name2;".as_bytes());
        assert_eq!(
            VariableAssigment::parse(reader.next_token().unwrap(), &mut reader),
            Ok(VariableAssigment {
                left: "name1".to_string().into(),
                right: VariableExpression::VariableValue(VariableValue::Identifier(
                    "name2".to_string().into()
                ))
            })
        );
    }

    #[test]
    fn precompile_variable_assigment_test_1() {
        let mut precompiler = Precompiler::new(Vec::new().into_iter());
        precompiler.variables.insert("name_1".to_string().into());

        let variable_assigment = VariableAssigment {
            left: "name_1".to_string().into(),
            right: VariableExpression::VariableValue(VariableValue::Number(64_f64)),
        };

        assert_eq!(
            variable_assigment.precompile(&mut precompiler),
            Ok(llvm_ast::VariableAssigment {
                name: llvm_ast::Identifier::new("name_1".to_string(), 0),
                value: llvm_ast::VariableValue::FloatNumber(64_f64),
            })
        );
        assert_eq!(
            precompiler.variables.get(&"name_1".to_string().into()),
            Some(0)
        );
    }

    #[test]
    fn precompile_variable_assigment_test_2() {
        let mut precompiler = Precompiler::new(Vec::new().into_iter());
        precompiler.variables.insert("name_2".to_string().into());
        precompiler.variables.insert("name_1".to_string().into());

        let variable_assigment = VariableAssigment {
            left: "name_1".to_string().into(),
            right: VariableExpression::VariableValue(VariableValue::Identifier(
                "name_2".to_string().into(),
            )),
        };

        assert_eq!(
            variable_assigment.precompile(&mut precompiler),
            Ok(llvm_ast::VariableAssigment {
                name: llvm_ast::Identifier::new("name_1".to_string(), 0),
                value: llvm_ast::VariableValue::Identifier(llvm_ast::Identifier::new(
                    "name_2".to_string(),
                    0
                )),
            })
        );
        assert_eq!(
            precompiler.variables.get(&"name_1".to_string().into()),
            Some(0)
        );
    }

    #[test]
    fn precompile_variable_assigment_error_test_1() {
        let mut precompiler = Precompiler::new(Vec::new().into_iter());

        let variable_assigment = VariableAssigment {
            left: "name_1".to_string().into(),
            right: VariableExpression::VariableValue(VariableValue::Number(64_f64)),
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
        precompiler.variables.insert("name_1".to_string().into());

        let variable_assigment = VariableAssigment {
            left: "name_1".to_string().into(),
            right: VariableExpression::VariableValue(VariableValue::Identifier(
                "name_2".to_string().into(),
            )),
        };

        assert_eq!(
            variable_assigment.precompile(&mut precompiler),
            Err(precompiler::Error::UndefinedVariable(
                "name_2".to_string().into(),
            ))
        );
    }
}
