use crate::utils::read_lines;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref REGEXP: Regex =
        Regex::new(r"(?P<min>\d+)-(?P<max>\d+) (?P<letter>[a-z]): (?P<password>[a-z]+)").unwrap();
}

pub fn run(filename: &str) -> usize {
    match read_lines(filename) {
        Ok(lines) => lines
            .map(|line| line.unwrap())
            .filter(|line| matches(&line))
            .count(),
        Err(_) => 0,
    }
}

fn matches(password_req: &str) -> bool {
    let (min, max, letter, password) = parse(password_req);
    let letter_count = password.chars().filter(|&p| p == letter).count();

    min <= letter_count && letter_count <= max
}

fn parse(password_req: &str) -> (usize, usize, char, &str) {
    let caps = REGEXP.captures(password_req).unwrap();
    let min: usize = caps.name("min").unwrap().as_str().parse().unwrap();
    let max: usize = caps.name("max").unwrap().as_str().parse().unwrap();
    let letter: char = caps.name("letter").unwrap().as_str().parse().unwrap();
    let password = caps.name("password").unwrap().as_str();

    (min, max, letter, password)
}
