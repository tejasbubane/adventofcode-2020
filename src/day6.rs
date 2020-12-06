use crate::utils::read_to_string;
use std::collections::HashSet;

pub fn run(filename: &str) -> Vec<usize> {
    grouped_answers(filename)
        .iter()
        .map(|ans| {
            let mut set = HashSet::new();
            ans.chars().for_each(|c| {
                set.insert(c);
            });
            set.len()
        })
        .collect()
}

fn grouped_answers(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .expect("Error reading the file.")
        .split("\n\n")
        .map(|s| s.replace("\n", ""))
        .collect()
}
