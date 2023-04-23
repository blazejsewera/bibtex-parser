fn main() {
    println!("Hello, world!");
    parse_bibtex(String::from("hello"));
}

#[derive(PartialEq, Debug)]
struct Book {
    title: String,
    edition: i32,
    isbn: String,
    series: String,
    pagetotal: i64,
    publisher: String,
    authors: Vec<Author>,
    date: Date,
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

fn parse_bibtex(_entry: String) -> Book {
    Book {
        title: String::from("Extreme Programming Explained: Embrace Change"),
        edition: 2,
        isbn: String::from("978-0-13-405199-4"),
        series: String::from("XP Series"),
        pagetotal: 0,
        publisher: String::from("Addison-Wesley Professional"),
        authors: vec![
            Author {
                first_name: String::from("Kent"),
                middle_name: None,
                last_name: String::from("Beck"),
            },
            Author {
                first_name: String::from("Cynthia"),
                middle_name: None,
                last_name: String::from("Andres"),
            },
        ],
        date: Date::Year(2004),
    }
}

#[test]
fn parse_bibtex_entry() {
    // given
    let entry = String::from(
        r#"
        @book{beck-2004,
          title     = {Extreme Programming Explained: Embrace Change},
          edition   = {2},
          isbn      = {978-0-13-405199-4},
          series    = {{XP} Series},
          pagetotal = {189},
          publisher = {Addison-Wesley Professional},
          author    = {Beck, Kent and Andres, Cynthia},
          date      = {2004},
        }"#,
    );

    let expected = Book {
        title: String::from("Extreme Programming Explained: Embrace Change"),
        edition: 2,
        isbn: String::from("978-0-13-405199-4"),
        series: String::from("XP Series"),
        pagetotal: 0,
        publisher: String::from("Addison-Wesley Professional"),
        authors: vec![
            Author {
                first_name: String::from("Kent"),
                middle_name: None,
                last_name: String::from("Beck"),
            },
            Author {
                first_name: String::from("Cynthia"),
                middle_name: None,
                last_name: String::from("Andres"),
            },
        ],
        date: Date::Year(2004),
    };

    // when
    let parsed = parse_bibtex(entry);

    // then
    assert_eq!(expected, parsed)
}
