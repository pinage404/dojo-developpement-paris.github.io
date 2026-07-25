use std::{fmt, ops::Add};

use crate::roman::digits::Digits;

mod digits;

#[derive(Debug, PartialEq, Default)]
pub struct Roman {
    digits: Vec<Digits>,
}

impl Roman {
    fn new(digits: Vec<Digits>) -> Self {
        Self { digits }
    }
}

impl From<Digits> for Roman {
    fn from(digit: Digits) -> Self {
        Self::new(vec![digit])
    }
}

impl From<u16> for Roman {
    fn from(decimal: u16) -> Self {
        if decimal == 0 {
            return Self::default();
        }

        if decimal == 5 - 1 {
            return Self::new(vec![Digits::I, Digits::V]);
        }

        if decimal == 50 - 10 {
            return Self::new(vec![Digits::X, Digits::L]);
        }

        if decimal == 500 - 100 {
            return Self::new(vec![Digits::C, Digits::D]);
        }

        let digit = Digits::all_digits()
            .into_iter()
            .find(|digit| decimal >= digit)
            .unwrap_or(Digits::I);

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
