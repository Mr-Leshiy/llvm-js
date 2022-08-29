#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BinaryExpression<V, UnaryOpType, BinaryOpType> {
    pub left: OutputExpression<V, UnaryOpType, BinaryOpType>,
    pub right: OutputExpression<V, UnaryOpType, BinaryOpType>,
    pub op_type: BinaryOpType,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UnaryExpression<V, UnaryOpType, BinaryOpType> {
    pub exp: OutputExpression<V, UnaryOpType, BinaryOpType>,
    pub op_type: UnaryOpType,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum OutputExpression<V, UnaryOpType, BinaryOpType> {
    Value(V),
    BinaryExpression(Box<BinaryExpression<V, UnaryOpType, BinaryOpType>>),
    UnaryExpression(Box<UnaryExpression<V, UnaryOpType, BinaryOpType>>),
}
