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
    variables: Set<T1>,
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

    pub fn insert_variable(&mut self, variable: T1) -> u32 {
        self.variables.insert(variable)
    }

    pub fn get_variable(&mut self, variable: T1) -> Result<(T1, u32), Error<T1>> {
        self.variables
            .get(&variable)
            .map(|index| (variable.clone(), index))
            .ok_or(Error::UndefinedVariable(variable))
    }

    pub fn remove_last_added_variables(&mut self, size: usize) -> Vec<(T1, u32)> {
        self.variables.remove_last_added(size)
    }

    pub fn variables_len(&self) -> usize {
        self.variables.len()
    }

    pub fn functions_len(&self) -> usize {
        self.functions.len()
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

        assert_eq!(
            precompiler.get_variable("var1".to_string()),
            Ok(("var1".to_string(), 1))
        );
        assert_eq!(
            precompiler.get_variable("var2".to_string()),
            Ok(("var2".to_string(), 0))
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
