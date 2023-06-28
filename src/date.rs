use chrono::Month;
use num_traits::FromPrimitive;

pub(crate) fn parse_month_from_str(s: &str) -> Option<Month> {
    let parsed_number = parse_month_from_number(s);
    match parsed_number {
        Some(n) => Some(n),
        None => s.parse::<Month>().ok(),
    }
}

fn parse_month_from_number(s: &str) -> Option<Month> {
    s.parse::<u8>().ok().and_then(Month::from_u8)
}

mod date_test {
    use super::*;

    #[test]
    fn parse_from_name() {
        // given
        let input = "january";
        let expected = Some(Month::January);

        // when
        let actual = parse_month_from_str(input);

        // then
        assert_eq!(actual, expected);
    }

    #[test]
    fn parse_from_number() {
        // given
        let input = "1";
        let expected = Some(Month::January);

        // when
        let actual = parse_month_from_str(input);

        // then
        assert_eq!(actual, expected);
    }
}
