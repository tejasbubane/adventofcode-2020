#[cfg(test)]
mod day9_test {
    use adventofcode_2020::day9::{run_1, run_2};

    #[test]
    fn part1_works_for_sample_input() {
        assert_eq!(run_1("tests/inputs/day9_sample.txt", 5).unwrap().0, 127);
    }

    #[test]
    fn part1_works_for_puzzle_input() {
        assert_eq!(
            run_1("tests/inputs/day9_puzzle.txt", 25).unwrap().0,
            400480901
        );
    }

    #[test]
    fn part2_works_for_sample_input() {
        assert_eq!(
            run_2("tests/inputs/day9_sample.txt", 5),
            vec![15, 25, 47, 40]
        );
    }

    #[test]
    fn part2_works_for_puzzle_input() {
        let set = run_2("tests/inputs/day9_puzzle.txt", 25);
        let min = set.iter().min().unwrap();
        let max = set.iter().max().unwrap();

        assert_eq!(min + max, 67587168);
    }
}
