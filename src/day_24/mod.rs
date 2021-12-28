
use std::fmt::{Display, Formatter};
use std::fs::read_to_string;
use std::path::Path;

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug, Clone, Copy)]
enum Variable {
    W,
    X,
    Y,
    Z,
}

impl From<&str> for Variable {
    fn from(input: &str) -> Self {
        match input {
            "w" => Variable::W,
            "x" => Variable::X,
            "y" => Variable::Y,
            "z" => Variable::Z,
            _ => panic!("You wot?!"),
        }
    }
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug, Clone, Copy)]
enum Rhs {
    Literal(i64),
    Variable(Variable),
}

impl From<&str> for Rhs {
    fn from(input: &str) -> Self {
        match input {
            "w" | "x" | "y" | "z" => Rhs::Variable(input.into()),
            _ => Rhs::Literal(input.parse().unwrap()),
        }
    }
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug, Clone, Copy)]
enum Instruction {
    Inp(Variable),
    Add(Variable, Rhs),
    Mul(Variable, Rhs),
    Div(Variable, Rhs),
    Mod(Variable, Rhs),
    Eql(Variable, Rhs),
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug, Clone, Copy)]
struct RupInstruction {
    x_number: i64,
    y_number: i64,
    divide: bool,
}

impl Display for RupInstruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{x: {}, y: {}, div: {}}}",
            self.x_number, self.y_number, self.divide
        )
    }
}

impl From<&str> for Instruction {
    fn from(input: &str) -> Self {
        let mut line_parts = input.split_whitespace();
        let instruction = line_parts.next().unwrap();
        let variable: Variable = line_parts.next().map(|n| n.into()).unwrap();
        let rhs: Option<Rhs> = line_parts.next().map(|n| n.into());
        match instruction {
            "inp" => Instruction::Inp(variable),
            "add" => Instruction::Add(variable, rhs.unwrap()),
            "mul" => Instruction::Mul(variable, rhs.unwrap()),
            "div" => Instruction::Div(variable, rhs.unwrap()),
            "mod" => Instruction::Mod(variable, rhs.unwrap()),
            "eql" => Instruction::Eql(variable, rhs.unwrap()),
            _ => panic!("You wot?! {}", input),
        }
    }
}

struct Computer {
    w: i64,
    x: i64,
    y: i64,
    z: i64,
}

impl Display for Computer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{w: {}, x: {}, y: {}, z: {}}}", self.w, self.x, self.y, self.z)
    }
}

#[allow(dead_code)]
impl Computer {
    fn new() -> Self {
        Computer { w: 0, x: 0, y: 0, z: 0 }
    }

    fn run_normal(&mut self, instructions: &[Instruction], input: Vec<i64>) -> Option<i64> {
        let mut input = input.into_iter();
        for (_i, instruction) in instructions.iter().enumerate() {
            self.run_normal_instruction(*instruction, &mut input).ok()?
        }
        Some(self.z)
    }

    fn deconstruct_rhs(&self, rhs: Rhs) -> i64 {
        match rhs {
            Rhs::Literal(n) => n,
            Rhs::Variable(Variable::W) => self.w,
            Rhs::Variable(Variable::X) => self.x,
            Rhs::Variable(Variable::Y) => self.y,
            Rhs::Variable(Variable::Z) => self.z,
        }
    }

    fn lhs_reference(&mut self, variable: Variable) -> &mut i64 {
        match variable {
            Variable::W => &mut self.w,
            Variable::X => &mut self.x,
            Variable::Y => &mut self.y,
            Variable::Z => &mut self.z,
        }
    }

    fn inp_instruction<T>(&mut self, variable: Variable, mut input: T) -> Result<(), ()>
    where
        T: Iterator<Item = i64>,
    {
        if variable != Variable::W {
            panic!("Got an input that didn't go to w")
        }
        let lhs = self.lhs_reference(variable);
        *lhs = input.next().unwrap();
        Ok(())
    }

    fn add_instruction(&mut self, variable: Variable, rhs: Rhs) -> Result<(), ()> {
        let rhs = self.deconstruct_rhs(rhs);
        let lhs = self.lhs_reference(variable);
        *lhs += rhs;
        Ok(())
    }

    fn mul_instruction(&mut self, variable: Variable, rhs: Rhs) -> Result<(), ()> {
        let rhs = self.deconstruct_rhs(rhs);
        let lhs = self.lhs_reference(variable);
        *lhs *= rhs;
        Ok(())
    }

    fn div_instruction(&mut self, variable: Variable, rhs: Rhs) -> Result<(), ()> {
        let rhs = self.deconstruct_rhs(rhs);
        let lhs = self.lhs_reference(variable);
        if rhs < 0 {
            return Err(());
        }
        *lhs /= rhs;
        Ok(())
    }

    fn mod_instruction(&mut self, variable: Variable, rhs: Rhs) -> Result<(), ()> {
        let rhs = self.deconstruct_rhs(rhs);
        let lhs = self.lhs_reference(variable);
        if rhs < 0 {
            return Err(());
        }
        *lhs %= rhs;
        Ok(())
    }

    fn eql_instruction(&mut self, variable: Variable, rhs: Rhs) -> Result<(), ()> {
        let rhs = self.deconstruct_rhs(rhs);
        let lhs = self.lhs_reference(variable);
        if *lhs == rhs {
            *lhs = 1;
        } else {
            *lhs = 0;
        }
        Ok(())
    }

    fn run_normal_instruction<T>(&mut self, instruction: Instruction, input: T) -> Result<(), ()>
    where
        T: Iterator<Item = i64>,
    {
        match instruction {
            Instruction::Inp(variable) => self.inp_instruction(variable, input),
            Instruction::Add(variable, rhs) => self.add_instruction(variable, rhs),
            Instruction::Mul(variable, rhs) => self.mul_instruction(variable, rhs),
            Instruction::Div(variable, rhs) => self.div_instruction(variable, rhs),
            Instruction::Mod(variable, rhs) => self.mod_instruction(variable, rhs),
            Instruction::Eql(variable, rhs) => self.eql_instruction(variable, rhs),
        }
    }

    fn run_rups(&mut self, instructions: &[RupInstruction], input: Vec<i64>) -> Option<i64> {
        let mut input = input.into_iter();
        for (_i, instruction) in instructions.iter().enumerate() {
            self.run_rup_instruction(*instruction, &mut input).ok()?
        }
        Some(self.z)
    }

    fn run_rup_instruction<T>(&mut self, instruction: RupInstruction, mut input: T) -> Result<(), ()>
    where
        T: Iterator<Item = i64>,
    {
        let RupInstruction {
            x_number,
            y_number,
            divide,
        } = instruction;
        if self.z < 0 {
            println!("Z was less than 0...");
            return Err(());
        }
        self.x = (self.z % 26) + x_number;
        if divide {
            self.z /= 26;
        }
        self.w = input.next().unwrap();
        if self.x != self.w {
            self.z *= 26;
            self.z += y_number + self.w;
        }
        Ok(())
    }
}

type Input = Vec<Instruction>;
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
    input.lines().map(|line| line.into()).collect()
}

#[allow(dead_code)]
fn parse_from_str_to_rups(input: &str) -> Vec<RupInstruction> {
    input
        .lines()
        .collect::<Vec<_>>()
        .chunks(18)
        .map(|chunk| -> RupInstruction {
            let div_ins = chunk[4];
            let x_ins = chunk[5];
            let y_ins = chunk[15];
            let divide: bool = match Instruction::from(div_ins) {
                Instruction::Div(Variable::Z, Rhs::Literal(26)) => true,
                Instruction::Div(Variable::Z, Rhs::Literal(1)) => false,
                _ => panic!("You wot? {:?}", Instruction::from(div_ins)),
            };
            let x_number = if let Instruction::Add(Variable::X, Rhs::Literal(n)) = Instruction::from(x_ins) {
                n
            } else {
                panic!("You wot? {:?}", Instruction::from(x_ins))
            };
            let y_number = if let Instruction::Add(Variable::Y, Rhs::Literal(n)) = Instruction::from(y_ins) {
                n
            } else {
                panic!("You wot? {:?}", Instruction::from(y_ins))
            };

            RupInstruction {
                x_number,
                y_number,
                divide,
            }
        })
        .collect()
}

fn part_one(_: Input) -> usize {
    unimplemented!()
}

fn part_two(_: Input) -> usize {
    unimplemented!();
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;
    const EXAMPLE_PROGRAM: &str = include_str!("../../inputs/day24.txt");

    #[test]
    fn test_part_one() {
        unimplemented!();
    }

    #[test]
    fn test_part_two() {
        unimplemented!();
    }

    #[test]
    fn test_rups_analysis_correct() {
        let full_instructions = parse_from_str(EXAMPLE_PROGRAM);
        let rup_instructions = parse_from_str_to_rups(EXAMPLE_PROGRAM);
        let mut rng = rand::thread_rng();
        for _ in 0..5 {
            let input: Vec<i64> = (0..14).map(|_| rng.gen_range(1..=9)).collect();
            let mut normal_computer = Computer::new();
            let mut rup_computer = Computer::new();
            println!("Trying out {:?}", input);
            assert_eq!(
                normal_computer.run_normal(&full_instructions, input.clone()),
                rup_computer.run_rups(&rup_instructions, input)
            )
        }
    }

    #[test]
    fn test_failing_case() {
        let full_instructions = parse_from_str(EXAMPLE_PROGRAM);
        let rup_instructions = parse_from_str_to_rups(EXAMPLE_PROGRAM);
        for instruction in &rup_instructions {
            println!("{}", instruction);
        }
        let input: Vec<i64> = vec![1, 9, 7, 6, 9, 2, 4, 1, 3, 3, 6, 1, 4, 4];
        let mut normal_computer = Computer::new();
        let mut rup_computer = Computer::new();
        println!("Trying out {:?}", input);
        assert_eq!(
            normal_computer.run_normal(&full_instructions, input.clone()),
            rup_computer.run_rups(&rup_instructions, input)
        )
    }

    #[test]
    fn test_some_assumptions() {
        let rup_instructions = parse_from_str_to_rups(EXAMPLE_PROGRAM);
        let input = vec![1, 3, 1, 6, 1, 1, 5, 1, 1, 3, 9, 6, 1, 7];
        let mut rup_computer = Computer::new();
        assert_eq!(rup_computer.run_rups(&rup_instructions, input), Some(0))
    }
}
