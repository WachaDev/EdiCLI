use std::{fs::File, io::Write, path::Path};

use super::utils;

use colored::*;

pub fn write<P: AsRef<Path>>(filename: P, texts: Vec<&str>) {
    let mut file = utils::open_file(filename, true);

    for text in texts {
        writeln!(&mut file, "{}", text).expect("Something went wrong writing the file");
    }

    utils::print_success("The file was written succesfully!");
}

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
            let mut new_file = File::create(&filename).expect("Unable to create the file");
            new_file
                .write(new_content.as_bytes())
                .expect("Unable to write on the file");

            println!(
                "Line: {}\n - {}\n + {}\n",
                line,
                c_line.bright_red(),
                text.bright_green()
            );
            utils::print_success("The line has been rewritten succesfully!");
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
            let new_content = content.replace(c_line, "");
            let mut new_file = File::create(&filename).expect("Unable to open the file");
            new_file
                .write(new_content.as_bytes())
                .expect("Unable to write on the file");

            utils::print_success("The line has been deleted succesfully!");
            break;
        }
    }
}
