#[cfg(test)]
mod day5_test {
    use adventofcode_2020::day5::run;

    #[test]
    fn part1_works_for_sample_input() {
        assert_eq!(run("tests/inputs/day5_sample.txt"), vec![357, 567, 119, 820]);
    }

    #[test]
    fn part1_works_for_puzzle_input() {
        assert_eq!(*run("tests/inputs/day5_puzzle.txt").iter().max().unwrap(), 828);
    }
}
