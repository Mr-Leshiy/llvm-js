use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Clone, PartialEq)]
pub enum Number {
    NaN,
    Infinity,
    NegInfinity,
    Number(f64),
}

impl Number {
    pub fn to_boolean(&self) -> bool {
        match self {
            Number::NaN => false,
            Number::Infinity => true,
            Number::NegInfinity => true,
            Number::Number(value) => value != &0_f64,
        }
    }

    #[allow(clippy::inherent_to_string)]
    pub fn to_string(&self) -> String {
        match self {
            Number::NaN => "NaN".to_string(),
            Number::Infinity => "Infinity".to_string(),
            Number::NegInfinity => "-Infinity".to_string(),
            Number::Number(value) => value.to_string(),
        }
    }
}

impl Add for &Number {
    type Output = Number;
    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (&Number::NaN, _) | (_, &Number::NaN) => Number::NaN,
            (&Number::Infinity, &Number::NegInfinity) => Number::NaN,
            (&Number::NegInfinity, &Number::Infinity) => Number::NaN,
            (&Number::Infinity, _) => Number::Infinity,
            (_, &Number::Infinity) => Number::Infinity,
            (&Number::NegInfinity, _) => Number::NegInfinity,
            (_, &Number::NegInfinity) => Number::NegInfinity,
            (&Number::Number(a), &Number::Number(b)) => Number::Number(a + b),
        }
    }
}

impl Sub for &Number {
    type Output = Number;
    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (&Number::NaN, _) | (_, &Number::NaN) => Number::NaN,
            (&Number::Infinity, &Number::Infinity) => Number::NaN,
            (&Number::Infinity, &Number::NegInfinity) => Number::Infinity,
            (&Number::NegInfinity, &Number::Infinity) => Number::NegInfinity,
            (&Number::NegInfinity, &Number::NegInfinity) => Number::NaN,
            (&Number::Infinity, _) => Number::Infinity,
            (_, &Number::Infinity) => Number::NegInfinity,
            (&Number::NegInfinity, _) => Number::NegInfinity,
            (_, &Number::NegInfinity) => Number::Infinity,
            (&Number::Number(a), &Number::Number(b)) => Number::Number(a - b),
        }
    }
}

impl Mul for &Number {
    type Output = Number;
    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (&Number::NaN, _) | (_, &Number::NaN) => Number::NaN,
            (&Number::Infinity, &Number::Infinity)
            | (&Number::NegInfinity, &Number::NegInfinity) => Number::Infinity,
            (&Number::Infinity, &Number::NegInfinity)
            | (&Number::NegInfinity, &Number::Infinity) => Number::NegInfinity,
            (&Number::Infinity, &Number::Number(a)) if a == 0.0 => Number::NaN,
            (&Number::Infinity, &Number::Number(a)) if a > 0.0 => Number::Infinity,
            (&Number::Infinity, &Number::Number(_)) => Number::NegInfinity,
            (&Number::NegInfinity, &Number::Number(a)) if a == 0.0 => Number::NaN,
            (&Number::NegInfinity, &Number::Number(a)) if a > 0.0 => Number::NegInfinity,
            (&Number::NegInfinity, &Number::Number(_)) => Number::Infinity,
            (&Number::Number(a), &Number::Infinity) if a == 0.0 => Number::NaN,
            (&Number::Number(a), &Number::Infinity) if a > 0.0 => Number::Infinity,
            (&Number::Number(_), &Number::Infinity) => Number::NegInfinity,
            (&Number::Number(a), &Number::NegInfinity) if a == 0.0 => Number::NaN,
            (&Number::Number(a), &Number::NegInfinity) if a > 0.0 => Number::NegInfinity,
            (&Number::Number(_), &Number::NegInfinity) => Number::Infinity,
            (&Number::Number(a), &Number::Number(b)) => Number::Number(a * b),
        }
    }
}

impl Div for &Number {
    type Output = Number;
    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (&Number::NaN, _) | (_, &Number::NaN) => Number::NaN,
            (&Number::Infinity, &Number::Infinity) => Number::NaN,
            (&Number::Infinity, &Number::NegInfinity) => Number::NaN,
            (&Number::NegInfinity, &Number::Infinity) => Number::NaN,
            (&Number::NegInfinity, &Number::NegInfinity) => Number::NaN,
            (&Number::Infinity, &Number::Number(a)) if a >= 0.0 => Number::Infinity,
            (&Number::Infinity, &Number::Number(_)) => Number::NegInfinity,
            (&Number::Number(_), &Number::Infinity) => Number::Number(0.0),

            (&Number::NegInfinity, &Number::Number(a)) if a >= 0.0 => Number::NegInfinity,
            (&Number::NegInfinity, &Number::Number(_)) => Number::Infinity,
            (&Number::Number(_), &Number::NegInfinity) => Number::Number(0.0),

            (&Number::Number(a), &Number::Number(b)) if a == 0.0 && b == 0.0 => Number::NaN,
            (&Number::Number(a), &Number::Number(b)) if a > 0.0 && b == 0.0 => Number::Infinity,
            (&Number::Number(a), &Number::Number(b)) if a < 0.0 && b == 0.0 => Number::NegInfinity,
            (&Number::Number(a), &Number::Number(b)) => Number::Number(a / b),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_boolean_test() {
        assert!(!Number::NaN.to_boolean());
        assert!(Number::Infinity.to_boolean());
        assert!(Number::NegInfinity.to_boolean());
        assert!(!Number::Number(0.0).to_boolean());
        assert!(Number::Number(1.0).to_boolean());
    }

    #[test]
    fn to_string_test() {
        assert_eq!(Number::NaN.to_string(), "NaN".to_string());
        assert_eq!(Number::Infinity.to_string(), "Infinity".to_string());
        assert_eq!(Number::NegInfinity.to_string(), "-Infinity".to_string());
        assert_eq!(Number::Number(0.0).to_string(), "0".to_string());
        assert_eq!(Number::Number(1.0).to_string(), "1".to_string());
    }

    #[test]
    fn add_test() {
        assert_eq!(
            &Number::Number(2.0) + &Number::Number(3.0),
            Number::Number(5.0)
        );
        assert_eq!(
            &Number::Number(0.0) + &Number::Number(2.0),
            Number::Number(2.0)
        );
        assert_eq!(
            &Number::Number(-2.0) + &Number::Number(3.0),
            Number::Number(1.0)
        );
        assert_eq!(
            &Number::Number(-2.5) + &Number::Number(-4.5),
            Number::Number(-7.0)
        );

        assert_eq!(&Number::NaN + &Number::Number(-4.5), Number::NaN);
        assert_eq!(&Number::Number(-4.5) + &Number::NaN, Number::NaN);
        assert_eq!(&Number::Infinity + &Number::NaN, Number::NaN);
        assert_eq!(&Number::NegInfinity + &Number::NaN, Number::NaN);

        assert_eq!(&Number::Infinity + &Number::Number(-4.5), Number::Infinity);
        assert_eq!(
            &Number::NegInfinity + &Number::Number(-4.5),
            Number::NegInfinity
        );
        assert_eq!(&Number::Infinity + &Number::NegInfinity, Number::NaN);
        assert_eq!(&Number::NegInfinity + &Number::Infinity, Number::NaN);
        assert_eq!(&Number::Infinity + &Number::Infinity, Number::Infinity);
        assert_eq!(
            &Number::NegInfinity + &Number::NegInfinity,
            Number::NegInfinity
        );
    }

    #[test]
    fn sub_test() {
        assert_eq!(
            &Number::Number(2.0) - &Number::Number(3.0),
            Number::Number(-1.0)
        );
        assert_eq!(
            &Number::Number(2.0) - &Number::Number(0.0),
            Number::Number(2.0)
        );
        assert_eq!(
            &Number::Number(0.0) - &Number::Number(2.0),
            Number::Number(-2.0)
        );
        assert_eq!(
            &Number::Number(-2.0) - &Number::Number(3.0),
            Number::Number(-5.0)
        );
        assert_eq!(
            &Number::Number(-2.5) - &Number::Number(-4.5),
            Number::Number(2.0)
        );

        assert_eq!(&Number::NaN - &Number::Number(-4.5), Number::NaN);
        assert_eq!(&Number::Number(-4.5) - &Number::NaN, Number::NaN);
        assert_eq!(&Number::Infinity - &Number::NaN, Number::NaN);
        assert_eq!(&Number::NegInfinity - &Number::NaN, Number::NaN);

        assert_eq!(&Number::Infinity - &Number::Number(-4.5), Number::Infinity);
        assert_eq!(
            &Number::NegInfinity - &Number::Number(-4.5),
            Number::NegInfinity
        );
        assert_eq!(&Number::Infinity - &Number::NegInfinity, Number::Infinity);
        assert_eq!(
            &Number::NegInfinity - &Number::Infinity,
            Number::NegInfinity
        );
        assert_eq!(&Number::Infinity - &Number::Infinity, Number::NaN);
        assert_eq!(&Number::NegInfinity - &Number::NegInfinity, Number::NaN);
    }

    #[test]
    fn mul_test() {
        assert_eq!(
            &Number::Number(2.0) * &Number::Number(3.0),
            Number::Number(6.0)
        );
        assert_eq!(
            &Number::Number(2.0) * &Number::Number(0.0),
            Number::Number(0.0)
        );
        assert_eq!(
            &Number::Number(0.0) * &Number::Number(2.0),
            Number::Number(0.0)
        );
        assert_eq!(
            &Number::Number(-2.0) * &Number::Number(3.0),
            Number::Number(-6.0)
        );
        assert_eq!(
            &Number::Number(-2.5) * &Number::Number(-4.5),
            Number::Number(11.25)
        );
        assert_eq!(&Number::NaN * &Number::Number(-4.5), Number::NaN);
        assert_eq!(&Number::Number(2.0) * &Number::NaN, Number::NaN);
        assert_eq!(&Number::Infinity * &Number::NaN, Number::NaN);
        assert_eq!(&Number::NaN * &Number::Infinity, Number::NaN);
        assert_eq!(&Number::NegInfinity * &Number::NaN, Number::NaN);
        assert_eq!(&Number::NaN * &Number::NegInfinity, Number::NaN);
        assert_eq!(&Number::Infinity * &Number::Number(1.0), Number::Infinity);
        assert_eq!(
            &Number::Infinity * &Number::Number(-1.0),
            Number::NegInfinity
        );
        assert_eq!(&Number::Infinity * &Number::Number(0.0), Number::NaN);
        assert_eq!(&Number::Number(1.0) * &Number::Infinity, Number::Infinity);
        assert_eq!(
            &Number::Number(-1.0) * &Number::Infinity,
            Number::NegInfinity
        );
        assert_eq!(&Number::Number(0.0) * &Number::Infinity, Number::NaN);
        assert_eq!(
            &Number::NegInfinity * &Number::Number(1.0),
            Number::NegInfinity
        );
        assert_eq!(
            &Number::NegInfinity * &Number::Number(-1.0),
            Number::Infinity
        );
        assert_eq!(&Number::NegInfinity * &Number::Number(0.0), Number::NaN);
        assert_eq!(
            &Number::Number(1.0) * &Number::NegInfinity,
            Number::NegInfinity
        );
        assert_eq!(
            &Number::Number(-1.0) * &Number::NegInfinity,
            Number::Infinity
        );
        assert_eq!(&Number::Number(0.0) * &Number::NegInfinity, Number::NaN);
        assert_eq!(&Number::Infinity * &Number::Infinity, Number::Infinity);
        assert_eq!(
            &Number::Infinity * &Number::NegInfinity,
            Number::NegInfinity
        );
        assert_eq!(
            &Number::NegInfinity * &Number::Infinity,
            Number::NegInfinity
        );
        assert_eq!(
            &Number::NegInfinity * &Number::NegInfinity,
            Number::Infinity
        );
    }

    #[test]
    fn div_test() {
        assert_eq!(
            &Number::Number(6.0) / &Number::Number(3.0),
            Number::Number(2.0)
        );
        assert_eq!(
            &Number::Number(-6.0) / &Number::Number(3.0),
            Number::Number(-2.0)
        );
        assert_eq!(
            &Number::Number(6.0) / &Number::Number(-3.0),
            Number::Number(-2.0)
        );
        assert_eq!(
            &Number::Number(-6.0) / &Number::Number(-3.0),
            Number::Number(2.0)
        );
        assert_eq!(
            &Number::Number(3.0) / &Number::Number(0.0),
            Number::Infinity
        );
        assert_eq!(
            &Number::Number(-3.0) / &Number::Number(0.0),
            Number::NegInfinity
        );

        assert_eq!(&Number::Number(0.0) / &Number::Number(0.0), Number::NaN);
        assert_eq!(&Number::NaN / &Number::Number(1.0), Number::NaN);
        assert_eq!(&Number::Number(1.0) / &Number::NaN, Number::NaN);
        assert_eq!(&Number::NaN / &Number::Infinity, Number::NaN);
        assert_eq!(&Number::Infinity / &Number::NaN, Number::NaN);
        assert_eq!(&Number::NaN / &Number::NegInfinity, Number::NaN);
        assert_eq!(&Number::NegInfinity / &Number::NaN, Number::NaN);

        assert_eq!(&Number::Infinity / &Number::Number(1.0), Number::Infinity);
        assert_eq!(
            &Number::Infinity / &Number::Number(-1.0),
            Number::NegInfinity
        );
        assert_eq!(
            &Number::NegInfinity / &Number::Number(1.0),
            Number::NegInfinity
        );
        assert_eq!(
            &Number::NegInfinity / &Number::Number(-1.0),
            Number::Infinity
        );

        assert_eq!(&Number::Infinity / &Number::Infinity, Number::NaN);
        assert_eq!(&Number::Infinity / &Number::NegInfinity, Number::NaN);
        assert_eq!(&Number::NegInfinity / &Number::Infinity, Number::NaN);
        assert_eq!(&Number::NegInfinity / &Number::NegInfinity, Number::NaN);

        assert_eq!(
            &Number::Number(1.0) / &Number::Infinity,
            Number::Number(0.0)
        );
        assert_eq!(
            &Number::Number(1.0) / &Number::NegInfinity,
            Number::Number(0.0)
        );
        assert_eq!(
            &Number::Number(-1.0) / &Number::Infinity,
            Number::Number(0.0)
        );
        assert_eq!(
            &Number::Number(-1.0) / &Number::NegInfinity,
            Number::Number(0.0)
        );
    }
}
