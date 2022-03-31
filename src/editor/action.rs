use std::{io::Write, path::Path};

use super::{file::File, status};

use colored::*;

pub fn show<P: AsRef<Path>>(file: &mut File<P>) {
    let content = &file.get_content();
    let path = &file.filename().display();
    println!(
        "+------------------------------------------->\n\
        | {path}\n\
        +------------------------------------------->\n\
        {content}"
    );
}

pub fn write<P: AsRef<Path>>(filename: P, texts: Vec<&str>) -> File<P> {
    let mut file = File::new(filename, true);

    for text in texts {
        if let Err(e) = writeln!(&mut file.file(), "{}", text) {
            status::print_error!("Unable to write on the file: {e}");
        };
    }

    status::print_success("The file was written successfully!");
    file
}

// FIXME: Replace every line with the same content of the given line
pub fn rewrite<P: AsRef<Path>>(filename: P, line: usize, text: &str) -> File<P> {
    let mut file = File::new(filename, false);

    if line > file.get_lines().len() {
        status::print_error!("The given line doesn't exist on the file");
    }

    for (i, c_line) in file.get_lines().iter().enumerate() {
        if i == line - 1 {
            if c_line == "" {
                status::print_error!("The line is empty");
            }

            let new_content = file.get_content().replace(c_line, text);
            File::update_file(file.filename(), new_content);
            println!(
                "Line: {line}\n\
                    - {c_line}\n\
                    + {text}\n",
                line = line, c_line = c_line.bright_red(), text = text.bright_green()
            );
            status::print_success("The line has been rewritten successfully!");
            return file;
        }
    }
    unreachable!()
}

// FIXME: Deletes every line with the same content of the given line
pub fn delete<P: AsRef<Path>>(filename: P, line: usize) -> File<P> {
    let mut file = File::new(filename, false);

    if line > file.get_lines().len() {
        status::print_error!("The given line doesn't exist on the file");
    }

    for (i, c_line) in file.get_lines().iter().enumerate() {
        if i == line - 1 {
            let blank_space = file.get_content().replace(c_line, "");
            File::update_file(file.filename(), blank_space);
            status::print_success("The line has been deleted successfully!");
            return file;
        }
    }
    unreachable!()
}
