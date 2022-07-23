use std::io;

use crate::parser;
use clap::Parser;

extern crate clap;

#[derive(Parser)]
#[clap(version = concat!("git-", env!("VERGEN_GIT_SHA_SHORT")))]
#[clap(author, about, long_about = None)]
struct Cli {
    #[clap(long = "bytes", short = 'c', help = "print the byte counts", action)]
    bytes: bool,

    #[clap(long = "words", short = 'w', help = "print the word counts", action)]
    words: bool,

    #[clap(long = "lines", short = 'l', help = "print the line counts", action)]
    lines: bool,

    #[clap(value_parser)]
    file: Option<String>,
}

fn output_parser_count(counts: &parser::ParserCounts) {
    print!("\r{}", counts);
}

pub fn run() -> io::Result<()> {
    let args = Cli::parse();

    let opts = if !args.bytes && !args.words && !args.lines {
        parser::ParserOpts {
            bytes: true,
            words: true,
            lines: true,
        }
    } else {
        parser::ParserOpts {
            bytes: args.bytes,
            words: args.words,
            lines: args.lines,
        }
    };

    let _ = match args.file {
        Some(value) => parser::parse(value, opts, output_parser_count),
        None => parser::parse(String::from("-"), opts, output_parser_count),
    };

    Ok(())
}
