use super::VariableExpression;
use crate::{llvm_ast, LexerError, Precompiler, PrecompilerError};
use lexer::{Token, TokenReader};
use std::io::Read;

#[derive(Clone, Debug, PartialEq)]
pub struct VariableAssigment {
    pub left: VariableExpression,
    pub right: VariableExpression,
}

impl VariableAssigment {
    pub fn parse<R: Read>(
        cur_token: Token,
        reader: &mut TokenReader<R>,
    ) -> Result<Self, LexerError> {
        let left = VariableExpression::parse(cur_token, reader)?;
        match reader.next_token()? {
            Token::Assign => {
                let right = VariableExpression::parse(reader.next_token()?, reader)?;
                Ok(Self { left, right })
            }
            token => Err(LexerError::UnexpectedToken(token)),
        }
    }
}

impl VariableAssigment {
    pub fn precompile(
        self,
        precompiler: &mut Precompiler,
    ) -> Result<llvm_ast::VariableAssigment, PrecompilerError> {
        let left = self.left.precompile(precompiler)?;
        let right = self.right.precompile(precompiler)?;
        Ok(llvm_ast::VariableAssigment { left, right })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::js_ast::{
        BinaryExpType, BinaryExpression, FunctionCall, ObjectExpression, VariableExpression,
        VariableValue,
    };

    #[test]
    fn parse_assigment_expression_test() {
        let mut reader = TokenReader::new("name = 12;".as_bytes());
        assert_eq!(
            VariableAssigment::parse(reader.next_token().unwrap(), &mut reader).unwrap(),
            VariableAssigment {
                left: VariableExpression::VariableValue(VariableValue::Identifier(
                    "name".to_string().into()
                )),
                right: VariableExpression::VariableValue(VariableValue::Number(12_f64))
            }
        );

        let mut reader = TokenReader::new("name1 = name2;".as_bytes());
        assert_eq!(
            VariableAssigment::parse(reader.next_token().unwrap(), &mut reader).unwrap(),
            VariableAssigment {
                left: VariableExpression::VariableValue(VariableValue::Identifier(
                    "name1".to_string().into()
                )),
                right: VariableExpression::VariableValue(VariableValue::Identifier(
                    "name2".to_string().into()
                ))
            }
        );

        let mut reader = TokenReader::new("foo() = name2;".as_bytes());
        assert_eq!(
            VariableAssigment::parse(reader.next_token().unwrap(), &mut reader).unwrap(),
            VariableAssigment {
                left: VariableExpression::FunctionCall(FunctionCall {
                    name: "foo".to_string().into(),
                    args: vec![]
                }),
                right: VariableExpression::VariableValue(VariableValue::Identifier(
                    "name2".to_string().into()
                ))
            }
        );

        let mut reader = TokenReader::new("(1 + 2) = name2;".as_bytes());
        assert_eq!(
            VariableAssigment::parse(reader.next_token().unwrap(), &mut reader).unwrap(),
            VariableAssigment {
                left: VariableExpression::BinaryExpression(
                    BinaryExpression {
                        left: VariableExpression::VariableValue(VariableValue::Number(1_f64)),
                        right: VariableExpression::VariableValue(VariableValue::Number(2_f64)),
                        exp_type: BinaryExpType::Add,
                    }
                    .into()
                ),
                right: VariableExpression::VariableValue(VariableValue::Identifier(
                    "name2".to_string().into()
                ))
            }
        );

        let mut reader = TokenReader::new(r#"{name: "Alex"} = name2;"#.as_bytes());
        assert_eq!(
            VariableAssigment::parse(reader.next_token().unwrap(), &mut reader).unwrap(),
            VariableAssigment {
                left: VariableExpression::VariableValue(VariableValue::ObjectExpression(
                    ObjectExpression {
                        properties: vec![(
                            "name".to_string().into(),
                            VariableExpression::VariableValue(VariableValue::String(
                                "Alex".to_string()
                            ))
                        )]
                        .into_iter()
                        .collect(),
                    }
                )),
                right: VariableExpression::VariableValue(VariableValue::Identifier(
                    "name2".to_string().into()
                ))
            }
        );
    }

    #[test]
    fn precompile_variable_assigment_test_1() {
        let mut precompiler = Precompiler::new(std::iter::empty());
        precompiler.insert_variable("name_1".to_string().into());

        let variable_assigment = VariableAssigment {
            left: VariableExpression::VariableValue(VariableValue::Identifier(
                "name_1".to_string().into(),
            )),
            right: VariableExpression::VariableValue(VariableValue::Number(64_f64)),
        };

        assert_eq!(
            variable_assigment.precompile(&mut precompiler),
            Ok(llvm_ast::VariableAssigment {
                left: llvm_ast::VariableExpression::VariableValue(
                    llvm_ast::VariableValue::Identifier(llvm_ast::Identifier::new(
                        "name_1".to_string(),
                        0
                    ))
                ),
                right: llvm_ast::VariableExpression::VariableValue(
                    llvm_ast::VariableValue::FloatNumber(64_f64)
                ),
            })
        );
        assert_eq!(precompiler.get_variable("name_1".to_string().into()), Ok(0));
    }

    #[test]
    fn precompile_variable_assigment_test_2() {
        let mut precompiler = Precompiler::new(std::iter::empty());
        precompiler.insert_variable("name_2".to_string().into());
        precompiler.insert_variable("name_1".to_string().into());

        let variable_assigment = VariableAssigment {
            left: VariableExpression::VariableValue(VariableValue::Identifier(
                "name_1".to_string().into(),
            )),
            right: VariableExpression::VariableValue(VariableValue::Identifier(
                "name_2".to_string().into(),
            )),
        };

        assert_eq!(
            variable_assigment.precompile(&mut precompiler),
            Ok(llvm_ast::VariableAssigment {
                left: llvm_ast::VariableExpression::VariableValue(
                    llvm_ast::VariableValue::Identifier(llvm_ast::Identifier::new(
                        "name_1".to_string(),
                        0
                    ))
                ),
                right: llvm_ast::VariableExpression::VariableValue(
                    llvm_ast::VariableValue::Identifier(llvm_ast::Identifier::new(
                        "name_2".to_string(),
                        0
                    ))
                ),
            })
        );
        assert_eq!(precompiler.get_variable("name_1".to_string().into()), Ok(0));
    }

    #[test]
    fn precompile_variable_assigment_error_test_1() {
        let mut precompiler = Precompiler::new(std::iter::empty());

        let variable_assigment = VariableAssigment {
            left: VariableExpression::VariableValue(VariableValue::Identifier(
                "name_1".to_string().into(),
            )),
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
        let mut precompiler = Precompiler::new(std::iter::empty());
        precompiler.insert_variable("name_1".to_string().into());

        let variable_assigment = VariableAssigment {
            left: VariableExpression::VariableValue(VariableValue::Identifier(
                "name_1".to_string().into(),
            )),
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
