use std::io::{Error, ErrorKind, Read};
use TokenizerState::*;

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum EntryToken {
    Type(String),
    Symbol(String),
    FieldName(String),
    Value(String),
}

pub(crate) struct Tokenizer {
    buffer: Box<dyn Read>,
    current_token_value: String,
    tokens: Vec<EntryToken>,
    state: TokenizerState,
    position: Position,
}

impl Tokenizer {
    pub(crate) fn new(buffer: Box<dyn Read>) -> Tokenizer {
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

    pub(crate) fn tokenize(&mut self) -> Result<Vec<EntryToken>, String> {
        loop {
            let result = match self.state {
                Idle => self.idle(),
                ReadType => self.read_type(),
                ReadSymbol => self.read_symbol(),
                ReadPropertyName => self.read_field_name(),
                ReadValue(TokenizerReadValueMode::Normal) => self.read_value(),
                ReadValue(TokenizerReadValueMode::DoubleQuoted) => self.read_value_quoted(),
                ReadValue(TokenizerReadValueMode::Braced(_)) => self.read_value_braced(),
            };
            match result {
                Ok(()) => continue,
                Err(e) if e.kind() == ErrorKind::WriteZero => break,
                Err(e) => return Err(format!("Tokenization error: {}", e)),
            };
        }
        Ok(self.tokens.clone())
    }

    fn idle(&mut self) -> Result<(), Error> {
        let literal = self.next_literal()?;
        match literal {
            EntryLiteral::AtSign => {
                self.transition(ReadType);
                Ok(())
            }
            EntryLiteral::Whitespace | EntryLiteral::Newline => Ok(()),
            EntryLiteral::EndOfFile => Err(Error::new(ErrorKind::WriteZero, "File ended")),
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
                self.add_token(EntryToken::Type(self.current_token_value.clone()));
                self.transition(ReadSymbol);
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
            EntryLiteral::Alphabetic(c) | EntryLiteral::Numeric(c) | EntryLiteral::Other(c) => {
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

    fn read_field_name(&mut self) -> Result<(), Error> {
        let literal = self.next_literal()?;
        match literal {
            EntryLiteral::Alphabetic(c) => {
                self.current_token_value.push(c);
                Ok(())
            }
            EntryLiteral::Equals => {
                self.add_token(EntryToken::FieldName(self.current_token_value.clone()));
                self.transition(ReadValue(TokenizerReadValueMode::Normal));
                Ok(())
            }
            EntryLiteral::RightBrace => {
                self.transition(Idle);
                Ok(())
            }
            EntryLiteral::Whitespace | EntryLiteral::Newline => Ok(()),
            EntryLiteral::EndOfFile => self.unexpected_eof(),
            l => self.invalid_token(l),
        }
    }

    fn read_value(&mut self) -> Result<(), Error> {
        let literal = self.next_literal()?;
        match literal {
            EntryLiteral::Alphabetic(c) | EntryLiteral::Numeric(c) => {
                self.current_token_value.push(c);
                Ok(())
            }
            EntryLiteral::DoubleQuote => {
                self.transition_keep_value(ReadValue(TokenizerReadValueMode::DoubleQuoted));
                Ok(())
            }
            EntryLiteral::LeftBrace => {
                self.transition_keep_value(ReadValue(TokenizerReadValueMode::Braced(0)));
                Ok(())
            }
            EntryLiteral::Comma => {
                self.add_token(EntryToken::Value(self.current_token_value.clone()));
                self.transition(ReadPropertyName);
                Ok(())
            }
            EntryLiteral::RightBrace => {
                self.add_token(EntryToken::Value(self.current_token_value.clone()));
                self.transition(Idle);
                Ok(())
            }
            EntryLiteral::Whitespace | EntryLiteral::Newline => Ok(()),
            EntryLiteral::EndOfFile => self.unexpected_eof(),
            l => self.invalid_token(l),
        }
    }

    fn read_value_quoted(&mut self) -> Result<(), Error> {
        let literal = self.next_literal()?;
        match literal {
            EntryLiteral::DoubleQuote => {
                self.transition_keep_value(ReadValue(TokenizerReadValueMode::Normal));
                Ok(())
            }
            EntryLiteral::EndOfFile => self.unexpected_eof(),
            l => {
                self.current_token_value.push(l.to_char());
                Ok(())
            }
        }
    }

    fn read_value_braced(&mut self) -> Result<(), Error> {
        let literal = self.next_literal()?;
        let brace_level: i32 = match self.state {
            ReadValue(TokenizerReadValueMode::Braced(i)) => i,
            _ => 0,
        };
        match literal {
            EntryLiteral::RightBrace => {
                self.transition_keep_value(ReadValue(Self::right_brace_mode(brace_level)));
                Ok(())
            }
            EntryLiteral::LeftBrace => {
                self.transition_keep_value(ReadValue(TokenizerReadValueMode::Braced(
                    brace_level + 1,
                )));
                Ok(())
            }
            EntryLiteral::EndOfFile => self.unexpected_eof(),
            l => {
                self.current_token_value.push(l.to_char());
                Ok(())
            }
        }
    }

    fn right_brace_mode(brace_level: i32) -> TokenizerReadValueMode {
        match brace_level {
            bl if bl > 0 => TokenizerReadValueMode::Braced(bl - 1),
            _ => TokenizerReadValueMode::Normal,
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

    fn transition_keep_value(&mut self, new_state: TokenizerState) {
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
    Numeric(char),
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
            c if c.is_numeric() => EntryLiteral::Numeric(c),
            c => EntryLiteral::Other(c),
        }
    }

    fn to_char(self) -> char {
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
            EntryLiteral::Other(c) | EntryLiteral::Alphabetic(c) | EntryLiteral::Numeric(c) => *c,
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
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum TokenizerReadValueMode {
    Normal,
    DoubleQuoted,
    Braced(i32),
}

#[cfg(test)]
mod tokenizer_test {
    use super::*;
    use crate::s;

    #[test]
    fn tokenize_bibtex_entry() {
        // given
        let input = r#"
            @book{beck-2004,
              title     = {Extreme Programming Explained: Embrace Change},
            }
            @online{malan-2008,
              title      = "Conway's Law",
              author     = "Malan, Ruth",
              year       = 2008
            }"#;

        let mut tokenizer = tokenizer_for_str(input);

        let expected = vec![
            EntryToken::Type(s!("book")),
            EntryToken::Symbol(s!("beck-2004")),
            EntryToken::FieldName(s!("title")),
            EntryToken::Value(s!("Extreme Programming Explained: Embrace Change")),
            EntryToken::Type(s!("online")),
            EntryToken::Symbol(s!("malan-2008")),
            EntryToken::FieldName(s!("title")),
            EntryToken::Value(s!("Conway's Law")),
            EntryToken::FieldName(s!("author")),
            EntryToken::Value(s!("Malan, Ruth")),
            EntryToken::FieldName(s!("year")),
            EntryToken::Value(s!("2008")),
        ];

        // when
        let actual: Vec<EntryToken> = tokenizer.tokenize().unwrap();

        // then
        assert_eq!(actual, expected);
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
        fn valid_eof() {
            // given
            let input = " \t\n";
            fn consume_whitespace(tokenizer: &mut Tokenizer) {
                for _ in 0..3 {
                    let _ = tokenizer.idle();
                }
            }
            let mut tokenizer = tokenizer_for_str(input);
            let expected = Error::new(ErrorKind::WriteZero, "File ended");

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
            let actual = tokenizer.read_type().unwrap_err();

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
            let expected = EntryToken::Type(s!("abc"));

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
            let expected = EntryToken::Type(s!("abc"));

            // when
            for _ in 0..4 {
                tokenizer.read_type().unwrap();
            }
            let actual = tokenizer.tokens.first().unwrap();

            assert_eq!(*actual, expected);
            assert_eq!(tokenizer.state, ReadSymbol)
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

    mod read_field_name {
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
            let actual = tokenizer.read_field_name().unwrap_err();

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
            let actual = tokenizer.read_field_name().unwrap_err();

            // then
            assert_io_error_eq(actual, expected);
        }

        #[test]
        fn valid_transition_to_reading_value() {
            // given
            let input = "abc=";
            let mut tokenizer = tokenizer_for_str(input);
            let expected = EntryToken::FieldName(s!("abc"));

            // when
            for _ in 0..4 {
                tokenizer.read_field_name().unwrap();
            }
            let actual = tokenizer.tokens.first().unwrap();

            // then
            assert_eq!(*actual, expected);
            assert_eq!(tokenizer.state, ReadValue(TokenizerReadValueMode::Normal));
        }

        #[test]
        fn valid_transition_to_end() {
            // given
            let input = "}";
            let mut tokenizer = tokenizer_for_str(input);

            // when
            tokenizer.read_field_name().unwrap();

            // then
            assert_eq!(tokenizer.state, Idle);
        }
    }

    mod read_value {
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
            let actual = tokenizer.read_value().unwrap_err();

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
            let actual = tokenizer.read_value().unwrap_err();

            // then
            assert_io_error_eq(actual, expected);
        }

        mod read_value_quoted {
            use super::*;

            #[test]
            fn valid_transition_to_quoted() {
                // given
                let input = "\"";
                let mut tokenizer = tokenizer_for_str(input);

                // when
                tokenizer.read_value().unwrap();

                // then
                assert_eq!(
                    tokenizer.state,
                    ReadValue(TokenizerReadValueMode::DoubleQuoted)
                );
            }

            #[test]
            fn valid_transition_out_of_quoted() {
                // given
                let input = "\"";
                let mut tokenizer = tokenizer_for_str(input);

                // when
                tokenizer.read_value_quoted().unwrap();

                // then
                assert_eq!(tokenizer.state, ReadValue(TokenizerReadValueMode::Normal))
            }

            #[test]
            fn valid_special_char_handling() {
                // given
                let input = "a b@c";
                let mut tokenizer = tokenizer_for_str(input);
                let expected = "a b@c";

                // when
                for _ in 0..5 {
                    tokenizer.read_value_quoted().unwrap();
                }
                let actual = tokenizer.current_token_value.as_str();

                assert_eq!(actual, expected);
            }
        }

        mod read_value_braced {
            use super::*;

            #[test]
            fn valid_transition_to_braced() {
                // given
                let input = "{";
                let mut tokenizer = tokenizer_for_str(input);

                // when
                tokenizer.read_value().unwrap();

                // then
                assert_eq!(
                    tokenizer.state,
                    ReadValue(TokenizerReadValueMode::Braced(0))
                );
            }

            #[test]
            fn valid_deeper_braced() {
                // given
                let input = "{{";
                let mut tokenizer = tokenizer_for_str(input);

                // when
                tokenizer.read_value().unwrap();
                tokenizer.read_value_braced().unwrap();

                // then
                assert_eq!(
                    tokenizer.state,
                    ReadValue(TokenizerReadValueMode::Braced(1))
                );
            }

            #[test]
            fn valid_transition_out_of_braced() {
                // given
                let input = "}";
                let mut tokenizer = tokenizer_for_str(input);

                // when
                tokenizer.read_value_braced().unwrap();

                // then
                assert_eq!(tokenizer.state, ReadValue(TokenizerReadValueMode::Normal));
            }

            #[test]
            fn valid_special_char_handling_multiple_braces() {
                // given
                let input = "a b{@}c";
                let mut tokenizer = tokenizer_for_str(input);
                let expected = "a b@c";

                // when
                for _ in 0..7 {
                    tokenizer.read_value_braced().unwrap();
                }
                let actual = tokenizer.current_token_value.as_str();

                assert_eq!(actual, expected);
            }
        }

        #[test]
        fn valid_transition_to_reading_field_name() {
            // given
            let input = "abc,";
            let mut tokenizer = tokenizer_for_str(input);
            let expected = EntryToken::Value(s!("abc"));

            // when
            for _ in 0..4 {
                tokenizer.read_value().unwrap();
            }
            let actual = tokenizer.tokens.first().unwrap();

            assert_eq!(*actual, expected);
            assert_eq!(tokenizer.state, ReadPropertyName);
        }

        #[test]
        fn valid_transition_to_end() {
            // given
            let input = "}";
            let mut tokenizer = tokenizer_for_str(input);

            // when
            tokenizer.read_value().unwrap();

            // then
            assert_eq!(tokenizer.state, Idle);
        }
    }

    #[cfg(test)]
    mod internal {
        use super::*;

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
    }

    fn tokenizer_for_str(input: &'static str) -> Tokenizer {
        let reader = reader_from_str(input);
        Tokenizer::new(reader)
    }

    fn reader_from_str(s: &str) -> Box<dyn Read + '_> {
        Box::new(s.as_bytes())
    }

    fn assert_io_error_eq(actual: Error, expected: Error) {
        assert_eq!(actual.kind(), expected.kind(),);
        assert_eq!(actual.to_string(), expected.to_string(),);
    }
}
