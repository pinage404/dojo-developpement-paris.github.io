mod digit;

mod roman;

#[cfg(test)]
mod test {
    use super::*;
    use crate::roman::Roman;
    use speculoos::*;

    #[test]
    fn tens() {
        check_roman(1_000, "M");
        check_roman(100, "C");
        check_roman(10, "X");
        check_roman(1, "I");
    }

    #[test]
    fn fives() {
        check_roman(500, "D");
        check_roman(50, "L");
        check_roman(5, "V");
    }

    #[test]
    fn several_time_concatenate() {
        check_roman(2_000, "MM");
        check_roman(3, "III");
    }

    #[test]
    fn concatenate_symbol() {
        check_roman(1_100, "MC");
        check_roman(110, "CX");
        check_roman(550, "DL");
        check_roman(3333, "MMMCCCXXXIII");
    }

    fn check_roman(number: u16, roman: &str) {
        assert_that(&Roman::from(number).to_string()).is_equal_to(String::from(roman));
    }
}
