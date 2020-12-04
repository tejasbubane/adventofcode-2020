#[cfg(test)]
mod day4_test {
    use adventofcode_2020::day4::{run_1, run_2};

    #[test]
    fn part1_works_for_sample_input() {
        assert_eq!(run_1("tests/inputs/day4_sample.txt"), 2);
    }

    #[test]
    fn part1_works_for_puzzle_input() {
        assert_eq!(run_1("tests/inputs/day4_puzzle.txt"), 216);
    }

    #[test]
    fn part2_works_for_sample_input() {
        assert_eq!(run_2("tests/inputs/day4_part2_sample.txt"), 4);
    }

    #[test]
    fn part2_works_for_puzzle_input() {
        assert_eq!(run_2("tests/inputs/day4_puzzle.txt"), 150);
    }
}
