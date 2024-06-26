use crate::s;
use serde::Serialize;

#[derive(PartialEq, Debug, Clone, Serialize)]
pub(crate) enum Person {
    FirstLast {
        first_name: String,
        last_name: String,
    },
    FirstMiddleLast {
        first_name: String,
        middle_names: Vec<String>,
        last_name: String,
    },
    FullName(String),
}

impl Person {
    const NAME_SEPARATOR: &'static str = " and ";
    const FIRST_LAST_SEPARATOR: &'static str = ", ";

    pub(crate) fn people_from_str(s: &str) -> Result<Vec<Person>, String> {
        let people_str = s.splitn(100, Self::NAME_SEPARATOR);
        people_str.map(Self::person_from_str).collect()
    }

    fn person_from_str(s: &str) -> Result<Person, String> {
        let names_str: Vec<&str> = s.splitn(2, Self::FIRST_LAST_SEPARATOR).collect();
        let (first, middle) = match names_str.get(1) {
            Some(s) => Self::first_or_first_and_middle(s),
            None => return Ok(Person::FullName(s!(s))),
        };

        let last = match names_str.first() {
            Some(s) => *s,
            None => return Err(s!("Could not parse person info")),
        };

        return if middle.is_empty() {
            Ok(Person::FirstLast {
                first_name: s!(first),
                last_name: s!(last),
            })
        } else {
            Ok(Person::FirstMiddleLast {
                first_name: s!(first),
                middle_names: middle.iter().map(|s| s!(*s)).collect(),
                last_name: s!(last),
            })
        };
    }

    fn first_or_first_and_middle(s: &str) -> (&str, Vec<&str>) {
        let names_str: Vec<&str> = s
            .split(|c| c == ' ' || c == '~' || c == '.')
            .filter(|s| !s.is_empty())
            .collect();
        let first_and_tail = names_str.split_first();
        match first_and_tail {
            Some((first, tail)) => (*first, tail.to_vec()),
            None => (s, vec![]),
        }
    }
}

#[cfg(test)]
mod person_test {
    use super::*;

    #[test]
    fn create_vec_of_four_people_from_str() {
        // given
        let input = "Gamma, Erich and Helm, Richard and Johnson, Ralph E. and Vlissides, John M.";
        let expected = Ok(vec![
            Person::FirstLast {
                first_name: s!("Erich"),
                last_name: s!("Gamma"),
            },
            Person::FirstLast {
                first_name: s!("Richard"),
                last_name: s!("Helm"),
            },
            Person::FirstMiddleLast {
                first_name: s!("Ralph"),
                middle_names: vec![s!("E")],
                last_name: s!("Johnson"),
            },
            Person::FirstMiddleLast {
                first_name: s!("John"),
                middle_names: vec![s!("M")],
                last_name: s!("Vlissides"),
            },
        ]);

        // when
        let actual = Person::people_from_str(input);

        // then
        assert_eq!(actual, expected);
    }

    #[test]
    fn create_vec_of_one_person_from_str() {
        // given
        let input = "Beck, Kent";
        let expected = Ok(vec![Person::FirstLast {
            first_name: s!("Kent"),
            last_name: s!("Beck"),
        }]);

        // when
        let actual = Person::people_from_str(input);

        // then
        assert_eq!(actual, expected);
    }

    #[test]
    fn single_person_first_last_from_str() {
        let input = "Beck, Kent";
        let expected = Ok(Person::FirstLast {
            first_name: s!("Kent"),
            last_name: s!("Beck"),
        });

        // when
        let actual = Person::person_from_str(input);

        // then
        assert_eq!(actual, expected);
    }

    #[test]
    fn single_person_first_middle_last_from_str() {
        let input = "Martin, Robert C.";
        let expected = Ok(Person::FirstMiddleLast {
            first_name: s!("Robert"),
            middle_names: vec![s!("C")],
            last_name: s!("Martin"),
        });

        // when
        let actual = Person::person_from_str(input);

        // then
        assert_eq!(actual, expected);
    }

    #[test]
    fn get_first_and_middle() {
        // given
        vec![
            ("Kent", ("Kent", vec![])),
            ("Robert Cecil", ("Robert", vec!["Cecil"])),
            ("Robert C.", ("Robert", vec!["C"])),
            ("R. C.", ("R", vec!["C"])),
            ("R.~C.", ("R", vec!["C"])),
            ("R.C.", ("R", vec!["C"])),
            ("J.R.R.", ("J", vec!["R", "R"])),
        ]
        .iter()
        .for_each(|(input, expected)| {
            // when
            let actual = Person::first_or_first_and_middle(input);

            // then
            assert_eq!(actual, *expected);
        });
    }
}
