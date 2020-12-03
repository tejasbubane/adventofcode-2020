use crate::utils::read_lines;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref REGEXP: Regex =
        Regex::new(r"(?P<x>\d+)-(?P<y>\d+) (?P<letter>[a-z]): (?P<password>[a-z]+)").unwrap();
}

pub fn run_1(filename: &str) -> usize {
    match read_lines(filename) {
        Ok(lines) => lines
            .map(|line| line.unwrap())
            .filter(|line| check_count(&line))
            .count(),
        Err(_) => 0,
    }
}

pub fn run_2(filename: &str) -> usize {
    match read_lines(filename) {
        Ok(lines) => lines
            .map(|line| line.unwrap())
            .filter(|line| check_position(&line))
            .count(),
        Err(_) => 0,
    }
}

fn check_count(password_req: &str) -> bool {
    let (min, max, letter, password) = parse(password_req);
    let letter_count = password.chars().filter(|&p| p == letter).count();

    min <= letter_count && letter_count <= max
}

fn check_position(password_req: &str) -> bool {
    let (first, second, letter, password) = parse(password_req);
    let chars: Vec<char> = password.chars().collect();
    let first_char = chars[first - 1];
    let second_char = chars[second - 1];

    first_char == letter && second_char != letter || first_char != letter && second_char == letter
}

fn parse(password_req: &str) -> (usize, usize, char, &str) {
    let caps = REGEXP.captures(password_req).unwrap();
    let x: usize = caps.name("x").unwrap().as_str().parse().unwrap();
    let y: usize = caps.name("y").unwrap().as_str().parse().unwrap();
    let letter: char = caps.name("letter").unwrap().as_str().parse().unwrap();
    let password = caps.name("password").unwrap().as_str();

    (x, y, letter, &password)
}
