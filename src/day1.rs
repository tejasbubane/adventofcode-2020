use itertools::Itertools;

const SUM: usize = 2020;

pub fn run(entries: &[usize], sum_count: usize) -> Option<usize> {
    let mut combinations = entries.iter().combinations(sum_count);

    match combinations.find(|c| sum(&c) == SUM) {
        Some(combination) => Some(product(&combination)),
        None => None,
    }
}

fn sum(items: &Vec<&usize>) -> usize {
    items.iter().fold(0, |acc, &x| acc + x)
}

fn product(items: &Vec<&usize>) -> usize {
    items.iter().fold(1, |acc, &x| acc * x)
}
