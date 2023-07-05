use crate::entry::Parser;
use crate::tokenizer::Tokenizer;

mod date;
mod edition;
mod entry;
mod entry_field;
mod entry_type;
mod pages;
mod person;
mod strings;
mod tokenizer;

fn main() {
    let mut parser = parser_for_str(EXAMPLE_ENTRIES);
    let entries = parser.parse();

    assert!(entries.is_ok());
    println!("{:#?}", entries.unwrap());
}

fn parser_for_str(input: &'static str) -> Parser {
    let reader = Box::new(input.as_bytes());
    let tokenizer = Tokenizer::new(reader);
    Parser::new(tokenizer)
}

static EXAMPLE_ENTRIES: &str = r#"
    @book{beck-2004,
      title     = {Extreme Programming Explained: Embrace Change},
      edition   = {2},
      isbn      = {978-0-13-405199-4},
      series    = {{XP} Series},
      pagetotal = {189},
      publisher = {Addison-Wesley Professional},
      author    = {Beck, Kent and Andres, Cynthia},
      date      = {2004},
    }
    @article{ieee-802-3-2018,
      journal={IEEE Std 802.3-2018 (Revision of IEEE Std 802.3-2015)},
      title={IEEE Standard for Ethernet},
      year={2018},
      doi={10.1109/IEEESTD.2018.8457469}
    }"#;
