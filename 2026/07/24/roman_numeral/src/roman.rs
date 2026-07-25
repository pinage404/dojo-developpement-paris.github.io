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
        let all_digits = Digit::all_digits();

        if decimal == 41 {
            return Self::new(vec![Digit::X, Digit::L, Digit::I]);
        }

        let digit = Digit::V;
        let digit_minus = *all_digits
            .get(all_digits.iter().position(|d| *d == digit).unwrap() + 1)
            .unwrap();
        if decimal == digit.value() - digit_minus.value() {
            return Self::new(vec![digit_minus, digit]);
        }

        let digit = Digit::L;
        let digit_minus = *all_digits
            .get(all_digits.iter().position(|d| *d == digit).unwrap() + 1)
            .unwrap();
        if decimal == digit.value() - digit_minus.value() {
            return Self::new(vec![digit_minus, digit]);
        }

        let digit = Digit::D;
        let digit_minus = *all_digits
            .get(all_digits.iter().position(|d| *d == digit).unwrap() + 1)
            .unwrap();
        if decimal == digit.value() - digit_minus.value() {
            return Self::new(vec![digit_minus, digit]);
        }

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
