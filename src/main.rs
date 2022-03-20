use clap::Command;

mod editor;
mod subcommands;

fn main() {
    let matches = Command::new("edicli")
        .author("Wachamuli, josemrr27@gmail.com")
        .version("0.1")
        .about("A quick editor for quick changes")
        .subcommands([
            subcommands::command_write(),
            subcommands::command_delete(),
            subcommands::command_rewrite(),
        ])
        .get_matches();

    match matches.subcommand() {
        Some(("write", sub_command)) => editor::write(
            sub_command.value_of("file").unwrap(),
            sub_command.values_of("text").unwrap().collect::<Vec<&str>>()
        ),
        Some(("delete", sub_command)) => editor::delete(
            sub_command.value_of("file").unwrap(),
            sub_command.value_of("line").unwrap().parse::<usize>().unwrap() // TODO: Handle the Err case on `parse`
        ),
        _ => println!("Input not given"),
    }
}
