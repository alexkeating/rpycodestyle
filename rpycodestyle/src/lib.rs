extern crate regex;
use regex::Regex;

#[derive(PartialEq, Debug, Clone)]
pub struct Error {
    column_number: usize,
    error_message: String,
}

pub fn reporting(path: &String, line_number: usize, line: &str, total_lines: usize,
                 previous_line: &str, num_blank_lines: usize) {
    let errors = checker(line, line_number, total_lines, previous_line,
                         num_blank_lines);
    for error_option in errors {
        if error_option != None {
            let error = error_option.unwrap();
            println!("./{}:{}:{} {}", path, line_number, error.column_number,
                     error.error_message);
        }
    }
}


fn checker(line: &str, line_number: usize, total_lines: usize,
           previous_line: &str, num_blank_lines: usize) ->  Vec<Option<Error>> {
    let mut errors = Vec::new();
    //    Config variables
    let max_length = 120;
    let indent_char = ' ';


    errors.push(maximum_line_length(line, max_length));
    errors.push(tabs_or_spaces(line, indent_char));
    errors.push(tabs_obsolete(line));
    errors.push(trailing_whitespace(line));
    errors.push(trailing_blank_lines(line, line_number, total_lines));
    errors.push(blank_lines(line, line_number, previous_line, num_blank_lines));
    errors.extend(extraneous_whitespace(line).iter().cloned());
    errors.extend(whitespace_around_keywords(line).iter().cloned());
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


    let re = Regex::new(r"([ \t]*)").unwrap();
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
    let re = Regex::new(r"([ \t]*)").unwrap();
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

fn blank_lines(_line: &str, _line_number: usize, previous_line: &str,
               num_blank_lines: usize) -> Option<Error> {
    // Not implementing 306, 301, 302, 305
    if previous_line.starts_with("@") {
        let error = Error {
            error_message: "E304 blank lines found after function decorator".to_string(),
            column_number: 0,
        };
        Some(error)
    }
    else if num_blank_lines > 2 {
        let error = Error {
            error_message: format!("E303 too many blank lines {}", num_blank_lines),
            column_number: 0,
        };
        Some(error)
    }
    else {
        None
    }
}

fn extraneous_whitespace(line: &str) -> Vec<Option<Error>> {
    //    Avoid extraneous whitespace.
    //
    //    Avoid extraneous whitespace in these situations:
    //    - Immediately inside parentheses, brackets or braces.
    //    - Immediately before a comma, semicolon, or colon.
    //
    //    Okay: spam(ham[1], {eggs: 2})
    //    E201: spam( ham[1], {eggs: 2})
    //    E201: spam(ham[ 1], {eggs: 2})
    //    E201: spam(ham[1], { eggs: 2})
    //    E202: spam(ham[1], {eggs: 2} )
    //    E202: spam(ham[1 ], {eggs: 2})
    //    E202: spam(ham[1], {eggs: 2 })
    //
    //    E203: if x == 4: print x, y; x, y = y , x
    //    E203: if x == 4: print x, y ; x, y = y, x
    //    E203: if x == 4 : print x, y; x, y = y, x
    let re = Regex::new(r"[\[\(\{] | [\]\}\),;:]").unwrap();
    let mut errors = Vec::new();
    for match_ in re.find_iter(line) {
        let text = match_.as_str();
        let char = text.trim().to_string();
        let found = match_.start();
        let before_char = &line.chars().nth(found - 1).unwrap();

        if text == char.clone() + " " {
            let error = Error {
                error_message: format!("E201 whitespace after {}", &char),
                column_number: found + 1
            };
            errors.push(Some(error));
        } else if before_char != &',' {
            let error_code = determine_extraneous_whitespace_error_code(text.trim().chars().nth(0).unwrap());
            let error = Error {
                error_message: format!("{} whitespace before {}", error_code,
                                       &char),
                column_number: found + 1
            };
            errors.push(Some(error));
        }
    }
    errors
}

fn determine_extraneous_whitespace_error_code(char: char) -> &'static str {
    if char == '}' || char == ']' || char == ')' {
        "E202"
    }
    else {
        "E203"
    }
}

fn whitespace_around_keywords(line: &str) -> Vec<Option<Error>>{
    let keywords = get_keywords();
    let joined_keywords = keywords.join("|");
    let regex_string = format!(r"(\s*)\b(?:{})\b(\s*)", joined_keywords);
    let re = Regex::new(regex_string.as_str()).unwrap();
    let mut errors = Vec::new();
    for match_ in re.find_iter(line) {
        let start = match_.start();
        let end = match_.end();

        if line.chars().nth(start).unwrap() == '\t' {
            let error = Error {
                error_message: "E274 tab before keyword".to_string(),
                column_number: start
            };
            errors.push(Some(error))
        } else if line.chars().nth(start).unwrap() == ' ' &&
            line.chars().nth(start + 1).unwrap() == ' ' {
            let error = Error {
                error_message: "E272 multiple spaces before keyword".to_string(),
                column_number: start
            };
            errors.push(Some(error))
        }

        if line.chars().nth(end - 1).unwrap() == '\t' {
            let error = Error {
                error_message: "E273 tab after keyword".to_string(),
                column_number: end
            };
            errors.push(Some(error))
        } else if line.chars().nth(end - 1).unwrap() == ' ' &&
            line.chars().nth(end - 2).unwrap() == ' ' {
            let error = Error {
                error_message: "E271 multiple spaces after keyword".to_string(),
                column_number: end
            };
            errors.push(Some(error))
        }
    }
    errors
}

fn get_keywords() -> Vec<&'static str> {
//    python keywords with print added and True, False and
//    None removed
    vec!["and", "as", "assert", "break", "class", "continue",
         "def", "del", "elif", "else", "except",
         "finally", "for", "from", "global",
         "if", "import", "in", "is", "lambda", "nonlocal",
         "not", "or", "pass", "raise", "return",
         "try", "while", "with", "yield", "print"]
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

    #[test]
    fn extraneous_whitespace_after_paren() {
        let line = "spam( ham[1], {eggs: 2})";
        let error =  extraneous_whitespace(line);
        let expected_error = Error {
            error_message: "E201 whitespace after (".to_string(),
            column_number: 5
        };
        assert_eq!(error, vec![Some(expected_error)])
    }

    #[test]
    fn extraneous_whitespace_after_square_bracket() {
        let line = "spam(ham[ 1], {eggs: 2})";
        let error =  extraneous_whitespace(line);
        let expected_error = Error {
            error_message: "E201 whitespace after [".to_string(),
            column_number: 9
        };
        assert_eq!(error, vec![Some(expected_error)])
    }

    #[test]
    fn extraneous_whitespace_after_bracket() {
        let line = "spam(ham[1], { eggs: 2})";
        let error =  extraneous_whitespace(line);
        let expected_error = Error {
            error_message: "E201 whitespace after {".to_string(),
            column_number: 14
        };
        assert_eq!(error, vec![Some(expected_error)])
    }

    #[test]
    fn extraneous_whitespace_before_paren() {
        let line = "spam(ham[1], {eggs: 2} )";
        let error =  extraneous_whitespace(line);
        let expected_error = Error {
            error_message: "E202 whitespace before )".to_string(),
            column_number: 23
        };
        assert_eq!(error, vec![Some(expected_error)])
    }

    #[test]
    fn extraneous_whitespace_before_square_bracket() {
        let line = "spam(ham[1 ], {eggs: 2})";
        let error =  extraneous_whitespace(line);
        let expected_error = Error {
            error_message: "E202 whitespace before ]".to_string(),
            column_number: 11
        };
        assert_eq!(error, vec![Some(expected_error)])
    }

    #[test]
    fn extraneous_whitespace_before_bracket() {
        let line = "spam(ham[1], {eggs: 2 })";
        let error =  extraneous_whitespace(line);
        let expected_error = Error {
            error_message: "E202 whitespace before }".to_string(),
            column_number: 22
        };
        assert_eq!(error, vec![Some(expected_error)])
    }

    #[test]
    fn extraneous_whitespace_before_comma() {
        let line = "if x == 4: print x, y; x, y = y , x";
        let error =  extraneous_whitespace(line);
        let expected_error = Error {
            error_message: "E203 whitespace before ,".to_string(),
            column_number: 32
        };
        assert_eq!(error, vec![Some(expected_error)])
    }

    #[test]
    fn extraneous_whitespace_before_semi_colon() {
        let line = "if x == 4: print x, y ; x, y = y, x";
        let error =  extraneous_whitespace(line);
        let expected_error = Error {
            error_message: "E203 whitespace before ;".to_string(),
            column_number: 22
        };
        assert_eq!(error, vec![Some(expected_error)])
    }

    #[test]
    fn extraneous_whitespace_before_colon() {
        let line = "if x == 4 : print x, y; x, y = y, x";
        let error =  extraneous_whitespace(line);
        let expected_error = Error {
            error_message: "E203 whitespace before :".to_string(),
            column_number: 10
        };
        assert_eq!(error, vec![Some(expected_error)])
    }

    #[test]
    fn whitespace_around_keywords_space_after_and() {
        let line = "True and  False";
        let error =  whitespace_around_keywords(line);
        let expected_error = Error {
            error_message: "E271 multiple spaces after keyword".to_string(),
            column_number: 10
        };
        assert_eq!(error, vec![Some(expected_error)])
    }

    #[test]
    fn whitespace_around_keywords_space_before_and() {
        let line = "True  and False";
        let error =  whitespace_around_keywords(line);
        let expected_error = Error {
            error_message: "E272 multiple spaces before keyword".to_string(),
            column_number: 4
        };
        assert_eq!(error, vec![Some(expected_error)])
    }

    #[test]
    fn whitespace_around_keywords_tab_after_and() {
        let line = "True and\tFalse";
        let error =  whitespace_around_keywords(line);
        let expected_error = Error {
            error_message: "E273 tab after keyword".to_string(),
            column_number: 9
        };
        assert_eq!(error, vec![Some(expected_error)])
    }

    #[test]
    fn whitespace_around_keywords_tab_before_and() {
        let line = "True\tand False";
        let error =  whitespace_around_keywords(line);
        let expected_error = Error {
            error_message: "E274 tab before keyword".to_string(),
            column_number: 4
        };
        assert_eq!(error, vec![Some(expected_error)])
    }
}