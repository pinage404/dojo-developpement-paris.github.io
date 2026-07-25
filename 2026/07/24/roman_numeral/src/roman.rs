use std::{fmt, ops::Add};

use crate::digit::Digit;

#[derive(Debug, PartialEq, Default)]
pub struct Roman {
    digits: Vec<Digit>,
}

impl Roman {
    fn new(digits: Vec<Digit>) -> Self {
        Self { digits }
    }
}

impl From<u16> for Roman {
    fn from(decimal: u16) -> Self {
        let digit = Digit::V;
        let digit_minus = Digit::I;
        if decimal == digit.value() - digit_minus.value() {
            return Self::new(vec![digit_minus, digit]);
        }

        if decimal == Digit::L.value() - Digit::X.value() {
            return Self::new(vec![Digit::X, Digit::L]);
        }

        if decimal == Digit::D.value() - Digit::C.value() {
            return Self::new(vec![Digit::C, Digit::D]);
        }

        let all_digits = Digit::all_digits();
        all_digits
            .into_iter()
            .find_map(|digit| {
                (decimal >= digit).then(|| Self::new(vec![digit]) + Self::from(decimal - digit))
            })
            .unwrap_or(Self::default())
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
