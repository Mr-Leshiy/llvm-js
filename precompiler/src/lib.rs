use collections::set::Set;
use std::{
    fmt::{Debug, Display},
    hash::Hash,
};
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum Error<T> {
    #[error("Undefined variable identifier {0}")]
    UndefinedVariable(T),
    #[error("Undefined function identifier {0}")]
    UndefinedFunction(T),
}

/// Precompiler - validate the exisitng AST tree, prepare data for the compiler
pub struct Precompiler<T1, T2>
where
    T1: Clone + Hash + PartialEq + Eq + Display,
{
    pub variables: Set<T1>,
    pub functions: Set<T1>,

    pub function_declarations: Vec<T2>,
}

impl<T1, T2> Precompiler<T1, T2>
where
    T1: Clone + Hash + PartialEq + Eq + Display + Debug,
{
    pub fn new<Iter>(predefined_functions: Iter) -> Self
    where
        Iter: Iterator<Item = T1>,
    {
        Self {
            variables: Set::new(),
            functions: predefined_functions.collect(),

            function_declarations: Vec::new(),
        }
    }
}
