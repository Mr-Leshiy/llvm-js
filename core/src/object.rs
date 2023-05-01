use crate::{pointer::Ptr, variable::Variable};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct Object {
    properties: HashMap<String, Ptr<Variable>>,
}

impl Object {
    pub fn new() -> Self {
        Self {
            properties: HashMap::new(),
        }
    }

    #[allow(clippy::inherent_to_string)]
    pub fn to_string(&self) -> String {
        let mut res = String::new();
        res.push('{');
        for (property_name, property) in &self.properties {
            res.push_str(
                format!("{0}: {1}, ", property_name, property.get_ref().to_string()).as_str(),
            );
        }
        res.push('}');
        res
    }

    pub fn add_property(&mut self, property_name: &Variable, property: Ptr<Variable>) {
        self.properties.insert(property_name.to_string(), property);
    }

    pub fn get_property(&mut self, property_name: &Variable) -> Ptr<Variable> {
        self.properties
            .entry(property_name.to_string())
            .or_insert(Ptr::allocate(Variable::Undefined))
            .copy()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_string_test() {
        let mut object = Object::new();

        assert_eq!(object.to_string(), "{}");

        object
            .properties
            .insert("key".to_string(), Ptr::allocate(Variable::Undefined));
        assert_eq!(object.to_string(), "{key: undefined, }");
    }

    #[test]
    fn add_property_test() {
        let mut object = Object::new();

        assert_eq!(object.properties.len(), 0);

        object.add_property(&Variable::Null, Ptr::allocate(Variable::Undefined));
        assert_eq!(object.properties.len(), 1);
        assert_eq!(
            object
                .properties
                .get(&Variable::Null.to_string())
                .unwrap()
                .get_ref(),
            Ptr::allocate(Variable::Undefined).get_ref()
        );
    }

    #[test]
    fn get_property_test() {
        let mut object = Object::new();

        assert_eq!(object.properties.len(), 0);

        let val = object.get_property(&Variable::Null);
        assert_eq!(object.properties.len(), 1);
        assert_eq!(
            object
                .properties
                .get(&Variable::Null.to_string())
                .unwrap()
                .get_ref(),
            &Variable::Undefined
        );
        assert_eq!(val.get_ref(), &Variable::Undefined);
    }
}
