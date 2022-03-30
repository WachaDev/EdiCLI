use std::{
    fs::{File, OpenOptions},
    io::{prelude::*, ErrorKind},
    path::Path,
    process,
};

use super::utils;

use colored::*;

pub struct MyFile {
    filename: String,
    file: File,
}

impl MyFile {
    pub fn new(filename: &str, create: bool) -> Self {
        let filename = filename.to_string();
        let file = MyFile::open(&filename, create);

        Self {
            file,
            filename,
        }
    }

    pub fn file(&mut self) -> &File {
        &mut self.file
    }

    pub fn filename(&self) -> &String {
        &self.filename
    }

    pub fn get_content(&mut self) -> String {
        if let Err(e) = self.file.rewind() {
            utils::print_error!("Unable to rewind the file: {}", e);
        }
        let mut content = String::new();
        if let Err(e) = self.file.read_to_string(&mut content) {
            utils::print_error!("Something went wrong reading the file: {}", e);
        };
        content
    }

    pub fn get_lines(&mut self) -> Vec<String> {
        let raw_lines = &self.get_content();
        let lines: Vec<String> = raw_lines.lines().map(|l| l.to_string()).collect();
        lines
    }

    fn open(filename: &String, create: bool) -> File {
        let file = OpenOptions::new()
            .write(true)
            .read(true)
            .append(true)
            .open(&filename)
            .unwrap_or_else(|err| {
                if ErrorKind::NotFound == err.kind() {
                    utils::print_warning("File not found");
                    if !create {
                        process::exit(1);
                    }
                    println!("{}", "Creating...".bright_green());
                    OpenOptions::new()
                        .write(true)
                        .read(true)
                        .create(true)
                        .open(&filename)
                        .unwrap_or_else(|e| utils::print_error!("Unable to create the file: {}", e))
                } else {
                    utils::print_error!("Something went wrong opening the file: {}", err);
                }
            });

        file
    }

    pub fn update_file(filename: &str, content: String) {
        let mut new_file = File::create(&filename).unwrap_or_else(|e| {
            utils::print_error!("Something went wrong creating the file: {}", e)
        });
        new_file
            .write(content.as_bytes())
            .unwrap_or_else(|e| utils::print_error!("Unable to write on the file: {}", e));
    }
}
