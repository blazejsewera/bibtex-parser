use crate::s;
use std::fs::read;

#[derive(PartialEq, Debug)]
pub(crate) enum Person {
    FirstLast {
        first_name: String,
        last_name: String,
    },
    FirstMiddleLast {
        first_name: String,
        middle_name: String,
        last_name: String,
    },
    FullName(String),
}

const NAME_SEPARATOR: &str = " and ";
const FIRST_LAST_SEPARATOR: &str = ", ";

impl Person {
    pub(crate) fn people_from_str(s: &str) -> Vec<Person> {
        let mut people = Vec::<Person>::new();
        let people_str = s.splitn(100, NAME_SEPARATOR);
        people_str.for_each(|person_str| {});
        vec![]
    }

    fn person_from_str(s: &str) -> Result<Person, &str> {
        let names_str: Vec<&str> = s.splitn(2, FIRST_LAST_SEPARATOR).collect();
        let first_and_middle = match names_str.get(1) {
            Some(s) => *s,
            None => return Ok(Person::FullName(s!(s))),
        };

        let last = match names_str.first() {
            Some(s) => *s,
            None => return Err("Could not parse person info"),
        };

        Ok(Person::FirstLast {
            first_name: s!(first_and_middle),
            last_name: s!(last),
        })
    }

    fn first_or_first_and_middle(s: &str) -> (&str, Option<&str>) {
        let names_str: Vec<&str> = s.split(|c| c == ' ' || c == '~').collect();
        let middle = match names_str.get(1) {
            Some(s) => *s,
            None => {
                let names = s.split('.').collect::<Vec<&str>>().split_first();
                ""
            }
        };

        let first = match names_str.first() {
            Some(s) => *s,
            None => "",
        };

        (first, Some(middle))
    }
}

#[cfg(test)]
mod person_test {
    use super::*;

    #[ignore]
    #[test]
    fn create_vec_of_one_person_from_str() {
        // given
        let input = "Beck, Kent";
        let expected = vec![Person::FirstLast {
            first_name: s!("Kent"),
            last_name: s!("Beck"),
        }];

        // when
        let actual = Person::people_from_str(input);

        // then
        assert_eq!(actual, expected);
    }

    #[test]
    fn first_last_from_str() {
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
    fn first_middle_last_from_str() {
        let input = "Martin, Robert C.";
        let expected = Ok(Person::FirstMiddleLast {
            first_name: s!("Robert"),
            middle_name: s!("C."),
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
            ("Kent", ("Kent", None)),
            ("Robert Cecil", ("Robert", Some("Cecil"))),
            ("Robert C.", ("Robert", Some("C."))),
            ("R. C.", ("R.", Some("C."))),
            ("R.~C.", ("R.", Some("C."))),
            ("R.C.", ("R.", Some("C."))),
            ("J.R.R.", ("J.", Some("R. R."))),
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
