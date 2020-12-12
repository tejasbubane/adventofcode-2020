use std::fs;
use std::io::{self, BufRead, Error};
use std::path::Path;

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
// Usage:
// match read_lines(filename) {
//     Ok(lines) => lines.map(|line| line.unwrap())......
//     Err(_) => ......
// }
pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<fs::File>>>
where
    P: AsRef<Path>,
{
    let file = fs::File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn read_to_string(filename: &str) -> Result<String, Error> {
    fs::read_to_string(filename)
}
