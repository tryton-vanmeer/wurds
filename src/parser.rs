use std::{
    fmt,
    fs::File,
    io::{self, BufReader, Read},
    iter::Peekable,
    slice::Iter,
};

use std::fmt::Write as _;

use colored::Colorize;

#[derive(Clone)]
pub struct ParserOpts {
    pub bytes: bool,
    pub words: bool,
    pub lines: bool,
}

impl Default for ParserOpts {
    fn default() -> Self {
        ParserOpts {
            bytes: true,
            words: true,
            lines: true,
        }
    }
}

#[derive(Default)]
pub struct ParserCounts {
    opts: ParserOpts,
    filename: String,
    buffer: Vec<u8>,
    bytes: i32,
    words: i32,
    lines: i32,
}

impl fmt::Display for ParserCounts {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut output = String::from("");

        if self.opts.lines {
            write!(output, "\t{}", self.lines.to_string().green())?;
        }

        if self.opts.words {
            write!(output, "\t{}", self.words.to_string().blue())?;
        }

        if self.opts.bytes {
            write!(output, "\t{}", self.bytes.to_string().red())?;
        }

        write!(output, "\t{}", self.filename)?;

        write!(f, "{}", output)
    }
}

fn read_file_into_buffer(path: String) -> io::Result<Vec<u8>> {
    let file = File::open(path)?;

    let mut reader = BufReader::new(file);
    let mut buffer = Vec::new();

    reader.read_to_end(&mut buffer)?;

    Ok(buffer)
}

fn read_stdin_into_buffer() -> io::Result<Vec<u8>> {
    let mut buffer = Vec::new();

    io::stdin().read_to_end(&mut buffer)?;

    Ok(buffer)
}

pub fn parse(
    files: Vec<String>,
    opts: ParserOpts,
    callback: fn(Option<&ParserCounts>),
) -> io::Result<Vec<ParserCounts>> {
    let mut counts: Vec<ParserCounts> = Vec::new();

    if files.is_empty() {
        counts.push(ParserCounts {
            opts,
            filename: "-".into(),
            buffer: read_stdin_into_buffer()?,
            ..Default::default()
        });
    } else {
        for file in files {
            counts.push(ParserCounts {
                opts: opts.clone(),
                filename: String::from(&file),
                buffer: read_file_into_buffer(file)?,
                ..Default::default()
            })
        }
    }

    let mut finished_counts: Vec<ParserCounts> = Vec::new();

    for mut count in counts {
        let mut peekable_buffer: Peekable<Iter<u8>> = count.buffer.iter().peekable();
        let mut previous = ' ';

        while let Some(value) = peekable_buffer.next() {
            count.bytes += 1;

            if (*value as char).is_whitespace() && !previous.is_whitespace() {
                count.words += 1;
            }

            if *value == (b'\n') {
                count.lines += 1;
            } else if peekable_buffer.peek().is_none() && !(*value as char).is_whitespace() {
                count.words += 1;
            }

            previous = *value as char;

            callback(Some(&count));
        }

        callback(None);
        finished_counts.push(count);
    }

    Ok(finished_counts)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser() {
        let files = vec!["LICENSE".to_string()];
        let counts = parse(files, ParserOpts::default(), |_| {}).unwrap();
        let count = &counts[0];

        assert_eq!(count.lines, 674);
        assert_eq!(count.words, 5644);
        assert_eq!(count.bytes, 35149);
    }
}
