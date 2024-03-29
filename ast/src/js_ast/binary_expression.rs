use super::VariableExpression;
use crate::{llvm_ast, Precompiler, PrecompilerError};
use rpn::input::Priority;

#[derive(Clone, Debug, PartialEq)]
pub struct BinaryExpression {
    pub left: VariableExpression,
    pub right: VariableExpression,
    pub exp_type: BinaryExpType,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BinaryExpType {
    // Logical
    And,
    Or,
    Eq,
    Ne,
    Gt,
    Ge,
    Lt,
    Le,
    // Arithmetic
    Add,
    Sub,
    Div,
    Mul,
}

#[allow(clippy::from_over_into)]
impl Into<llvm_ast::BinaryExpType> for BinaryExpType {
    fn into(self) -> llvm_ast::BinaryExpType {
        match self {
            // Logical
            Self::And => llvm_ast::BinaryExpType::And,
            Self::Or => llvm_ast::BinaryExpType::Or,
            Self::Eq => llvm_ast::BinaryExpType::Eq,
            Self::Ne => llvm_ast::BinaryExpType::Ne,
            Self::Gt => llvm_ast::BinaryExpType::Gt,
            Self::Ge => llvm_ast::BinaryExpType::Ge,
            Self::Lt => llvm_ast::BinaryExpType::Lt,
            Self::Le => llvm_ast::BinaryExpType::Le,
            // Arithmetic
            Self::Add => llvm_ast::BinaryExpType::Add,
            Self::Sub => llvm_ast::BinaryExpType::Sub,
            Self::Div => llvm_ast::BinaryExpType::Div,
            Self::Mul => llvm_ast::BinaryExpType::Mul,
        }
    }
}

impl Priority for BinaryExpType {
    fn priority(&self) -> u8 {
        match self {
            // Logical
            BinaryExpType::Eq | BinaryExpType::Ne => 8,
            BinaryExpType::Gt | BinaryExpType::Ge | BinaryExpType::Lt | BinaryExpType::Le => 9,
            BinaryExpType::And => 4,
            BinaryExpType::Or => 3,
            // Arithmetic
            BinaryExpType::Add | BinaryExpType::Sub => 11,
            BinaryExpType::Div | BinaryExpType::Mul => 12,
        }
    }
}

impl BinaryExpression {
    pub fn precompile(
        self,
        precompiler: &mut Precompiler,
    ) -> Result<llvm_ast::BinaryExpression, PrecompilerError> {
        Ok(llvm_ast::BinaryExpression {
            left: self.left.precompile(precompiler)?,
            right: self.right.precompile(precompiler)?,
            exp_type: self.exp_type.into(),
        })
    }
}
