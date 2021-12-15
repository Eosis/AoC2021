use std::fs;
use std::path::Path;

pub fn parse_grid_from_file<T: AsRef<Path>>(filename: T) -> Vec<Vec<usize>> {
    let input = fs::read_to_string(filename).unwrap();
    parse_grid_from_str(&input)
}

pub fn parse_grid_from_str(input: &str) -> Vec<Vec<usize>> {
    input
        .lines()
        .map(|line| line.chars().map(|n| n.to_digit(10).unwrap() as usize).collect())
        .collect()
}
