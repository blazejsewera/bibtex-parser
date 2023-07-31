use std::fs::File;
use std::io::{stdin, stdout, BufReader, Read, Stdin, Write};

use clap::Parser as ArgParser;

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

#[derive(ArgParser, Debug)]
#[command(name = "BibTeX Parser")]
#[command(author = "Blazej Sewera <https://github.com/sewera>")]
#[command(version = "1.0")]
#[command(
    about = "Takes Bib(La)TeX files, parses them, and outputs a JSON",
    long_about = r#"A command line tool and Rust library
for parsing Bib(La)TeX entries and converting them to JSON.
Without any arguments it takes stdin and outputs to stdout."#
)]
struct Args {
    #[arg(help = "Input file. stdin if not set.")]
    infile: Option<String>,

    #[arg(short, help = "Output file. stdout if not set.")]
    outfile: Option<String>,
}

fn main() {
    let args = Args::parse();

    let mut parser = args
        .infile
        .map(open_file_read)
        .map(|file| file.unwrap())
        .map(parser_for_file)
        .unwrap_or(parser_for_stdin());

    let entries = parser.parse().unwrap();

    let outfile = args.outfile.map(open_file_write).map(|file| file.unwrap());

    let mut writer: Box<dyn Write> = match outfile {
        Some(file) => Box::new(file),
        None => Box::new(stdout()),
    };

    let json = serde_json::to_string(&entries).unwrap();

    write!(writer, "{}", json).unwrap();
}

fn open_file_write(filename: String) -> Result<File, String> {
    File::options()
        .read(false)
        .write(true)
        .create(true)
        .open(filename)
        .map_err(|err| err.to_string())
}

fn open_stdin() -> BufReader<Stdin> {
    BufReader::new(stdin())
}

fn open_file_read(filename: String) -> Result<File, String> {
    File::open(filename).map_err(|err| err.to_string())
}

fn parser_for_file(file: File) -> Parser {
    let reader = Box::new(BufReader::new(file));
    parser_for_reader(reader)
}

fn parser_for_stdin() -> Parser {
    let reader = Box::new(open_stdin());
    parser_for_reader(reader)
}

fn parser_for_reader(reader: Box<dyn Read>) -> Parser {
    let tokenizer = Tokenizer::new(reader);
    Parser::new(tokenizer)
}
