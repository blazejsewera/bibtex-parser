use crate::s;

#[derive(PartialEq, Debug, Clone)]
pub(crate) enum Pages {
    Single(Page),
    Range(Page, Page),
}

#[derive(PartialEq, Debug, Clone)]
pub(crate) enum Page {
    Numeric(u32),
    Literal(String),
}

impl Pages {
    const MULTI_SEPARATOR: &'static str = ",";
    const RANGE_SEPARATOR: [&'static str; 3] = ["-", "–", "—"];

    pub(crate) fn pages_from_str(s: &str) -> Vec<Pages> {
        s.to_string()
            .split(Self::MULTI_SEPARATOR)
            .map(|it| Self::single_or_range_from_str(it.trim()))
            .collect::<Vec<Pages>>()
    }

    fn single_or_range_from_str(s: &str) -> Pages {
        let single_or_range = Self::split_range(s);
        match single_or_range.len() {
            2 => Pages::Range(
                Self::page_from_str(single_or_range.get(0).unwrap_or(&s!(""))),
                Self::page_from_str(single_or_range.get(1).unwrap_or(&s!(""))),
            ),
            _ => Pages::Single(Self::page_from_str(s)),
        }
    }

    fn split_range(s: &str) -> Vec<String> {
        Self::RANGE_SEPARATOR
            .iter()
            .filter(|range_sep| s.to_string().contains(*range_sep))
            .flat_map(|range_sep| {
                s.to_string()
                    .split(range_sep)
                    .filter(|it| !it.is_empty())
                    .map(|it| it.trim().to_string())
                    .collect::<Vec<String>>()
            })
            .collect::<Vec<String>>()
    }

    fn page_from_str(s: &str) -> Page {
        match s.to_string().parse::<u32>() {
            Ok(i) => Page::Numeric(i),
            Err(_) => Page::Literal(s.to_string()),
        }
    }
}

#[cfg(test)]
mod pages_test {
    use super::*;
    use crate::s;

    #[test]
    fn parse_single_from_str() {
        // given
        vec![
            ("2", vec![Pages::Single(Page::Numeric(2))]),
            ("ii", vec![Pages::Single(Page::Literal(s!("ii")))]),
        ]
        .iter()
        .for_each(|(input, expected)| {
            // when
            let actual = Pages::pages_from_str(input);

            // then
            assert_eq!(actual, *expected);
        });
    }

    #[test]
    fn parse_numerical_range_from_str() {
        // given
        vec![
            "2-4", "2--4", "2---4", "2 - 4", "2 -- 4", "2 --- 4", "2–4", "2—4", "2 – 4", "2 — 4",
        ]
        .iter()
        .for_each(|input| {
            let expected = vec![Pages::Range(Page::Numeric(2), Page::Numeric(4))];

            // when
            let actual = Pages::pages_from_str(input);

            // then
            assert_eq!(actual, expected);
        });
    }

    #[test]
    fn parse_literal_range_from_str() {
        // given
        let input = "ii - iv";
        let expected = vec![Pages::Range(
            Page::Literal(s!("ii")),
            Page::Literal(s!("iv")),
        )];

        // when
        let actual = Pages::pages_from_str(input);

        // then
        assert_eq!(actual, expected);
    }

    #[test]
    fn parse_multiple_from_str() {
        // given
        let input = "ii--iv, 12";
        let expected = vec![
            Pages::Range(Page::Literal(s!("ii")), Page::Literal(s!("iv"))),
            Pages::Single(Page::Numeric(12)),
        ];

        // when
        let actual = Pages::pages_from_str(input);

        // then
        assert_eq!(actual, expected);
    }

    #[test]
    fn parse_pages_single() {
        // given
        let input = "12";
        let expected = Pages::Single(Page::Numeric(12));

        // when
        let actual = Pages::single_or_range_from_str(input);

        // then
        assert_eq!(actual, expected);
    }

    #[test]
    fn split_range() {
        // given
        vec![
            "2-4", "2--4", "2---4", "2 - 4", "2 -- 4", "2 --- 4", "2–4", "2—4", "2 – 4", "2 — 4",
        ]
        .iter()
        .for_each(|input| {
            let expected = vec![s!("2"), s!("4")];

            // when
            let actual = Pages::split_range(input);

            // then
            assert_eq!(actual, expected);
        });
    }

    #[test]
    fn parse_numeric_page() {
        // given
        let input = "12";
        let expected = Page::Numeric(12);

        // when
        let actual = Pages::page_from_str(input);

        // then
        assert_eq!(actual, expected);
    }
}
