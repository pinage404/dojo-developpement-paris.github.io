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

        let all_digit_minus = [Digit::I];
        all_digit_minus
            .iter()
            .find_map(|digit_minus| {
                let digit_minus = *digit_minus;
                let digit = *all_digits
                    .get(all_digits.iter().position(|d| *d == digit_minus).unwrap() - 1)
                    .unwrap();
                (decimal >= digit - digit_minus && decimal < digit).then(|| {
                    Self::new(vec![digit_minus, digit])
                        + Self::from(decimal - (digit - digit_minus))
                })
            })
            .unwrap_or_else(|| {
                let digit_minus = Digit::X;
                let digit = *all_digits
                    .get(all_digits.iter().position(|d| *d == digit_minus).unwrap() - 1)
                    .unwrap();
                if decimal >= digit - digit_minus && decimal < digit {
                    return Self::new(vec![digit_minus, digit])
                        + Self::from(decimal - (digit - digit_minus));
                }

                let digit_minus = Digit::C;
                let digit = *all_digits
                    .get(all_digits.iter().position(|d| *d == digit_minus).unwrap() - 1)
                    .unwrap();
                if decimal >= digit - digit_minus && decimal < digit {
                    return Self::new(vec![digit_minus, digit])
                        + Self::from(decimal - (digit - digit_minus));
                }

                all_digits
                    .into_iter()
                    .find_map(|digit| {
                        (decimal >= digit)
                            .then(|| Self::new(vec![digit]) + Self::from(decimal - digit))
                    })
                    .unwrap_or(Self::default())
            })
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
