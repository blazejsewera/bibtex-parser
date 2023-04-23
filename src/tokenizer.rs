use crate::s;
use std::ops::Add;
use EntryEnvironment::*;

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
enum BookProperty {
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

impl BookProperty {
    fn from_str(s: &str) -> BookProperty {
        match s {
            "title" => BookProperty::Title,
            "author" => BookProperty::Author,
            "date" => BookProperty::Date,
            "edition" => BookProperty::Edition,
            "isbn" => BookProperty::Isbn,
            "series" => BookProperty::Series,
            "pagetotal" => BookProperty::PageTotal,
            "publisher" => BookProperty::Publisher,
            s => BookProperty::Other(String::from(s)),
        }
    }
}

#[derive(Debug, PartialEq)]
enum EntryToken {
    Type(EntryType),
    Symbol(String),
    Property(BookProperty),
    Value(String),
}

enum EntryLiteral {
    AtSign,
    LeftBrace,
    RightBrace,
    Comma,
    Equals,
    Whitespace,
    Other(char),
}

impl EntryLiteral {
    fn from_char(c: char) -> EntryLiteral {
        match c {
            '@' => EntryLiteral::AtSign,
            '{' => EntryLiteral::LeftBrace,
            '}' => EntryLiteral::RightBrace,
            ',' => EntryLiteral::Comma,
            '=' => EntryLiteral::Equals,
            ' ' | '\t' | '\r' | '\n' => EntryLiteral::Whitespace,
            c => EntryLiteral::Other(c),
        }
    }
}

#[derive(PartialEq)]
enum EntryEnvironment {
    Idle,
    ReadType,
    ReadSymbol,
    ReadPropertyName,
    ReadValue(i32),
    End,
    ErrorEnvironment(&'static str),
}

impl EntryEnvironment {
    fn transition(&self, t: EntryLiteral) -> EntryEnvironment {
        match (&self, t) {
            (Idle, EntryLiteral::AtSign) => ReadType,
            (ReadType, EntryLiteral::LeftBrace) => ReadSymbol,
            (ReadSymbol, EntryLiteral::Comma) => Idle,
            (Idle, EntryLiteral::Other(_)) => ReadPropertyName,
            (ReadPropertyName, EntryLiteral::Whitespace) => Idle,
            (Idle, EntryLiteral::Equals) => ReadValue(0),
            (ReadValue(i), EntryLiteral::LeftBrace) => ReadValue(i + 1),
            (ReadValue(i), EntryLiteral::RightBrace) => match i {
                i32::MIN..=-1 => ErrorEnvironment(
                    "Error when parsing bibtex. Possibly too many closing brackets?",
                ),
                i => ReadValue(*i - 1),
            },
            (ReadValue(i), EntryLiteral::Comma) => match i {
                0 => Idle,
                i => ReadValue(*i),
            },
            (Idle, EntryLiteral::Whitespace) => Idle,
            (Idle, EntryLiteral::RightBrace) => End,
            (_, _) => ErrorEnvironment("Error when parsing bibtex. Unexpected end of file."),
        }
    }
}

struct EntryContext {
    value: String,
    env: EntryEnvironment,
}

fn tokenize_bibtex(entry: String) -> Vec<EntryToken> {
    let mut context: EntryContext = EntryContext {
        value: String::from(""),
        env: Idle,
    };
    let tokens = entry
        .chars()
        .map(EntryLiteral::from_char)
        .scan(context, tokenize_from_literals)
        .filter(Option::is_some);

    vec![
        EntryToken::Type(EntryType::Book),
        EntryToken::Symbol(s!("beck-2004")),
        EntryToken::Property(BookProperty::Title),
        EntryToken::Value(s!("Extreme Programming Explained: Embrace Change")),
    ]
}

fn tokenize_from_literals(
    ctx: &mut EntryContext,
    literal: EntryLiteral,
) -> Option<Option<EntryToken>> {
    let to_save = match ctx.env {
        Idle | End | ErrorEnvironment(_) => None,
        _ => match literal {
            EntryLiteral::Other(c) => Some(c),
            EntryLiteral::Whitespace => Some(' '),
            _ => None,
        },
    };
    ctx.env = ctx.env.transition(literal);
    if let Some(c) = to_save {
        ctx.value = format!("{}{}", ctx.value, c)
    };
    Some(None)
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
            EntryToken::Type(EntryType::Book),
            EntryToken::Symbol(s!("beck-2004")),
            EntryToken::Property(BookProperty::Title),
            EntryToken::Value(s!("Extreme Programming Explained: Embrace Change")),
        ];

        // when
        let actual = tokenize_bibtex(entry);

        // then
        assert_eq!(expected, actual)
    }
}
