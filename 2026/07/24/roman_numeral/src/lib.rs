mod digit;

pub mod roman;

#[cfg(test)]
mod test {
    use crate::roman::Roman;
    use speculoos::*;

    #[test]
    fn tens() {
        check_to_roman(1_000, "M");
        check_to_roman(100, "C");
        check_to_roman(10, "X");
        check_to_roman(1, "I");
    }

    #[test]
    fn fives() {
        check_to_roman(500, "D");
        check_to_roman(50, "L");
        check_to_roman(5, "V");
    }

    #[test]
    fn several_time_concatenate() {
        check_to_roman(2_000, "MM");
        check_to_roman(3, "III");
    }

    #[test]
    fn concatenate_symbol() {
        check_to_roman(1_100, "MC");
        check_to_roman(110, "CX");
        check_to_roman(550, "DL");
        check_to_roman(3333, "MMMCCCXXXIII");
    }

    #[test]
    fn fourth() {
        check_to_roman(4, "IV");
        check_to_roman(40, "XL");
        check_to_roman(400, "CD");
    }

    #[test]
    fn fourth_with_others_numbers() {
        check_to_roman(141, "CXLI");
    }

    fn check_to_roman(number: u16, roman: &str) {
        assert_that(&Roman::from(number).to_string()).is_equal_to(String::from(roman));
    }
}
