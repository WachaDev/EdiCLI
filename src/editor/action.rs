
use std::{io::Write, path::Path};

use colored::*;

use super::{file::File, status};

pub fn show<P: AsRef<Path>>(file: &mut File<P>) {
    let corner_sign = '+';
    let topbottom_sign = '-';
    let side_sign = '|';

    let content = &file
        .lines()
        .iter()
        .enumerate()
        .map(|(i, line)| {
            let line_number = i + 1;
            format!("{line_number}   | ") + line
        })
        .collect::<Vec<String>>()
        .join("\n");
    let filename = &file.filename().to_str().unwrap();

    let left_padding = " ".to_string();
    let width = 50;
    let adjusted_width = width - (filename.len() + left_padding.len());

    let mut topbottom_border = corner_sign.to_string();
    let mut side_border = side_sign.to_string() + &left_padding + filename;

    for _ in 0..width {
        topbottom_border.push(topbottom_sign);
    }

    for _ in 0..adjusted_width {
        side_border.push(' ');
    }

    topbottom_border.push(corner_sign);
    side_border.push(side_sign);

    println!(
        "                   \n\
        {topbottom_border}  \n\
        {side_border}       \n\
        {topbottom_border}  \n\
        {content}
        "
    );
}

pub fn write<P: AsRef<Path>>(filename: P, texts: Vec<&str>) -> File<P> {
    let mut file = File::new(filename, true);

    for text in texts {
        if let Err(e) = writeln!(&mut file.file(), "{text}") {
            status::print_error!("Unable to write on the file: {e}");
        };
    }

    status::print_success!("The file was written successfully!");

    file
}

pub fn rewrite<P: AsRef<Path>>(filename: P, line: usize, text: &str) -> File<P> {
    let mut file = File::new(filename, false);
    let mut file_lines = file.lines();
    File::check_existing_line(line, &mut file);

    let old_line = file_lines[line - 1].clone();
    file_lines[line - 1] = text.to_string();
    let new_content = file_lines.join("\n");
    File::update(file.filename(), new_content);

    println!(
        "\
        Line: {line}   \n\
            - {c_line} \n\
            + {text}   \n
        ",
        line = line,
        c_line = old_line.bright_red(),
        text = text.bright_green()
    );

    status::print_success!("The line has been rewritten successfully!");
    return file;
}

pub fn delete<P: AsRef<Path>>(filename: P, line: usize) -> File<P> {
    let mut file = File::new(filename, false);
    let mut file_lines = file.lines();
    File::check_existing_line(line, &mut file);

    file_lines[line - 1] = "".to_string();
    let new_content = file_lines.join("\n");
    File::update(file.filename(), new_content);

    status::print_success!("The line has been deleted successfully!");
    return file;
}
