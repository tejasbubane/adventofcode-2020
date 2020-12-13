#[cfg(test)]
mod day13_test {
    use adventofcode_2020::day13::{run_1, run_2};

    #[test]
    fn part1_works_for_sample_input() {
        assert_eq!(run_1("tests/inputs/day13_sample.txt"), 295);
    }

    #[test]
    fn part1_works_for_puzzle_input() {
        assert_eq!(run_1("tests/inputs/day13_puzzle.txt"), 203);
    }

    #[test]
    fn part2_works_for_all_sample_inputs() {
        assert_eq!(run_2("7,13,x,x,59,x,31,19"), 1068781);
        assert_eq!(run_2("17,x,13,19"), 3417);
        assert_eq!(run_2("67,7,59,61"), 754018);
        assert_eq!(run_2("67,x,7,59,61"), 779210);
        assert_eq!(run_2("67,7,x,59,61"), 1261476);
        assert_eq!(run_2("1789,37,47,1889"), 1202161486);
    }

    #[test]
    fn part2_works_for_puzzle_input() {
        assert_eq!(run_2("19,x,x,x,x,x,x,x,x,41,x,x,x,x,x,x,x,x,x,859,x,x,x,x,x,x,x,23,x,x,x,x,13,x,x,x,17,x,x,x,x,x,x,x,x,x,x,x,29,x,373,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,37"), 905694340256752);
    }
}
