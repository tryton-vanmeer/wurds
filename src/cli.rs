use std::io::{self, Write};
use std::thread;
use std::time;

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
    file: Vec<String>,
}

fn output_parser_count(counts: Option<&parser::ParserCounts>) {
    match counts {
        Some(counts) => {
            print!("\r{}", counts);

            io::stdout().flush().unwrap();
            thread::sleep(time::Duration::from_millis(8));
        }
        None => println!(),
    }
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

    parser::parse(args.file, opts, output_parser_count)?;

    Ok(())
}
