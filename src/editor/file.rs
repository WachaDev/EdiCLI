use std::{
    fs,
    io::{self, prelude::*, ErrorKind},
    path::Path,
    process,
};

use colored::*;

use super::status;

pub struct File<P: AsRef<Path>> {
    file: fs::File,
    filename: P,
}

impl<P: AsRef<Path>> File<P> {
    pub fn file(&mut self) -> &fs::File {
        &mut self.file
    }

    pub fn filename(&self) -> &Path {
        self.filename.as_ref()
    }

    pub fn new(filename: P, create: bool) -> Self {
        let file = File::open(&filename).unwrap_or_else(|err| {
            if err.kind() != ErrorKind::NotFound {
                status::print_error!("Something went wrong opening the file {err}");
            }

            status::print_warning!("File not found");
            if !create { process::exit(1); }
            println!("{}", "Creating...".green());
            File::create(&filename)
        });

        Self { file, filename }
    }

    pub fn content(&mut self) -> String {
        if let Err(e) = self.file.rewind() {
            status::print_error!("Unable to rewind the file: {e}");
        }

        let mut content = String::new();
        if let Err(e) = self.file.read_to_string(&mut content) {
            status::print_error!("Something went wrong reading the file: {e}");
        };

        content
    }

    pub fn lines(&mut self) -> Vec<String> {
        let raw_lines = &self.content();
        let lines: Vec<String> = raw_lines.lines().map(|l| l.to_string()).collect();

        lines
    }

    pub fn create(filename: P) -> fs::File {
        fs::OpenOptions::new()
            .write(true)
            .read(true)
            .create(true)
            .truncate(true)
            .open(filename.as_ref())
            .unwrap_or_else(|e| status::print_error!("Unable to create the file: {e}"))
    }

    pub fn open(filename: P) -> Result<fs::File, io::Error> {
        let file = fs::OpenOptions::new()
            .write(true)
            .read(true)
            .append(true)
            .open(filename.as_ref())?;

        Ok(file)
    }

    pub fn update(filename: P, content: String) {
        let mut new_file = File::create(filename.as_ref());
        new_file
            .write(content.as_bytes())
            .unwrap_or_else(|e| status::print_error!("Unable to write on the file: {e}"));
    }
}
