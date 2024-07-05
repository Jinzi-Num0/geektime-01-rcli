use std::{fs::File, io::Read};

pub fn get_reader(input: &str) -> Box<dyn Read> {
    if input == "-" {
        Box::new(std::io::stdin())
    } else {
        Box::new(File::open(input).unwrap())
    }
}
