use super::{BinaryExpression, Identifier, UnaryExpression, VariableValue};
use crate::llvm_ast;
use lexer::{Logical, Parser, Separator, Token, TokenReader};
use precompiler::{self, Precompiler};
use std::io::Read;

/// VariableExpression
#[derive(Clone, Debug, PartialEq)]
pub enum VariableExpression {
    VariableValue(VariableValue),
    UnaryExpression(Box<UnaryExpression>),
    BinaryExpression(Box<BinaryExpression>),
    Grouping(Box<VariableExpression>),
}

impl VariableExpression {
    fn parse_impl<R: Read>(
        cur_token: Token,
        reader: &mut TokenReader<R>,
        is_unary: bool,
    ) -> Result<Self, lexer::Error> {
        let left = match cur_token {
            Token::Logical(Logical::Not) => Self::UnaryExpression(Box::new(UnaryExpression::Not(
                Self::parse_impl(reader.next_token()?, reader, true)?,
            ))),
            Token::Separator(Separator::OpenBrace) => {
                let value = Self::parse_impl(reader.next_token()?, reader, false)?;
                match reader.next_token()? {
                    Token::Separator(Separator::CloseBrace) => Self::Grouping(Box::new(value)),
                    token => return Err(lexer::Error::UnexpectedToken(token)),
                }
            }
            token => Self::VariableValue(VariableValue::parse(token, reader)?),
        };
        if is_unary {
            Ok(left)
        } else {
            reader.start_saving();
            match reader.next_token()? {
                Token::Logical(Logical::Or) => {
                    reader.reset_saving();
                    Ok(Self::BinaryExpression(Box::new(BinaryExpression::Or {
                        left,
                        right: VariableExpression::parse_impl(reader.next_token()?, reader, false)?,
                    })))
                }
                Token::Logical(Logical::And) => {
                    reader.reset_saving();
                    Ok(Self::BinaryExpression(Box::new(BinaryExpression::And {
                        left,
                        right: VariableExpression::parse_impl(reader.next_token()?, reader, false)?,
                    })))
                }
                _ => {
                    reader.stop_saving();
                    Ok(left)
                }
            }
        }
    }
}

impl Parser for VariableExpression {
    fn parse<R: Read>(cur_token: Token, reader: &mut TokenReader<R>) -> Result<Self, lexer::Error> {
        Self::parse_impl(cur_token, reader, false)
    }
}

impl VariableExpression {
    pub fn precompile(
        self,
        precompiler: &mut Precompiler<Identifier, llvm_ast::FunctionDeclaration>,
    ) -> Result<llvm_ast::VariableValue, precompiler::Error<Identifier>> {
        match self {
            Self::VariableValue(value) => Ok(value.precompile(precompiler)?),
            Self::UnaryExpression(_) => todo!("implement"),
            Self::BinaryExpression(_) => todo!("implement"),
            Self::Grouping(_) => todo!("implement"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_not_logical_expression_test() {
        let mut reader = TokenReader::new("!true".as_bytes());
        assert_eq!(
            VariableExpression::parse(reader.next_token().unwrap(), &mut reader),
            Ok(VariableExpression::UnaryExpression(Box::new(
                UnaryExpression::Not(VariableExpression::VariableValue(VariableValue::Boolean(
                    true
                )))
            ))),
        );

        let mut reader = TokenReader::new("!false".as_bytes());
        assert_eq!(
            VariableExpression::parse(reader.next_token().unwrap(), &mut reader),
            Ok(VariableExpression::UnaryExpression(Box::new(
                UnaryExpression::Not(VariableExpression::VariableValue(VariableValue::Boolean(
                    false
                )))
            ))),
        );

        let mut reader = TokenReader::new("!a".as_bytes());
        assert_eq!(
            VariableExpression::parse(reader.next_token().unwrap(), &mut reader),
            Ok(VariableExpression::UnaryExpression(Box::new(
                UnaryExpression::Not(VariableExpression::VariableValue(
                    VariableValue::Identifier("a".to_string().into())
                ))
            ))),
        );
    }

    #[test]
    fn parse_and_logical_expression_test() {
        let mut reader = TokenReader::new("true && false".as_bytes());
        assert_eq!(
            VariableExpression::parse(reader.next_token().unwrap(), &mut reader),
            Ok(VariableExpression::BinaryExpression(Box::new(
                BinaryExpression::And {
                    left: VariableExpression::VariableValue(VariableValue::Boolean(true)),
                    right: VariableExpression::VariableValue(VariableValue::Boolean(false))
                }
            ))),
        );

        let mut reader = TokenReader::new("false && a".as_bytes());
        assert_eq!(
            VariableExpression::parse(reader.next_token().unwrap(), &mut reader),
            Ok(VariableExpression::BinaryExpression(Box::new(
                BinaryExpression::And {
                    left: VariableExpression::VariableValue(VariableValue::Boolean(false)),
                    right: VariableExpression::VariableValue(VariableValue::Identifier(
                        "a".to_string().into()
                    ))
                }
            ))),
        );

        let mut reader = TokenReader::new("a && b".as_bytes());
        assert_eq!(
            VariableExpression::parse(reader.next_token().unwrap(), &mut reader),
            Ok(VariableExpression::BinaryExpression(Box::new(
                BinaryExpression::And {
                    left: VariableExpression::VariableValue(VariableValue::Identifier(
                        "a".to_string().into()
                    )),
                    right: VariableExpression::VariableValue(VariableValue::Identifier(
                        "b".to_string().into()
                    ))
                }
            ))),
        );
    }

    #[test]
    fn parse_or_logical_expression_test() {
        let mut reader = TokenReader::new("true || false".as_bytes());
        assert_eq!(
            VariableExpression::parse(reader.next_token().unwrap(), &mut reader),
            Ok(VariableExpression::BinaryExpression(Box::new(
                BinaryExpression::Or {
                    left: VariableExpression::VariableValue(VariableValue::Boolean(true)),
                    right: VariableExpression::VariableValue(VariableValue::Boolean(false))
                }
            ))),
        );

        let mut reader = TokenReader::new("false || a".as_bytes());
        assert_eq!(
            VariableExpression::parse(reader.next_token().unwrap(), &mut reader),
            Ok(VariableExpression::BinaryExpression(Box::new(
                BinaryExpression::Or {
                    left: VariableExpression::VariableValue(VariableValue::Boolean(false)),
                    right: VariableExpression::VariableValue(VariableValue::Identifier(
                        "a".to_string().into()
                    ))
                }
            ))),
        );

        let mut reader = TokenReader::new("a || b".as_bytes());
        assert_eq!(
            VariableExpression::parse(reader.next_token().unwrap(), &mut reader),
            Ok(VariableExpression::BinaryExpression(Box::new(
                BinaryExpression::Or {
                    left: VariableExpression::VariableValue(VariableValue::Identifier(
                        "a".to_string().into()
                    )),
                    right: VariableExpression::VariableValue(VariableValue::Identifier(
                        "b".to_string().into()
                    ))
                }
            ))),
        );
    }

    #[test]
    fn parse_logical_expression_test_1() {
        let mut reader = TokenReader::new("!a || b && !c".as_bytes());
        assert_eq!(
            VariableExpression::parse(reader.next_token().unwrap(), &mut reader),
            Ok(VariableExpression::BinaryExpression(Box::new(
                BinaryExpression::Or {
                    left: VariableExpression::UnaryExpression(Box::new(UnaryExpression::Not(
                        VariableExpression::VariableValue(VariableValue::Identifier(
                            "a".to_string().into()
                        ))
                    ))),
                    right: VariableExpression::BinaryExpression(Box::new(BinaryExpression::And {
                        left: VariableExpression::VariableValue(VariableValue::Identifier(
                            "b".to_string().into()
                        )),
                        right: VariableExpression::UnaryExpression(Box::new(UnaryExpression::Not(
                            VariableExpression::VariableValue(VariableValue::Identifier(
                                "c".to_string().into()
                            ))
                        )))
                    }))
                }
            ))),
        );

        let mut reader = TokenReader::new("!!!!a".as_bytes());
        assert_eq!(
            VariableExpression::parse(reader.next_token().unwrap(), &mut reader),
            Ok(VariableExpression::UnaryExpression(Box::new(
                UnaryExpression::Not(VariableExpression::UnaryExpression(Box::new(
                    UnaryExpression::Not(VariableExpression::UnaryExpression(Box::new(
                        UnaryExpression::Not(VariableExpression::UnaryExpression(Box::new(
                            UnaryExpression::Not(VariableExpression::VariableValue(
                                VariableValue::Identifier("a".to_string().into())
                            ))
                        )))
                    )))
                )))
            )))
        );
    }

    #[test]
    fn parse_grouping_test() {
        let mut reader = TokenReader::new("(!a || (b && !c))".as_bytes());
        assert_eq!(
            VariableExpression::parse(reader.next_token().unwrap(), &mut reader),
            Ok(VariableExpression::Grouping(Box::new(
                VariableExpression::BinaryExpression(Box::new(BinaryExpression::Or {
                    left: VariableExpression::UnaryExpression(Box::new(UnaryExpression::Not(
                        VariableExpression::VariableValue(VariableValue::Identifier(
                            "a".to_string().into()
                        ))
                    ))),
                    right: VariableExpression::Grouping(Box::new(
                        VariableExpression::BinaryExpression(Box::new(BinaryExpression::And {
                            left: VariableExpression::VariableValue(VariableValue::Identifier(
                                "b".to_string().into()
                            )),
                            right: VariableExpression::UnaryExpression(Box::new(
                                UnaryExpression::Not(VariableExpression::VariableValue(
                                    VariableValue::Identifier("c".to_string().into())
                                ))
                            ))
                        }))
                    ))
                }))
            )))
        );

        let mut reader = TokenReader::new("(!(a))".as_bytes());
        assert_eq!(
            VariableExpression::parse(reader.next_token().unwrap(), &mut reader),
            Ok(VariableExpression::Grouping(Box::new(
                VariableExpression::UnaryExpression(Box::new(UnaryExpression::Not(
                    VariableExpression::Grouping(Box::new(VariableExpression::VariableValue(
                        VariableValue::Identifier("a".to_string().into())
                    )))
                )))
            )))
        );
    }
}
