use crate::utils::read_lines;

pub fn run_1(filename: &str) -> usize {
    if let Ok(lines) = read_lines(filename) {
        let lines: Vec<String> = lines.map(|line| line.unwrap()).collect();
        let arrival_time: usize = lines[0].parse().unwrap();
        let bus_numbers: Vec<usize> = lines[1]
            .split(',')
            .filter(|&b| b != "x")
            .map(|b| b.parse::<usize>().unwrap())
            .collect();
        let (bus_no, earliest_departure) = bus_numbers
            .iter()
            .zip(departures(arrival_time, &bus_numbers))
            .min_by(|x, y| x.1.cmp(&y.1))
            .unwrap();
        let waiting_time = earliest_departure - arrival_time;
        bus_no * waiting_time
    } else {
        panic!("Error reading file {}", filename)
    }
}

fn departures(arrival_time: usize, bus_numbers: &[usize]) -> Vec<usize> {
    bus_numbers
        .iter()
        .map(|b| {
            let remainder = arrival_time % b;
            arrival_time - remainder + b
        })
        .collect()
}

pub fn run_2(input: &str) -> usize {
    let buses: Vec<(usize, usize)> = input
        .split(',')
        .enumerate()
        .map(|(i, b)| (i, b.parse::<usize>().unwrap_or(0)))
        .filter(|&(_, b)| b != 0)
        .collect();

    let mut step = 1; // for first LCM
    let mut time = 0;
    for bus in buses {
        loop {
            time += step;
            if (time + bus.0) % bus.1 == 0 {
                step *= bus.1; // assuming all prime numbers as per inputs, LCM is multiplication
                break;
            }
        }
    }
    time
}
