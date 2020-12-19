#[cfg(test)]
mod day10_test {
    use adventofcode_2020::day10::{run_1, run_2};

    #[test]
    fn part1_works_for_sample_input() {
        assert_eq!(run_1("tests/inputs/day10_sample1.txt"), (7, 5));
        assert_eq!(run_1("tests/inputs/day10_sample2.txt"), (22, 10));
    }

    #[test]
    fn part1_works_for_puzzle_input() {
        let (diff_one, diff_three) = run_1("tests/inputs/day10_puzzle.txt");

        assert_eq!(diff_one * diff_three, 2450);
    }

    #[test]
    fn part2_works_for_sample_input() {
        assert_eq!(run_2("tests/inputs/day10_sample1.txt"), 8);
        assert_eq!(run_2("tests/inputs/day10_sample2.txt"), 19208);
    }

    #[test]
    fn part2_works_for_puzzle_input() {
        assert_eq!(run_2("tests/inputs/day10_puzzle.txt"), 32396521357312);
    }
}
