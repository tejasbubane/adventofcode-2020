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

#[derive(Clone, Copy, PartialEq, Eq)]
enum NeighbourType {
    Immediate,
    Visible,
}

pub fn run_1(filename: &str) -> usize {
    run(filename, NeighbourType::Immediate)
}

pub fn run_2(filename: &str) -> usize {
    run(filename, NeighbourType::Visible)
}

fn run(filename: &str, neighbour_type: NeighbourType) -> usize {
    if let Ok(mut layout) = parse_grid(filename) {
        loop {
            let new_layout = mutate(&layout, neighbour_type);
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
fn mutate(layout: &Grid<Position>, neighbour_type: NeighbourType) -> Grid<Position> {
    let mut new_layout = layout.clone();
    let (row_count, col_count) = layout.size();
    let occupy_limit = if neighbour_type == NeighbourType::Immediate {
        4
    } else {
        5
    };

    for i in 0..row_count {
        for j in 0..col_count {
            let adjacent: Vec<Position> = if neighbour_type == NeighbourType::Immediate {
                neighbours(i, j, &layout)
            } else {
                visible_neighbours(i, j, &layout)
            };

            match layout[i][j] {
                Position::Occupied => {
                    if over_occupied(&adjacent, occupy_limit) {
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

// Immediate neighbours for part-1
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

// Neighbours visible in all eight directions for part-2
fn visible_neighbours(row: usize, col: usize, layout: &Grid<Position>) -> Vec<Position> {
    let mut result: Vec<Position> = Vec::new();
    let (row_limit, col_limit) = layout.size();
    let mut r;
    let mut c;

    // 1. top left diagonal
    r = row;
    c = col;
    while r > 0 && c > 0 {
        r -= 1;
        c -= 1;
        let position = layout[r][c];
        if is_seat(position) {
            result.push(position);
            break;
        }
    }

    // 2. top up
    r = row;
    c = col;
    while r > 0 {
        r -= 1;
        let position = layout[r][c];
        if is_seat(position) {
            result.push(position);
            break;
        }
    }

    // 3. top right diagonal
    r = row;
    c = col;
    while r > 0 && c < (col_limit - 1) {
        r -= 1;
        c += 1;
        let position = layout[r][c];
        if is_seat(position) {
            result.push(position);
            break;
        }
    }

    // 4. right
    r = row;
    c = col;
    while c < (col_limit - 1) {
        c += 1;
        let position = layout[r][c];
        if is_seat(position) {
            result.push(position);
            break;
        }
    }

    // 5. right down diagonal
    r = row;
    c = col;
    while r < (row_limit - 1) && c < (col_limit - 1) {
        r += 1;
        c += 1;
        let position = layout[r][c];
        if is_seat(position) {
            result.push(position);
            break;
        }
    }

    // 6. down
    r = row;
    c = col;
    while r < (row_limit - 1) {
        r += 1;
        let position = layout[r][c];
        if is_seat(position) {
            result.push(position);
            break;
        }
    }

    // 7. down left diagonal
    r = row;
    c = col;
    while r < (row_limit - 1) && c > 0 {
        r += 1;
        c -= 1;
        let position = layout[r][c];
        if is_seat(position) {
            result.push(position);
            break;
        }
    }

    // 8. left
    r = row;
    c = col;
    while c > 0 {
        c -= 1;
        let position = layout[r][c];
        if is_seat(position) {
            result.push(position);
            break;
        }
    }

    result
}

// ---- Predicate helpers ----

fn over_occupied(list: &[Position], limit: usize) -> bool {
    list.iter().filter(|&&l| l == Position::Occupied).count() >= limit
}

fn no_occupied(list: &[Position]) -> bool {
    list.iter().all(|&l| l != Position::Occupied)
}

fn is_seat(position: Position) -> bool {
    position != Position::Floor
}

// ---- Parsing ----

fn parse_grid(filename: &str) -> Result<Grid<Position>, std::io::Error> {
    match read_lines(filename) {
        Ok(lines) => {
            let lines: Vec<String> = lines.map(|line| line.unwrap()).collect();
            let mut layout: Grid<Position> = grid![];

            // Iterate over lines and populate the Grid
            lines.iter().for_each(|line| parse_line(line, &mut layout));
            Ok(layout)
        }
        Err(e) => Err(e),
    }
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
