#[cfg(test)]
mod day7_test {
    use adventofcode_2020::day7::{run_1, run_2};

    #[test]
    fn part1_works_for_sample_input() {
        assert_eq!(run_1("tests/inputs/day7_sample.txt"), 4);
    }

    #[test]
    fn part1_works_for_puzzle_input() {
        assert_eq!(run_1("tests/inputs/day7_puzzle.txt"), 248);
    }

    #[test]
    fn part2_works_for_sample_input() {
        assert_eq!(run_2("tests/inputs/day7_sample.txt"), 32);
    }

    #[test]
    fn part2_works_for_second_sample_input() {
        assert_eq!(run_2("tests/inputs/day7_sample2.txt"), 126);
    }

    #[test]
    fn part2_works_for_puzzle_input() {
        assert_eq!(run_2("tests/inputs/day7_puzzle.txt"), 57281);
    }
}
