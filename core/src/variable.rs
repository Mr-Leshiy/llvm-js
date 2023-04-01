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

    // #[allow(clippy::inherent_to_string)]
    // pub fn to_string(&self) -> String {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_boolean_test() {
        assert!(!Variable::Undefined.to_boolean());
        assert!(!Variable::Null.to_boolean());
        assert!(Variable::Number(Number::Number(1.0)).to_boolean());
        assert!(Variable::Boolean(true).to_boolean());
        assert!(!Variable::Boolean(false).to_boolean());
        assert!(Variable::String("Hello".to_string()).to_boolean());
        assert!(!Variable::String("".to_string()).to_boolean());
    }
}
