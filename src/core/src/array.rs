use crate::{number::Number, ptr::RawPtr, variable::VariableValue};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct Array {
    values: Vec<RawPtr<VariableValue>>,
    properties: HashMap<String, RawPtr<VariableValue>>,
}

impl Array {
    const LENGTH_PROPERTY: &'static str = "length";

    pub fn new() -> Self {
        Self {
            values: Vec::new(),
            properties: HashMap::new(),
        }
    }

    #[allow(clippy::inherent_to_string)]
    pub fn to_string(&self) -> String {
        let mut res = String::new();
        res.push('[');
        let mut values_iter = self.values.iter();
        if let Some(value) = values_iter.next() {
            res.push_str(value.to_string().as_str());
            for property in values_iter {
                res.push_str(format!(", {0}", property.to_string()).as_str());
            }
        }
        res.push(']');
        res
    }

    fn add_value(&mut self, index: usize, value: RawPtr<VariableValue>) {
        while index >= self.values.len() {
            self.values.push(RawPtr::allocate(VariableValue::Undefined));
        }
        self.values[index] = value;
    }

    fn get_value(&mut self, index: usize) -> RawPtr<VariableValue> {
        self.values
            .get(index)
            .map_or(RawPtr::allocate(VariableValue::Undefined), |val| val.copy())
    }

    pub fn add_property(&mut self, property_name: &VariableValue, property: RawPtr<VariableValue>) {
        match property_name {
            VariableValue::Number(Number::Number(index)) => {
                self.add_value(*index as usize, property);
            }
            property_name => {
                let property_name = property_name.to_string();
                if property_name == Array::LENGTH_PROPERTY {
                    // TODO print runtime error
                } else {
                    self.properties.insert(property_name, property);
                }
            }
        }
    }

    pub fn get_property(&mut self, property_name: &VariableValue) -> RawPtr<VariableValue> {
        match property_name {
            VariableValue::Number(Number::Number(index)) => self.get_value(*index as usize),
            property_name => {
                let property_name = property_name.to_string();
                if property_name == Array::LENGTH_PROPERTY {
                    RawPtr::allocate(VariableValue::Number(Number::Number(
                        self.values.len() as f64
                    )))
                } else {
                    self.properties
                        .get(&property_name)
                        .map_or(RawPtr::allocate(VariableValue::Undefined), |val| val.copy())
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ops::Deref;

    #[test]
    fn to_string_test() {
        let mut array = Array::new();

        assert_eq!(array.to_string(), "[]");

        array
            .values
            .push(RawPtr::allocate(VariableValue::Undefined));
        array.properties.insert(
            "key".to_string(),
            RawPtr::allocate(VariableValue::Undefined),
        );
        assert_eq!(array.to_string(), "[undefined]");

        array
            .values
            .push(RawPtr::allocate(VariableValue::Undefined));
        assert_eq!(array.to_string(), "[undefined, undefined]");
    }

    #[test]
    fn add_value_test() {
        let mut array = Array::new();

        assert_eq!(array.values.len(), 0);
        assert_eq!(array.properties.len(), 0);

        array.add_value(0, RawPtr::allocate(VariableValue::Undefined));
        assert_eq!(array.values.len(), 1);
        assert_eq!(array.values[0].deref(), &VariableValue::Undefined);
        assert_eq!(array.properties.len(), 0);

        array.add_value(1, RawPtr::allocate(VariableValue::Undefined));
        assert_eq!(array.values.len(), 2);
        assert_eq!(array.values[1].deref(), &VariableValue::Undefined);
        assert_eq!(array.properties.len(), 0);

        array.add_value(0, RawPtr::allocate(VariableValue::Null));
        assert_eq!(array.values.len(), 2);
        assert_eq!(array.values[0].deref(), &VariableValue::Null);
        assert_eq!(array.properties.len(), 0);
    }

    #[test]
    fn get_value_test() {
        let mut array = Array::new();

        assert_eq!(array.values.len(), 0);
        assert_eq!(array.properties.len(), 0);

        let val = array.get_value(0);
        assert_eq!(array.values.len(), 0);
        assert_eq!(array.properties.len(), 0);
        assert_eq!(val.deref(), &VariableValue::Undefined);

        let val = array.get_value(3);
        assert_eq!(array.values.len(), 0);
        assert_eq!(array.properties.len(), 0);
        assert_eq!(val.deref(), &VariableValue::Undefined);
    }

    #[test]
    fn add_property_test() {
        let mut array = Array::new();

        assert_eq!(array.values.len(), 0);
        assert_eq!(array.properties.len(), 0);

        array.add_property(
            &VariableValue::Null,
            RawPtr::allocate(VariableValue::Undefined),
        );
        assert_eq!(array.properties.len(), 1);
        assert_eq!(array.values.len(), 0);
        assert_eq!(
            array
                .properties
                .get(&VariableValue::Null.to_string())
                .unwrap(),
            &RawPtr::allocate(VariableValue::Undefined)
        );

        array.add_property(
            &VariableValue::Number(Number::Number(0.0)),
            RawPtr::allocate(VariableValue::Undefined),
        );
        assert_eq!(array.properties.len(), 1);
        assert_eq!(array.values.len(), 1);
        assert_eq!(
            array
                .properties
                .get(&VariableValue::Null.to_string())
                .unwrap(),
            &RawPtr::allocate(VariableValue::Undefined)
        );
    }

    #[test]
    fn get_property_test() {
        let mut array = Array::new();

        assert_eq!(array.values.len(), 0);
        assert_eq!(array.properties.len(), 0);

        let val = array.get_property(&VariableValue::String(Array::LENGTH_PROPERTY.to_string()));
        assert_eq!(array.values.len(), 0);
        assert_eq!(array.properties.len(), 0);
        assert_eq!(val.deref(), &VariableValue::Number(Number::Number(0.0)));

        let val = array.get_property(&VariableValue::Null);
        assert_eq!(array.values.len(), 0);
        assert_eq!(array.properties.len(), 0);
        assert_eq!(val.deref(), &VariableValue::Undefined);

        let val = array.get_property(&VariableValue::Number(Number::Number(0.0)));
        assert_eq!(array.values.len(), 0);
        assert_eq!(array.properties.len(), 0);
        assert_eq!(val.deref(), &VariableValue::Undefined);

        let val = array.get_property(&VariableValue::String(Array::LENGTH_PROPERTY.to_string()));
        assert_eq!(array.values.len(), 0);
        assert_eq!(array.properties.len(), 0);
        assert_eq!(val.deref(), &VariableValue::Number(Number::Number(0.0)));
    }
}
