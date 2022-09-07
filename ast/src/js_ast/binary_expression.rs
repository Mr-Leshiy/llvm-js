use super::{Identifier, VariableExpression};
use crate::llvm_ast;
use precompiler::{rpn::input::Priority, Precompiler};

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
    SEq,
    SNe,
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
            Self::SEq => llvm_ast::BinaryExpType::SEq,
            Self::SNe => llvm_ast::BinaryExpType::SNe,
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
            BinaryExpType::Eq => 8,
            BinaryExpType::Ne => 8,
            BinaryExpType::SEq => 8,
            BinaryExpType::SNe => 8,
            BinaryExpType::And => 4,
            BinaryExpType::Or => 3,
            // Arithmetic
            BinaryExpType::Add => 11,
            BinaryExpType::Sub => 11,
            BinaryExpType::Div => 12,
            BinaryExpType::Mul => 12,
        }
    }
}

impl BinaryExpression {
    pub fn precompile(
        self,
        precompiler: &mut Precompiler<Identifier, llvm_ast::FunctionDeclaration>,
    ) -> Result<llvm_ast::BinaryExpression, precompiler::Error<Identifier>> {
        Ok(llvm_ast::BinaryExpression {
            left: self.left.precompile(precompiler)?,
            right: self.right.precompile(precompiler)?,
            exp_type: self.exp_type.into(),
        })
    }
}
