#[cfg(test)]
mod day3_test {
    use adventofcode_2020::day3::run;

    #[test]
    fn part1_works_for_sample_input() {
        assert_eq!(run("tests/inputs/day3_sample.txt"), 7);
    }

    #[test]
    fn part1_works_for_puzzle_input() {
        assert_eq!(run("tests/inputs/day3_puzzle.txt"), 173);
    }
}
