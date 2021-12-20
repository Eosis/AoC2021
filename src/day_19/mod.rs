use std::collections::VecDeque;
use std::fmt::{Display, Formatter};
use regex::Regex;
use std::fs::read_to_string;
use std::path::Path;
use hashbrown::HashMap;
use itertools::enumerate;

type Input = HashMap<usize, Vec<(i32, i32, i32)>>;

pub fn solve_part_1() -> Result<(), ()> {
    let input = parse_from_file("./inputs/day20.txt");
    println!("Solution: {}", part_one(input));
    Ok(())
}

pub fn solve_part_2() -> Result<(), ()> {
    let input = parse_from_file("./inputs/day20.txt");
    println!("Solution: {}", part_two(input));
    Ok(())
}

fn parse_from_file<T: AsRef<Path>>(filename: T) -> Input {
    let input = read_to_string(filename).unwrap();
    parse_from_str(&input)
}

fn parse_from_str(input: &str) -> Input {
    let header = Regex::new(r"---.*scanner.*---").unwrap();

    header
        .split(input)
        .into_iter()
        .skip(1)
        .enumerate()
        .map(|(i, scanner_entry)| {
            let beacons = scanner_entry
                .lines()
                .filter(|line| !line.is_empty())
                .map(|line| -> (i32, i32, i32) {
                    let items:Vec<_> = line
                        .split(',')
                        .map(|n| n.parse::<i32>().unwrap())
                        .collect();
                    (items[0], items[1], items[2])
                })
                .collect();
            (i, beacons)
        }).collect()
}

pub fn part_one(input: Input) -> usize {
    unimplemented!();
}

pub fn part_two(input: Input) -> usize {
    unimplemented!();
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = include_str!("../../test_inputs/day19.txt");
    #[test]
    fn test_part_one() {
        let input = parse_from_str(TEST_INPUT);
        assert_eq!(part_one(input), 35)
    }

    #[test]
    fn test_part_two() {
        unimplemented!();
    }

    #[test]
    #[ignore]
    fn test_parse_report() {
       println!("{:#?}", parse_from_str(TEST_INPUT));
    }
}
