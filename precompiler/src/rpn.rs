use std::cmp::Ordering;

pub trait Priority {
    fn priority(&self) -> u8;
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Operation<UnaryOpType, BinaryOpType: Priority> {
    // e.g. !x - factorial, ++x
    PrefixOp(UnaryOpType),
    // e.g. x--
    PostfixOp(UnaryOpType),
    BinaryOp(BinaryOpType),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Expression<V, UnaryOpType, BinaryOpType: Priority> {
    Value(V),
    Operation(Operation<UnaryOpType, BinaryOpType>),
    OpenBrace,
    CloseBrace,
}

/// RPN - Reverse Polish Notation representation
pub struct RPN<V, UnaryOpType, BinaryOpType: Priority> {
    result: Vec<Expression<V, UnaryOpType, BinaryOpType>>,
    stack: Vec<Expression<V, UnaryOpType, BinaryOpType>>,
}

impl<V, UnaryOpType, BinaryOpType: Priority> RPN<V, UnaryOpType, BinaryOpType> {
    pub fn new() -> Self {
        Self {
            result: Vec::new(),
            stack: Vec::new(),
        }
    }

    /// transform expression from infix notation to Reverse Polish Notation
    pub fn transform_from_infix(&mut self, expr: Expression<V, UnaryOpType, BinaryOpType>) {
        match &expr {
            Expression::Value(_) => self.result.push(expr),
            Expression::OpenBrace => self.stack.push(expr),
            Expression::CloseBrace => {
                let mut last = self.stack.pop();
                loop {
                    match last {
                        Some(Expression::OpenBrace) => break,
                        Some(value) => {
                            self.result.push(value);
                        }
                        None => panic!("can not find corresponding OpenBrace"),
                    }
                    last = self.stack.pop();
                }
            }
            Expression::Operation(op) => match op {
                Operation::PrefixOp(_) => self.stack.push(expr),
                Operation::PostfixOp(_) => self.result.push(expr),
                Operation::BinaryOp(op1) => {
                    let last = self.stack.pop();
                    if let Some(last) = last {
                        match &last {
                            Expression::Operation(Operation::BinaryOp(op2)) => {
                                match op2.priority().cmp(&op1.priority()) {
                                    Ordering::Equal => self.result.push(last),
                                    Ordering::Greater => self.result.push(last),
                                    Ordering::Less => self.stack.push(last),
                                }
                            }
                            _ => self.stack.push(last),
                        }
                    }
                    self.stack.push(expr);
                }
            },
        }
    }

    pub fn finish(mut self) -> Self {
        let mut last = self.stack.pop();
        loop {
            match last {
                Some(expr) => self.result.push(expr),
                None => break,
            }
            last = self.stack.pop();
        }
        self
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
        Div,
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
                Self::Div => 0,
                Self::Mul => 1,
            }
        }
    }

    #[test]
    fn infix_to_rpn_test() {
        let mut rpn = RPN::new();

        // (1 + ++2) * 4++ - 3
        rpn.transform_from_infix(Expression::<i32, UnOp, BinOp>::OpenBrace);
        rpn.transform_from_infix(Expression::<i32, UnOp, BinOp>::Value(1));
        rpn.transform_from_infix(Expression::<i32, UnOp, BinOp>::Operation(
            Operation::BinaryOp(BinOp::Sum),
        ));
        rpn.transform_from_infix(Expression::<i32, UnOp, BinOp>::Operation(
            Operation::PrefixOp(UnOp::PrefixInc),
        ));
        rpn.transform_from_infix(Expression::<i32, UnOp, BinOp>::Value(2));
        rpn.transform_from_infix(Expression::<i32, UnOp, BinOp>::CloseBrace);
        rpn.transform_from_infix(Expression::<i32, UnOp, BinOp>::Operation(
            Operation::BinaryOp(BinOp::Mul),
        ));
        rpn.transform_from_infix(Expression::<i32, UnOp, BinOp>::Value(4));
        rpn.transform_from_infix(Expression::<i32, UnOp, BinOp>::Operation(
            Operation::PostfixOp(UnOp::PostfixInc),
        ));
        rpn.transform_from_infix(Expression::<i32, UnOp, BinOp>::Operation(
            Operation::BinaryOp(BinOp::Div),
        ));
        rpn.transform_from_infix(Expression::<i32, UnOp, BinOp>::Value(3));

        rpn = rpn.finish();

        // 1 2 ++ + 4 ++ × 3 +
        assert_eq!(
            rpn.result,
            vec![
                Expression::<i32, UnOp, BinOp>::Value(1),
                Expression::<i32, UnOp, BinOp>::Value(2),
                Expression::<i32, UnOp, BinOp>::Operation(Operation::PrefixOp(UnOp::PrefixInc)),
                Expression::<i32, UnOp, BinOp>::Operation(Operation::BinaryOp(BinOp::Sum)),
                Expression::<i32, UnOp, BinOp>::Value(4),
                Expression::<i32, UnOp, BinOp>::Operation(Operation::PostfixOp(UnOp::PostfixInc)),
                Expression::<i32, UnOp, BinOp>::Operation(Operation::BinaryOp(BinOp::Mul)),
                Expression::<i32, UnOp, BinOp>::Value(3),
                Expression::<i32, UnOp, BinOp>::Operation(Operation::BinaryOp(BinOp::Div))
            ]
        );
    }
}
