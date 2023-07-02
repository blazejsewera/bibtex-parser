use crate::entry_field::EntryField;
use crate::entry_type::EntryType;

#[derive(PartialEq, Debug)]
pub(crate) struct Entry {
    r#type: EntryType,
    symbol: String,
    fields: Vec<EntryField>,
}

impl Entry {
    pub(crate) fn parse_entries(input: &str) -> Result<Vec<Entry>, String> {
        todo!()
    }
}

#[cfg(test)]
mod entry_test {
    use super::*;
    use crate::edition::Edition;
    use crate::person::Person;
    use crate::s;

    #[test]
    fn parse_entries() {
        // given
        let input = r#"
            @book{beck-2004,
                title     = {Extreme Programming Explained: Embrace Change},
                edition   = {2},
                isbn      = {978-0-13-405199-4},
                series    = {{XP} Series},
                pagetotal = {189},
                publisher = {Addison-Wesley Professional},
                author    = {Beck, Kent and Andres, Cynthia},
                year      = {2004},
            }"#;
        let expected = Ok(vec![Entry {
            r#type: EntryType::Book,
            symbol: s!("beck-2004"),
            fields: vec![
                EntryField::Title(s!("Extreme Programming Explained: Embrace Change")),
                EntryField::Edition(Edition::Numeric(2)),
                EntryField::Isbn(s!("978-0-13-405199-4")),
                EntryField::Series(s!("XP Series")),
                EntryField::PageTotal(189),
                EntryField::Publisher(s!("Addison-Wesley Professional")),
                EntryField::Author(vec![
                    Person::FirstLast {
                        first_name: s!("Kent"),
                        last_name: s!("Beck"),
                    },
                    Person::FirstLast {
                        first_name: s!("Cynthia"),
                        last_name: s!("Andres"),
                    },
                ]),
                EntryField::Year(2004),
            ],
        }]);

        // when
        let actual = Entry::parse_entries(input);

        // then
        assert_eq!(actual, expected);
    }

    #[test]
    fn parse_entries_err_on_invalid() {
        // given
        let without_symbol = r#"
            @online{
                title = "a",
            }
            "#;

        // when
        let actual = Entry::parse_entries(without_symbol);

        // then
        assert!(actual.is_err());
    }
}
