use std::env;
use std::fs::File;
use std::io::prelude::*;

extern crate rpycodestyle;
use rpycodestyle::reporting;

fn main() {
    let path = env::args().nth(1).unwrap();
    let mut file = File::open(&path).expect("File not found");
    let mut content = String::new();

    file.read_to_string(&mut content);
    let total_lines = content.lines().count();
    for (index, line) in content.lines().enumerate() {
        reporting(&path, index + 1, line, total_lines)
    }
}



//Logical lines