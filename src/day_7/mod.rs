
use std::fs;
use std::path::Path;

type Input = Vec<usize>;
pub fn solve_part_1() -> Result<(), ()> {
    let input = parse_from_file("./inputs/day7.txt");
    println!("Solution: {}", part_one(&input));
    Ok(())
}

pub fn solve_part_2() -> Result<(), ()> {
    let input = parse_from_file("./inputs/day7.txt");
    println!("Solution: {}", part_two(&input));
    Ok(())
}

fn parse_from_file<T: AsRef<Path>>(filename: T) -> Input {
    let input = fs::read_to_string(filename).unwrap();
    parse_from_str(&input)
}

fn parse_from_str(input: &str) -> Input {
    input.split(',').map(|n| n.parse().unwrap()).collect()
}

fn calculate_cost(position_to_check: usize, crab_positions: &[usize]) -> usize {
    crab_positions
        .iter()
        .copied()
        .map(|crab_position| crab_position.abs_diff(position_to_check))
        .sum::<usize>()
}

fn calculate_bigger_cost(position_to_check: usize, crab_positions: &[usize]) -> usize {
    crab_positions
        .iter()
        .copied()
        .map(|crab_position| {
            let diff = crab_position.abs_diff(position_to_check);
            (diff * (diff + 1)) / 2
        })
        .sum::<usize>()
}

pub fn part_one(crab_positions: &[usize]) -> usize {
    let min = crab_positions.iter().copied().min().unwrap();
    let max = crab_positions.iter().copied().max().unwrap();
    (min..=max)
        .map(|position| calculate_cost(position, &crab_positions))
        .min()
        .unwrap()
}

pub fn part_two(crab_positions: &[usize]) -> usize {
    let min = crab_positions.iter().copied().min().unwrap();
    let max = crab_positions.iter().copied().max().unwrap();
    (min..=max)
        .map(|position| calculate_bigger_cost(position, &crab_positions))
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: [usize; 10] = [16, 1, 2, 0, 4, 2, 7, 1, 2, 14];

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(&TEST_INPUT), 37);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(&TEST_INPUT), 168);
    }
}
