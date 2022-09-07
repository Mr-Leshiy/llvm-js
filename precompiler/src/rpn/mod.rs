use self::input::Value;
use input::{InputExpression, Operation, Priority};
use output::{BinaryExpression, OutputExpression, UnaryExpression};
use std::cmp::Ordering;
use thiserror::Error;

pub mod input;
pub mod output;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum Error {
    #[error("Cannot find corresponding open brace")]
    MissedOpenBrace,
    #[error("Not a operation")]
    NotOperation,
}

/// RPN - Reverse Polish Notation representation
pub struct RPN<V, UnaryOpType, BinaryOpType: Priority> {
    result: Vec<Value<V, UnaryOpType, BinaryOpType>>,
    stack: Vec<InputExpression<V, UnaryOpType, BinaryOpType>>,
}

impl<V, UnaryOpType, BinaryOpType: Priority> Default for RPN<V, UnaryOpType, BinaryOpType> {
    fn default() -> Self {
        Self::new()
    }
}

impl<V, UnaryOpType, BinaryOpType: Priority> RPN<V, UnaryOpType, BinaryOpType> {
    pub fn new() -> Self {
        Self {
            result: Vec::new(),
            stack: Vec::new(),
        }
    }

    /// transform expression from infix notation to Reverse Polish Notation
    pub fn build(
        &mut self,
        expr: InputExpression<V, UnaryOpType, BinaryOpType>,
    ) -> Result<(), Error> {
        match expr {
            InputExpression::Value(value) => match value {
                Value::Value(value) => self.result.push(Value::Value(value)),
                Value::Operation(op) => match op {
                    Operation::PrefixOp(op) => self.stack.push(InputExpression::Value(
                        Value::Operation(Operation::PrefixOp(op)),
                    )),
                    Operation::PostfixOp(op) => {
                        self.result.push(Value::Operation(Operation::PostfixOp(op)))
                    }
                    Operation::BinaryOp(op1) => {
                        let last = self.stack.pop();
                        if let Some(last) = last {
                            match last {
                                InputExpression::Value(Value::Operation(Operation::BinaryOp(
                                    op2,
                                ))) => match op2.priority().cmp(&op1.priority()) {
                                    Ordering::Equal => {
                                        self.result.push(Value::Operation(Operation::BinaryOp(op2)))
                                    }
                                    Ordering::Greater => {
                                        self.result.push(Value::Operation(Operation::BinaryOp(op2)))
                                    }
                                    Ordering::Less => self.stack.push(InputExpression::Value(
                                        Value::Operation(Operation::BinaryOp(op2)),
                                    )),
                                },
                                InputExpression::Value(Value::Operation(Operation::PrefixOp(
                                    op2,
                                ))) => self.result.push(Value::Operation(Operation::PrefixOp(op2))),
                                last => self.stack.push(last),
                            }
                        }
                        self.stack.push(InputExpression::Value(Value::Operation(
                            Operation::BinaryOp(op1),
                        )));
                    }
                },
            },
            InputExpression::OpenBrace => self.stack.push(InputExpression::OpenBrace),
            InputExpression::CloseBrace => {
                let mut last = self.stack.pop();
                loop {
                    match last {
                        Some(InputExpression::OpenBrace) => break,
                        Some(InputExpression::Value(value)) => {
                            self.result.push(value);
                        }
                        _ => return Err(Error::MissedOpenBrace),
                    }
                    last = self.stack.pop();
                }
            }
        }
        Ok(())
    }

    pub fn finish(mut self) -> Result<Self, Error> {
        while let Some(expr) = self.stack.pop() {
            match expr {
                InputExpression::Value(Value::Operation(op)) => {
                    self.result.push(Value::Operation(op))
                }
                _ => return Err(Error::NotOperation),
            }
        }
        Ok(self)
    }

    pub fn evaluate(self) -> OutputExpression<V, UnaryOpType, BinaryOpType> {
        let mut stack = Vec::new();
        for expr in self.result {
            match expr {
                Value::Value(value) => {
                    stack.push(OutputExpression::Value(value));
                }
                Value::Operation(Operation::BinaryOp(exp_type)) => {
                    let right = stack.pop().unwrap();
                    let left = stack.pop().unwrap();
                    stack.push(OutputExpression::BinaryExpression(Box::new(
                        BinaryExpression {
                            left,
                            right,
                            exp_type,
                        },
                    )));
                }
                Value::Operation(Operation::PostfixOp(exp_type)) => {
                    let exp = stack.pop().unwrap();
                    stack.push(OutputExpression::UnaryExpression(Box::new(
                        UnaryExpression { exp, exp_type },
                    )))
                }
                Value::Operation(Operation::PrefixOp(exp_type)) => {
                    let exp = stack.pop().unwrap();
                    stack.push(OutputExpression::UnaryExpression(Box::new(
                        UnaryExpression { exp, exp_type },
                    )))
                }
            }
        }
        stack.pop().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq)]
    enum BinOp {
        // +
        Sum,
        // -
        Sub,
        // *
        Mul,
    }

    #[derive(Debug, PartialEq)]
    enum UnOp {
        // x++
        PostfixInc,
        // --x
        PrefixInc,
    }

    impl Priority for BinOp {
        fn priority(&self) -> u8 {
            match self {
                Self::Sum => 0,
                Self::Sub => 0,
                Self::Mul => 1,
            }
        }
    }

    #[test]
    fn infix_to_rpn_test() {
        // (1 + ++2) * 4++ - 3
        let mut rpn = RPN::new();
        rpn.build(InputExpression::<i32, UnOp, BinOp>::OpenBrace)
            .unwrap();
        rpn.build(InputExpression::<i32, UnOp, BinOp>::Value(Value::Value(1)))
            .unwrap();
        rpn.build(InputExpression::<i32, UnOp, BinOp>::Value(
            Value::Operation(Operation::BinaryOp(BinOp::Sum)),
        ))
        .unwrap();
        rpn.build(InputExpression::<i32, UnOp, BinOp>::Value(
            Value::Operation(Operation::PrefixOp(UnOp::PrefixInc)),
        ))
        .unwrap();
        rpn.build(InputExpression::<i32, UnOp, BinOp>::Value(Value::Value(2)))
            .unwrap();
        rpn.build(InputExpression::<i32, UnOp, BinOp>::CloseBrace)
            .unwrap();
        rpn.build(InputExpression::<i32, UnOp, BinOp>::Value(
            Value::Operation(Operation::BinaryOp(BinOp::Mul)),
        ))
        .unwrap();
        rpn.build(InputExpression::<i32, UnOp, BinOp>::Value(Value::Value(4)))
            .unwrap();
        rpn.build(InputExpression::<i32, UnOp, BinOp>::Value(
            Value::Operation(Operation::PostfixOp(UnOp::PostfixInc)),
        ))
        .unwrap();
        rpn.build(InputExpression::<i32, UnOp, BinOp>::Value(
            Value::Operation(Operation::BinaryOp(BinOp::Sub)),
        ))
        .unwrap();
        rpn.build(InputExpression::<i32, UnOp, BinOp>::Value(Value::Value(3)))
            .unwrap();

        rpn = rpn.finish().unwrap();
        // 1 2 ++ + 4 ++ × 3 -
        assert_eq!(
            rpn.result,
            vec![
                Value::<i32, UnOp, BinOp>::Value(1),
                Value::<i32, UnOp, BinOp>::Value(2),
                Value::<i32, UnOp, BinOp>::Operation(Operation::PrefixOp(UnOp::PrefixInc)),
                Value::<i32, UnOp, BinOp>::Operation(Operation::BinaryOp(BinOp::Sum)),
                Value::<i32, UnOp, BinOp>::Value(4),
                Value::<i32, UnOp, BinOp>::Operation(Operation::PostfixOp(UnOp::PostfixInc)),
                Value::<i32, UnOp, BinOp>::Operation(Operation::BinaryOp(BinOp::Mul)),
                Value::<i32, UnOp, BinOp>::Value(3),
                Value::<i32, UnOp, BinOp>::Operation(Operation::BinaryOp(BinOp::Sub))
            ]
        );
    }

    #[test]
    fn evaluate_test() {
        let mut rpn = RPN::new();
        // 1 2 ++ + 4 ++ × 3 -
        rpn.result = vec![
            Value::<i32, UnOp, BinOp>::Value(1),
            Value::<i32, UnOp, BinOp>::Value(2),
            Value::<i32, UnOp, BinOp>::Operation(Operation::PrefixOp(UnOp::PrefixInc)),
            Value::<i32, UnOp, BinOp>::Operation(Operation::BinaryOp(BinOp::Sum)),
            Value::<i32, UnOp, BinOp>::Value(4),
            Value::<i32, UnOp, BinOp>::Operation(Operation::PostfixOp(UnOp::PostfixInc)),
            Value::<i32, UnOp, BinOp>::Operation(Operation::BinaryOp(BinOp::Mul)),
            Value::<i32, UnOp, BinOp>::Value(3),
            Value::<i32, UnOp, BinOp>::Operation(Operation::BinaryOp(BinOp::Sub)),
        ];

        let result = rpn.evaluate();

        assert_eq!(
            result,
            OutputExpression::BinaryExpression(Box::new(BinaryExpression {
                left: OutputExpression::BinaryExpression(Box::new(BinaryExpression {
                    left: OutputExpression::BinaryExpression(Box::new(BinaryExpression {
                        left: OutputExpression::Value(1),
                        right: OutputExpression::UnaryExpression(Box::new(UnaryExpression {
                            exp: OutputExpression::Value(2),
                            exp_type: UnOp::PrefixInc
                        })),
                        exp_type: BinOp::Sum,
                    })),
                    right: OutputExpression::UnaryExpression(Box::new(UnaryExpression {
                        exp: OutputExpression::Value(4),
                        exp_type: UnOp::PostfixInc
                    })),
                    exp_type: BinOp::Mul
                })),
                right: OutputExpression::Value(3),
                exp_type: BinOp::Sub,
            }))
        );
    }
}
