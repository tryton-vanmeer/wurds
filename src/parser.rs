use std::{
    fmt,
    io::{self, Write},
    thread, time,
};

#[derive(Default)]
struct ParserCounts {
    bytes: i32,
    words: i32,
    lines: i32,
}

impl fmt::Display for ParserCounts {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.lines, self.words, self.bytes)
    }
}

pub fn parse(file: &str) {
    println!("Parsing file: {}", file);

    let mut counts = ParserCounts::default();

    for _ in 0..100 {
        counts.bytes += 1;
        counts.words += 1;
        counts.lines += 1;

        print!("\r{}", counts);
        io::stdout().flush().unwrap();
        thread::sleep(time::Duration::from_millis(16));
    }
}
