use crate::utils::read_lines;
use grid::*;
use std::fmt;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Position {
    Empty,
    Occupied,
    Floor,
}

impl Default for Position {
    fn default() -> Self {
        Position::Empty
    }
}

impl fmt::Debug for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Position::Empty => write!(f, "L"),
            Position::Occupied => write!(f, "#"),
            Position::Floor => write!(f, "."),
        }
    }
}

pub fn run(filename: &str) -> usize {
    if let Ok(lines) = read_lines(filename) {
        let lines: Vec<String> = lines.map(|line| line.unwrap()).collect();
        let mut layout: Grid<Position> = grid![];

        // Iterate over lines and populate the Grid
        lines.iter().for_each(|line| parse_line(line, &mut layout));

        loop {
            let new_layout = mutate(&layout);
            if new_layout == layout {
                break;
            }
            layout = new_layout;
        }

        layout.iter().filter(|&&p| p == Position::Occupied).count()
    } else {
        0
    }
}

// Perform one round of mutation on the entire grid
// Returns new grid - does NOT do in-place mutation
fn mutate(layout: &Grid<Position>) -> Grid<Position> {
    let mut new_layout = layout.clone();
    let (row_count, col_count) = layout.size();

    for i in 0..row_count {
        for j in 0..col_count {
            let adjacent: Vec<Position> = neighbours(i, j, &layout);
            match layout[i][j] {
                Position::Occupied => {
                    if over_occupied(&adjacent) {
                        new_layout[i][j] = Position::Empty;
                    }
                }
                Position::Empty => {
                    if no_occupied(&adjacent) {
                        new_layout[i][j] = Position::Occupied;
                    }
                }
                _ => {}
            }
        }
    }
    new_layout
}

fn neighbours<T: Clone + Copy>(row: usize, col: usize, layout: &Grid<T>) -> Vec<T> {
    let mut result: Vec<T> = Vec::new();
    let (row_limit, col_limit) = layout.size();

    if row > 0 && col > 0 {
        result.push(layout[row - 1][col - 1]);
    }
    if row > 0 {
        result.push(layout[row - 1][col]);
    }
    if row > 0 && col < (col_limit - 1) {
        result.push(layout[row - 1][col + 1]);
    }
    if col > 0 {
        result.push(layout[row][col - 1]);
    }
    if col < (col_limit - 1) {
        result.push(layout[row][col + 1]);
    }
    if row < (row_limit - 1) && col > 0 {
        result.push(layout[row + 1][col - 1])
    }
    if row < (row_limit - 1) {
        result.push(layout[row + 1][col])
    }
    if row < (row_limit - 1) && col < (col_limit - 1) {
        result.push(layout[row + 1][col + 1]);
    }

    result
}

fn over_occupied(list: &[Position]) -> bool {
    list.iter().filter(|&&l| l == Position::Occupied).count() >= 4
}

fn no_occupied(list: &[Position]) -> bool {
    list.iter().all(|&l| l != Position::Occupied)
}

fn parse_line(line: &str, layout: &mut Grid<Position>) {
    let grid_line: Vec<Position> = line
        .chars()
        .map(|c| {
            if c == 'L' {
                Position::Empty
            } else if c == '#' {
                Position::Occupied
            } else if c == '.' {
                Position::Floor
            } else {
                panic!("Invalid char found")
            }
        })
        .collect();
    layout.push_row(grid_line);
}
