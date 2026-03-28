use std::{path::PathBuf, process::ExitCode};

use clap::Parser;

/// Print  newline, word, and byte counts for each FILE, and a total line if more than one FILE is specified.
/// A word is a non-zero-length sequence of printable characters delimited by white space.
/// With no FILE, or when FILE is -, read standard input.
#[derive(Debug, Parser)]
#[command(version, about, author)]
pub struct App {
    file: Option<PathBuf>,
}

impl App {
    pub fn new() -> Self {
        Self::parse()
    }

    pub fn run(&self) -> Result<ExitCode, ExitCode> {
        println!("{:?}", &self.file);

        Ok(ExitCode::SUCCESS)
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}
