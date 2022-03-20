use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::path::Path;

use colored::*;

// TODO: Handle errors with Clap API

pub fn write<P: AsRef<Path>>(filename: P, texts: Vec<&str>) {
    let file = OpenOptions::new()
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

pub fn rewrite<P: AsRef<Path>>(filename: P, line: usize, text: &str) {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(&filename);

    let mut file = match file {
        Ok(f) => f,
        Err(_) => {
            eprintln!("The given file doesn't exist");
            return;
        }
    };

    let mut buffer = String::new();
    file.read_to_string(&mut buffer).expect("Something went wrong trying to read the file");
    drop(file);

    let f_lines = buffer.lines();
    let f_len = buffer.lines().collect::<Vec<&str>>().len();

    if line >= f_len {
        println!("The line #{} doesn't exist on the file", line);
        return;
    }

    for (i, f_line) in f_lines.enumerate() {
        if i == line {
            println!("Line: {}\nWith the content: {}\n", line, f_line);
            print_success_msg("The line has been deleted succesfully!");

            let new_content = buffer.replace(f_line, text);
            let mut new_file = File::create(filename).expect("goo");
            new_file.write(new_content.as_bytes());

            break;
        }
    }

}

// FIXME: This function deletes every line with the same content of the given line
pub fn delete<P: AsRef<Path>>(filename: P, line: usize) {
    rewrite(filename, line, "");
}

fn print_success_msg(msg: &str) {
    let message = msg.green().bold();
    println!("{}", message);
}

fn print_warning_msg(msg: &str) {
    let message = msg.yellow().bold();
    println!("{}", message);
}
