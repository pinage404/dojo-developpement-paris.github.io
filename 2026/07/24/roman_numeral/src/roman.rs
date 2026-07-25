use std::{fmt, ops::Add};

use crate::digit::Digit;

type Digits = Vec<Digit>;

#[derive(Debug, PartialEq, Default)]
pub struct Roman {
    digits: Digits,
}

impl Roman {
    fn new(digits: Vec<Digit>) -> Self {
        Self { digits }
    }
}

impl From<Digit> for Roman {
    fn from(digit: Digit) -> Self {
        Self::new(vec![digit])
    }
}

impl From<u16> for Roman {
    fn from(decimal: u16) -> Self {
        if decimal == 0 {
            return Self::default();
        }

        if decimal == 5 - 1 {
            return Self::new(vec![Digit::I, Digit::V]);
        }

        if decimal == 50 - 10 {
            return Self::new(vec![Digit::X, Digit::L]);
        }

        if decimal == 500 - 100 {
            return Self::new(vec![Digit::C, Digit::D]);
        }

        let digit = Digit::all_digits()
            .into_iter()
            .find(|digit| decimal >= digit)
            .unwrap_or(Digit::I);

        Self::from(digit) + Self::from(decimal - digit)
    }
}

impl fmt::Display for Roman {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.digits
            .iter()
            .try_for_each(|digit| write!(f, "{}", digit))
    }
}

impl Add for Roman {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        let mut digits = self.digits;
        digits.extend(other.digits);
        Self::new(digits)
    }
}
