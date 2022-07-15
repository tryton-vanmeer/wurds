use std::{
    fmt,
    io::{self, Write},
    thread, time,
};

use colored::Colorize;

#[derive(Default)]
struct ParserCounts {
    bytes: i32,
    words: i32,
    lines: i32,
}

impl fmt::Display for ParserCounts {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "\t{}\t{}\t{}",
            self.lines.to_string().green(),
            self.words.to_string().blue(),
            self.bytes.to_string().red()
        )
    }
}

pub fn parse(file: &str) {
    let mut counts = ParserCounts::default();

    for _ in 0..100 {
        counts.bytes += 1;
        counts.words += 1;
        counts.lines += 1;

        print!("\r{} \t{}", counts, file);
        io::stdout().flush().unwrap();
        thread::sleep(time::Duration::from_millis(16));
    }

    println!();
}
