use super::{
    BinaryExpType, BinaryExpression, Identifier, UnaryExpType, UnaryExpression, VariableValue,
};
use crate::llvm_ast;
use lexer::{Logical, Separator, Token, TokenReader};
use precompiler::{
    self,
    rpn::{Expression, Operation, RPN},
    Precompiler,
};
use std::io::Read;

/// VariableExpression
#[derive(Clone, Debug, PartialEq)]
pub enum VariableExpression {
    VariableValue(VariableValue),
    UnaryExpression(Box<UnaryExpression>),
    BinaryExpression(Box<BinaryExpression>),
    Grouping(Box<Self>),
}

impl VariableExpression {
    fn parse_impl<R: Read>(
        cur_token: Token,
        reader: &mut TokenReader<R>,
        is_unary: bool,
    ) -> Result<Self, lexer::Error> {
        let left = match cur_token {
            Token::Logical(Logical::Not) => Self::UnaryExpression(Box::new(UnaryExpression {
                exp: Self::parse_impl(reader.next_token()?, reader, true)?,
                exp_type: UnaryExpType::Not,
            })),
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
                    Ok(Self::BinaryExpression(Box::new(BinaryExpression {
                        left,
                        right: Self::parse_impl(reader.next_token()?, reader, false)?,
                        op_type: BinaryExpType::Or,
                    })))
                }
                Token::Logical(Logical::And) => {
                    reader.reset_saving();
                    Ok(Self::BinaryExpression(Box::new(BinaryExpression {
                        left,
                        right: Self::parse_impl(reader.next_token()?, reader, false)?,
                        op_type: BinaryExpType::And,
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

impl VariableExpression {
    pub fn parse<R: Read>(
        cur_token: Token,
        reader: &mut TokenReader<R>,
    ) -> Result<Self, lexer::Error> {
        Self::parse_impl(cur_token, reader, false)
    }
}

impl VariableExpression {
    pub fn evaluate(self, rpn: &mut RPN<VariableValue, UnaryExpression, BinaryExpression>) {
        match self {
            Self::VariableValue(value) => rpn.transform_from_infix(Expression::Value(value)),
            Self::UnaryExpression(unary_expression) => rpn.transform_from_infix(
                Expression::Operation(Operation::PrefixOp(*unary_expression)),
            ),
            Self::BinaryExpression(binary_expression) => rpn.transform_from_infix(
                Expression::Operation(Operation::BinaryOp(*binary_expression)),
            ),
            Self::Grouping(grouping) => {
                rpn.transform_from_infix(Expression::OpenBrace);
                grouping.evaluate(rpn);
                rpn.transform_from_infix(Expression::CloseBrace);
            }
        }
    }

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
                UnaryExpression {
                    exp: VariableExpression::VariableValue(VariableValue::Boolean(true)),
                    exp_type: UnaryExpType::Not,
                },
            ))),
        );

        let mut reader = TokenReader::new("!false".as_bytes());
        assert_eq!(
            VariableExpression::parse(reader.next_token().unwrap(), &mut reader),
            Ok(VariableExpression::UnaryExpression(Box::new(
                UnaryExpression {
                    exp: VariableExpression::VariableValue(VariableValue::Boolean(false)),
                    exp_type: UnaryExpType::Not,
                }
            ))),
        );

        let mut reader = TokenReader::new("!a".as_bytes());
        assert_eq!(
            VariableExpression::parse(reader.next_token().unwrap(), &mut reader),
            Ok(VariableExpression::UnaryExpression(Box::new(
                UnaryExpression {
                    exp: VariableExpression::VariableValue(VariableValue::Identifier(
                        "a".to_string().into()
                    )),
                    exp_type: UnaryExpType::Not,
                }
            ))),
        );
    }

    #[test]
    fn parse_and_logical_expression_test() {
        let mut reader = TokenReader::new("true && false".as_bytes());
        assert_eq!(
            VariableExpression::parse(reader.next_token().unwrap(), &mut reader),
            Ok(VariableExpression::BinaryExpression(Box::new(
                BinaryExpression {
                    left: VariableExpression::VariableValue(VariableValue::Boolean(true)),
                    right: VariableExpression::VariableValue(VariableValue::Boolean(false)),
                    op_type: BinaryExpType::And,
                }
            ))),
        );

        let mut reader = TokenReader::new("false && a".as_bytes());
        assert_eq!(
            VariableExpression::parse(reader.next_token().unwrap(), &mut reader),
            Ok(VariableExpression::BinaryExpression(Box::new(
                BinaryExpression {
                    left: VariableExpression::VariableValue(VariableValue::Boolean(false)),
                    right: VariableExpression::VariableValue(VariableValue::Identifier(
                        "a".to_string().into()
                    )),
                    op_type: BinaryExpType::And,
                }
            ))),
        );

        let mut reader = TokenReader::new("a && b".as_bytes());
        assert_eq!(
            VariableExpression::parse(reader.next_token().unwrap(), &mut reader),
            Ok(VariableExpression::BinaryExpression(Box::new(
                BinaryExpression {
                    left: VariableExpression::VariableValue(VariableValue::Identifier(
                        "a".to_string().into()
                    )),
                    right: VariableExpression::VariableValue(VariableValue::Identifier(
                        "b".to_string().into()
                    )),
                    op_type: BinaryExpType::And,
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
                BinaryExpression {
                    left: VariableExpression::VariableValue(VariableValue::Boolean(true)),
                    right: VariableExpression::VariableValue(VariableValue::Boolean(false)),
                    op_type: BinaryExpType::Or,
                }
            ))),
        );

        let mut reader = TokenReader::new("false || a".as_bytes());
        assert_eq!(
            VariableExpression::parse(reader.next_token().unwrap(), &mut reader),
            Ok(VariableExpression::BinaryExpression(Box::new(
                BinaryExpression {
                    left: VariableExpression::VariableValue(VariableValue::Boolean(false)),
                    right: VariableExpression::VariableValue(VariableValue::Identifier(
                        "a".to_string().into()
                    )),
                    op_type: BinaryExpType::Or,
                }
            ))),
        );

        let mut reader = TokenReader::new("a || b".as_bytes());
        assert_eq!(
            VariableExpression::parse(reader.next_token().unwrap(), &mut reader),
            Ok(VariableExpression::BinaryExpression(Box::new(
                BinaryExpression {
                    left: VariableExpression::VariableValue(VariableValue::Identifier(
                        "a".to_string().into()
                    )),
                    right: VariableExpression::VariableValue(VariableValue::Identifier(
                        "b".to_string().into()
                    )),
                    op_type: BinaryExpType::Or,
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
                BinaryExpression {
                    left: VariableExpression::UnaryExpression(Box::new(UnaryExpression {
                        exp: VariableExpression::VariableValue(VariableValue::Identifier(
                            "a".to_string().into()
                        )),
                        exp_type: UnaryExpType::Not,
                    })),
                    right: VariableExpression::BinaryExpression(Box::new(BinaryExpression {
                        left: VariableExpression::VariableValue(VariableValue::Identifier(
                            "b".to_string().into()
                        )),
                        right: VariableExpression::UnaryExpression(Box::new(UnaryExpression {
                            exp: VariableExpression::VariableValue(VariableValue::Identifier(
                                "c".to_string().into()
                            )),
                            exp_type: UnaryExpType::Not,
                        })),
                        op_type: BinaryExpType::And,
                    })),
                    op_type: BinaryExpType::Or,
                }
            ))),
        );

        let mut reader = TokenReader::new("!!!!a".as_bytes());
        assert_eq!(
            VariableExpression::parse(reader.next_token().unwrap(), &mut reader),
            Ok(VariableExpression::UnaryExpression(Box::new(
                UnaryExpression {
                    exp: VariableExpression::UnaryExpression(Box::new(UnaryExpression {
                        exp: VariableExpression::UnaryExpression(Box::new(UnaryExpression {
                            exp: VariableExpression::UnaryExpression(Box::new(UnaryExpression {
                                exp: VariableExpression::VariableValue(VariableValue::Identifier(
                                    "a".to_string().into()
                                )),
                                exp_type: UnaryExpType::Not
                            })),
                            exp_type: UnaryExpType::Not,
                        })),
                        exp_type: UnaryExpType::Not,
                    })),
                    exp_type: UnaryExpType::Not,
                }
            )))
        );
    }

    #[test]
    fn parse_grouping_test() {
        let mut reader = TokenReader::new("(!a || (b && !c))".as_bytes());
        assert_eq!(
            VariableExpression::parse(reader.next_token().unwrap(), &mut reader),
            Ok(VariableExpression::Grouping(Box::new(
                VariableExpression::BinaryExpression(Box::new(BinaryExpression {
                    left: VariableExpression::UnaryExpression(Box::new(UnaryExpression {
                        exp: VariableExpression::VariableValue(VariableValue::Identifier(
                            "a".to_string().into()
                        )),
                        exp_type: UnaryExpType::Not,
                    })),
                    right: VariableExpression::Grouping(Box::new(
                        VariableExpression::BinaryExpression(Box::new(BinaryExpression {
                            left: VariableExpression::VariableValue(VariableValue::Identifier(
                                "b".to_string().into()
                            )),
                            right: VariableExpression::UnaryExpression(Box::new(UnaryExpression {
                                exp: VariableExpression::VariableValue(VariableValue::Identifier(
                                    "c".to_string().into()
                                )),
                                exp_type: UnaryExpType::Not,
                            })),
                            op_type: BinaryExpType::And,
                        }))
                    )),
                    op_type: BinaryExpType::Or,
                }))
            )))
        );

        let mut reader = TokenReader::new("(!(a))".as_bytes());
        assert_eq!(
            VariableExpression::parse(reader.next_token().unwrap(), &mut reader),
            Ok(VariableExpression::Grouping(Box::new(
                VariableExpression::UnaryExpression(Box::new(UnaryExpression {
                    exp: VariableExpression::Grouping(Box::new(VariableExpression::VariableValue(
                        VariableValue::Identifier("a".to_string().into())
                    ))),
                    exp_type: UnaryExpType::Not
                }))
            )))
        );
    }
}
