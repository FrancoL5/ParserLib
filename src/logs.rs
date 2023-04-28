use chrono;
use std::fs::OpenOptions;
use std::io::prelude::*;
pub fn output_error(err: std::io::Error, arguments: &str) {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open("log.txt")
        .unwrap();

    writeln!(
        file,
        "fecha: {}\n{}\n{}\n---------------",
        chrono::offset::Local::now(),
        err.to_string(),
        arguments
    )
    .unwrap();
}
