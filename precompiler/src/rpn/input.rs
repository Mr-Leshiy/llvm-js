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
pub enum InputExpression<V, UnaryOpType, BinaryOpType: Priority> {
    Value(V),
    Operation(Operation<UnaryOpType, BinaryOpType>),
    OpenBrace,
    CloseBrace,
}
