use hashbrown::{HashMap, HashSet};
use itertools::Itertools;
use regex::Regex;
use std::collections::VecDeque;
use std::fs::read_to_string;
use std::ops::{Add, Sub};
use std::path::Path;

#[derive(Clone, Copy, Eq, PartialEq, Debug, Hash)]
pub struct Point(i32, i32, i32);

impl Add for Point {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Point(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl Sub for Point {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Point(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

type Input = HashMap<usize, Vec<Point>>;
type SolvedScanner = (Point, Vec<Point>);
type Solved = HashMap<usize, (Point, Vec<Point>)>;
pub fn solve_part_1() -> Result<(), ()> {
    let input = parse_from_file("./inputs/day19.txt");
    println!("Solution: {}", part_one(input));
    Ok(())
}

pub fn solve_part_2() -> Result<(), ()> {
    let input = parse_from_file("./inputs/day19.txt");
    println!("Solution: {}", part_two(input));
    Ok(())
}

fn parse_from_file<T: AsRef<Path>>(filename: T) -> Input {
    let input = read_to_string(filename).unwrap();
    parse_from_str(&input)
}

fn parse_from_str(input: &str) -> Input {
    let header = Regex::new(r"---.*scanner.*---").unwrap();

    header
        .split(input)
        .into_iter()
        .skip(1)
        .enumerate()
        .map(|(i, scanner_entry)| {
            let beacons = scanner_entry
                .lines()
                .filter(|line| !line.is_empty())
                .map(|line| -> Point {
                    let items: Vec<_> = line.split(',').map(|n| n.parse::<i32>().unwrap()).collect();
                    Point(items[0], items[1], items[2])
                })
                .collect();
            (i, beacons)
        })
        .collect()
}

fn rotate_about_x(Point(x, y, z): Point) -> Point {
    Point(x, -z, y)
}

fn rotate_about_z(Point(x, y, z): Point) -> Point {
    Point(-y, x, z)
}

fn rotate_about_y(Point(x, y, z): Point) -> Point {
    Point(z, y, -x)
}

fn rotate_point_about(mut point: Point, (x_times, y_times, z_times): (usize, usize, usize)) -> Point {
    for _ in 0..y_times {
        point = rotate_about_y(point);
    }
    for _ in 0..z_times {
        point = rotate_about_z(point);
    }
    for _ in 0..x_times {
        point = rotate_about_x(point);
    }
    point
}

const ROTATIONS: [(usize, usize, usize); 24] = [
    (0, 0, 0),
    (1, 0, 0),
    (2, 0, 0),
    (3, 0, 0),
    (0, 0, 1),
    (1, 0, 1),
    (2, 0, 1),
    (3, 0, 1),
    (0, 0, 2),
    (1, 0, 2),
    (2, 0, 2),
    (3, 0, 2),
    (0, 0, 3),
    (1, 0, 3),
    (2, 0, 3),
    (3, 0, 3),
    (0, 1, 0),
    (1, 1, 0),
    (2, 1, 0),
    (3, 1, 0),
    (0, 3, 0),
    (1, 3, 0),
    (2, 3, 0),
    (3, 3, 0),
];

pub fn part_one(input: Input) -> usize {
    count_unique_beacons(solve_for_scanners(input))
}

fn solve_for_scanners(mut input: Input) -> Solved {
    let mut known_scanners: HashMap<usize, SolvedScanner> = HashMap::new();
    known_scanners.insert(0, (Point(0, 0, 0), input.remove(&0).unwrap()));
    let mut known_to_check_against: VecDeque<usize> = VecDeque::new();
    known_to_check_against.push_back(0);

    while dbg!(known_to_check_against.len()) > 0 && dbg!(input.len()) > 0 {
        let mut to_remove_from_input: Vec<usize> = vec![];
        let current_known_id = known_to_check_against.pop_front().unwrap();
        let current_known = known_scanners.get(&current_known_id).unwrap().clone();
        for (id, scanner) in input.iter() {
            if let Some(overlap_result) = overlapping_cubes(&current_known.1, scanner) {
                let new_known_beacon = transform_beacon(&current_known, scanner, overlap_result);
                known_scanners.insert(*id, new_known_beacon);
                known_to_check_against.push_back(*id);
                to_remove_from_input.push(*id);
            }
        }
        for to_remove in &to_remove_from_input {
            input.remove(to_remove);
        }
    }
    known_scanners
}

fn count_unique_beacons(solved: Solved) -> usize {
    solved
        .iter()
        .flat_map(|(_id, scanner)| scanner.1.iter().copied())
        .unique()
        .count()
}

fn manhattan(a: Point, b: Point) -> u32 {
    let distance = b - a;
    (distance.0.abs() + distance.1.abs() + distance.2.abs()) as u32
}

pub fn part_two(input: Input) -> u32 {
    let solved = solve_for_scanners(input);
    solved
        .iter()
        .combinations(2)
        .map(|items| manhattan(items[0].1 .0, items[1].1 .0))
        .max()
        .unwrap()
}

struct OverlappingCubes {
    rotation: (usize, usize, usize),
    equal_beacons: (usize, usize),
}

fn overlapping_cubes(known_beacon: &[Point], potential: &[Point]) -> Option<OverlappingCubes> {
    for rotation in ROTATIONS.iter().copied() {
        let rotated_beacons: Vec<_> = potential
            .iter()
            .copied()
            .map(|point| rotate_point_about(point, rotation))
            .collect();
        if let Some(equal_beacons) = check_overlap_after_rotation(known_beacon, &rotated_beacons) {
            return Some(OverlappingCubes {
                rotation,
                equal_beacons,
            });
        }
    }
    None
}

fn check_overlap_after_rotation(known: &[Point], potential: &[Point]) -> Option<(usize, usize)> {
    for known_lead_idx in 0..known.len() {
        let known_lead = known[known_lead_idx];
        let known_others = known
            .iter()
            .copied()
            .enumerate()
            .filter(|(idx, _)| *idx != known_lead_idx)
            .map(|(_, other)| other);
        let differences_from_known_lead: HashSet<Point> =
            known_others.map(|known_other| known_other - known_lead).collect();

        for lead_idx in 0..potential.len() {
            let lead = potential[lead_idx];
            let others = potential
                .iter()
                .copied()
                .enumerate()
                .filter(|(idx, _)| *idx != lead_idx)
                .map(|(_, other)| other);
            let differences_from_this_lead: HashSet<Point> = others.map(|other| other - lead).collect();
            let intersection = differences_from_known_lead.intersection(&differences_from_this_lead);
            if intersection.count() >= 11 {
                return Some((known_lead_idx, lead_idx));
            }
        }
    }
    None
}

fn transform_beacon(
    known_scanner: &(Point, Vec<Point>),
    scanner: &[Point],
    overlap: OverlappingCubes,
) -> (Point, Vec<Point>) {
    let scanner_with_correct_rotation: Vec<_> = scanner
        .iter()
        .copied()
        .map(|point| rotate_point_about(point, overlap.rotation))
        .collect();
    let known_beacon = known_scanner.1[overlap.equal_beacons.0];
    let unknown_beacon = scanner_with_correct_rotation[overlap.equal_beacons.1];
    let new_position = known_beacon - unknown_beacon;
    let positions_from_known_scanner = scanner_with_correct_rotation
        .into_iter()
        .map(|point| new_position + point)
        .collect();

    // Transform all the points of the other beacon to be relative to 0 beacon ... ?
    (new_position, positions_from_known_scanner)
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = include_str!("../../test_inputs/day19.txt");

    fn eight_things() -> impl Iterator<Item = (i32, i32, i32)> {
        (0..8).map(|n| {
            (
                if (n >> 2) % 2 == 0 { -1 } else { 1 },
                if (n >> 1) % 2 == 0 { -1 } else { 1 },
                if n % 2 == 0 { -1 } else { 1 },
            )
        })
    }

    #[test]
    #[ignore]
    fn test_dem_eight_things() {
        println!("{:?}", eight_things().collect::<Vec<_>>());
    }
    #[test]
    fn test_part_one() {
        let input = parse_from_str(TEST_INPUT);
        assert_eq!(part_one(input), 79)
    }

    #[test]
    fn test_part_two() {
        let input = parse_from_str(TEST_INPUT);
        assert_eq!(part_two(input), 3621)
    }

    #[test]
    #[ignore]
    fn test_parse_report() {
        println!("{:#?}", parse_from_str(TEST_INPUT));
    }

    #[test]
    #[ignore]
    fn test_generating_rotations() {
        let example_beacons = [
            Point(-1, -1, 1),
            Point(-2, -2, 2),
            Point(-3, -3, 3),
            Point(-2, -3, 1),
            Point(5, 6, -4),
            Point(8, 0, 7),
        ];
        for rotation in ROTATIONS.iter().copied() {
            println!("Rotation: {:?}", rotation);
            println!(
                "{:#?}",
                example_beacons
                    .iter()
                    .copied()
                    .map(|beacon| rotate_point_about(beacon, rotation))
                    .collect::<Vec<_>>()
            );
        }
    }

    #[test]
    fn check_overlapping_detection_cubes() {
        let beacons = parse_from_str(TEST_INPUT);
        assert!(overlapping_cubes(beacons.get(&0).unwrap(), beacons.get(&1).unwrap()).is_some());

        assert!(overlapping_cubes(beacons.get(&1).unwrap(), beacons.get(&4).unwrap()).is_some());

        assert!(!overlapping_cubes(beacons.get(&0).unwrap(), beacons.get(&3).unwrap()).is_some());
    }

    #[test]
    fn check_correct_relative_location_from_overlapping() {
        let mut scanners = parse_from_str(TEST_INPUT);
        let mut known_scanners: Solved = HashMap::new();
        known_scanners.insert(0, (Point(0, 0, 0), scanners.remove(&0).unwrap()));
        let overlap_result = overlapping_cubes(&known_scanners.get(&0).unwrap().1, scanners.get(&1).unwrap()).unwrap();
        let new_known_beacon = transform_beacon(
            known_scanners.get(&0).unwrap(),
            scanners.get(&1).unwrap(),
            overlap_result,
        );
        assert_eq!(new_known_beacon.0, Point(68, -1246, -43));
        let set_from_0: HashSet<_> = known_scanners.get(&0).unwrap().1.iter().copied().collect();
        let new_set: HashSet<_> = new_known_beacon.1.iter().copied().collect();
        let expected_set: HashSet<Point> = vec![
            Point(-618, -824, -621),
            Point(-537, -823, -458),
            Point(-447, -329, 318),
            Point(404, -588, -901),
            Point(544, -627, -890),
            Point(528, -643, 409),
            Point(-661, -816, -575),
            Point(390, -675, -793),
            Point(423, -701, 434),
            Point(-345, -311, 381),
            Point(459, -707, 401),
            Point(-485, -357, 347),
        ]
        .into_iter()
        .collect();
        assert_eq!(
            set_from_0.intersection(&new_set).copied().collect::<HashSet<_>>(),
            expected_set
        );
    }

    #[test]
    fn test_manhattan() {
        assert_eq!(manhattan(Point(1105, -1205, 1229), Point(-92, -2380, -20)), 3621)
    }
}
