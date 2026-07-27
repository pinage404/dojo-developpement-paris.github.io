mod digit;

pub mod roman;

#[cfg(test)]
mod test {
    use crate::roman::Roman;
    use speculoos::*;

    mod to_roman {
        use super::*;

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
        fn four_ish() {
            check_to_roman(4, "IV");
            check_to_roman(40, "XL");
            check_to_roman(400, "CD");
        }

        #[test]
        fn four_ish_with_others_numbers() {
            check_to_roman(141, "CXLI");
            check_to_roman(3444, "MMMCDXLIV");
        }

        #[test]
        fn nine_ish() {
            check_to_roman(9, "IX");
            check_to_roman(90, "XC");
            check_to_roman(900, "CM");
        }

        #[test]
        fn nine_ish_with_others_numbers() {
            check_to_roman(191, "CXCI");
            check_to_roman(3999, "MMMCMXCIX");
        }

        fn check_to_roman(number: u16, roman: &str) {
            assert_that(&Roman::from(number).to_string()).is_equal_to(String::from(roman));
        }
    }

    mod from_roman {
        use speculoos::result::ResultAssertions;

        use super::*;

        #[test]
        fn simple_number() {
            check_from_roman("I", 1);
            check_from_roman("X", 10);
        }

        #[test]
        fn several_numbers() {
            check_from_roman("XI", 11);
        }

        #[test]
        fn four_ish() {
            check_from_roman("IV", 4);
            check_from_roman("MMMCMXCIX", 3999);
        }

        #[test]
        fn can_not_parse_invalid_roman_number() {
            assert_that(&Roman::try_from("invalid")).is_err();
        }

        fn check_from_roman(roman: &str, number: u16) {
            assert_that(&u16::from(Roman::try_from(roman).unwrap())).is_equal_to(number);
        }
    }

    mod both_way {
        use quickcheck_macros::quickcheck;

        use super::*;

        #[quickcheck]
        fn decimal_roman_decimal(number: u16) {
            let roman_from_decimal = Roman::from(number);

            let roman_from_string =
                Roman::try_from(roman_from_decimal.to_string().as_str()).unwrap();

            let decimal_from_roman = u16::from(roman_from_string);

            assert_that(&decimal_from_roman).is_equal_to(number);
        }
    }
}
