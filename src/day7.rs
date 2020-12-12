use crate::utils::read_lines;
use std::collections::HashMap;
use std::collections::HashSet;

type Color = String;
type Capacity = usize;
type Containers = Vec<(Color, Capacity)>;
type RuleMap = HashMap<Color, Containers>; // Allowed containers for the color with capacity for each

// Part 1

pub fn run_1(filename: &str) -> usize {
    match read_lines(filename) {
        Ok(lines) => {
            let color = "shiny gold";
            let mut rules = HashMap::new();
            lines.for_each(|line| parse_rule(&mut rules, &line.unwrap(), true));

            // Use Set to remove duplicate containers in recursion path
            let mut allowed_containers: HashSet<Color> = HashSet::new();
            recurse_1(color, &rules, &mut allowed_containers);
            allowed_containers.len() - 1 // remove shiny-gold itself
        }
        Err(_) => {
            println!("Error reading file {}", filename);
            0
        }
    }
}

// `inverse`  argument is used to create inverse map of containers.
// For example: "dark orange bags contain 3 bright white bags, 4 muted yellow bags."
// inverse = false : { "dark_orange" => [("bright white", 3), ("muted yellow", 4)] }
// inverse = true : { "bright white" => [("dark orange", 3)], "muted yellow" => ["muted yellow", 4] }
fn parse_rule(rules: &mut RuleMap, line: &str, inverse: bool) {
    let line_parts: Vec<&str> = line.split(" bags contain ").collect();
    let container = line_parts[0];

    line_parts[1].split(", ").for_each(|c| {
        let words: Vec<&str> = c.split(' ').collect();

        if let Ok(count) = words[0].parse::<usize>() {
            let color = format!("{} {}", words[1], words[2]);
            let init_vec = vec![];
            if inverse {
                let mut inners: Containers = rules.get(&color).unwrap_or(&init_vec).clone();
                inners.push((container.to_string(), count));
                rules.insert(color, inners);
            } else {
                let mut inners: Containers = rules.get(container).unwrap_or(&init_vec).clone();
                inners.push((color, count));
                rules.insert(container.to_string(), inners);
            }
        };
    })
}

fn recurse_1(color: &str, rules: &RuleMap, acc: &mut HashSet<Color>) {
    acc.insert(color.to_string());

    rules
        .get(color)
        .unwrap_or(&vec![])
        .iter()
        .for_each(|c| recurse_1(&c.0, rules, acc));
}

// Part 2

pub fn run_2(filename: &str) -> usize {
    match read_lines(filename) {
        Ok(lines) => {
            let color = "shiny gold";
            let mut rules = HashMap::new();
            lines.for_each(|line| parse_rule(&mut rules, &line.unwrap(), false));
            recurse_2(color, &rules)
        }
        Err(_) => {
            println!("Error reading file {}", filename);
            0
        }
    }
}

fn recurse_2(color: &str, rules: &RuleMap) -> usize {
    rules
        .get(color)
        .unwrap_or(&vec![])
        .iter()
        .map(|c| c.1 + c.1 * recurse_2(&c.0, rules))
        .sum()
}
