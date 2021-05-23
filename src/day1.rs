use crate::utils::read_lines;
use itertools::Itertools;

const SUM: usize = 2020;

pub fn run(filename: &str, sum_count: usize) -> Option<usize> {
    match read_lines(filename) {
        Ok(lines) => {
            let entries: Vec<usize> = lines
                .map(|line| line.unwrap().parse::<usize>().unwrap())
                .collect();
            let mut combinations = entries.iter().combinations(sum_count);

            combinations
                .find(|c| sum(c) == SUM)
                .map(|combination| product(&combination))
        }
        Err(_) => None,
    }
}

fn sum(items: &[&usize]) -> usize {
    items.iter().map(|&&x| x).sum()
}

fn product(items: &[&usize]) -> usize {
    items.iter().map(|&&x| x).product()
}
