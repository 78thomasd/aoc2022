use std::fs::File;
use std::io::BufRead;
use std::io::{Result, BufReader, Lines};

pub fn aoc_input_lines(path: &str) -> Result<Lines<BufReader<File>>> {
    let file = File::open(format!("./data/{}", path))?;
    Ok(BufReader::new(file).lines())
}