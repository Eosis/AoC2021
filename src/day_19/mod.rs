use regex::Regex;
use std::fs::read_to_string;
use std::path::Path;
use hashbrown::{HashMap, HashSet};

type Input = HashMap<usize, Vec<(i32, i32, i32)>>;
type Solved = HashMap<usize, ((i32, i32, i32), Vec<(i32, i32, i32)>)>;
pub fn solve_part_1() -> Result<(), ()> {
    let input = parse_from_file("./inputs/day20.txt");
    println!("Solution: {}", part_one(input));
    Ok(())
}

pub fn solve_part_2() -> Result<(), ()> {
    let input = parse_from_file("./inputs/day20.txt");
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
                .map(|line| -> (i32, i32, i32) {
                    let items:Vec<_> = line
                        .split(',')
                        .map(|n| n.parse::<i32>().unwrap())
                        .collect();
                    (items[0], items[1], items[2])
                })
                .collect();
            (i, beacons)
        }).collect()
}

fn rotate_about_x((x, y, z): (i32, i32, i32)) -> (i32, i32, i32) {
    (x, -z, y)
}

fn rotate_about_z((x, y, z): (i32, i32, i32)) -> (i32, i32, i32) {
    (-y, x, z)
}

fn rotate_about_y((x, y, z): (i32, i32, i32)) -> (i32, i32, i32) {
    (z, y, -x)
}

fn rotate_point_about(mut point: (i32, i32, i32),
                      (x_times, y_times, z_times): (usize, usize, usize)
) -> (i32, i32, i32) {
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
    (3, 3, 0)
];




pub fn part_one(input: Input) -> usize {
    unimplemented!();
}

pub fn part_two(input: Input) -> usize {
    unimplemented!();
}

struct OverlappingCubes {
    rotation: (usize, usize, usize),
    equal_beacons: (usize, usize),
}
fn overlapping_cubes(
    known_beacon: &Vec<(i32, i32, i32)>,
    potential: &Vec<(i32, i32, i32)>
) -> Option<OverlappingCubes> {
    for rotation in ROTATIONS.iter().copied() {
        let rotated_beacons: Vec<_> = potential.
            iter()
            .copied()
            .map(|point| rotate_point_about(point, rotation))
            .collect();
        if let Some(equal_beacons) = check_overlap_after_rotation(&known_beacon, &rotated_beacons) {
            return Some(OverlappingCubes{ rotation, equal_beacons } );
        }
    }
    None
}


fn check_overlap_after_rotation(known: &Vec<(i32, i32, i32)>, potential: &Vec<(i32, i32, i32)>) -> Option<(usize, usize)> {
    for known_lead_idx in 0..known.len() {
        let known_lead = known[known_lead_idx];
        let known_others = known
            .iter()
            .copied()
            .enumerate()
            .filter(|(idx, _)| *idx != known_lead_idx)
            .map(|(_, other)| other);
        let differences_from_known_lead: HashSet<(i32, i32, i32)> = known_others
            .map(|known_other| (
                known_other.0 - known_lead.0,
                known_other.1 - known_lead.1,
                known_other.2 - known_lead.2
            ))
            .collect();

        for lead_idx in 0..potential.len() {
            let lead = potential[lead_idx];
            let others = potential
                .iter()
                .copied()
                .enumerate()
                .filter(|(idx, _)| *idx != lead_idx)
                .map(|(_, other)| other);
            let differences_from_this_lead: HashSet<(i32, i32, i32)> = others
                .map(|other| (other.0 - lead.0, other.1 - lead.1, other.2 - lead.2))
                .collect();
            let intersection = differences_from_known_lead.intersection(&differences_from_this_lead);
            if intersection.count() >= 11 {
                return Some((known_lead_idx, lead_idx))
            }
        }
    }
    None
}

fn transform_beacon(known_scanner: &((i32, i32, i32), Vec<(i32, i32, i32)>),
                    scanner: &Vec<(i32, i32, i32)>,
                    overlap: OverlappingCubes) -> ((i32, i32, i32), Vec<(i32, i32, i32)>) {
    let scanner_with_correct_rotation: Vec<_> = scanner
        .iter()
        .copied()
        .map(|point| rotate_point_about(point, overlap.rotation))
        .collect();
    let known_beacon = known_scanner.1[overlap.equal_beacons.0];
    let unknown_beacon = scanner_with_correct_rotation[overlap.equal_beacons.1];
    let new_position = (
        known_beacon.0 - unknown_beacon.0,
        known_beacon.1 - unknown_beacon.1,
        known_beacon.2 - unknown_beacon.2
    );

    // Transform all the points of the other beacon to be relative to 0 beacon ... ?
    (new_position, vec![])
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = include_str!("../../test_inputs/day19.txt");

    fn eight_things() -> impl Iterator<Item=(i32, i32, i32)> {
        (0..8).map(|n|
            (
                if (n >> 2) % 2 == 0 { -1 } else { 1 },
                if (n >> 1) % 2 == 0 { -1 } else { 1 },
                if n % 2 == 0 { -1 } else { 1 },
            )
        )
    }

    #[test]
    #[ignore]
    fn test_dem_eight_things() {
        println!("{:?}", eight_things().collect::<Vec<_>>());
    }
    #[test]
    fn test_part_one() {
        let input = parse_from_str(TEST_INPUT);
        assert_eq!(part_one(input), 35)
    }

    #[test]
    fn test_part_two() {
        unimplemented!();
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
            (-1, -1,  1),
            (-2, -2,  2),
            (-3, -3,  3),
            (-2, -3,  1),
            ( 5,  6, -4),
            ( 8,  0,  7),
        ];
        for rotation in ROTATIONS.iter().copied() {
            println!("Rotation: {:?}", rotation);
            println!("{:#?}",
                example_beacons
                    .iter()
                    .copied()
                    .map(|beacon|
                        rotate_point_about(beacon, rotation)
                ).collect::<Vec<_>>());
        }
    }

    #[test]
    fn check_overlapping_detection_cubes() {
        let beacons = parse_from_str(TEST_INPUT);
        assert!(overlapping_cubes(
            beacons.get(&0).unwrap(),
            beacons.get(&1).unwrap()
        ).is_some());

        assert!(overlapping_cubes(
            beacons.get(&1).unwrap(),
            beacons.get(&4).unwrap()
        ).is_some());

        assert!(!overlapping_cubes(
            beacons.get(&0).unwrap(),
            beacons.get(&3).unwrap()
        ).is_some());
    }

    #[test]
    fn check_correct_relative_location_from_overlapping() {
        let mut scanners = parse_from_str(TEST_INPUT);
        let mut known_scanners: Solved = HashMap::new();
        known_scanners.insert(0, ((0, 0, 0), scanners.remove(&0).unwrap()));
        let overlap_result = overlapping_cubes(
            &known_scanners.get(&0).unwrap().1,
            scanners.get(&1).unwrap(),
        ).unwrap();
        let new_known_beacon = transform_beacon(
            known_scanners.get(&0).unwrap(),
            scanners.get(&1).unwrap(),
            overlap_result
        );
        assert_eq!(new_known_beacon.0, (68,-1246,-43));
    }
}
