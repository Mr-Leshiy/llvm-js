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
        let mut properties_iter = self.properties.iter();
        if let Some((property_name, property)) = properties_iter.next() {
            res.push_str(
                format!("{0}: {1}", property_name, property.get_ref().to_string()).as_str(),
            );
            for (property_name, property) in properties_iter {
                res.push_str(
                    format!(", {0}: {1}", property_name, property.get_ref().to_string()).as_str(),
                );
            }
        }
        res.push('}');
        res
    }

    pub fn add_property(&mut self, property_name: &Variable, property: Ptr<Variable>) {
        self.properties.insert(property_name.to_string(), property);
    }

    pub fn get_property(&mut self, property_name: &Variable) -> Ptr<Variable> {
        self.properties
            .get(&property_name.to_string())
            .map_or(Ptr::allocate(Variable::Undefined), |val| val.copy())
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
            .insert("key1".to_string(), Ptr::allocate(Variable::Undefined));
        assert_eq!(object.to_string(), "{key1: undefined}");
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
        assert_eq!(object.properties.len(), 0);
        assert_eq!(val.get_ref(), &Variable::Undefined);
    }
}
