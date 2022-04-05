use clap::ArgMatches;

use crate::editor::{action, status};

pub fn run(cli: ArgMatches) {
    let mut file = match cli.subcommand() {
        Some(("write", sub_command)) => {
            let file = sub_command.value_of_os("file").unwrap();
            let text = sub_command.values_of("text").unwrap().collect();

            action::write(file, text)
        },
        Some(("rewrite", sub_command)) => { 
            let file = sub_command.value_of_os("file").unwrap();
            let line = sub_command.value_of_t("line").unwrap();
            let text = sub_command.value_of("text").unwrap();

            action::rewrite(file, line, text)
        },
        Some(("delete", sub_command)) => {
            let file = sub_command.value_of_os("file").unwrap();
            let line = sub_command.value_of_t("line").unwrap();

            action::delete(file, line)
        }, 
        _ => status::print_error!("Input not given"),
    };

    if cli.is_present("show") {
        action::show(&mut file);
    }
}