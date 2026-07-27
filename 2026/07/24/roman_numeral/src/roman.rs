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

    fn find_four_ish(decimal: u16) -> Option<Roman> {
        let all_digits = Digit::all_digits();

        Digit::dozens().clone().into_iter().find_map(|digit_minus| {
            let digit_minus_position = all_digits.iter().position(|d| *d == digit_minus).unwrap();
            if digit_minus_position == 0 {
                return None;
            }
            let digit = *all_digits.get(digit_minus_position - 1).unwrap();
            (decimal >= digit - digit_minus && decimal < digit).then(|| {
                Self::new(vec![digit_minus, digit]) + Self::from(decimal - (digit - digit_minus))
            })
        })
    }

    fn find_nine_ish(decimal: u16) -> Option<Roman> {
        let dozens = Digit::dozens();
        dozens.clone().into_iter().find_map(|digit_minus| {
            let digit_minus_position = dozens.iter().position(|d| *d == digit_minus).unwrap();
            if digit_minus_position == 0 {
                return None;
            }
            let digit = *dozens.get(digit_minus_position - 1).unwrap();
            (decimal >= digit - digit_minus && decimal < digit).then(|| {
                Self::new(vec![digit_minus, digit]) + Self::from(decimal - (digit - digit_minus))
            })
        })
    }
}

impl From<u16> for Roman {
    fn from(decimal: u16) -> Self {
        Roman::find_four_ish(decimal)
            .or_else(|| Roman::find_nine_ish(decimal))
            .or_else(|| {
                Digit::all_digits().into_iter().find_map(|digit| {
                    (decimal >= digit).then(|| Self::new(vec![digit]) + Self::from(decimal - digit))
                })
            })
            .unwrap_or_default()
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
