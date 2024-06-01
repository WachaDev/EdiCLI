macro_rules! print_success {
    ($($args:tt)+) => {{
        use colored::*;
        println!("{}", format_args!($($args)*).to_string().green().bold());
    }}
}

macro_rules! print_warning {
    ($($args:tt)+) => {{
        use colored::*;
        println!("{}{}", "Warn: ".yellow().bold(), format_args!($($args)*).to_string().yellow());
    }}
}

macro_rules! print_error {
    ($($args:tt)+) => {{
        use colored::*;
        println!("{}{}", "Error: ".red().bold(), format_args!($($args)*).to_string().red());
        std::process::exit(1)
    }}
}


pub(crate) use print_error;
pub(crate) use print_success;
pub(crate) use print_warning;
