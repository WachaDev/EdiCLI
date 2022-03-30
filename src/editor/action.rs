use std::{io::Write, path::Path};

use super::{file::File, status};

use colored::*;

pub fn show(file: &mut File) {
    print!(
        "\
        +------------------------------------------->\n\
        | {}\n\
        +------------------------------------------->\n",
        &file.filename(),
    );
    println!("{}", &file.get_content())
}

pub fn write(filename: &str, texts: Vec<&str>) -> File {
    let mut file = File::new(filename, true);

    for text in texts {
        if let Err(e) = writeln!(&mut file.file(), "{}", text) {
            status::print_error!("Unable to write on the file: {}", e);
        };
    }

    status::print_success("The file was written successfully!");
    file
}

// FIXME: Replace every line with the same content of the given line
pub fn rewrite(filename: &str, line: usize, text: &str) -> File {
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
            File::update_file(filename, new_content);

            println!(
                "Line: {}\n - {}\n + {}\n",
                line,
                c_line.bright_red(),
                text.bright_green()
            );
            status::print_success("The line has been rewritten successfully!");
            return file;
        }
    }
    unreachable!()
}

// FIXME: Deletes every line with the same content of the given line
pub fn delete(filename: &str, line: usize) -> File {
    let mut file = File::new(&filename, false);

    if line > file.get_lines().len() {
        status::print_error!("The given line doesn't exist on the file");
    }

    for (i, c_line) in file.get_lines().iter().enumerate() {
        if i == line - 1 {
            let blank_space = file.get_content().replace(c_line, "");
            File::update_file(filename, blank_space);

            status::print_success("The line has been deleted successfully!");
            return file;
        }
    }
    unreachable!()
}
