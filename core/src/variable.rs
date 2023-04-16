use crate::number::Number;

#[derive(Debug, Clone, PartialEq)]
pub enum Variable {
    Undefined,
    Null,
    Number(Number),
    Boolean(bool),
    String(String),
}

impl From<Number> for Variable {
    fn from(value: Number) -> Self {
        Self::Number(value)
    }
}

impl From<bool> for Variable {
    fn from(value: bool) -> Self {
        Self::Boolean(value)
    }
}

impl From<String> for Variable {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl Variable {
    pub fn to_boolean(&self) -> bool {
        match self {
            Self::Undefined => false,
            Self::Null => false,
            Self::Number(number) => number.to_boolean(),
            Self::Boolean(boolean) => *boolean,
            Self::String(string) => !string.is_empty(),
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
            Self::String(string) => format!(r#""{}""#, string),
        }
    }
}

// arithmetic operations

impl Variable {
    pub fn add(a: &Variable, b: &Variable) -> Variable {
        match (a, b) {
            (Variable::String(a), Variable::String(b)) => format!("{a}{b}").into(),
            (Variable::String(a), b) => format!("{a}{}", b.to_string()).into(),
            (a, Variable::String(b)) => format!("{}{b}", a.to_string()).into(),
            (a, b) => Number::add(&a.to_number(), &b.to_number()).into(),
        }
    }

    pub fn sub(a: &Variable, b: &Variable) -> Variable {
        Number::sub(&a.to_number(), &b.to_number()).into()
    }

    pub fn mul(a: &Variable, b: &Variable) -> Variable {
        Number::mul(&a.to_number(), &b.to_number()).into()
    }

    pub fn div(a: &Variable, b: &Variable) -> Variable {
        Number::div(&a.to_number(), &b.to_number()).into()
    }
}

// logical operations

impl Variable {
    pub fn not(&self) -> Variable {
        (!self.to_boolean()).into()
    }

    pub fn and(a: &Variable, b: &Variable) -> Variable {
        if a.to_boolean() {
            b.clone()
        } else {
            a.clone()
        }
    }

    pub fn or(a: &Variable, b: &Variable) -> Variable {
        if a.to_boolean() {
            a.clone()
        } else {
            b.clone()
        }
    }

    pub fn gt(a: &Variable, b: &Variable) -> Variable {
        Number::gt(&a.to_number(), &b.to_number()).into()
    }

    pub fn ge(a: &Variable, b: &Variable) -> Variable {
        Number::ge(&a.to_number(), &b.to_number()).into()
    }

    pub fn lt(a: &Variable, b: &Variable) -> Variable {
        Number::lt(&a.to_number(), &b.to_number()).into()
    }

    pub fn le(a: &Variable, b: &Variable) -> Variable {
        Number::le(&a.to_number(), &b.to_number()).into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_strategy::proptest;

    #[proptest]
    fn from_tests(number: Number, boolean: bool, string: String) {
        assert_eq!(Variable::Number(number.clone()), number.into());
        assert_eq!(Variable::Boolean(boolean), boolean.into());
        assert_eq!(Variable::String(string.clone()), string.into());
    }

    #[proptest]
    fn to_boolean_test(number: Number, string: String) {
        assert!(!Variable::Undefined.to_boolean());
        assert!(!Variable::Null.to_boolean());
        assert_eq!(
            Variable::Number(number.clone()).to_boolean(),
            number.to_boolean()
        );
        assert!(Variable::Boolean(true).to_boolean());
        assert!(!Variable::Boolean(false).to_boolean());
        assert_eq!(
            Variable::String(string.clone()).to_boolean(),
            !string.is_empty()
        );
    }

    #[proptest]
    fn to_number_test(number: Number, string: String) {
        assert_eq!(Variable::Undefined.to_number(), Number::NaN);
        assert_eq!(Variable::Null.to_number(), Number::Number(0.0));
        assert_eq!(Variable::Number(number.clone()).to_number(), number);
        assert_eq!(Variable::Boolean(true).to_number(), Number::Number(1.0));
        assert_eq!(Variable::Boolean(false).to_number(), Number::Number(0.0));
        assert_eq!(Variable::String(string).to_number(), Number::NaN);
    }

    #[proptest]
    fn to_string_test(number: Number, string: String) {
        assert_eq!(Variable::Undefined.to_string(), "undefined".to_string());
        assert_eq!(Variable::Null.to_string(), "null".to_string());
        assert_eq!(
            Variable::Number(number.clone()).to_string(),
            number.to_string()
        );
        assert_eq!(Variable::Boolean(true).to_string(), "true".to_string());
        assert_eq!(Variable::Boolean(false).to_string(), "false".to_string());
        assert_eq!(
            Variable::String(string.clone()).to_string(),
            format!(r#""{}""#, string)
        );
    }

    #[proptest]
    fn arithmetic_test(number1: Number, number2: Number, string1: String, string2: String) {
        assert_eq!(
            Variable::add(
                &Variable::String(string1.clone()),
                &Variable::String(string2.clone())
            ),
            Variable::String(format!("{}{}", &string1, &string2))
        );
        assert_eq!(
            Variable::add(
                &Variable::Number(number1.clone()),
                &Variable::String(string2.clone())
            ),
            Variable::String(format!("{}{}", number1.to_string(), &string2))
        );
        assert_eq!(
            Variable::add(
                &Variable::String(string1.clone()),
                &Variable::Number(number2.clone())
            ),
            Variable::String(format!("{}{}", &string1, number2.to_string()))
        );

        assert_eq!(
            Variable::add(
                &Variable::Number(number1.clone()),
                &Variable::Number(number2.clone())
            ),
            Number::add(&number1, &number2).into()
        );
        assert_eq!(
            Variable::sub(
                &Variable::Number(number1.clone()),
                &Variable::Number(number2.clone())
            ),
            Number::sub(&number1, &number2).into()
        );
        assert_eq!(
            Variable::mul(
                &Variable::Number(number1.clone()),
                &Variable::Number(number2.clone())
            ),
            Number::mul(&number1, &number2).into()
        );
        assert_eq!(
            Variable::div(
                &Variable::Number(number1.clone()),
                &Variable::Number(number2.clone())
            ),
            Number::div(&number1, &number2).into()
        );
    }

    #[proptest]
    fn not_test(boolean: bool) {
        assert_eq!(Variable::Boolean(boolean).not(), (!boolean).into());
    }

    #[proptest]
    fn and_test(a: bool, b: bool) {
        assert_eq!(
            Variable::and(&Variable::Boolean(a), &Variable::Boolean(b)),
            (a && b).into()
        );
    }

    #[proptest]
    fn or_test(a: bool, b: bool) {
        assert_eq!(
            Variable::or(&Variable::Boolean(a), &Variable::Boolean(b)),
            (a || b).into()
        );
    }
}
