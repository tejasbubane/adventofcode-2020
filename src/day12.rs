use crate::utils::read_lines;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum Direction {
    North,
    South,
    East,
    West,
    Left,
    Right,
    Forward,
}

type Instruction = (Direction, usize);

pub fn run(filename: &str) -> usize {
    let instructions = parse(filename);
    let mut facing = Direction::East;
    let mut north_south_direction = Direction::North;
    let mut north_south_distance = 0;
    let mut east_west_direction = Direction::East;
    let mut east_west_distance = 0;

    instructions
        .iter()
        .for_each(|instruction| match instruction {
            (Direction::North, _) | (Direction::South, _) => navigate(
                *instruction,
                &mut north_south_direction,
                &mut north_south_distance,
            ),
            (Direction::East, _) | (Direction::West, _) => navigate(
                *instruction,
                &mut east_west_direction,
                &mut east_west_distance,
            ),
            (Direction::Left, degrees) | (Direction::Right, degrees) => {
                facing = turn(facing, instruction.0, *degrees)
            }
            (Direction::Forward, v) => match facing {
                Direction::North | Direction::South => navigate(
                    (facing, *v),
                    &mut north_south_direction,
                    &mut north_south_distance,
                ),
                Direction::East | Direction::West => navigate(
                    (facing, *v),
                    &mut east_west_direction,
                    &mut east_west_distance,
                ),
                _ => {
                    panic!("Invalid facing: {:?}", facing)
                }
            },
        });

    north_south_distance + east_west_distance
}

fn navigate(instruction: Instruction, ship_dir: &mut Direction, ship_dist: &mut usize) {
    let (nav_dir, nav_val) = instruction;
    if nav_dir == *ship_dir {
        // same direction - just increment value
        *ship_dist += nav_val;
    } else {
        // different direction - decide to change direction based on values
        if nav_val > *ship_dist {
            // change direction
            *ship_dir = nav_dir;
            *ship_dist = nav_val - *ship_dist;
        } else {
            *ship_dist -= nav_val;
        }
    }
}

// new direction after turning
fn turn(direction: Direction, turn: Direction, degrees: usize) -> Direction {
    match (direction, turn, degrees) {
        (Direction::North, _, 180) => Direction::South,
        (Direction::South, _, 180) => Direction::North,
        (Direction::East, _, 180) => Direction::West,
        (Direction::West, _, 180) => Direction::East,

        (Direction::North, Direction::Left, 90) => Direction::West,
        (Direction::North, Direction::Left, 270) => Direction::East,
        (Direction::North, Direction::Right, 90) => Direction::East,
        (Direction::North, Direction::Right, 270) => Direction::West,

        (Direction::East, Direction::Left, 90) => Direction::North,
        (Direction::East, Direction::Left, 270) => Direction::South,
        (Direction::East, Direction::Right, 90) => Direction::South,
        (Direction::East, Direction::Right, 270) => Direction::North,

        (Direction::South, Direction::Left, 90) => Direction::East,
        (Direction::South, Direction::Left, 270) => Direction::West,
        (Direction::South, Direction::Right, 90) => Direction::West,
        (Direction::South, Direction::Right, 270) => Direction::East,

        (Direction::West, Direction::Left, 90) => Direction::South,
        (Direction::West, Direction::Left, 270) => Direction::North,
        (Direction::West, Direction::Right, 90) => Direction::North,
        (Direction::West, Direction::Right, 270) => Direction::South,

        _ => direction, // no change
    }
}

fn parse(filename: &str) -> Vec<Instruction> {
    if let Ok(lines) = read_lines(filename) {
        lines
            .map(|line| {
                let line = line.unwrap();
                (
                    parse_direction(&line[0..1]),
                    line[1..].parse::<usize>().unwrap(),
                )
            })
            .collect()
    } else {
        panic!("Cannot read file: {}", filename);
    }
}

fn parse_direction(s: &str) -> Direction {
    if s == "N" {
        Direction::North
    } else if s == "S" {
        Direction::South
    } else if s == "E" {
        Direction::East
    } else if s == "W" {
        Direction::West
    } else if s == "R" {
        Direction::Right
    } else if s == "L" {
        Direction::Left
    } else if s == "F" {
        Direction::Forward
    } else {
        panic!("Invalid direction found: {}", s)
    }
}
