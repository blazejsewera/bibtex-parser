use chrono::{Month, NaiveDate};
use num_traits::FromPrimitive;

pub(crate) fn parse_month_from_str(s: &str) -> Result<Month, String> {
    let parsed_number = parse_month_from_number(s);
    match parsed_number {
        Some(n) => Ok(n),
        None => s
            .parse::<Month>()
            .map_err(|_| format!("Could not parse Month from '{}'", s)),
    }
}

fn parse_month_from_number(s: &str) -> Option<Month> {
    s.parse::<u8>().ok().and_then(Month::from_u8)
}

pub(crate) fn parse_date_from_str(s: &str) -> Result<NaiveDate, String> {
    s.parse::<NaiveDate>()
        .map_err(|_| format!("Could not parse Date from '{}'", s))
}

pub(crate) fn parse_year_from_str(s: &str) -> Result<i32, String> {
    s.parse::<i32>()
        .map_err(|_| format!("Could not parse Year from '{}'", s))
}

mod date_test {
    use super::*;
    use crate::s;

    mod date {
        use super::*;

        #[test]
        fn parse_date() {
            // given
            let input = "2004-03-02";
            let expected = Ok(NaiveDate::from_ymd_opt(2004, 3, 2).unwrap());

            // when
            let actual = parse_date_from_str(input);

            // then
            assert_eq!(actual, expected);
        }

        #[test]
        fn parse_date_err_on_invalid() {
            // given
            let input = "2004-03";
            let expected = Err(s!("Could not parse Date from '2004-03'"));

            // when
            let actual = parse_date_from_str(input);

            // then
            assert_eq!(actual, expected);
        }
    }

    mod year {
        use super::*;

        #[test]
        fn parse_year() {
            // given
            let input = "2004";
            let expected = Ok(2004);

            // when
            let actual = parse_year_from_str(input);

            // then
            assert_eq!(actual, expected);
        }

        #[test]
        fn parse_year_err_on_invalid() {
            // given
            let input = "j";
            let expected = Err(s!("Could not parse Year from 'j'"));

            // when
            let actual = parse_year_from_str(input);

            // then
            assert_eq!(actual, expected);
        }
    }

    mod month {
        use super::*;

        #[test]
        fn parse_month_from_name() {
            // given
            let input = "january";
            let expected = Ok(Month::January);

            // when
            let actual = parse_month_from_str(input);

            // then
            assert_eq!(actual, expected);
        }

        #[test]
        fn parse_month_from_number() {
            // given
            let input = "1";
            let expected = Ok(Month::January);

            // when
            let actual = parse_month_from_str(input);

            // then
            assert_eq!(actual, expected);
        }

        #[test]
        fn parse_month_err_on_invalid() {
            // given
            let input = "j";
            let expected = Err(s!("Could not parse Month from 'j'"));

            // when
            let actual = parse_month_from_str(input);

            // then
            assert_eq!(actual, expected);
        }
    }
}
