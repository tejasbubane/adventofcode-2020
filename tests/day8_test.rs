#[cfg(test)]
mod day8_test {
    use adventofcode_2020::day8::{run_1, run_2};

    #[test]
    fn part1_works_for_sample_input() {
        assert_eq!(run_1("tests/inputs/day8_sample.txt"), 5);
    }

    #[test]
    fn part1_works_for_puzzle_input() {
        assert_eq!(run_1("tests/inputs/day8_puzzle.txt"), 1528);
    }

    #[test]
    fn part2_works_for_sample_input() {
        assert_eq!(run_2("tests/inputs/day8_sample.txt"), 8);
    }

    #[test]
    fn part2_works_for_puzzle_input() {
        assert_eq!(run_2("tests/inputs/day8_puzzle.txt"), 640);
    }
}
