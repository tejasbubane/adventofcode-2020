use crate::utils::read_lines;
use std::collections::HashSet;

#[derive(Clone, Debug)]
struct Instruction {
    operation: String,
    argument: isize,
}

pub fn run_1(filename: &str) -> isize {
    match read_lines(filename) {
        Ok(lines) => {
            let instructions: Vec<Instruction> = lines.map(|line| parse(&line.unwrap())).collect();
            has_loop(&instructions).1 // assuming this always has loop as per inputs - return acc directly
        }
        Err(_) => {
            println!("Error reading file {}", filename);
            0
        }
    }
}

pub fn run_2(filename: &str) -> isize {
    let mut acc = 0;

    match read_lines(filename) {
        Ok(lines) => {
            let instructions: Vec<Instruction> = lines.map(|line| parse(&line.unwrap())).collect();

            // Iterate over all instructions and try replacing each jmp and nop to see if the code halts
            for (i, ins) in instructions.iter().enumerate() {
                if ins.operation == "acc" {
                    continue;
                }

                let new_ins = match &ins.operation[..] {
                    "jmp" => Instruction {
                        operation: "nop".to_string(),
                        argument: ins.argument,
                    },
                    "nop" => Instruction {
                        operation: "jmp".to_string(),
                        argument: ins.argument,
                    },
                    _ => {
                        panic!();
                    }
                };
                let mut new_instructions = vec![];
                new_instructions.extend_from_slice(&instructions[..i]);
                new_instructions.push(new_ins);
                new_instructions.extend_from_slice(&instructions[i + 1..]);
                if let (false, count) = has_loop(&new_instructions) {
                    acc = count;
                    break;
                }
            }
        }
        Err(_) => {
            println!("Error reading file {}", filename);
        }
    };
    acc
}

fn has_loop(instructions: &[Instruction]) -> (bool, isize) {
    let total = instructions.len() as isize;
    let mut executed_instructions: HashSet<isize> = HashSet::new();
    let mut pointer: isize = 0;
    let mut accumulator: isize = 0;

    while pointer < total && !executed_instructions.contains(&pointer) {
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
    (pointer < total, accumulator)
}

fn parse(line: &str) -> Instruction {
    let parts: Vec<&str> = line.split(' ').collect();

    Instruction {
        operation: parts[0].to_string(),
        argument: parts[1].parse().unwrap(),
    }
}
