use super::{Identifier, VariableExpression, VariableValue};
use crate::{llvm_ast, Error, Precompiler, PrecompilerError};
use lexer::{Separator, Token, TokenReader};
use std::io::Read;

#[derive(Clone, Debug, PartialEq)]
pub struct Property {
    pub object: VariableExpression,
    pub property: Option<Box<Property>>,
}

impl Property {
    pub fn parse<R: Read>(
        cur_token: Token,
        reader: &mut TokenReader<R>,
    ) -> Result<Option<Box<Self>>, Error> {
        match cur_token {
            Token::Separator(Separator::Dot) => {
                // Actually it is as a hack how we are representing Indetifier
                let object = VariableExpression::VariableValue(VariableValue::String(
                    Identifier::parse(reader.next_token()?, reader)?.name,
                ));
                reader.start_saving();
                let property = Self::parse(reader.next_token()?, reader)?;
                Ok(Some(Self { object, property }.into()))
            }
            Token::Separator(Separator::OpenSquareBracket) => {
                let object = VariableExpression::parse(reader.next_token()?, reader)?;
                match reader.next_token()? {
                    Token::Separator(Separator::CloseSquareBracket) => {
                        reader.start_saving();
                        let property = Self::parse(reader.next_token()?, reader)?;
                        Ok(Some(Self { object, property }.into()))
                    }
                    token => Err(Error::UnexpectedToken(token)),
                }
            }
            _ => {
                reader.stop_saving();
                Ok(None)
            }
        }
    }

    pub fn precompile(
        self,
        precompiler: &mut Precompiler,
    ) -> Result<llvm_ast::Property, PrecompilerError> {
        let object = self.object.precompile(precompiler)?;
        let property = if let Some(property) = self.property {
            Some(property.precompile(precompiler)?.into())
        } else {
            None
        };
        Ok(llvm_ast::Property { object, property })
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct MemberExpression {
    pub variable_name: Identifier,
    pub property: Option<Box<Property>>,
}

impl MemberExpression {
    pub fn parse<R: Read>(cur_token: Token, reader: &mut TokenReader<R>) -> Result<Self, Error> {
        let variable_name = Identifier::parse(cur_token, reader)?;
        reader.start_saving();
        let property = Property::parse(reader.next_token()?, reader)?;
        Ok(Self {
            variable_name,
            property,
        })
    }

    pub fn precompile(
        self,
        precompiler: &mut Precompiler,
    ) -> Result<llvm_ast::MemberExpression, PrecompilerError> {
        let index = precompiler.get_variable(self.variable_name.clone())?;
        let variable_name = llvm_ast::Identifier::new(self.variable_name.name, index);
        let property = if let Some(property) = self.property {
            Some(property.precompile(precompiler)?.into())
        } else {
            None
        };

        Ok(llvm_ast::MemberExpression {
            variable_name,
            property,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_member_expression_test() {
        let mut reader = TokenReader::new("name".as_bytes());
        assert_eq!(
            MemberExpression::parse(reader.next_token().unwrap(), &mut reader),
            Ok(MemberExpression {
                variable_name: "name".to_string().into(),
                property: None
            }),
        );

        let mut reader = TokenReader::new("name.name".as_bytes());
        assert_eq!(
            MemberExpression::parse(reader.next_token().unwrap(), &mut reader),
            Ok(MemberExpression {
                variable_name: "name".to_string().into(),
                property: Some(
                    Property {
                        object: VariableExpression::VariableValue(VariableValue::String(
                            "name".to_string()
                        )),
                        property: None
                    }
                    .into()
                )
            }),
        );

        let mut reader = TokenReader::new("name.name.name".as_bytes());
        assert_eq!(
            MemberExpression::parse(reader.next_token().unwrap(), &mut reader),
            Ok(MemberExpression {
                variable_name: "name".to_string().into(),
                property: Some(
                    Property {
                        object: VariableExpression::VariableValue(VariableValue::String(
                            "name".to_string()
                        )),
                        property: Some(
                            Property {
                                object: VariableExpression::VariableValue(VariableValue::String(
                                    "name".to_string()
                                )),
                                property: None
                            }
                            .into()
                        )
                    }
                    .into()
                )
            }),
        );

        let mut reader = TokenReader::new("name.name.name.name".as_bytes());
        assert_eq!(
            MemberExpression::parse(reader.next_token().unwrap(), &mut reader),
            Ok(MemberExpression {
                variable_name: "name".to_string().into(),
                property: Some(
                    Property {
                        object: VariableExpression::VariableValue(VariableValue::String(
                            "name".to_string()
                        )),
                        property: Some(
                            Property {
                                object: VariableExpression::VariableValue(VariableValue::String(
                                    "name".to_string()
                                )),
                                property: Some(
                                    Property {
                                        object: VariableExpression::VariableValue(
                                            VariableValue::String("name".to_string())
                                        ),
                                        property: None
                                    }
                                    .into()
                                )
                            }
                            .into()
                        )
                    }
                    .into()
                )
            }),
        );
    }

    #[test]
    fn parse_member_expression_test2() {
        let mut reader = TokenReader::new("name[name]".as_bytes());
        assert_eq!(
            MemberExpression::parse(reader.next_token().unwrap(), &mut reader),
            Ok(MemberExpression {
                variable_name: "name".to_string().into(),
                property: Some(
                    Property {
                        object: VariableExpression::VariableValue(VariableValue::MemberExpression(
                            MemberExpression {
                                variable_name: "name".to_string().into(),
                                property: None
                            },
                        )),
                        property: None
                    }
                    .into()
                )
            }),
        );

        let mut reader = TokenReader::new(r#"name[name]["name"]"#.as_bytes());
        assert_eq!(
            MemberExpression::parse(reader.next_token().unwrap(), &mut reader),
            Ok(MemberExpression {
                variable_name: "name".to_string().into(),
                property: Some(
                    Property {
                        object: VariableExpression::VariableValue(VariableValue::MemberExpression(
                            MemberExpression {
                                variable_name: "name".to_string().into(),
                                property: None
                            },
                        )),
                        property: Some(
                            Property {
                                object: VariableExpression::VariableValue(VariableValue::String(
                                    "name".to_string()
                                )),
                                property: None
                            }
                            .into()
                        )
                    }
                    .into()
                )
            }),
        );

        let mut reader = TokenReader::new(r#"name[name]["name"][name.name]"#.as_bytes());
        assert_eq!(
            MemberExpression::parse(reader.next_token().unwrap(), &mut reader),
            Ok(MemberExpression {
                variable_name: "name".to_string().into(),
                property: Some(
                    Property {
                        object: VariableExpression::VariableValue(VariableValue::MemberExpression(
                            MemberExpression {
                                variable_name: "name".to_string().into(),
                                property: None
                            },
                        )),
                        property: Some(
                            Property {
                                object: VariableExpression::VariableValue(VariableValue::String(
                                    "name".to_string()
                                )),
                                property: Some(
                                    Property {
                                        object: VariableExpression::VariableValue(
                                            VariableValue::MemberExpression(MemberExpression {
                                                variable_name: "name".to_string().into(),
                                                property: Some(
                                                    Property {
                                                        object: VariableExpression::VariableValue(
                                                            VariableValue::String(
                                                                "name".to_string()
                                                            )
                                                        ),
                                                        property: None
                                                    }
                                                    .into()
                                                )
                                            },)
                                        ),
                                        property: None
                                    }
                                    .into()
                                )
                            }
                            .into()
                        )
                    }
                    .into()
                )
            }),
        );

        let mut reader =
            TokenReader::new(r#"name[name]["name"][name.name][name["name"].name]"#.as_bytes());
        assert_eq!(
            MemberExpression::parse(reader.next_token().unwrap(), &mut reader),
            Ok(MemberExpression {
                variable_name: "name".to_string().into(),
                property: Some(
                    Property {
                        object: VariableExpression::VariableValue(VariableValue::MemberExpression(
                            MemberExpression {
                                variable_name: "name".to_string().into(),
                                property: None
                            },
                        )),
                        property: Some(
                            Property {
                                object: VariableExpression::VariableValue(VariableValue::String(
                                    "name".to_string()
                                )),
                                property: Some(
                                    Property {
                                        object: VariableExpression::VariableValue(VariableValue::MemberExpression(
                                            MemberExpression {
                                                variable_name: "name".to_string().into(),
                                                property: Some(
                                                    Property {
                                                        object: VariableExpression::VariableValue(
                                                            VariableValue::String("name".to_string())
                                                        ),
                                                        property: None
                                                    }
                                                    .into()
                                                )
                                            },
                                        )),
                                        property: Some(
                                            Property {
                                                object: VariableExpression::VariableValue(VariableValue::MemberExpression(
                                                    MemberExpression {
                                                        variable_name: "name".to_string().into(),
                                                        property: Some(
                                                            Property {
                                                                object: VariableExpression::VariableValue(
                                                                    VariableValue::String("name".to_string())
                                                                ),
                                                                property: Some(
                                                                    Property {
                                                                        object: VariableExpression::VariableValue(
                                                                            VariableValue::String("name".to_string())
                                                                        ),
                                                                        property: None
                                                                    }
                                                                    .into()
                                                                )
                                                            }
                                                            .into()
                                                        )
                                                    },
                                                )),
                                                property: None
                                            }
                                            .into()
                                        )
                                    }
                                    .into()
                                )
                            }
                            .into()
                        )
                    }
                    .into()
                )
            }),
        );
    }
}
