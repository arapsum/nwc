use std::{
    fs::File,
    io::{BufReader, Read},
    path::PathBuf,
};

use clap::Parser;

/// Print  newline, word, and byte counts for each FILE, and a total line if more than one FILE is specified.
/// A word is a non-zero-length sequence of printable characters delimited by white space.
/// With no FILE, or when FILE is -, read standard input.
#[derive(Debug, Parser)]
#[command(version, about, author)]
pub struct App {
    /// prints the byte count
    #[arg(short = 'c', long = "bytes")]
    bytes: bool,

    /// The file to be worked on
    file: Option<PathBuf>,
}

impl App {
    pub fn new() -> Self {
        Self::parse()
    }

    pub fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        let show_bytes = self.bytes;

        if let Some(path) = &self.file {
            let file = File::open(path)?;
            let mut reader = BufReader::new(file);

            let mut buffer = Vec::new();

            reader.read_to_end(&mut buffer)?;

            if show_bytes {
                let contents = std::str::from_utf8(&buffer)?;

                let bytes = contents.len();

                println!("{} {}", bytes, path.display());
            }
        }

        Ok(())
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}
