#[cfg(test)]
mod day12_test {
    use adventofcode_2020::day12::run;

    #[test]
    fn part1_works_for_sample_input() {
        assert_eq!(run("tests/inputs/day12_sample.txt"), 25);
    }

    #[test]
    fn part1_works_for_puzzle_input() {
        assert_eq!(run("tests/inputs/day12_puzzle.txt"), 590);
    }
}
