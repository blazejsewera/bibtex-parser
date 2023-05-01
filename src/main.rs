mod strings;
mod tokenizer;

fn main() {
    println!("Hello, world!");
    parse_bibtex(s!("hello"));
}

#[derive(Debug, PartialEq)]
enum EntryType {
    Book,
    Other(String),
}

impl EntryType {
    fn from_str(s: &str) -> EntryType {
        match s {
            "book" => EntryType::Book,
            s => EntryType::Other(String::from(s)),
        }
    }
}

#[derive(Debug, PartialEq)]
enum EntryProperty {
    Title,
    Author,
    Date,
    Edition,
    Isbn,
    Series,
    PageTotal,
    Publisher,
    Other(String),
}

impl EntryProperty {
    fn from_str(s: &str) -> EntryProperty {
        match s {
            "title" => EntryProperty::Title,
            "author" => EntryProperty::Author,
            "date" => EntryProperty::Date,
            "edition" => EntryProperty::Edition,
            "isbn" => EntryProperty::Isbn,
            "series" => EntryProperty::Series,
            "pagetotal" => EntryProperty::PageTotal,
            "publisher" => EntryProperty::Publisher,
            s => EntryProperty::Other(String::from(s)),
        }
    }
}

#[derive(PartialEq, Debug)]
struct Book {
    symbol: String,
    title: String,
    authors: Vec<Author>,
    date: Option<Date>,
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

#[derive(PartialEq, Debug)]
enum Date {
    Year(i32),
    YearMonth(i32, time::Month),
    YearMonthDay(i32, time::Month, i8),
}

fn parse_bibtex(_entry: String) -> Result<Book, String> {
    Ok(Book {
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
        date: Some(Date::Year(2004)),
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

        let expected = Ok(Book {
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
            date: Some(Date::Year(2004)),
        });

        // when
        let parsed = parse_bibtex(entry);

        // then
        assert_eq!(expected, parsed)
    }
}
