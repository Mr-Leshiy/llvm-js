#![warn(clippy::pedantic)]
#![allow(
    clippy::must_use_candidate,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::module_name_repetitions
)]

use collections::set::Set;
use std::{
    fmt::{Debug, Display},
    hash::Hash,
};

#[derive(thiserror::Error, Debug, PartialEq, Eq)]
pub enum Error<T> {
    #[error("Undefined variable identifier {0}")]
    UndefinedVariable(T),
}

/// Precompiler - validate the exisitng AST tree, prepare data for the compiler
#[derive(Debug)]
pub struct Precompiler<T1, T2>
where
    T1: Clone + Hash + PartialEq + Eq + Display,
{
    variables: Set<T1>,

    function_declarations: Vec<T2>,
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
            variables: predefined_functions.collect(),
            function_declarations: Vec::new(),
        }
    }

    pub fn insert_variable(&mut self, variable: T1) -> u32 {
        self.variables.insert(variable)
    }

    pub fn get_variable(&mut self, variable: T1) -> Result<u32, Error<T1>> {
        self.variables
            .get(&variable)
            .ok_or(Error::UndefinedVariable(variable))
    }

    pub fn remove_last_added_variables(&mut self, size: usize) -> Vec<(T1, u32)> {
        self.variables.remove_last_added(size)
    }

    pub fn variables_len(&self) -> usize {
        self.variables.len()
    }

    pub fn insert_function_declaration(&mut self, function_declaration: T2) {
        self.function_declarations.push(function_declaration);
    }

    pub fn get_function_declarations(self) -> Vec<T2> {
        self.function_declarations
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn precompiler_variables_test() {
        let mut precompiler = Precompiler::<String, String>::new(std::iter::empty());

        assert_eq!(precompiler.variables_len(), 0);
        assert_eq!(precompiler.insert_variable("var1".to_string()), 0);
        assert_eq!(precompiler.insert_variable("var1".to_string()), 1);
        assert_eq!(precompiler.insert_variable("var2".to_string()), 0);
        assert_eq!(precompiler.variables_len(), 3);

        assert_eq!(precompiler.get_variable("var1".to_string()), Ok(1));
        assert_eq!(precompiler.get_variable("var2".to_string()), Ok(0));
        assert_eq!(
            precompiler.get_variable("var3".to_string()),
            Err(Error::UndefinedVariable("var3".to_string()))
        );

        assert_eq!(
            precompiler.remove_last_added_variables(3),
            vec![
                ("var2".to_string(), 0),
                ("var1".to_string(), 1),
                ("var1".to_string(), 0)
            ]
        );
        assert_eq!(precompiler.variables_len(), 0);
    }
}
