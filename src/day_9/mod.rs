
use std::fs;
use std::path::Path;
use hashbrown::{HashMap};
use std::collections::BTreeSet;

type Input = Vec<Vec<usize>>;
pub fn solve_part_1() -> Result<(), ()> {
    let input = parse_from_file("./inputs/day9.txt");
    println!("Solution: {}", part_one(&input));
    Ok(())
}

pub fn solve_part_2() -> Result<(), ()> {
    let input = parse_from_file("./inputs/day9.txt");
    println!("Solution: {}", part_two(&input));
    Ok(())
}

fn parse_from_file<T: AsRef<Path>>(filename: T) -> Input {
    let input = fs::read_to_string(filename).unwrap();
    parse_from_str(&input)
}

fn parse_from_str(input: &str) -> Input {
    input
        .lines()
        .map(|line|
            line
                .chars()
                .map(|n| n.to_digit(10).unwrap() as usize)
                .collect()
        )
        .collect()
}

fn get_risk_value((y, x): (usize, usize), grid: &[Vec<usize>], value: usize) -> usize {
    let above = if y > 0 {
        Some(grid[y - 1 ][x])
    } else { None };
    let below = grid.get(y + 1).map(|item| grid[y + 1][x]);
    let left = if x > 0 {
        Some(grid[y][x-1])
    } else {
        None
    };
    let right = grid[y].get(x + 1).map(|item| grid[y][x + 1]);
    let low_point = [above, right, below, left]
        .into_iter()
        .filter_map(|x| x)
        .all(|neighbour| value < neighbour);
    if low_point {
        value + 1
    } else {
        0
    }
}

pub fn part_one(grid: &[Vec<usize>] )-> usize {
    let mut total = 0;
    grid.iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter().enumerate().map(move |(x, value)| {
                get_risk_value((y, x), grid, *value)
            })
        })
        .sum()
}

pub fn part_two(items: &Input) -> usize {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../../test_inputs/day9.txt");

    #[test]
    fn test_part_one() {
        let input = parse_from_str(TEST_INPUT);
        assert_eq!(part_one(&input), 15);
    }

    #[test]
    fn test_part_two() {
        unimplemented!();
        let input = parse_from_str(TEST_INPUT);
        assert_eq!(part_two(&input), 61229);
    }
}
