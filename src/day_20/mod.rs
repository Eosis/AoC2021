use std::collections::VecDeque;

use bitvec::prelude::*;
use std::fs::read_to_string;
use std::path::Path;

type Grid = VecDeque<VecDeque<bool>>;
type Input = (BitVec, VecDeque<VecDeque<bool>>);
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
    let mut lines = input.lines();
    let bits: BitVec = lines
        .by_ref()
        .next()
        .unwrap()
        .chars()
        .map(|c| c == '#')
        .collect();
    let grid = lines
        .skip(1)
        .map(|line| line.chars().map(|c| c == '#').collect())
        .collect();
    (bits, grid)
}

fn add_borders(grid: &mut Grid, iteration: usize, _bits: &BitVec) {
    let to_add = iteration % 2 == 1;

    for row in grid.iter_mut() {
        row.push_front(to_add);
        row.push_front(to_add);
        row.push_back(to_add);
        row.push_back(to_add);
    }

    let new_length = grid[0].len();
    let blank_row: VecDeque<_> = vec![to_add; new_length].into();
    grid.push_front(blank_row.clone());
    grid.push_front(blank_row.clone());
    grid.push_back(blank_row.clone());
    grid.push_back(blank_row);
}

fn number_from_offset((y, x): (usize, usize), grid: &Grid) -> usize {
    [
        grid[y].range(x..x + 3),
        grid[y + 1].range(x..x + 3),
        grid[y + 2].range(x..x + 3),
    ]
    .into_iter()
    .flatten()
    .map(|b| if *b { 1usize } else { 0usize })
    .reduce(|acc, b| acc << 1 | b)
    .unwrap()
}

#[allow(dead_code)]
fn print_grid(grid: &Grid) {
    for y in 0..grid.len() {
        println!();
        for x in 0..(grid[0].len()) {
            print!("{}", if grid[y][x] { '#' } else { '.' })
        }
    }
    println!();
}

fn set_border(grid: &mut Grid, iteration: usize, _bitmap: &BitVec) {
    let to_set = iteration % 2 == 0;
    for x in 0..grid[0].len() {
        grid[0][x] = to_set;
        grid.iter_mut().last().unwrap()[x] = to_set;
    }

    for row in grid {
        row[0] = to_set;
        *row.iter_mut().last().unwrap() = to_set;
    }
}
pub fn part_one((bitmap, mut grid): Input) -> usize {
    for iteration in 0..2 {
        add_borders(&mut grid, iteration, &bitmap);
        grid = iterate_image((&bitmap, grid));
        set_border(&mut grid, iteration, &bitmap);
    }
    grid.iter().flat_map(|row| row.iter()).filter(|x| **x).count()
}

fn iterate_image((bitmap, input_grid): (&BitVec, Grid)) -> Grid {
    let mut output = input_grid.clone();
    for y in 0..(input_grid.len() - 2) {
        for x in 0..(input_grid[0].len() - 2) {
            let idx = number_from_offset((y, x), &input_grid);
            let new_val_at_center = bitmap[idx];
            output[y + 1][x + 1] = new_val_at_center;
        }
    }
    output
}

pub fn part_two((bitmap, mut grid): Input) -> usize {
    for iteration in 0..50 {
        add_borders(&mut grid, iteration, &bitmap);
        grid = iterate_image((&bitmap, grid));
        set_border(&mut grid, iteration, &bitmap);
    }
    grid.iter().flat_map(|row| row.iter()).filter(|x| **x).count()
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = include_str!("../../test_inputs/day20.txt");
    #[test]
    fn test_part_one() {
        let input = parse_from_str(TEST_INPUT);
        assert_eq!(part_one(input), 35)
    }

    #[test]
    fn test_part_two() {
        unimplemented!();
        // assert_eq!(part_two(input), 3993);
    }

    #[test]
    fn test_number_from_slices() {
        let grid = vec![
            vec![true, false, false].into(),
            vec![true, false, false].into(),
            vec![true, false, false].into(),
        ]
        .into();
        assert_eq!(number_from_offset((0, 0), &grid), 292);
        let grid = vec![
            vec![false, false, true].into(),
            vec![true, false, false].into(),
            vec![true, false, false].into(),
        ]
        .into();
        assert_eq!(number_from_offset((0, 0), &grid), 100);
        let grid = vec![
            vec![false, false, false].into(),
            vec![false, false, false].into(),
            vec![false, false, false].into(),
        ]
        .into();
        assert_eq!(number_from_offset((0, 0), &grid), 0);
        let grid = vec![
            vec![true, false, false, false].into(),
            vec![false, false, true, false].into(),
            vec![false, false, true, false].into(),
        ]
        .into();
        assert_eq!(number_from_offset((0, 1), &grid), 18);
    }
}
