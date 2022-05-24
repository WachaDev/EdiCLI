use clap::ArgMatches;

use crate::editor::{action, status};

pub fn run(cli: ArgMatches) {
    let mut file = match cli.subcommand() {
        Some(("write", sub_command)) => {
            action::write(
                sub_command.value_of_os("file").unwrap(),
                sub_command.values_of("text").unwrap().collect()
            )
        }
        Some(("rewrite", sub_command)) => {
            action::rewrite(
                sub_command.value_of_os("file").unwrap(),
                sub_command.value_of_t("line").unwrap(),
                sub_command.value_of("text").unwrap()
            )
        }
        Some(("delete", sub_command)) => {
            action::delete(
                sub_command.value_of_os("file").unwrap(),
                sub_command.value_of_t("line").unwrap()
            )
        }
        _ => status::print_error!("Input not given"),
    };

    if cli.is_present("show") {
        action::show(&mut file);
    }
}
