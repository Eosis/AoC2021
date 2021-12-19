use std::borrow::BorrowMut;
use std::fmt::{Display, Formatter};
use std::fs;
use std::fs::read_to_string;
use std::ops::Add;

use bitvec::prelude::*;
use std::path::Path;
use itertools::Itertools;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum FishyNumber {
    Single(usize),
    Pair { left: Box<FishyNumber>, right: Box<FishyNumber> }
}

struct FishyNumberArena {
    fishy_numbers: Vec<FishyNumber>
}

impl Add for FishyNumber {
    type Output = FishyNumber;
    fn add(self, rhs: Self) -> Self {
        FishyNumber::Pair {
            left: Box::new(self),
            right: Box::new(rhs),
        }
    }
}

impl FishyNumber {
    fn magnitude(&self) -> usize {
        match self {
            FishyNumber::Pair {
                left,
                right
            } => 3 * left.magnitude() + 2 * right.magnitude(),
            FishyNumber::Single(n) => *n,
        }
    }
}
type Input = Vec<FishyNumber>;
pub fn solve_part_1() -> Result<(), ()> {
    let input = parse_from_file("./inputs/day18.txt");
    println!("Solution: {}", part_one(input));
    Ok(())
}

pub fn solve_part_2() -> Result<(), ()> {
    let input = parse_from_file("./inputs/day18.txt");
    println!("Solution: {}", part_two(input));
    Ok(())
}

fn parse_from_file<T: AsRef<Path>>(filename: T) -> Input {
    let input = read_to_string(filename).unwrap();
    parse_list_of_fishy_numbers(&input)
}

pub fn part_one(input: Input) -> usize {
    let reduced = reduce_list(input);
    reduced.magnitude()
}

pub fn part_two(input: Input) -> usize {
    input
        .into_iter()
        .permutations(2)
        .map(|perm| reduce_fully(perm[0].clone() + perm[1].clone()).magnitude() )
        .max()
        .unwrap()
}

pub trait PeekableIterator : std::iter::Iterator {
    fn peek(&mut self) -> Option<&Self::Item>;
}

impl<I: std::iter::Iterator> PeekableIterator for std::iter::Peekable<I> {
    fn peek(&mut self) -> Option<&Self::Item> {
        std::iter::Peekable::peek(self)
    }
}

fn parse_fishy_number<T: Iterator<Item=char>  + itertools::PeekingNext + PeekableIterator>(
    chars_iter: &mut T) -> FishyNumber {
    let next = chars_iter.peek().unwrap();
    match next {
        '[' => {
            let _ = chars_iter.next(); // Nom nom [
            let left = Box::new(parse_fishy_number(chars_iter));
            let _comma = chars_iter.next().unwrap();
            let right = Box::new(parse_fishy_number(chars_iter));
            let _closer = chars_iter.next().unwrap();
            FishyNumber::Pair { left, right }
        },
        '0'..='9' => {
            let digits: String = chars_iter
                .peeking_take_while(|c| matches!(c, '0'..='9'))
                .collect();
            FishyNumber::Single(digits.parse().unwrap())
        },
        _ => panic!("You wat? {}", next)
    }

}

struct ReductionAction {
    left_to_add: Option<usize>,
    right_to_add: Option<usize>
}

fn add_to_leftmost( fishy_number: FishyNumber, to_add: usize) -> FishyNumber {
    match fishy_number {
        FishyNumber::Single(n) => FishyNumber::Single(n + to_add),
        FishyNumber::Pair { left, right } => FishyNumber::Pair {
            left: Box::new(add_to_leftmost(*left, to_add)),
            right,
        }
    }
}

fn add_to_rightmost(fishy_number: FishyNumber, to_add: usize) -> FishyNumber {
    match fishy_number {
        FishyNumber::Single(n) => FishyNumber::Single(n + to_add),
        FishyNumber::Pair { left, right } => FishyNumber::Pair {
            left,
            right: Box::new(add_to_rightmost(*right, to_add)),
        }
    }
}

fn perform_reduction(fishy_number: FishyNumber) -> (FishyNumber, bool) {
    if let Some(number) = explode_leftmost_pair(fishy_number.clone()) {
        return (number, true)
    }

    if let Some(number) = split_regular_number(fishy_number.clone()) {
        return (number, true)
    }
    return (fishy_number, false)
}

fn explode_leftmost_pair(fishy_number: FishyNumber) -> Option<FishyNumber> {
    let mut reduction_happened = false;
    let result = make_an_explody_fishy_reduction(fishy_number, 0, &mut reduction_happened);
    if reduction_happened {
        Some(result.0)
    } else {
        None
    }
}


fn split_regular_number(fishy_number: FishyNumber) -> Option<FishyNumber> {
    let mut reduction_happened = false;
    let result = make_a_splity_fishy_reduction(fishy_number, &mut reduction_happened);
    if reduction_happened {
        Some(result)
    } else {
        None
    }
}

fn make_a_splity_fishy_reduction(fishy_number: FishyNumber, reduction_happened: &mut bool)
    -> FishyNumber {
    if *reduction_happened {
        return fishy_number;
    }

    match fishy_number {
        FishyNumber::Single(n) if n >= 10 => {
            let left = Box::new(FishyNumber::Single(n / 2));
            let right = Box::new(FishyNumber::Single(n / 2 + (n % 2)));
            *reduction_happened = true;
            FishyNumber::Pair { left, right}
        },
        FishyNumber::Single(n) => fishy_number,
        FishyNumber::Pair {left, right} => {
            let resulting_left= Box::new(make_a_splity_fishy_reduction(*left, reduction_happened));
            let resulting_right= Box::new(make_a_splity_fishy_reduction(*right, reduction_happened));
            FishyNumber::Pair {left: resulting_left, right: resulting_right}
        }
    }
}


fn make_an_explody_fishy_reduction(fishy_number: FishyNumber, nesting_level: usize, reduction_happened: &mut bool)
    -> (FishyNumber, Option<usize>, Option<usize>) {
    if *reduction_happened {
        return (fishy_number, None, None);
    }

    match fishy_number {
        FishyNumber::Single(n) => (fishy_number, None, None),
        FishyNumber::Pair {left, right} if nesting_level >= 4 => {
            *reduction_happened = true;
            (
                FishyNumber::Single(0),
                Some(destruct_fishy_or_panic(*left)),
                Some(destruct_fishy_or_panic(*right)),
            )
        },
        FishyNumber::Pair {left, right} => {
            let (
                resulting_left,
                left_to_add,
                right_to_add
            ) = make_an_explody_fishy_reduction(*left, nesting_level + 1, reduction_happened);
            match (left_to_add, right_to_add) {
                (Some(left_to_add), Some(right_to_add)) => {
                    return (
                        FishyNumber::Pair {
                            left: Box::new(resulting_left),
                            right: Box::new(add_to_leftmost(*right, right_to_add)),
                        },
                        Some(left_to_add),
                        None
                    );
                },
                (Some(left_to_add), None) => {
                    return (
                        FishyNumber::Pair {
                            left: Box::new(resulting_left),
                            right: Box::new(*right),
                        },
                        Some(left_to_add),
                        None
                    );
                }
                (None, Some(right_to_add)) => {
                    return (
                        FishyNumber::Pair {
                            left: Box::new(resulting_left),
                            right: Box::new(add_to_leftmost(*right, right_to_add)),
                        },
                        None,
                        None
                    );
                },
                (None, None) => ()
            }
            let (
                resulting_right,
                left_to_add,
                right_to_add
            ) = make_an_explody_fishy_reduction(*right, nesting_level + 1, reduction_happened);
            match (left_to_add, right_to_add) {
                (Some(left_to_add), Some(right_to_add)) => {
                    return (
                        FishyNumber::Pair {
                            left: Box::new(add_to_rightmost(resulting_left, left_to_add)),
                            right: Box::new(resulting_right)
                        },
                        None,
                        Some(right_to_add)
                    )
                },
                (None, Some(right_to_add)) => {
                    return (
                        FishyNumber::Pair {
                            left: Box::new(resulting_left),
                            right: Box::new(resulting_right),
                        },
                        None,
                        Some(right_to_add),
                    )
                }
                (Some(left_to_add), None) => {
                    return (
                        FishyNumber::Pair {
                            left: Box::new(add_to_rightmost(resulting_left, left_to_add)),
                            right: Box::new(resulting_right),
                        },
                        None,
                        None
                    )
                },
                (None, None) => {
                    return (
                        FishyNumber::Pair {
                            left: Box::new(resulting_left), right: Box::new(resulting_right),
                        },
                        None,
                        None,
                    )
                }
            }
        }
    }
}

fn destruct_fishy_or_panic(fishy_number: FishyNumber) -> usize {
    match fishy_number {
        FishyNumber::Single(n) => n,
        _ => panic!("You wot m8?!"),
    }
}

fn reduce_fully(mut fishy_number: FishyNumber) -> FishyNumber {
    loop {
        let (resulting_number, should_continue) = perform_reduction(fishy_number);
        fishy_number = resulting_number;
        if !should_continue {
            break fishy_number;
        }
    }
}

fn fishy_number_from_str(input: &str) -> FishyNumber {
    parse_fishy_number(&mut input.chars().peekable())
}

impl From<&str> for FishyNumber {
    fn from(value: &str) -> Self {
        parse_fishy_number(&mut value.chars().peekable())
    }
}

impl Display for FishyNumber {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            FishyNumber::Single(n) => write!(f, "{}", n),
            FishyNumber::Pair { left, right } => write!(f, "[{},{}]", left, right),
        }
    }
}

fn parse_list_of_fishy_numbers(input: &str) -> Vec<FishyNumber> {
    input
        .lines()
        .map(|line| line.into())
        .collect()
}

fn reduce_list(input: Vec<FishyNumber>) -> FishyNumber {
    let result: FishyNumber = input
        .into_iter()
        .reduce(|acc: FishyNumber, new: FishyNumber| reduce_fully(acc + new))
        .unwrap();
    reduce_fully(result)
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;
    use super::*;

    #[test]
    fn test_parsing() {
        const EXAMPLES: [&str; 4] = [
            "[1,2]",
            "[[1,2],3]",
            "[9,[8,7]]",
            "[[1,9],[8,5]]",
        ];
        dbg!(parse_fishy_number(&mut EXAMPLES[0].chars().peekable()));
        dbg!(parse_fishy_number(&mut EXAMPLES[1].chars().peekable()));
        dbg!(parse_fishy_number(&mut EXAMPLES[2].chars().peekable()));
        dbg!(parse_fishy_number(&mut EXAMPLES[3].chars().peekable()));
    }

    #[test]
    fn test_addition() {
        let lhs: FishyNumber = "[1,2]".into();
        let rhs = "[[3,4],5]".into();
        let result = "[[1,2],[[3,4],5]]".into();
        assert_eq!(lhs + rhs, result)
    }

    #[test]
    fn test_reduction_by_explosion() {
        let input = parse_fishy_number(&mut "[[[[[9,8],1],2],3],4]".chars().peekable());
        let output = parse_fishy_number(&mut "[[[[0,9],2],3],4]".chars().peekable());
        assert_eq!(perform_reduction(input).0, output);
        let input = parse_fishy_number(&mut "[7,[6,[5,[4,[3,2]]]]]".chars().peekable());
        let output = parse_fishy_number(&mut "[7,[6,[5,[7,0]]]]".chars().peekable());
        assert_eq!(perform_reduction(input).0, output);
        let input = parse_fishy_number(&mut "[[6,[5,[4,[3,2]]]],1]".chars().peekable());
        let output = parse_fishy_number(&mut "[[6,[5,[7,0]]],3]".chars().peekable());
        assert_eq!(perform_reduction(input).0, output);
        let input = parse_fishy_number(&mut "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]".chars().peekable());
        let output = parse_fishy_number(&mut "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]".chars().peekable());
        assert_eq!(perform_reduction(input).0, output);
        let input = parse_fishy_number(&mut "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]".chars().peekable());
        let output = parse_fishy_number(&mut "[[3,[2,[8,0]]],[9,[5,[7,0]]]]".chars().peekable());
        assert_eq!(perform_reduction(input).0, output);
    }

    #[test]
    fn test_exploding_and_splitting_example() {
        let lhs: FishyNumber = "[[[[4,3],4],4],[7,[[8,4],9]]]".into();
        let rhs: FishyNumber = "[1,1]".into();
        let result = lhs + rhs;
        assert_eq!(result, "[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]".into());
        let result = perform_reduction(result).0;
        assert_eq!(result, "[[[[0,7],4],[7,[[8,4],9]]],[1,1]]".into());
        let result = perform_reduction(result).0;
        assert_eq!(result, "[[[[0,7],4],[15,[0,13]]],[1,1]]".into());
        let result = perform_reduction(result).0;
        assert_eq!(result, "[[[[0,7],4],[[7,8],[0,13]]],[1,1]]".into());
        let result= perform_reduction(result).0;
        assert_eq!(result, "[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]".into());
        let result = perform_reduction(result).0;
        assert_eq!(result, "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]".into());
    }

    #[test]
    fn test_failing_case() {
        let input = "[[[[4,0],[5,0]],[[[4,5],[2,6]],[9,5]]],[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]]".into();
        let result = perform_reduction(input).0;
        println!("{}", result);
        assert_eq!(result, "[[[[4,0],[5,4]],[[0,[7,6]],[9,5]]],[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]]".into());
    }

    #[test]
    fn test_reduce_fully() {
        let lhs: FishyNumber = "[[[[4,3],4],4],[7,[[8,4],9]]]".into();
        let rhs: FishyNumber = "[1,1]".into();
        let output = "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]".into();
        assert_eq!(reduce_fully(lhs + rhs), output)
    }

    #[test]
    fn test_longer_example() {
        let lhs: FishyNumber = "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]".into();
        let rhs: FishyNumber = "[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]".into();
        println!("{}", lhs.clone() + rhs.clone());
        println!("{}", reduce_fully(lhs + rhs))
    }

    #[test]
    fn test_summing_lists() {
        let four_list = include_str!("../../test_inputs/day18/four_list.txt");
        let four_list = parse_list_of_fishy_numbers(four_list);
        let result = reduce_list(four_list);
        assert_eq!(result, "[[[[1,1],[2,2]],[3,3]],[4,4]]".into());
        let five_list = include_str!("../../test_inputs/day18/five_list.txt");
        let five_list = parse_list_of_fishy_numbers(five_list);
        let result = reduce_list(five_list);
        assert_eq!(result, "[[[[3,0],[5,3]],[4,4]],[5,5]]".into());
        let six_list = include_str!("../../test_inputs/day18/six_list.txt");
        let six_list = parse_list_of_fishy_numbers(six_list);
        let result = reduce_list(six_list);
        dbg!(&result);
        assert_eq!(result, "[[[[5,0],[7,4]],[5,5]],[6,6]]".into());
    }

    #[test]
    fn test_long_addition_example() {
        let input = include_str!("../../test_inputs/day18/long_addition_example.txt");
        let long_list = parse_list_of_fishy_numbers(input);
        let result = reduce_list(long_list);
        assert_eq!(result, "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]".into());
    }

    #[test]
    fn test_individual_longer_parts() {
        let lhs: FishyNumber = "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]".into();
        let rhs = "[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]".into();

        assert_eq!(reduce_fully(lhs + rhs), "[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]".into());

        let lhs: FishyNumber = "[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]".into();
        let rhs = "[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]".into();
        assert_eq!(reduce_fully(lhs + rhs), "[[[[6,7],[6,7]],[[7,7],[0,7]]],[[[8,7],[7,7]],[[8,8],[8,0]]]]".into());

        let lhs: FishyNumber = "[[[[6,7],[6,7]],[[7,7],[0,7]]],[[[8,7],[7,7]],[[8,8],[8,0]]]]".into();
        let rhs = "[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]".into();
        assert_eq!(reduce_fully(lhs + rhs), "[[[[7,0],[7,7]],[[7,7],[7,8]]],[[[7,7],[8,8]],[[7,7],[8,7]]]]".into());

        let lhs: FishyNumber = "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]".into();
        let rhs = "[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]".into();
        assert_eq!(reduce_fully(lhs + rhs), "[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]".into());

        let lhs: FishyNumber = "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]".into();
        let rhs = "[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]".into();
        assert_eq!(reduce_fully(lhs + rhs), "[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]".into());

        let lhs: FishyNumber = "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]".into();
        let rhs = "[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]".into();
        assert_eq!(reduce_fully(lhs + rhs), "[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]".into());

    }

    #[test]
    fn test_magnitudes() {
        let number: FishyNumber = "[[1,2],[[3,4],5]]".into();
        assert_eq!(number.magnitude(), 143);
        let number: FishyNumber = "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]".into();
        assert_eq!(number.magnitude(), 3488);
    }

    #[test]
    fn test_part_one() {
        let input = include_str!("../../test_inputs/day18.txt");
        let input = parse_list_of_fishy_numbers(input);
        assert_eq!(part_one(input), 4140);
    }

    #[test]
    fn test_part_two() {
        let input = include_str!("../../test_inputs/day18.txt");
        let input = parse_list_of_fishy_numbers(input);
        assert_eq!(part_two(input), 3993);
    }
}
