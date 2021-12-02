use std::fs;
use std::convert::{TryFrom, TryInto};

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub enum Instruction {
    Up(usize),
    Down(usize),
    Forward(usize),
}

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
struct Position {
    horizontal: usize,
    depth: usize
}

pub fn solve_part_1() -> Result<(), ()> {
    let input = parse_from_file("./inputs/day2.txt");
    println!("Solution: {}", part_one(input));
    Ok(())
}

pub fn solve_part_2() -> Result<(), ()> {
    // let vals = get_vals();
    // println!("Solution: {}", part_two(&vals));
    // Ok(())
    unimplemented!()
}

impl TryFrom<&str> for Instruction {
    type Error = ();
    fn try_from(line: &str) -> Result<Self, Self::Error> {
        let parts: Vec<_> = line.split_whitespace()
            .collect();
        match &parts[..] {
            &["forward", n] => Ok(Instruction::Forward(n.parse().unwrap())),
            &["down", n] => Ok(Instruction::Down(n.parse().unwrap())),
            &["up", n] => Ok(Instruction::Up(n.parse().unwrap())),
            _ => { println!("Didn't parse {:?}", parts);  Err(()) }
        }
    }
}

fn parse_from_file(filename: &str) -> Vec<Instruction> {
    let input = fs::read_to_string(filename).unwrap();
    parse_input(&input)
}

fn parse_input(input: &str) -> Vec<Instruction> {
    input.lines().filter_map(|s| s.try_into().ok()).collect()
}

pub fn part_one(instructions: Vec<Instruction>) -> usize {
    let forward: usize = instructions
        .iter()
        .filter_map( |ins| match ins {
            Instruction::Forward(n) => Some(n),
            _ => None,
        }).sum();
    let depth: i32 = instructions
        .iter()
        .filter_map( |ins| match ins {
            Instruction::Down(n) => Some(*n as i32),
            Instruction::Up(n) => Some(-(*n as i32)),
            _ => None,
        }).sum();
    forward * (depth as usize)
}


fn part_two(vals: &[usize]) -> usize {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_STRING: &'static str = "forward 5
        down 5
        forward 8
        up 3
        down 8
        forward 2";

    #[test]
    fn test_parsing() {
        assert_eq!(
            &parse_input(TEST_STRING),
            &[
                Instruction::Forward(5),
                Instruction::Down(5),
                Instruction::Forward(8),
                Instruction::Up(3),
                Instruction::Down(8),
                Instruction::Forward(2),
            ]
        )
    }


    #[test]
    fn test_part_one() {
        assert_eq!(part_one(parse_input(&TEST_STRING)), (5 + 8 + 2) * (5 - 3 + 8))
    }

    #[test]
    fn test_part_two() {
        unimplemented!()
    }

}
