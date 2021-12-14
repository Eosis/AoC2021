use hashbrown::HashSet;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Fold {
    Y(usize),
    X(usize),
}

type Input = (HashSet<(usize, usize)>, Vec<Fold>);
pub fn solve_part_1() -> Result<(), ()> {
    let input = parse_from_file("./inputs/day13.txt");
    println!("Solution: {}", part_one(input));
    Ok(())
}

pub fn solve_part_2() -> Result<(), ()> {
    let input = parse_from_file("./inputs/day13.txt");
    part_two(input);
    Ok(())
}

fn parse_from_file<T: AsRef<Path>>(filename: T) -> Input {
    let input = fs::read_to_string(filename).unwrap();
    parse_from_str(&input)
}

fn parse_from_str(input: &str) -> Input {
    let mut lines = input.lines();
    let points: HashSet<(usize, usize)> = lines
        .by_ref()
        .take_while(|x| !x.trim().is_empty())
        .map(|line| {
            let point = line.split(',').map(|n| n.parse().unwrap()).collect::<Vec<usize>>();
            (point[1], point[0])
        })
        .collect();
    let folds: Vec<Fold> = lines
        .map(|line| line.split(' ').last().unwrap())
        .map(|item| item.split('=').collect::<Vec<&str>>())
        .map(|item| match &item[..] {
            ["y", n] => Fold::Y(n.parse().unwrap()),
            ["x", n] => Fold::X(n.parse().unwrap()),
            _ => panic!("You wot!? {:?}", item),
        })
        .collect();
    (points, folds)
}

pub fn part_one(input: Input) -> usize {
    fold(input.0, input.1[0]).len()
}

pub fn part_two(input: Input) {
    let folds = input.1;
    let result = folds.iter().fold(input.0, |acc, new| fold(acc, *new));
    print_dem_lettas(result);
}

fn print_dem_lettas(points: HashSet<(usize, usize)>) {
    let (min_x, max_x) = (
        points.iter().map(|(_, x)| *x).min().unwrap(),
        points.iter().map(|(_, x)| *x).max().unwrap(),
    );

    let (min_y, max_y) = (
        points.iter().map(|(y, _)| *y).min().unwrap(),
        points.iter().map(|(y, _)| *y).max().unwrap(),
    );

    for y in min_y..=max_y {
        println!();
        for x in min_x..=max_x {
            if points.contains(&(y, x)) {
                print!("#");
            } else {
                print!(".");
            }
        }
    }
}

fn fold(points: HashSet<(usize, usize)>, fold: Fold) -> HashSet<(usize, usize)> {
    points
        .into_iter()
        .filter(|(y, x)| match fold {
            Fold::X(n) => *x != n,
            Fold::Y(n) => *y != n,
        })
        .map(|(y, x)| match fold {
            Fold::X(n) => {
                if x < n {
                    (y, x)
                } else {
                    (y, (2 * n) - x)
                }
            }
            Fold::Y(n) => {
                if y < n {
                    (y, x)
                } else {
                    ((2 * n) - y, x)
                }
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../../test_inputs/day13.txt");

    #[test]
    #[ignore]
    fn test_parse() {
        let _things = parse_from_str(TEST_INPUT);
    }

    #[test]
    fn test_part_one() {
        let input = parse_from_str(TEST_INPUT);
        let after_one = fold(input.0, input.1[0]);
        assert_eq!(after_one.len(), 17);
        assert_eq!(fold(after_one, input.1[1]).len(), 16);
    }

    #[test]
    #[ignore]
    fn test_part_two() {
        unimplemented!();
    }
}
