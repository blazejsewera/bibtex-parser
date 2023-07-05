use chrono::Month;
use num_traits::FromPrimitive;

#[derive(PartialEq, Debug, Copy, Clone)]
pub(crate) enum Date {
    YearMonthDay(i32, u8, u8),
    YearMonth(i32, u8),
    Year(i32),
    Month(u8),
}

impl Date {
    pub(crate) fn parse_month_from_str(s: &str) -> Result<Date, String> {
        let parsed_number = Self::parse_month_from_number(s);
        match parsed_number {
            Some(n) => Ok(n),
            None => s
                .parse::<Month>()
                .map_err(|_| format!("Could not parse Month from '{}'", s)),
        }
        .map(Self::month_into_date)
    }

    fn parse_month_from_number(s: &str) -> Option<Month> {
        s.parse::<u8>().ok().and_then(Month::from_u8)
    }

    fn month_into_date(month: Month) -> Date {
        Date::Month(month.number_from_month() as u8)
    }

    pub(crate) fn parse_date_from_str(s: &str) -> Result<Date, String> {
        let sections = s.split('-').collect::<Vec<&str>>();
        match sections[..] {
            [year] => Self::parse_year_from_str(year),
            [year, month] => Self::parse_year_month_from_str(year, month),
            [year, month, day] => Self::parse_year_month_day_from_str(year, month, day),
            _ => Err(format!("Could not parse Date from '{}'", s)),
        }
    }

    pub(crate) fn parse_year_from_str(s: &str) -> Result<Date, String> {
        s.parse::<i32>()
            .map(Date::Year)
            .map_err(|_| format!("Could not parse Year from '{}'", s))
    }

    fn parse_year_month_from_str(year: &str, month: &str) -> Result<Date, String> {
        let parsed_year = match Self::parse_year_from_str(year)? {
            Date::Year(y) => y,
            _ => panic!("Unreachable code when parsing Year"),
        };
        let parsed_month = match Self::parse_month_from_str(month)? {
            Date::Month(m) => m,
            _ => panic!("Unreachable code when parsing Month"),
        };
        Ok(Date::YearMonth(parsed_year, parsed_month))
    }

    fn parse_year_month_day_from_str(year: &str, month: &str, day: &str) -> Result<Date, String> {
        let (parsed_year, parsed_month) = match Self::parse_year_month_from_str(year, month)? {
            Date::YearMonth(y, m) => (y, m),
            _ => panic!("Unreachable code when parsing Year and Month"),
        };
        let parsed_day = day
            .parse::<u8>()
            .map_err(|_| format!("Could not parse Day from '{}'", day))?;
        match parsed_day {
            d if d > 31 => Err(format!("Day must be at most 31, but was {}", parsed_day)),
            _ => Ok(Date::YearMonthDay(parsed_year, parsed_month, parsed_day)),
        }
    }

    #[allow(dead_code)]
    fn merge(date1: Date, date2: Date) -> Date {
        match date1 {
            Date::Year(y1) => match date2 {
                Date::Month(m2) => Date::YearMonth(y1, m2),
                _ => date2,
            },
            Date::Month(m1) => match date2 {
                Date::Year(y2) => Date::YearMonth(y2, m1),
                _ => date2,
            },
            _ => date1,
        }
    }
}

#[cfg(test)]
mod date_test {
    use super::*;

    #[test]
    fn parse_date_full() {
        // given
        let input = "2004-03-02";
        let expected = Ok(Date::YearMonthDay(2004, 3, 2));

        // when
        let actual = Date::parse_date_from_str(input);

        // then
        assert_eq!(actual, expected);
    }

    #[test]
    fn parse_date_year_month() {
        // given
        let input = "2004-03";
        let expected = Ok(Date::YearMonth(2004, 3));

        // when
        let actual = Date::parse_date_from_str(input);

        // then
        assert_eq!(actual, expected);
    }

    #[test]
    fn parse_date_year_only() {
        // given
        let input = "2004";
        let expected = Ok(Date::Year(2004));

        // when
        let actual = Date::parse_date_from_str(input);

        // then
        assert_eq!(actual, expected);
    }

    #[test]
    fn parse_date_month() {
        // given
        let expected = Ok(Date::Month(3));
        vec!["03", "3", "mar", "march", "Mar", "March"]
            .iter()
            .for_each(|input| {
                // when
                let actual = Date::parse_month_from_str(input);

                // then
                assert_eq!(actual, expected);
            });
    }

    #[test]
    fn parse_date_invalid_month() {
        // given
        let input = "42";

        // when
        let actual = Date::parse_month_from_str(input);

        // then
        assert!(actual.is_err());
    }

    #[test]
    fn merge() {
        // given
        vec![
            ((Date::Year(2004), Date::Month(3)), Date::YearMonth(2004, 3)),
            ((Date::Month(3), Date::Year(2004)), Date::YearMonth(2004, 3)),
            (
                (Date::YearMonth(2004, 3), Date::Year(2005)),
                Date::YearMonth(2004, 3),
            ),
        ]
        .iter()
        .for_each(|((date1, date2), expected)| {
            // when
            let actual = Date::merge(*date1, *date2);

            // then
            assert_eq!(actual, *expected);
        });
    }
}
