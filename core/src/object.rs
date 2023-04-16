use crate::{pointer::Ptr, variable::Variable};
use std::collections::HashMap;

pub struct Object {
    properties: HashMap<String, Ptr<Variable>>,
}

impl Object {
    pub fn new() -> Self {
        Self {
            properties: HashMap::new(),
        }
    }

    pub fn add_property(&mut self, property_name: String, property: Ptr<Variable>) {
        self.properties.insert(property_name, property);
    }

    pub fn get_property(&self, property_name: String) -> Option<&Ptr<Variable>> {
        self.properties.get(&property_name).map(|el| el)
    }

    pub fn to_string(&self) -> String {
        let mut res = "{".to_string();
        for (property_name, property) in &self.properties {
            res.push_str(
                format!("{0}: {1}", property_name, property.get_ref().to_string()).as_str(),
            );
        }
        res.push('}');
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_test() {
        let mut object = Object::new();
        assert_eq!(object.to_string(), "{}".to_string());

        assert_eq!(object.get_property("name".to_string()), None);

        let prop = Ptr::new(&mut Variable::Undefined).unwrap();

        object.add_property("name".to_string(), prop.clone());

        assert_eq!(
            object.get_property("name".to_string()).unwrap().get_raw(),
            prop.get_raw(),
        );

        assert_eq!(object.to_string(), r#"{name: undefined}"#.to_string());
    }
}
