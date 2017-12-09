use std::cmp::PartialEq;
use std::env;
use std::fs::File;
use std::io::prelude::*;

extern crate regex;
use regex::Regex;

fn main() {
    let path = env::args().nth(1).unwrap();
    let mut file = File::open(&path).expect("File not found");
    let mut content = String::new();

    file.read_to_string(&mut content);
    let total_lines = content.lines().count() - 1;
    for (index, line) in content.lines().enumerate() {
        reporting(&path, index, line, total_lines)
    }
}

struct Error {
    column_number: usize,
    error_message: String,
}

impl PartialEq<Error> for Error {
    fn eq( &self, other: &Error) -> bool {
        self == other
    }

    fn ne( &self, other: &Error) -> bool {
        self != other
    }
}

fn reporting(path: &String, line_number: usize, line: &str, total_lines: usize) {
    let errors = checker(line, line_number, total_lines);
    for error_option in errors {
        if error_option != None {
            let error = error_option.unwrap();
            println!("./{}:{}:{} {}", path, line_number, error.column_number,
                     error.error_message);
        }
    }
}


fn checker(line: &str, line_number: usize, total_lines: usize) ->  Vec<Option<Error>> {
    let mut errors = Vec::new();
    //    Config variables
    let max_length = 120;
    let indent_char = '\t';


    errors.push(maximum_line_length(line, max_length));
    errors.push(tabs_or_spaces(line, indent_char));
    errors.push(tabs_obsolete(line));
    errors.push(trailing_whitespace(line));
    errors.push(trailing_blank_lines(line, line_number, total_lines));
    errors
}

// Physical lines
fn tabs_or_spaces(line: &str, indent_char: char) -> Option<Error> {
//    Never mix tabs and spaces.
//
//    The most popular way of indenting Python is with spaces only.  The
//    second-most popular way is with tabs only.  Code indented with a mixture
//    of tabs and spaces should be converted to using spaces exclusively.  When
//    invoking the Python command line interpreter with the -t option, it issues
//    warnings about code that illegally mixes tabs and spaces.  When using -tt
//    these warnings become errors.  These options are highly recommended!
//
//    Okay: if a == 0:\n        a = 1\n        b = 1
//    E101: if a == 0:\n        a = 1\n\tb = 1


    let re = Regex::new("([ \t]*)").unwrap();
    let indent = re.find(line).unwrap();
    for (offset, char) in indent.as_str().chars().enumerate() {
        if char != indent_char {
            let error = Error {
                column_number: offset,
                error_message: format!("E101 indentation contains mixed spaces and tabs")
            };
            return Some(error)
        }
    }
    None
}

fn tabs_obsolete(line: &str) -> Option<Error> {
    let re = Regex::new("([ \t]*)").unwrap();
    let indent = re.find(line).unwrap();
    if indent.as_str().contains('\t') {
        let error_message = format!("W191 indentation contains tabs");
        let column_number = indent.as_str().find('\t').unwrap();
        let error = Error {
            error_message: error_message,
            column_number: column_number,
        };
        Some(error)
    }
    else {
        None
    }
}

fn trailing_whitespace(line: &str) -> Option<Error>{
    //    Trailing whitespace is superfluous.
    //
    //    The warning returned varies on whether the line itself is blank, for easier
    //    filtering for those who want to indent their blank lines.
    //
    //    Okay: spam(1)\n#
    //    W291: spam(1) \n#
    //    W293: class Foo(object):\n    \n    bang = 12
    let stripped_line = line.trim_right();
    if line != stripped_line {
        if stripped_line.is_empty() {
            let message = format!("W291 trailing whitespace");
            let error = Error {
                column_number: stripped_line.len(),
                error_message: message
            };
            Some(error)
        }
        else {
            let message = format!("W293 blank line contains whitespace");
            let error = Error {
                error_message: message,
                column_number: 0,
            };
            Some(error)
        }
    }
    else {
        None
    }
}

fn trailing_blank_lines(line: &str, line_number: usize,
                        total_lines: usize) -> Option<Error>{
    //    Trailing blank lines are superfluous.
    //
    //    Okay: spam(1)
    //    W391: spam(1)\n
    //
    //    However the last line should end with a new line (warning W292).
    if line_number == total_lines {
        let stripped_last_line = line.trim_right();
        if stripped_last_line.is_empty() {
            let error = Error{
                error_message: format!("W391 blank line at end of file"),
                column_number: 0
            };
            return Some(error)
        }
        if stripped_last_line == line {
            let error = Error{
                error_message: format!("W292 no newline at end of file"),
                column_number: line.len()
            };
            return Some(error)
        }
        return None
    }
    return None
}

fn maximum_line_length(line: &str, max_line_length: usize) -> Option<Error> {
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
        let error_message = format!("{} ES501 line to long ({} > {} characters)",
                                    max_line_length, length, max_line_length);
        let error = Error{
            error_message: error_message,
            column_number: max_line_length
        };
        Some(error)
    }
    else {
        None
    }
}

//Logical lines