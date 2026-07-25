use std::fmt;

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
        write!(
            f,
            "{}",
            match self {
                Digit::M => "M",
                Digit::D => "D",
                Digit::C => "C",
                Digit::L => "L",
                Digit::X => "X",
                Digit::V => "V",
                Digit::I => "I",
            }
        )
    }
}
