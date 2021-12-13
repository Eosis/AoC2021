use std::collections::{HashMap, VecDeque};
use hashbrown::HashSet;
use std::fs;
use std::path::Path;
use std::thread::current;
use std::io::{self, Write};

type Input = HashMap<String, Vec<String>>;
pub fn solve_part_1() -> Result<(), ()> {
    let mut input = parse_from_file("./inputs/day12.txt");
    println!("Solution: {}", part_one(input));
    Ok(())
}

pub fn solve_part_2() -> Result<(), ()> {
    let mut input = parse_from_file("./inputs/day12.txt");
    println!("Solution: {}", part_two(input));
    Ok(())
}

fn parse_from_file<T: AsRef<Path>>(filename: T) -> Input {
    let input = fs::read_to_string(filename).unwrap();
    parse_from_str(&input)
}

fn parse_from_str(input: &str) -> Input {
    let line_items: Vec<Vec<&str>> = input
        .lines()
        .map(|line| {
            line.split('-').collect()
        })
        .collect();
    let reversed_line_items: Vec<Vec<&str>> = input
        .lines()
        .map(|line| {
            line.split('-').rev().collect()
        })
        .collect();
    let mut cave_system: HashMap<String, Vec<String>> = HashMap::new();
    for line_item in line_items.into_iter().chain(reversed_line_items.into_iter()) {
        let start = line_item[0];
        let end = line_item[1];
        cave_system
            .entry(start.to_string())
            .or_insert(vec![])
            .push(end.to_string());
    }
    cave_system
}

pub fn part_one(map: Input) -> usize {
    let mut visited: HashSet<String>= HashSet::new();
    let result = get_paths_from("start", &map, HashSet::new());
    result
        .iter()
        .filter(|path| path.iter().last().unwrap() == "end")
        .count()
}

pub fn part_two(map: Input) -> usize {
    let result = get_longer_paths_from("start", &map, HashMap::new());
    result
        .iter()
        .filter(|path| path.iter().last().unwrap() == "end")
        .count()
}

fn can_visit(node: &str, visited: &mut HashMap<String, usize>) -> bool {
    if node == "start" {
        return  *visited.entry(node.to_string()).or_insert(0) < 1
    }

    node.chars().next().unwrap().is_uppercase()
    || *visited.entry(node.to_string()).or_insert(0) < 1
    || ( visited
            .iter()
            .filter(|(k, _v)| k.chars().next().unwrap().is_lowercase())
            .all(|(k, v)| *v <= 1)
    )
}

fn get_longer_paths_from(node: &str, map: &Input, mut visited: HashMap<String, usize>) -> Vec<VecDeque<String>> {
    if node == "end" {
        return vec![vec!["end".to_string()].into()]
    }

    if !can_visit(node, &mut visited) {
        return vec![];
    }

    let mut new_visited = visited.clone();
    *new_visited.entry(node.to_string()).or_insert(0) += 1;

    let connected = map.get(node).unwrap();
    let paths_from_here: Vec<Vec<VecDeque<String>>> = connected
        .iter()
        .map(|other| get_longer_paths_from(other, map, new_visited.clone()))
        .collect();

    paths_from_here
        .into_iter()
        .map(|mut paths| {
            for path in &mut paths {
                path.push_front(node.to_string());
            }
            paths
        })
        .flat_map(|paths| paths.into_iter())
        .collect()
}

fn get_paths_from(node: &str, map: &Input, mut visited: HashSet<String>) -> Vec<VecDeque<String>> {
    if node == "end" {
        return vec![vec!["end".to_string()].into()]
    }

    if visited.contains(node) && node.chars().next().unwrap().is_lowercase() {
        return vec![];
    }

    let mut new_visited = visited.clone();
    new_visited.insert(node.to_string());

    let connected = map.get(node).unwrap();
    let paths_from_here: Vec<Vec<VecDeque<String>>> = connected
        .iter()
        .map(|other| get_paths_from(other, map, new_visited.clone()))
        .collect();

    paths_from_here
        .into_iter()
        .map(|mut paths| {
            for path in &mut paths {
                path.push_front(node.to_string());
            }
            paths
        })
        .flat_map(|paths| paths.into_iter())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../../test_inputs/day12.txt");

    #[test]
    #[ignore]
    fn test_parse() {
        let map = parse_from_str(TEST_INPUT);
        dbg!(map);
    }

    #[test]
    fn test_part_one() {
        let map = parse_from_str(TEST_INPUT);
        assert_eq!(get_paths_from("start", &map, HashSet::new()).len(), 10);
        let larger_map = parse_from_file("./test_inputs/day12_larger.txt");
        assert_eq!(get_paths_from("start", &larger_map, HashSet::new()).len(), 226);
    }

    #[test]
    fn test_part_two() {
        let map = parse_from_str(TEST_INPUT);
        assert_eq!(get_longer_paths_from("start", &map, HashMap::new()).len(), 36);
    }
}
