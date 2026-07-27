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

    fn find_number_ish(decimal: u16) -> impl Fn((Digit, Digit)) -> Option<Roman> {
        move |(digit_minus, digit)| {
            (decimal >= digit - digit_minus && decimal < digit).then(|| {
                Self::new(vec![digit_minus, digit]) + Self::from(decimal - (digit - digit_minus))
            })
        }
    }

    fn find_four_ish(decimal: u16) -> Option<Roman> {
        let all_digits = Digit::all_digits();

        Digit::dozens()
            .into_iter()
            .filter_map(|digit_minus| {
                all_digits
                    .iter()
                    .position(|d| d == &digit_minus)
                    .map(|digit_minus_position| (digit_minus, digit_minus_position))
            })
            .filter(|(_digit_minus, digit_minus_position)| digit_minus_position != &0)
            .filter_map(|(digit_minus, digit_minus_position)| {
                (*all_digits)
                    .get(digit_minus_position - 1)
                    .map(|&digit| (digit_minus, digit))
            })
            .find_map(Self::find_number_ish(decimal))
    }

    fn find_nine_ish(decimal: u16) -> Option<Roman> {
        let dozens = Digit::dozens();

        dozens
            .clone()
            .into_iter()
            .filter_map(|digit_minus| {
                dozens
                    .iter()
                    .position(|d| *d == digit_minus)
                    .map(|digit_minus_position| (digit_minus, digit_minus_position))
            })
            .filter(|(_digit_minus, digit_minus_position)| digit_minus_position != &0)
            .filter_map(|(digit_minus, digit_minus_position)| {
                (*dozens)
                    .get(digit_minus_position - 1)
                    .map(|&digit| (digit_minus, digit))
            })
            .find_map(Self::find_number_ish(decimal))
    }

    fn find_normal_number(decimal: u16) -> Option<Roman> {
        Digit::all_digits().into_iter().find_map(|digit| {
            (decimal >= digit).then(|| Self::new(vec![digit]) + Self::from(decimal - digit))
        })
    }
}

impl From<u16> for Roman {
    fn from(decimal: u16) -> Self {
        Roman::find_four_ish(decimal)
            .or_else(|| Roman::find_nine_ish(decimal))
            .or_else(|| Roman::find_normal_number(decimal))
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
