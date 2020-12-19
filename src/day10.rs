use crate::utils::read_lines;
use itertools::Itertools;

pub fn run(filename: &str) -> (usize, usize) {
    if let Ok(lines) = read_lines(filename) {
        let mut joltages: Vec<usize> = lines
            .map(|line| line.unwrap().parse::<usize>().unwrap())
            .collect();
        let source_joltage = 0;
        let device_joltage = joltages.iter().max().unwrap() + 3;
        joltages.push(source_joltage);
        joltages.push(device_joltage);
        let sorted: Vec<usize> = joltages.into_iter().sorted().collect();
        let mut differences: Vec<usize> = Vec::new();

        for i in 0..(sorted.len() - 1) {
            differences.push(sorted[i + 1] - sorted[i]);
        }
        (count(&differences, 1), count(&differences, 3))
    } else {
        (0, 0)
    }
}

fn count<T: Eq>(list: &[T], el: T) -> usize {
    list.iter().filter(|&x| *x == el).count()
}
