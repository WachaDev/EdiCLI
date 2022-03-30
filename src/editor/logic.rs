use std::{io::{Write, Seek}, path::Path};

use super::{my_file::MyFile, utils};

use colored::*;

pub fn show(file: &mut MyFile) {
    print!(
        "\
        +------------------------------------------->\n\
        | {}\n\
        +------------------------------------------->\n",
        &file.filename(),
    );
    print!("{}", &file.get_content())
}

pub fn write(filename: &str, texts: Vec<&str>) -> MyFile {
    let mut file = MyFile::new(filename, true);

    for text in texts {
        if let Err(e) = writeln!(&mut file.file(), "{}", text) {
            utils::print_error!("Unable to write on the file: {}", e);
        };
    }

    utils::print_success("The file was written successfully!");

    file
}

// FIXME: Replace every line with the same content of the given line
pub fn rewrite(filename: &str, line: usize, text: &str) -> Option<MyFile> {
    let mut file = MyFile::new(filename, false);

    if line > file.get_lines().len() {
        utils::print_error!("The given line doesn't exist on the file");
    }

    for (i, c_line) in file.get_lines().iter().enumerate() {
        if i == line - 1 {
            if c_line == "" {
                utils::print_warning("The line is empty");
                break;
            }

            let new_content = file.get_content().replace(c_line, text);
            MyFile::update_file(filename, new_content);

            println!(
                "Line: {}\n - {}\n + {}\n",
                line,
                c_line.bright_red(),
                text.bright_green()
            );
            utils::print_success("The line has been rewritten successfully!");
            return Some(file);
        }
    }
    None
}

// FIXME: Deletes every line with the same content of the given line
pub fn delete(filename: &str, line: usize) -> Option<MyFile> {
    let mut file = MyFile::new(&filename, false);

    if line > file.get_lines().len() {
        utils::print_error!("The given line doesn't exist on the file");
    }

    for (i, c_line) in file.get_lines().iter().enumerate() {
        if i == line - 1 {
            let blank_space = file.get_content().replace(c_line, "");
            MyFile::update_file(filename, blank_space);

            utils::print_success("The line has been deleted successfully!");
            return Some(file);
        }
    }
    None
}
