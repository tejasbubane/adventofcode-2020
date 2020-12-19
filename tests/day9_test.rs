#[cfg(test)]
mod day9_test {
    use adventofcode_2020::day9::run;

    #[test]
    fn part1_works_for_sample_input() {
        assert_eq!(run("tests/inputs/day9_sample.txt", 5), 127);
    }

    #[test]
    fn part1_works_for_puzzle_input() {
        assert_eq!(run("tests/inputs/day9_puzzle.txt", 25), 400480901);
    }
}
