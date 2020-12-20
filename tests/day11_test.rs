#[cfg(test)]
mod day11_test {
    use adventofcode_2020::day11::run;

    #[test]
    fn part1_works_for_sample_input() {
        assert_eq!(run("tests/inputs/day11_sample.txt"), 37);
    }

    #[test]
    fn part1_works_for_puzzle_input() {
        assert_eq!(run("tests/inputs/day11_puzzle.txt"), 2152);
    }
}
