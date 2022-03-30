use std::{
    fs,
    io::{prelude::*, ErrorKind, Write},
    path::Path,
    process,
};

use super::status;

use colored::*;

// TODO: Replace String type for Path from filename
pub struct File {
    filename: String,
    file: std::fs::File,
}

impl File {
    pub fn file(&mut self) -> &fs::File {
        &mut self.file
    }

    pub fn filename(&self) -> &String {
        &self.filename
    }

    pub fn new(filename: &str, create: bool) -> Self {
        let filename = filename.to_string();
        let file = File::open(&filename, create);

        Self { file, filename }
    }

    pub fn get_content(&mut self) -> String {
        if let Err(e) = self.file.rewind() {
            status::print_error!("Unable to rewind the file: {}", e);
        }
        let mut content = String::new();
        if let Err(e) = self.file.read_to_string(&mut content) {
            status::print_error!("Something went wrong reading the file: {}", e);
        };
        content
    }

    pub fn get_lines(&mut self) -> Vec<String> {
        let raw_lines = &self.get_content();
        let lines: Vec<String> = raw_lines.lines().map(|l| l.to_string()).collect();
        lines
    }

    pub fn create(filename: &String) -> fs::File {
        fs::OpenOptions::new()
            .write(true)
            .read(true)
            .create(true)
            .truncate(true)
            .open(&filename)
            .unwrap_or_else(|e| status::print_error!("Unable to create the file: {}", e))
    }

    pub fn open(filename: &String, create: bool) -> fs::File {
        let file = fs::OpenOptions::new()
            .write(true)
            .read(true)
            .append(true)
            .open(&filename)
            .unwrap_or_else(|err| {
                if err.kind() != ErrorKind::NotFound {
                    status::print_error!("Something went wrong opening the file: {}", err);
                }
                status::print_warning("File not found");
                if !create {
                    process::exit(1);
                }

                println!("{}", "Creating...".bright_green());
                File::create(filename)
            });

        file
    }

    pub fn update_file(filename: &str, content: String) {
        let mut new_file = File::create(&filename.to_string());
        new_file
            .write(content.as_bytes())
            .unwrap_or_else(|e| status::print_error!("Unable to write on the file: {}", e));
    }
}
