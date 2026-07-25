mod digit;

mod roman;

pub fn to_roman(arg: u16) -> roman::Roman {
    if arg == 0 {
        return roman::Roman::new(vec![]);
    }

    let digit = digit::Digit::all_digits()
        .into_iter()
        .find(|digit| arg >= digit.value())
        .unwrap_or(digit::Digit::I);
    roman::Roman::from(digit) + to_roman(arg - digit.value())
}

#[cfg(test)]
mod test {
    use super::*;
    use speculoos::*;

    #[test]
    fn tens() {
        check_roman(1_000, "M");
        check_roman(100, "C");
        check_roman(10, "X");
        check_roman(1, "I");
    }

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
