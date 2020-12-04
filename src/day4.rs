use crate::utils::read_to_string;
use std::collections::HashSet;

pub fn run(filename: &str) -> usize {
    let contents = read_to_string(filename).expect("Error reading file.");
    let req_fields: HashSet<&str> = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
        .iter()
        .cloned()
        .collect();

    passport_strings(&contents)
        .iter()
        .map(|s| passport(s))
        .filter(|p| req_fields.is_subset(p))
        .count()
}

fn passport_strings(filecontents: &str) -> Vec<String> {
    filecontents
        .split("\n\n")
        .map(|s| s.replace("\n", " "))
        .collect()
}

fn passport(passport_string: &str) -> HashSet<&str> {
    let mut keys = HashSet::new();
    passport_string.split(' ').for_each(|p| {
        let split: Vec<&str> = p.split(':').collect();
        keys.insert(split[0]);
    });
    keys
}
