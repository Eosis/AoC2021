use crate::helpers::parse_grid_from_file;
use hashbrown::HashSet;
use std::fs;
use std::path::Path;

type Input = Vec<Vec<usize>>;
pub fn solve_part_1() -> Result<(), ()> {
    let mut input = parse_from_file("./inputs/day11.txt");
    println!("Solution: {}", part_one(&mut input));
    Ok(())
}

pub fn solve_part_2() -> Result<(), ()> {
    let mut input = parse_from_file("./inputs/day11.txt");
    println!("Solution: {}", part_two(&mut input));
    Ok(())
}

fn parse_from_file<T: AsRef<Path>>(filename: T) -> Input {
    parse_grid_from_file(filename)
}

pub fn part_one(grid: &mut [Vec<usize>]) -> usize {
    let mut flashes = 0;
    for _step in 0..100 {
        flashes += iterate_dem_octos(grid);
        print_grid(grid);
    }
    flashes
}

fn iterate_dem_octos(grid: &mut [Vec<usize>]) -> usize {
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            grid[y][x] += 1;
        }
    }

    let mut flashed: HashSet<(usize, usize)> = HashSet::new();
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] >= 10 {
                flash_dem_neighbours(grid, (y, x), &mut flashed);
            }
        }
    }

    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] >= 10 {
                grid[y][x] = 0;
            }
        }
    }
    flashed.len()
}

fn print_grid(grid: &[Vec<usize>]) {
    for row in grid {
        for val in row {
            print!(" {:2} ", val);
        }
        println!();
    }
}

fn flash_dem_neighbours(grid: &mut [Vec<usize>], (y, x): (usize, usize), flashed: &mut HashSet<(usize, usize)>) {
    if flashed.contains(&(y, x)) {
        return;
    }
    flashed.insert((y, x));
    let neighbours = get_neighbours(grid, (y, x));
    for (other_y, other_x) in neighbours {
        grid[other_y][other_x] += 1;
        if grid[other_y][other_x] >= 10 {
            flash_dem_neighbours(grid, (other_y, other_x), flashed);
        }
    }
}

fn get_neighbours(grid: &[Vec<usize>], (y, x): (usize, usize)) -> impl Iterator<Item = (usize, usize)> {
    let (y, x) = (y as i32, x as i32);
    let max_x = grid[0].len() as i32;
    let max_y = grid.len() as i32;
    (-1..=1)
        .flat_map(move |dy| {
            (-1..=1).map(move |dx| {
                if (dx == 0 && dy == 0) || dx + x < 0 || dx + x >= max_x || dy + y < 0 || dy + y >= max_y {
                    None
                } else {
                    Some(((y + dy) as usize, (x + dx) as usize))
                }
            })
        })
        .flatten()
}

pub fn part_two(grid: &mut [Vec<usize>]) -> usize {
    let mut step = 0;
    let size = grid[0].len() * grid.len();
    loop {
        step += 1;
        let flashes_this_time = iterate_dem_octos(grid);
        if flashes_this_time == size {
            break step;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::helpers::parse_grid_from_str;
    const TEST_INPUT: &str = include_str!("../../test_inputs/day11.txt");

    #[test]
    fn test_part_one() {
        let mut input = parse_grid_from_str(TEST_INPUT);
        assert_eq!(part_one(&mut input), 1656);
    }

    #[test]
    fn test_get_neighbours() {
        let mut input = parse_grid_from_str(TEST_INPUT);
        dbg!(get_neighbours(&mut input, (1, 1)).collect::<Vec<_>>());
        assert_eq!(get_neighbours(&mut input, (2, 2)).count(), 8)
    }

    #[test]
    fn test_part_two() {
        let mut input = parse_grid_from_str(TEST_INPUT);
        assert_eq!(part_two(&mut input), 195);
    }
}
