mod editor;
mod subcommands;

use clap::{Command, arg};

fn main() {
    let matches = Command::new("edicli")
        .author("Wachamuli <josemrr27@gmail.com>")
        .version("0.1")
        .about("A quick editor for quick changes")
        .arg(arg!(-s --show "Show the file"))
        .subcommands([
            subcommands::command_write(),
            subcommands::command_delete(),
            subcommands::command_rewrite(),
        ])
        .get_matches();

    let mut file = match matches.subcommand() {
        Some(("write", sub_command)) => editor::write(
            sub_command.value_of_os("file").unwrap().to_str().unwrap(),
            sub_command.values_of("text").unwrap().collect()
        ),
        Some(("rewrite", sub_command)) => editor::rewrite(
            sub_command.value_of_os("file").unwrap().to_str().unwrap(),
            sub_command.value_of("line").unwrap().parse::<usize>().unwrap(),
            sub_command.value_of("text").unwrap()
        ).unwrap(),
        Some(("delete", sub_command)) => editor::delete(
            sub_command.value_of("file").unwrap(),
            sub_command.value_of("line").unwrap().parse::<usize>().unwrap()
        ).unwrap(),
        _ => editor::utils::print_error!("Input not given"),
    };

    if matches.is_present("show") {
        editor::show(&mut file);
    }
}
