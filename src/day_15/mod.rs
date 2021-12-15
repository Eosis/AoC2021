use crate::helpers::parse_grid_from_file;

use hashbrown::HashSet;
use itertools::Itertools;



use std::path::Path;

type Input = Vec<Vec<usize>>;
pub fn solve_part_1() -> Result<(), ()> {
    let input = parse_from_file("./inputs/day15.txt");
    println!("Solution: {}", part_one(input));
    Ok(())
}

pub fn solve_part_2() -> Result<(), ()> {
    let input = parse_from_file("./inputs/day15.txt");
    println!("Solution: {}", part_two(input));
    Ok(())
}

fn parse_from_file<T: AsRef<Path>>(filename: T) -> Input {
    parse_grid_from_file(filename)
}

fn expanded_row(row: &[usize], y: usize, number_of_rows: usize) -> Vec<usize> {
    let width = row.len();
    (0..width * 5)
        .map(|x| {
            let y_addition = y / number_of_rows;
            let x_addition = x / width;
            let mut result = row[x % width] + y_addition + x_addition;
            loop {
                if result < 10 {
                    break result;
                } else {
                    result -= 9;
                }
            }
        })
        .collect()
}

fn build_bigger_map(input: Input) -> Input {
    let number_of_rows = input.len();
    (0..number_of_rows * 5)
        .map(|y| expanded_row(&input[y % number_of_rows], y, number_of_rows))
        .collect()
}

fn get_lowest_value_currently(distances: &[Vec<usize>], visited: &HashSet<(usize, usize)>) -> (usize, usize) {
    let min = distances
        .iter()
        .enumerate()
        .flat_map(move |(y, row)| {
            row.iter()
                .enumerate()
                .map(move |(x, value)| (y, x, value))
                .filter(|(y, x, _)| !visited.contains(&(*y, *x)))
        })
        .min_by(|(_y1, _x1, dist1), (_, _, dist2)| dist1.cmp(&dist2))
        .unwrap();
    (min.0, min.1)
}

fn get_trundlable_neighbours(
    (y, x): (usize, usize),
    grid: &[Vec<usize>],
    visited: &HashSet<(usize, usize)>,
) -> Vec<(usize, usize)> {
    let above = if y > 0 {
        Some(((y - 1, x), grid[y - 1][x]))
    } else {
        None
    };
    let below = grid.get(y + 1).map(|_| ((y + 1, x), grid[y + 1][x]));
    let left = if x > 0 {
        Some(((y, x - 1), grid[y][x - 1]))
    } else {
        None
    };
    let right = grid[y].get(x + 1).map(|_| ((y, x + 1), grid[y][x + 1]));
    [above, below, left, right]
        .into_iter()
        .flatten()
        .filter(|((new_y, new_x), _value)| !visited.contains(&(*new_y, *new_x)))
        .map(|((new_y, new_x), _)| (new_y, new_x))
        .collect()
}

fn do_dijkstra_instead(input: Input) -> usize {
    let max_y = input.len() - 1;
    let max_x = input[0].len() - 1;
    println!("Maxes: {:?}", (max_y, max_x));
    let mut output = input.clone();

    for y in 0..output.len() {
        for x in 0..output[0].len() {
            output[y][x] = usize::MAX;
        }
    }

    output[0][0] = 0;

    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    while !visited.contains(&(max_y, max_x)) {
        // Do the Dijkstra!
        let (y, x) = get_lowest_value_currently(&output, &visited);
        let trundlable_neighbours = get_trundlable_neighbours((y, x), &output, &visited);
        for (y_n, x_n) in trundlable_neighbours {
            let value_from_here = input[y_n][x_n] + output[y][x];
            if value_from_here < output[y_n][x_n] {
                output[y_n][x_n] = value_from_here;
            }
        }
        visited.insert((y, x));
    }
    output[max_y][max_x]
}

pub fn part_one(input: Input) -> usize {
    do_dijkstra_instead(input)
}

pub fn part_two(input: Input) -> usize {
    let bigger_map = build_bigger_map(input);
    part_one(bigger_map)
}

fn print_map(input: Input) {
    for y in 0..input.len() {
        println!();
        for x in 0..input[y].len() {
            print!("{}", input[y][x]);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::helpers::parse_grid_from_str;

    const TEST_INPUT: &str = include_str!("../../test_inputs/day15.txt");
    #[test]
    fn test_part_one() {
        let input = parse_grid_from_str(TEST_INPUT);
        assert_eq!(part_one(input), 40)
    }

    #[test]
    fn test_part_two() {
        let input = parse_grid_from_str(TEST_INPUT);
        assert_eq!(part_two(input), 315)
    }

    #[test]
    #[ignore]
    fn test_build_bigger() {
        let input = parse_grid_from_str(TEST_INPUT);
        let result = parse_grid_from_file("./test_inputs/bigger_test_parse_day_15.txt");
        assert_eq!(build_bigger_map(input), result)
    }
}
