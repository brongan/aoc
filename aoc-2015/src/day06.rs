use super::AOC2015;
use aoc_runner::point2d::Point2D;
use aoc_runner::{Day, ParseInput, Part, Solution};
use nom::sequence::delimited;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{newline, space0},
    combinator::{map, map_res, value},
    multi::separated_list1,
    sequence::{preceded, tuple},
    IResult, InputTakeAtPosition,
};
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use strum_macros::Display;

use anyhow::Result;

#[derive(Clone, Debug, PartialEq, Display)]
pub enum InstructionType {
    On,
    Off,
    Toggle,
}

impl InstructionType {
    fn parse(input: &str) -> IResult<&str, Self> {
        alt((
            value(InstructionType::On, tag("turn on")),
            value(InstructionType::Off, tag("turn off")),
            value(InstructionType::Toggle, tag("toggle")),
        ))(input)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Instruction {
    instruction_type: InstructionType,
    start: Point2D<usize>,
    end: Point2D<usize>,
}

impl Display for Instruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} through {}",
            self.instruction_type, self.start, self.end
        )
    }
}

fn non_space(input: &str) -> IResult<&str, &str> {
    input.split_at_position_complete(char::is_whitespace)
}

impl Instruction {
    fn parse(input: &str) -> IResult<&str, Self> {
        map(
            tuple((
                InstructionType::parse,
                preceded(space0, map_res(non_space, Point2D::from_str)),
                preceded(
                    delimited(space0, tag("through"), space0),
                    map_res(non_space, Point2D::from_str),
                ),
            )),
            |(instruction_type, start, end)| Instruction {
                instruction_type,
                start,
                end,
            },
        )(input)
    }
}

impl ParseInput<'_, { Day::Day6 }> for AOC2015<{ Day::Day6 }> {
    type Parsed = Vec<Instruction>;

    fn parse_input(&self, input: &'_ str) -> Result<Self::Parsed> {
        let (_, instructions) =
            separated_list1(newline, Instruction::parse)(input).map_err(|e| e.to_owned())?;
        Ok(instructions)
    }
}

impl Solution<'_, { Day::Day6 }, { Part::One }> for AOC2015<{ Day::Day6 }> {
    type Input = Vec<Instruction>;
    type Output = usize;

    fn solve(&self, input: &Self::Input) -> Result<Self::Output> {
        let mut state = [[false; 1000]; 1000];
        for instruction in input {
            for i in instruction.start.x..(instruction.end.x + 1) {
                for j in instruction.start.y..(instruction.end.y + 1) {
                    state[i][j] = match instruction.instruction_type {
                        InstructionType::On => true,
                        InstructionType::Off => false,
                        InstructionType::Toggle => !state[i][j],
                    }
                }
            }
        }
        Ok(state
            .iter()
            .map(|row| row.iter().filter(|light| **light))
            .flatten()
            .count())
    }
}

impl Solution<'_, { Day::Day6 }, { Part::Two }> for AOC2015<{ Day::Day6 }> {
    type Input = Vec<Instruction>;
    type Output = u32;

    fn solve(&self, input: &Self::Input) -> Result<Self::Output> {
        let mut state = [[0u32; 1000]; 1000];
        for instruction in input {
            for i in instruction.start.x..(instruction.end.x + 1) {
                for j in instruction.start.y..(instruction.end.y + 1) {
                    match instruction.instruction_type {
                        InstructionType::On => state[i][j] += 1,
                        InstructionType::Off => state[i][j] = state[i][j].saturating_sub(1),
                        InstructionType::Toggle => state[i][j] += 2,
                    }
                }
            }
        }
        Ok(state.iter().map(|row| row.iter()).flatten().sum())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_runner::PartOneVerifier;

    #[test]
    fn test() -> Result<()> {
        let problem = super::AOC2015::<{ Day::Day6 }>;
        problem.test_part1("turn on 0,0 through 999,999", 1_000_000)
    }
}
