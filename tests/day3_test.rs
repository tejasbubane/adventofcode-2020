#[cfg(test)]
mod day3_test {
    use adventofcode_2020::day3::{slope_for, run};

    const STEPS: [(usize, usize); 5] = [(1,1), (3,1), (5,1), (7,1), (1,2)];

    #[test]
    fn part1_works_for_sample_input() {
        assert_eq!(slope_for("tests/inputs/day3_sample.txt", 3, 1), 7);
    }

    #[test]
    fn part1_works_for_puzzle_input() {
        assert_eq!(slope_for("tests/inputs/day3_puzzle.txt", 3, 1), 173);
    }

    #[test]
    fn part2_works_for_sample_input() {
        assert_eq!(run("tests/inputs/day3_sample.txt", &STEPS), 336);
    }

    #[test]
    fn part2_works_for_puzzle_input() {
        assert_eq!(run("tests/inputs/day3_puzzle.txt", &STEPS), 4385176320);
    }
}
