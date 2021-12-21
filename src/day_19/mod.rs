use regex::Regex;
use std::fs::read_to_string;
use std::path::Path;
use hashbrown::{HashMap, HashSet};

type Input = HashMap<usize, Vec<(i32, i32, i32)>>;

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

// Switching up the Z: We will get 4 of these.
fn rotate_about_x((x, y, z): (i32, i32, i32)) -> (i32, i32, i32) {
    (x, z, -y)
}

// Switching up the "facing" direction
fn rotate_about_z((x, y, z): (i32, i32, i32)) -> (i32, i32, i32) {
    (y, -x, z)
}

// Missing 2... Sort of... Man I'm crap at this!
fn rotate_about_y((x, y, z): (i32, i32, i32)) -> (i32, i32, i32) {
    (-z, y, x)
}

fn rotate_point_about(mut point: (i32, i32, i32),
                      (x_times, y_times, z_times): (usize, usize, usize)
) -> (i32, i32, i32) {
    for _ in 0..x_times {
        point = rotate_about_x(point);
    }
    for _ in 0..y_times {
        point = rotate_about_y(point);
    }
    for _ in 0..z_times {
        point = rotate_about_z(point);
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

fn overlapping_cubes(
    known_beacon: &Vec<(i32, i32, i32)>,
    potential: &Vec<(i32, i32, i32)>
) -> Option<(usize, usize, usize)> {
    for rotation in ROTATIONS.iter().copied() {
        let rotated_beacons: Vec<_> = potential.
            iter()
            .copied()
            .map(|point| rotate_point_about(point, rotation))
            .collect();
        if check_overlap_after_rotation(&known_beacon, &rotated_beacons) {
            return dbg!(Some(rotation));
        }
    }
    None
}



fn check_overlap_after_rotation(known: &Vec<(i32, i32, i32)>, potential: &Vec<(i32, i32, i32)>) -> bool {
    let known_lead = known[0];
    let known_others = known.iter().copied()
        .skip(1);
    let differences_from_known_lead: HashSet<(i32, i32, i32)> = known_others
        .map(|known_other| (
            known_other.0 - known_lead.0,
            known_other.1 - known_lead.1,
            known_other.2 - known_lead.2
        ))
        .collect();
    dbg!(&differences_from_known_lead);
    for lead_idx in 0..potential.len() {
        let lead = potential[lead_idx];
        let others = potential
            .iter()
            .copied()
            .enumerate()
            .filter(|(idx, _)| *idx != lead_idx)
            .map(|(_, other)| other);
        let differences_from_this_lead: HashSet<(i32, i32, i32)> = others
            .map(|other|  (other.0 - lead.0, other.1 - lead.1, other.2 - lead.2))
            .collect();
        dbg!(&differences_from_this_lead);
        let intersection = differences_from_known_lead.intersection(&differences_from_this_lead);
        if dbg!(intersection.count()) >= 11 {
            return true;
        }
    }
    false
}



#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = include_str!("../../test_inputs/day19.txt");
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

        // assert!(overlapping_cubes(
        //     beacons.get(&1).unwrap(),
        //     beacons.get(&4).unwrap()
        // ).is_some());

        // assert!(overlapping_cubes(
        //     beacons.get(&1).unwrap(),
        //     beacons.get(&4).unwrap()
        // ).is_some());

        // assert!(!overlapping_cubes(
        //     beacons.get(&0).unwrap(),
        //     beacons.get(&3).unwrap()
        // ).is_some());
    }
}
