use std::{io::Write, path::Path};

use super::utils;

use colored::*;

pub fn write<P: AsRef<Path>>(filename: P, texts: Vec<&str>) {
    let mut file = utils::open_file(filename, true);

    for text in texts {
        if let Err(e) = writeln!(&mut file, "{}", text) {
            utils::print_error!("Unable to write on the file: {}", e);
        };
    }

    utils::print_success("The file was written successfully!");
}

// FIXME: Replace every line with the same content of the given line
pub fn rewrite<P: AsRef<Path>>(filename: P, line: usize, text: &str) {
    let mut file = utils::open_file(&filename, false);
    let content = utils::get_content(&mut file, line);
    let lines = content.lines();

    for (i, c_line) in lines.enumerate() {
        if i == line - 1 {
            if c_line == "" {
                utils::print_warning("The line is empty");
                break;
            }

            let new_content = content.replace(c_line, text);
            utils::update_file(filename, new_content);

            println!(
                "Line: {}\n - {}\n + {}\n",
                line,
                c_line.bright_red(),
                text.bright_green()
            );
            utils::print_success("The line has been rewritten successfully!");
            break;
        }
    }
}

// FIXME: Deletes every line with the same content of the given line
pub fn delete<P: AsRef<Path>>(filename: P, line: usize) {
    let mut file = utils::open_file(&filename, false);
    let content = utils::get_content(&mut file, line);
    let lines = content.lines();

    for (i, c_line) in lines.enumerate() {
        if i == line - 1 {
            let blank_space = content.replace(c_line, "");
            utils::update_file(filename, blank_space);

            utils::print_success("The line has been deleted successfully!");
            break;
        }
    }
}
