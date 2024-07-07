use std::fs::File;
use std::io::{Read, stdin};

pub fn get_reader(input: &str) -> Box<dyn Read> {
    let reader: Box<dyn Read> = if input == "-" {
        Box::new(stdin())
    } else {
        println!("Reading from file: {}", input);
        Box::new(File::open(input).unwrap())
    };
    reader
}
