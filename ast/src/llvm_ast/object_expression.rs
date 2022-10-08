use super::{Identifier, VariableExpression};
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub struct ObjectExpression {
    pub properties: HashMap<Identifier, VariableExpression>,
}
