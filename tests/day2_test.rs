#[cfg(test)]
mod day2_test {
    use adventofcode_2020::day2::{run_1, run_2};

    #[test]
    fn part1_works_for_sample_input() {
        assert_eq!(run_1("tests/inputs/day2_sample.txt"), 2);
    }

    #[test]
    fn part1_works_for_puzzle_input() {
        assert_eq!(run_1("tests/inputs/day2_puzzle.txt"), 614);
    }

    #[test]
    fn part2_works_for_sample_input() {
        assert_eq!(run_2("tests/inputs/day2_sample.txt"), 1);
    }

    #[test]
    fn part2_works_for_puzzle_input() {
        assert_eq!(run_2("tests/inputs/day2_puzzle.txt"), 354);
    }
}
