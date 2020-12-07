use crate::utils::read_to_string;
use std::collections::HashSet;

// Part 1
// Questions "anyone" answered yes
pub fn run_1(filename: &str) -> Vec<usize> {
    all_answers(filename)
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

fn all_answers(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .expect("Error reading the file.")
        .split("\n\n")
        .map(|s| s.replace("\n", ""))
        .collect()
}

// Part 2
// Questions "everyone" answered yes
pub fn run_2(filename: &str) -> Vec<usize> {
    common_answers(filename).iter().map(|x| x.len()).collect()
}

fn common_answers(filename: &str) -> Vec<HashSet<char>> {
    let input = read_to_string(filename).expect("Error reading the file.");
    let groups: Vec<&str> = input.split("\n\n").collect();

    groups
        .iter()
        .map(|group| {
            let answers: Vec<&str> = group.split('\n').collect();
            let mut set: HashSet<char> = answers[0].chars().collect();

            answers[1..].iter().for_each(|a| {
                let aset: HashSet<char> = a.chars().collect();
                if !aset.is_empty() {
                    set = set.intersection(&aset).copied().collect::<HashSet<char>>();
                }
            });
            set
        })
        .collect()
}
