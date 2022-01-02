use std::collections::VecDeque;
use hashbrown::HashSet;
use regex::Regex;
use std::fs::read_to_string;
use std::ops::Index;
use std::path::Path;
use itertools::Itertools;

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

#[allow(dead_code)]
impl Cuboid {
    fn new(x_range: (i64, i64), y_range: (i64, i64), z_range: (i64, i64)) -> Cuboid {
        Cuboid {
            x_range,
            y_range,
            z_range,
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
        },
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
                (cuboid.y_range.0..=cuboid.y_range.1).flat_map(move |y| {
                    (cuboid.z_range.0..=cuboid.z_range.1).map(move |z| (instruction.action, (x, y, z)))
                })
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

fn efficient_part_one(input: Input) -> usize {
    let instructions = input
        .iter()
        .filter(|instruction| {
            let cuboid = instruction.cuboid;
            cuboid.x_range.0 >= -50
                && cuboid.x_range.1 <= 50
                && cuboid.y_range.0 >= -50
                && cuboid.y_range.1 <= 50
                && cuboid.z_range.0 >= -50
                && cuboid.z_range.1 <= 50
        });
    let mut cuboids: HashSet<Cuboid> = HashSet::new();
    for instruction in instructions {
        match instruction {
            Instruction { action: Action::On, cuboid} => {
                add_and_remove_overlapping(&mut cuboids, *cuboid);
            },
            Instruction { action: Action::Off, cuboid } => {
                turn_things_off(&mut cuboids, *cuboid);
            }
        }
    }
    count_on(&cuboids)
}

pub fn part_two(input: Input) -> usize {
    let mut cuboids: HashSet<Cuboid> = HashSet::new();
    for (i, instruction) in input.into_iter().enumerate() {
        println!("{}, {:?}", i, instruction);
        println!("The set of cuboids is now of size {}", cuboids.len());
        match instruction {
            Instruction { action: Action::On, cuboid} => {
                add_and_remove_overlapping(&mut cuboids, cuboid);
            },
            Instruction { action: Action::Off, cuboid } => {
                turn_things_off(&mut cuboids, cuboid);
            }
        }
    }
    count_on(&cuboids)
}

fn count_on(cuboids: &HashSet<Cuboid>) -> usize {
    cuboids
        .iter()
        .map(
            |Cuboid {
                 x_range: (x1, x2),
                 y_range: (y1, y2),
                 z_range: (z1, z2),
             }| usize::try_from((*x2 - *x1 + 1) * (*y2 - *y1 + 1) * (*z2 - *z1 + 1)).unwrap(),
        )
        .sum()
}

fn ranges_overlap((min, max): (i64, i64), (min1, max1): (i64, i64)) -> bool {
  !( min1 < min && max1 < min || min1 > max )
}

fn check_cubes_overlap(lhs: Cuboid, rhs: Cuboid) -> bool {
    ranges_overlap(lhs.x_range, rhs.x_range)
    && ranges_overlap(lhs.y_range, rhs.y_range)
    && ranges_overlap(lhs.z_range, rhs.z_range)
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct Overlap {
    x: OverlapRanges,
    y: OverlapRanges,
    z: OverlapRanges,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct OverlapRanges {
    minus: Option<(i64, i64)>,
    overlap: Option<(i64, i64)>,
    plus: Option<(i64, i64)>,
}

impl OverlapRanges {
    fn just_overlap(&self) -> Option<(i64, i64)> {
        self.overlap
    }

    fn just_new(&self) -> impl Iterator<Item=(i64, i64)>{
        vec![self.minus.clone(), self.plus.clone()]
            .into_iter()
            .flatten()
    }

    fn all(&self) -> impl Iterator<Item=(i64, i64)> {
        vec![self.minus.clone(), self.overlap.clone(), self.plus.clone()]
            .into_iter()
            .flatten()
    }
}


impl Index<usize> for OverlapRanges {
    type Output = Option<(i64, i64)>;

    fn index(&self, idx: usize) -> &Self::Output {
        match idx {
            0 => &self.minus,
            1 => &self.overlap,
            2 => &self.plus,
            _ => panic!("You wot?"),
        }
    }
}

fn get_new_cuboids_ranges(overlap: Overlap) -> Vec<Cuboid> {
    let mut ranges = vec![];
    for x in 0..3 {
        for y in 0..3 {
            for z in 0..3 {
                if !(x == 1 && y == 1 && z == 1) {
                    match (overlap.x[x], overlap.y[y], overlap.z[z]) {
                        (Some(x_range), Some(y_range), Some(z_range)) => {
                            ranges.push(
                                Cuboid {
                                    x_range,
                                    y_range,
                                    z_range
                                }
                            )
                        }
                        _ => (),
                    }
                }
            }
        }
    }
    ranges
}

fn get_overlap_of_ranges(existing: (i64, i64), new: (i64, i64)) -> OverlapRanges {
    if !ranges_overlap(existing, new) {
        return OverlapRanges {
            minus: None,
            overlap: None,
            plus: None,
        }
    }

    let minus = if new.0 < existing.0 {
        Some((new.0, existing.0 - 1))
    } else {
        None
    };

    let overlap = {
        let lower_overlap = if new.0 > existing.0 {
            new.0
        } else {
            existing.0
        };
        let upper_overlap = if new.1 < existing.1 {
            new.1
        } else {
            existing.1
        };
        Some((lower_overlap, upper_overlap))
    };

    let plus = if new.1 > existing.1 {
        Some((existing.1 + 1, new.1))
    } else {
        None
    };

    OverlapRanges {
        minus,
        overlap,
        plus,
    }
}

fn add_and_remove_overlapping(cuboids: &mut HashSet<Cuboid>, to_add: Cuboid) {
    let mut problem_cuboids: HashSet<_>  = cuboids.iter().copied().filter(|cuboid| check_cubes_overlap(*cuboid, to_add)).collect();
    for cuboid in &problem_cuboids {
        cuboids.remove(cuboid);
    }

    problem_cuboids.insert(to_add);

    println!("There are {} problem cuboids", problem_cuboids.len());

    while problem_cuboids.iter().combinations(2).any(|items| check_cubes_overlap(*items[0], *items[1])) {
        let to_fix = problem_cuboids
            .iter()
            .cloned()
            .combinations(2)
            .filter(|items| check_cubes_overlap(items[0], items[1]))
            .next()
            .unwrap();
        let keep = to_fix[0];
        let to_remove = to_fix[1];
        let overlap = get_overlap(&keep, &to_remove);
        let to_add = get_new_cuboids_ranges(overlap);
        problem_cuboids.remove(&to_remove);
        for cuboid in to_add {
            problem_cuboids.insert(cuboid);
        }
    }

    for cuboid in problem_cuboids.drain() {
        cuboids.insert(cuboid);
    }
}

fn turn_things_off(cuboids: &mut HashSet<Cuboid>, off_boid: Cuboid) {
    let need_altered: Vec<_> = cuboids.iter().copied().filter(|boid| check_cubes_overlap(*boid, off_boid)).collect();
    for cuboid in need_altered {
        cuboids.remove(&cuboid);
        let overlap = get_overlap(&off_boid, &cuboid);
        let to_add = get_new_cuboids_ranges(overlap);
        for cuboid in to_add {
            cuboids.insert(cuboid);
        }
    }
}

fn get_overlap(existing: &Cuboid, new: &Cuboid) -> Overlap {
    Overlap {
        x: get_overlap_of_ranges(existing.x_range, new.x_range),
        y: get_overlap_of_ranges(existing.y_range, new.y_range),
        z: get_overlap_of_ranges(existing.z_range, new.z_range),
    }
}

impl From<((i64, i64), (i64, i64), (i64, i64))> for Cuboid {
    fn from((x_range, y_range, z_range): ((i64, i64), (i64, i64), (i64, i64))) -> Self {
        Cuboid {
            x_range,
            y_range,
            z_range
        }
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;
    use super::*;
    const TEST_INPUT: &str = include_str!("../../test_inputs/day22.txt");
    const BIG_TEST_INPUT: &str = include_str!("../../test_inputs/day22_bigger.txt");

    #[test]
    fn test_range_combos() {
        let inner = Cuboid {
            x_range: (0, 0),
            y_range: (0, 0),
            z_range: (0, 0)
        };
        let outer = Cuboid {
            x_range: (-1, 1),
            y_range: (-1, 1),
            z_range: (-1, 1),
        };

        let overlap = get_overlap(&inner, &outer);
        let new_ranges = get_new_cuboids_ranges(overlap);
        assert_eq!(new_ranges.len(), 26);
        assert!(new_ranges.contains(&((-1, -1), (-1, -1), (-1, -1)).into()));
        assert!(new_ranges.contains(&((1, 1), (1, 1), (1, 1)).into()));
        assert!(new_ranges.contains(&((-1, -1), (1, 1), (-1, -1)).into()));
    }

    #[test]
    fn test_overlap_description() {
        let one = Cuboid {
            x_range: (0, 3),
            y_range: (0, 3),
            z_range: (0, 3),
        };
        let two = Cuboid {
            x_range: (2, 4),
            y_range: (2, 4),
            z_range: (2, 4),
        };
        let overlap = get_overlap(&one, &two);
        let expected = Overlap{
            x: OverlapRanges { minus: None, overlap: Some((2, 3)), plus: Some((4, 4)) },
            y: OverlapRanges { minus: None, overlap: Some((2, 3)), plus: Some((4, 4)) },
            z: OverlapRanges { minus: None, overlap: Some((2, 3)), plus: Some((4, 4)) },
        };

        let overlap = get_overlap(&two, &one);
        let expected = Overlap{
            x: OverlapRanges { minus: Some((0, 1)), overlap: Some((2, 3)), plus: None },
            y: OverlapRanges { minus: Some((0, 1)), overlap: Some((2, 3)), plus: None },
            z: OverlapRanges { minus: Some((0, 1)), overlap: Some((2, 3)), plus: None },
        };
        assert_eq!(overlap, expected)
    }

    #[test]
    fn test_overlapping_cubes() {
        assert!(
            check_cubes_overlap(
            Cuboid::new((0, 0), (0, 0), (0, 0)),
            Cuboid::new((0, 0), (0, 0), (0, 0))
            )
        );
        assert!(
            !check_cubes_overlap(
                Cuboid::new((0, 0), (0, 0), (0, 0)),
                Cuboid::new((1, 1), (1, 1), (1, 1))
            )
        );
        assert!(
            check_cubes_overlap(
                Cuboid::new((-1, 1), (1, 1), (1, 1)),
                Cuboid::new((1, 3), (1, 3), (1, 3))
            )
        );
        assert!(
            !check_cubes_overlap(
                Cuboid::new((-1, 1), (1, 1), (1, 1)),
                Cuboid::new((-1, 1), (10, 12), (1, 3))
            )
        );

    }
    #[test]
    fn test_overlapping_ranges() {
        assert!(ranges_overlap((0, 0), (0, 0)));
        assert!(ranges_overlap((0, 1), (1, 2)));
        assert!(ranges_overlap((-10, 0), (0, 1)));
        assert!(!ranges_overlap((0, 0), (1, 2)));
        assert!(!ranges_overlap((-10, 0), (1, 1)));
    }

    #[test]
    fn test_part_one() {
        let input = parse_from_str(TEST_INPUT);
        assert_eq!(part_one(input), 590784)
    }

    #[test]
    fn test_efficient_part_one() {
        let input = parse_from_str(TEST_INPUT);
        assert_eq!(efficient_part_one(input), 590784)
    }

    #[test]
    fn test_part_two() {
        let input = parse_from_str(BIG_TEST_INPUT);
        assert_eq!(part_two(input), 2758514936282235);
    }

    #[ignore]
    #[test]
    fn check_some_subsets() {
        let input = parse_from_str(TEST_INPUT);
        assert_eq!(part_one(input.clone()), efficient_part_one(input));
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
    fn check_performing_instructions() {
        let mut cuboids: HashSet<_> = vec![Cuboid::new((0, 0), (0, 0), (0, 0))].into_iter().collect();
        assert_eq!(count_on(&cuboids), 1);
        add_and_remove_overlapping(&mut cuboids, Cuboid::new((-1, 1), (-1, 1), (-1, 1)));
        assert_eq!(count_on(&cuboids), 27);
        add_and_remove_overlapping(&mut cuboids, Cuboid::new((-5, -1), (0, 0), (0, 0)));
        assert_eq!(count_on(&cuboids), 27 + 4);
        turn_things_off(&mut cuboids, Cuboid::new((-10, -10), (-10, -10), (-10, -10)));
        assert_eq!(count_on(&cuboids), 27 + 4);
        turn_things_off(&mut cuboids, Cuboid::new((-5, -5), (0, 0), (0, 0)));
        assert_eq!(count_on(&cuboids), 27 + 3);
        turn_things_off(&mut cuboids, Cuboid::new((-2, -1), (-1, 1), (-1, 1)));
        assert_eq!(count_on(&cuboids), 27 + 3 - 9 - 1)
    }

    #[test]
    fn check_failing() {
        let mut cuboids: HashSet<_> = HashSet::new();
        add_and_remove_overlapping(&mut cuboids, Cuboid::new((-1, 1), (-1, 1), (-1, 1)));
        add_and_remove_overlapping(&mut cuboids, Cuboid::new((0, 0), (-2, 0), (-2, 0)));
        println!("{:?}", cuboids);
        assert_eq!(count_on(&cuboids), 27 + 5)
    }

    #[test]
    fn test_on_size_from_ranges() {
        let cuboids: HashSet<_> = vec![Cuboid::new((0, 0), (0, 0), (0, 0))].into_iter().collect();
        assert_eq!(count_on(&cuboids), 1);
        let cuboids: HashSet<_> = vec![Cuboid::new((-1, 1), (-1, 1), (-1, 1))].into_iter().collect();
        assert_eq!(count_on(&cuboids), 27);
        let cuboids: HashSet<_> = vec![
            Cuboid::new((-1, 1), (-1, 1), (-1, 1)),
            Cuboid::new((-10, -8), (-10, -8), (-10, -8)),
        ]
        .into_iter()
        .collect();
        assert_eq!(count_on(&cuboids), 27 * 2);
    }

    #[test]
    fn test_overlapping_reduction() {
        let base_instruction = Instruction {
            action: Action::On,
            cuboid: Cuboid {
                x_range: (-1, 1),
                y_range: (-1, 1),
                z_range: (-1, 1),
            },
        };
        let on_cuboids: HashSet<Cuboid> = vec![base_instruction.cuboid].into_iter().collect();
        let _off_instruction = Instruction {
            action: Action::Off,
            cuboid: Cuboid {
                x_range: (1, 3),
                y_range: (1, 3),
                z_range: (1, 3),
            },
        };

        assert_eq!(count_on(&on_cuboids), 8);
    }

    #[test]
    #[ignore]
    fn check_any_overlapping_offs_in_input() {
        let input = parse_from_file("./inputs/day22.txt");
        let overlapping_offs = input
            .into_iter()
            .filter(|instruction| instruction.action == Action::Off)
            .combinations(2)
            .filter(|combo| check_cubes_overlap(combo[0].cuboid, combo[1].cuboid))
            .count();
        assert_eq!(overlapping_offs, 0);
    }
}
