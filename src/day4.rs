#![allow(clippy::ptr_arg)]

use crate::utils::read_to_string;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap, HashSet};

pub fn run_1(filename: &str) -> usize {
    let filestr = read_to_string(filename).expect("Error reading file.");
    passports_with_required_fields(&filestr).iter().count()
}

pub fn run_2(filename: &str) -> usize {
    let filestr = read_to_string(filename).expect("Error reading file.");
    passports_with_required_fields(&filestr)
        .iter()
        .filter(|&p| is_valid(p))
        .count()
}

// Validations

fn is_valid(passport: &HashMap<String, String>) -> bool {
    passport.get("byr").map_or_else(|| false, valid_byr)
        && passport.get("iyr").map_or_else(|| false, valid_iyr)
        && passport.get("eyr").map_or_else(|| false, valid_eyr)
        && passport.get("hgt").map_or_else(|| false, valid_hgt)
        && passport.get("hcl").map_or_else(|| false, valid_hcl)
        && passport.get("ecl").map_or_else(|| false, valid_ecl)
        && passport.get("pid").map_or_else(|| false, valid_pid)
}

fn valid_byr(birth_year: &String) -> bool {
    match birth_year.parse::<usize>() {
        Ok(year) => year >= 1920 && year <= 2002,
        Err(_) => false,
    }
}

fn valid_iyr(issue_year: &String) -> bool {
    match issue_year.parse::<usize>() {
        Ok(year) => year >= 2010 && year <= 2020,
        Err(_) => false,
    }
}

fn valid_eyr(issue_year: &String) -> bool {
    match issue_year.parse::<usize>() {
        Ok(year) => year >= 2020 && year <= 2030,
        Err(_) => false,
    }
}

fn valid_hgt(height_str: &String) -> bool {
    let len = height_str.len();
    let height = &height_str[..len - 2];
    let unit = &height_str[(len - 2)..];

    match (height.parse::<usize>(), unit) {
        (Ok(l), "cm") => l >= 150 && l <= 193,
        (Ok(l), "in") => l >= 59 && l <= 76,
        _ => false,
    }
}

lazy_static! {
    static ref HAIR_COLOR_REGEX: Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
}

fn valid_hcl(hair_color: &String) -> bool {
    HAIR_COLOR_REGEX.is_match(hair_color)
}

const EYE_COLORS: [&str; 7] = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

fn valid_ecl(eye_color: &String) -> bool {
    EYE_COLORS.iter().any(|&c| c == eye_color)
}

lazy_static! {
    static ref PASSPORT_ID_REGEX: Regex = Regex::new(r"^[0-9]{9}$").unwrap();
}

fn valid_pid(passport_id: &String) -> bool {
    PASSPORT_ID_REGEX.is_match(passport_id)
}

// Parsing & helper functions

fn passports_with_required_fields(filestr: &str) -> Vec<HashMap<String, String>> {
    let req_fields: HashSet<String> = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
        .iter()
        .map(|p| p.to_string())
        .collect();

    passport_strings(filestr)
        .iter()
        .map(passport)
        .filter(|p| {
            let keys: HashSet<String> = p.keys().map(|p| p.to_string()).collect();
            req_fields.is_subset(&keys)
        })
        .collect()
}

fn passport_strings(filecontents: &str) -> Vec<String> {
    filecontents
        .split("\n\n")
        .map(|s| s.replace("\n", " "))
        .collect()
}

fn passport(passport_string: &String) -> HashMap<String, String> {
    let mut map = HashMap::new();
    passport_string.split(' ').for_each(|p| {
        let split: Vec<&str> = p.split(':').collect();
        if split.len() > 1 {
            map.insert(split[0].to_string(), split[1].to_string());
        }
    });
    map
}
