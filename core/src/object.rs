use crate::{pointer::Ptr, variable::Variable};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Object {
    Object {
        properties: HashMap<String, Ptr<Variable>>,
    },
    Array {
        properties: HashMap<String, Ptr<Variable>>,
    },
}

impl Object {
    #[allow(clippy::inherent_to_string)]
    pub fn to_string(&self) -> String {
        let mut res = String::new();
        match self {
            Self::Object { properties } => {
                res.push('{');
                for (property_name, property) in properties {
                    res.push_str(
                        format!("{0}: {1}", property_name, property.get_ref().to_string()).as_str(),
                    );
                }
                res.push('}');
            }
            Self::Array { properties } => {
                res.push('[');
                for (_, property) in properties {
                    res.push_str(format!("{0}, ", property.get_ref().to_string()).as_str());
                }
                res.push(']');
            }
        }
        res
    }
}

impl Object {
    pub fn new_object() -> Self {
        Self::Object {
            properties: HashMap::new(),
        }
    }

    pub fn new_array() -> Self {
        Self::Array {
            properties: HashMap::new(),
        }
    }

    pub fn add_property(&mut self, property_name: String, property: Ptr<Variable>) {
        match self {
            Self::Object { properties } => {
                properties.insert(property_name, property);
            }
            Self::Array { properties } => {
                properties.insert(property_name, property);
            }
        }
    }

    pub fn get_property(&self, property_name: String) -> Ptr<Variable> {
        match self {
            Self::Object { properties } => properties
                .get(&property_name)
                .map(|el| el.copy())
                .unwrap_or_else(|| Ptr::allocate(Variable::Undefined)),
            Self::Array { properties } => properties
                .get(&property_name)
                .map(|el| el.copy())
                .unwrap_or_else(|| Ptr::allocate(Variable::Undefined)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_test() {
        let mut object = Object::new_object();
        assert_eq!(object.to_string(), "{}".to_string());

        assert_eq!(
            object.get_property("name".to_string()),
            Ptr::allocate(Variable::Undefined)
        );

        let prop = Ptr::from_raw(&mut Variable::Undefined).unwrap();

        object.add_property("name".to_string(), prop.clone());

        assert_eq!(
            object.get_property("name".to_string()).get_raw(),
            prop.get_raw(),
        );

        assert_eq!(object.to_string(), r#"{name: undefined}"#.to_string());
    }
}
