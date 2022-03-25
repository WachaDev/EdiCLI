use std::{
    fs::{File, OpenOptions},
    io::{prelude::*, ErrorKind},
    path::Path,
    process,
};

use colored::*;

macro_rules! print_error {
    ($($args:tt)+) => {{
        use colored::*;
        println!("{}{}", "Error: ".red().bold(), format_args!($($args)*).to_string().red());
        std::process::exit(1)
    }}
}

pub(crate) use print_error;

pub fn open_file<P: AsRef<Path>>(filename: P, create: bool) -> File {
    let file = OpenOptions::new()
        .write(true)
        .read(false)
        .append(true)
        .open(&filename)
        .unwrap_or_else(|err| {
            if ErrorKind::NotFound == err.kind() {
                print_warning("File not found");
                if !create {
                    process::exit(1);
                }
                println!("{}", "Creating...".bright_green());
                File::create(filename)
                    .unwrap_or_else(|e| print_error!("Unable to create the file: {}", e))
            } else {
                print_error!("Something went wrong opening the file: {}", err);
            }
        });

    file
}

pub fn update_file<P: AsRef<Path>>(filename: P, content: String) {
    let mut new_file = File::create(&filename)
        .unwrap_or_else(|e| print_error!("Something went wrong creating the file: {}", e));
    new_file
        .write(content.as_bytes())
        .unwrap_or_else(|e| print_error!("Unable to write on the file: {}", e));
}

pub fn get_content(file: &mut File, line: usize) -> String {
    let mut content = String::new();
    if let Err(e) = file.read_to_string(&mut content) {
        print_error!("Something went wrong reading the file: {}", e);
    };
    drop(file);

    let length = content.lines().collect::<Vec<&str>>().len();

    if line > length {
        print_error!("The given line doesn't exist on the file");
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
