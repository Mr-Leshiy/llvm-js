use crate::{js_ast::Identifier, llvm_ast::FunctionDeclaration};
use collections::set::Set;
use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum Error {
    #[error("Undefined variable identifier {0}")]
    UndefinedVariable(Identifier),
    #[error("Variable with this identifier {0} already declared")]
    AlreadyDeclaredVariable(Identifier),
    #[error("Function with this identifier {0} already declared")]
    AlreadyDeclaredFunction(Identifier),
    #[error("Undefined function identifier {0}")]
    UndefinedFunction(Identifier),
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
    pub fn new<Iter>(predefined_functions: Iter) -> Self
    where
        Iter: Iterator<Item = Identifier>,
    {
        Self {
            variables: Set::new(),
            functions: predefined_functions.collect(),

            function_declarations: Vec::new(),
        }
    }
}
