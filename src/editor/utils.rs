
use colored::*;

macro_rules! print_error {
    ($($args:tt)+) => {{
        use colored::*;
        println!("{}{}", "Error: ".red().bold(), format_args!($($args)*).to_string().red());
        std::process::exit(1)
    }}
}

pub(crate) use print_error;


pub fn print_success(msg: &str) {
    let message = msg.green().bold();
    println!("{}", message);
}

pub fn print_warning(msg: &str) {
    let message = "Warn: ".yellow().bold().to_string() + &msg.yellow().to_string();
    println!("{}", message);
}
