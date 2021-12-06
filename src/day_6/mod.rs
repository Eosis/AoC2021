use counter::Counter;

use std::collections::HashMap;
use std::fs;
use std::path::Path;

type Input = Vec<usize>;

pub fn solve_part_1() -> Result<(), ()> {
    let input = parse_from_file("./inputs/day6.txt");
    println!("Solution: {}", part_one(input));
    Ok(())
}

pub fn solve_part_2() -> Result<(), ()> {
    let input = parse_from_file("./inputs/day6.txt");
    println!("Solution: {}", part_two(input));
    Ok(())
}

fn parse_from_file<T: AsRef<Path>>(filename: T) -> Input {
    let input = fs::read_to_string(filename).unwrap();
    parse_from_str(&input)
}

fn parse_from_str(input: &str) -> Input {
    input.split(',').map(|n| n.parse().unwrap()).collect()
}

pub fn part_one(input: Input) -> usize {
    count_fish_after(&input, 80)
}

pub fn part_two(input: Input) -> usize {
    count_fish_after(&input, 256)
}

fn iterate_dem_fishies(counts: &mut HashMap<usize, usize>) {
    let initial_counts = counts.clone();
    for n in 0usize..8 {
        *counts.get_mut(&n).unwrap() = *initial_counts.get(&(n + 1)).unwrap();
    }
    // the 6 and the 8 are special fishies
    *counts.get_mut(&6).unwrap() = *initial_counts.get(&0).unwrap() + *initial_counts.get(&7).unwrap();
    *counts.get_mut(&8).unwrap() = *initial_counts.get(&0).unwrap();
}

fn count_fish_after(input: &[usize], days: usize) -> usize {
    let initial_counts: Counter<usize> = input.iter().copied().collect();
    let mut counts: HashMap<usize, usize> = (0..9).map(|n| (n, *initial_counts.get(&n).unwrap_or(&0))).collect();
    for _ in 0..days {
        iterate_dem_fishies(&mut counts);
    }
    counts.values().sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    

    const TEST_INPUT: [usize; 5] = [3, 4, 3, 1, 2];

    #[test]
    #[ignore]
    fn test_parse() {
        unimplemented!()
    }

    #[test]
    fn test_part_one() {
        assert_eq!(count_fish_after(&TEST_INPUT, 1), 5);
        assert_eq!(count_fish_after(&TEST_INPUT, 2), 6);
        assert_eq!(count_fish_after(&TEST_INPUT, 3), 7);
        assert_eq!(count_fish_after(&TEST_INPUT, 15), 20);
        assert_eq!(count_fish_after(&TEST_INPUT, 18), 26);
    }
}
