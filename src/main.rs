mod setup;
mod editor;

use clap::{Command, arg};

fn main() {
    let app = Command::new("edicli")
        .author("Wachamuli <josemrr27@gmail.com>")
        .version("0.1")
        .about("A quick editor for quick changes")
        .arg(arg!(-s --show "Show the file"))
        .subcommands([
            setup::subcommand::command_write(),
            setup::subcommand::command_rewrite(),
            setup::subcommand::command_delete(),
        ])
        .get_matches();
        
    setup::run(app);
}
