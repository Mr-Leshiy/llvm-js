use super::{
    BinaryExpType, BinaryExpression, FunctionCall, UnaryExpType, UnaryExpression, VariableValue,
};
use crate::{llvm_ast, LexerError, Precompiler, PrecompilerError};
use lexer::{Arithmetic, Logical, Separator, Token, TokenReader};
use rpn::{
    input::{InputExpression, Operation, Value},
    output::OutputExpression,
    RPN,
};
use std::io::Read;

#[derive(Clone, Debug, PartialEq)]
pub enum VariableExpression {
    VariableValue(VariableValue),
    UnaryExpression(Box<UnaryExpression>),
    BinaryExpression(Box<BinaryExpression>),
    FunctionCall(FunctionCall),
}

impl From<OutputExpression<RpnValue, UnaryExpType, BinaryExpType>> for VariableExpression {
    fn from(val: OutputExpression<RpnValue, UnaryExpType, BinaryExpType>) -> Self {
        match val {
            OutputExpression::Value(RpnValue::VariableValue(value)) => Self::VariableValue(value),
            OutputExpression::Value(RpnValue::FunctionCall(function_call)) => {
                Self::FunctionCall(function_call)
            }
            OutputExpression::UnaryExpression(expr) => {
                Self::UnaryExpression(Box::new(UnaryExpression {
                    exp: expr.exp.into(),
                    exp_type: expr.exp_type,
                }))
            }
            OutputExpression::BinaryExpression(expr) => {
                Self::BinaryExpression(Box::new(BinaryExpression {
                    left: expr.left.into(),
                    right: expr.right.into(),
                    exp_type: expr.exp_type,
                }))
            }
        }
    }
}

enum RpnValue {
    VariableValue(VariableValue),
    FunctionCall(FunctionCall),
}

impl From<FunctionCall> for RpnValue {
    fn from(val: FunctionCall) -> Self {
        Self::FunctionCall(val)
    }
}

impl From<VariableValue> for RpnValue {
    fn from(val: VariableValue) -> Self {
        Self::VariableValue(val)
    }
}

impl VariableExpression {
    pub fn parse<R: Read>(
        cur_token: Token,
        reader: &mut TokenReader<R>,
    ) -> Result<Self, LexerError> {
        let mut rpn = RPN::new();
        Self::parse_impl(cur_token, reader, &mut rpn, false)?;
        Ok(rpn.finish()?.evaluate().into())
    }

    fn parse_impl<R: Read>(
        cur_token: Token,
        reader: &mut TokenReader<R>,
        rpn: &mut RPN<RpnValue, UnaryExpType, BinaryExpType>,
        is_unary: bool,
    ) -> Result<(), LexerError> {
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
                    token => return Err(LexerError::UnexpectedToken(token)),
                }
            }
            Token::Ident(_) => {
                reader.start_saving();
                if let Ok(function_call) = FunctionCall::parse(cur_token.clone(), reader) {
                    reader.reset_saving();
                    rpn.build(InputExpression::Value(Value::Value(function_call.into())))?;
                } else {
                    reader.stop_saving();
                    rpn.build(InputExpression::Value(Value::Value(
                        VariableValue::parse(cur_token, reader)?.into(),
                    )))?;
                }
            }
            token => {
                rpn.build(InputExpression::Value(Value::Value(
                    VariableValue::parse(token, reader)?.into(),
                )))?;
            }
        }
        if !is_unary {
            reader.start_saving();
            let parse_binary_op = |reader: &mut TokenReader<R>,
                                   rpn: &mut RPN<RpnValue, UnaryExpType, BinaryExpType>,
                                   exp_type|
             -> Result<(), LexerError> {
                reader.reset_saving();
                rpn.build(InputExpression::Value(Value::Operation(
                    Operation::BinaryOp(exp_type),
                )))?;
                Self::parse_impl(reader.next_token()?, reader, rpn, false)?;
                Ok(())
            };

            match reader.next_token()? {
                // Logical
                Token::Logical(Logical::Or) => {
                    parse_binary_op(reader, rpn, BinaryExpType::Or)?;
                }
                Token::Logical(Logical::And) => {
                    parse_binary_op(reader, rpn, BinaryExpType::And)?;
                }
                Token::Logical(Logical::Eq) => {
                    parse_binary_op(reader, rpn, BinaryExpType::Eq)?;
                }
                Token::Logical(Logical::Ne) => {
                    parse_binary_op(reader, rpn, BinaryExpType::Ne)?;
                }
                Token::Logical(Logical::SEq) => {
                    parse_binary_op(reader, rpn, BinaryExpType::SEq)?;
                }
                Token::Logical(Logical::SNe) => {
                    parse_binary_op(reader, rpn, BinaryExpType::SNe)?;
                }
                Token::Logical(Logical::Gt) => {
                    parse_binary_op(reader, rpn, BinaryExpType::Gt)?;
                }
                Token::Logical(Logical::Ge) => {
                    parse_binary_op(reader, rpn, BinaryExpType::Ge)?;
                }
                Token::Logical(Logical::Lt) => {
                    parse_binary_op(reader, rpn, BinaryExpType::Lt)?;
                }
                Token::Logical(Logical::Le) => {
                    parse_binary_op(reader, rpn, BinaryExpType::Le)?;
                }
                // Arithmetic
                Token::Arithmetic(Arithmetic::Add) => {
                    parse_binary_op(reader, rpn, BinaryExpType::Add)?;
                }
                Token::Arithmetic(Arithmetic::Sub) => {
                    parse_binary_op(reader, rpn, BinaryExpType::Sub)?;
                }
                Token::Arithmetic(Arithmetic::Div) => {
                    parse_binary_op(reader, rpn, BinaryExpType::Div)?;
                }
                Token::Arithmetic(Arithmetic::Mul) => {
                    parse_binary_op(reader, rpn, BinaryExpType::Mul)?;
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
        precompiler: &mut Precompiler,
    ) -> Result<llvm_ast::VariableExpression, PrecompilerError> {
        match self {
            Self::VariableValue(value) => Ok(llvm_ast::VariableExpression::VariableValue(
                value.precompile(precompiler)?,
            )),
            Self::UnaryExpression(expr) => Ok(llvm_ast::VariableExpression::UnaryExpression(
                Box::new(expr.precompile(precompiler)?),
            )),
            Self::BinaryExpression(expr) => Ok(llvm_ast::VariableExpression::BinaryExpression(
                Box::new(expr.precompile(precompiler)?),
            )),
            Self::FunctionCall(function_call) => Ok(llvm_ast::VariableExpression::FunctionCall(
                function_call.precompile(precompiler)?,
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::js_ast::MemberExpression;

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
                    exp: VariableExpression::VariableValue(VariableValue::MemberExpression(
                        MemberExpression {
                            variable_name: "a".to_string().into(),
                            property: None
                        }
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
                    exp_type: BinaryExpType::And,
                }
            ))),
        );

        let mut reader = TokenReader::new("false && a".as_bytes());
        assert_eq!(
            VariableExpression::parse(reader.next_token().unwrap(), &mut reader),
            Ok(VariableExpression::BinaryExpression(Box::new(
                BinaryExpression {
                    left: VariableExpression::VariableValue(VariableValue::Boolean(false)),
                    right: VariableExpression::VariableValue(VariableValue::MemberExpression(
                        MemberExpression {
                            variable_name: "a".to_string().into(),
                            property: None,
                        }
                    )),
                    exp_type: BinaryExpType::And,
                }
            ))),
        );

        let mut reader = TokenReader::new("a && b".as_bytes());
        assert_eq!(
            VariableExpression::parse(reader.next_token().unwrap(), &mut reader),
            Ok(VariableExpression::BinaryExpression(Box::new(
                BinaryExpression {
                    left: VariableExpression::VariableValue(VariableValue::MemberExpression(
                        MemberExpression {
                            variable_name: "a".to_string().into(),
                            property: None
                        }
                    )),
                    right: VariableExpression::VariableValue(VariableValue::MemberExpression(
                        MemberExpression {
                            variable_name: "b".to_string().into(),
                            property: None
                        }
                    )),
                    exp_type: BinaryExpType::And,
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
                    exp_type: BinaryExpType::Or,
                }
            ))),
        );

        let mut reader = TokenReader::new("false || a".as_bytes());
        assert_eq!(
            VariableExpression::parse(reader.next_token().unwrap(), &mut reader),
            Ok(VariableExpression::BinaryExpression(Box::new(
                BinaryExpression {
                    left: VariableExpression::VariableValue(VariableValue::Boolean(false)),
                    right: VariableExpression::VariableValue(VariableValue::MemberExpression(
                        MemberExpression {
                            variable_name: "a".to_string().into(),
                            property: None,
                        }
                    )),
                    exp_type: BinaryExpType::Or,
                }
            ))),
        );

        let mut reader = TokenReader::new("a || b".as_bytes());
        assert_eq!(
            VariableExpression::parse(reader.next_token().unwrap(), &mut reader),
            Ok(VariableExpression::BinaryExpression(Box::new(
                BinaryExpression {
                    left: VariableExpression::VariableValue(VariableValue::MemberExpression(
                        MemberExpression {
                            variable_name: "a".to_string().into(),
                            property: None
                        }
                    )),
                    right: VariableExpression::VariableValue(VariableValue::MemberExpression(
                        MemberExpression {
                            variable_name: "b".to_string().into(),
                            property: None
                        }
                    )),
                    exp_type: BinaryExpType::Or,
                }
            ))),
        );
    }

    #[test]
    fn parse_eq_logical_expression_test() {
        let mut reader = TokenReader::new("true == false".as_bytes());
        assert_eq!(
            VariableExpression::parse(reader.next_token().unwrap(), &mut reader),
            Ok(VariableExpression::BinaryExpression(Box::new(
                BinaryExpression {
                    left: VariableExpression::VariableValue(VariableValue::Boolean(true)),
                    right: VariableExpression::VariableValue(VariableValue::Boolean(false)),
                    exp_type: BinaryExpType::Eq,
                }
            ))),
        );

        let mut reader = TokenReader::new("false == a".as_bytes());
        assert_eq!(
            VariableExpression::parse(reader.next_token().unwrap(), &mut reader),
            Ok(VariableExpression::BinaryExpression(Box::new(
                BinaryExpression {
                    left: VariableExpression::VariableValue(VariableValue::Boolean(false)),
                    right: VariableExpression::VariableValue(VariableValue::MemberExpression(
                        MemberExpression {
                            variable_name: "a".to_string().into(),
                            property: None,
                        }
                    )),
                    exp_type: BinaryExpType::Eq,
                }
            ))),
        );

        let mut reader = TokenReader::new("a == b".as_bytes());
        assert_eq!(
            VariableExpression::parse(reader.next_token().unwrap(), &mut reader),
            Ok(VariableExpression::BinaryExpression(Box::new(
                BinaryExpression {
                    left: VariableExpression::VariableValue(VariableValue::MemberExpression(
                        MemberExpression {
                            variable_name: "a".to_string().into(),
                            property: None
                        }
                    )),
                    right: VariableExpression::VariableValue(VariableValue::MemberExpression(
                        MemberExpression {
                            variable_name: "b".to_string().into(),
                            property: None
                        }
                    )),
                    exp_type: BinaryExpType::Eq,
                }
            ))),
        );
    }

    #[test]
    fn parse_ne_logical_expression_test() {
        let mut reader = TokenReader::new("true != false".as_bytes());
        assert_eq!(
            VariableExpression::parse(reader.next_token().unwrap(), &mut reader),
            Ok(VariableExpression::BinaryExpression(Box::new(
                BinaryExpression {
                    left: VariableExpression::VariableValue(VariableValue::Boolean(true)),
                    right: VariableExpression::VariableValue(VariableValue::Boolean(false)),
                    exp_type: BinaryExpType::Ne,
                }
            ))),
        );

        let mut reader = TokenReader::new("false != a".as_bytes());
        assert_eq!(
            VariableExpression::parse(reader.next_token().unwrap(), &mut reader),
            Ok(VariableExpression::BinaryExpression(Box::new(
                BinaryExpression {
                    left: VariableExpression::VariableValue(VariableValue::Boolean(false)),
                    right: VariableExpression::VariableValue(VariableValue::MemberExpression(
                        MemberExpression {
                            variable_name: "a".to_string().into(),
                            property: None
                        }
                    )),
                    exp_type: BinaryExpType::Ne,
                }
            ))),
        );

        let mut reader = TokenReader::new("a != b".as_bytes());
        assert_eq!(
            VariableExpression::parse(reader.next_token().unwrap(), &mut reader),
            Ok(VariableExpression::BinaryExpression(Box::new(
                BinaryExpression {
                    left: VariableExpression::VariableValue(VariableValue::MemberExpression(
                        MemberExpression {
                            variable_name: "a".to_string().into(),
                            property: None
                        }
                    )),
                    right: VariableExpression::VariableValue(VariableValue::MemberExpression(
                        MemberExpression {
                            variable_name: "b".to_string().into(),
                            property: None
                        }
                    )),
                    exp_type: BinaryExpType::Ne,
                }
            ))),
        );
    }

    #[test]
    fn parse_seq_logical_expression_test() {
        let mut reader = TokenReader::new("true === false".as_bytes());
        assert_eq!(
            VariableExpression::parse(reader.next_token().unwrap(), &mut reader),
            Ok(VariableExpression::BinaryExpression(Box::new(
                BinaryExpression {
                    left: VariableExpression::VariableValue(VariableValue::Boolean(true)),
                    right: VariableExpression::VariableValue(VariableValue::Boolean(false)),
                    exp_type: BinaryExpType::SEq,
                }
            ))),
        );

        let mut reader = TokenReader::new("false === a".as_bytes());
        assert_eq!(
            VariableExpression::parse(reader.next_token().unwrap(), &mut reader),
            Ok(VariableExpression::BinaryExpression(Box::new(
                BinaryExpression {
                    left: VariableExpression::VariableValue(VariableValue::Boolean(false)),
                    right: VariableExpression::VariableValue(VariableValue::MemberExpression(
                        MemberExpression {
                            variable_name: "a".to_string().into(),
                            property: None,
                        }
                    )),
                    exp_type: BinaryExpType::SEq,
                }
            ))),
        );

        let mut reader = TokenReader::new("a === b".as_bytes());
        assert_eq!(
            VariableExpression::parse(reader.next_token().unwrap(), &mut reader),
            Ok(VariableExpression::BinaryExpression(Box::new(
                BinaryExpression {
                    left: VariableExpression::VariableValue(VariableValue::MemberExpression(
                        MemberExpression {
                            variable_name: "a".to_string().into(),
                            property: None
                        }
                    )),
                    right: VariableExpression::VariableValue(VariableValue::MemberExpression(
                        MemberExpression {
                            variable_name: "b".to_string().into(),
                            property: None
                        }
                    )),
                    exp_type: BinaryExpType::SEq,
                }
            ))),
        );
    }

    #[test]
    fn parse_sne_logical_expression_test() {
        let mut reader = TokenReader::new("true !== false".as_bytes());
        assert_eq!(
            VariableExpression::parse(reader.next_token().unwrap(), &mut reader),
            Ok(VariableExpression::BinaryExpression(Box::new(
                BinaryExpression {
                    left: VariableExpression::VariableValue(VariableValue::Boolean(true)),
                    right: VariableExpression::VariableValue(VariableValue::Boolean(false)),
                    exp_type: BinaryExpType::SNe,
                }
            ))),
        );

        let mut reader = TokenReader::new("false !== a".as_bytes());
        assert_eq!(
            VariableExpression::parse(reader.next_token().unwrap(), &mut reader),
            Ok(VariableExpression::BinaryExpression(Box::new(
                BinaryExpression {
                    left: VariableExpression::VariableValue(VariableValue::Boolean(false)),
                    right: VariableExpression::VariableValue(VariableValue::MemberExpression(
                        MemberExpression {
                            variable_name: "a".to_string().into(),
                            property: None
                        }
                    )),
                    exp_type: BinaryExpType::SNe,
                }
            ))),
        );

        let mut reader = TokenReader::new("a !== b".as_bytes());
        assert_eq!(
            VariableExpression::parse(reader.next_token().unwrap(), &mut reader),
            Ok(VariableExpression::BinaryExpression(Box::new(
                BinaryExpression {
                    left: VariableExpression::VariableValue(VariableValue::MemberExpression(
                        MemberExpression {
                            variable_name: "a".to_string().into(),
                            property: None
                        }
                    )),
                    right: VariableExpression::VariableValue(VariableValue::MemberExpression(
                        MemberExpression {
                            variable_name: "b".to_string().into(),
                            property: None
                        }
                    )),
                    exp_type: BinaryExpType::SNe,
                }
            ))),
        );
    }

    #[test]
    fn parse_gt_logical_expression_test() {
        let mut reader = TokenReader::new("true > false".as_bytes());
        assert_eq!(
            VariableExpression::parse(reader.next_token().unwrap(), &mut reader),
            Ok(VariableExpression::BinaryExpression(Box::new(
                BinaryExpression {
                    left: VariableExpression::VariableValue(VariableValue::Boolean(true)),
                    right: VariableExpression::VariableValue(VariableValue::Boolean(false)),
                    exp_type: BinaryExpType::Gt,
                }
            ))),
        );

        let mut reader = TokenReader::new("false > a".as_bytes());
        assert_eq!(
            VariableExpression::parse(reader.next_token().unwrap(), &mut reader),
            Ok(VariableExpression::BinaryExpression(Box::new(
                BinaryExpression {
                    left: VariableExpression::VariableValue(VariableValue::Boolean(false)),
                    right: VariableExpression::VariableValue(VariableValue::MemberExpression(
                        MemberExpression {
                            variable_name: "a".to_string().into(),
                            property: None
                        }
                    )),
                    exp_type: BinaryExpType::Gt,
                }
            ))),
        );

        let mut reader = TokenReader::new("a > b".as_bytes());
        assert_eq!(
            VariableExpression::parse(reader.next_token().unwrap(), &mut reader),
            Ok(VariableExpression::BinaryExpression(Box::new(
                BinaryExpression {
                    left: VariableExpression::VariableValue(VariableValue::MemberExpression(
                        MemberExpression {
                            variable_name: "a".to_string().into(),
                            property: None
                        }
                    )),
                    right: VariableExpression::VariableValue(VariableValue::MemberExpression(
                        MemberExpression {
                            variable_name: "b".to_string().into(),
                            property: None
                        }
                    )),
                    exp_type: BinaryExpType::Gt,
                }
            ))),
        );
    }

    #[test]
    fn parse_ge_logical_expression_test() {
        let mut reader = TokenReader::new("true >= false".as_bytes());
        assert_eq!(
            VariableExpression::parse(reader.next_token().unwrap(), &mut reader),
            Ok(VariableExpression::BinaryExpression(Box::new(
                BinaryExpression {
                    left: VariableExpression::VariableValue(VariableValue::Boolean(true)),
                    right: VariableExpression::VariableValue(VariableValue::Boolean(false)),
                    exp_type: BinaryExpType::Ge,
                }
            ))),
        );

        let mut reader = TokenReader::new("false >= a".as_bytes());
        assert_eq!(
            VariableExpression::parse(reader.next_token().unwrap(), &mut reader),
            Ok(VariableExpression::BinaryExpression(Box::new(
                BinaryExpression {
                    left: VariableExpression::VariableValue(VariableValue::Boolean(false)),
                    right: VariableExpression::VariableValue(VariableValue::MemberExpression(
                        MemberExpression {
                            variable_name: "a".to_string().into(),
                            property: None
                        }
                    )),
                    exp_type: BinaryExpType::Ge,
                }
            ))),
        );

        let mut reader = TokenReader::new("a >= b".as_bytes());
        assert_eq!(
            VariableExpression::parse(reader.next_token().unwrap(), &mut reader),
            Ok(VariableExpression::BinaryExpression(Box::new(
                BinaryExpression {
                    left: VariableExpression::VariableValue(VariableValue::MemberExpression(
                        MemberExpression {
                            variable_name: "a".to_string().into(),
                            property: None
                        }
                    )),
                    right: VariableExpression::VariableValue(VariableValue::MemberExpression(
                        MemberExpression {
                            variable_name: "b".to_string().into(),
                            property: None
                        }
                    )),
                    exp_type: BinaryExpType::Ge,
                }
            ))),
        );
    }

    #[test]
    fn parse_lt_logical_expression_test() {
        let mut reader = TokenReader::new("true < false".as_bytes());
        assert_eq!(
            VariableExpression::parse(reader.next_token().unwrap(), &mut reader),
            Ok(VariableExpression::BinaryExpression(Box::new(
                BinaryExpression {
                    left: VariableExpression::VariableValue(VariableValue::Boolean(true)),
                    right: VariableExpression::VariableValue(VariableValue::Boolean(false)),
                    exp_type: BinaryExpType::Lt,
                }
            ))),
        );

        let mut reader = TokenReader::new("false < a".as_bytes());
        assert_eq!(
            VariableExpression::parse(reader.next_token().unwrap(), &mut reader),
            Ok(VariableExpression::BinaryExpression(Box::new(
                BinaryExpression {
                    left: VariableExpression::VariableValue(VariableValue::Boolean(false)),
                    right: VariableExpression::VariableValue(VariableValue::MemberExpression(
                        MemberExpression {
                            variable_name: "a".to_string().into(),
                            property: None
                        }
                    )),
                    exp_type: BinaryExpType::Lt,
                }
            ))),
        );

        let mut reader = TokenReader::new("a < b".as_bytes());
        assert_eq!(
            VariableExpression::parse(reader.next_token().unwrap(), &mut reader),
            Ok(VariableExpression::BinaryExpression(Box::new(
                BinaryExpression {
                    left: VariableExpression::VariableValue(VariableValue::MemberExpression(
                        MemberExpression {
                            variable_name: "a".to_string().into(),
                            property: None
                        }
                    )),
                    right: VariableExpression::VariableValue(VariableValue::MemberExpression(
                        MemberExpression {
                            variable_name: "b".to_string().into(),
                            property: None
                        }
                    )),
                    exp_type: BinaryExpType::Lt,
                }
            ))),
        );
    }

    #[test]
    fn parse_le_logical_expression_test() {
        let mut reader = TokenReader::new("true <= false".as_bytes());
        assert_eq!(
            VariableExpression::parse(reader.next_token().unwrap(), &mut reader),
            Ok(VariableExpression::BinaryExpression(Box::new(
                BinaryExpression {
                    left: VariableExpression::VariableValue(VariableValue::Boolean(true)),
                    right: VariableExpression::VariableValue(VariableValue::Boolean(false)),
                    exp_type: BinaryExpType::Le,
                }
            ))),
        );

        let mut reader = TokenReader::new("false <= a".as_bytes());
        assert_eq!(
            VariableExpression::parse(reader.next_token().unwrap(), &mut reader),
            Ok(VariableExpression::BinaryExpression(Box::new(
                BinaryExpression {
                    left: VariableExpression::VariableValue(VariableValue::Boolean(false)),
                    right: VariableExpression::VariableValue(VariableValue::MemberExpression(
                        MemberExpression {
                            variable_name: "a".to_string().into(),
                            property: None
                        }
                    )),
                    exp_type: BinaryExpType::Le,
                }
            ))),
        );

        let mut reader = TokenReader::new("a <= b".as_bytes());
        assert_eq!(
            VariableExpression::parse(reader.next_token().unwrap(), &mut reader),
            Ok(VariableExpression::BinaryExpression(Box::new(
                BinaryExpression {
                    left: VariableExpression::VariableValue(VariableValue::MemberExpression(
                        MemberExpression {
                            variable_name: "a".to_string().into(),
                            property: None
                        }
                    )),
                    right: VariableExpression::VariableValue(VariableValue::MemberExpression(
                        MemberExpression {
                            variable_name: "b".to_string().into(),
                            property: None
                        }
                    )),
                    exp_type: BinaryExpType::Le,
                }
            ))),
        );
    }

    #[test]
    fn parse_add_arithmetic_expression_test() {
        let mut reader = TokenReader::new("true + false".as_bytes());
        assert_eq!(
            VariableExpression::parse(reader.next_token().unwrap(), &mut reader),
            Ok(VariableExpression::BinaryExpression(Box::new(
                BinaryExpression {
                    left: VariableExpression::VariableValue(VariableValue::Boolean(true)),
                    right: VariableExpression::VariableValue(VariableValue::Boolean(false)),
                    exp_type: BinaryExpType::Add,
                }
            ))),
        );

        let mut reader = TokenReader::new("false + a".as_bytes());
        assert_eq!(
            VariableExpression::parse(reader.next_token().unwrap(), &mut reader),
            Ok(VariableExpression::BinaryExpression(Box::new(
                BinaryExpression {
                    left: VariableExpression::VariableValue(VariableValue::Boolean(false)),
                    right: VariableExpression::VariableValue(VariableValue::MemberExpression(
                        MemberExpression {
                            variable_name: "a".to_string().into(),
                            property: None
                        }
                    )),
                    exp_type: BinaryExpType::Add,
                }
            ))),
        );

        let mut reader = TokenReader::new("a + b".as_bytes());
        assert_eq!(
            VariableExpression::parse(reader.next_token().unwrap(), &mut reader),
            Ok(VariableExpression::BinaryExpression(Box::new(
                BinaryExpression {
                    left: VariableExpression::VariableValue(VariableValue::MemberExpression(
                        MemberExpression {
                            variable_name: "a".to_string().into(),
                            property: None
                        }
                    )),
                    right: VariableExpression::VariableValue(VariableValue::MemberExpression(
                        MemberExpression {
                            variable_name: "b".to_string().into(),
                            property: None
                        }
                    )),
                    exp_type: BinaryExpType::Add,
                }
            ))),
        );
    }

    #[test]
    fn parse_sub_arithmetic_expression_test() {
        let mut reader = TokenReader::new("true - false".as_bytes());
        assert_eq!(
            VariableExpression::parse(reader.next_token().unwrap(), &mut reader),
            Ok(VariableExpression::BinaryExpression(Box::new(
                BinaryExpression {
                    left: VariableExpression::VariableValue(VariableValue::Boolean(true)),
                    right: VariableExpression::VariableValue(VariableValue::Boolean(false)),
                    exp_type: BinaryExpType::Sub,
                }
            ))),
        );

        let mut reader = TokenReader::new("false - a".as_bytes());
        assert_eq!(
            VariableExpression::parse(reader.next_token().unwrap(), &mut reader),
            Ok(VariableExpression::BinaryExpression(Box::new(
                BinaryExpression {
                    left: VariableExpression::VariableValue(VariableValue::Boolean(false)),
                    right: VariableExpression::VariableValue(VariableValue::MemberExpression(
                        MemberExpression {
                            variable_name: "a".to_string().into(),
                            property: None
                        }
                    )),
                    exp_type: BinaryExpType::Sub,
                }
            ))),
        );

        let mut reader = TokenReader::new("a - b".as_bytes());
        assert_eq!(
            VariableExpression::parse(reader.next_token().unwrap(), &mut reader),
            Ok(VariableExpression::BinaryExpression(Box::new(
                BinaryExpression {
                    left: VariableExpression::VariableValue(VariableValue::MemberExpression(
                        MemberExpression {
                            variable_name: "a".to_string().into(),
                            property: None
                        }
                    )),
                    right: VariableExpression::VariableValue(VariableValue::MemberExpression(
                        MemberExpression {
                            variable_name: "b".to_string().into(),
                            property: None
                        }
                    )),
                    exp_type: BinaryExpType::Sub,
                }
            ))),
        );
    }

    #[test]
    fn parse_div_arithmetic_expression_test() {
        let mut reader = TokenReader::new("true / false".as_bytes());
        assert_eq!(
            VariableExpression::parse(reader.next_token().unwrap(), &mut reader),
            Ok(VariableExpression::BinaryExpression(Box::new(
                BinaryExpression {
                    left: VariableExpression::VariableValue(VariableValue::Boolean(true)),
                    right: VariableExpression::VariableValue(VariableValue::Boolean(false)),
                    exp_type: BinaryExpType::Div,
                }
            ))),
        );

        let mut reader = TokenReader::new("false / a".as_bytes());
        assert_eq!(
            VariableExpression::parse(reader.next_token().unwrap(), &mut reader),
            Ok(VariableExpression::BinaryExpression(Box::new(
                BinaryExpression {
                    left: VariableExpression::VariableValue(VariableValue::Boolean(false)),
                    right: VariableExpression::VariableValue(VariableValue::MemberExpression(
                        MemberExpression {
                            variable_name: "a".to_string().into(),
                            property: None
                        }
                    )),
                    exp_type: BinaryExpType::Div,
                }
            ))),
        );

        let mut reader = TokenReader::new("a / b".as_bytes());
        assert_eq!(
            VariableExpression::parse(reader.next_token().unwrap(), &mut reader),
            Ok(VariableExpression::BinaryExpression(Box::new(
                BinaryExpression {
                    left: VariableExpression::VariableValue(VariableValue::MemberExpression(
                        MemberExpression {
                            variable_name: "a".to_string().into(),
                            property: None
                        }
                    )),
                    right: VariableExpression::VariableValue(VariableValue::MemberExpression(
                        MemberExpression {
                            variable_name: "b".to_string().into(),
                            property: None,
                        }
                    )),
                    exp_type: BinaryExpType::Div,
                }
            ))),
        );
    }

    #[test]
    fn parse_mul_arithmetic_expression_test() {
        let mut reader = TokenReader::new("true * false".as_bytes());
        assert_eq!(
            VariableExpression::parse(reader.next_token().unwrap(), &mut reader),
            Ok(VariableExpression::BinaryExpression(Box::new(
                BinaryExpression {
                    left: VariableExpression::VariableValue(VariableValue::Boolean(true)),
                    right: VariableExpression::VariableValue(VariableValue::Boolean(false)),
                    exp_type: BinaryExpType::Mul,
                }
            ))),
        );

        let mut reader = TokenReader::new("false * a".as_bytes());
        assert_eq!(
            VariableExpression::parse(reader.next_token().unwrap(), &mut reader),
            Ok(VariableExpression::BinaryExpression(Box::new(
                BinaryExpression {
                    left: VariableExpression::VariableValue(VariableValue::Boolean(false)),
                    right: VariableExpression::VariableValue(VariableValue::MemberExpression(
                        MemberExpression {
                            variable_name: "a".to_string().into(),
                            property: None
                        }
                    )),
                    exp_type: BinaryExpType::Mul,
                }
            ))),
        );

        let mut reader = TokenReader::new("a * b".as_bytes());
        assert_eq!(
            VariableExpression::parse(reader.next_token().unwrap(), &mut reader),
            Ok(VariableExpression::BinaryExpression(Box::new(
                BinaryExpression {
                    left: VariableExpression::VariableValue(VariableValue::MemberExpression(
                        MemberExpression {
                            variable_name: "a".to_string().into(),
                            property: None
                        }
                    )),
                    right: VariableExpression::VariableValue(VariableValue::MemberExpression(
                        MemberExpression {
                            variable_name: "b".to_string().into(),
                            property: None
                        }
                    )),
                    exp_type: BinaryExpType::Mul,
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
                        exp: VariableExpression::VariableValue(VariableValue::MemberExpression(
                            MemberExpression {
                                variable_name: "a".to_string().into(),
                                property: None
                            }
                        )),
                        exp_type: UnaryExpType::Not,
                    })),
                    right: VariableExpression::BinaryExpression(Box::new(BinaryExpression {
                        left: VariableExpression::VariableValue(VariableValue::MemberExpression(
                            MemberExpression {
                                variable_name: "b".to_string().into(),
                                property: None
                            }
                        )),
                        right: VariableExpression::UnaryExpression(Box::new(UnaryExpression {
                            exp: VariableExpression::VariableValue(
                                VariableValue::MemberExpression(MemberExpression {
                                    variable_name: "c".to_string().into(),
                                    property: None,
                                })
                            ),
                            exp_type: UnaryExpType::Not,
                        })),
                        exp_type: BinaryExpType::And,
                    })),
                    exp_type: BinaryExpType::Or,
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
                                exp: VariableExpression::VariableValue(
                                    VariableValue::MemberExpression(MemberExpression {
                                        variable_name: "a".to_string().into(),
                                        property: None
                                    })
                                ),
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
        let mut reader = TokenReader::new("(!a || (b && !c) && d && g)".as_bytes());
        assert_eq!(
            VariableExpression::parse(reader.next_token().unwrap(), &mut reader),
            Ok(VariableExpression::BinaryExpression(Box::new(
                BinaryExpression {
                    left: VariableExpression::UnaryExpression(Box::new(UnaryExpression {
                        exp: VariableExpression::VariableValue(VariableValue::MemberExpression(
                            MemberExpression {
                                variable_name: "a".to_string().into(),
                                property: None
                            }
                        )),
                        exp_type: UnaryExpType::Not,
                    })),
                    right: VariableExpression::BinaryExpression(Box::new(BinaryExpression {
                        left: VariableExpression::BinaryExpression(Box::new(BinaryExpression {
                            left: VariableExpression::BinaryExpression(Box::new(
                                BinaryExpression {
                                    left: VariableExpression::VariableValue(
                                        VariableValue::MemberExpression(MemberExpression {
                                            variable_name: "b".to_string().into(),
                                            property: None
                                        })
                                    ),
                                    right: VariableExpression::UnaryExpression(Box::new(
                                        UnaryExpression {
                                            exp: VariableExpression::VariableValue(
                                                VariableValue::MemberExpression(MemberExpression {
                                                    variable_name: "c".to_string().into(),
                                                    property: None
                                                })
                                            ),
                                            exp_type: UnaryExpType::Not,
                                        }
                                    )),
                                    exp_type: BinaryExpType::And,
                                }
                            )),
                            right: VariableExpression::VariableValue(
                                VariableValue::MemberExpression(MemberExpression {
                                    variable_name: "d".to_string().into(),
                                    property: None
                                })
                            ),
                            exp_type: BinaryExpType::And,
                        })),
                        right: VariableExpression::VariableValue(VariableValue::MemberExpression(
                            MemberExpression {
                                variable_name: "g".to_string().into(),
                                property: None
                            }
                        )),
                        exp_type: BinaryExpType::And,
                    })),
                    exp_type: BinaryExpType::Or,
                }
            )))
        );

        let mut reader = TokenReader::new("(!(a))".as_bytes());
        assert_eq!(
            VariableExpression::parse(reader.next_token().unwrap(), &mut reader),
            Ok(VariableExpression::UnaryExpression(Box::new(
                UnaryExpression {
                    exp: VariableExpression::VariableValue(VariableValue::MemberExpression(
                        MemberExpression {
                            variable_name: "a".to_string().into(),
                            property: None
                        }
                    )),
                    exp_type: UnaryExpType::Not
                }
            )))
        );
    }

    #[test]
    fn parse_function_call_test() {
        let mut reader = TokenReader::new(r#"foo(a, b, "val", 5)"#.as_bytes());
        assert_eq!(
            VariableExpression::parse(reader.next_token().unwrap(), &mut reader),
            Ok(VariableExpression::FunctionCall(FunctionCall {
                name: "foo".to_string().into(),
                args: vec![
                    VariableExpression::VariableValue(VariableValue::MemberExpression(
                        MemberExpression {
                            variable_name: "a".to_string().into(),
                            property: None
                        }
                    )),
                    VariableExpression::VariableValue(VariableValue::MemberExpression(
                        MemberExpression {
                            variable_name: "b".to_string().into(),
                            property: None
                        }
                    )),
                    VariableExpression::VariableValue(VariableValue::String("val".to_string())),
                    VariableExpression::VariableValue(VariableValue::Number(5_f64)),
                ]
            })),
        );
    }
}
