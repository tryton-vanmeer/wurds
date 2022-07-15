use crate::parser;
use clap::{App, Arg};

extern crate clap;

pub fn run() {
    let matches = App::new("wurds")
        .version("0.1.0")
        .author("Tryton Van Meer <hello@trytonvanmeer.dev>")
        .about("Rust rewrite of wc with an interactive count-up")
        .arg(
            Arg::with_name("FILE")
                .help("Sets the input file to use")
                .required(true),
        )
        .arg(
            Arg::with_name("bytes")
                .short('c')
                .help("print the byte counts"),
        )
        .arg(
            Arg::with_name("words")
                .short('w')
                .help("print the word counts"),
        )
        .arg(
            Arg::with_name("lines")
                .short('l')
                .help("print the newline counts"),
        )
        .get_matches();

    match parser::parse(matches.value_of("FILE").unwrap()) {
        Ok(()) => (),
        Err(error) => panic!("Ran into an error parsing file: {:?}", error),
    }
}
