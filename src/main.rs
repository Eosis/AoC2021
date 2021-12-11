#![feature(int_abs_diff)]
mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;
mod day_8;
mod day_9;
mod day_10;
mod day_11;

use argh::FromArgs;

#[derive(FromArgs)]
/// Rup's Advent of Code 2021
struct Args {
    /// day that we are doing the puzzle for.
    #[argh(positional)]
    day: usize,

    #[argh(positional)]
    /// part of the puzzle to do.
    part: usize,
}

use anyhow::Result;
fn main() -> Result<()> {
    let args: Args = argh::from_env();
    const FAILURE_TEXT: &str = "Failed to find the answer";
    match (args.day, args.part) {
        (1, 1) => day_1::solve_part_1().expect(FAILURE_TEXT),
        (1, 2) => day_1::solve_part_2().expect(FAILURE_TEXT),
        (2, 1) => day_2::solve_part_1().expect(FAILURE_TEXT),
        (2, 2) => day_2::solve_part_2().expect(FAILURE_TEXT),
        (3, 1) => day_3::solve_part_1().expect(FAILURE_TEXT),
        (3, 2) => day_3::solve_part_2().expect(FAILURE_TEXT),
        (4, 1) => day_4::solve_part_1().expect(FAILURE_TEXT),
        (4, 2) => day_4::solve_part_2().expect(FAILURE_TEXT),
        (5, 1) => day_5::solve_part_1().expect(FAILURE_TEXT),
        (5, 2) => day_5::solve_part_2().expect(FAILURE_TEXT),
        (6, 1) => day_6::solve_part_1().expect(FAILURE_TEXT),
        (6, 2) => day_6::solve_part_2().expect(FAILURE_TEXT),
        (7, 1) => day_7::solve_part_1().expect(FAILURE_TEXT),
        (7, 2) => day_7::solve_part_2().expect(FAILURE_TEXT),
        (8, 1) => day_8::solve_part_1().expect(FAILURE_TEXT),
        (8, 2) => day_8::solve_part_2().expect(FAILURE_TEXT),
        (9, 1) => day_9::solve_part_1().expect(FAILURE_TEXT),
        (9, 2) => day_9::solve_part_2().expect(FAILURE_TEXT),
        (10, 1) => day_10::solve_part_1().expect(FAILURE_TEXT),
        (10, 2) => day_10::solve_part_2().expect(FAILURE_TEXT),
        (11, 1) => day_11::solve_part_1().expect(FAILURE_TEXT),
        (11, 2) => day_11::solve_part_2().expect(FAILURE_TEXT),
        (_, _) => unimplemented!("This day no work yet, brah."),
    };
    Ok(())
}
