use std::fs;
use std::convert::{TryFrom, TryInto};
use counter::Counter;
use std::path::Path;

pub fn solve_part_1() -> Result<(), ()> {
    let input = parse_from_file("./inputs/day3.txt");
    println!("Solution: {}", part_one(input));
    Ok(())
}

pub fn solve_part_2() -> Result<(), ()> {
    let input = parse_from_file("./inputs/day3.txt");
    println!("Solution: {}", part_two(input));
    Ok(())
}

fn parse_from_file<T: AsRef<Path>>(filename: T) -> Vec<String> {
    let input = fs::read_to_string(filename).unwrap();
    parse_from_str(&input)
}

fn parse_from_str(input: &str) -> Vec<String> {
    input.lines().filter_map(|s| s.try_into().ok()).collect()
}

pub fn part_one(instructions: Vec<String>) -> usize {
    let length = instructions[0].len();
    let counts: Vec<Counter<_>> = (0..instructions[0].len())
        .map(|pos|
            instructions
                .iter()
                .map(|item| item.chars().nth(pos).unwrap())
                .collect()
        ).collect();
    let counts: Vec<(_, _)> = counts.iter().map(|count| {
        let ordered = count.most_common_ordered();
        (ordered.get(0).unwrap().0, ordered.last().unwrap().0)
    }).collect();
    let mosts_number: String = counts.iter().map(|(b, _)| b).collect();
    let leasts_number: String = counts.iter().map(|(_, b)| b).collect();
    let gamma_number = usize::from_str_radix(&mosts_number, 2).unwrap();
    let epsilon_number = usize::from_str_radix(&leasts_number, 2).unwrap();
    gamma_number * epsilon_number
}


pub fn part_two(instructions: Vec<String>) -> i32 {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = include_str!("../../test_inputs/day3.txt");


    #[test]
    fn test_part_one() {
        let input = parse_from_str(TEST_INPUT);
        assert_eq!(part_one(input), 198);
    }

    #[test]
    fn test_part_two() {
        unimplemented!()
    }

}
