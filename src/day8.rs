use crate::utils::read_lines;
use std::collections::HashSet;

struct Instruction {
    operation: String,
    argument: isize,
}

pub fn run(filename: &str) -> isize {
    match read_lines(filename) {
        Ok(lines) => {
            let instructions: Vec<Instruction> = lines.map(|line| parse(&line.unwrap())).collect();
            let mut executed_instructions: HashSet<isize> = HashSet::new();
            let mut pointer: isize = 0;
            let mut accumulator: isize = 0;

            while !executed_instructions.contains(&pointer) {
                executed_instructions.insert(pointer);

                let instruction = &instructions[pointer as usize];
                match &instruction.operation[..] {
                    "acc" => {
                        accumulator += instruction.argument;
                    }
                    "jmp" => {
                        pointer += instruction.argument;
                        continue;
                    }
                    _ => {}
                };
                pointer += 1;
            }
            accumulator
        }
        Err(_) => {
            println!("Error reading file {}", filename);
            0
        }
    }
}

fn parse(line: &str) -> Instruction {
    let parts: Vec<&str> = line.split(' ').collect();

    Instruction {
        operation: parts[0].to_string(),
        argument: parts[1].parse().unwrap(),
    }
}
