use crate::{
    number::Number,
    ptr::RawPtr,
    variable::{Variable, VariableValue},
};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct Array {
    values: Vec<RawPtr<Variable>>,
    properties: HashMap<String, RawPtr<Variable>>,
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

    fn add_value(&mut self, index: usize, mut value: RawPtr<Variable>) {
        value.inc_counter();
        while index >= self.values.len() {
            self.values.push(RawPtr::default());
        }
        self.values[index] = value;
    }

    fn get_value(&mut self, index: usize) -> RawPtr<Variable> {
        self.values
            .get(index)
            .map_or(RawPtr::default(), |val| val.copy())
    }

    pub fn add_property(&mut self, property_name: &VariableValue, mut property: RawPtr<Variable>) {
        property.inc_counter();
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

    pub fn get_property(&mut self, property_name: &VariableValue) -> RawPtr<Variable> {
        match property_name {
            VariableValue::Number(Number::Number(index)) => self.get_value(*index as usize),
            property_name => {
                let property_name = property_name.to_string();
                if property_name == Array::LENGTH_PROPERTY {
                    RawPtr::from(Variable::allocate(VariableValue::Number(Number::Number(
                        self.values.len() as f64,
                    ))))
                } else {
                    self.properties
                        .get(&property_name)
                        .map_or(RawPtr::default(), |val| val.copy())
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
            .push(RawPtr::from(Variable::allocate(VariableValue::Undefined)));
        array.properties.insert(
            "key".to_string(),
            RawPtr::from(Variable::allocate(VariableValue::Undefined)),
        );
        assert_eq!(array.to_string(), "[undefined]");

        array
            .values
            .push(RawPtr::from(Variable::allocate(VariableValue::Undefined)));
        assert_eq!(array.to_string(), "[undefined, undefined]");
    }

    #[test]
    fn add_value_test() {
        let mut array = Array::new();

        assert_eq!(array.values.len(), 0);
        assert_eq!(array.properties.len(), 0);

        array.add_value(
            0,
            RawPtr::from(Variable::allocate(VariableValue::Undefined)),
        );
        assert_eq!(array.values.len(), 1);
        assert_eq!(
            array.values[0].deref(),
            &Variable::allocate(VariableValue::Undefined)
        );
        assert_eq!(array.properties.len(), 0);

        array.add_value(
            1,
            RawPtr::from(Variable::allocate(VariableValue::Undefined)),
        );
        assert_eq!(array.values.len(), 2);
        assert_eq!(array.values[1].deref().deref(), &VariableValue::Undefined);
        assert_eq!(array.properties.len(), 0);

        array.add_value(0, RawPtr::from(Variable::allocate(VariableValue::Null)));
        assert_eq!(array.values.len(), 2);
        assert_eq!(array.values[0].deref().deref(), &VariableValue::Null);
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
        assert_eq!(val.deref().deref(), &VariableValue::Undefined);

        let val = array.get_value(3);
        assert_eq!(array.values.len(), 0);
        assert_eq!(array.properties.len(), 0);
        assert_eq!(val.deref().deref(), &VariableValue::Undefined);
    }

    #[test]
    fn add_property_test() {
        let mut array = Array::new();

        assert_eq!(array.values.len(), 0);
        assert_eq!(array.properties.len(), 0);

        array.add_property(
            &VariableValue::Null,
            RawPtr::from(Variable::allocate(VariableValue::Undefined)),
        );
        assert_eq!(array.properties.len(), 1);
        assert_eq!(array.values.len(), 0);
        assert_eq!(
            array
                .properties
                .get(&VariableValue::Null.to_string())
                .unwrap(),
            &RawPtr::from(Variable::allocate(VariableValue::Undefined))
        );

        array.add_property(
            &VariableValue::Number(Number::Number(0.0)),
            RawPtr::from(Variable::allocate(VariableValue::Undefined)),
        );
        assert_eq!(array.properties.len(), 1);
        assert_eq!(array.values.len(), 1);
        assert_eq!(
            array
                .properties
                .get(&VariableValue::Null.to_string())
                .unwrap(),
            &RawPtr::from(Variable::allocate(VariableValue::Undefined))
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
        assert_eq!(
            val.deref().deref(),
            &VariableValue::Number(Number::Number(0.0))
        );

        let val = array.get_property(&VariableValue::Null);
        assert_eq!(array.values.len(), 0);
        assert_eq!(array.properties.len(), 0);
        assert_eq!(val.deref().deref(), &VariableValue::Undefined);

        let val = array.get_property(&VariableValue::Number(Number::Number(0.0)));
        assert_eq!(array.values.len(), 0);
        assert_eq!(array.properties.len(), 0);
        assert_eq!(val.deref().deref(), &VariableValue::Undefined);

        let val = array.get_property(&VariableValue::String(Array::LENGTH_PROPERTY.to_string()));
        assert_eq!(array.values.len(), 0);
        assert_eq!(array.properties.len(), 0);
        assert_eq!(
            val.deref().deref(),
            &VariableValue::Number(Number::Number(0.0))
        );
    }
}
