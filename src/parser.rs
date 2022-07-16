use std::{
    fmt,
    fs::File,
    io::{self, BufReader, Read, Write},
    thread, time,
};

use colored::Colorize;

#[derive(Default)]
pub struct ParserOpts {
    pub bytes: bool,
    pub words: bool,
    pub lines: bool,
}

#[derive(Default)]
struct ParserCounts {
    opts: ParserOpts,
    bytes: i32,
    words: i32,
    lines: i32,
}

impl fmt::Display for ParserCounts {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut output = String::from("");

        if self.opts.lines {
            output.push_str(&format!("\t{}", self.lines.to_string().green()));
        }

        if self.opts.words {
            output.push_str(&format!("\t{}", self.words.to_string().blue()));
        }

        if self.opts.bytes {
            output.push_str(&format!("\t{}", self.bytes.to_string().red()));
        }

        write!(f, "{}", output)
    }
}

fn read_file_into_buffer(path: &str) -> io::Result<Vec<u8>> {
    let file = File::open(path)?;

    let mut reader = BufReader::new(file);
    let mut buffer = Vec::new();

    reader.read_to_end(&mut buffer)?;

    return Ok(buffer);
}

pub fn parse(file: String, opts: ParserOpts) -> io::Result<()> {
    let mut counts = ParserCounts {
        opts,
        ..Default::default()
    };
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
