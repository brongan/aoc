use std::collections::HashMap;

use super::AOC2022;
use aoc_runner::{Day, ParseInput, Part, Solution};

use anyhow::Result;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::newline;
use nom::character::complete::space1;
use nom::combinator::map;
use nom::combinator::value;
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use nom::IResult;

#[derive(Clone, Debug)]
pub enum Instruction {
    Noop,
    Addx(i32),
}

impl Instruction {
    fn parse(input: &str) -> IResult<&str, Self> {
        alt((
            value(Instruction::Noop, tag("noop")),
            map(
                separated_pair(tag("addx"), space1, nom::character::complete::i32),
                |(_, operand)| Instruction::Addx(operand),
            ),
        ))(input)
    }
}

impl ParseInput<'_, { Day::Day10 }> for AOC2022<{ Day::Day10 }> {
    type Parsed = Vec<Instruction>;

    fn parse_input(&self, input: &'_ str) -> Result<Self::Parsed> {
        let (_, instructions) =
            separated_list1(newline, Instruction::parse)(input).map_err(|e| e.to_owned())?;
        Ok(instructions)
    }
}
impl Solution<'_, { Day::Day10 }, { Part::One }> for AOC2022<{ Day::Day10 }> {
    type Input = Vec<Instruction>;
    type Output = i32;

    fn solve(&self, input: &Self::Input) -> Result<Self::Output> {
        let mut cycle = 1;
        let mut register = 1;
        let mut signal_strengths = Vec::new();
        for instruction in input {
            if cycle == 20 || (cycle - 20) % 40 == 0 {
                signal_strengths.push(register * cycle);
            }
            cycle += 1;

            if let Instruction::Addx(operand) = instruction {
                if cycle == 20 || (cycle - 20) % 40 == 0 {
                    signal_strengths.push(register * cycle);
                }
                cycle += 1;
                register += operand;
            }
        }
        Ok(signal_strengths.iter().sum())
    }
}

fn should_light_pixel(cycle: usize, register: i32) -> bool {
    let sprite_center = (cycle - 1) % 40;
    return register + 1 == sprite_center as i32
        || register - 1 == sprite_center as i32
        || register == sprite_center as i32;
}

impl Solution<'_, { Day::Day10 }, { Part::Two }> for AOC2022<{ Day::Day10 }> {
    type Input = Vec<Instruction>;
    type Output = String;

    fn solve(&self, input: &Self::Input) -> Result<Self::Output> {
        let mut crt = HashMap::new();
        let mut cycle: usize = 1;
        let mut register: i32 = 1;
        for instruction in input {
            crt.insert(
                (cycle / 40, cycle % 40),
                should_light_pixel(cycle, register),
            );
            cycle += 1;

            if let Instruction::Addx(operand) = instruction {
                crt.insert(
                    (cycle / 40, cycle % 40),
                    should_light_pixel(cycle, register),
                );
                cycle += 1;
                register += operand;
            }
        }
        let mut ret = String::from("\n");
        for row in 0..6 {
            for col in 1..41 {
                if *crt.get(&(row, col)).unwrap_or(&false) {
                    ret.push('#');
                } else {
                    ret.push('.');
                }
            }
            ret.push('\n');
        }
        Ok(ret)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_runner::PartOneVerifier;

    #[test]
    fn test() -> Result<()> {
        let input = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";
        let problem = super::AOC2022::<{ Day::Day10 }>;
        problem.test_part1(input, 13140)
    }
}
