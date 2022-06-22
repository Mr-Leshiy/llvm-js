use crate::{
    llvm_ast::{VariableName, VariableValue},
    map::Map,
};
use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum Error {
    #[error("Undefined variable identifier {0}")]
    UndefinedVariable(VariableName),
    #[error("Variable with this identifier {0} already declared")]
    AlreadyDeclaredVariable(VariableName),
    // #[error("Function with this identifier {0} already declared")]
    // AlreadyDeclaredFunction(Identifier),
}

pub trait Precompile: Sized {
    type Output;
    fn precompile(self, precompiler: &mut Precompiler) -> Result<Self::Output, Error>;
}

/// Precompiler - validate the exisitng AST tree, prepare data for the compiler
pub struct Precompiler {
    pub variables: Map<VariableName, VariableValue>,
}

impl Precompiler {
    pub fn new() -> Self {
        Self {
            variables: Map::new(),
        }
    }
}
