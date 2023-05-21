use crate::{
    array::Array,
    function::Function,
    number::Number,
    object::Object,
    ptr::{RawPtr, SmartPtr},
};

pub type Variable = SmartPtr<VariableValue>;

#[derive(Debug, Clone, PartialEq)]
pub enum VariableValue {
    Undefined,
    Null,
    Number(Number),
    Boolean(bool),
    String(String),
    Object(Object),
    Array(Array),
    Function(Function),
}

impl Default for VariableValue {
    fn default() -> Self {
        Self::Undefined
    }
}

impl From<Number> for VariableValue {
    fn from(value: Number) -> Self {
        Self::Number(value)
    }
}

impl From<bool> for VariableValue {
    fn from(value: bool) -> Self {
        Self::Boolean(value)
    }
}

impl From<String> for VariableValue {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl VariableValue {
    pub fn to_boolean(&self) -> bool {
        match self {
            Self::Undefined => false,
            Self::Null => false,
            Self::Number(number) => number.to_boolean(),
            Self::Boolean(boolean) => *boolean,
            Self::String(string) => !string.is_empty(),
            Self::Object(_) => false,
            Self::Array(_) => false,
            Self::Function(_) => true,
        }
    }

    pub fn to_number(&self) -> Number {
        match self {
            Self::Undefined => Number::NaN,
            Self::Null => Number::Number(0.0),
            Self::Number(number) => number.clone(),
            Self::Boolean(true) => Number::Number(1.0),
            Self::Boolean(false) => Number::Number(0.0),
            Self::String(_) => Number::NaN,
            Self::Object(_) => Number::NaN,
            Self::Array(_) => Number::NaN,
            Self::Function(_) => Number::NaN,
        }
    }

    #[allow(clippy::inherent_to_string)]
    pub fn to_string(&self) -> String {
        match self {
            Self::Undefined => "undefined".to_string(),
            Self::Null => "null".to_string(),
            Self::Number(number) => number.to_string(),
            Self::Boolean(true) => "true".to_string(),
            Self::Boolean(false) => "false".to_string(),
            Self::String(string) => string.clone(),
            Self::Object(object) => object.to_string(),
            Self::Array(array) => array.to_string(),
            Self::Function(function) => function.to_string(),
        }
    }
}

impl VariableValue {
    pub fn add_property(&mut self, property_name: &VariableValue, property: RawPtr<Variable>) {
        // TODO print runtime error
        match self {
            Self::Object(object) => object.add_property(property_name, property),
            Self::Array(array) => array.add_property(property_name, property),
            _ => {}
        }
    }

    pub fn get_property(&mut self, property_name: &VariableValue) -> RawPtr<Variable> {
        match self {
            Self::Object(object) => object.get_property(property_name),
            Self::Array(array) => array.get_property(property_name),
            _ => RawPtr::default(),
        }
    }

    pub fn function_call(&self, args: &mut Vec<*mut Variable>) -> RawPtr<Variable> {
        if let Self::Function(function) = self {
            function.call(args)
        } else {
            RawPtr::default()
        }
    }
}

// arithmetic operations

impl VariableValue {
    pub fn add(a: &VariableValue, b: &VariableValue) -> VariableValue {
        match (a, b) {
            (VariableValue::String(a), VariableValue::String(b)) => format!("{a}{b}").into(),
            (VariableValue::String(a), b) => format!("{a}{}", b.to_string()).into(),
            (a, VariableValue::String(b)) => format!("{}{b}", a.to_string()).into(),
            (a, b) => Number::add(&a.to_number(), &b.to_number()).into(),
        }
    }

    pub fn sub(a: &VariableValue, b: &VariableValue) -> VariableValue {
        Number::sub(&a.to_number(), &b.to_number()).into()
    }

    pub fn mul(a: &VariableValue, b: &VariableValue) -> VariableValue {
        Number::mul(&a.to_number(), &b.to_number()).into()
    }

    pub fn div(a: &VariableValue, b: &VariableValue) -> VariableValue {
        Number::div(&a.to_number(), &b.to_number()).into()
    }
}

// logical operations

impl VariableValue {
    pub fn not(&self) -> VariableValue {
        (!self.to_boolean()).into()
    }

    pub fn and(a: &VariableValue, b: &VariableValue) -> VariableValue {
        if a.to_boolean() {
            b.clone()
        } else {
            a.clone()
        }
    }

    pub fn or(a: &VariableValue, b: &VariableValue) -> VariableValue {
        if a.to_boolean() {
            a.clone()
        } else {
            b.clone()
        }
    }

    pub fn gt(a: &VariableValue, b: &VariableValue) -> VariableValue {
        match (a, b) {
            (VariableValue::String(a), VariableValue::String(b)) => (a > b).into(),
            (a, b) => Number::gt(&a.to_number(), &b.to_number()).into(),
        }
    }

    pub fn ge(a: &VariableValue, b: &VariableValue) -> VariableValue {
        match (a, b) {
            (VariableValue::String(a), VariableValue::String(b)) => (a >= b).into(),
            (a, b) => Number::ge(&a.to_number(), &b.to_number()).into(),
        }
    }

    pub fn lt(a: &VariableValue, b: &VariableValue) -> VariableValue {
        match (a, b) {
            (VariableValue::String(a), VariableValue::String(b)) => (a < b).into(),
            (a, b) => Number::lt(&a.to_number(), &b.to_number()).into(),
        }
    }

    pub fn le(a: &VariableValue, b: &VariableValue) -> VariableValue {
        match (a, b) {
            (VariableValue::String(a), VariableValue::String(b)) => (a <= b).into(),
            (a, b) => Number::le(&a.to_number(), &b.to_number()).into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_strategy::proptest;

    #[proptest]
    fn from_tests(number: Number, boolean: bool, string: String) {
        assert_eq!(VariableValue::Number(number.clone()), number.into());
        assert_eq!(VariableValue::Boolean(boolean), boolean.into());
        assert_eq!(VariableValue::String(string.clone()), string.into());
    }

    #[proptest]
    fn to_boolean_test(number: Number, string: String) {
        assert!(!VariableValue::Undefined.to_boolean());
        assert!(!VariableValue::Null.to_boolean());
        assert_eq!(
            VariableValue::Number(number.clone()).to_boolean(),
            number.to_boolean()
        );
        assert!(VariableValue::Boolean(true).to_boolean());
        assert!(!VariableValue::Boolean(false).to_boolean());
        assert_eq!(
            VariableValue::String(string.clone()).to_boolean(),
            !string.is_empty()
        );
    }

    #[proptest]
    fn to_number_test(number: Number, string: String) {
        assert_eq!(VariableValue::Undefined.to_number(), Number::NaN);
        assert_eq!(VariableValue::Null.to_number(), Number::Number(0.0));
        assert_eq!(VariableValue::Number(number.clone()).to_number(), number);
        assert_eq!(
            VariableValue::Boolean(true).to_number(),
            Number::Number(1.0)
        );
        assert_eq!(
            VariableValue::Boolean(false).to_number(),
            Number::Number(0.0)
        );
        assert_eq!(VariableValue::String(string).to_number(), Number::NaN);
    }

    #[proptest]
    fn to_string_test(number: Number, string: String) {
        assert_eq!(
            VariableValue::Undefined.to_string(),
            "undefined".to_string()
        );
        assert_eq!(VariableValue::Null.to_string(), "null".to_string());
        assert_eq!(
            VariableValue::Number(number.clone()).to_string(),
            number.to_string()
        );
        assert_eq!(VariableValue::Boolean(true).to_string(), "true".to_string());
        assert_eq!(
            VariableValue::Boolean(false).to_string(),
            "false".to_string()
        );
        assert_eq!(VariableValue::String(string.clone()).to_string(), string);
    }

    #[proptest]
    fn arithmetic_test(number1: Number, number2: Number, string1: String, string2: String) {
        assert_eq!(
            VariableValue::add(
                &VariableValue::String(string1.clone()),
                &VariableValue::String(string2.clone())
            ),
            VariableValue::String(format!("{}{}", &string1, &string2))
        );
        assert_eq!(
            VariableValue::add(
                &VariableValue::Number(number1.clone()),
                &VariableValue::String(string2.clone())
            ),
            VariableValue::String(format!("{}{}", number1.to_string(), &string2))
        );
        assert_eq!(
            VariableValue::add(
                &VariableValue::String(string1.clone()),
                &VariableValue::Number(number2.clone())
            ),
            VariableValue::String(format!("{}{}", &string1, number2.to_string()))
        );

        assert_eq!(
            VariableValue::add(
                &VariableValue::Number(number1.clone()),
                &VariableValue::Number(number2.clone())
            ),
            Number::add(&number1, &number2).into()
        );
        assert_eq!(
            VariableValue::sub(
                &VariableValue::Number(number1.clone()),
                &VariableValue::Number(number2.clone())
            ),
            Number::sub(&number1, &number2).into()
        );
        assert_eq!(
            VariableValue::mul(
                &VariableValue::Number(number1.clone()),
                &VariableValue::Number(number2.clone())
            ),
            Number::mul(&number1, &number2).into()
        );
        assert_eq!(
            VariableValue::div(
                &VariableValue::Number(number1.clone()),
                &VariableValue::Number(number2.clone())
            ),
            Number::div(&number1, &number2).into()
        );
    }

    #[proptest]
    fn not_test(boolean: bool) {
        assert_eq!(VariableValue::Boolean(boolean).not(), (!boolean).into());
    }

    #[proptest]
    fn and_test(a: bool, b: bool) {
        assert_eq!(
            VariableValue::and(&VariableValue::Boolean(a), &VariableValue::Boolean(b)),
            (a && b).into()
        );
    }

    #[proptest]
    fn or_test(a: bool, b: bool) {
        assert_eq!(
            VariableValue::or(&VariableValue::Boolean(a), &VariableValue::Boolean(b)),
            (a || b).into()
        );
    }

    #[proptest]
    fn gt_test(a1: String, b1: String, a2: Number, b2: Number) {
        assert_eq!(
            VariableValue::Boolean(a1 > b1),
            VariableValue::gt(&VariableValue::String(a1), &VariableValue::String(b1)),
        );

        assert_eq!(
            VariableValue::Boolean(Number::gt(&a2, &b2)),
            VariableValue::gt(&a2.into(), &b2.into()),
        );
    }

    #[proptest]
    fn ge_test(a1: String, b1: String, a2: Number, b2: Number) {
        assert_eq!(
            VariableValue::Boolean(a1 >= b1),
            VariableValue::ge(&VariableValue::String(a1), &VariableValue::String(b1)),
        );

        assert_eq!(
            VariableValue::Boolean(Number::ge(&a2, &b2)),
            VariableValue::ge(&a2.into(), &b2.into()),
        );
    }

    #[proptest]
    fn lt_test(a1: String, b1: String, a2: Number, b2: Number) {
        assert_eq!(
            VariableValue::Boolean(a1 < b1),
            VariableValue::lt(&VariableValue::String(a1), &VariableValue::String(b1)),
        );

        assert_eq!(
            VariableValue::Boolean(Number::lt(&a2, &b2)),
            VariableValue::lt(&a2.into(), &b2.into()),
        );
    }

    #[proptest]
    fn le_test(a1: String, b1: String, a2: Number, b2: Number) {
        assert_eq!(
            VariableValue::Boolean(a1 <= b1),
            VariableValue::le(&VariableValue::String(a1), &VariableValue::String(b1)),
        );

        assert_eq!(
            VariableValue::Boolean(Number::le(&a2, &b2)),
            VariableValue::le(&a2.into(), &b2.into()),
        );
    }
}
