use std::ops::{Add, Div, Mul, Sub};

use crate::number::Number;

#[derive(Debug, Clone, PartialEq)]
pub enum Variable {
    Undefined,
    Null,
    Number(Number),
    Boolean(bool),
    String(String),
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
            Self::String(string) => string.clone(),
        }
    }
}

impl Add for &Variable {
    type Output = Variable;
    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Variable::String(a), Variable::String(b)) => Variable::String(format!("{a}{b}")),
            (Variable::String(a), b) => Variable::String(format!("{a}{}", b.to_string())),
            (a, Variable::String(b)) => Variable::String(format!("{}{b}", a.to_string())),
            (a, b) => Variable::Number((&a.to_number()) + (&b.to_number())),
        }
    }
}

impl Sub for &Variable {
    type Output = Variable;
    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (a, b) => Variable::Number((&a.to_number()) - (&b.to_number())),
        }
    }
}

impl Mul for &Variable {
    type Output = Variable;
    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (a, b) => Variable::Number((&a.to_number()) * (&b.to_number())),
        }
    }
}

impl Div for &Variable {
    type Output = Variable;
    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (a, b) => Variable::Number((&a.to_number()) / (&b.to_number())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_boolean_test() {
        assert!(!Variable::Undefined.to_boolean());
        assert!(!Variable::Null.to_boolean());
        assert_eq!(
            Variable::Number(Number::Number(1.0)).to_boolean(),
            Number::Number(1.0).to_boolean()
        );
        assert!(Variable::Boolean(true).to_boolean());
        assert!(!Variable::Boolean(false).to_boolean());
        assert!(Variable::String("Hello".to_string()).to_boolean());
        assert!(!Variable::String("".to_string()).to_boolean());
    }

    #[test]
    fn to_number_test() {
        assert_eq!(Variable::Undefined.to_number(), Number::NaN);
        assert_eq!(Variable::Null.to_number(), Number::Number(0.0));
        assert_eq!(
            Variable::Number(Number::Number(1.0)).to_number(),
            Number::Number(1.0)
        );
        assert_eq!(Variable::Boolean(true).to_number(), Number::Number(1.0));
        assert_eq!(Variable::Boolean(false).to_number(), Number::Number(0.0));
        assert_eq!(
            Variable::String("Hello".to_string()).to_number(),
            Number::NaN
        );
        assert_eq!(Variable::String("".to_string()).to_number(), Number::NaN);
    }

    #[test]
    fn to_string_test() {
        assert_eq!(Variable::Undefined.to_string(), "undefined".to_string());
        assert_eq!(Variable::Null.to_string(), "null".to_string());
        assert_eq!(
            Variable::Number(Number::Number(1.0)).to_string(),
            Number::Number(1.0).to_string()
        );
        assert_eq!(Variable::Boolean(true).to_string(), "true".to_string());
        assert_eq!(Variable::Boolean(false).to_string(), "false".to_string());
        assert_eq!(
            Variable::String("Hello".to_string()).to_string(),
            "Hello".to_string()
        );
        assert_eq!(Variable::String("".to_string()).to_string(), "".to_string());
    }

    #[test]
    fn arithmetic_test() {
        assert_eq!(
            &Variable::String("Hello ".to_string()) + &Variable::String("world".to_string()),
            Variable::String("Hello world".to_string())
        );
        assert_eq!(
            &Variable::Number(Number::Number(2.0)) + &Variable::String(" world".to_string()),
            Variable::String("2 world".to_string())
        );
        assert_eq!(
            &Variable::String("Hello ".to_string()) + &Variable::Number(Number::Number(2.0)),
            Variable::String("Hello 2".to_string())
        );
        assert_eq!(
            &Variable::Number(Number::Number(1.0)) + &Variable::Number(Number::Number(2.0)),
            Variable::Number(Number::Number(3.0))
        );
        assert_eq!(
            &Variable::Number(Number::Number(1.0)) - &Variable::Number(Number::Number(2.0)),
            Variable::Number(Number::Number(-1.0))
        );
        assert_eq!(
            &Variable::Number(Number::Number(1.0)) * &Variable::Number(Number::Number(2.0)),
            Variable::Number(Number::Number(2.0))
        );
        assert_eq!(
            &Variable::Number(Number::Number(1.0)) / &Variable::Number(Number::Number(2.0)),
            Variable::Number(Number::Number(0.5))
        );
    }
}
