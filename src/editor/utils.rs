use std::{
    fs::{File, OpenOptions},
    io::{prelude::*, ErrorKind},
    path::Path,
};

use colored::*;

pub fn open_file<P: AsRef<Path>>(filename: P, create: bool) -> File {
    let file = OpenOptions::new()
        .write(true)
        .read(true)
        .append(true)
        .open(&filename)
        .unwrap_or_else(|err| {
            if ErrorKind::NotFound == err.kind() {
                print_warning("File not found");
                if create {
                    println!("{}", "Creating...".bright_green());
                    return File::create(filename).expect("Unable to create the file");
                }
                panic!();
            } else {
                panic!("Something went wrong opening the file: {}", err);
            }
        });

    file
}

pub fn get_content(file: &mut File, line: usize) -> String {
    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect("Something went wrong");
    drop(file);

    let length = content.lines().collect::<Vec<&str>>().len();

    if line > length {
        panic!("The line #{} doesn't exist on the file", line);
    }

    content
}

pub fn print_success(msg: &str) {
    let message = msg.green().bold();
    println!("{}", message);
}

pub fn print_warning(msg: &str) {
    let message = "Warn: ".yellow().bold().to_string() + &msg.yellow().to_string();
    println!("{}", message);
}

#[allow(dead_code)]
pub fn print_error(msg: &str) {
    let message = "Error: ".red().bold().to_string() + &msg.red().to_string();
    println!("{}: ", message);
}
