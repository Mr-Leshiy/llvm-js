use super::{Identifier, VariableExpression};
use crate::{llvm_ast, Error, Precompiler};
use lexer::{Keyword, Token, TokenReader};
use std::io::Read;

/// VariableDeclaration - Expression type for variable assigment, like "var a = 4" or "let a = 4"
#[derive(Clone, Debug, PartialEq)]
pub struct VariableDeclaration {
    pub name: Identifier,
    pub value: Option<VariableExpression>,
}

impl VariableDeclaration {
    fn parse_impl<R: Read>(cur_token: Token, reader: &mut TokenReader<R>) -> Result<Self, Error> {
        let name = Identifier::parse(cur_token, reader)?;

        reader.start_saving();
        match reader.next_token()? {
            Token::Assign => {
                reader.reset_saving();
                let value = Some(VariableExpression::parse(reader.next_token()?, reader)?);
                Ok(Self { name, value })
            }
            _ => {
                reader.stop_saving();
                Ok(Self { name, value: None })
            }
        }
    }

    pub fn parse<R: Read>(cur_token: Token, reader: &mut TokenReader<R>) -> Result<Self, Error> {
        match cur_token {
            Token::Keyword(Keyword::Var) => Self::parse_impl(reader.next_token()?, reader),
            Token::Keyword(Keyword::Let) => Self::parse_impl(reader.next_token()?, reader),
            token => Err(Error::UnexpectedToken(token)),
        }
    }
}

impl VariableDeclaration {
    pub fn precompile(
        self,
        precompiler: &mut Precompiler,
    ) -> Result<llvm_ast::VariableDeclaration, Error> {
        let value = match self.value {
            Some(expr) => Some(expr.precompile(precompiler)?),
            None => None,
        };
        let index = precompiler.insert_variable(self.name.clone());
        Ok(llvm_ast::VariableDeclaration {
            name: llvm_ast::Identifier::new(self.name.name, index),
            value,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::js_ast::{MemberExpression, VariableExpression, VariableValue};

    #[test]
    fn parse_variable_declaration_test1() {
        let mut reader = TokenReader::new("var name = 12;".as_bytes());
        assert_eq!(
            VariableDeclaration::parse(reader.next_token().unwrap(), &mut reader),
            Ok(VariableDeclaration {
                name: "name".to_string().into(),
                value: Some(VariableExpression::VariableValue(VariableValue::Number(
                    12_f64
                )))
            })
        );

        let mut reader = TokenReader::new("var name1 = name2;".as_bytes());
        assert_eq!(
            VariableDeclaration::parse(reader.next_token().unwrap(), &mut reader),
            Ok(VariableDeclaration {
                name: "name1".to_string().into(),
                value: Some(VariableExpression::VariableValue(
                    VariableValue::MemberExpression(MemberExpression {
                        variable_name: "name2".to_string().into(),
                        property: None
                    })
                ))
            })
        );

        let mut reader = TokenReader::new("var name1;".as_bytes());
        assert_eq!(
            VariableDeclaration::parse(reader.next_token().unwrap(), &mut reader),
            Ok(VariableDeclaration {
                name: "name1".to_string().into(),
                value: None
            })
        );
    }

    #[test]
    fn parse_variable_declaration_test2() {
        let mut reader = TokenReader::new("let name = 12;".as_bytes());
        assert_eq!(
            VariableDeclaration::parse(reader.next_token().unwrap(), &mut reader),
            Ok(VariableDeclaration {
                name: "name".to_string().into(),
                value: Some(VariableExpression::VariableValue(VariableValue::Number(
                    12_f64
                )))
            })
        );

        let mut reader = TokenReader::new("let name1 = name2;".as_bytes());
        assert_eq!(
            VariableDeclaration::parse(reader.next_token().unwrap(), &mut reader),
            Ok(VariableDeclaration {
                name: "name1".to_string().into(),
                value: Some(VariableExpression::VariableValue(
                    VariableValue::MemberExpression(MemberExpression {
                        variable_name: "name2".to_string().into(),
                        property: None
                    })
                ))
            })
        );

        let mut reader = TokenReader::new("let name1;".as_bytes());
        assert_eq!(
            VariableDeclaration::parse(reader.next_token().unwrap(), &mut reader),
            Ok(VariableDeclaration {
                name: "name1".to_string().into(),
                value: None
            })
        );
    }

    #[test]
    fn precompile_variable_declaration_test_1() {
        let mut precompiler = Precompiler::new(std::iter::empty());

        let variable_declaration = VariableDeclaration {
            name: "name_1".to_string().into(),
            value: Some(VariableExpression::VariableValue(VariableValue::Number(
                64_f64,
            ))),
        };

        assert_eq!(
            variable_declaration.precompile(&mut precompiler),
            Ok(llvm_ast::VariableDeclaration {
                name: llvm_ast::Identifier::new("name_1".to_string(), 0),
                value: Some(llvm_ast::VariableExpression::VariableValue(
                    llvm_ast::VariableValue::FloatNumber(64_f64)
                )),
            })
        );
        assert_eq!(
            precompiler.get_variable("name_1".to_string().into()),
            Ok(("name_1".to_string().into(), 0))
        );
    }

    #[test]
    fn precompile_variable_declaration_test_2() {
        let mut precompiler = Precompiler::new(std::iter::empty());
        precompiler.insert_variable("name_2".to_string().into());

        let variable_declaration = VariableDeclaration {
            name: "name_1".to_string().into(),
            value: Some(VariableExpression::VariableValue(
                VariableValue::MemberExpression(MemberExpression {
                    variable_name: "name_2".to_string().into(),
                    property: None,
                }),
            )),
        };

        assert_eq!(
            variable_declaration.precompile(&mut precompiler),
            Ok(llvm_ast::VariableDeclaration {
                name: llvm_ast::Identifier::new("name_1".to_string(), 0),
                value: Some(llvm_ast::VariableExpression::VariableValue(
                    llvm_ast::VariableValue::MemberExpression(llvm_ast::MemberExpression {
                        variable_name: llvm_ast::Identifier::new("name_2".to_string(), 0),
                        property: None,
                    })
                )),
            })
        );
        assert_eq!(
            precompiler.get_variable("name_1".to_string().into()),
            Ok(("name_1".to_string().into(), 0))
        );
    }

    #[test]
    fn precompile_variable_declaration_test_3() {
        let mut precompiler = Precompiler::new(std::iter::empty());
        precompiler.insert_variable("name_1".to_string().into());

        let variable_declaration = VariableDeclaration {
            name: "name_1".to_string().into(),
            value: Some(VariableExpression::VariableValue(VariableValue::Number(
                64_f64,
            ))),
        };

        assert_eq!(
            variable_declaration.precompile(&mut precompiler),
            Ok(llvm_ast::VariableDeclaration {
                name: llvm_ast::Identifier::new("name_1".to_string(), 1),
                value: Some(llvm_ast::VariableExpression::VariableValue(
                    llvm_ast::VariableValue::FloatNumber(64_f64)
                )),
            })
        );
    }

    #[test]
    fn precompile_variable_declaration_error_test_2() {
        let mut precompiler = Precompiler::new(std::iter::empty());

        let variable_declaration = VariableDeclaration {
            name: "name_1".to_string().into(),
            value: Some(VariableExpression::VariableValue(
                VariableValue::MemberExpression(MemberExpression {
                    variable_name: "name_2".to_string().into(),
                    property: None,
                }),
            )),
        };

        assert_eq!(
            variable_declaration.precompile(&mut precompiler),
            Err(precompiler::Error::UndefinedVariable("name_2".to_string().into(),).into())
        );
    }
}
