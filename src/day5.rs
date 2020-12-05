use crate::utils::read_lines;

pub fn run(filename: &str) -> Vec<usize> {
    match read_lines(filename) {
        Ok(lines) => lines.map(|line| calc_seat_number(&line.unwrap())).collect(),
        Err(_) => {
            println!("Error reading file {}", filename);
            vec![]
        }
    }
}

fn calc_seat_number(seat_str: &str) -> usize {
    let seat_str = seat_str.replace("F", "0").replace("B", "1").replace("L", "0").replace("R", "1");
    usize::from_str_radix(&seat_str, 2).unwrap()
}
