use super::{FunctionCall, Identifier, VariableExpression};
use crate::{llvm_ast, LexerError, Precompiler, PrecompilerError};
use lexer::{Separator, Token, TokenReader};
use std::io::Read;

#[derive(Clone, Debug, PartialEq)]
pub enum PropertyType {
    Identifier(Identifier),
    FunctionCall(FunctionCall),
    VariableExpression(VariableExpression),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Property {
    pub object: PropertyType,
    pub property: Option<Box<Property>>,
}

impl Property {
    pub fn parse<R: Read>(
        cur_token: &Token,
        reader: &mut TokenReader<R>,
    ) -> Result<Option<Self>, LexerError> {
        match cur_token {
            Token::Separator(Separator::Dot) => {
                reader.start_saving();
                let object = if let Ok(res) = FunctionCall::parse(reader.next_token()?, reader) {
                    reader.reset_saving();
                    PropertyType::FunctionCall(res)
                } else {
                    reader.stop_saving();
                    PropertyType::Identifier(Identifier::parse(reader.next_token()?, reader)?)
                };
                reader.start_saving();
                if let Some(property) = Self::parse(&reader.next_token()?, reader)? {
                    reader.reset_saving();
                    Ok(Some(Self {
                        object,
                        property: Some(property.into()),
                    }))
                } else {
                    reader.stop_saving();
                    Ok(Some(Self {
                        object,
                        property: None,
                    }))
                }
            }
            Token::Separator(Separator::OpenSquareBracket) => {
                let object = PropertyType::VariableExpression(VariableExpression::parse(
                    reader.next_token()?,
                    reader,
                )?);
                match reader.next_token()? {
                    Token::Separator(Separator::CloseSquareBracket) => {
                        reader.start_saving();
                        if let Some(property) = Self::parse(&reader.next_token()?, reader)? {
                            reader.reset_saving();
                            Ok(Some(Self {
                                object,
                                property: Some(property.into()),
                            }))
                        } else {
                            reader.stop_saving();
                            Ok(Some(Self {
                                object,
                                property: None,
                            }))
                        }
                    }
                    token => Err(LexerError::UnexpectedToken(token)),
                }
            }
            _ => Ok(None),
        }
    }

    pub fn precompile(
        self,
        precompiler: &mut Precompiler,
    ) -> Result<llvm_ast::Property, PrecompilerError> {
        let object = match self.object {
            PropertyType::Identifier(identifier) => {
                llvm_ast::PropertyType::Identifier(llvm_ast::Identifier::new(identifier.name, 0))
            }
            PropertyType::FunctionCall(function_call) => {
                // check if arguments exist
                let mut args = Vec::new();
                for arg in function_call.args {
                    args.push(arg.precompile(precompiler)?);
                }

                llvm_ast::PropertyType::FunctionCall(llvm_ast::FunctionCall {
                    name: llvm_ast::Identifier::new(function_call.name.name, 0),
                    args,
                })
            }
            PropertyType::VariableExpression(variable_expression) => {
                llvm_ast::PropertyType::VariableExpression(
                    variable_expression.precompile(precompiler)?,
                )
            }
        };
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
    pub object: VariableExpression,
    pub property: Property,
}

impl MemberExpression {
    pub fn precompile(
        self,
        precompiler: &mut Precompiler,
    ) -> Result<llvm_ast::MemberExpression, PrecompilerError> {
        let object = self.object.precompile(precompiler)?.into();
        let property = self.property.precompile(precompiler)?;
        Ok(llvm_ast::MemberExpression { object, property })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::js_ast::{ObjectExpression, VariableValue};

    #[test]
    fn parse_member_expression_test() {
        let mut reader = TokenReader::new("name.name".as_bytes());
        assert_eq!(
            VariableExpression::parse(reader.next_token().unwrap(), &mut reader).unwrap(),
            VariableExpression::MemberExpression(
                MemberExpression {
                    object: VariableExpression::VariableValue(VariableValue::Identifier(
                        "name".to_string().into()
                    )),
                    property: Property {
                        object: PropertyType::Identifier("name".to_string().into()),
                        property: None
                    }
                }
                .into()
            ),
        );

        let mut reader = TokenReader::new("name.name.name".as_bytes());
        assert_eq!(
            VariableExpression::parse(reader.next_token().unwrap(), &mut reader).unwrap(),
            VariableExpression::MemberExpression(
                MemberExpression {
                    object: VariableExpression::VariableValue(VariableValue::Identifier(
                        "name".to_string().into()
                    )),
                    property: Property {
                        object: PropertyType::Identifier("name".to_string().into()),
                        property: Some(
                            Property {
                                object: PropertyType::Identifier("name".to_string().into()),
                                property: None
                            }
                            .into()
                        )
                    }
                }
                .into()
            ),
        );

        let mut reader = TokenReader::new("name.name.name.name".as_bytes());
        assert_eq!(
            VariableExpression::parse(reader.next_token().unwrap(), &mut reader).unwrap(),
            VariableExpression::MemberExpression(
                MemberExpression {
                    object: VariableExpression::VariableValue(VariableValue::Identifier(
                        "name".to_string().into()
                    )),
                    property: Property {
                        object: PropertyType::Identifier("name".to_string().into()),
                        property: Some(
                            Property {
                                object: PropertyType::Identifier("name".to_string().into()),
                                property: Some(
                                    Property {
                                        object: PropertyType::Identifier("name".to_string().into()),
                                        property: None
                                    }
                                    .into()
                                )
                            }
                            .into()
                        )
                    }
                }
                .into()
            ),
        );
    }

    #[test]
    fn parse_member_expression_test2() {
        let mut reader = TokenReader::new("name.name()".as_bytes());
        assert_eq!(
            VariableExpression::parse(reader.next_token().unwrap(), &mut reader).unwrap(),
            VariableExpression::MemberExpression(
                MemberExpression {
                    object: VariableExpression::VariableValue(VariableValue::Identifier(
                        "name".to_string().into()
                    )),
                    property: Property {
                        object: PropertyType::FunctionCall(FunctionCall {
                            name: "name".to_string().into(),
                            args: vec![]
                        }),
                        property: None
                    }
                }
                .into()
            ),
        );

        let mut reader = TokenReader::new("name.name().name()".as_bytes());
        assert_eq!(
            VariableExpression::parse(reader.next_token().unwrap(), &mut reader).unwrap(),
            VariableExpression::MemberExpression(
                MemberExpression {
                    object: VariableExpression::VariableValue(VariableValue::Identifier(
                        "name".to_string().into()
                    )),
                    property: Property {
                        object: PropertyType::FunctionCall(FunctionCall {
                            name: "name".to_string().into(),
                            args: vec![]
                        }),
                        property: Some(
                            Property {
                                object: PropertyType::FunctionCall(FunctionCall {
                                    name: "name".to_string().into(),
                                    args: vec![]
                                }),
                                property: None
                            }
                            .into()
                        )
                    }
                }
                .into()
            ),
        );

        let mut reader = TokenReader::new("name.name().name().name()".as_bytes());
        assert_eq!(
            VariableExpression::parse(reader.next_token().unwrap(), &mut reader).unwrap(),
            VariableExpression::MemberExpression(
                MemberExpression {
                    object: VariableExpression::VariableValue(VariableValue::Identifier(
                        "name".to_string().into()
                    )),
                    property: Property {
                        object: PropertyType::FunctionCall(FunctionCall {
                            name: "name".to_string().into(),
                            args: vec![]
                        }),
                        property: Some(
                            Property {
                                object: PropertyType::FunctionCall(FunctionCall {
                                    name: "name".to_string().into(),
                                    args: vec![]
                                }),
                                property: Some(
                                    Property {
                                        object: PropertyType::FunctionCall(FunctionCall {
                                            name: "name".to_string().into(),
                                            args: vec![]
                                        }),
                                        property: None
                                    }
                                    .into()
                                )
                            }
                            .into()
                        )
                    }
                }
                .into()
            ),
        );
    }

    #[test]
    fn parse_member_expression_test3() {
        let mut reader = TokenReader::new(r#"{name: "Alex"}.name()"#.as_bytes());
        assert_eq!(
            VariableExpression::parse(reader.next_token().unwrap(), &mut reader).unwrap(),
            VariableExpression::MemberExpression(
                MemberExpression {
                    object: VariableExpression::VariableValue(VariableValue::ObjectExpression(
                        ObjectExpression {
                            properties: vec![(
                                "name".to_string().into(),
                                VariableExpression::VariableValue(VariableValue::String(
                                    "Alex".to_string()
                                ))
                            )]
                            .into_iter()
                            .collect()
                        }
                    )),
                    property: Property {
                        object: PropertyType::FunctionCall(FunctionCall {
                            name: "name".to_string().into(),
                            args: vec![]
                        }),
                        property: None
                    }
                }
                .into()
            ),
        );

        let mut reader = TokenReader::new("name().name()".as_bytes());
        assert_eq!(
            VariableExpression::parse(reader.next_token().unwrap(), &mut reader).unwrap(),
            VariableExpression::MemberExpression(
                MemberExpression {
                    object: VariableExpression::FunctionCall(FunctionCall {
                        name: "name".to_string().into(),
                        args: vec![]
                    }),
                    property: Property {
                        object: PropertyType::FunctionCall(FunctionCall {
                            name: "name".to_string().into(),
                            args: vec![]
                        }),
                        property: None
                    }
                }
                .into()
            ),
        );
    }

    #[test]
    #[allow(clippy::too_many_lines)]
    fn parse_member_expression_test4() {
        let mut reader = TokenReader::new("name[name]".as_bytes());
        assert_eq!(
            VariableExpression::parse(reader.next_token().unwrap(), &mut reader).unwrap(),
            VariableExpression::MemberExpression(
                MemberExpression {
                    object: VariableExpression::VariableValue(VariableValue::Identifier(
                        "name".to_string().into()
                    )),
                    property: Property {
                        object: PropertyType::VariableExpression(
                            VariableExpression::VariableValue(VariableValue::Identifier(
                                "name".to_string().into()
                            ))
                        ),
                        property: None
                    }
                }
                .into()
            ),
        );

        let mut reader = TokenReader::new(r#"name[name]["name"]"#.as_bytes());
        assert_eq!(
            VariableExpression::parse(reader.next_token().unwrap(), &mut reader).unwrap(),
            VariableExpression::MemberExpression(
                MemberExpression {
                    object: VariableExpression::VariableValue(VariableValue::Identifier(
                        "name".to_string().into()
                    )),
                    property: Property {
                        object: PropertyType::VariableExpression(
                            VariableExpression::VariableValue(VariableValue::Identifier(
                                "name".to_string().into()
                            ))
                        ),
                        property: Some(
                            Property {
                                object: PropertyType::VariableExpression(
                                    VariableExpression::VariableValue(VariableValue::String(
                                        "name".to_string()
                                    ))
                                ),
                                property: None
                            }
                            .into()
                        )
                    }
                }
                .into()
            ),
        );

        let mut reader = TokenReader::new(r#"name[name]["name"][name.name]"#.as_bytes());
        assert_eq!(
            VariableExpression::parse(reader.next_token().unwrap(), &mut reader).unwrap(),
            VariableExpression::MemberExpression(
                MemberExpression {
                    object: VariableExpression::VariableValue(VariableValue::Identifier(
                        "name".to_string().into()
                    )),
                    property: Property {
                        object: PropertyType::VariableExpression(
                            VariableExpression::VariableValue(VariableValue::Identifier(
                                "name".to_string().into()
                            ))
                        ),
                        property: Some(
                            Property {
                                object: PropertyType::VariableExpression(
                                    VariableExpression::VariableValue(VariableValue::String(
                                        "name".to_string()
                                    ))
                                ),
                                property: Some(
                                    Property {
                                        object: PropertyType::VariableExpression(
                                            VariableExpression::MemberExpression(
                                                MemberExpression {
                                                    object: VariableExpression::VariableValue(
                                                        VariableValue::Identifier(
                                                            "name".to_string().into()
                                                        )
                                                    ),
                                                    property: Property {
                                                        object: PropertyType::Identifier(
                                                            "name".to_string().into()
                                                        ),
                                                        property: None
                                                    }
                                                }
                                                .into()
                                            )
                                        ),
                                        property: None
                                    }
                                    .into()
                                )
                            }
                            .into()
                        )
                    }
                }
                .into()
            ),
        );

        let mut reader =
            TokenReader::new(r#"name[name]["name"][name.name][name["name"].name]"#.as_bytes());
        assert_eq!(
            VariableExpression::parse(reader.next_token().unwrap(), &mut reader).unwrap(),
            VariableExpression::MemberExpression(MemberExpression {
                object: VariableExpression::VariableValue(VariableValue::Identifier("name".to_string().into())),
                property:
                    Property {
                        object: PropertyType::VariableExpression(VariableExpression::VariableValue(VariableValue::Identifier("name".to_string().into()))),
                        property: Some(
                            Property {
                                object: PropertyType::VariableExpression(VariableExpression::VariableValue(VariableValue::String("name".to_string()))),
                                property: Some(
                                    Property {
                                        object: PropertyType::VariableExpression(VariableExpression::MemberExpression(
                                            MemberExpression {
                                                object: VariableExpression::VariableValue(VariableValue::Identifier("name".to_string().into())),
                                                property: Property {
                                                        object: PropertyType::Identifier("name".to_string().into()),
                                                        property: None
                                                    }
                                            }.into()
                                        )),
                                        property: Some(
                                            Property {
                                                object: PropertyType::VariableExpression(VariableExpression::MemberExpression(
                                                    MemberExpression {
                                                        object: VariableExpression::VariableValue(VariableValue::Identifier("name".to_string().into())),
                                                        property: Property {
                                                                object: PropertyType::VariableExpression(VariableExpression::VariableValue(VariableValue::String("name".to_string()))),
                                                                property: Some(
                                                                    Property {
                                                                        object: PropertyType::Identifier("name".to_string().into()),
                                                                        property: None
                                                                    }
                                                                    .into()
                                                                )
                                                            }
                                                    }.into(),
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
            }.into()),
        );
    }
}
