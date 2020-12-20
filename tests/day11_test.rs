#[cfg(test)]
mod day11_test {
    use adventofcode_2020::day11::{run_1, run_2};

    #[test]
    fn part1_works_for_sample_input() {
        assert_eq!(run_1("tests/inputs/day11_sample.txt"), 37);
    }

    #[test]
    fn part1_works_for_puzzle_input() {
        assert_eq!(run_1("tests/inputs/day11_puzzle.txt"), 2152);
    }

    #[test]
    fn part2_works_for_sample_input() {
        assert_eq!(run_2("tests/inputs/day11_sample.txt"), 26);
    }

    #[test]
    fn part2_works_for_puzzle_input() {
        assert_eq!(run_2("tests/inputs/day11_puzzle.txt"), 1937);
    }
}
