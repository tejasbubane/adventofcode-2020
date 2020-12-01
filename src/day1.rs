const SUM: u32 = 2020;

pub fn run(entries: Vec<u32>) -> Option<u32> {
    for (index, entry) in entries.iter().enumerate() {
        let rest = &entries[(index + 1)..];

        match rest.iter().find(|&x| x + entry == SUM) {
            Some(&other) => return Some(entry * other),
            None => continue,
        }
    }
    None
}
