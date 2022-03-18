use clap::{arg, Command};

pub fn command_write() -> Command<'static> {
    let command = Command::new("write")
        .about("Write on a file")
        .args([
            arg!(<file> "The file to edit"),
            arg!(<text> "The text to be inserted"),
        ]);
    command
}

pub fn command_delete() -> Command<'static> {
    let command = Command::new("delete")
        .about("Delete the selected line")
        .args([
            arg!(<file> "The file to edit"),
            arg!(<line> "Line to edit")
        ]);
    command
}

pub fn command_rewrite() -> Command<'static> {
    let command = Command::new("rewrite")
        .about("Replace the selected line with the new text")
        .args([
            arg!(<file> "The file to edit"),
            arg!(<line> "Line to edit"),
            arg!(<text> "The text to be inserted")
        ]);
    command
}
