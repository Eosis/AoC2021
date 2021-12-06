use counter::Counter;
use std::cmp::{max, min};
use std::fs;
use std::path::Path;

type Input = Vec<((usize, usize), (usize, usize))>;

pub fn solve_part_1() -> Result<(), ()> {
    let input = parse_from_file("./inputs/day5.txt");
    println!("Solution: {}", part_one(input));
    Ok(())
}

pub fn solve_part_2() -> Result<(), ()> {
    let input = parse_from_file("./inputs/day5.txt");
    println!("Solution: {}", part_two(input));
    Ok(())
}

fn parse_from_file<T: AsRef<Path>>(filename: T) -> Input {
    let input = fs::read_to_string(filename).unwrap();
    parse_from_str(&input)
}

fn parse_from_str(input: &str) -> Input {
    input
        .lines()
        .map(|line| {
            let coords = line
                .split("->")
                .map(|item| {
                    let numbers: Vec<&str> = item.trim().split(',').collect();
                    (numbers[0].parse().unwrap(), numbers[1].parse().unwrap())
                })
                .collect::<Vec<_>>();
            (coords[0], coords[1])
        })
        .collect()
}

enum Gradient {
    Positive,
    Negative,
}

fn get_diagonal_spaces((x1, y1): (usize, usize), (x2, y2): (usize, usize)) -> Box<dyn Iterator<Item = (usize, usize)>> {
    let dx = if x2 > x1 { 1 } else { -1 };
    let dy = if y2 > y1 { 1 } else { -1 };
    let gradient = if dy == dx {
        Gradient::Positive
    } else {
        Gradient::Negative
    };
    let ((start_x, start_y), (end_x, _)) = if x1 < x2 {
        ((x1, y1), (x2, y2))
    } else {
        ((x2, y2), (x1, y1))
    };

    match gradient {
        Gradient::Positive => Box::new((start_x..=end_x).enumerate().map(move |(i, x)| (x, start_y + i))),
        Gradient::Negative => Box::new((start_x..=end_x).enumerate().map(move |(i, x)| (x, start_y - i))),
    }
}

fn get_visited((x1, y1): (usize, usize), (x2, y2): (usize, usize)) -> Box<dyn Iterator<Item = (usize, usize)>> {
    match ((x1, y1), (x2, y2)) {
        ((x1, y1), (x2, y2)) if x1 == x2 => Box::new((min(y1, y2)..=max(y1, y2)).map(move |y| (x1, y))),
        ((x1, y1), (x2, y2)) if y1 == y2 => Box::new((min(x1, x2)..=max(x1, x2)).map(move |x| (x, y1))),
        ((x1, y1), (x2, y2)) => Box::new(get_diagonal_spaces((x1, y1), (x2, y2))),
    }
}

pub fn part_one(input: Input) -> usize {
    let counts: Counter<(usize, usize)> = input
        .iter()
        .filter(|((x1, y1), (x2, y2))| x1 == x2 || y1 == y2)
        .flat_map(|(start, end)| get_visited(*start, *end))
        .collect();
    counts.iter().filter(|(_, count)| **count >= 2).count()
}

pub fn part_two(input: Input) -> usize {
    let counts: Counter<(usize, usize)> = input
        .iter()
        .flat_map(|(start, end)| get_visited(*start, *end))
        .collect();
    counts.iter().filter(|(_, count)| **count >= 2).count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use hashbrown::HashSet;

    const TEST_INPUT: &str = include_str!("../../test_inputs/day5.txt");

    #[test]
    #[ignore]
    fn test_parse() {
        unimplemented!()
    }

    #[test]
    fn test_part_one() {
        let input = parse_from_str(TEST_INPUT);
        assert_eq!(part_one(input), 5)
    }

    #[test]
    fn test_part_two() {
        let input = parse_from_str(TEST_INPUT);
        assert_eq!(part_two(input), 12)
    }

    #[test]
    fn test_get_diagonal_spaces() {
        let correct: Vec<(usize, usize)> = vec![(1, 1), (2, 2)];
        let correct: HashSet<_> = correct.into_iter().collect();
        let result: HashSet<_> = get_diagonal_spaces((1, 1), (2, 2)).collect::<HashSet<_>>();
        let result2: HashSet<_> = get_diagonal_spaces((2, 2), (1, 1)).collect::<HashSet<_>>();
        assert_eq!(result, correct);
        assert_eq!(result2, correct);

        let correct: Vec<(usize, usize)> = vec![(0, 1), (1, 0)];
        let correct: HashSet<_> = correct.into_iter().collect();
        let result: HashSet<_> = get_diagonal_spaces((0, 1), (1, 0)).collect::<HashSet<_>>();
        let result2: HashSet<_> = get_diagonal_spaces((1, 0), (0, 1)).collect::<HashSet<_>>();
        assert_eq!(result, correct);
        assert_eq!(result2, correct);
    }
}
