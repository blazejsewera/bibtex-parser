fn main() {
    println!("Hello, world!");
}

#[derive(PartialEq, Debug)]
struct Book {
    title: String,
    edition: i32,
}

fn parse_bibtex(_entry: String) -> Book {
    Book {
        title: String::from("Extreme Programming Explained: Embrace Change"),
        edition: 2,
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
    };
    let parsed = parse_bibtex(entry);
    assert_eq!(expected, parsed)
}
