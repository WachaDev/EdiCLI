mod setup;
mod editor;

use clap::{Command, arg};

fn main() {
    let cli = Command::new("edicli")
        .author("Wachamuli <josemrr27@gmail.com>")
        .version("0.1.1")
        .about("A quick editor for quick changes")
        .arg(arg!(-s --show "Show the file"))
        .subcommands([
            Command::new("write")
                .about("Write on a file")
                .args([
                    arg!(<file> "The file to edit").allow_invalid_utf8(true),
                    arg!(<text> "The text to be inserted").multiple_occurrences(true)
                ]),
            Command::new("rewrite")
                .about("Replace the selected line with the new text")
                .args([
                    arg!(<file> "The file to edit").allow_invalid_utf8(true),
                    arg!(<line> "Line to edit").validator(|l| l.parse::<usize>()),
                    arg!(<text> "The text to be inserted")
                ]),
            Command::new("delete")
                .about("Delete a selected line from a file")
                .args([
                    arg!(<file> "The file to edit").allow_invalid_utf8(true),
                    arg!(<line> "Line to delete").validator(|l| l.parse::<usize>())
                ])
        ])
        .get_matches();
        
    setup::run(cli);
}
