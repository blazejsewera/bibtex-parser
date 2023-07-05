use crate::entry_field::EntryField;
use crate::entry_type::EntryType;
use crate::s;
use crate::tokenizer::{EntryToken, Tokenizer};
use std::io::Read;
use std::slice::Iter;

#[derive(PartialEq, Debug)]
pub(crate) struct Entry {
    r#type: EntryType,
    symbol: String,
    fields: Vec<EntryField>,
}

impl Entry {
    fn new(t: EntryType, symbol: String, fields: Vec<EntryField>) -> Entry {
        Entry {
            r#type: t,
            symbol,
            fields,
        }
    }
}

pub(crate) struct Parser {
    tokenizer: Tokenizer,
}

impl Parser {
    pub(crate) fn new(tokenizer: Tokenizer) -> Parser {
        Parser { tokenizer }
    }
    pub(crate) fn parse(&mut self) -> Result<Vec<Entry>, String> {
        let tokens = self.tokenizer.tokenize()?;
        let mut tokens_iter = tokens.iter();
        let mut entries: Vec<Entry> = vec![];

        while let Some(entry) = Self::parse_entry(&mut tokens_iter)? {
            entries.push(entry);
        }

        Ok(entries)
    }

    fn parse_entry(tokens: &mut Iter<EntryToken>) -> Result<Option<Entry>, String> {
        let mut entry_type: Option<EntryType> = None;
        let mut symbol: Option<String> = None;
        let mut fields: Vec<EntryField> = vec![];

        let mut field_name: Option<String> = None;

        for token in tokens.by_ref() {
            match token {
                EntryToken::Type(t) => match entry_type {
                    None => entry_type = Some(EntryType::from_str(t.as_str())),
                    _ => break,
                },
                EntryToken::Symbol(s) => match symbol {
                    None => symbol = Some(s.clone()),
                    _ => {
                        return Err(s!("Symbol was duplicated in a single Entry \
                                       or Type is missing in another Entry"))
                    }
                },
                EntryToken::FieldName(f) => match field_name.clone() {
                    None => field_name = Some(f.clone()),
                    Some(old) => {
                        return Err(format!(
                            "Field Name occurred twice in a row \
                             in an Entry - Value was missing for '{}'",
                            old.as_str()
                        ))
                    }
                },
                EntryToken::Value(v) => match field_name.clone() {
                    Some(f) => {
                        fields.push(EntryField::from_field_name_and_value(
                            f.as_str(),
                            v.as_str(),
                        )?);
                        field_name = None;
                    }
                    None => {
                        return Err(format!(
                            "Value does not have a preceding Field Name \
                             in an Entry - Field Name was missing for '{}'",
                            v.as_str()
                        ))
                    }
                },
            }
        }

        let t = match entry_type {
            Some(t) => t,
            None => return Ok(None),
        };
        let s = symbol.ok_or(s!("Symbol was missing from Entry"))?;

        Ok(Some(Entry::new(t, s, fields)))
    }
}

#[cfg(test)]
mod entry_test {
    use super::*;
    use crate::date::Date;
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
                date      = {2004},
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
                EntryField::Date(Date::Year(2004)),
            ],
        }]);

        // when
        let mut parser = parser_for_str(input);
        let actual = parser.parse();

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
        let mut parser = parser_for_str(without_symbol);
        let actual = parser.parse();

        // then
        assert!(actual.is_err());
    }

    fn parser_for_str(input: &'static str) -> Parser {
        let reader = reader_from_str(input);
        Parser::new(Tokenizer::new(reader))
    }

    fn reader_from_str(s: &str) -> Box<dyn Read + '_> {
        Box::new(s.as_bytes())
    }
}
