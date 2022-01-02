use std::collections::VecDeque;
use hashbrown::HashSet;
use regex::Regex;
use std::fs::read_to_string;
use std::ops::Index;
use std::path::Path;
use itertools::Itertools;

type Input = Vec<Vec<Option<Direction>>>;
pub fn solve_part_1() -> Result<(), ()> {
    let input = parse_from_file("./inputs/day25.txt");
    println!("Solution: {}", part_one(input));
    Ok(())
}

pub fn solve_part_2() -> Result<(), ()> {
    let input = parse_from_file("./inputs/day25.txt");
    println!("Solution: {}", part_two(input));
    Ok(())
}

fn parse_from_file<T: AsRef<Path>>(filename: T) -> Input {
    let input = read_to_string(filename).unwrap();
    parse_from_str(&input)
}

fn parse_from_str(input: &str) -> Input {
    input.lines().map(| row|
        row.chars().map(move |item|
            match item {
                '.' => None,
                '>' => Some(Direction::Right),
                'v' => Some(Direction::Down),
                _ => panic!("You wot?"),
            }
        ).collect()
    ).collect()
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Direction {
    Down,
    Right,
}

fn filter_direction_locations(seabed: &Vec<Vec<Option<Direction>>>, sought: Direction) -> Vec<(usize, usize)>{
    seabed
        .iter()
        .enumerate()
        .flat_map(move |(y, row)| row
            .iter()
            .enumerate()
            .filter(move |(_, item)| item.map(move |direction| direction == sought).unwrap_or(false))
            .map(move|(x, _)|(y, x))
        )
        .collect()
}

fn move_righties(seabed: &mut Vec<Vec<Option<Direction>>>) -> bool {
    let mut movement_occurred = false;
    let row_len = seabed[0].len();
    let original = seabed.clone();
    for (y, x) in filter_direction_locations(seabed, Direction::Right) {
        if original[y][(x + 1) % row_len].is_none() {
            seabed[y][(x + 1) % row_len] = seabed[y][x].take();
            movement_occurred = true
        }
    }
    movement_occurred
}

fn move_downers(seabed: &mut Vec<Vec<Option<Direction>>>) -> bool {
    let mut movement_occurred = false;
    let height = seabed.len();
    let original = seabed.clone();
    for (y, x) in filter_direction_locations(seabed, Direction::Down) {
        if original[(y + 1) % height][x].is_none() {
            seabed[(y + 1) % height][x] = seabed[y][x].take();
            movement_occurred = true
        }
    }
    movement_occurred
}

fn print_seabed(seabed: &[Vec<Option<Direction>>]) {
    for row in seabed {
        println!();
        for item in row {
            match item {
                Some(Direction::Down) => print!("{}", 'v'),
                Some(Direction::Right) => print!("{}", '>'),
                None => print!("{}", '.'),
            }
        }
    }
    println!();
}

pub fn part_one(mut input: Input) -> usize {
    let mut movement_occurred = true;
    let mut steps = 0;
    while movement_occurred {
        movement_occurred = false;
        movement_occurred |= move_righties(&mut input);
        movement_occurred |= move_downers(&mut input);
        steps += 1;
    }
    steps
}

pub fn part_two(input: Input) -> usize {
    unimplemented!();
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;
    use super::*;
    #[test]
    fn test_part_one() {
        const TEST_INPUT: &str = include_str!("../../test_inputs/day25.txt");
        let input = parse_from_str(TEST_INPUT);
        assert_eq!(part_one(input), 58);
    }

    #[ignore]
    #[test]
    fn test_parse() {
        const TEST_INPUT: &str = include_str!("../../test_inputs/day25.txt");
        let input = parse_from_str(TEST_INPUT);
        println!("{:#?}", input)
    }

    #[ignore]
    #[test]
    fn test_filter_direction() {
        const TEST_INPUT: &str = include_str!("../../test_inputs/day25.txt");
        let input = parse_from_str(TEST_INPUT);
        println!("{:#?}", filter_direction_locations(&input, Direction::Right));
        println!("{:#?}", filter_direction_locations(&input, Direction::Down));
    }
}
