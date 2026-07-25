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
            Digit::M => 1_000,
            Digit::D => 500,
            Digit::C => 100,
            Digit::L => 50,
            Digit::X => 10,
            Digit::V => 5,
            Digit::I => 1,
        }
    }

    pub(crate) fn all_digits() -> Vec<Digit> {
        vec![
            Digit::M,
            Digit::D,
            Digit::C,
            Digit::L,
            Digit::X,
            Digit::V,
            Digit::I,
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
