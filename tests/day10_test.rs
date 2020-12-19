#[cfg(test)]
mod day10_test {
    use adventofcode_2020::day10::run;

    #[test]
    fn part1_works_for_sample_input() {
        assert_eq!(run("tests/inputs/day10_sample1.txt"), (7, 5));
        assert_eq!(run("tests/inputs/day10_sample2.txt"), (22, 10));
    }

    #[test]
    fn part1_works_for_puzzle_input() {
        let (diff_one, diff_three) = run("tests/inputs/day10_puzzle.txt");

        assert_eq!(diff_one * diff_three, 2450);
    }
}
