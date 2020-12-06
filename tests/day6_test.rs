#[cfg(test)]
mod day6_test {
    use adventofcode_2020::day6::run;

    #[test]
    fn part1_works_for_sample_input() {
        assert_eq!(run("tests/inputs/day6_sample.txt"), &[3, 3, 3, 1, 1]);
    }

    #[test]
    fn part1_works_for_puzzle_input() {
        assert_eq!(
            run("tests/inputs/day6_puzzle.txt").iter().sum::<usize>(),
            6782
        );
    }
}
