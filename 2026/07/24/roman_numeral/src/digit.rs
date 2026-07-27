use std::{
    cmp, fmt,
    ops::{Add, Sub},
};

#[derive(Debug, PartialEq, Clone, Copy)]
pub(super) enum Digit {
    M,
    D,
    C,
    L,
    X,
    V,
    I,
}

impl Digit {
    pub(super) fn all_digits() -> Vec<Self> {
        vec![
            Self::M,
            Self::D,
            Self::C,
            Self::L,
            Self::X,
            Self::V,
            Self::I,
        ]
    }

    pub(super) fn dozens() -> Vec<Self> {
        Self::all_digits()
            .into_iter()
            .filter(|digit| is_dozen(digit.value()))
            .collect()
    }
}

fn is_dozen(number: u16) -> bool {
    if number > 10 {
        is_dozen(number / 10)
    } else {
        number.is_multiple_of(10) || number == 1
    }
}

impl Digit {
    pub(super) fn value(self) -> u16 {
        match self {
            Self::M => 1_000,
            Self::D => 500,
            Self::C => 100,
            Self::L => 50,
            Self::X => 10,
            Self::V => 5,
            Self::I => 1,
        }
    }

    fn to_symbol(self) -> char {
        match self {
            Self::M => 'M',
            Self::D => 'D',
            Self::C => 'C',
            Self::L => 'L',
            Self::X => 'X',
            Self::V => 'V',
            Self::I => 'I',
        }
    }
}

impl fmt::Display for Digit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_symbol())
    }
}

impl Add<Digit> for u16 {
    type Output = u16;

    fn add(self, digit: Digit) -> Self::Output {
        self + digit.value()
    }
}

impl Add<Digit> for i32 {
    type Output = i32;

    fn add(self, digit: Digit) -> Self::Output {
        self + i32::from(digit.value())
    }
}

impl Sub<Digit> for u16 {
    type Output = u16;

    fn sub(self, other: Digit) -> Self::Output {
        self - other.value()
    }
}

impl Sub<Digit> for i32 {
    type Output = i32;

    fn sub(self, other: Digit) -> Self::Output {
        self - i32::from(other.value())
    }
}

impl Sub<Digit> for Digit {
    type Output = u16;

    fn sub(self, other: Digit) -> Self::Output {
        self.value() - other.value()
    }
}

impl PartialEq<Digit> for u16 {
    fn eq(&self, other: &Digit) -> bool {
        *self == other.value()
    }
}

impl PartialOrd<Digit> for u16 {
    fn partial_cmp(&self, other: &Digit) -> Option<cmp::Ordering> {
        self.partial_cmp(&other.value())
    }
}

impl PartialOrd<Digit> for Digit {
    fn partial_cmp(&self, other: &Digit) -> Option<cmp::Ordering> {
        self.value().partial_cmp(&other.value())
    }
}

impl TryFrom<char> for Digit {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'M' => Ok(Self::M),
            'D' => Ok(Self::D),
            'C' => Ok(Self::C),
            'L' => Ok(Self::L),
            'X' => Ok(Self::X),
            'V' => Ok(Self::V),
            'I' => Ok(Self::I),
            _ => Err(format!("Invalid roman number : {value}")),
        }
    }
}
