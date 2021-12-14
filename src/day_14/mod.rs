use hashbrown::HashSet;
use std::collections::{BTreeMap, VecDeque};
use std::fs;
use counter::Counter;
use itertools::Itertools;
use std::path::Path;
use cached::proc_macro::cached;
use lazy_static::lazy_static;


type Input = (String, BTreeMap<(char, char), char>);
pub fn solve_part_1() -> Result<(), ()> {
    let input = parse_from_file("./inputs/day14.txt");
    println!("Solution: {}", part_one(input));
    Ok(())
}

pub fn solve_part_2() -> Result<(), ()> {
    let input = parse_from_file("./inputs/day14.txt");
    println!("Solution: {}", part_two(input));
    Ok(())
}

fn parse_from_file<T: AsRef<Path>>(filename: T) -> Input {
    let input = fs::read_to_string(filename).unwrap();
    parse_from_str(&input)
}

fn parse_from_str(input: &str) -> Input {
    let mut lines = input.lines();
    let starting = lines.next().unwrap().to_owned();
    lines.next();
    let inbetweeners: BTreeMap<(char, char), char> = lines.map(|line| {
        let parts: Vec<&str> = line
            .split(" -> ")
            .collect();
        let outers: (char, char) = (parts[0].chars().next().unwrap(), parts[0].chars().last().unwrap());
        let inner = parts[1].chars().next().unwrap();
        (outers, inner)
    }
    ).collect();
    (starting, inbetweeners)
}



pub fn part_one((initial_value, inbetweeners): Input) -> usize {
    let initial_count: Counter<char> = initial_value.chars().collect();
    let initial_count: BTreeMap<char, usize> = initial_count.into_iter().map(|(&c, &count)| (c, count)).collect();
    let mut cache = BTreeMap::new();
    let resulting: BTreeMap<char, usize> = initial_value.chars()
        .tuple_windows()
        .map(|(left, right)| get_between_count((left, right), 10, &inbetweeners, &mut cache))
        .fold(initial_count, |acc, new| combine_dem_counts(acc, new));

    let mut resulting: Vec<(char, usize)> = resulting.into_iter().collect();
    resulting.sort_by(|a, b| a.1.cmp(&b.1));

    resulting.last().unwrap().1 - resulting.first().unwrap().1
}

pub fn part_two((initial_value, inbetweeners): Input) -> usize {
    let initial_count: Counter<char> = initial_value.chars().collect();
    let initial_count: BTreeMap<char, usize> = initial_count.into_iter().map(|(&c, &count)| (c, count)).collect();
    let mut cache = BTreeMap::new();
    let resulting: BTreeMap<char, usize> = initial_value.chars()
        .tuple_windows()
        .map(|(left, right)| get_between_count((left, right), 40, &inbetweeners, &mut cache))
        .fold(initial_count, |acc, new| combine_dem_counts(acc, new));

    let mut resulting: Vec<(char, usize)> = resulting.into_iter().collect();
    resulting.sort_by(|a, b| a.1.cmp(&b.1));

    resulting.last().unwrap().1 - resulting.first().unwrap().1
}

fn combine_dem_counts(mut lhs: BTreeMap<char, usize>, rhs: BTreeMap<char, usize>) -> BTreeMap<char, usize> {
    for (char, count) in rhs.into_iter() {
        *lhs.entry(char).or_insert(0)+= count;
    }
    lhs
}

fn get_between_count((left, right): (char, char),
                     iterations_remaining: usize,
                     inbetweeners: &BTreeMap<(char, char), char>,
                     cache: &mut BTreeMap<((char, char), usize), BTreeMap<char, usize>>
) -> BTreeMap<char, usize> {
    if iterations_remaining == 0 {
        return BTreeMap::new();
    }

    if let Some(cached_val) = cache.get(&((left, right), iterations_remaining)) {
        return cached_val.clone()
    }

    let inbetweener = *inbetweeners.get(&(left, right)).unwrap();
    let mut result = BTreeMap::new();
    result.insert(inbetweener, 1);
    let result = combine_dem_counts(result, get_between_count((left, inbetweener), iterations_remaining - 1, inbetweeners, cache));
    let result = combine_dem_counts(result, get_between_count((inbetweener, right), iterations_remaining - 1, inbetweeners, cache));
    cache.insert(((left, right), iterations_remaining), result.clone());
    result
}


#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../../test_inputs/day14.txt");

    #[test]
    #[ignore]
    fn test_combining() {
        let left: BTreeMap<char, usize> = [('a', 3), ('b', 1)].into();
        let right:  BTreeMap<char, usize> = [('b', 4), ('c', 2)].into();
        let result:  BTreeMap<char, usize> = [('a', 3), ('b', 5), ('c', 2)].into();
        assert_eq!(combine_dem_counts(left, right), result)
    }

    #[test]
    fn test_part_one() {
        let input = parse_from_str(TEST_INPUT);
        assert_eq!(part_one(input), 1588)
    }

    #[test]
    fn test_part_two() {
        unimplemented!();
    }
}
