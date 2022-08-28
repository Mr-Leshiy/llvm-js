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
}

/// RPN - Reverse Polish Notation representation
pub struct RPN<V, UnaryOpType, BinaryOpType: Priority> {
    result: Vec<InputExpression<V, UnaryOpType, BinaryOpType>>,
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
        match &expr {
            InputExpression::Value(_) => self.result.push(expr),
            InputExpression::OpenBrace => self.stack.push(expr),
            InputExpression::CloseBrace => {
                let mut last = self.stack.pop();
                loop {
                    match last {
                        Some(InputExpression::OpenBrace) => break,
                        Some(value) => {
                            self.result.push(value);
                        }
                        None => return Err(Error::MissedOpenBrace),
                    }
                    last = self.stack.pop();
                }
            }
            InputExpression::Operation(op) => match op {
                Operation::PrefixOp(_) => self.stack.push(expr),
                Operation::PostfixOp(_) => self.result.push(expr),
                Operation::BinaryOp(op1) => {
                    let last = self.stack.pop();
                    if let Some(last) = last {
                        match &last {
                            InputExpression::Operation(Operation::BinaryOp(op2)) => {
                                match op2.priority().cmp(&op1.priority()) {
                                    Ordering::Equal => self.result.push(last),
                                    Ordering::Greater => self.result.push(last),
                                    Ordering::Less => self.stack.push(last),
                                }
                            }
                            InputExpression::Operation(Operation::PrefixOp(_)) => {
                                self.result.push(last)
                            }
                            _ => self.stack.push(last),
                        }
                    }
                    self.stack.push(expr);
                }
            },
        }
        Ok(())
    }

    pub fn finish(mut self) -> Self {
        while let Some(expr) = self.stack.pop() {
            self.result.push(expr);
        }
        self
    }

    pub fn evaluate(self) -> OutputExpression<V, UnaryOpType, BinaryOpType> {
        let mut stack = Vec::new();

        for expr in self.result {
            match expr {
                InputExpression::Value(value) => {
                    stack.push(OutputExpression::Value(value));
                }
                InputExpression::Operation(Operation::BinaryOp(op_type)) => {
                    let right = stack.pop().unwrap();
                    let left = stack.pop().unwrap();
                    stack.push(OutputExpression::BinaryExpression(Box::new(
                        BinaryExpression {
                            left,
                            right,
                            op_type,
                        },
                    )));
                }
                InputExpression::Operation(Operation::PostfixOp(op_type)) => {
                    let exp = stack.pop().unwrap();
                    stack.push(OutputExpression::UnaryExpression(Box::new(
                        UnaryExpression { exp, op_type },
                    )))
                }
                InputExpression::Operation(Operation::PrefixOp(op_type)) => {
                    let exp = stack.pop().unwrap();
                    stack.push(OutputExpression::UnaryExpression(Box::new(
                        UnaryExpression { exp, op_type },
                    )))
                }
                _ => panic!(),
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
        rpn.build(InputExpression::<i32, UnOp, BinOp>::Value(1))
            .unwrap();
        rpn.build(InputExpression::<i32, UnOp, BinOp>::Operation(
            Operation::BinaryOp(BinOp::Sum),
        ))
        .unwrap();
        rpn.build(InputExpression::<i32, UnOp, BinOp>::Operation(
            Operation::PrefixOp(UnOp::PrefixInc),
        ))
        .unwrap();
        rpn.build(InputExpression::<i32, UnOp, BinOp>::Value(2))
            .unwrap();
        rpn.build(InputExpression::<i32, UnOp, BinOp>::CloseBrace)
            .unwrap();
        rpn.build(InputExpression::<i32, UnOp, BinOp>::Operation(
            Operation::BinaryOp(BinOp::Mul),
        ))
        .unwrap();
        rpn.build(InputExpression::<i32, UnOp, BinOp>::Value(4))
            .unwrap();
        rpn.build(InputExpression::<i32, UnOp, BinOp>::Operation(
            Operation::PostfixOp(UnOp::PostfixInc),
        ))
        .unwrap();
        rpn.build(InputExpression::<i32, UnOp, BinOp>::Operation(
            Operation::BinaryOp(BinOp::Sub),
        ))
        .unwrap();
        rpn.build(InputExpression::<i32, UnOp, BinOp>::Value(3))
            .unwrap();

        rpn = rpn.finish();
        // 1 2 ++ + 4 ++ × 3 -
        assert_eq!(
            rpn.result,
            vec![
                InputExpression::<i32, UnOp, BinOp>::Value(1),
                InputExpression::<i32, UnOp, BinOp>::Value(2),
                InputExpression::<i32, UnOp, BinOp>::Operation(Operation::PrefixOp(
                    UnOp::PrefixInc
                )),
                InputExpression::<i32, UnOp, BinOp>::Operation(Operation::BinaryOp(BinOp::Sum)),
                InputExpression::<i32, UnOp, BinOp>::Value(4),
                InputExpression::<i32, UnOp, BinOp>::Operation(Operation::PostfixOp(
                    UnOp::PostfixInc
                )),
                InputExpression::<i32, UnOp, BinOp>::Operation(Operation::BinaryOp(BinOp::Mul)),
                InputExpression::<i32, UnOp, BinOp>::Value(3),
                InputExpression::<i32, UnOp, BinOp>::Operation(Operation::BinaryOp(BinOp::Sub))
            ]
        );
    }

    #[test]
    fn evaluate_test() {
        let mut rpn = RPN::new();
        // 1 2 ++ + 4 ++ × 3 -
        rpn.result = vec![
            InputExpression::<i32, UnOp, BinOp>::Value(1),
            InputExpression::<i32, UnOp, BinOp>::Value(2),
            InputExpression::<i32, UnOp, BinOp>::Operation(Operation::PrefixOp(UnOp::PrefixInc)),
            InputExpression::<i32, UnOp, BinOp>::Operation(Operation::BinaryOp(BinOp::Sum)),
            InputExpression::<i32, UnOp, BinOp>::Value(4),
            InputExpression::<i32, UnOp, BinOp>::Operation(Operation::PostfixOp(UnOp::PostfixInc)),
            InputExpression::<i32, UnOp, BinOp>::Operation(Operation::BinaryOp(BinOp::Mul)),
            InputExpression::<i32, UnOp, BinOp>::Value(3),
            InputExpression::<i32, UnOp, BinOp>::Operation(Operation::BinaryOp(BinOp::Sub)),
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
                            op_type: UnOp::PrefixInc
                        })),
                        op_type: BinOp::Sum,
                    })),
                    right: OutputExpression::UnaryExpression(Box::new(UnaryExpression {
                        exp: OutputExpression::Value(4),
                        op_type: UnOp::PostfixInc
                    })),
                    op_type: BinOp::Mul
                })),
                right: OutputExpression::Value(3),
                op_type: BinOp::Sub,
            }))
        );
    }
}
