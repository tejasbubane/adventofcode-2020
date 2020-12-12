#[cfg(test)]
mod day7_test {
    use adventofcode_2020::day7::run;

    #[test]
    fn part1_works_for_sample_input() {
        assert_eq!(run("tests/inputs/day7_sample.txt"), 4);
    }

    #[test]
    fn part1_works_for_puzzle_input() {
        assert_eq!(run("tests/inputs/day7_puzzle.txt"), 248);
    }
}
