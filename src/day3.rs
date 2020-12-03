use crate::utils::read_lines;
use grid::*;

// Part 1
pub fn slope_for(filename: &str, right_step: usize, down_step: usize) -> usize {
    let mut terrain = grid![];
    let mut tree_count = 0;
    let mut row = down_step;
    let mut col = right_step;

    match read_lines(filename) {
        Ok(lines) => lines.for_each(|line| terrain.push_row(construct_grid_line(&line.unwrap()))),
        Err(_) => println!("Error reading file {}", filename),
    };

    let row_limit = terrain.rows();
    let col_limit = terrain.cols();
    while row < row_limit {
        col = col % col_limit;
        if *terrain.get(row, col).unwrap() {
            tree_count = tree_count + 1
        }
        row = row + down_step;
        col = col + right_step;
    }
    tree_count
}

// Part 2
pub fn run(filename: &str, paths: &[(usize, usize)]) -> usize {
    paths.iter().map(|(right_step, down_step)| slope_for(filename, *right_step, *down_step)).product()
}

fn construct_grid_line(line: &str) -> Vec<bool> {
    let mut result: Vec<bool> = vec![];

    line.chars().for_each(|c| match c {
        '.' => result.push(false), // no tree
        '#' => result.push(true),
        x => panic!("Found unknown character: {}", x)
    });
    result
}
