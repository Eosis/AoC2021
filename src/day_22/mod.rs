use bitvec::prelude::*;
use hashbrown::HashSet;
use regex::Regex;
use std::fs::read_to_string;
use std::path::Path;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Action {
    On,
    Off,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Cuboid {
    x_range: (i64, i64),
    y_range: (i64, i64),
    z_range: (i64, i64),
}

impl Cuboid {
    fn new(x_range: (i64, i64), y_range: (i64, i64), z_range: (i64, i64)) -> Cuboid {
        Cuboid {
            x_range,
            y_range,
            z_range
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Instruction {
    action: Action,
    cuboid: Cuboid,
}

type Input = Vec<Instruction>;
pub fn solve_part_1() -> Result<(), ()> {
    let input = parse_from_file("./inputs/day22.txt");
    println!("Solution: {}", part_one(input));
    Ok(())
}

pub fn solve_part_2() -> Result<(), ()> {
    let input = parse_from_file("./inputs/day22.txt");
    println!("Solution: {}", part_two(input));
    Ok(())
}

fn parse_from_file<T: AsRef<Path>>(filename: T) -> Input {
    let input = read_to_string(filename).unwrap();
    parse_from_str(&input)
}

fn parse_from_str(input: &str) -> Input {
    input.lines().map(instruction_from_line).collect()
}

fn instruction_from_line(line: &str) -> Instruction {
    let regex = r"(on|off) x=(-?\d+)\.\.(-?\d+),y=(-?\d+)\.\.(-?\d+),z=(-?\d+)\.\.(-?\d+)";
    let regex = Regex::new(regex).unwrap();
    let caps = regex.captures(line).unwrap();
    Instruction {
        action: if caps.get(1).unwrap().as_str() == "on" {
            Action::On
        } else {
            Action::Off
        },
        cuboid: Cuboid {
            x_range: (
                caps.get(2).and_then(|val| val.as_str().parse().ok()).unwrap(),
                caps.get(3).and_then(|val| val.as_str().parse().ok()).unwrap(),
            ),
            y_range: (
                caps.get(4).and_then(|val| val.as_str().parse().ok()).unwrap(),
                caps.get(5).and_then(|val| val.as_str().parse().ok()).unwrap(),
            ),
            z_range: (
                caps.get(6).and_then(|val| val.as_str().parse().ok()).unwrap(),
                caps.get(7).and_then(|val| val.as_str().parse().ok()).unwrap(),
            ),
        }
    }
}

pub fn part_one(input: Input) -> usize {
    let mut result: HashSet<(i64, i64, i64)> = HashSet::new();
    let actions = input
        .iter()
        .filter(|instruction| {
            let cuboid = instruction.cuboid;
            cuboid.x_range.0 >= -50
                && cuboid.x_range.1 <= 50
                && cuboid.y_range.0 >= -50
                && cuboid.y_range.1 <= 50
                && cuboid.z_range.0 >= -50
                && cuboid.z_range.1 <= 50
        })
        .flat_map(|instruction| {
            let cuboid = instruction.cuboid;
            (cuboid.x_range.0..=cuboid.x_range.1).flat_map(move |x| {
                (cuboid.y_range.0..=cuboid.y_range.1)
                    .flat_map(move |y| (cuboid.z_range.0..=cuboid.z_range.1).map(move |z| (instruction.action, (x, y, z))))
            })
        });
    for action in actions {
        if action.0 == Action::On {
            result.insert(action.1);
        } else {
            result.remove(&action.1);
        }
    }

    result.len()
}

pub fn part_two(_input: Input) -> usize {
    unimplemented!();
}

fn count_on(cuboids: &HashSet<Cuboid>) -> usize {
    cuboids.iter()
        .map(|Cuboid { x_range: (x1, x2), y_range: (y1, y2), z_range: (z1, z2) }|
            usize::try_from(((*x2 - *x1 + 1) * (*y2 - *y1 + 1) * (*z2 - *z1 + 1))).unwrap()
        ).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = include_str!("../../test_inputs/day22.txt");
    #[test]
    fn test_part_one() {
        let input = parse_from_str(TEST_INPUT);
        assert_eq!(part_one(input), 590784)
    }

    #[test]
    fn test_part_two() {
        unimplemented!();
    }

    #[test]
    fn test_parsing_instruction_from_line() {
        let instruction = instruction_from_line("on x=-54112..-39298,y=-85059..-49293,z=-27449..7877");
        assert_eq!(
            instruction,
            Instruction {
                action: Action::On,
                cuboid: Cuboid {
                    x_range: (-54112, -39298),
                    y_range: (-85059, -49293),
                    z_range: (-27449, 7877),
                }
            }
        )
    }

    #[test]
    fn test_on_size_from_ranges() {
        let cuboids: HashSet<_> = vec![
            Cuboid::new((0, 0), (0, 0), (0, 0))
        ].into_iter().collect();
        assert_eq!(count_on(&cuboids), 1);
        let cuboids: HashSet<_> = vec![
            Cuboid::new((-1, 1), (-1, 1), (-1, 1))
        ].into_iter().collect();
        assert_eq!(count_on(&cuboids), 27);
        let cuboids: HashSet<_> = vec![
            Cuboid::new((-1, 1), (-1, 1), (-1, 1)),
            Cuboid::new((-10, -8), (-10, -8), (-10, -8)),

        ].into_iter().collect();
        assert_eq!(count_on(&cuboids), 27 * 2);
    }

    #[test]
    fn test_overlapping_reduction() {
        let base_instruction = Instruction {
            action: Action::On,
            cuboid: Cuboid {
                x_range: (-1, 1), y_range: (-1, 1), z_range: (-1, 1),
            }};
        let mut on_cuboids: HashSet<Cuboid> = vec![base_instruction.cuboid].into_iter().collect();
        let off_instruction = Instruction {
            action: Action::Off,
            cuboid: Cuboid {
                x_range: (1, 3), y_range: (1, 3), z_range: (1, 3),
            }};
        //apply_off_instruction(&mut on_cuboids, off_instruction.cuboid);
        assert_eq!(count_on(&on_cuboids), 8);
    }
}
