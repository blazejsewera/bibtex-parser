use serde::Serialize;

#[derive(PartialEq, Debug, Clone, Serialize)]
pub(crate) enum Edition {
    Numeric(u32),
    Literal(String),
}

impl Edition {
    pub(crate) fn parse(s: &str) -> Edition {
        let without_trailing_dot = s
            .to_string()
            .strip_suffix('.')
            .map(|it| it.to_string())
            .unwrap_or(s.to_string());
        match without_trailing_dot.parse::<u32>() {
            Ok(i) => Edition::Numeric(i),
            Err(_) => Edition::Literal(without_trailing_dot),
        }
    }
}

#[cfg(test)]
mod edition_test {
    use super::*;
    use crate::s;

    #[test]
    fn parse_from_str() {
        // given
        vec![
            ("2", Edition::Numeric(2)),
            ("2.", Edition::Numeric(2)),
            ("Second", Edition::Literal(s!("Second"))),
        ]
        .iter()
        .for_each(|(input, expected)| {
            // when
            let actual = Edition::parse(input);

            // then
            assert_eq!(actual, *expected);
        });
    }
}
