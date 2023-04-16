use std::collections::HashMap;

use crate::variable::Variable;

#[derive(Debug, PartialEq)]
pub struct Object<'a> {
    properties: HashMap<String, &'a mut Variable>,
}

impl<'a> Object<'a> {
    pub fn new() -> Self {
        Self {
            properties: HashMap::new(),
        }
    }

    pub fn add_property(&mut self, property_name: String, property: &'a mut Variable) {
        self.properties.insert(property_name, property);
    }

    pub fn get_property(&self, property_name: String) -> Option<&&'a mut Variable> {
        self.properties.get(&property_name)
    }

    pub fn to_string(&self) -> String {
        let mut res = "{".to_string();
        for (property_name, property) in &self.properties {
            res.push_str(format!("{0}: {1}", property_name, property.to_string()).as_str());
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

        let mut var = Variable::String("Jack".to_string());
        object.add_property("name".to_string(), &mut var);

        assert_eq!(
            object.get_property("name".to_string()),
            Some(&&mut Variable::String("Jack".to_string()))
        );

        assert_eq!(object.to_string(), r#"{name: "Jack"}"#.to_string());
    }

    #[test]
    fn test() {
        let var = Box::new(Variable::Undefined);
    }
}
