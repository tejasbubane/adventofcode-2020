#[cfg(test)]
mod day2_test {
    use adventofcode_2020::day2::run;

    #[test]
    fn part1_works_for_sample_input() {
        assert_eq!(run("tests/inputs/day2_sample.txt"), 2);
    }

    #[test]
    fn part1_works_for_puzzle_input() {
        assert_eq!(run("tests/inputs/day2_puzzle.txt"), 614);
    }
}
