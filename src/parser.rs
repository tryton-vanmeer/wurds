use std::{
    fs::File,
    io::{self, BufReader, Read},
    iter::Peekable,
    slice::Iter,
};

#[derive(Default)]
pub struct ParserCounts {
    buffer: Vec<u8>,
    pub filename: String,
    pub bytes: i32,
    pub words: i32,
    pub lines: i32,
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

pub fn parse<F: Fn(Option<&ParserCounts>)>(
    files: Vec<String>,
    callback: F,
) -> io::Result<Vec<ParserCounts>> {
    let mut counts: Vec<ParserCounts> = Vec::new();

    if files.is_empty() {
        counts.push(ParserCounts {
            filename: "-".into(),
            buffer: read_stdin_into_buffer()?,
            ..Default::default()
        });
    } else {
        for file in files {
            counts.push(ParserCounts {
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
        let counts = parse(files, |_| {}).unwrap();
        let count = &counts[0];

        assert_eq!(count.lines, 674);
        assert_eq!(count.words, 5644);
        assert_eq!(count.bytes, 35149);
    }
}
