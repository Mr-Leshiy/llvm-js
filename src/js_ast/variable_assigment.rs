use super::{Identifier, Literal, RightAssigmentValue};
use crate::{
    lexer::{self, CharReader, Token},
    llvm_ast,
    parser::{self, Parser},
    precompiler::{self, Precompile, Precompiler},
};
use std::io::Read;

/// VariableAssigment - Expression type for variable assigment, like "a = 4"
#[derive(Clone, Debug, PartialEq)]
pub struct VariableAssigment {
    pub left: Identifier,
    pub right: RightAssigmentValue,
}

impl Parser for VariableAssigment {
    fn parse<R: Read>(cur_token: Token, reader: &mut CharReader<R>) -> Result<Self, parser::Error> {
        let left = Identifier::parse(cur_token, reader)?;

        match lexer::get_token(reader)? {
            Token::Assign => {}
            token => return Err(parser::Error::UnexpectedToken(token)),
        }

        let right = RightAssigmentValue::parse(lexer::get_token(reader)?, reader)?;
        Ok(Self { left, right })
    }
}

impl Precompile for VariableAssigment {
    type Output = llvm_ast::VariableAssigment;
    fn precompile(self, precompiler: &mut Precompiler) -> Result<Self::Output, precompiler::Error> {
        if precompiler.variables.contains(&self.left) {
            match self.right {
                RightAssigmentValue::Literal(literal) => match literal {
                    Literal::Number(value) => Ok(llvm_ast::VariableAssigment {
                        name: self.left.name.clone(),
                        value: llvm_ast::VariableValue::FloatNumber(value),
                    }),
                    Literal::String(value) => Ok(llvm_ast::VariableAssigment {
                        name: self.left.name.clone(),
                        value: llvm_ast::VariableValue::String(value),
                    }),
                },
                RightAssigmentValue::Identifier(identifier) => {
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
    use crate::js_ast::{Identifier, Literal, RightAssigmentValue};

    #[test]
    fn parse_assigment_expression_test() {
        let mut reader = CharReader::new("name = 12;".as_bytes());
        assert_eq!(
            VariableAssigment::parse(lexer::get_token(&mut reader).unwrap(), &mut reader).unwrap(),
            VariableAssigment {
                left: Identifier {
                    name: "name".to_string()
                },
                right: RightAssigmentValue::Literal(Literal::Number(12_f64))
            }
        );

        let mut reader = CharReader::new("name1 = name2;".as_bytes());
        assert_eq!(
            VariableAssigment::parse(lexer::get_token(&mut reader).unwrap(), &mut reader).unwrap(),
            VariableAssigment {
                left: Identifier {
                    name: "name1".to_string()
                },
                right: RightAssigmentValue::Identifier(Identifier {
                    name: "name2".to_string()
                })
            }
        );
    }

    #[test]
    fn precompile_variable_assigment_test_1() {
        let mut precompiler = Precompiler::new();
        precompiler
            .variables
            .insert(Identifier {
                name: "name_1".to_string(),
            })
            .unwrap();

        let variable_assigment = VariableAssigment {
            left: Identifier {
                name: "name_1".to_string(),
            },
            right: RightAssigmentValue::Literal(Literal::Number(64_f64)),
        };

        assert_eq!(
            variable_assigment.precompile(&mut precompiler),
            Ok(llvm_ast::VariableAssigment {
                name: "name_1".to_string(),
                value: llvm_ast::VariableValue::FloatNumber(64_f64),
            })
        );
        assert!(precompiler.variables.contains(&Identifier {
            name: "name_1".to_string(),
        }),);
    }

    #[test]
    fn precompile_variable_assigment_test_2() {
        let mut precompiler = Precompiler::new();
        precompiler
            .variables
            .insert(Identifier {
                name: "name_2".to_string(),
            })
            .unwrap();
        precompiler
            .variables
            .insert(Identifier {
                name: "name_1".to_string(),
            })
            .unwrap();

        let variable_assigment = VariableAssigment {
            left: Identifier {
                name: "name_1".to_string(),
            },
            right: RightAssigmentValue::Identifier(Identifier {
                name: "name_2".to_string(),
            }),
        };

        assert_eq!(
            variable_assigment.precompile(&mut precompiler),
            Ok(llvm_ast::VariableAssigment {
                name: "name_1".to_string(),
                value: llvm_ast::VariableValue::Identifier("name_2".to_string()),
            })
        );
        assert!(precompiler.variables.contains(&Identifier {
            name: "name_1".to_string(),
        }));
    }

    #[test]
    fn precompile_variable_assigment_error_test_1() {
        let mut precompiler = Precompiler::new();

        let variable_assigment = VariableAssigment {
            left: Identifier {
                name: "name_1".to_string(),
            },
            right: RightAssigmentValue::Literal(Literal::Number(64_f64)),
        };

        assert_eq!(
            variable_assigment.precompile(&mut precompiler),
            Err(precompiler::Error::UndefinedVariable(Identifier {
                name: "name_1".to_string(),
            }))
        );
    }

    #[test]
    fn precompile_variable_assigment_error_test_2() {
        let mut precompiler = Precompiler::new();
        precompiler
            .variables
            .insert(Identifier {
                name: "name_1".to_string(),
            })
            .unwrap();

        let variable_assigment = VariableAssigment {
            left: Identifier {
                name: "name_1".to_string(),
            },
            right: RightAssigmentValue::Identifier(Identifier {
                name: "name_2".to_string(),
            }),
        };

        assert_eq!(
            variable_assigment.precompile(&mut precompiler),
            Err(precompiler::Error::UndefinedVariable(Identifier {
                name: "name_2".to_string(),
            }))
        );
    }
}
