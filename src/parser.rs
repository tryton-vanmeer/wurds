use std::{
    fmt,
    fs::File,
    io::{self, BufReader, Read, Write},
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

fn read_file_into_buffer(path: &str) -> io::Result<Vec<u8>> {
    let file = File::open(path)?;

    let mut reader = BufReader::new(file);
    let mut buffer = Vec::new();

    reader.read_to_end(&mut buffer)?;

    return Ok(buffer);
}

pub fn parse(file: String) -> io::Result<()> {
    let mut counts = ParserCounts::default();
    let mut previous = '0';

    let buffer = read_file_into_buffer(&file)?;

    for value in buffer {
        counts.bytes += 1;

        if (value as char).is_whitespace() && !previous.is_whitespace() {
            counts.words += 1;
        }

        if value == ('\n' as u8) {
            counts.lines += 1;
        }

        print!("\r{} \t{}", counts, file);

        io::stdout().flush().unwrap();
        thread::sleep(time::Duration::from_millis(8));

        previous = value as char;
    }

    println!();
    Ok(())
}
