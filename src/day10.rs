use crate::utils::read_lines;
use itertools::Itertools;

// part 1
pub fn run_1(filename: &str) -> (usize, usize) {
    if let Some(sorted) = joltages(filename) {
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

// reads file, parses numbers, collects them in vector
// adds 0 and device (max + 3) to the list - sorts and then returns
fn joltages(filename: &str) -> Option<Vec<usize>> {
    if let Ok(lines) = read_lines(filename) {
        let mut result: Vec<usize> = lines
            .map(|line| line.unwrap().parse::<usize>().unwrap())
            .collect();
        let source_joltage = 0;
        let device_joltage = result.iter().max().unwrap() + 3;
        result.push(source_joltage);
        result.push(device_joltage);
        let sorted: Vec<usize> = result.into_iter().sorted().collect();
        Some(sorted)
    } else {
        None
    }
}

// part 2
use std::collections::HashMap;
use std::collections::HashSet;

pub fn run_2(filename: &str) -> usize {
    if let Some(joltages) = joltages(filename) {
        let set: HashSet<usize> = joltages.iter().cloned().collect();
        let mut acc: HashMap<usize, usize> = HashMap::new();

        recurse(0, &set, &mut acc) // sorted - so first element always zero
    } else {
        0
    }
}

// count number of paths from given point onwards
fn recurse(num: usize, set: &HashSet<usize>, cache: &mut HashMap<usize, usize>) -> usize {
    if let Some(&val) = cache.get(&num) {
        val
    } else {
        let mut result = 0;

        if set.contains(&(num + 1)) {
            result += recurse(num + 1, &set, cache)
        }
        if set.contains(&(num + 2)) {
            result += recurse(num + 2, &set, cache)
        }
        if set.contains(&(num + 3)) {
            result += recurse(num + 3, &set, cache)
        }

        if result == 0 {
            result = 1
        }
        cache.insert(num, result);
        result
    }
}
