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
    And,
    Or,
    Eq,
    Ne,
    SEq,
    SNe,
}

#[allow(clippy::from_over_into)]
impl Into<llvm_ast::BinaryExpType> for BinaryExpType {
    fn into(self) -> llvm_ast::BinaryExpType {
        match self {
            Self::And => llvm_ast::BinaryExpType::And,
            Self::Or => llvm_ast::BinaryExpType::Or,
            Self::Eq => llvm_ast::BinaryExpType::Eq,
            Self::Ne => llvm_ast::BinaryExpType::Ne,
            Self::SEq => llvm_ast::BinaryExpType::SEq,
            Self::SNe => llvm_ast::BinaryExpType::SNe,
        }
    }
}

impl Priority for BinaryExpType {
    fn priority(&self) -> u8 {
        match self {
            BinaryExpType::Eq => 8,
            BinaryExpType::Ne => 8,
            BinaryExpType::SEq => 8,
            BinaryExpType::SNe => 8,
            BinaryExpType::And => 4,
            BinaryExpType::Or => 3,
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
