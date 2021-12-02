use num::Integer;
use std::fs;

fn get_vals() -> Vec<usize> {
    let input = fs::read_to_string("./inputs/day1.txt").unwrap();
    input.lines().filter_map(|s| s.parse().ok()).collect()
}

pub fn solve_part_1() -> Result<(), ()> {
    let vals = get_vals();
    println!("Solution: {}", part_one(&vals));
    Ok(())
}

pub fn solve_part_2() -> Result<(), ()> {
    let vals = get_vals();
    println!("Solution: {}", part_two(&vals));
    Ok(())
}

fn part_one(vals: &[usize]) -> usize {
    vals.windows(2)
        .filter(|window| window[1] > window[0])
        .count()
}

fn part_two(vals: &[usize]) -> usize {
    let window_vals: Vec<usize> = vals.windows(3)
        .map(|window| window.iter().sum())
        .collect();
    part_one(&window_vals)
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_VALS: [usize; 10] = [
        199,
        200,
        208,
        210,
        200,
        207,
        240,
        269,
        260,
        263
    ];

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(&TEST_VALS), 7)
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(&TEST_VALS), 5)
    }

}
