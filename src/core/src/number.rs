use test_strategy::Arbitrary;

#[derive(Arbitrary, Debug, Clone, PartialEq, PartialOrd)]
pub enum Number {
    NaN,
    Infinity,
    NegInfinity,
    Num(f64),
}

impl From<f64> for Number {
    fn from(value: f64) -> Self {
        if value == f64::INFINITY {
            Number::Infinity
        } else if value == f64::NEG_INFINITY {
            Number::NegInfinity
        } else if value.is_nan() {
            Number::NaN
        } else {
            Number::Num(value)
        }
    }
}

impl Number {
    pub fn to_boolean(&self) -> bool {
        match self {
            Number::NaN => false,
            Number::Infinity => true,
            Number::NegInfinity => true,
            Number::Num(value) => value != &0.0,
        }
    }

    #[allow(clippy::inherent_to_string)]
    pub fn to_string(&self) -> String {
        match self {
            Number::NaN => "NaN".to_string(),
            Number::Infinity => "Infinity".to_string(),
            Number::NegInfinity => "-Infinity".to_string(),
            Number::Num(value) => value.to_string(),
        }
    }
}

// arithmetic operations

impl Number {
    pub fn add(a: &Number, b: &Number) -> Number {
        match (a, b) {
            (&Number::NaN, _) | (_, &Number::NaN) => Number::NaN,
            (&Number::Infinity, &Number::NegInfinity) => Number::NaN,
            (&Number::NegInfinity, &Number::Infinity) => Number::NaN,
            (&Number::Infinity, _) => Number::Infinity,
            (_, &Number::Infinity) => Number::Infinity,
            (&Number::NegInfinity, _) => Number::NegInfinity,
            (_, &Number::NegInfinity) => Number::NegInfinity,
            (&Number::Num(a), &Number::Num(b)) => (a + b).into(),
        }
    }

    pub fn sub(a: &Number, b: &Number) -> Number {
        match (a, b) {
            (&Number::NaN, _) | (_, &Number::NaN) => Number::NaN,
            (&Number::Infinity, &Number::Infinity) => Number::NaN,
            (&Number::Infinity, &Number::NegInfinity) => Number::Infinity,
            (&Number::NegInfinity, &Number::Infinity) => Number::NegInfinity,
            (&Number::NegInfinity, &Number::NegInfinity) => Number::NaN,
            (&Number::Infinity, _) => Number::Infinity,
            (_, &Number::Infinity) => Number::NegInfinity,
            (&Number::NegInfinity, _) => Number::NegInfinity,
            (_, &Number::NegInfinity) => Number::Infinity,
            (&Number::Num(a), &Number::Num(b)) => (a - b).into(),
        }
    }

    pub fn mul(a: &Number, b: &Number) -> Number {
        match (a, b) {
            (&Number::NaN, _) | (_, &Number::NaN) => Number::NaN,
            (&Number::Infinity, &Number::Infinity)
            | (&Number::NegInfinity, &Number::NegInfinity) => Number::Infinity,
            (&Number::Infinity, &Number::NegInfinity)
            | (&Number::NegInfinity, &Number::Infinity) => Number::NegInfinity,
            (&Number::Infinity, &Number::Num(a)) if a == 0.0 => Number::NaN,
            (&Number::Infinity, &Number::Num(a)) if a > 0.0 => Number::Infinity,
            (&Number::Infinity, &Number::Num(_)) => Number::NegInfinity,
            (&Number::NegInfinity, &Number::Num(a)) if a == 0.0 => Number::NaN,
            (&Number::NegInfinity, &Number::Num(a)) if a > 0.0 => Number::NegInfinity,
            (&Number::NegInfinity, &Number::Num(_)) => Number::Infinity,
            (&Number::Num(a), &Number::Infinity) if a == 0.0 => Number::NaN,
            (&Number::Num(a), &Number::Infinity) if a > 0.0 => Number::Infinity,
            (&Number::Num(_), &Number::Infinity) => Number::NegInfinity,
            (&Number::Num(a), &Number::NegInfinity) if a == 0.0 => Number::NaN,
            (&Number::Num(a), &Number::NegInfinity) if a > 0.0 => Number::NegInfinity,
            (&Number::Num(_), &Number::NegInfinity) => Number::Infinity,
            (&Number::Num(a), &Number::Num(b)) => (a * b).into(),
        }
    }

    pub fn div(a: &Number, b: &Number) -> Number {
        match (a, b) {
            (&Number::NaN, _) | (_, &Number::NaN) => Number::NaN,
            (&Number::Infinity, &Number::Infinity) => Number::NaN,
            (&Number::Infinity, &Number::NegInfinity) => Number::NaN,
            (&Number::NegInfinity, &Number::Infinity) => Number::NaN,
            (&Number::NegInfinity, &Number::NegInfinity) => Number::NaN,
            (&Number::Infinity, &Number::Num(a)) if a >= 0.0 => Number::Infinity,
            (&Number::Infinity, &Number::Num(_)) => Number::NegInfinity,
            (&Number::Num(_), &Number::Infinity) => Number::Num(0.0),

            (&Number::NegInfinity, &Number::Num(a)) if a >= 0.0 => Number::NegInfinity,
            (&Number::NegInfinity, &Number::Num(_)) => Number::Infinity,
            (&Number::Num(_), &Number::NegInfinity) => Number::Num(0.0),

            (&Number::Num(a), &Number::Num(b)) if a == 0.0 && b == 0.0 => Number::NaN,
            (&Number::Num(a), &Number::Num(b)) if a > 0.0 && b == 0.0 => Number::Infinity,
            (&Number::Num(a), &Number::Num(b)) if a < 0.0 && b == 0.0 => Number::NegInfinity,
            (&Number::Num(a), &Number::Num(b)) => (a / b).into(),
        }
    }
}

// logical operations

impl Number {
    pub fn gt(a: &Number, b: &Number) -> bool {
        match (a, b) {
            (&Number::NaN, _) => false,
            (_, &Number::NaN) => false,
            (&Number::Infinity, &Number::Infinity) => false,
            (&Number::Infinity, _) => true,
            (&Number::NegInfinity, &Number::NegInfinity) => false,
            (_, &Number::NegInfinity) => true,
            (&Number::Num(a), &Number::Num(b)) => a > b,
            _ => false,
        }
    }

    pub fn ge(a: &Number, b: &Number) -> bool {
        match (a, b) {
            (&Number::NaN, _) => false,
            (_, &Number::NaN) => false,
            (&Number::Infinity, &Number::Infinity) => true,
            (&Number::Infinity, _) => true,
            (&Number::NegInfinity, &Number::NegInfinity) => true,
            (_, &Number::NegInfinity) => true,
            (&Number::Num(a), &Number::Num(b)) => a >= b,
            _ => false,
        }
    }

    pub fn lt(a: &Number, b: &Number) -> bool {
        match (a, b) {
            (&Number::NaN, _) => false,
            (_, &Number::NaN) => false,
            (&Number::Infinity, &Number::Infinity) => false,
            (&Number::Infinity, _) => false,
            (&Number::NegInfinity, &Number::NegInfinity) => false,
            (_, &Number::NegInfinity) => false,
            (&Number::Num(a), &Number::Num(b)) => a < b,
            _ => true,
        }
    }

    pub fn le(a: &Number, b: &Number) -> bool {
        match (a, b) {
            (&Number::NaN, _) => false,
            (_, &Number::NaN) => false,
            (&Number::Infinity, &Number::Infinity) => true,
            (&Number::Infinity, _) => false,
            (&Number::NegInfinity, &Number::NegInfinity) => true,
            (_, &Number::NegInfinity) => false,
            (&Number::Num(a), &Number::Num(b)) => a <= b,
            _ => true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_strategy::proptest;

    #[proptest]
    fn from_f64_test(number: f64) {
        assert_eq!(Number::Num(number), number.into());
        assert_eq!(Number::Infinity, f64::INFINITY.into());
        assert_eq!(Number::NegInfinity, f64::NEG_INFINITY.into());
        assert_eq!(Number::NaN, f64::NAN.into());
    }

    #[proptest]
    fn to_boolean_test(a: f64) {
        assert!(!Number::NaN.to_boolean());
        assert!(Number::Infinity.to_boolean());
        assert!(Number::NegInfinity.to_boolean());
        assert_eq!(Number::Num(a).to_boolean(), a != 0.0);
    }

    #[proptest]
    fn to_string_test(a: f64) {
        assert_eq!(Number::NaN.to_string(), "NaN".to_string());
        assert_eq!(Number::Infinity.to_string(), "Infinity".to_string());
        assert_eq!(Number::NegInfinity.to_string(), "-Infinity".to_string());
        assert_eq!(Number::Num(a).to_string(), a.to_string());
    }

    #[proptest]
    fn add_test(a: f64, b: f64) {
        assert_eq!(
            Number::add(&Number::Num(a), &Number::Num(b)),
            (a + b).into()
        );

        assert_eq!(Number::add(&Number::NaN, &Number::Num(b)), Number::NaN);
        assert_eq!(Number::add(&Number::Num(a), &Number::NaN), Number::NaN);
        assert_eq!(Number::add(&Number::Infinity, &Number::NaN), Number::NaN);
        assert_eq!(Number::add(&Number::NegInfinity, &Number::NaN), Number::NaN);

        assert_eq!(
            Number::add(&Number::Infinity, &Number::Num(b)),
            Number::Infinity
        );
        assert_eq!(
            Number::add(&Number::NegInfinity, &Number::Num(b)),
            Number::NegInfinity
        );
        assert_eq!(
            Number::add(&Number::Infinity, &Number::NegInfinity),
            Number::NaN
        );
        assert_eq!(
            Number::add(&Number::NegInfinity, &Number::Infinity),
            Number::NaN
        );
        assert_eq!(
            Number::add(&Number::Infinity, &Number::Infinity),
            Number::Infinity
        );
        assert_eq!(
            Number::add(&Number::NegInfinity, &Number::NegInfinity),
            Number::NegInfinity
        );
    }

    #[proptest]
    fn sub_test(a: f64, b: f64) {
        assert_eq!(
            Number::sub(&Number::Num(a), &Number::Num(b)),
            (a - b).into()
        );

        assert_eq!(Number::sub(&Number::NaN, &Number::Num(b)), Number::NaN);
        assert_eq!(Number::sub(&Number::Num(a), &Number::NaN), Number::NaN);
        assert_eq!(Number::sub(&Number::Infinity, &Number::NaN), Number::NaN);
        assert_eq!(Number::sub(&Number::NegInfinity, &Number::NaN), Number::NaN);

        assert_eq!(
            Number::sub(&Number::Infinity, &Number::Num(b)),
            Number::Infinity
        );
        assert_eq!(
            Number::sub(&Number::NegInfinity, &Number::Num(b)),
            Number::NegInfinity
        );
        assert_eq!(
            Number::sub(&Number::Infinity, &Number::NegInfinity),
            Number::Infinity
        );
        assert_eq!(
            Number::sub(&Number::NegInfinity, &Number::Infinity),
            Number::NegInfinity
        );
        assert_eq!(
            Number::sub(&Number::Infinity, &Number::Infinity),
            Number::NaN
        );
        assert_eq!(
            Number::sub(&Number::NegInfinity, &Number::NegInfinity),
            Number::NaN
        );
    }

    #[proptest]
    fn mul_test(#[filter(#a != 0.0)] a: f64, #[filter(#b != 0.0)] b: f64) {
        assert_eq!(
            Number::mul(&Number::Num(a), &Number::Num(b)),
            (a * b).into()
        );

        assert_eq!(Number::mul(&Number::NaN, &Number::Num(b)), Number::NaN);
        assert_eq!(Number::mul(&Number::Num(a), &Number::NaN), Number::NaN);
        assert_eq!(Number::mul(&Number::Infinity, &Number::NaN), Number::NaN);
        assert_eq!(Number::mul(&Number::NaN, &Number::Infinity), Number::NaN);
        assert_eq!(Number::mul(&Number::NegInfinity, &Number::NaN), Number::NaN);
        assert_eq!(Number::mul(&Number::NaN, &Number::NegInfinity), Number::NaN);
        assert_eq!(
            Number::mul(&Number::Infinity, &Number::Num(f64::abs(b))),
            Number::Infinity
        );
        assert_eq!(
            Number::mul(&Number::Infinity, &Number::Num(-f64::abs(b))),
            Number::NegInfinity
        );
        assert_eq!(
            Number::mul(&Number::Infinity, &Number::Num(0.0)),
            Number::NaN
        );
        assert_eq!(
            Number::mul(&Number::Num(f64::abs(a)), &Number::Infinity),
            Number::Infinity
        );
        assert_eq!(
            Number::mul(&Number::Num(-f64::abs(a)), &Number::Infinity),
            Number::NegInfinity
        );
        assert_eq!(
            Number::mul(&Number::Num(0.0), &Number::Infinity),
            Number::NaN
        );
        assert_eq!(
            Number::mul(&Number::NegInfinity, &Number::Num(f64::abs(b))),
            Number::NegInfinity
        );
        assert_eq!(
            Number::mul(&Number::NegInfinity, &Number::Num(-f64::abs(b))),
            Number::Infinity
        );
        assert_eq!(
            Number::mul(&Number::NegInfinity, &Number::Num(0.0)),
            Number::NaN
        );
        assert_eq!(
            Number::mul(&Number::Num(f64::abs(a)), &Number::NegInfinity),
            Number::NegInfinity
        );
        assert_eq!(
            Number::mul(&Number::Num(-f64::abs(a)), &Number::NegInfinity),
            Number::Infinity
        );
        assert_eq!(
            Number::mul(&Number::Num(0.0), &Number::NegInfinity),
            Number::NaN
        );
        assert_eq!(
            Number::mul(&Number::Infinity, &Number::Infinity),
            Number::Infinity
        );
        assert_eq!(
            Number::mul(&Number::Infinity, &Number::NegInfinity),
            Number::NegInfinity
        );
        assert_eq!(
            Number::mul(&Number::NegInfinity, &Number::Infinity),
            Number::NegInfinity
        );
        assert_eq!(
            Number::mul(&Number::NegInfinity, &Number::NegInfinity),
            Number::Infinity
        );
    }

    #[proptest]
    fn div_test(#[filter(#a != 0.0)] a: f64, #[filter(#b != 0.0)] b: f64) {
        assert_eq!(
            Number::div(&Number::Num(a), &Number::Num(b)),
            (a / b).into()
        );

        assert_eq!(
            Number::div(&Number::Num(0.0), &Number::Num(0.0)),
            Number::NaN
        );
        assert_eq!(Number::div(&Number::NaN, &Number::Num(b)), Number::NaN);
        assert_eq!(Number::div(&Number::Num(a), &Number::NaN), Number::NaN);
        assert_eq!(Number::div(&Number::NaN, &Number::Infinity), Number::NaN);
        assert_eq!(Number::div(&Number::Infinity, &Number::NaN), Number::NaN);
        assert_eq!(Number::div(&Number::NaN, &Number::NegInfinity), Number::NaN);
        assert_eq!(Number::div(&Number::NegInfinity, &Number::NaN), Number::NaN);

        assert_eq!(
            Number::div(&Number::Infinity, &Number::Num(f64::abs(b))),
            Number::Infinity
        );
        assert_eq!(
            Number::div(&Number::Infinity, &Number::Num(-f64::abs(b))),
            Number::NegInfinity
        );
        assert_eq!(
            Number::div(&Number::NegInfinity, &Number::Num(f64::abs(b))),
            Number::NegInfinity
        );
        assert_eq!(
            Number::div(&Number::NegInfinity, &Number::Num(-f64::abs(b))),
            Number::Infinity
        );

        assert_eq!(
            Number::div(&Number::Infinity, &Number::Infinity),
            Number::NaN
        );
        assert_eq!(
            Number::div(&Number::Infinity, &Number::NegInfinity),
            Number::NaN
        );
        assert_eq!(
            Number::div(&Number::NegInfinity, &Number::Infinity),
            Number::NaN
        );
        assert_eq!(
            Number::div(&Number::NegInfinity, &Number::NegInfinity),
            Number::NaN
        );

        assert_eq!(
            Number::div(&Number::Num(a), &Number::Infinity),
            Number::Num(0.0)
        );
        assert_eq!(
            Number::div(&Number::Num(a), &Number::NegInfinity),
            Number::Num(0.0)
        );
    }

    #[proptest]
    fn gt_test(a: f64, b: f64) {
        assert!(!Number::gt(&Number::NaN, &Number::NaN));
        assert!(!Number::gt(&Number::NaN, &Number::Infinity));
        assert!(!Number::gt(&Number::NaN, &Number::NegInfinity));
        assert!(!Number::gt(&Number::NaN, &Number::Num(b)));
        assert!(!Number::gt(&Number::Infinity, &Number::NaN));
        assert!(!Number::gt(&Number::NegInfinity, &Number::NaN));
        assert!(!Number::gt(&Number::Num(a), &Number::NaN));

        assert!(!Number::gt(&Number::Infinity, &Number::Infinity));
        assert!(Number::gt(&Number::Infinity, &Number::NegInfinity));
        assert!(Number::gt(&Number::Infinity, &Number::Num(b)));
        assert!(!Number::gt(&Number::NegInfinity, &Number::Infinity));
        assert!(!Number::gt(&Number::Num(a), &Number::Infinity));

        assert!(!Number::gt(&Number::NegInfinity, &Number::NegInfinity));
        assert!(!Number::gt(&Number::NegInfinity, &Number::Num(b)));
        assert!(Number::gt(&Number::Num(a), &Number::NegInfinity));

        assert_eq!(Number::gt(&Number::Num(a), &Number::Num(b)), a > b,);
    }

    #[proptest]
    fn ge_test(a: f64, b: f64) {
        assert!(!Number::ge(&Number::NaN, &Number::NaN));
        assert!(!Number::ge(&Number::NaN, &Number::Infinity));
        assert!(!Number::ge(&Number::NaN, &Number::NegInfinity));
        assert!(!Number::ge(&Number::NaN, &Number::Num(b)));
        assert!(!Number::ge(&Number::Infinity, &Number::NaN));
        assert!(!Number::ge(&Number::NegInfinity, &Number::NaN));
        assert!(!Number::ge(&Number::Num(a), &Number::NaN));

        assert!(Number::ge(&Number::Infinity, &Number::Infinity));
        assert!(Number::ge(&Number::Infinity, &Number::NegInfinity));
        assert!(Number::ge(&Number::Infinity, &Number::Num(b)));
        assert!(!Number::ge(&Number::NegInfinity, &Number::Infinity));
        assert!(!Number::ge(&Number::Num(a), &Number::Infinity));

        assert!(Number::ge(&Number::NegInfinity, &Number::NegInfinity));
        assert!(!Number::ge(&Number::NegInfinity, &Number::Num(b)));
        assert!(Number::ge(&Number::Num(a), &Number::NegInfinity));

        assert_eq!(Number::ge(&Number::Num(a), &Number::Num(b)), a >= b,);
    }

    #[proptest]
    fn lt_test(a: f64, b: f64) {
        assert!(!Number::lt(&Number::NaN, &Number::NaN));
        assert!(!Number::lt(&Number::NaN, &Number::Infinity));
        assert!(!Number::lt(&Number::NaN, &Number::NegInfinity));
        assert!(!Number::lt(&Number::NaN, &Number::Num(b)));
        assert!(!Number::lt(&Number::Infinity, &Number::NaN));
        assert!(!Number::lt(&Number::NegInfinity, &Number::NaN));
        assert!(!Number::lt(&Number::Num(a), &Number::NaN));

        assert!(!Number::lt(&Number::Infinity, &Number::Infinity));
        assert!(!Number::lt(&Number::Infinity, &Number::NegInfinity));
        assert!(!Number::lt(&Number::Infinity, &Number::Num(b)));
        assert!(Number::lt(&Number::NegInfinity, &Number::Infinity));
        assert!(Number::lt(&Number::Num(a), &Number::Infinity));

        assert!(!Number::lt(&Number::NegInfinity, &Number::NegInfinity));
        assert!(Number::lt(&Number::NegInfinity, &Number::Num(b)));
        assert!(!Number::lt(&Number::Num(a), &Number::NegInfinity));

        assert_eq!(Number::lt(&Number::Num(a), &Number::Num(b)), a < b,);
    }

    #[proptest]
    fn le_test(a: f64, b: f64) {
        assert!(!Number::le(&Number::NaN, &Number::NaN));
        assert!(!Number::le(&Number::NaN, &Number::Infinity));
        assert!(!Number::le(&Number::NaN, &Number::NegInfinity));
        assert!(!Number::le(&Number::NaN, &Number::Num(b)));
        assert!(!Number::le(&Number::Infinity, &Number::NaN));
        assert!(!Number::le(&Number::NegInfinity, &Number::NaN));
        assert!(!Number::le(&Number::Num(a), &Number::NaN));

        assert!(Number::le(&Number::Infinity, &Number::Infinity));
        assert!(!Number::le(&Number::Infinity, &Number::NegInfinity));
        assert!(!Number::le(&Number::Infinity, &Number::Num(b)));
        assert!(Number::le(&Number::NegInfinity, &Number::Infinity));
        assert!(Number::le(&Number::Num(a), &Number::Infinity));

        assert!(Number::le(&Number::NegInfinity, &Number::NegInfinity));
        assert!(Number::le(&Number::NegInfinity, &Number::Num(b)));
        assert!(!Number::le(&Number::Num(a), &Number::NegInfinity));

        assert_eq!(Number::le(&Number::Num(a), &Number::Num(b)), a <= b,);
    }
}
