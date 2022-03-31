use clap::{arg, Command};

pub fn command_write<'a>() -> Command<'a> {
    Command::new("write")
        .about("Write on a file")
        .args([
            arg!(<file> "The file to edit").allow_invalid_utf8(true),
            arg!(<text> "The text to be inserted").multiple_occurrences(true)
        ])
}

pub fn command_delete<'a>() -> Command<'a> {
    Command::new("delete")
        .about("Delete a selected line from a file")
        .args([
            arg!(<file> "The file to edit").allow_invalid_utf8(true),
            arg!(<line> "Line to delete").validator(|l| l.parse::<usize>())
        ])
}

pub fn command_rewrite<'a>() -> Command<'a> {
    Command::new("rewrite")
        .about("Replace the selected line with the new text")
        .args([
            arg!(<file> "The file to edit").allow_invalid_utf8(true),
            arg!(<line> "Line to edit").validator(|l| l.parse::<usize>()),
            arg!(<text> "The text to be inserted")
        ])
}
