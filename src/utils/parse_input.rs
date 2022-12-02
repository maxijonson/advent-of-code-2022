use std::fs;

pub fn parse_input(path: &str) -> String {
    let input = fs::read_to_string(path).unwrap();
    return input;
}
