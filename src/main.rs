use std::io;

mod cli;
mod parser;

fn main() -> io::Result<()> {
    cli::run()
}
