use std::{fmt, ops::Add};

mod digit;

#[derive(Debug, PartialEq)]
pub struct Roman {
    digits: Vec<digit::Digit>,
}

impl Roman {
    pub fn new(digits: Vec<digit::Digit>) -> Self {
        Self { digits }
    }
}

impl From<digit::Digit> for Roman {
    fn from(digit: digit::Digit) -> Self {
        Self::new(vec![digit])
    }
}

impl fmt::Display for Roman {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.digits
            .iter()
            .try_for_each(|digit| write!(f, "{}", digit))
    }
}

impl Add<Roman> for Roman {
    type Output = Roman;

    fn add(self, other: Roman) -> Self::Output {
        let mut new_digits = self.digits.clone();
        new_digits.extend(other.digits.clone());
        Roman { digits: new_digits }
    }
}

pub fn to_roman(arg: u16) -> Roman {
    if arg == 0 {
        return Roman::new(vec![]);
    }

    let digit = digit::Digit::all_digits()
        .into_iter()
        .find(|digit| arg >= digit.value())
        .unwrap_or(digit::Digit::I);
    Roman::from(digit) + to_roman(arg - digit.value())
}

#[cfg(test)]
mod test {
    use super::*;
    use speculoos::*;

    #[test]
    fn check_romans() {
        check_roman(1_000, "M");
        check_roman(100, "C");
        check_roman(2_000, "MM");
        check_roman(1_100, "MC");
        check_roman(110, "CX");
        check_roman(1, "I");
        check_roman(3, "III");
        check_roman(5, "V");
        check_roman(550, "DL");
    }

    fn check_roman(number: u16, roman: &str) {
        assert_that(&to_roman(number).to_string()).is_equal_to(String::from(roman));
    }
}
