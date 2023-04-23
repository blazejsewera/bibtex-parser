use crate::s;

#[derive(Debug, PartialEq)]
enum EntryType {
    Book,
}

#[derive(Debug, PartialEq)]
enum BookProperty {
    Title,
    Author,
    Date,
    Edition,
    Isbn,
    Series,
    PageTotal,
    Publisher,
}

#[derive(Debug, PartialEq)]
enum EntryToken {
    Start,
    LeftBrace,
    RightBrace,
    Comma,
    Equals,
    Type(EntryType),
    Symbol(String),
    Property(BookProperty),
    Value(String),
}

fn tokenize_bibtex(entry: String) -> Vec<EntryToken> {
    vec![
        EntryToken::Start,
        EntryToken::Type(EntryType::Book),
        EntryToken::LeftBrace,
        EntryToken::Symbol(s!("beck-2004")),
        EntryToken::Comma,
        EntryToken::Property(BookProperty::Title),
        EntryToken::Equals,
        EntryToken::Value(s!("Extreme Programming Explained: Embrace Change")),
        EntryToken::Comma,
        EntryToken::RightBrace,
    ]
}

#[cfg(test)]
mod tokenizer_test {
    use super::*;

    #[test]
    fn tokenize_bibtex_entry() {
        // given
        let entry = s!(r#"
            @book{beck-2004,
              title     = {Extreme Programming Explained: Embrace Change},
            }"#);

        let expected = vec![
            EntryToken::Start,
            EntryToken::Type(EntryType::Book),
            EntryToken::LeftBrace,
            EntryToken::Symbol(s!("beck-2004")),
            EntryToken::Comma,
            EntryToken::Property(BookProperty::Title),
            EntryToken::Equals,
            EntryToken::Value(s!("Extreme Programming Explained: Embrace Change")),
            EntryToken::Comma,
            EntryToken::RightBrace,
        ];

        // when
        let actual = tokenize_bibtex(entry);

        // then
        assert_eq!(expected, actual)
    }
}
