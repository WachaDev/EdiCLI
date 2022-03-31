use clap::ArgMatches;

use crate::editor::{action, status};

pub fn run(app: ArgMatches) {
    let mut file = match app.subcommand() {
        Some(("write", sub_command)) => {
            let file = sub_command.value_of_os("file").unwrap().to_str().unwrap();
            let text = sub_command.values_of("text").unwrap().collect();
            action::write(file, text)
        },
        Some(("rewrite", sub_command)) => { 
            let file = sub_command.value_of_os("file").unwrap().to_str().unwrap();
            let line: usize = sub_command.value_of("line").unwrap().parse().unwrap();
            let text = sub_command.value_of("text").unwrap();
            action::rewrite(file, line, text)
        },
        Some(("delete", sub_command)) => {
            let file = sub_command.value_of("file").unwrap();
            let line: usize = sub_command.value_of("line").unwrap().parse().unwrap();
            action::delete(file, line)
        }, 
        _ => status::print_error!("Input not given"),
    };

    if app.is_present("show") {
        action::show(&mut file);
    }

}