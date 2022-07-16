use crate::parser;
use clap::Parser;

extern crate clap;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(long = "bytes", short = 'c', help = "print the byte counts", action)]
    bytes: bool,

    #[clap(long = "words", short = 'w', help = "print the word counts", action)]
    words: bool,

    #[clap(long = "lines", short = 'l', help = "print the line counts", action)]
    lines: bool,

    #[clap(required = true, value_parser)]
    file: String,
}

pub fn run() {
    let args = Cli::parse();
    let opts: parser::ParserOpts;

    if !args.bytes && !args.words && !args.lines {
        opts = parser::ParserOpts {
            bytes: true,
            words: true,
            lines: true,
        }
    } else {
        opts = parser::ParserOpts {
            bytes: args.bytes,
            words: args.words,
            lines: args.lines,
        };
    }

    match parser::parse(args.file, opts) {
        Ok(()) => (),
        Err(error) => panic!("Ran into an error parsing file: {:?}", error),
    }
}
