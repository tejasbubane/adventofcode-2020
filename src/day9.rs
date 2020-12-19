use crate::utils::read_lines;
use itertools::Itertools;

type Number = usize;
type NumberList = Vec<usize>;

// Finds first number in given list which does not satisfy the XMAS property
// Also returns parsed NumberList to be used later
pub fn run_1(filename: &str, preamble_length: usize) -> Option<(Number, NumberList)> {
    if let Ok(lines) = read_lines(filename) {
        let numbers: Vec<usize> = lines
            .map(|line| line.unwrap().parse::<usize>().unwrap())
            .collect();
        let mut sum_candidates: Vec<usize> = numbers[0..preamble_length].to_owned();

        let result = numbers[preamble_length..]
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
            .unwrap_or(&0);
        Some((*result, numbers))
    } else {
        None
    }
}

// Finds contiguous list of numbers which adds up to
// the invalid-number which does not satisfy XMAS property (result of `run_1`)
pub fn run_2(filename: &str, preamble_length: usize) -> Vec<usize> {
    let (invalid_number, numbers) = run_1(filename, preamble_length).unwrap();
    let mut set_size = 2; // atleast two numbers to sum - increase from there
    let mut result: Vec<usize> = Vec::new();

    while set_size < numbers.len() {
        match chunks(&numbers, set_size)
            .into_iter()
            .find(|&s| s.iter().sum::<usize>() == invalid_number)
        {
            Some(set) => {
                result = set.iter().copied().collect();
                break;
            }
            None => set_size += 1,
        }
    }
    result
}

// All consecutive slices of given length from the iterable `base`
fn chunks<T>(base: &[T], size: usize) -> Vec<&[T]> {
    let mut result: Vec<&[T]> = Vec::new();
    let limit = base.len() - size;
    for i in 0..limit {
        result.push(&base[i..i + size]);
    }
    result
}

// Checks if given number satisfies the XMAS property
// number must be sum of two distinct elements from given candidates
fn is_xmas(number: usize, candidates: &[usize]) -> bool {
    candidates
        .iter()
        .combinations(2)
        .any(|c| c[0] + c[1] == number)
}
