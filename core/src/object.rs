use crate::{ptr::RawPtr, variable::VariableValue};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct Object {
    properties: HashMap<String, RawPtr<VariableValue>>,
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
            res.push_str(format!("{0}: {1}", property_name, property.to_string()).as_str());
            for (property_name, property) in properties_iter {
                res.push_str(format!(", {0}: {1}", property_name, property.to_string()).as_str());
            }
        }
        res.push('}');
        res
    }

    pub fn add_property(&mut self, property_name: &VariableValue, property: RawPtr<VariableValue>) {
        self.properties.insert(property_name.to_string(), property);
    }

    pub fn get_property(&mut self, property_name: &VariableValue) -> RawPtr<VariableValue> {
        self.properties
            .get(&property_name.to_string())
            .map_or(RawPtr::default(), |val| val.copy())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ops::Deref;

    #[test]
    fn to_string_test() {
        let mut object = Object::new();

        assert_eq!(object.to_string(), "{}");

        object
            .properties
            .insert("key1".to_string(), RawPtr::from(VariableValue::Undefined));
        assert_eq!(object.to_string(), "{key1: undefined}");
    }

    #[test]
    fn add_property_test() {
        let mut object = Object::new();

        assert_eq!(object.properties.len(), 0);

        object.add_property(&VariableValue::Null, RawPtr::from(VariableValue::Undefined));
        assert_eq!(object.properties.len(), 1);
        assert_eq!(
            object
                .properties
                .get(&VariableValue::Null.to_string())
                .unwrap(),
            &RawPtr::from(VariableValue::Undefined)
        );
    }

    #[test]
    fn get_property_test() {
        let mut object = Object::new();

        assert_eq!(object.properties.len(), 0);

        let val = object.get_property(&VariableValue::Null);
        assert_eq!(object.properties.len(), 0);
        assert_eq!(val.deref(), &VariableValue::Undefined);
    }
}
