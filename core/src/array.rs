use crate::{number::Number, ptr::Ptr, variable::Variable};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct Array {
    values: Vec<Ptr<Variable>>,
    properties: HashMap<String, Ptr<Variable>>,
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
            res.push_str(format!("{0}", value.get_ref().to_string()).as_str());
            for property in values_iter {
                res.push_str(format!(", {0}", property.get_ref().to_string()).as_str());
            }
        }
        res.push(']');
        res
    }

    fn add_value(&mut self, index: usize, value: Ptr<Variable>) {
        while index >= self.values.len() {
            self.values.push(Ptr::allocate(Variable::Undefined));
        }
        self.values[index] = value;
    }

    fn get_value(&mut self, index: usize) -> Ptr<Variable> {
        self.values
            .get(index)
            .map_or(Ptr::allocate(Variable::Undefined), |val| val.copy())
    }

    pub fn add_property(&mut self, property_name: &Variable, property: Ptr<Variable>) {
        match property_name {
            Variable::Number(Number::Number(index)) => {
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

    pub fn get_property(&mut self, property_name: &Variable) -> Ptr<Variable> {
        match property_name {
            Variable::Number(Number::Number(index)) => self.get_value(*index as usize),
            property_name => {
                let property_name = property_name.to_string();
                if property_name == Array::LENGTH_PROPERTY {
                    Ptr::allocate(Variable::Number(Number::Number(self.values.len() as f64)))
                } else {
                    self.properties
                        .get(&property_name)
                        .map_or(Ptr::allocate(Variable::Undefined), |val| val.copy())
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_string_test() {
        let mut array = Array::new();

        assert_eq!(array.to_string(), "[]");

        array.values.push(Ptr::allocate(Variable::Undefined));
        array
            .properties
            .insert("key".to_string(), Ptr::allocate(Variable::Undefined));
        assert_eq!(array.to_string(), "[undefined]");

        array.values.push(Ptr::allocate(Variable::Undefined));
        assert_eq!(array.to_string(), "[undefined, undefined]");
    }

    #[test]
    fn add_value_test() {
        let mut array = Array::new();

        assert_eq!(array.values.len(), 0);
        assert_eq!(array.properties.len(), 0);

        array.add_value(0, Ptr::allocate(Variable::Undefined));
        assert_eq!(array.values.len(), 1);
        assert_eq!(array.values[0].get_ref(), &Variable::Undefined);
        assert_eq!(array.properties.len(), 0);

        array.add_value(1, Ptr::allocate(Variable::Undefined));
        assert_eq!(array.values.len(), 2);
        assert_eq!(array.values[1].get_ref(), &Variable::Undefined);
        assert_eq!(array.properties.len(), 0);

        array.add_value(0, Ptr::allocate(Variable::Null));
        assert_eq!(array.values.len(), 2);
        assert_eq!(array.values[0].get_ref(), &Variable::Null);
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
        assert_eq!(val.get_ref(), &Variable::Undefined);

        let val = array.get_value(3);
        assert_eq!(array.values.len(), 0);
        assert_eq!(array.properties.len(), 0);
        assert_eq!(val.get_ref(), &Variable::Undefined);
    }

    #[test]
    fn add_property_test() {
        let mut array = Array::new();

        assert_eq!(array.values.len(), 0);
        assert_eq!(array.properties.len(), 0);

        array.add_property(&Variable::Null, Ptr::allocate(Variable::Undefined));
        assert_eq!(array.properties.len(), 1);
        assert_eq!(array.values.len(), 0);
        assert_eq!(
            array
                .properties
                .get(&Variable::Null.to_string())
                .unwrap()
                .get_ref(),
            Ptr::allocate(Variable::Undefined).get_ref()
        );

        array.add_property(
            &Variable::Number(Number::Number(0.0)),
            Ptr::allocate(Variable::Undefined),
        );
        assert_eq!(array.properties.len(), 1);
        assert_eq!(array.values.len(), 1);
        assert_eq!(
            array
                .properties
                .get(&Variable::Null.to_string())
                .unwrap()
                .get_ref(),
            Ptr::allocate(Variable::Undefined).get_ref()
        );
    }

    #[test]
    fn get_property_test() {
        let mut array = Array::new();

        assert_eq!(array.values.len(), 0);
        assert_eq!(array.properties.len(), 0);

        let val = array.get_property(&Variable::String(Array::LENGTH_PROPERTY.to_string()));
        assert_eq!(array.values.len(), 0);
        assert_eq!(array.properties.len(), 0);
        assert_eq!(val.get_ref(), &Variable::Number(Number::Number(0.0)));

        let val = array.get_property(&Variable::Null);
        assert_eq!(array.values.len(), 0);
        assert_eq!(array.properties.len(), 0);
        assert_eq!(val.get_ref(), &Variable::Undefined);

        let val = array.get_property(&Variable::Number(Number::Number(0.0)));
        assert_eq!(array.values.len(), 0);
        assert_eq!(array.properties.len(), 0);
        assert_eq!(val.get_ref(), &Variable::Undefined);

        let val = array.get_property(&Variable::String(Array::LENGTH_PROPERTY.to_string()));
        assert_eq!(array.values.len(), 0);
        assert_eq!(array.properties.len(), 0);
        assert_eq!(val.get_ref(), &Variable::Number(Number::Number(0.0)));
    }
}
