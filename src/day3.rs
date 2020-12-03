use crate::utils::read_lines;
use grid::*;

pub fn run(filename: &str) -> usize {
    let mut terrain = grid![];
    let mut tree_count = 0;
    let mut row = 1;
    let mut col = 3;

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
        row = row + 1;
        col = col + 3;
    }
    tree_count
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
