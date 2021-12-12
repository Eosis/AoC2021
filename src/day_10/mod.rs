use std::fs;
use std::path::Path;

type Input = Vec<String>;
pub fn solve_part_1() -> Result<(), ()> {
    let input = parse_from_file("./inputs/day10.txt");
    println!("Solution: {}", part_one(&input));
    Ok(())
}

pub fn solve_part_2() -> Result<(), ()> {
    let input = parse_from_file("./inputs/day10.txt");
    println!("Solution: {}", part_two(&input));
    Ok(())
}

fn parse_from_file<T: AsRef<Path>>(filename: T) -> Input {
    let input = fs::read_to_string(filename).unwrap();
    input.lines().map(|x| x.to_owned()).collect()
}

fn matching_closer(opener: char) -> char {
    match opener {
        '[' => ']',
        '{' => '}',
        '(' => ')',
        '<' => '>',
        _ => panic!("Unknown opener! {}", opener),
    }
}

fn is_opener(c: char) -> bool {
    matches!(c, '[' | '{' | '(' | '<')
}

fn is_closer(c: char) -> bool {
    matches!(c, ']' | '}' | ')' | '>')
}

fn get_naughty_closer<T: Iterator<Item = char>>(chars: &mut T, opener: char) -> Option<char> {
    let seeking = matching_closer(opener);
    loop {
        let next = chars.next();
        if next.is_none() {
            break None;
        }
        let next = next.unwrap();
        if is_closer(next) {
            if next == seeking {
                break None;
            } else {
                break Some(next);
            }
        }
        if is_opener(next) {
            let next_chunk_parse_result = get_naughty_closer(chars, next);
            if next_chunk_parse_result.is_some() {
                break next_chunk_parse_result;
            }
        }
    }
}

fn parse_naughty_string(string: &str) -> Option<char> {
    let mut chars = string.chars().peekable();
    loop {
        let opener = chars.next()?;
        if !is_opener(opener) {
            return Some(opener);
        }
        let parsed_result = get_naughty_closer(&mut chars, opener);
        if parsed_result.is_some() {
            return parsed_result;
        }
    }
}

fn values_from_naughty_bois(c: char) -> usize {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => panic!("You wot?"),
    }
}

pub fn part_one(lines: &[String]) -> usize {
    lines
        .iter()
        .filter_map(|line| parse_naughty_string(line))
        .map(values_from_naughty_bois)
        .sum()
}

pub fn part_two(lines: &[String]) -> usize {
    let mut values: Vec<_> = lines
        .iter()
        .filter(|line| parse_naughty_string(line).is_none())
        .map(|line| get_closers(line))
        .map(|closers| closers.chars().fold(0, do_arbitrary_additional_mathematics))
        .collect();
    values.sort_unstable();
    values[values.len() / 2]
}

fn do_arbitrary_additional_mathematics(acc: usize, new: char) -> usize {
    match new {
        ')' => acc * 5 + 1,
        ']' => acc * 5 + 2,
        '}' => acc * 5 + 3,
        '>' => acc * 5 + 4,
        _ => panic!("You wot?"),
    }
}

fn get_closers(characters: &str) -> String {
    let mut result = vec![];
    for c in characters.chars() {
        if is_opener(c) {
            result.push(c);
        }
        if is_closer(c) {
            result.pop();
        }
    }
    result.into_iter().rev().map(matching_closer).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../../test_inputs/day10.txt");
    const TEST_STRING: &str = "{([(<{}[<>[]}>{[]{[(<()>";
    const TEST_STRING2: &str = "[[<[([]))<([[{}[[()]]]";
    const OPEN_STRING: &str = "[({(<(())[]>[[{[]{<()<>>";

    #[test]
    fn test_part_one() {
        let input = parse_from_file("./test_inputs/day10.txt");
        assert_eq!(part_one(&input), 26397);
    }

    #[test]
    fn test_part_two() {
        let input = parse_from_file("./test_inputs/day10.txt");
        assert_eq!(part_two(&input), 288957);
    }

    #[test]
    fn test_example() {
        let _iter = TEST_STRING.chars();
        assert_eq!(parse_naughty_string(TEST_STRING), Some('}'));
        assert_eq!(parse_naughty_string(TEST_STRING2), Some(')'));
        assert_eq!(parse_naughty_string(OPEN_STRING), None);
    }

    #[test]
    fn test_simples() {
        assert_eq!(parse_naughty_string("[]"), None);
        assert_eq!(parse_naughty_string("[)]"), Some(')'));
        assert_eq!(parse_naughty_string("{([(<{}[<>[]}>"), Some('}'));
        assert_eq!(parse_naughty_string("[<>[]}"), Some('}'));
    }

    #[test]
    fn test_getting_correct_closers() {
        assert_eq!(&get_closers("[({(<(())[]>[[{[]{<()<>>"), "}}]])})]");
        assert_eq!(&get_closers("<{([{{}}[<[[[<>{}]]]>[]]"), "])}>");
    }

    #[test]
    fn test_arbitrary_mathematics() {
        assert_eq!("}}]])})]".chars().fold(0, do_arbitrary_additional_mathematics), 288957);
    }
}
