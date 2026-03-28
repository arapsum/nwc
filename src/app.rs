#![allow(clippy::missing_errors_doc)]
#![allow(clippy::struct_excessive_bools)]
use std::{
    fmt::Write,
    fs::File,
    io::{self, BufReader, Read, Stdin},
    path::PathBuf,
};

use clap::Parser;

/// Print  newline, word, and byte counts for each FILE, and a total line if more than one FILE is specified.
///
/// A word is a non-zero-length sequence of printable characters delimited by white space.
/// With no FILE, or when FILE is -, read standard input.
#[derive(Debug, Parser)]
#[command(version, about, author)]
pub struct App {
    /// prints the byte count
    #[arg(short = 'c', long = "bytes")]
    bytes: bool,

    /// prints the number of lines
    #[arg(short = 'l', long = "lines")]
    lines: bool,

    /// prints the word count
    #[arg(short = 'w', long = "words")]
    words: bool,

    /// prints the characters count
    #[arg(short = 'm', long = "chars")]
    chars: bool,

    /// The file to be worked on
    file: Option<PathBuf>,
}

impl App {
    #[must_use]
    pub fn new() -> Self {
        Self::parse()
    }

    pub fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        let no_flags_specified = !self.bytes && !self.lines && !self.words && !self.chars;

        let show_bytes = self.bytes || no_flags_specified;
        let show_lines = self.lines || no_flags_specified;
        let show_words = self.words || no_flags_specified;
        let show_chars = self.chars;

        let reader = if let Some(path) = &self.file {
            Input::File(File::open(path)?)
        } else {
            Input::Stdin(io::stdin())
        };

        let mut buffer = Vec::new();
        BufReader::new(reader).read_to_end(&mut buffer)?;

        let mut output = String::new();

        if show_lines {
            let lines = bytecount::count(&buffer, b'\n');
            write!(output, "{lines} ")?;
        }

        if show_words || show_chars {
            let contents = std::str::from_utf8(&buffer).ok();

            if show_words {
                let words = contents.map_or(0, |c| c.split_whitespace().count());
                write!(output, "{words} ")?;
            }

            if show_chars {
                let chars = contents.map_or(buffer.len(), |c| c.chars().count());
                write!(output, "{chars} ")?;
            }
        }

        if show_bytes {
            let bytes = buffer.len();
            write!(output, "{bytes} ")?;
        }

        if let Some(path) = &self.file {
            write!(output, "{}", path.display())?;
        }

        println!("{output}");

        Ok(())
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

enum Input {
    File(File),
    Stdin(Stdin),
}

impl Read for Input {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        match self {
            Self::File(file) => file.read(buf),
            Self::Stdin(stdin) => stdin.read(buf),
        }
    }
}
