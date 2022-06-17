use crate::ast::Identifier;
use inkwell::values::PointerValue;
use std::collections::HashMap;

pub struct VariablesStorage<'ctx> {
    variables_names: Vec<Identifier>,
    variables: HashMap<Identifier, PointerValue<'ctx>>,
}

impl<'ctx> VariablesStorage<'ctx> {
    pub fn new() -> Self {
        Self {
            variables_names: Vec::new(),
            variables: HashMap::new(),
        }
    }

    pub fn insert(&mut self, variable_identifier: Identifier, value: PointerValue<'ctx>) {
        self.variables_names.push(variable_identifier.clone());
        self.variables.insert(variable_identifier, value);
    }

    pub fn get(&self, variable_identifier: &Identifier) -> Option<&PointerValue<'ctx>> {
        self.variables.get(variable_identifier)
    }

    pub fn len(&self) -> usize {
        assert!(self.variables.len() == self.variables_names.len());
        self.variables_names.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn variables_storage_test() {
        let mut storage = VariablesStorage::new();

        storage.insert(variable_identifier, value)
    }
}
