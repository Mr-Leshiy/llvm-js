use std::cmp::Ordering;

#[derive(Clone, Debug, PartialEq, Eq)]
enum Operation<T1, T2, T3: Ord> {
    // e.g. !x - factorial
    PostfixFunction(T1),
    // e.g. sin(x), cos(x)
    PrefixFunction(T2),
    BinaryOp(T3),
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Expression<V, T1, T2, T3: Ord> {
    Value(V),
    Operation(Operation<T1, T2, T3>),
    OpenBrace,
    CloseBrace,
}

/// RPN - Reverse Polish Notation representation
struct RPN<V, T1, T2, T3: Ord> {
    result: Vec<Expression<V, T1, T2, T3>>,
    stack: Vec<Expression<V, T1, T2, T3>>,
}

impl<V, T1, T2, T3: Ord> RPN<V, T1, T2, T3> {
    fn new() -> Self {
        Self {
            result: Vec::new(),
            stack: Vec::new(),
        }
    }

    /// transform expression from infix notation to Reverse Polish Notation
    fn transform_from_infix(&mut self, expr: Expression<V, T1, T2, T3>) {
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
                Operation::PostfixFunction(_) => self.result.push(expr),
                Operation::PrefixFunction(_) => self.stack.push(expr),
                Operation::BinaryOp(op1) => {
                    let last = self.stack.pop();
                    if let Some(last) = last {
                        match &last {
                            Expression::Operation(Operation::PrefixFunction(_)) => {
                                self.result.push(last)
                            }
                            Expression::Operation(Operation::BinaryOp(op2)) => {
                                match op2.cmp(&op1) {
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

    fn finish(mut self) -> Self {
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

    #[derive(Debug, PartialEq, Eq, PartialOrd)]
    enum BinOp {
        // +
        Sum,
        // -
        Div,
        // *
        Mul,
    }

    impl Ord for BinOp {
        fn cmp(&self, other: &Self) -> Ordering {
            match (self, other) {
                // Sum
                (BinOp::Sum, BinOp::Sum) => Ordering::Equal,
                (BinOp::Sum, BinOp::Div) => Ordering::Equal,
                (BinOp::Sum, BinOp::Mul) => Ordering::Less,
                // Div
                (BinOp::Div, BinOp::Sum) => Ordering::Equal,
                (BinOp::Div, BinOp::Div) => Ordering::Equal,
                (BinOp::Div, BinOp::Mul) => Ordering::Less,
                // Mul
                (BinOp::Mul, BinOp::Sum) => Ordering::Greater,
                (BinOp::Mul, BinOp::Div) => Ordering::Greater,
                (BinOp::Mul, BinOp::Mul) => Ordering::Equal,
            }
        }
    }

    #[test]
    fn infix_to_rpn_test() {
        let mut rpn = RPN::new();

        // (1 + 2) * 4 + 3
        rpn.transform_from_infix(Expression::<i32, (), (), BinOp>::OpenBrace);
        rpn.transform_from_infix(Expression::<i32, (), (), BinOp>::Value(1));
        rpn.transform_from_infix(Expression::<i32, (), (), BinOp>::Operation(
            Operation::BinaryOp(BinOp::Sum),
        ));
        rpn.transform_from_infix(Expression::<i32, (), (), BinOp>::Value(2));
        rpn.transform_from_infix(Expression::<i32, (), (), BinOp>::CloseBrace);
        rpn.transform_from_infix(Expression::<i32, (), (), BinOp>::Operation(
            Operation::BinaryOp(BinOp::Mul),
        ));
        rpn.transform_from_infix(Expression::<i32, (), (), BinOp>::Value(4));
        rpn.transform_from_infix(Expression::<i32, (), (), BinOp>::Operation(
            Operation::BinaryOp(BinOp::Sum),
        ));
        rpn.transform_from_infix(Expression::<i32, (), (), BinOp>::Value(3));

        rpn = rpn.finish();

        // 1 2 + 4 × 3 +
        assert_eq!(
            rpn.result,
            vec![
                Expression::<i32, (), (), BinOp>::Value(1),
                Expression::<i32, (), (), BinOp>::Value(2),
                Expression::<i32, (), (), BinOp>::Operation(Operation::BinaryOp(BinOp::Sum)),
                Expression::<i32, (), (), BinOp>::Value(4),
                Expression::<i32, (), (), BinOp>::Operation(Operation::BinaryOp(BinOp::Mul)),
                Expression::<i32, (), (), BinOp>::Value(3),
                Expression::<i32, (), (), BinOp>::Operation(Operation::BinaryOp(BinOp::Sum))
            ]
        );
    }
}
