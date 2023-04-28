use std::io::{Error, ErrorKind, Read};
use TokenizerState::*;

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

#[derive(Debug, PartialEq)]
enum EntryToken {
    Type(EntryType),
    Symbol(String),
    Property(EntryProperty),
    Value(String),
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum EntryLiteral {
    AtSign,
    LeftBrace,
    RightBrace,
    Comma,
    DoubleQuote,
    Hash,
    Equals,
    Whitespace,
    Newline,
    Alphabetic(char),
    Other(char),
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
            c if c.is_alphabetic() => EntryLiteral::Alphabetic(c),
            c => EntryLiteral::Other(c),
        }
    }

    fn to_char(&self) -> char {
        match &self {
            EntryLiteral::AtSign => '@',
            EntryLiteral::LeftBrace => '{',
            EntryLiteral::RightBrace => '}',
            EntryLiteral::Comma => ',',
            EntryLiteral::DoubleQuote => '"',
            EntryLiteral::Hash => '#',
            EntryLiteral::Equals => '=',
            EntryLiteral::Whitespace => ' ',
            EntryLiteral::Newline => '\n',
            EntryLiteral::Other(c) | EntryLiteral::Alphabetic(c) => *c,
            EntryLiteral::EndOfFile => '%',
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum TokenizerState {
    Idle,
    ReadType,
    ReadSymbol,
    ReadPropertyName,
    ReadValue(TokenizerReadValueMode),
    End,
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum TokenizerReadValueMode {
    Normal,
    DoubleQuoted,
    Braced,
}

struct Tokenizer {
    buffer: Box<dyn Read>,
    current_token_value: String,
    tokens: Vec<EntryToken>,
    state: TokenizerState,
    position: Position,
}

impl Tokenizer {
    fn new(buffer: Box<dyn Read>) -> Tokenizer {
        Tokenizer {
            buffer,
            current_token_value: String::new(),
            tokens: Vec::new(),
            state: Idle,
            position: Position {
                byte: 0,
                line: 1,
                column: 0,
            },
        }
    }

    fn tokenize(&self) -> Vec<EntryToken> {
        todo!()
    }

    fn read_value(&mut self) -> Result<(), Error> {
        todo!("Here also has to be the handling of EOF in case there is no trailing comma.")
    }

    fn read_property_name(&mut self) -> Result<(), Error> {
        let literal = self.next_literal()?;
        match literal {
            EntryLiteral::Alphabetic(c) => {
                self.current_token_value.push(c);
                Ok(())
            }
            EntryLiteral::Equals => {
                self.add_token(EntryToken::Property(EntryProperty::from_str(
                    self.current_token_value.as_str(),
                )));
                self.transition(ReadValue(TokenizerReadValueMode::Normal));
                Ok(())
            }
            EntryLiteral::RightBrace => {
                self.transition(End);
                Ok(())
            }
            EntryLiteral::Whitespace | EntryLiteral::Newline => Ok(()),
            EntryLiteral::EndOfFile => self.unexpected_eof(),
            l => self.invalid_token(l),
        }
    }

    fn read_symbol(&mut self) -> Result<(), Error> {
        let literal = self.next_literal()?;
        match literal {
            EntryLiteral::Alphabetic(c) | EntryLiteral::Other(c) => {
                self.current_token_value.push(c);
                Ok(())
            }
            EntryLiteral::Comma => {
                self.add_token(EntryToken::Symbol(self.current_token_value.clone()));
                self.transition(ReadPropertyName);
                Ok(())
            }
            EntryLiteral::Whitespace | EntryLiteral::Newline => Ok(()),
            EntryLiteral::EndOfFile => self.unexpected_eof(),
            l => self.invalid_token(l),
        }
    }

    fn read_type(&mut self) -> Result<(), Error> {
        let literal = self.next_literal()?;
        match literal {
            EntryLiteral::Alphabetic(c) => {
                let cl = c
                    .to_lowercase()
                    .next()
                    .expect("Couldn't convert to lowercase.");
                self.current_token_value.push(cl);
                Ok(())
            }
            EntryLiteral::LeftBrace => {
                self.add_token(EntryToken::Type(EntryType::from_str(
                    self.current_token_value.as_str(),
                )));
                self.transition(ReadSymbol);
                Ok(())
            }
            EntryLiteral::Whitespace | EntryLiteral::Newline => Ok(()),
            EntryLiteral::EndOfFile => self.unexpected_eof(),
            l => self.invalid_token(l),
        }
    }

    fn idle(&mut self) -> Result<(), Error> {
        let literal = self.next_literal()?;
        match literal {
            EntryLiteral::AtSign => {
                self.transition(ReadType);
                Ok(())
            }
            EntryLiteral::Whitespace | EntryLiteral::Newline => Ok(()),
            EntryLiteral::EndOfFile => self.unexpected_eof(),
            l => self.invalid_token(l),
        }
    }

    fn unexpected_eof(&self) -> Result<(), Error> {
        Err(Error::new(
            ErrorKind::UnexpectedEof,
            format!("Unexpected EOF. Position: {}", self.position_str()),
        ))
    }

    fn invalid_token(&self, l: EntryLiteral) -> Result<(), Error> {
        Err(Error::new(
            ErrorKind::InvalidInput,
            format!(
                "Unexpected token: '{}'. Position: {}",
                l.to_char(),
                self.position_str()
            ),
        ))
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
        self.position.column = 0;
        self.position.line += 1;
    }

    fn position_str(&self) -> String {
        format!(
            "byte: {} (line {}, column {})",
            self.position.byte, self.position.line, self.position.column
        )
    }

    fn transition(&mut self, new_state: TokenizerState) {
        self.current_token_value = String::new();
        self.state = new_state;
    }

    fn add_token(&mut self, token: EntryToken) {
        self.tokens.push(token);
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
    env: TokenizerState,
}

#[cfg(test)]
mod tokenizer_test {
    use super::*;

    mod read_property_name {
        use super::*;

        #[test]
        fn invalid_token() {
            // given
            let input = "@";
            let mut tokenizer = tokenizer_for_str(input);
            let expected = Error::new(
                ErrorKind::InvalidInput,
                "Unexpected token: '@'. Position: byte: 1 (line 1, column 1)",
            );

            // when
            let actual = tokenizer.read_property_name().unwrap_err();

            // then
            assert_io_error_eq(actual, expected);
        }

        #[test]
        fn unexpected_eof() {
            // given
            let input = "";
            let mut tokenizer = tokenizer_for_str(input);
            let expected = Error::new(
                ErrorKind::UnexpectedEof,
                "Unexpected EOF. Position: byte: 0 (line 1, column 0)",
            );

            // when
            let actual = tokenizer.read_property_name().unwrap_err();

            // then
            assert_io_error_eq(actual, expected);
        }

        #[test]
        fn valid_transition_to_reading_value() {
            // given
            let input = "abc=";
            let mut tokenizer = tokenizer_for_str(input);
            let expected = EntryToken::Property(EntryProperty::Other(s!("abc")));

            // when
            for _ in 0..4 {
                tokenizer.read_property_name().unwrap();
            }
            let actual = tokenizer.tokens.first().unwrap();

            assert_eq!(*actual, expected);
            assert_eq!(tokenizer.state, ReadValue(TokenizerReadValueMode::Normal));
        }

        #[test]
        fn valid_transition_to_end() {
            // given
            let input = "}";
            let mut tokenizer = tokenizer_for_str(input);

            // when
            tokenizer.read_property_name().unwrap();

            assert_eq!(tokenizer.state, End);
        }
    }

    mod read_symbol {
        use super::*;

        #[test]
        fn invalid_token() {
            // given
            let input = "@";
            let mut tokenizer = tokenizer_for_str(input);
            let expected = Error::new(
                ErrorKind::InvalidInput,
                "Unexpected token: '@'. Position: byte: 1 (line 1, column 1)",
            );

            // when
            let actual = tokenizer.read_symbol().unwrap_err();

            // then
            assert_io_error_eq(actual, expected);
        }

        #[test]
        fn unexpected_eof() {
            // given
            let input = "";
            let mut tokenizer = tokenizer_for_str(input);
            let expected = Error::new(
                ErrorKind::UnexpectedEof,
                "Unexpected EOF. Position: byte: 0 (line 1, column 0)",
            );

            // when
            let actual = tokenizer.read_symbol().unwrap_err();

            // then
            assert_io_error_eq(actual, expected);
        }

        #[test]
        fn valid_token() {
            // given
            let input = "a-1,";
            let mut tokenizer = tokenizer_for_str(input);
            let expected = EntryToken::Symbol(s!("a-1"));

            // when
            for _ in 0..4 {
                tokenizer.read_symbol().unwrap();
            }
            let actual = tokenizer.tokens.first().unwrap();

            assert_eq!(*actual, expected);
            assert_eq!(tokenizer.state, ReadPropertyName)
        }
    }

    mod read_type {
        use super::*;

        #[test]
        fn invalid_token() {
            // given
            let input = "!";
            let mut tokenizer = tokenizer_for_str(input);
            let expected = Error::new(
                ErrorKind::InvalidInput,
                "Unexpected token: '!'. Position: byte: 1 (line 1, column 1)",
            );

            // when
            let actual = tokenizer.idle().unwrap_err();

            // then
            assert_io_error_eq(actual, expected);
        }

        #[test]
        fn unexpected_eof() {
            // given
            let input = "";
            let mut tokenizer = tokenizer_for_str(input);
            let expected = Error::new(
                ErrorKind::UnexpectedEof,
                "Unexpected EOF. Position: byte: 0 (line 1, column 0)",
            );

            // when
            let actual = tokenizer.read_type().unwrap_err();

            // then
            assert_io_error_eq(actual, expected);
        }

        #[test]
        fn valid_token() {
            // given
            let input = "abc{";
            let mut tokenizer = tokenizer_for_str(input);
            let expected = EntryToken::Type(EntryType::Other(s!("abc")));

            // when
            for _ in 0..4 {
                tokenizer.read_type().unwrap();
            }
            let actual = tokenizer.tokens.first().unwrap();

            assert_eq!(*actual, expected);
            assert_eq!(tokenizer.state, ReadSymbol)
        }

        #[test]
        fn to_lower_case() {
            // given
            let input = "AbC{";
            let mut tokenizer = tokenizer_for_str(input);
            let expected = EntryToken::Type(EntryType::Other(s!("abc")));

            // when
            for _ in 0..4 {
                tokenizer.read_type().unwrap();
            }
            let actual = tokenizer.tokens.first().unwrap();

            assert_eq!(*actual, expected);
            assert_eq!(tokenizer.state, ReadSymbol)
        }
    }

    mod idle {
        use super::*;

        #[test]
        fn invalid_token() {
            // given
            let input = "abc";
            let mut tokenizer = tokenizer_for_str(input);
            let expected = Error::new(
                ErrorKind::InvalidInput,
                "Unexpected token: 'a'. Position: byte: 1 (line 1, column 1)",
            );

            // when
            let actual = tokenizer.idle().unwrap_err();

            // then
            assert_io_error_eq(actual, expected);
        }

        #[test]
        fn unexpected_eof() {
            // given
            let input = " \t\n";
            fn consume_whitespace(tokenizer: &mut Tokenizer) {
                for _ in 0..3 {
                    let _ = tokenizer.idle();
                }
            }
            let mut tokenizer = tokenizer_for_str(input);
            let expected = Error::new(
                ErrorKind::UnexpectedEof,
                "Unexpected EOF. Position: byte: 3 (line 2, column 0)",
            );

            // when
            consume_whitespace(&mut tokenizer);
            let actual = tokenizer.idle().unwrap_err();

            // then
            assert_io_error_eq(actual, expected);
        }

        #[test]
        fn valid_token() {
            // given
            let input = "@abc";
            let mut tokenizer = tokenizer_for_str(input);

            // when
            tokenizer.idle().unwrap();

            // then
            assert_eq!(tokenizer.state, ReadType);
        }
    }

    #[test]
    fn literals() {
        vec![
            ("@", EntryLiteral::AtSign),
            ("\n", EntryLiteral::Newline),
            ("a", EntryLiteral::Alphabetic('a')),
            ("!", EntryLiteral::Other('!')),
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
            for _ in 0..3 {
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
            let expected = Error::new(
                ErrorKind::InvalidInput,
                s!("Cannot decode bytes to UTF-8. Bytes: [ff fe fd fc]"),
            );

            // when
            let actual = tokenizer.next_char().unwrap_err();

            // then
            assert_io_error_eq(actual, expected);
        }
    }

    fn assert_io_error_eq(actual: Error, expected: Error) {
        assert_eq!(actual.kind(), expected.kind(),);
        assert_eq!(actual.to_string(), expected.to_string(),);
    }

    fn reader_from_str(s: &str) -> Box<dyn Read + '_> {
        Box::new(s.as_bytes())
    }

    #[ignore]
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
            EntryToken::Property(EntryProperty::Title),
            EntryToken::Value(s!("Extreme Programming Explained: Embrace Change")),
        ];

        // when
        let actual: Vec<EntryToken> = tokenizer.tokenize();

        // then
        assert_eq!(actual, expected)
    }

    fn tokenizer_for_str(input: &'static str) -> Tokenizer {
        let reader = reader_from_str(input);
        Tokenizer::new(reader)
    }
}
