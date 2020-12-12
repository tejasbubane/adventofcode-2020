#[cfg(test)]
mod day8_test {
    use adventofcode_2020::day8::run;

    #[test]
    fn part1_works_for_sample_input() {
        assert_eq!(run("tests/inputs/day8_sample.txt"), 5);
    }

    #[test]
    fn part1_works_for_puzzle_input() {
        assert_eq!(run("tests/inputs/day8_puzzle.txt"), 1528);
    }
}
