use crate::utils::read_lines;

pub fn run_1(filename: &str) -> Vec<usize> {
    match read_lines(filename) {
        Ok(lines) => lines.map(|line| calc_seat_number(&line.unwrap())).collect(),
        Err(_) => {
            println!("Error reading file {}", filename);
            vec![]
        }
    }
}

pub fn run_2(filename: &str) -> usize {
    let all_seats = run_1(&filename);
    match (all_seats.iter().min(), all_seats.iter().max()) {
        (Some(&min), Some(&max)) => ((min + 1)..max)
            .find(|&s| {
                !all_seats.contains(&s)
                    && all_seats.contains(&(s - 1))
                    && all_seats.contains(&(s + 1))
            })
            .unwrap(),
        _ => 0,
    }
}

fn calc_seat_number(seat_str: &str) -> usize {
    let seat_str = seat_str
        .replace("F", "0")
        .replace("B", "1")
        .replace("L", "0")
        .replace("R", "1");
    usize::from_str_radix(&seat_str, 2).unwrap()
}
