use crate::parser;
use clap::Parser;

extern crate clap;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(long = "bytes", short = 'c', action)]
    bytes: bool,

    #[clap(long = "words", short = 'w', action)]
    words: bool,

    #[clap(long = "lines", short = 'l', action)]
    lines: bool,

    #[clap(required = true, value_parser)]
    file: String,
}

pub fn run() {
    let args = Cli::parse();

    match parser::parse(args.file) {
        Ok(()) => (),
        Err(error) => panic!("Ran into an error parsing file: {:?}", error),
    }
}
