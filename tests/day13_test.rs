#[cfg(test)]
mod day13_test {
    use adventofcode_2020::day13::run;

    #[test]
    fn part1_works_for_sample_input() {
        assert_eq!(run("tests/inputs/day13_sample.txt"), 295);
    }

    #[test]
    fn part1_works_for_puzzle_input() {
        assert_eq!(run("tests/inputs/day13_puzzle.txt"), 203);
    }
}
