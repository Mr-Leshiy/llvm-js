use super::{Identifier, VariableExpression};
use crate::llvm_ast;
use precompiler::Precompiler;

#[derive(Clone, Debug, PartialEq)]
pub struct UnaryExpression {
    pub exp: VariableExpression,
    pub exp_type: UnaryExpType,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum UnaryExpType {
    Not,
}

impl Into<llvm_ast::UnaryExpType> for UnaryExpType {
    fn into(self) -> llvm_ast::UnaryExpType {
        match self {
            Self::Not => llvm_ast::UnaryExpType::Not,
        }
    }
}

impl UnaryExpression {
    pub fn precompile(
        self,
        precompiler: &mut Precompiler<Identifier, llvm_ast::FunctionDeclaration>,
    ) -> Result<llvm_ast::UnaryExpression, precompiler::Error<Identifier>> {
        Ok(llvm_ast::UnaryExpression {
            exp: self.exp.precompile(precompiler)?,
            exp_type: self.exp_type.into(),
        })
    }
}
