extern crate regex;
use regex::Regex;

#[derive(PartialEq, Debug)]
pub struct Error {
    column_number: usize,
    error_message: String,
}

pub fn reporting(path: &String, line_number: usize, line: &str, total_lines: usize) {
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
                error_message: "E101 indentation contains mixed spaces and tabs".to_string()
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
        let error_message = "W191 indentation contains tabs".to_string();
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
        if !stripped_line.is_empty() {
            let message = "W291 trailing whitespace".to_string();
            let error = Error {
                column_number: stripped_line.len(),
                error_message: message
            };
            Some(error)
        }
        else {
            let message = "W293 blank line contains whitespace".to_string();
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
                error_message: "W391 blank line at end of file".to_string(),
                column_number: 0
            };
            return Some(error)
        }
        if stripped_last_line == line {
            let error = Error{
                error_message: "W292 no newline at end of file".to_string(),
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tabs_or_spaces_tabs_test() {
        let indent_char = ' ';
        let line = "\tHello world";
        let error = tabs_or_spaces(line, indent_char).unwrap();
        let expected_error = Error {
            column_number: 0,
            error_message: "E101 indentation contains mixed spaces and tabs".to_string()
        };
        assert_eq!(error, expected_error);
    }

    #[test]
    fn tabs_or_spaces_spaces_test() {
        let indent_char = '\t';
        let line = " Hello world";
        let error = tabs_or_spaces(line, indent_char).unwrap();
        let expected_error = Error {
            column_number: 0,
            error_message: "E101 indentation contains mixed spaces and tabs".to_string()
        };
        assert_eq!(error, expected_error);
    }

    #[test]
    fn tabs_or_spaces_base_test() {
        let indent_char = ' ';
        let line = " Hello world";
        let error = tabs_or_spaces(line, indent_char);
        assert_eq!(None, error);
    }

    #[test]
    fn tabs_obsolete_tab_test() {
        let line = "\tHello world";
        let error = tabs_obsolete(line).unwrap();
        let expected_error = Error {
            column_number: 0,
            error_message: "W191 indentation contains tabs".to_string()
        };
        assert_eq!(error, expected_error);
    }

    #[test]
    fn tabs_obsolete_base_test() {
        let line = "Hello world";
        let error = tabs_obsolete(line);
        assert_eq!(error, None);
    }

    #[test]
    fn trailing_whitespace_line_test() {
        let line = "Hello world ";
        let error = trailing_whitespace(line).unwrap();
        let expected_error = Error {
            error_message: "W291 trailing whitespace".to_string(),
            column_number: 11
        };
        assert_eq!(error, expected_error)
    }

    #[test]
    fn trailing_whitespace_blank_test() {
        let line = " ";
        let error = trailing_whitespace(line).unwrap();
        let expected_error = Error {
            error_message: "W293 blank line contains whitespace".to_string(),
            column_number: 0
        };
        assert_eq!(error, expected_error)
    }

    #[test]
    fn trailing_whitespace_base_test() {
        let line = "Hello world";
        let error = trailing_whitespace(line);
        assert_eq!(error, None)
    }

    #[test]
    fn trailing_blank_lines_new_line_test() {
        let line = "Hello world";
        let line_number = 10;
        let total_lines = 10;
        let error = trailing_blank_lines(line, line_number, total_lines).unwrap();
        let expected_error = Error {
            error_message: "W292 no newline at end of file".to_string(),
            column_number: 11
        };
        assert_eq!(error, expected_error)
    }

    #[test]
    fn trailing_blank_lines_test() {
//        I think this behavior is wrong
        let line = "";
        let line_number = 10;
        let total_lines = 10;
        let error = trailing_blank_lines(line, line_number, total_lines).unwrap();
        let expected_error = Error {
            error_message: "W391 blank line at end of file".to_string(),
            column_number: 0
        };
        assert_eq!(error, expected_error)
    }

    #[test]
    fn maximum_line_length_test() {
        let line = "Hello world";
        let max_line_length = 10;
        let error = maximum_line_length(line, max_line_length).unwrap();
        let expected_error = Error {
            error_message: "10 ES501 line to long (11 > 10 characters)".to_string(),
            column_number: 10
        };
        assert_eq!(error, expected_error)
    }

    #[test]
    fn maximum_line_length_none_test() {
        let line = "Hello world";
        let max_line_length = 11;
        let error = maximum_line_length(line, max_line_length);
        assert_eq!(error, None)
    }
}