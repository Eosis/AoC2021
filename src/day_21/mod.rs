use std::fs::read_to_string;
use bitvec::prelude::*;
use std::path::Path;
use hashbrown::HashMap;
use itertools::Itertools;

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub struct Game {
    player_one: usize,
    player_one_score: usize,
    player_two: usize,
    player_two_score: usize,
    target: usize,
    player_turn: usize
}
type Input = Game;

pub fn solve_part_1() -> Result<(), ()> {
    println!("Solution: {}", part_one(Game {
        player_one: 6,
        player_two: 8,
        player_one_score: 0,
        player_two_score: 0,
        target: 1000,
        player_turn: 0
    }));
    Ok(())
}

pub fn solve_part_2() -> Result<(), ()> {
    println!("Solution: {}", part_two(Game {
        player_one: 6,
        player_two: 8,
        player_one_score: 0,
        player_two_score: 0,
        target: 21,
        player_turn: 0
    }));
    Ok(())
}

fn parse_from_file<T: AsRef<Path>>(filename: T) -> Input {
    let input = read_to_string(filename).unwrap();
    parse_from_str(&input)
}

fn parse_from_str(input: &str) -> Input {
    unimplemented!();
}

fn iterate_game(mut game: Game) -> (usize, usize) {
    let mut rolls = 0;
    let mut dice_iter = (1usize..=100usize)
        .cycle();
    let Game {
        mut player_one,
        mut player_two,
        mut player_one_score,
        mut player_two_score,
        mut target,
        ..
    } = game;

    while player_one_score < target && player_two_score < target {
        let (current_player, current_score) = if rolls % 2 == 0 {
            (&mut player_one, &mut player_one_score)
        } else {
            (&mut player_two, &mut player_two_score)
        };
        let new_roll: usize = dice_iter.by_ref().take(3).sum();
        rolls += 1;
        *current_player += new_roll;
        let scored = if *current_player % 10 == 0 { 10 } else { *current_player % 10 };
        *current_score += scored;
        if *current_player > 10 { *current_player %= 10 };
    }

    let losing_score = if player_one_score >= target {
        player_two_score
    } else {
        player_one_score
    };
    (rolls * 3, losing_score)
}

fn iterate_game_once(mut game: Game, new_rolls: (usize, usize, usize)) -> Game {
    let (current_player, current_score) = if game.player_turn == 0 {
        (&mut game.player_one, &mut game.player_one_score)
    } else {
        (&mut game.player_two, &mut game.player_two_score)
    };
    *current_player += (new_rolls.0 + new_rolls.1 + new_rolls.2);
    let scored = if *current_player % 10 == 0 { 10 } else { *current_player % 10 };
    *current_score += scored;
    if *current_player > 10 { *current_player %= 10 };
    game.player_turn += 1;
    game.player_turn %= 2;
    game
}

pub fn part_one(input: Input) -> usize {
    let (rolls, losing_score) = iterate_game(input);
    rolls * losing_score
}

fn count_winning_outcomes(game: Game,
                          rolls: (usize, usize, usize),
                          cache: &mut HashMap<(Game, (usize, usize, usize)), (usize, usize)>)
    -> (usize, usize) {
    if cache.get(&(game.clone(), rolls)).is_some() {
        return cache.get(&(game.clone(), rolls)).unwrap().clone();
    }

    let next_game = iterate_game_once(game.clone(), rolls);

    // Determine if there is a winner already (this is the base case)
    if next_game.player_one_score >= next_game.target {
        cache.insert((game, rolls), (1, 0));
        return (1, 0);
    }

    if next_game.player_two_score >= next_game.target {
        cache.insert((game, rolls), (0, 1));
        return (0, 1);
    }

    let results: Vec<_> = base_three_tuples()
        .map(|rolls| {
            count_winning_outcomes(next_game.clone(), rolls, cache)
        }).collect();
    let player_one_wins = results.iter().map(|(one_wins, _)| one_wins).sum();
    let player_two_wins = results.iter().map(|(_, two_wins)| two_wins).sum();
    cache.insert((game, rolls), (player_one_wins, player_two_wins));
    (player_one_wins, player_two_wins)
}


pub fn part_two(game: Input) -> usize {
    let mut cache: HashMap<(Game, (usize, usize, usize)), (usize, usize)> = HashMap::new();

    let results: Vec<_> = base_three_tuples()
        .map(|rolls| {
            count_winning_outcomes(game.clone(), rolls, &mut cache)
        }).collect();
    let player_one_wins = results.iter().map(|(one_wins, _)| one_wins).sum();
    let player_two_wins = results.iter().map(|(_, two_wins)| two_wins).sum();
    if player_one_wins > player_two_wins {
        player_one_wins
    } else {
        player_two_wins
    }
}

fn base_three_tuples() -> impl Iterator<Item=(usize, usize, usize)> {
    (0usize..((3 * 3 * 3) as usize)).map( |i|
        (
            (i / (3 * 3)) % 3 + 1,
            (i / 3 ) % 3 + 1,
            i % 3 + 1,
        )
    )
}

#[cfg(test)]
mod tests {
    use super::*;
        #[test]
    fn test_part_one() {
        let game = Game {
            player_one: 4,
            player_two: 8,
            player_one_score: 0,
            player_two_score: 0,
            target: 1000,
            player_turn: 0,
        };
        assert_eq!(part_one(game), 739785)
    }

    #[test]
    fn test_part_two() {
        let base_game = Game {
            player_one: 4,
            player_two: 8,
            player_one_score: 0,
            player_two_score: 0,
            player_turn: 0,
            target: 21,
        };
        assert_eq!(part_two(base_game), 444356092776315)
    }

    #[test]
    #[ignore]
    fn test_combos() {
        let x: Vec<_> = base_three_tuples().collect();
        println!("{:?}", x);
        assert_eq!(x.len(), 27)
    }
}
