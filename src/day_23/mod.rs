use hashbrown::{HashMap, HashSet};
use std::collections::VecDeque;
use std::fmt::{Display, Formatter};

pub fn solve_part_1() -> Result<(), ()> {
    println!("{}", part_one());
    Ok(())
}

pub fn solve_part_2() -> Result<(), ()> {
    println!("{}", part_two());
    Ok(())
}

fn part_one() -> usize {
    // #############
    // #...........#
    // ###B#B#C#D###
    //   #D#C#A#A#
    //   #########

    let map = grotto_map();
    let grotto = Grotto {
        active_amphis: vec![
            Amphipod::B(1, 2),
            Amphipod::D(2, 2),
            Amphipod::B(1, 4),
            Amphipod::C(2, 4),
            Amphipod::C(1, 6),
            Amphipod::A(2, 6),
            Amphipod::D(1, 8),
            Amphipod::A(2, 8),
        ]
        .into_iter()
        .collect(),
        finished_amphis: vec![].into_iter().collect(),
        current_score: 0,
        map_depth: 2,
        map: &map,
    };
    let mut min_score: usize = usize::MAX;
    get_min_score(grotto, &mut min_score);
    min_score
}

fn part_two() -> usize {
    // #############
    // #...........#
    // ###B#B#C#D###
    //   #D#C#B#A#
    //   #D#B#A#C#
    //   #D#C#A#A#
    //   #########

    let map = bigger_grotto_map();
    let grotto = Grotto {
        active_amphis: vec![
            Amphipod::B(1, 2),
            Amphipod::D(2, 2),
            Amphipod::D(3, 2),
            Amphipod::D(4, 2),
            Amphipod::B(1, 4),
            Amphipod::C(2, 4),
            Amphipod::B(3, 4),
            Amphipod::C(4, 4),
            Amphipod::C(1, 6),
            Amphipod::B(2, 6),
            Amphipod::A(3, 6),
            Amphipod::A(4, 6),
            Amphipod::D(1, 8),
            Amphipod::A(2, 8),
            Amphipod::C(3, 8),
            Amphipod::A(4, 8),
        ]
        .into_iter()
        .collect(),
        finished_amphis: vec![].into_iter().collect(),
        current_score: 0,
        map_depth: 4,
        map: &map,
    };
    let mut min_score: usize = usize::MAX;
    get_min_score(grotto, &mut min_score);
    min_score
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Hash)]
enum Amphipod {
    A(usize, usize),
    B(usize, usize),
    C(usize, usize),
    D(usize, usize),
}
impl Display for Amphipod {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::A(_, _) => write!(f, "A"),
            Self::B(_, _) => write!(f, "B"),
            Self::C(_, _) => write!(f, "C"),
            Self::D(_, _) => write!(f, "D"),
        }
    }
}

fn default_char_for_row((y, x): (usize, usize)) -> &'static str {
    match (y, x) {
        (0, _) => ".",
        (_, 2) | (_, 4) | (_, 6) | (_, 8) => ".",
        _ => "#",
    }
}
impl<'a> Display for Grotto<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let positions_to_letters: HashMap<(usize, usize), String> = self
            .all_amphis()
            .into_iter()
            .map(|amphi| (amphi.destructure_location(), format!("{}", amphi)))
            .collect();
        println!("{:?}", self.all_amphis());
        println!("{:?}", self.active_amphis);
        println!("{:?}", self.finished_amphis);
        let mut result = String::new();
        for y in 0..=4 {
            result.push_str("\n");
            for x in 0..=10 {
                result.push_str(
                    &positions_to_letters
                        .get(&(y, x))
                        .map(|letter| letter.as_str())
                        .unwrap_or_else(|| default_char_for_row((y, x))),
                );
            }
        }
        write!(f, "{}", result)
    }
}

impl Amphipod {
    fn destructure_location(&self) -> (usize, usize) {
        match self {
            Amphipod::A(y, x) => (*y, *x),
            Amphipod::B(y, x) => (*y, *x),
            Amphipod::C(y, x) => (*y, *x),
            Amphipod::D(y, x) => (*y, *x),
        }
    }
    fn in_new_loc(&self, (y, x): (usize, usize)) -> Amphipod {
        match self {
            Amphipod::A(_, _) => Amphipod::A(y, x),
            Amphipod::B(_, _) => Amphipod::B(y, x),
            Amphipod::C(_, _) => Amphipod::C(y, x),
            Amphipod::D(_, _) => Amphipod::D(y, x),
        }
    }

    fn same_type(&self, rhs: &Amphipod) -> bool {
        match self {
            Amphipod::A(_, _) => match rhs {
                Amphipod::A(_, _) => true,
                _ => false,
            },
            Amphipod::B(_, _) => match rhs {
                Amphipod::B(_, _) => true,
                _ => false,
            },
            Amphipod::C(_, _) => match rhs {
                Amphipod::C(_, _) => true,
                _ => false,
            },
            Amphipod::D(_, _) => match rhs {
                Amphipod::D(_, _) => true,
                _ => false,
            },
        }
    }

    fn cost_of_move(&self, spaces: usize) -> usize {
        match self {
            Amphipod::A(_, _) => spaces,
            Amphipod::B(_, _) => spaces * 10,
            Amphipod::C(_, _) => spaces * 100,
            Amphipod::D(_, _) => spaces * 1000,
        }
    }

    fn in_final_location(&self, grotto: &Grotto) -> bool {
        let in_own_room = match self {
            Amphipod::A(y, x) if *y > 0 && *x == 2 => true,
            Amphipod::B(y, x) if *y > 0 && *x == 4 => true,
            Amphipod::C(y, x) if *y > 0 && *x == 6 => true,
            Amphipod::D(y, x) if *y > 0 && *x == 8 => true,
            _ => false,
        };

        if !in_own_room {
            return false;
        }

        let (y, x) = self.destructure_location();
        (1..=grotto.map_depth)
            .filter(|depth| *depth > y)
            .all(|depth_to_check| grotto.all_amphis().contains(&self.in_new_loc((depth_to_check, x))))
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Move {
    from: Amphipod,
    to: Amphipod,
    cost: usize,
}

impl Move {
    fn new(from: Amphipod, to: ((usize, usize), usize)) -> Self {
        Move {
            from,
            to: from.in_new_loc(to.0),
            cost: from.cost_of_move(to.1),
        }
    }
}

#[derive(Clone, Debug)]
struct Grotto<'a> {
    active_amphis: HashSet<Amphipod>,
    finished_amphis: HashSet<Amphipod>,
    current_score: usize,
    map: &'a HashMap<(usize, usize), Vec<(usize, usize)>>,
    map_depth: usize,
}

impl<'a> Grotto<'a> {
    fn all_amphis(&self) -> HashSet<Amphipod> {
        self.finished_amphis.union(&self.active_amphis).copied().collect()
    }

    fn move_finished_amphis(&mut self) {
        let finished_amphis: Vec<_> = self
            .active_amphis
            .clone()
            .iter()
            .copied()
            .filter(|amphi| amphi.in_final_location(&self))
            .collect();
        for amphi in finished_amphis {
            self.active_amphis.remove(&amphi);
            self.finished_amphis.insert(amphi);
        }
    }

    fn is_finished(&self) -> bool {
        self.finished_amphis.len() == if self.map_depth == 2 { 8 } else { 16 }
    }
}

fn make_move<'a>(grotto: &'a Grotto, move_to_make: &Move) -> Grotto<'a> {
    let mut new_grotto = grotto.clone();
    new_grotto.active_amphis.remove(&move_to_make.from);
    new_grotto.active_amphis.insert(move_to_make.to);
    new_grotto.current_score += move_to_make.cost;
    new_grotto.move_finished_amphis();
    new_grotto
}
const DEBUG: bool = false;
fn get_min_score(mut grotto: Grotto, min_score: &mut usize) -> Option<usize> {
    if grotto.current_score >= *min_score {
        if DEBUG {
            println!("Exit Early");
        }
        return None;
    }

    grotto.move_finished_amphis();

    let available_moves = get_all_possible_moves(&grotto);
    if available_moves.is_empty() && !grotto.is_finished() {
        if DEBUG {
            println!("fuct");
        }
        return None;
    }
    for to_make in available_moves {
        if DEBUG {
            println!("Making Move: {:?}", to_make);
        }
        let new_grotto = make_move(&grotto, &to_make);
        if DEBUG {
            println!("New state: \n{}\n", new_grotto);
            let mut buf = String::new();
            let _x = std::io::stdin().read_line(&mut buf).unwrap();
        }
        if new_grotto.is_finished() && new_grotto.current_score < *min_score {
            *min_score = new_grotto.current_score;
            dbg!(*min_score);
        } else {
            get_min_score(new_grotto, min_score);
        }
    }
    Some(*min_score)
}

fn grotto_map() -> HashMap<(usize, usize), Vec<(usize, usize)>> {
    vec![
        ((0, 0), vec![(0, 1)]),
        ((0, 1), vec![(0, 0), (0, 2)]),
        ((0, 2), vec![(0, 1), (0, 3), (1, 2)]),
        ((0, 3), vec![(0, 2), (0, 4)]),
        ((0, 4), vec![(0, 3), (0, 5), (1, 4)]),
        ((0, 5), vec![(0, 4), (0, 6)]),
        ((0, 6), vec![(0, 5), (0, 7), (1, 6)]),
        ((0, 7), vec![(0, 6), (0, 8)]),
        ((0, 8), vec![(0, 7), (0, 9), (1, 8)]),
        ((0, 9), vec![(0, 8), (0, 10)]),
        ((0, 10), vec![(0, 9)]),
        ((1, 2), vec![(0, 2), (2, 2)]),
        ((2, 2), vec![(1, 2)]),
        ((1, 4), vec![(0, 4), (2, 4)]),
        ((2, 4), vec![(1, 4)]),
        ((1, 6), vec![(0, 6), (2, 6)]),
        ((2, 6), vec![(1, 6)]),
        ((1, 8), vec![(0, 8), (2, 8)]),
        ((2, 8), vec![(1, 8)]),
    ]
    .into_iter()
    .collect()
}

fn bigger_grotto_map() -> HashMap<(usize, usize), Vec<(usize, usize)>> {
    vec![
        ((0, 0), vec![(0, 1)]),
        ((0, 1), vec![(0, 0), (0, 2)]),
        ((0, 2), vec![(0, 1), (0, 3), (1, 2)]),
        ((0, 3), vec![(0, 2), (0, 4)]),
        ((0, 4), vec![(0, 3), (0, 5), (1, 4)]),
        ((0, 5), vec![(0, 4), (0, 6)]),
        ((0, 6), vec![(0, 5), (0, 7), (1, 6)]),
        ((0, 7), vec![(0, 6), (0, 8)]),
        ((0, 8), vec![(0, 7), (0, 9), (1, 8)]),
        ((0, 9), vec![(0, 8), (0, 10)]),
        ((0, 10), vec![(0, 9)]),
        ((1, 2), vec![(0, 2), (2, 2)]),
        ((2, 2), vec![(1, 2), (3, 2)]),
        ((3, 2), vec![(2, 2), (4, 2)]),
        ((4, 2), vec![(3, 2)]),
        ((1, 4), vec![(0, 4), (2, 4)]),
        ((2, 4), vec![(1, 4), (3, 4)]),
        ((3, 4), vec![(2, 4), (4, 4)]),
        ((4, 4), vec![(3, 4)]),
        ((1, 6), vec![(0, 6), (2, 6)]),
        ((2, 6), vec![(1, 6), (3, 6)]),
        ((3, 6), vec![(2, 6), (4, 6)]),
        ((4, 6), vec![(3, 6)]),
        ((1, 8), vec![(0, 8), (2, 8)]),
        ((2, 8), vec![(1, 8), (3, 8)]),
        ((3, 8), vec![(2, 8), (4, 8)]),
        ((4, 8), vec![(3, 8)]),
    ]
    .into_iter()
    .collect()
}

fn is_corridor((y, _): (usize, usize)) -> bool {
    y == 0
}

fn doorway((y, x): (usize, usize)) -> bool {
    y == 0 && matches!(x, 2 | 4 | 6 | 8)
}

fn is_room((y, _): (usize, usize)) -> bool {
    y > 0
}

fn is_allowed_room(prospect: Amphipod) -> bool {
    match prospect {
        Amphipod::A(_, x) => x == 2,
        Amphipod::B(_, x) => x == 4,
        Amphipod::C(_, x) => x == 6,
        Amphipod::D(_, x) => x == 8,
    }
}

fn is_occupied((y, x): (usize, usize), grotto: &Grotto) -> bool {
    grotto
        .all_amphis()
        .iter()
        .any(|amphi| amphi.destructure_location() == (y, x))
}

fn room_contains_other_type(prospect: Amphipod, grotto: &Grotto) -> bool {
    let prospect_location = prospect.destructure_location();
    let mut in_same_room = grotto.active_amphis.iter().filter(|amphi| {
        let loc = amphi.destructure_location();
        loc.0 > 0 && loc.1 == prospect_location.1
    });
    in_same_room.any(|amphi| !prospect.same_type(amphi))
}

fn is_deepest_available(amphipod: Amphipod, grotto: &Grotto) -> bool {
    let (y, x) = amphipod.destructure_location();

    let amphi_locations: HashSet<_> = grotto
        .all_amphis()
        .into_iter()
        .map(|amphi| amphi.destructure_location())
        .collect();

    (1..=grotto.map_depth)
        .filter(|depth| *depth > y)
        .all(|depth_to_check| amphi_locations.contains(&(depth_to_check, x)))
}

fn able_to_stop(current: Amphipod, prospect: Amphipod, grotto: &Grotto) -> bool {
    let prospect_location = prospect.destructure_location();
    let current_location = current.destructure_location();
    if doorway(prospect_location) {
        return false;
    }

    if is_room(prospect_location) {
        if !is_allowed_room(prospect) {
            return false;
        }
        if room_contains_other_type(prospect, grotto) {
            return false;
        }
        if !is_deepest_available(prospect, grotto) {
            return false;
        }
    }

    if is_corridor(current_location) && is_corridor(prospect_location) {
        return false;
    }
    true
}

type Distance = usize;
fn get_adjacent_unoccupied_spaces(
    location: (usize, usize),
    grotto: &Grotto,
    distance: usize,
) -> VecDeque<((usize, usize), Distance)> {
    grotto
        .map
        .get(&location)
        .unwrap()
        .iter()
        .copied()
        .filter(|location| !is_occupied(*location, grotto))
        .map(|location| (location, distance + 1))
        .collect()
}

fn get_unobstructed_spaces(grotto: &Grotto, amphipod: Amphipod) -> HashMap<(usize, usize), Distance> {
    let current_location = amphipod.destructure_location();
    let current_distance = 0;
    let mut to_explore = get_adjacent_unoccupied_spaces(current_location, grotto, current_distance);
    let mut explored: HashMap<(usize, usize), Distance> = HashMap::new();

    while !to_explore.is_empty() {
        let to_add = to_explore.pop_front().unwrap();
        let current_location = to_add.0;
        let current_distance = to_add.1;
        explored.insert(to_add.0, to_add.1);
        let mut next_to_explore = get_adjacent_unoccupied_spaces(current_location, grotto, current_distance)
            .into_iter()
            .filter(|(loc, _distance)| !explored.get(loc).is_some())
            .collect();
        to_explore.append(&mut next_to_explore);
    }
    explored
}

fn get_all_possible_moves(grotto: &Grotto) -> Vec<Move> {
    let all: Vec<Move> = grotto
        .active_amphis
        .iter()
        .flat_map(|amphi| get_possible_moves(grotto, *amphi))
        .collect();
    if let Some(possible) = all
        .iter()
        .copied()
        .find(|possible| possible.to.in_final_location(grotto))
    {
        vec![possible]
    } else {
        all
    }
}

fn get_possible_moves(grotto: &Grotto, amphipod: Amphipod) -> Vec<Move> {
    let unobstructed_spaces = get_unobstructed_spaces(grotto, amphipod);
    unobstructed_spaces
        .into_iter()
        .filter(|(loc, _)| able_to_stop(amphipod, amphipod.in_new_loc(*loc), grotto))
        .map(|item| Move::new(amphipod, item))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    

    #[test]
    fn get_min_score_of_grotto() {
        let map = grotto_map();
        let mut min = usize::MAX;
        let grotto = Grotto {
            active_amphis: vec![
                Amphipod::A(2, 2),
                Amphipod::B(1, 2),
                Amphipod::D(2, 4),
                Amphipod::C(1, 4),
                Amphipod::C(2, 6),
                Amphipod::B(1, 6),
                Amphipod::A(2, 8),
                Amphipod::D(1, 8),
            ]
            .into_iter()
            .collect(),
            finished_amphis: vec![].into_iter().collect(),
            current_score: 0,
            map: &map,
            map_depth: 2,
        };
        get_min_score(grotto, &mut min);
        assert_eq!(min, 12521);
    }

    #[test]
    fn get_min_score_of_bigger_grotto() {
        let map = bigger_grotto_map();
        let mut min = usize::MAX;
        let grotto = Grotto {
            active_amphis: vec![
                Amphipod::B(1, 2),
                Amphipod::D(2, 2),
                Amphipod::D(3, 2),
                Amphipod::A(4, 2),
                Amphipod::C(1, 4),
                Amphipod::C(2, 4),
                Amphipod::B(3, 4),
                Amphipod::D(4, 4),
                Amphipod::B(1, 6),
                Amphipod::B(2, 6),
                Amphipod::A(3, 6),
                Amphipod::C(4, 6),
                Amphipod::D(1, 8),
                Amphipod::A(2, 8),
                Amphipod::C(3, 8),
                Amphipod::A(4, 8),
            ]
            .into_iter()
            .collect(),
            finished_amphis: vec![].into_iter().collect(),
            current_score: 0,
            map: &map,
            map_depth: 4,
        };
        get_min_score(grotto, &mut min);
        assert_eq!(min, 44169);
    }

    #[test]
    fn test_get_unobstructed_spaces() {
        let map = grotto_map();
        let grotto = Grotto {
            active_amphis: vec![Amphipod::A(2, 2), Amphipod::B(1, 2)].into_iter().collect(),
            finished_amphis: vec![].into_iter().collect(),
            current_score: 0,
            map: &map,
            map_depth: 2,
        };
        let unobstructed_spaces: Vec<(usize, usize)> = get_unobstructed_spaces(&grotto, Amphipod::A(2, 2))
            .into_iter()
            .map(|(location, _)| location)
            .collect();

        assert_eq!(unobstructed_spaces, vec![]);

        let unobstructed_spaces: HashSet<_> = get_unobstructed_spaces(&grotto, Amphipod::B(1, 2))
            .into_iter()
            .map(|(location, _)| location)
            .collect();

        assert_eq!(
            unobstructed_spaces,
            vec![
                (0, 0),
                (0, 1),
                (0, 2),
                (0, 3),
                (0, 4),
                (0, 5),
                (0, 6),
                (0, 7),
                (0, 8),
                (0, 9),
                (0, 10),
                (1, 4),
                (1, 6),
                (1, 8),
                (2, 4),
                (2, 6),
                (2, 8)
            ]
            .into_iter()
            .collect()
        );
        let distances_with_spaces: HashSet<_> = get_unobstructed_spaces(&grotto, Amphipod::B(1, 2))
            .into_iter()
            .collect();
        assert!(distances_with_spaces.contains(&((2, 8), 9)));
        assert!(distances_with_spaces.contains(&((0, 10), 9)));
    }

    #[test]
    fn test_get_possible_moves() {
        let map = grotto_map();
        let grotto = Grotto {
            active_amphis: vec![Amphipod::A(2, 2), Amphipod::B(1, 2)].into_iter().collect(),
            finished_amphis: vec![].into_iter().collect(),
            current_score: 0,
            map: &map,
            map_depth: 2,
        };
        assert_eq!(get_possible_moves(&grotto, Amphipod::A(2, 2)), vec![]);
        assert_eq!(
            get_possible_moves(&grotto, Amphipod::B(1, 2))
                .into_iter()
                .collect::<HashSet<_>>(),
            vec![
                ((0, 1), 2),
                ((0, 9), 8),
                ((2, 4), 5),
                ((0, 3), 2),
                ((0, 5), 4),
                ((0, 0), 3),
                ((0, 7), 6),
                ((0, 10), 9)
            ]
            .into_iter()
            .map(|item| Move::new(Amphipod::B(1, 2), item))
            .collect::<HashSet<_>>()
        );

        let grotto = Grotto {
            active_amphis: vec![Amphipod::A(0, 0), Amphipod::B(2, 2)].into_iter().collect(),
            finished_amphis: vec![].into_iter().collect(),
            current_score: 0,
            map: &map,
            map_depth: 2,
        };
        assert_eq!(
            get_possible_moves(&grotto, Amphipod::A(0, 0))
                .into_iter()
                .collect::<HashSet<_>>(),
            vec![]
                .into_iter()
                .map(|item| Move::new(Amphipod::B(1, 2), item))
                .collect::<HashSet<_>>()
        );

        let grotto = Grotto {
            active_amphis: vec![Amphipod::A(0, 0), Amphipod::B(0, 1)].into_iter().collect(),
            finished_amphis: vec![].into_iter().collect(),
            current_score: 0,
            map: &map,
            map_depth: 2,
        };
        assert_eq!(
            get_possible_moves(&grotto, Amphipod::A(0, 0))
                .into_iter()
                .collect::<HashSet<_>>(),
            vec![]
                .into_iter()
                .map(|item| Move::new(Amphipod::B(1, 2), item))
                .collect::<HashSet<_>>()
        );

        let grotto = Grotto {
            active_amphis: vec![Amphipod::A(0, 0), Amphipod::B(0, 8)].into_iter().collect(),
            finished_amphis: vec![].into_iter().collect(),
            current_score: 0,
            map: &map,
            map_depth: 2,
        };
        assert_eq!(
            get_possible_moves(&grotto, Amphipod::A(0, 0))
                .into_iter()
                .collect::<HashSet<_>>(),
            vec![((2, 2), 4)]
                .into_iter()
                .map(|item| Move::new(Amphipod::A(0, 0), item))
                .collect::<HashSet<_>>()
        );
    }

    #[test]
    fn test_get_bigger_possible_moves() {
        let _map = grotto_map();
        let map = bigger_grotto_map();
        let _min = usize::MAX;
        let grotto = Grotto {
            active_amphis: vec![
                Amphipod::B(1, 2),
                Amphipod::D(2, 2),
                Amphipod::D(3, 2),
                Amphipod::A(4, 2),
            ]
            .into_iter()
            .collect(),
            finished_amphis: vec![].into_iter().collect(),
            current_score: 0,
            map: &map,
            map_depth: 4,
        };
        assert_eq!(get_possible_moves(&grotto, Amphipod::B(1, 2)), vec![]);
    }

    #[test]
    fn check_score() {
        let map = grotto_map();
        let grotto = Grotto {
            active_amphis: vec![Amphipod::A(0, 0), Amphipod::D(0, 10)].into_iter().collect(),
            finished_amphis: vec![].into_iter().collect(),
            current_score: 0,
            map: &map,
            map_depth: 2,
        };
        println!("{}", grotto);
        let set_of_moves = get_possible_moves(&grotto, Amphipod::D(0, 10))
            .into_iter()
            .collect::<HashSet<_>>();
        assert!(set_of_moves.contains(&Move {
            from: Amphipod::D(0, 10),
            to: Amphipod::D(2, 8),
            cost: 4000
        }))
    }

    #[test]
    fn check_get_all_possible_moves() {
        let map = grotto_map();
        let grotto = Grotto {
            active_amphis: vec![Amphipod::A(0, 0)].into_iter().collect(),
            finished_amphis: vec![].into_iter().collect(),
            current_score: 0,
            map: &map,
            map_depth: 2,
        };
        let possible_moves = get_all_possible_moves(&grotto).into_iter().collect::<HashSet<_>>();
        assert!(possible_moves.contains(&Move {
            from: Amphipod::D(0, 10),
            to: Amphipod::D(2, 8),
            cost: 4000
        }));
        assert!(possible_moves.contains(&Move {
            from: Amphipod::A(0, 0),
            to: Amphipod::A(2, 2),
            cost: 4
        }));
        let grotto = Grotto {
            active_amphis: vec![Amphipod::A(0, 7), Amphipod::D(2, 2)].into_iter().collect(),
            finished_amphis: vec![].into_iter().collect(),
            current_score: 0,
            map: &map,
            map_depth: 2,
        };
        let possible_moves = get_all_possible_moves(&grotto).into_iter().collect::<HashSet<_>>();
        assert!(possible_moves.contains(&Move {
            from: Amphipod::D(2, 2),
            to: Amphipod::D(0, 0),
            cost: 4000
        }));
        assert!(!possible_moves
            .into_iter()
            .map(|example_move| (example_move.from, example_move.to))
            .any(|amphis| amphis == (Amphipod::A(0, 7), Amphipod::A(2, 1))))
    }

    #[test]
    fn check_failing_case() {
        let map = grotto_map();
        let grotto = Grotto {
            active_amphis: vec![Amphipod::A(0, 0)].into_iter().collect(),
            finished_amphis: vec![Amphipod::A(2, 2)].into_iter().collect(),
            current_score: 0,
            map: &map,
            map_depth: 2,
        };
        let possible_moves = get_all_possible_moves(&grotto).into_iter().collect::<HashSet<_>>();
        assert_eq!(possible_moves.iter().count(), 1);
        assert!(possible_moves.contains(&Move {
            from: Amphipod::A(0, 0),
            to: Amphipod::A(1, 2),
            cost: 3
        }));
        assert!(!possible_moves.contains(&Move {
            from: Amphipod::A(0, 0),
            to: Amphipod::A(2, 2),
            cost: 4
        }));
    }

    #[test]
    fn check_in_final_location() {
        let map = grotto_map();
        let mut grotto = Grotto {
            active_amphis: vec![].into_iter().collect(),
            finished_amphis: vec![].into_iter().collect(),
            current_score: 0,
            map: &map,
            map_depth: 2,
        };
        grotto.active_amphis = vec![Amphipod::A(0, 0), Amphipod::D(0, 10)].into_iter().collect();
        assert!(Amphipod::A(2, 2).in_final_location(&grotto));
        assert!(!Amphipod::A(0, 0).in_final_location(&grotto));

        grotto.active_amphis = vec![Amphipod::A(1, 2), Amphipod::A(2, 2)].into_iter().collect();
        assert!(Amphipod::A(1, 2).in_final_location(&grotto));
        assert!(Amphipod::A(2, 2).in_final_location(&grotto));

        grotto.active_amphis = vec![Amphipod::A(1, 2), Amphipod::B(2, 2)].into_iter().collect();
        assert!(!Amphipod::A(1, 2).in_final_location(&grotto));
        assert!(!Amphipod::B(2, 2).in_final_location(&grotto));

        // Bigger Examples
        grotto.map_depth = 4;
        grotto.active_amphis = vec![
            Amphipod::A(1, 2),
            Amphipod::B(2, 2),
            Amphipod::A(3, 2),
            Amphipod::A(4, 2),
        ]
        .into_iter()
        .collect();
        assert!(!Amphipod::A(1, 2).in_final_location(&grotto));
        assert!(!Amphipod::B(2, 2).in_final_location(&grotto));
        assert!(Amphipod::A(3, 2).in_final_location(&grotto));
        assert!(Amphipod::A(4, 2).in_final_location(&grotto));
    }
}
