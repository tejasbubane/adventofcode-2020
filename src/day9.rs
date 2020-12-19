use crate::utils::read_lines;
use itertools::Itertools;

pub fn run(filename: &str, preamble_length: usize) -> usize {
    if let Ok(lines) = read_lines(filename) {
        let lines: Vec<usize> = lines
            .map(|line| line.unwrap().parse::<usize>().unwrap())
            .collect();
        let mut sum_candidates: Vec<usize> = lines[0..preamble_length].to_owned();

        *lines[preamble_length..]
            .iter()
            .find(|&&number| {
                let not_xmas = !is_xmas(number, &sum_candidates);
                if !not_xmas {
                    // move the preamble window forward
                    sum_candidates.remove(0);
                    sum_candidates.push(number);
                }
                not_xmas
            })
            .unwrap_or(&0)
    } else {
        0
    }
}

// Checks if given number satisfies the XMAS property
// number must be sum of two distinct elements from given candidates
fn is_xmas(number: usize, candidates: &[usize]) -> bool {
    candidates
        .iter()
        .combinations(2)
        .any(|c| c[0] + c[1] == number)
}
