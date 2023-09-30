use super::AOC2015;
use anyhow::{Context, Result};
use aoc_runner::{Day, ParseInput, Part, Solution};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, digit1, newline},
    combinator::{map, map_res},
    multi::separated_list1,
    sequence::{preceded, separated_pair, tuple},
    IResult,
};
use std::collections::HashMap;

type WireName = String;
type SignalValue = u16;

#[derive(Debug, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub enum Operand {
    Signal(SignalValue),
    Wire(WireName),
}

impl Operand {
    fn parse(input: &str) -> IResult<&str, Operand> {
        let parse_wire = |input| map(alpha1, |s: &str| s.to_string())(input);
        let parse_signal = |input| map_res(digit1, |s: &str| s.parse())(input);
        alt((
            map(parse_wire, |w| Operand::Wire(w)),
            map(parse_signal, |s| Operand::Signal(s)),
        ))(input)
    }
}

#[derive(Debug)]
pub enum Operation {
    Value(Operand),
    Not(Operand),
    And(Operand, Operand),
    Or(Operand, Operand),
    LShift(Operand, u8),
    RShift(Operand, u8),
}

fn parse_shift(input: &str) -> IResult<&str, u8> {
    map_res(digit1, |s: &str| s.parse())(input)
}

impl Operation {
    fn parse(input: &str) -> IResult<&str, Operation> {
        let parse_literal = |input| map(Operand::parse, |operand| Operation::Value(operand))(input);
        let parse_not = |input| {
            map(preceded(tag("NOT "), Operand::parse), |operand: Operand| {
                Operation::Not(operand)
            })(input)
        };
        let parse_and = |input| {
            map(
                separated_pair(Operand::parse, tag(" AND "), Operand::parse),
                |(l, r)| Operation::And(l, r),
            )(input)
        };
        let parse_or = |input| {
            map(
                separated_pair(Operand::parse, tag(" OR "), Operand::parse),
                |(l, r)| Operation::Or(l, r),
            )(input)
        };
        let parse_lshift = |input| {
            map(
                separated_pair(Operand::parse, tag(" LSHIFT "), parse_shift),
                |(wire, shift)| Operation::LShift(wire, shift),
            )(input)
        };
        let parse_rshift = |input| {
            map(
                separated_pair(Operand::parse, tag(" RSHIFT "), parse_shift),
                |(wire, shift)| Operation::RShift(wire, shift),
            )(input)
        };

        alt((
            parse_literal,
            parse_not,
            parse_and,
            parse_or,
            parse_lshift,
            parse_rshift,
        ))(input)
    }
}

#[derive(Debug)]
pub struct Instruction {
    operator: Operation,
    wire: WireName,
}

impl Instruction {
    fn parse(input: &str) -> IResult<&str, Instruction> {
        map(
            tuple((
                Operation::parse,
                preceded(tag(" -> "), map(alpha1, |s: &str| s.to_string())),
            )),
            |(operator, wire)| Instruction { operator, wire },
        )(input)
    }
}

impl ParseInput<'_, { Day::Day7 }> for AOC2015<{ Day::Day7 }> {
    type Parsed = HashMap<WireName, Operation>;

    fn parse_input(&self, input: &'_ str) -> Result<Self::Parsed> {
        let (remainder, instructions) =
            separated_list1(newline, Instruction::parse)(input).map_err(|e| e.to_owned())?;
        eprintln!("remainder: {remainder}");
        Ok(instructions
            .into_iter()
            .map(|instruction| (instruction.wire, instruction.operator))
            .collect())
    }
}

fn eval_wire(wire: &WireName, input: &HashMap<WireName, Operation>) -> Result<SignalValue> {
    let val = match input.get(wire).context(format!("Missing {wire}"))? {
        Operation::Value(operand) => eval_operand(operand, input)?,
        Operation::Not(wire) => !eval_operand(&wire, input)?,
        Operation::Or(l, r) => eval_operand(&l, input)? | eval_operand(&r, input)?,
        Operation::And(l, r) => eval_operand(&l, input)? & eval_operand(&r, input)?,
        Operation::LShift(wire, bits) => eval_operand(&wire, input)? << bits,
        Operation::RShift(wire, bits) => eval_operand(&wire, input)? >> bits,
    };
    Ok(val)
}

fn eval_operand(operand: &Operand, input: &HashMap<WireName, Operation>) -> Result<SignalValue> {
    match operand {
        Operand::Wire(wire) => eval_wire(wire, input),
        Operand::Signal(num) => Ok(*num),
    }
}

impl Solution<'_, { Day::Day7 }, { Part::One }> for AOC2015<{ Day::Day7 }> {
    type Input = HashMap<WireName, Operation>;
    type Output = SignalValue;

    fn solve(&self, input: &Self::Input) -> Result<Self::Output> {
        eval_wire(&"aa".to_string(), &input)
    }
}
