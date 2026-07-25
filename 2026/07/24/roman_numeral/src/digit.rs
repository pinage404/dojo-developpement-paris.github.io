use std::{fmt, ops::Sub};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Digit {
    M,
    D,
    C,
    L,
    X,
    V,
    I,
}

impl Digit {
    pub(crate) fn value(self) -> u16 {
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

    fn to_symbol(self) -> String {
        match self {
            Self::M => "M",
            Self::D => "D",
            Self::C => "C",
            Self::L => "L",
            Self::X => "X",
            Self::V => "V",
            Self::I => "I",
        }
        .to_string()
    }

    pub(crate) fn all_digits() -> Vec<Self> {
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
}

impl fmt::Display for Digit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_symbol())
    }
}

impl Sub<Digit> for u16 {
    type Output = u16;

    fn sub(self, other: Digit) -> Self::Output {
        self - other.value()
    }
}
