use super::{Identifier, VariableExpression};
use crate::{llvm_ast, LexerError, Precompiler, PrecompilerError};
use lexer::{Separator, Token, TokenReader};
use std::{collections::HashMap, io::Read};

#[derive(Clone, Debug, PartialEq)]
pub struct ObjectExpression {
    pub properties: HashMap<Identifier, VariableExpression>,
}

impl ObjectExpression {
    pub fn parse<R: Read>(
        mut cur_token: Token,
        reader: &mut TokenReader<R>,
    ) -> Result<Self, LexerError> {
        match cur_token {
            Token::Separator(Separator::OpenCurlyBrace) => {
                let mut properties = HashMap::new();
                cur_token = reader.next_token()?;
                loop {
                    match cur_token {
                        Token::Separator(Separator::CloseCurlyBrace) => break,
                        cur_token => {
                            let key = Identifier::parse(cur_token, reader)?;

                            let value = match reader.next_token()? {
                                Token::Separator(Separator::Colon) => {
                                    VariableExpression::parse(reader.next_token()?, reader)
                                }
                                token => Err(LexerError::UnexpectedToken(token)),
                            }?;
                            properties.insert(key, value);
                        }
                    }
                    cur_token = match reader.next_token()? {
                        Token::Separator(Separator::CloseCurlyBrace) => break,
                        Token::Separator(Separator::Comma) => reader.next_token()?,
                        token => return Err(LexerError::UnexpectedToken(token)),
                    };
                }

                Ok(Self { properties })
            }
            token => Err(LexerError::UnexpectedToken(token)),
        }
    }
}

impl ObjectExpression {
    pub fn precompile(
        self,
        precompiler: &mut Precompiler,
    ) -> Result<llvm_ast::ObjectExpression, PrecompilerError> {
        let mut properties = HashMap::new();
        for (key, value) in self.properties {
            properties.insert(
                llvm_ast::Identifier::new(key.name, 0),
                value.precompile(precompiler)?,
            );
        }
        Ok(llvm_ast::ObjectExpression { properties })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::js_ast::VariableValue;

    #[test]
    fn parse_object_expression_test() {
        let mut reader = TokenReader::new(r#"{}"#.as_bytes());
        assert_eq!(
            ObjectExpression::parse(reader.next_token().unwrap(), &mut reader),
            Ok(ObjectExpression {
                properties: HashMap::new(),
            })
        );

        let mut reader = TokenReader::new(r#"{ name: 12 }"#.as_bytes());
        assert_eq!(
            ObjectExpression::parse(reader.next_token().unwrap(), &mut reader),
            Ok(ObjectExpression {
                properties: vec![(
                    "name".to_string().into(),
                    VariableExpression::VariableValue(VariableValue::Number(12_f64))
                ),]
                .into_iter()
                .collect(),
            })
        );

        let mut reader = TokenReader::new(r#"{ name: 12, age: false, }"#.as_bytes());
        assert_eq!(
            ObjectExpression::parse(reader.next_token().unwrap(), &mut reader),
            Ok(ObjectExpression {
                properties: vec![
                    (
                        "name".to_string().into(),
                        VariableExpression::VariableValue(VariableValue::Number(12_f64))
                    ),
                    (
                        "age".to_string().into(),
                        VariableExpression::VariableValue(VariableValue::Boolean(false))
                    )
                ]
                .into_iter()
                .collect(),
            })
        );

        let mut reader = TokenReader::new(
            r#"{ name: {name: "Alex", surname: "Pozhilenkov"}, age: false, }"#.as_bytes(),
        );
        assert_eq!(
            ObjectExpression::parse(reader.next_token().unwrap(), &mut reader),
            Ok(ObjectExpression {
                properties: vec![
                    (
                        "name".to_string().into(),
                        VariableExpression::VariableValue(VariableValue::ObjectExpression(
                            ObjectExpression {
                                properties: vec![
                                    (
                                        "name".to_string().into(),
                                        VariableExpression::VariableValue(VariableValue::String(
                                            "Alex".to_string()
                                        ))
                                    ),
                                    (
                                        "surname".to_string().into(),
                                        VariableExpression::VariableValue(VariableValue::String(
                                            "Pozhilenkov".to_string()
                                        ))
                                    )
                                ]
                                .into_iter()
                                .collect(),
                            }
                        ))
                    ),
                    (
                        "age".to_string().into(),
                        VariableExpression::VariableValue(VariableValue::Boolean(false))
                    )
                ]
                .into_iter()
                .collect(),
            })
        );
    }
}
