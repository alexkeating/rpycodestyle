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
    let mut blank_lines = 0;
    let total_lines = content.lines().count();
    for (index, line) in content.lines().enumerate() {
        blank_lines = increment_blank_lines(line, &blank_lines);
        if index > 0 {
            let previous_line = content.lines().nth(index - 1).unwrap();
            reporting(&path, index + 1, line, total_lines, previous_line,
                      blank_lines);
        }
        else {
            reporting(&path, index + 1, line, total_lines, "", blank_lines);
        }
    }
}

fn increment_blank_lines(line: &str, &blank_lines: &usize) -> usize {
    if line.is_empty() {
        let num_blank_lines = blank_lines + 1;
        num_blank_lines
    } else {
        let num_blank_lines = 0;
        num_blank_lines
    }
}



//Logical lines