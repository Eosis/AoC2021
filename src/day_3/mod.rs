use counter::Counter;

use std::convert::TryInto;
use std::fs;
use std::hash::Hash;
use std::path::Path;

pub fn solve_part_1() -> Result<(), ()> {
    let input = parse_from_file("./inputs/day3.txt");
    println!("Solution: {}", part_one(input));
    Ok(())
}

pub fn solve_part_2() -> Result<(), ()> {
    let input = parse_from_file("./inputs/day3.txt");
    println!("Solution: {}", part_two(input));
    Ok(())
}

fn parse_from_file<T: AsRef<Path>>(filename: T) -> Vec<String> {
    let input = fs::read_to_string(filename).unwrap();
    parse_from_str(&input)
}

fn parse_from_str(input: &str) -> Vec<String> {
    input.lines().filter_map(|s| s.try_into().ok()).collect()
}

fn get_count_at_position(items: &[String], pos: usize) -> Counter<char> {
    items.iter().map(|item| item.chars().nth(pos).unwrap()).collect()
}

fn get_max_and_min_from_counter<T: Copy + Eq + Hash + Ord>(counter: &Counter<T>) -> (T, T) {
    let ordered = counter.most_common_tiebreaker(|&a, &b| b.cmp(&a));
    (ordered.get(0).unwrap().0, ordered.last().unwrap().0)
}

pub fn part_one(items: Vec<String>) -> usize {
    let counts: Vec<Counter<_>> = (0..items[0].len())
        .map(|pos| get_count_at_position(&items, pos))
        .collect();
    let counts: Vec<(_, _)> = counts
        .iter()
        .map( get_max_and_min_from_counter)
        .collect();
    let mosts_number: String = counts.iter().map(|(b, _)| b).collect();
    let leasts_number: String = counts.iter().map(|(_, b)| b).collect();
    let gamma_number = usize::from_str_radix(&mosts_number, 2).unwrap();
    let epsilon_number = usize::from_str_radix(&leasts_number, 2).unwrap();
    gamma_number * epsilon_number
}

pub fn part_two(items: Vec<String>) -> usize {
    let oxygen_number = reduce_list_to_single_element(0, items.clone(), Rating::Oxygen);
    let carbon_dioxide_number = reduce_list_to_single_element(0, items, Rating::CarbonDioxide);
    oxygen_number * carbon_dioxide_number
}

pub enum Rating {
    Oxygen,
    CarbonDioxide,
}

pub fn reduce_list_to_single_element(pos: usize, items: Vec<String>, rating: Rating) -> usize {
    if items.len() == 1 {
        return usize::from_str_radix(&items[0], 2).unwrap();
    }
    let counter = get_count_at_position(&items, pos);

    let (max, min) = get_max_and_min_from_counter(&counter);

    let new_items: Vec<_> = match rating {
        Rating::Oxygen => items
            .iter()
            .cloned()
            .filter(|x| x.chars().nth(pos).unwrap() == max)
            .collect(),
        Rating::CarbonDioxide => items
            .iter()
            .cloned()
            .filter(|x| x.chars().nth(pos).unwrap() == min)
            .collect(),
    };

    reduce_list_to_single_element(pos + 1, new_items, rating)
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = include_str!("../../test_inputs/day3.txt");

    #[test]
    fn test_part_one() {
        let input = parse_from_str(TEST_INPUT);
        assert_eq!(part_one(input), 198);
    }

    #[test]
    fn test_part_two() {
        let input = parse_from_str(TEST_INPUT);
        assert_eq!(part_two(input), 230);
    }
}
