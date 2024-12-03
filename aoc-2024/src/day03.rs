use super::AOC2024;
use anyhow::Result;
use aoc_runner::{Day, ParseInput, Part, Solution};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::digit1,
    combinator::{map, map_res},
    sequence::{delimited, separated_pair},
    IResult,
};

type Num = i32;
pub enum Instruction {
    Mul(Num, Num),
    Do,
    Dont,
}
type IR = Vec<Instruction>;

fn parse_element(input: &str) -> IResult<&str, Num> {
    map_res(digit1, |num| Num::from_str_radix(num, 10))(input)
}

fn parse_pair(input: &str) -> IResult<&str, (Num, Num)> {
    separated_pair(parse_element, tag(","), parse_element)(input)
}

fn parse_mul(input: &str) -> IResult<&str, Instruction> {
    map(delimited(tag("mul("), parse_pair, tag(")")), |mul| {
        Instruction::Mul(mul.0, mul.1)
    })(input)
}

fn parse_do(input: &str) -> IResult<&str, Instruction> {
    map(tag("do()"), |_| Instruction::Do)(input)
}

fn parse_dont(input: &str) -> IResult<&str, Instruction> {
    map(tag("don't()"), |_| Instruction::Dont)(input)
}

fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
    alt((parse_mul, parse_do, parse_dont))(input)
}

impl ParseInput<'_, { Day::Day3 }> for AOC2024<{ Day::Day3 }> {
    type Parsed = IR;

    fn parse_input(&self, input: &'_ str) -> Result<Self::Parsed> {
        let mut ret = Vec::new();
        for i in 0..input.len() {
            if let Ok((_, instruction)) = parse_instruction(&input[i..]) {
                ret.push(instruction);
            }
        }
        Ok(ret)
    }
}

impl Solution<'_, { Day::Day3 }, { Part::One }> for AOC2024<{ Day::Day3 }> {
    type Input = IR;
    type Output = Num;

    fn solve(&self, input: &Self::Input) -> Result<Self::Output> {
        Ok(input
            .iter()
            .map(|instruction| match instruction {
                Instruction::Mul(l, r) => l * r,
                _ => 0,
            })
            .sum())
    }
}

impl Solution<'_, { Day::Day3 }, { Part::Two }> for AOC2024<{ Day::Day3 }> {
    type Input = IR;
    type Output = Num;

    fn solve(&self, input: &Self::Input) -> Result<Self::Output> {
        let mut ret = 0;
        let mut enabled = true;
        for instruction in input {
            match instruction {
                Instruction::Mul(l, r) => {
                    if enabled {
                        ret += l * r
                    }
                }
                Instruction::Do => enabled = true,
                Instruction::Dont => enabled = false,
            }
        }
        Ok(ret)
    }
}
