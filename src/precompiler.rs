use crate::{js_ast::Identifier, llvm_ast::FunctionDeclaration, set::Set};
use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum Error {
    #[error("Undefined variable identifier {0}")]
    UndefinedVariable(Identifier),
    #[error("Variable with this identifier {0} already declared")]
    AlreadyDeclaredVariable(Identifier),
    #[error("Function with this identifier {0} already declared")]
    AlreadyDeclaredFunction(Identifier),
}

pub trait Precompile: Sized {
    type Output;
    fn precompile(self, precompiler: &mut Precompiler) -> Result<Self::Output, Error>;
}

/// Precompiler - validate the exisitng AST tree, prepare data for the compiler
pub struct Precompiler {
    pub variables: Set<Identifier>,
    pub functions: Set<Identifier>,

    pub function_declarations: Vec<FunctionDeclaration>,
}

impl Precompiler {
    pub fn new() -> Self {
        Self {
            variables: Set::new(),
            functions: Set::new(),

            function_declarations: Vec::new(),
        }
    }
}
