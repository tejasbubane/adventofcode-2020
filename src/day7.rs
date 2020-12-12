use crate::utils::read_lines;
use std::collections::HashMap;
use std::collections::HashSet;

type Color = String;
type Capacity = usize;
type Containers = Vec<(Color, Capacity)>;
type RuleMap = HashMap<Color, Containers>; // Allowed containers for the color with capacity for each

pub fn run(filename: &str) -> usize {
    match read_lines(filename) {
        Ok(lines) => {
            let color = "shiny gold";
            let mut rules = HashMap::new();
            lines.for_each(|line| parse_rule(&mut rules, &line.unwrap()));

            // Use Set to remove duplicate containers in recursion path
            let mut allowed_containers: HashSet<Color> = HashSet::new();
            recurse(color, &rules, &mut allowed_containers);
            allowed_containers.len() - 1 // remove shiny-gold itself
        }
        Err(_) => {
            println!("Error reading file {}", filename);
            0
        }
    }
}

fn parse_rule(rules: &mut RuleMap, line: &str) {
    let line_parts: Vec<&str> = line.split(" bags contain ").collect();
    let container = line_parts[0];

    line_parts[1].split(", ").for_each(|c| {
        let words: Vec<&str> = c.split(' ').collect();

        if let Ok(count) = words[0].parse::<usize>() {
            let color = format!("{} {}", words[1], words[2]);
            let init_vec = vec![];
            let mut containers: Containers = rules.get(&color).unwrap_or(&init_vec).clone();
            containers.push((container.to_string(), count));
            rules.insert(color, containers);
        };
    })
}

fn recurse(color: &str, rules: &RuleMap, acc: &mut HashSet<Color>) {
    acc.insert(color.to_string());

    rules
        .get(color)
        .unwrap_or(&vec![])
        .iter()
        .for_each(|c| recurse(&c.0, rules, acc));
}
