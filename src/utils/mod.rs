use std::fs::File;
use std::io::Read;

pub mod square_grid;

pub fn read_file(filename: &str) -> String {
    let mut input = String::new();
    let mut file = File::open(format!("input/{}", filename)).unwrap();
    file.read_to_string(&mut input).unwrap();
    input
}