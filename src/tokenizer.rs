use std::io::{Error, ErrorKind, Read};
use EntryEnvironment::*;

use crate::s;

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

#[derive(Copy, Clone, Debug, PartialEq)]
enum EntryLiteral {
    AtSign,
    LeftBrace,
    RightBrace,
    Comma,
    Equals,
    Whitespace,
    Other(char),
    Newline,
    DoubleQuote,
    Hash,
    EndOfFile,
}

impl EntryLiteral {
    fn from_char(c: char) -> EntryLiteral {
        match c {
            '@' => EntryLiteral::AtSign,
            '{' => EntryLiteral::LeftBrace,
            '}' => EntryLiteral::RightBrace,
            ',' => EntryLiteral::Comma,
            '"' => EntryLiteral::DoubleQuote,
            '#' => EntryLiteral::Hash,
            '=' => EntryLiteral::Equals,
            ' ' | '\t' | '\r' => EntryLiteral::Whitespace,
            '\n' => EntryLiteral::Newline,
            c => EntryLiteral::Other(c),
        }
    }
}

#[derive(Copy, Clone, PartialEq)]
enum EntryEnvironment {
    Idle,
    ReadType,
    ReadSymbol,
    ReadPropertyName,
    ReadValue(i32),
    End,
}

struct Tokenizer {
    buffer: Box<dyn Read>,
    state: EntryEnvironment,
    position: Position,
}

impl Tokenizer {
    fn new(buffer: Box<dyn Read>) -> Tokenizer {
        Tokenizer {
            buffer,
            state: Idle,
            position: Position {
                byte: 1,
                line: 1,
                column: 1,
            },
        }
    }

    fn idle(&mut self) -> Result<(), Error> {
        let next = self.next_char()?;
        todo!()
    }

    fn next_literal(&mut self) -> Result<EntryLiteral, Error> {
        let c = self.next_char()?;
        match c {
            Some(c) => Ok(EntryLiteral::from_char(c)),
            None => Ok(EntryLiteral::EndOfFile),
        }
    }

    fn next_char(&mut self) -> Result<Option<char>, Error> {
        let mut utf8_buffer = [0u8; 4];

        for i in 0..4 {
            let mut current_byte = 0u8;
            let read = self.buffer.read(std::slice::from_mut(&mut current_byte))?;
            if read == 0 {
                return Ok(None);
            }
            self.advance_byte();
            utf8_buffer[i] = current_byte;

            let maybe_parsed = std::str::from_utf8(&utf8_buffer);
            if let Ok(parsed) = maybe_parsed {
                let ch = parsed.chars().next();
                if let Some(c) = ch {
                    self.advance(c);
                }
                return Ok(ch);
            } else {
                continue;
            }
        }

        Err(Error::new(
            ErrorKind::InvalidInput,
            format!(
                "Cannot decode bytes to UTF-8. Bytes: [{:02x} {:02x} {:02x} {:02x}]",
                utf8_buffer[0], utf8_buffer[1], utf8_buffer[2], utf8_buffer[3]
            ),
        ))
    }

    fn advance_byte(&mut self) {
        self.position.byte += 1;
    }

    fn advance(&mut self, c: char) {
        if c == '\n' {
            self.advance_newline()
        } else {
            self.position.column += 1;
        }
    }

    fn advance_newline(&mut self) {
        self.position.column = 1;
        self.position.line += 1;
    }

    fn position_str(&self) -> String {
        format!(
            "byte: {} (line {}, column {})",
            self.position.byte, self.position.line, self.position.column
        )
    }

    fn new_state_from_literal(
        &self,
        literal: EntryLiteral,
    ) -> Result<EntryEnvironment, &'static str> {
        match (&self.state, literal) {
            (Idle, EntryLiteral::AtSign) => Ok(ReadType),
            (ReadType, EntryLiteral::LeftBrace) => Ok(ReadSymbol),
            (ReadSymbol, EntryLiteral::Comma) => Ok(Idle),
            (Idle, EntryLiteral::Other(_)) => Ok(ReadPropertyName),
            (ReadPropertyName, EntryLiteral::Whitespace) => Ok(Idle),
            (Idle, EntryLiteral::Equals) => Ok(ReadValue(0)),
            (ReadValue(i), EntryLiteral::LeftBrace) => Ok(ReadValue(i + 1)),
            (ReadValue(i), EntryLiteral::RightBrace) => match i {
                i32::MIN..=-1 => {
                    Err("Error when parsing bibtex. Possibly too many closing brackets?")
                }
                i => Ok(ReadValue(*i - 1)),
            },
            (ReadValue(i), EntryLiteral::Comma) => match i {
                0 => Ok(Idle),
                i => Ok(ReadValue(*i)),
            },
            (Idle, EntryLiteral::Whitespace) => Ok(Idle),
            (Idle, EntryLiteral::RightBrace) => Ok(End),
            (_, _) => Err("Error when parsing bibtex. Unexpected end of file."),
        }
    }
}

impl Iterator for Tokenizer {
    type Item = EntryToken;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
struct Position {
    byte: usize,
    line: usize,
    column: usize,
}

struct EntryContext {
    value: String,
    env: EntryEnvironment,
}

#[cfg(test)]
mod tokenizer_test {
    use super::*;

    #[test]
    fn literals() {
        vec![
            ("@", EntryLiteral::AtSign),
            ("\n", EntryLiteral::Newline),
            ("a", EntryLiteral::Other('a')),
            ("", EntryLiteral::EndOfFile),
        ]
        .iter()
        .for_each(|(input, expected)| {
            // given
            let mut tokenizer = tokenizer_for_str(input);

            // when
            let actual = tokenizer.next_literal().unwrap();

            // then
            assert_eq!(actual, *expected);
        });
    }

    #[test]
    fn position() {
        vec![
            (
                "aaa",
                Position {
                    byte: 3,
                    line: 1,
                    column: 3,
                },
            ),
            (
                "a👌b",
                Position {
                    byte: 6,
                    line: 1,
                    column: 3,
                },
            ),
            (
                "a\nb",
                Position {
                    byte: 3,
                    line: 2,
                    column: 1,
                },
            ),
            (
                "👌\nb",
                Position {
                    byte: 6,
                    line: 2,
                    column: 1,
                },
            ),
        ]
        .iter()
        .for_each(|(input, expected)| {
            // given
            let mut tokenizer = tokenizer_for_str(input);

            // when
            for _ in 1..3 {
                let _ = tokenizer.next_char();
            }
            let actual = tokenizer.position;

            // then
            assert_eq!(actual, *expected);
        });
    }

    #[test]
    fn utf8_next_char() {
        vec![("abc", Some('a')), ("👌", Some('👌')), ("", None)]
            .iter()
            .for_each(|(input, expected)| {
                // given
                let mut tokenizer = tokenizer_for_str(input);

                // when
                let actual = tokenizer.next_char().unwrap();

                // then
                assert_eq!(actual, *expected);
            });
    }

    #[test]
    #[allow(clippy::invalid_utf8_in_unchecked)]
    fn non_valid_utf8_next_char() {
        unsafe {
            // given
            let utf8_buffer = &[255, 254, 253, 252];
            let input = std::str::from_utf8_unchecked(utf8_buffer);
            let mut tokenizer = tokenizer_for_str(input);

            // when
            let actual = tokenizer.next_char().unwrap_err().to_string();

            // then
            assert_eq!(
                actual,
                s!("Cannot decode bytes to UTF-8. Bytes: [ff fe fd fc]"),
            );
        }
    }

    fn reader_from_str(s: &str) -> Box<dyn Read + '_> {
        Box::new(s.as_bytes())
    }

    #[test]
    fn tokenize_bibtex_entry() {
        // given
        let input = r#"
            @book{beck-2004,
              title     = {Extreme Programming Explained: Embrace Change},
            }"#;

        let tokenizer = tokenizer_for_str(input);

        let expected = vec![
            EntryToken::Type(EntryType::Book),
            EntryToken::Symbol(s!("beck-2004")),
            EntryToken::Property(BookProperty::Title),
            EntryToken::Value(s!("Extreme Programming Explained: Embrace Change")),
        ];

        // when
        let actual: Vec<EntryToken> = tokenizer.collect();

        // then
        assert_eq!(actual, expected)
    }

    fn tokenizer_for_str(input: &'static str) -> Tokenizer {
        let reader = reader_from_str(input);
        Tokenizer::new(reader)
    }
}
