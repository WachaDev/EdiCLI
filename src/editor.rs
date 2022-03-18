use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::io::{self, BufRead};
use std::path::Path;

pub fn write<P: AsRef<Path>>(filename: P, text: &str) {
    let file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(&filename);

    let mut file = match file {
        Ok(f) => f,
        Err(_) => {
            eprintln!("File not found, creating...");
            File::create(filename).expect("Unable to open the file")
        }
    };

    writeln!(&mut file, "{}", text);
    println!("The file was written succesfully!")
}

pub fn delete<P: AsRef<Path>>(
    filename: P,
    line_: u8,
) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open("foo.txt")?;
    let lines = io::BufReader::new(file).lines();

    for line in lines {
        if let Ok(ip) = line {
            println!("{}", ip);
        }
    }
    Ok(io::BufReader::new(File::open("foo.txt")?).lines())
}
