use super::{
    BinaryExpType, BinaryExpression, Identifier, UnaryExpType, UnaryExpression, VariableValue,
};
use crate::{llvm_ast, Error};
use lexer::{Logical, Separator, Token, TokenReader};
use precompiler::{
    self,
    rpn::{
        input::{InputExpression, Operation, Value},
        output::OutputExpression,
        RPN,
    },
    Precompiler,
};
use std::io::Read;

/// VariableExpression
#[derive(Clone, Debug, PartialEq)]
pub enum VariableExpression {
    VariableValue(VariableValue),
    UnaryExpression(Box<UnaryExpression>),
    BinaryExpression(Box<BinaryExpression>),
}

impl From<OutputExpression<VariableValue, UnaryExpType, BinaryExpType>> for VariableExpression {
    fn from(val: OutputExpression<VariableValue, UnaryExpType, BinaryExpType>) -> Self {
        match val {
            OutputExpression::Value(value) => Self::VariableValue(value),
            OutputExpression::UnaryExpression(expr) => {
                Self::UnaryExpression(Box::new(UnaryExpression {
                    exp: expr.exp.into(),
                    exp_type: expr.op_type,
                }))
            }
            OutputExpression::BinaryExpression(expr) => {
                Self::BinaryExpression(Box::new(BinaryExpression {
                    left: expr.left.into(),
                    right: expr.right.into(),
                    op_type: expr.op_type,
                }))
            }
        }
    }
}

impl VariableExpression {
    pub fn parse<R: Read>(cur_token: Token, reader: &mut TokenReader<R>) -> Result<Self, Error> {
        let mut rpn = RPN::new();
        Self::parse_impl(cur_token, reader, &mut rpn, false)?;
        Ok(rpn.finish()?.evaluate().into())
    }

    pub fn parse_impl<R: Read>(
        cur_token: Token,
        reader: &mut TokenReader<R>,
        rpn: &mut RPN<VariableValue, UnaryExpType, BinaryExpType>,
        is_unary: bool,
    ) -> Result<(), Error> {
        match cur_token {
            Token::Logical(Logical::Not) => {
                rpn.build(InputExpression::Value(Value::Operation(
                    Operation::PrefixOp(UnaryExpType::Not),
                )))?;
                Self::parse_impl(reader.next_token()?, reader, rpn, true)?;
            }
            Token::Separator(Separator::OpenBrace) => {
                rpn.build(InputExpression::OpenBrace)?;
                Self::parse_impl(reader.next_token()?, reader, rpn, false)?;
                match reader.next_token()? {
                    Token::Separator(Separator::CloseBrace) => {
                        rpn.build(InputExpression::CloseBrace)?;
                    }
                    token => return Err(Error::UnexpectedToken(token)),
                }
            }
            token => {
                rpn.build(InputExpression::Value(Value::Value(VariableValue::parse(
                    token, reader,
                )?)))?;
            }
        }
        if !is_unary {
            reader.start_saving();
            match reader.next_token()? {
                Token::Logical(Logical::Or) => {
                    reader.reset_saving();
                    rpn.build(InputExpression::Value(Value::Operation(
                        Operation::BinaryOp(BinaryExpType::Or),
                    )))?;
                    Self::parse_impl(reader.next_token()?, reader, rpn, false)?;
                }
                Token::Logical(Logical::And) => {
                    reader.reset_saving();
                    rpn.build(InputExpression::Value(Value::Operation(
                        Operation::BinaryOp(BinaryExpType::And),
                    )))?;
                    Self::parse_impl(reader.next_token()?, reader, rpn, false)?;
                }
                _ => {
                    reader.stop_saving();
                }
            }
        }
        Ok(())
    }

    pub fn precompile(
        self,
        precompiler: &mut Precompiler<Identifier, llvm_ast::FunctionDeclaration>,
    ) -> Result<llvm_ast::VariableExpression, precompiler::Error<Identifier>> {
        match self {
            Self::VariableValue(value) => Ok(llvm_ast::VariableExpression::VariableValue(
                value.precompile(precompiler)?,
            )),
            Self::UnaryExpression(_) => todo!("implement"),
            Self::BinaryExpression(_) => todo!("implement"),
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
                    left: VariableExpression::BinaryExpression(Box::new(BinaryExpression {
                        left: VariableExpression::UnaryExpression(Box::new(UnaryExpression {
                            exp: VariableExpression::VariableValue(VariableValue::Identifier(
                                "a".to_string().into()
                            )),
                            exp_type: UnaryExpType::Not,
                        })),
                        right: VariableExpression::VariableValue(VariableValue::Identifier(
                            "b".to_string().into()
                        )),
                        op_type: BinaryExpType::Or,
                    })),
                    right: VariableExpression::UnaryExpression(Box::new(UnaryExpression {
                        exp: VariableExpression::VariableValue(VariableValue::Identifier(
                            "c".to_string().into()
                        )),
                        exp_type: UnaryExpType::Not,
                    })),
                    op_type: BinaryExpType::And,
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
            )))
        );

        let mut reader = TokenReader::new("(!(a))".as_bytes());
        assert_eq!(
            VariableExpression::parse(reader.next_token().unwrap(), &mut reader),
            Ok(VariableExpression::UnaryExpression(Box::new(
                UnaryExpression {
                    exp: VariableExpression::VariableValue(VariableValue::Identifier(
                        "a".to_string().into()
                    )),
                    exp_type: UnaryExpType::Not
                }
            )))
        );
    }
}
