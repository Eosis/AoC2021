use hashbrown::HashMap;
use std::collections::BTreeSet;
use std::fs;
use std::path::Path;

type Input = Vec<(Vec<String>, Vec<String>)>;
pub fn solve_part_1() -> Result<(), ()> {
    let input = parse_from_file("./inputs/day8.txt");
    println!("Solution: {}", part_one(&input));
    Ok(())
}

pub fn solve_part_2() -> Result<(), ()> {
    let input = parse_from_file("./inputs/day8.txt");
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
        .map(|line| -> (Vec<String>, Vec<String>) {
            let line: Vec<Vec<String>> = line
                .split('|')
                .map(|item| -> Vec<String> { item.trim().split_whitespace().map(|item| item.to_string()).collect() })
                .collect();
            (line[0].clone(), line[1].clone())
        })
        .collect()
}

pub fn part_one(items: &[(Vec<String>, Vec<String>)]) -> usize {
    items
        .iter()
        .map(|(_, shown)| shown)
        .flat_map(|shown| shown.iter().filter(|digit| matches!(digit.len(), 2 | 3 | 4 | 7)))
        .count()
}

pub fn part_two(items: &Input) -> usize {
    let to_set: fn(&str) -> BTreeSet<char> = |digit| digit.chars().collect();
    let as_sets: Vec<_> = items
        .into_iter()
        .map(|(all, shown)| -> (Vec<BTreeSet<char>>, Vec<BTreeSet<char>>) {
            (
                all.iter().map(|x| to_set(x)).collect(),
                shown.iter().map(|x| to_set(x)).collect(),
            )
        })
        .collect();

    as_sets
        .into_iter()
        .map(|(all, shown)| (determine_digits(&all), shown))
        .map(|(known, shown)| get_value_from_display(&known, &shown))
        .sum()
}

fn get_value_from_display(known: &HashMap<BTreeSet<char>, usize>, shown: &[BTreeSet<char>]) -> usize {
    shown.iter().fold(0, |acc, shown| acc * 10 + known.get(shown).unwrap())
}

fn determine_digits(all: &[BTreeSet<char>]) -> HashMap<BTreeSet<char>, usize> {
    let mut result = HashMap::new();
    let set_of_number_one = all.iter().find(|set| set.len() == 2).unwrap().clone();
    let set_of_number_seven = all.iter().find(|set| set.len() == 3).unwrap().clone();
    let set_of_number_four = all.iter().find(|set| set.len() == 4).unwrap().clone();
    let set_of_number_eight = all.iter().find(|set| set.len() == 7).unwrap().clone();

    result.insert(set_of_number_one, 1);
    result.insert(set_of_number_four, 4);
    result.insert(set_of_number_seven, 7);
    result.insert(set_of_number_eight, 8);
    let digits_to_sets: HashMap<usize, BTreeSet<char>> = result.iter().map(|(k, v)| (v.clone(), k.clone())).collect();
    let sets_of_cardinality_5: Vec<_> = all.iter().filter(|set| set.len() == 5).cloned().collect();
    let sets_of_cardinality_6: Vec<_> = all.iter().filter(|set| set.len() == 6).cloned().collect();
    add_cardinality_five_digits(sets_of_cardinality_5, &digits_to_sets, &mut result);
    add_cardinality_six_digits(sets_of_cardinality_6, &digits_to_sets, &mut result);
    result
}

fn get_id_tuple(set: &BTreeSet<char>, digits_to_sets: &HashMap<usize, BTreeSet<char>>) -> (usize, usize, usize) {
    (
        set.intersection(digits_to_sets.get(&1usize).unwrap()).count(),
        set.intersection(digits_to_sets.get(&4usize).unwrap()).count(),
        set.intersection(digits_to_sets.get(&7usize).unwrap()).count(),
    )
}

fn add_cardinality_five_digits(
    sets_of_cardinality_5: Vec<BTreeSet<char>>,
    digits_to_sets: &HashMap<usize, BTreeSet<char>>,
    result: &mut HashMap<BTreeSet<char>, usize>,
) {
    let cardinality_5_digits: HashMap<usize, BTreeSet<char>> = sets_of_cardinality_5
        .into_iter()
        .map(|set| (get_id_tuple(&set, digits_to_sets), set))
        .map(|(tuple, set)| -> (usize, BTreeSet<char>) {
            match tuple {
                (1, 2, 2) => (2, set),
                (2, 3, 3) => (3, set),
                (1, 3, 2) => (5, set),
                _ => {
                    panic!("Unexpected ID in cardinality 5 check: {:?}", tuple)
                }
            }
        })
        .collect();
    for (value, set) in cardinality_5_digits.into_iter() {
        result.insert(set, value);
    }
}

fn add_cardinality_six_digits(
    sets_of_cardinality_5: Vec<BTreeSet<char>>,
    digits_to_sets: &HashMap<usize, BTreeSet<char>>,
    result: &mut HashMap<BTreeSet<char>, usize>,
) {
    let cardinality_6_digits: HashMap<usize, BTreeSet<char>> = sets_of_cardinality_5
        .into_iter()
        .map(|set| (get_id_tuple(&set, digits_to_sets), set))
        .map(|(tuple, set)| -> (usize, BTreeSet<char>) {
            match tuple {
                (2, 3, 3) => (0, set),
                (1, 3, 2) => (6, set),
                (2, 4, 3) => (9, set),
                _ => panic!("Unexpected ID in cardinality 6 check"),
            }
        })
        .collect();
    for (value, set) in cardinality_6_digits.into_iter() {
        result.insert(set, value);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../../test_inputs/day8.txt");

    #[test]
    fn test_part_one() {
        let input = parse_from_str(TEST_INPUT);
        assert_eq!(part_one(&input), 26);
    }

    #[test]
    fn test_part_two() {
        let input = parse_from_str(TEST_INPUT);
        assert_eq!(part_two(&input), 61229);
    }
}
