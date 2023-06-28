#[derive(PartialEq, Debug)]
pub(crate) enum Pages {
    Single(Page),
    Range(Page, Page),
}

#[derive(PartialEq, Debug)]
pub(crate) enum Page {
    Numeric(u32),
    Literal(String),
}

impl Pages {
    pub(crate) fn pages_from_str(s: &str) -> Vec<Pages> {
        vec![]
    }
}

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
}
