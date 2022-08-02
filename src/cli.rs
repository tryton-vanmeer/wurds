use std::io::{self, Write};
use std::thread::sleep;
use std::time::Duration;

use crate::parser;
use clap::Parser;
use colored::Colorize;

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

pub fn run() -> io::Result<()> {
    let args = Cli::parse();

    parser::parse(args.file, |counts| match counts {
        Some(counts) => {
            print!("\r");

            if !args.lines && !args.words && !args.bytes {
                print!(
                    "\t{} \t{} \t{}",
                    counts.lines.to_string().green(),
                    counts.words.to_string().blue(),
                    counts.bytes.to_string().red()
                );
            } else {
                if args.lines {
                    print!("\t{}", counts.lines.to_string().green());
                }

                if args.words {
                    print!("\t{}", counts.words.to_string().blue());
                }

                if args.bytes {
                    print!("\t{}", counts.bytes.to_string().red());
                }
            }

            print!("\t{}", counts.filename);

            io::stdout().flush().unwrap();
            sleep(Duration::from_millis(8));
        }
        None => println!(),
    })?;

    Ok(())
}
