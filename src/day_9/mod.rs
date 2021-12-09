use hashbrown::HashSet;
use std::fs;
use std::path::Path;

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
        .map(|line| line.chars().map(|n| n.to_digit(10).unwrap() as usize).collect())
        .collect()
}

fn get_risk_value((y, x): (usize, usize), grid: &[Vec<usize>]) -> usize {
    let value = grid[y][x];
    let above = if y > 0 { Some(grid[y - 1][x]) } else { None };
    let below = grid.get(y + 1).map(|_| grid[y + 1][x]);
    let left = if x > 0 { Some(grid[y][x - 1]) } else { None };
    let right = grid[y].get(x + 1).map(|_| grid[y][x + 1]);
    let low_point = [above, right, below, left]
        .into_iter()
        .flatten()
        .all(|neighbour| value < neighbour);
    if low_point {
        value + 1
    } else {
        0
    }
}

pub fn part_one(grid: &[Vec<usize>]) -> usize {
    (0usize..grid.len())
        .flat_map(|y| (0usize..grid[y].len()).map(move |x| get_risk_value((y, x), grid)))
        .sum()
}

fn get_basin_sizes(grid: &[Vec<usize>]) -> Vec<usize> {
    let low_points = (0usize..grid.len())
        .flat_map(|y| (0usize..grid[y].len()).map(move |x| (y, x)))
        .filter(|(y, x)| get_risk_value((*y, *x), grid) > 0);
    low_points
        .map(|(y, x)| trundle_and_count((y, x), grid, &mut HashSet::new()))
        .collect()
}

fn trundle_and_count((y, x): (usize, usize), grid: &[Vec<usize>], visited: &mut HashSet<(usize, usize)>) -> usize {
    // Account for a previous trundling!
    if visited.contains(&(y, x)) {
        return 0;
    }

    visited.insert((y, x));
    1 + get_trundlable_neighbours((y, x), grid, visited)
        .into_iter()
        .map(|(y, x)| trundle_and_count((y, x), grid, visited))
        .sum::<usize>()
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
        .filter(|((new_y, new_x), value)| *value >= grid[y][x] && !visited.contains(&(*new_y, *new_x)) && *value != 9)
        .map(|((new_y, new_x), _)| (new_y, new_x))
        .collect()
}

pub fn part_two(grid: &[Vec<usize>]) -> usize {
    let mut basin_sizes = get_basin_sizes(grid);
    basin_sizes.sort_unstable();
    basin_sizes.into_iter().rev().take(3).product::<usize>()
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
        let input = parse_from_str(TEST_INPUT);
        assert_eq!(part_two(&input), 1134);
    }
}
