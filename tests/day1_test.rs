#[cfg(test)]
mod day1_test {
    use adventofcode_2020::day1::run;

    #[test]
    fn part1_works_for_sample_input() {
        assert_eq!(run("tests/inputs/day1_sample.txt", 2).unwrap(), 514579);
    }

    #[test]
    fn part1_works_for_puzzle_input() {
        assert_eq!(run("tests/inputs/day1_puzzle.txt", 2).unwrap(), 713184);
    }

    #[test]
    fn part2_works_for_sample_input() {
        assert_eq!(run("tests/inputs/day1_sample.txt", 3).unwrap(), 241861950);
    }

    #[test]
    fn part2_works_for_puzzle_input() {
        assert_eq!(run("tests/inputs/day1_puzzle.txt", 3).unwrap(), 261244452);
    }
}
