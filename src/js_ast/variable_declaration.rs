use super::{AssigmentExpression, Literal, RightAssigmentValue};
use crate::{
    lexer::{self, CharReader, Keyword, Token},
    llvm_ast,
    parser::{self, Parser},
    precompiler::{self, Precompile, Precompiler},
};
use std::io::Read;

/// VariableDeclaration - Expression type for variable assigment, like "var a = 4"
#[derive(Debug, PartialEq)]
pub struct VariableDeclaration(pub AssigmentExpression);

impl Parser for VariableDeclaration {
    fn parse<R: Read>(cur_token: Token, reader: &mut CharReader<R>) -> Result<Self, parser::Error> {
        match cur_token {
            Token::Keyword(Keyword::Var) => Ok(Self(AssigmentExpression::parse(
                lexer::get_token(reader)?,
                reader,
            )?)),
            token => Err(parser::Error::UnexpectedToken(token)),
        }
    }
}

impl Precompile for VariableDeclaration {
    type Output = llvm_ast::VariableDeclaration;
    fn precompile(self, precompiler: &mut Precompiler) -> Result<Self::Output, precompiler::Error> {
        let variable_declaration = match self.0.right {
            RightAssigmentValue::Literal(literal) => match literal {
                Literal::Number(value) => Self::Output {
                    name: self.0.left.name,
                    value: llvm_ast::VariableValue::FloatNumber(value),
                },
                Literal::String(value) => Self::Output {
                    name: self.0.left.name,
                    value: llvm_ast::VariableValue::String(value),
                },
            },
            RightAssigmentValue::Identifier(identifier) => {
                let value = precompiler
                    .variables
                    .get(&identifier.name)
                    .ok_or(precompiler::Error::UndefinedVariable(identifier.name))?
                    .clone();
                Self::Output {
                    name: self.0.left.name,
                    value,
                }
            }
        };
        precompiler
            .variables
            .insert(
                variable_declaration.name.clone(),
                variable_declaration.value.clone(),
            )
            .map_err(|_| {
                precompiler::Error::AlreadyDeclaredVariable(variable_declaration.name.clone())
            })?;
        Ok(variable_declaration)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::js_ast::{Identifier, Literal, RightAssigmentValue};

    #[test]
    fn parse_variable_declaration_test() {
        let mut reader = CharReader::new("var name = 12;".as_bytes());
        assert_eq!(
            VariableDeclaration::parse(lexer::get_token(&mut reader).unwrap(), &mut reader)
                .unwrap(),
            VariableDeclaration(AssigmentExpression {
                left: Identifier {
                    name: "name".to_string()
                },
                right: RightAssigmentValue::Literal(Literal::Number(12_f64))
            })
        );

        let mut reader = CharReader::new("var name1 = name2;".as_bytes());
        assert_eq!(
            VariableDeclaration::parse(lexer::get_token(&mut reader).unwrap(), &mut reader)
                .unwrap(),
            VariableDeclaration(AssigmentExpression {
                left: Identifier {
                    name: "name1".to_string()
                },
                right: RightAssigmentValue::Identifier(Identifier {
                    name: "name2".to_string()
                })
            })
        );
    }

    #[test]
    fn precompile_variable_declaration_test() {
        let mut precompiler = Precompiler::new();

        let variable_declaration = VariableDeclaration(AssigmentExpression {
            left: Identifier {
                name: "name_1".to_string(),
            },
            right: RightAssigmentValue::Literal(Literal::Number(64_f64)),
        });

        assert_eq!(
            variable_declaration.precompile(&mut precompiler),
            Ok(llvm_ast::VariableDeclaration {
                name: "name_1".to_string(),
                value: llvm_ast::VariableValue::FloatNumber(64_f64)
            })
        );
        assert_eq!(
            precompiler.variables.get(&"name_1".to_string()),
            Some(&llvm_ast::VariableValue::FloatNumber(64_f64))
        );
    }

    #[test]
    fn precompile_variable_declaration_test_1() {
        let mut precompiler = Precompiler::new();
        precompiler
            .variables
            .insert(
                "name_2".to_string(),
                llvm_ast::VariableValue::FloatNumber(64_f64),
            )
            .unwrap();

        let variable_declaration = VariableDeclaration(AssigmentExpression {
            left: Identifier {
                name: "name_1".to_string(),
            },
            right: RightAssigmentValue::Identifier(Identifier {
                name: "name_2".to_string(),
            }),
        });

        assert_eq!(
            variable_declaration.precompile(&mut precompiler),
            Ok(llvm_ast::VariableDeclaration {
                name: "name_1".to_string(),
                value: llvm_ast::VariableValue::FloatNumber(64_f64)
            })
        );
        assert_eq!(
            precompiler.variables.get(&"name_1".to_string()),
            Some(&llvm_ast::VariableValue::FloatNumber(64_f64))
        );
    }

    #[test]
    fn precompile_variable_declaration_error_test_1() {
        let mut precompiler = Precompiler::new();
        precompiler
            .variables
            .insert(
                "name_1".to_string(),
                llvm_ast::VariableValue::FloatNumber(64_f64),
            )
            .unwrap();

        let variable_declaration = VariableDeclaration(AssigmentExpression {
            left: Identifier {
                name: "name_1".to_string(),
            },
            right: RightAssigmentValue::Literal(Literal::Number(64_f64)),
        });

        assert_eq!(
            variable_declaration.precompile(&mut precompiler),
            Err(precompiler::Error::AlreadyDeclaredVariable(
                "name_1".to_string()
            ))
        );
    }

    #[test]
    fn precompile_variable_declaration_error_test_2() {
        let mut precompiler = Precompiler::new();

        let variable_declaration = VariableDeclaration(AssigmentExpression {
            left: Identifier {
                name: "name_1".to_string(),
            },
            right: RightAssigmentValue::Identifier(Identifier {
                name: "name_2".to_string(),
            }),
        });

        assert_eq!(
            variable_declaration.precompile(&mut precompiler),
            Err(precompiler::Error::UndefinedVariable("name_2".to_string()))
        );
    }
}
