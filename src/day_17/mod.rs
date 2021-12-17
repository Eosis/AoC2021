type Input = ((i32, i32), (i32, i32));
pub fn solve_part_1() -> Result<(), ()> {
    println!("{}", part_one(((0, 0), (-89, -59))));
    Ok(())
}

pub fn solve_part_2() -> Result<(), ()> {
    let input = ((192, 251), (-89, -59));
    println!("{}", part_two(input));
    Ok(())
}

pub fn part_one(input: Input) -> usize {
    let ((_x_min, _x_max), (y_min, _y_max)) = input;
    ((y_min - 1) * y_min / 2) as usize
}

fn is_a_good_steppings((x_step, y_step): (i32, i32), input: Input) -> bool {
    let ((min_x, max_x), (min_y, max_y)) = input;
    let (mut x, mut y) = (0, 0);
    let (mut x_step, mut y_step) = (x_step, y_step);
    while x <= max_x && y >= min_y {
        x += x_step;
        y += y_step;
        x_step = if x_step <= 0 { 0 } else { x_step - 1 };
        y_step -= 1;
        if x >= min_x && y >= min_y && x <= max_x && y <= max_y {
            return true;
        }
    }
    false
}

fn part_two(input: Input) -> usize {
    let ((min_x, max_x), (min_y, _max_y)) = input;
    // Embarrassingly I do not remember the quadratic equation, and have no internet,
    // so I will brute it.
    let min_x_step = (0..)
        .find_map(|step| {
            if step * (step + 1) / 2 > min_x {
                Some(step)
            } else {
                None
            }
        })
        .unwrap();

    let max_x_step = max_x;
    let min_y_step = min_y;
    let max_y_step = min_y.abs();

    (min_x_step..=max_x_step)
        .map(move |x_step| (min_y_step..=max_y_step).map(move |y_step| is_a_good_steppings((x_step, y_step), input)))
        .flatten()
        .filter(|step_result| *step_result)
        .count() // Must be doubled for two values of y uppers and downers
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_two() {
        let input = ((20, 30), (-10, -5));
        assert_eq!(part_two(input), 112)
    }

    #[test]
    fn test_some_good_steppings_and_bad_steppings() {
        let input = ((20, 30), (-10, -5));
        assert!(is_a_good_steppings((23, -10), input));
        assert!(is_a_good_steppings((7, 0), input));
        assert!(is_a_good_steppings((7, 1), input));
        assert!(!is_a_good_steppings((20, 6), input));
    }
}
