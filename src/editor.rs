use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::io::{self, BufRead};
use std::path::Path;

use colored::*;

pub fn write<P: AsRef<Path>>(filename: P, texts: Vec<&str>) {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .append(true)
        .open(&filename);

    let mut file = match file {
        Ok(f) => f,
        Err(_) => {
            print_warning_msg("File not found, creating...");
            File::create(filename).expect("Unable to open the) file")
        }
    };

    for text in texts {
        writeln!(&mut file, "{}", text).expect("Something went wrong writing the file");
    }

    print_success_msg("The file was written succesfully!");
}

pub fn delete<P: AsRef<Path>>(filename: P, line: usize) {
    let file = OpenOptions::new().read(true).write(true).open(&filename);
    let mut file = match file {
        Ok(f) => f,
        Err(_) => {
            eprintln!("The given file doesn't exist");
            return;
        }
    };

    let mut file_copy = file.try_clone().unwrap();
    let mut content = String::new();
    let file_len = content.lines().collect::<Vec<&str>>();

    let f_lines = io::BufReader::new(file_copy).lines();

    for (i, f_line) in f_lines.enumerate() {
        if let Ok(ip) = f_line {
            if i == line {
                println!("Line: {}\nWith the content: {}\n", line, ip);
                print_success_msg("Has been deleted succesfully!");
                break;
            }
        }
    }

    // println!("That line doesn't exist on the file");
}

fn print_success_msg(msg: &str) {
    let message = msg.green().bold();
    println!("{}", message);
}

fn print_warning_msg(msg: &str) {
    let message = msg.yellow().bold();
    println!("{}", message);
}
