use crate::tokenizer::Tokenizer;

mod entry;
mod entry_field;
mod entry_type;
mod person;
mod strings;
mod tokenizer;

fn main() {
    let reader = Box::new(EXAMPLE_ENTRY.as_bytes());
    let mut tokenizer = Tokenizer::new(reader);
    let tokens = tokenizer.tokenize();
    println!("{:#?}", tokens);
}

#[derive(PartialEq, Debug)]
struct Entry {
    symbol: String,
    title: String,
    authors: Vec<Author>,
    edition: Option<i32>,
    isbn: Option<String>,
    series: Option<String>,
    page_count: Option<i64>,
    publisher: Option<String>,
}

#[derive(PartialEq, Debug)]
struct Author {
    first_name: String,
    middle_name: Option<String>,
    last_name: String,
}

fn parse_bibtex(_entry: String) -> Result<Entry, String> {
    Ok(Entry {
        symbol: s!("beck-2004"),
        title: s!("Extreme Programming Explained: Embrace Change"),
        authors: vec![
            Author {
                first_name: s!("Kent"),
                middle_name: None,
                last_name: s!("Beck"),
            },
            Author {
                first_name: s!("Cynthia"),
                middle_name: None,
                last_name: s!("Andres"),
            },
        ],
        edition: Some(2),
        isbn: Some(s!("978-0-13-405199-4")),
        series: Some(s!("XP Series")),
        page_count: Some(0),
        publisher: Some(s!("Addison-Wesley Professional")),
    })
}

#[cfg(test)]
mod tokenizer_test {
    use super::*;

    #[test]
    fn parse_bibtex_entry() {
        // given
        let entry = s!(r#"
        @book{beck-2004,
          title     = {Extreme Programming Explained: Embrace Change},
          edition   = {2},
          isbn      = {978-0-13-405199-4},
          series    = {{XP} Series},
          pagetotal = {189},
          publisher = {Addison-Wesley Professional},
          author    = {Beck, Kent and Andres, Cynthia},
          date      = {2004},
        }"#);

        let expected = Ok(Entry {
            symbol: s!("beck-2004"),
            title: s!("Extreme Programming Explained: Embrace Change"),
            authors: vec![
                Author {
                    first_name: s!("Kent"),
                    middle_name: None,
                    last_name: s!("Beck"),
                },
                Author {
                    first_name: s!("Cynthia"),
                    middle_name: None,
                    last_name: s!("Andres"),
                },
            ],
            edition: Some(2),
            isbn: Some(s!("978-0-13-405199-4")),
            series: Some(s!("XP Series")),
            page_count: Some(0),
            publisher: Some(s!("Addison-Wesley Professional")),
        });

        // when
        let parsed = parse_bibtex(entry);

        // then
        assert_eq!(expected, parsed)
    }
}

static EXAMPLE_ENTRY: &str = r#"
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
