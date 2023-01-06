use super::{MemberExpression, VariableExpression};
use crate::{llvm_ast, LexerError, Precompiler, PrecompilerError};
use lexer::{Token, TokenReader};
use std::io::Read;

/// VariableAssigment - Expression type for variable assigment, like "a = 4"
#[derive(Clone, Debug, PartialEq)]
pub struct VariableAssigment {
    pub left: MemberExpression,
    pub right: Option<VariableExpression>,
}

impl VariableAssigment {
    pub fn parse<R: Read>(
        cur_token: Token,
        reader: &mut TokenReader<R>,
    ) -> Result<Self, LexerError> {
        let left = MemberExpression::parse(cur_token, reader)?;

        reader.start_saving();
        match reader.next_token()? {
            Token::Assign => {
                reader.reset_saving();
                let right = Some(VariableExpression::parse(reader.next_token()?, reader)?);
                Ok(Self { left, right })
            }
            _ => {
                reader.stop_saving();
                Ok(Self { left, right: None })
            }
        }
    }
}

impl VariableAssigment {
    pub fn precompile(
        self,
        precompiler: &mut Precompiler,
    ) -> Result<llvm_ast::VariableAssigment, PrecompilerError> {
        let left = self.left.precompile(precompiler)?;
        let value = match self.right {
            Some(expr) => Some(expr.precompile(precompiler)?),
            None => None,
        };
        Ok(llvm_ast::VariableAssigment { left, right: value })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::js_ast::{MemberExpression, Property, VariableExpression, VariableValue};

    #[test]
    fn parse_assigment_expression_test() {
        let mut reader = TokenReader::new("name = 12;".as_bytes());
        assert_eq!(
            VariableAssigment::parse(reader.next_token().unwrap(), &mut reader),
            Ok(VariableAssigment {
                left: MemberExpression {
                    variable_name: "name".to_string().into(),
                    property: None
                },
                right: Some(VariableExpression::VariableValue(VariableValue::Number(
                    12_f64
                )))
            })
        );

        let mut reader = TokenReader::new("name1 = name2;".as_bytes());
        assert_eq!(
            VariableAssigment::parse(reader.next_token().unwrap(), &mut reader),
            Ok(VariableAssigment {
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
            })
        );

        let mut reader = TokenReader::new("name1;".as_bytes());
        assert_eq!(
            VariableAssigment::parse(reader.next_token().unwrap(), &mut reader),
            Ok(VariableAssigment {
                left: MemberExpression {
                    variable_name: "name1".to_string().into(),
                    property: None
                },
                right: None
            })
        );

        let mut reader = TokenReader::new("name.name".as_bytes());
        assert_eq!(
            VariableAssigment::parse(reader.next_token().unwrap(), &mut reader),
            Ok(VariableAssigment {
                left: MemberExpression {
                    variable_name: "name".to_string().into(),
                    property: Some(
                        Property {
                            object: VariableExpression::VariableValue(VariableValue::String(
                                "name".to_string()
                            )),
                            property: None,
                        }
                        .into()
                    ),
                },
                right: None
            })
        );
    }

    #[test]
    fn precompile_variable_assigment_test_1() {
        let mut precompiler = Precompiler::new(std::iter::empty());
        precompiler.insert_variable("name_1".to_string().into());

        let variable_assigment = VariableAssigment {
            left: MemberExpression {
                variable_name: "name_1".to_string().into(),
                property: None,
            },
            right: Some(VariableExpression::VariableValue(VariableValue::Number(
                64_f64,
            ))),
        };

        assert_eq!(
            variable_assigment.precompile(&mut precompiler),
            Ok(llvm_ast::VariableAssigment {
                left: llvm_ast::MemberExpression {
                    variable_name: llvm_ast::Identifier::new("name_1".to_string(), 0),
                    property: None,
                },
                right: Some(llvm_ast::VariableExpression::VariableValue(
                    llvm_ast::VariableValue::FloatNumber(64_f64)
                )),
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
            left: MemberExpression {
                variable_name: "name_1".to_string().into(),
                property: None,
            },
            right: Some(VariableExpression::VariableValue(
                VariableValue::MemberExpression(MemberExpression {
                    variable_name: "name_2".to_string().into(),
                    property: None,
                }),
            )),
        };

        assert_eq!(
            variable_assigment.precompile(&mut precompiler),
            Ok(llvm_ast::VariableAssigment {
                left: llvm_ast::MemberExpression {
                    variable_name: llvm_ast::Identifier::new("name_1".to_string(), 0),
                    property: None
                },
                right: Some(llvm_ast::VariableExpression::VariableValue(
                    llvm_ast::VariableValue::MemberExpression(llvm_ast::MemberExpression {
                        variable_name: llvm_ast::Identifier::new("name_2".to_string(), 0),
                        property: None,
                    })
                )),
            })
        );
        assert_eq!(precompiler.get_variable("name_1".to_string().into()), Ok(0));
    }

    #[test]
    fn precompile_variable_assigment_test_3() {
        let mut precompiler = Precompiler::new(std::iter::empty());
        precompiler.insert_variable("name_1".to_string().into());

        let variable_assigment = VariableAssigment {
            left: MemberExpression {
                variable_name: "name_1".to_string().into(),
                property: None,
            },
            right: None,
        };

        assert_eq!(
            variable_assigment.precompile(&mut precompiler),
            Ok(llvm_ast::VariableAssigment {
                left: llvm_ast::MemberExpression {
                    variable_name: llvm_ast::Identifier::new("name_1".to_string(), 0),
                    property: None
                },
                right: None,
            })
        );
        assert_eq!(precompiler.get_variable("name_1".to_string().into()), Ok(0));
    }

    #[test]
    fn precompile_variable_assigment_error_test_1() {
        let mut precompiler = Precompiler::new(std::iter::empty());

        let variable_assigment = VariableAssigment {
            left: MemberExpression {
                variable_name: "name_1".to_string().into(),
                property: None,
            },
            right: Some(VariableExpression::VariableValue(VariableValue::Number(
                64_f64,
            ))),
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
            left: MemberExpression {
                variable_name: "name_1".to_string().into(),
                property: None,
            },
            right: Some(VariableExpression::VariableValue(
                VariableValue::MemberExpression(MemberExpression {
                    variable_name: "name_2".to_string().into(),
                    property: None,
                }),
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
