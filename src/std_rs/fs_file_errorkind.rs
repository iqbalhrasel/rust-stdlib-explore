use std::{fs::File, io::ErrorKind};

pub fn file_errorkind() {
    match File::open("data.txt") {
        Err(e) => match e.kind() {
            ErrorKind::NotFound => println!("file not found"),
            _ => println!("other error"),
        },
        Ok(_f) => println!("file opened"),
    }
}
