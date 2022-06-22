#[derive(Debug, Clone, PartialEq)]
pub enum VariableValue {
    FloatNumber(f64),
    String(String),
}

pub type VariableName = String;

#[derive(Debug, Clone, PartialEq)]
pub struct VariableDeclaration {
    pub name: VariableName,
    pub value: VariableValue,
}
