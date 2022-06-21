use crate::ast::Identifier;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    // #[error("Undefined variable identifier {0}")]
    // UndefinedVariable(Identifier),
    // #[error("Variable with this identifier {0} already declared")]
    // AlreadyDeclaredVariable(Identifier),
    // #[error("Function with this identifier {0} already declared")]
    // AlreadyDeclaredFunction(Identifier),
}

pub trait Precompile: Sized {
    fn precompile(&self, precompiler: &mut Precompiler) -> Result<(), Error>;
}

/// Precompiler - validate the exisitng AST tree, prepare data for the compiler
pub struct Precompiler {
    pub variables: Vec<Identifier>,
    pub functions: Vec<Identifier>,
}

impl Precompiler {
    pub fn new() -> Self {
        Self {
            variables: Vec::new(),
            functions: Vec::new(),
        }
    }
}
