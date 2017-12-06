use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let path = env::args().nth(1).unwrap();
    let mut file = File::open(&path).expect("File not found");
    let mut content = String::new();
    let max_length = 120;
    file.read_to_string(&mut content);
    for (index, line) in content.lines().enumerate() {
        let error = maximum_line_length(line, max_length);
        if error != "skip" {
             println!("./{}:{}:{} {}", path, index, max_length, error);
        }
    }
}

fn maximum_line_length(line: &str, max_line_length: usize) -> String {
//    Limit all lines to a maximum of 79 characters.
//
//    There are still many devices around that are limited to 80 character
//    lines; plus, limiting windows to 80 characters makes it possible to have
//    several windows side-by-side.  The default wrapping on such devices looks
//    ugly.  Therefore, please limit all lines to a maximum of 79 characters.
//    For flowing long blocks of text (docstrings or comments), limiting the
//    length to 72 characters is recommended.
//
//    Reports error E501.
    let length = line.len();
    if length > max_line_length {
        format!("{} ES501 line to long ({} > {} characters)",
                    max_line_length, length, max_line_length)
    }
    else {
        format!("skip")
    }
}