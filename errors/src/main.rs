use std::fs::{File, read_to_string};
use std::io;
use std::io::{ErrorKind, Read};

fn main() {
//    let f = File::open("hello.txt");
//    let f = match f {
//        Ok(file) => file,
//        Err(error) => match error.kind() {
//            ErrorKind::NotFound => match File::create("hello.txt") {
//                Ok(file) => file,
//                Err(error) => panic!("Tried to create a file but there was a probleme: {:?}", error),
//            },
//            error => panic!("There was a problem opening the file {:?}", error),
//        }
//    };
//    let f = File::open("hello.txt").unwrap();
    let result = read_username_from_file().unwrap();
    println!("{:?}", result);
}

fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}
