use std::path::Path;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::io::ErrorKind;

use colored::*;


pub fn open_file<P: AsRef<Path>>(filename: P) -> File {
    let file = OpenOptions::new()
        .write(true)
        .read(true)
        .append(true)
        .open(&filename)
        .unwrap_or_else(|err| {
            if ErrorKind::NotFound == err.kind() {
                print_warning("File not found");
                File::create(filename).expect("Unable to create the file")
            } else {
                panic!("Unable to open the file: {}", err);
            } 
        });
    
    file
}

pub fn get_content(file: &mut File, line: usize) -> String {
    let mut content = String::new();
    file.read_to_string(&mut content).expect("Something went wrong");
    drop(file);

    let length = content.lines().collect::<Vec<&str>>().len();

    if line >= length {
        panic!("The line #{} doesn't exist on the file", line);
    }

    content
}

pub fn print_success(msg: &str) {
    let message = msg.green().bold();
    println!("{}", message);
}

pub fn print_warning(msg: &str) {
    let message = msg.yellow().bold();
    println!("{}", message);
}

#[allow(dead_code)]
pub fn print_error(msg: &str) {
    let message = msg.red().bold();
    println!("{}", message);
}