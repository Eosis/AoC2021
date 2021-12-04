use hashbrown::HashSet;

use std::fs;
use std::path::Path;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Grid {
    columns: Vec<HashSet<usize>>,
    rows: Vec<HashSet<usize>>,
}

impl Grid {
    pub fn row_or_column_is_subset_of(&self, to_check: &HashSet<usize>) -> bool {
        self.columns.iter().any(|column| column.is_subset(to_check))
            || self.rows.iter().any(|row| row.is_subset(to_check))
    }

    pub fn calculate_result_from_grid(&self, called: &[usize]) -> usize {
        let called_set: HashSet<_> = called.iter().copied().collect();
        let all_in_grid: HashSet<_> = self.rows.iter().flat_map(|row| row.iter().copied()).collect();
        let remaining: Vec<usize> = all_in_grid.difference(&called_set).copied().collect();
        remaining.iter().sum::<usize>() * called.last().unwrap()
    }
}

pub fn solve_part_1() -> Result<(), ()> {
    let input = parse_from_file("./inputs/day4.txt");
    println!("Solution: {}", part_one(input));
    Ok(())
}

pub fn solve_part_2() -> Result<(), ()> {
    let input = parse_from_file("./inputs/day4.txt");
    println!("Solution: {}", part_two(input));
    Ok(())
}

fn parse_from_file<T: AsRef<Path>>(filename: T) -> (Vec<usize>, Vec<Grid>) {
    let input = fs::read_to_string(filename).unwrap();
    parse_from_str(&input)
}

fn parse_grid_row(line: &str) -> HashSet<usize> {
    line.split_whitespace().map(|number| number.parse().unwrap()).collect()
}

fn parse_grid_columns(input: &[&str]) -> Vec<HashSet<usize>> {
    (0..5)
        .map(|i| {
            input
                .iter()
                .map(|line| line.split_whitespace().nth(i).unwrap().parse().unwrap())
                .collect()
        })
        .collect()
}

fn parse_from_str(input: &str) -> (Vec<usize>, Vec<Grid>) {
    let mut iter = input.lines();
    let numbers = iter
        .by_ref()
        .next()
        .unwrap()
        .split(',')
        .map(|number| number.parse().unwrap())
        .collect();

    let just_grids: Vec<&str> = iter.filter(|line| !line.is_empty()).collect();
    let grids = just_grids
        .chunks(5)
        .map(|lines| -> Grid {
            let rows = lines.iter().map(|line| parse_grid_row(*line)).collect();
            let columns = parse_grid_columns(lines);
            Grid { rows, columns }
        })
        .collect();
    (numbers, grids)
}

pub fn part_one((numbers, grids): (Vec<usize>, Vec<Grid>)) -> usize {
    let mut size = 5;
    loop {
        let as_called = &numbers[0..size];
        let as_set: HashSet<usize> = as_called.iter().copied().collect();
        let result = grids.iter().find(|grid| grid.row_or_column_is_subset_of(&as_set));
        if let Some(grid) = result {
            break grid.calculate_result_from_grid(as_called);
        }
        size += 1
    }
}

pub fn part_two((numbers, grids): (Vec<usize>, Vec<Grid>)) -> usize {
    let final_grid = reduce_to_one_grid(&numbers, 5, grids);
    // Play until the end
    part_one((numbers, vec![final_grid]))
}

fn reduce_to_one_grid(all_numbers: &[usize], position: usize, grids: Vec<Grid>) -> Grid {
    if grids.len() == 1 {
        return grids.first().unwrap().clone();
    }
    let called = &all_numbers[0..position];
    let as_set: HashSet<usize> = called.iter().copied().collect();
    let new_grids = grids
        .iter()
        .filter(|grid| !grid.row_or_column_is_subset_of(&as_set))
        .cloned()
        .collect();
    reduce_to_one_grid(all_numbers, position + 1, new_grids)
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = include_str!("../../test_inputs/day4.txt");

    #[test]
    #[ignore]
    fn test_parse() {
        let (_numbers, _grids) = parse_from_str(TEST_INPUT);
        print!("{:?}", parse_from_str(TEST_INPUT));
    }

    #[test]
    fn test_part_one() {
        let input = parse_from_str(TEST_INPUT);
        assert_eq!(part_one(input), 4512);
    }

    #[test]
    fn test_part_two() {
        let input = parse_from_str(TEST_INPUT);
        assert_eq!(part_two(input), 1924);
    }
}
