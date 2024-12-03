use super::AOC2015;
use anyhow::{Context, Result};
use aoc_runner::{Day, ParseInput, Part, Solution};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, digit1},
    combinator::{all_consuming, map, map_res},
    sequence::{preceded, separated_pair, tuple},
    IResult,
};
use std::collections::HashMap;

type WireName = String;
type SignalValue = u16;

#[derive(Debug, Hash, Eq, PartialEq, PartialOrd, Ord, Clone)]
pub enum Operand {
    Signal(SignalValue),
    Wire(WireName),
}

impl Operand {
    fn parse(input: &str) -> IResult<&str, Operand> {
        let parse_wire = |input| map(alpha1, |s: &str| s.to_string())(input);
        let parse_signal = |input| map_res(digit1, |s: &str| s.parse())(input);
        alt((
            map(parse_wire, Operand::Wire),
            map(parse_signal, Operand::Signal),
        ))(input)
    }
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Clone)]
pub enum Operation {
    Literal(Operand),
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
        let parse_literal =
            |input| map(Operand::parse, Operation::Literal)(input);
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
            parse_not,
            parse_and,
            parse_or,
            parse_lshift,
            parse_rshift,
            parse_literal,
        ))(input)
    }
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Clone)]
pub struct Instruction {
    operation: Operation,
    wire: WireName,
}

impl Instruction {
    fn parse(input: &str) -> IResult<&str, Instruction> {
        map(
            tuple((
                Operation::parse,
                preceded(tag(" -> "), map(alpha1, |s: &str| s.to_string())),
            )),
            |(operation, wire)| Instruction { operation, wire },
        )(input)
    }
}

impl ParseInput<'_, { Day::Day7 }> for AOC2015<{ Day::Day7 }> {
    type Parsed = Vec<Instruction>;

    fn parse_input(&self, input: &'_ str) -> Result<Self::Parsed> {
        input
            .lines()
            .map(|line| {
                let (_, instruction) =
                    all_consuming(Instruction::parse)(line).map_err(|e| e.to_owned())?;
                Ok(instruction)
            })
            .collect()
    }
}

fn eval_wire(
    wire: &WireName,
    input: &HashMap<WireName, Operation>,
    cache: &mut HashMap<WireName, SignalValue>,
) -> Result<SignalValue> {
    if cache.contains_key(wire) {
        return Ok(cache[wire]);
    }
    let val = match input.get(wire).context(format!("Missing {wire}"))? {
        Operation::Literal(operand) => eval_operand(operand, input, cache)?,
        Operation::Not(wire) => !eval_operand(wire, input, cache)?,
        Operation::Or(l, r) => eval_operand(l, input, cache)? | eval_operand(r, input, cache)?,
        Operation::And(l, r) => eval_operand(l, input, cache)? & eval_operand(r, input, cache)?,
        Operation::LShift(wire, bits) => eval_operand(wire, input, cache)? << bits,
        Operation::RShift(wire, bits) => eval_operand(wire, input, cache)? >> bits,
    };
    cache.insert(wire.clone(), val);
    Ok(val)
}

fn eval_operand(
    operand: &Operand,
    input: &HashMap<WireName, Operation>,
    cache: &mut HashMap<WireName, SignalValue>,
) -> Result<SignalValue> {
    match operand {
        Operand::Wire(wire) => eval_wire(wire, input, cache),
        Operand::Signal(num) => Ok(*num),
    }
}

impl Solution<'_, { Day::Day7 }, { Part::One }> for AOC2015<{ Day::Day7 }> {
    type Input = Vec<Instruction>;
    type Output = SignalValue;

    fn solve(&self, input: &Self::Input) -> Result<Self::Output> {
        let input: HashMap<WireName, Operation> = input
            .clone()
            .into_iter()
            .map(|instruction| (instruction.wire, instruction.operation))
            .collect();

        let mut cache = HashMap::new();
        eval_wire(&"a".to_string(), &input, &mut cache)
    }
}

impl Solution<'_, { Day::Day7 }, { Part::Two }> for AOC2015<{ Day::Day7 }> {
    type Input = Vec<Instruction>;
    type Output = SignalValue;

    fn solve(&self, input: &Self::Input) -> Result<Self::Output> {
        let mut input: HashMap<WireName, Operation> = input
            .clone()
            .into_iter()
            .map(|instruction| (instruction.wire, instruction.operation))
            .collect();

        let a = "a".to_string();
        let b = "b".to_string();
        let mut cache = HashMap::new();
        let a_signal = eval_wire(&a, &input, &mut cache)?;
        input.insert(b, Operation::Literal(Operand::Signal(a_signal)));
        cache.clear();
        eval_wire(&a, &input, &mut cache)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() -> Result<()> {
        let problem = super::AOC2015::<{ Day::Day7 }>;
        let lf = "lf".to_string();
        assert_eq!(Operand::parse("lf "), Ok((" ", Operand::Wire(lf.clone()))));
        assert_eq!(Operand::parse("5 "), Ok((" ", Operand::Signal(5))));

        let operation = Operation::And(Operand::Wire(lf), Operand::Wire("lq".to_string()));
        assert_eq!(
            Operation::parse("lf AND lq -> ls"),
            Ok((" -> ls", operation.clone()))
        );

        let expected = Instruction {
            operation,
            wire: "ls".to_string(),
        };

        assert_eq!(Instruction::parse("lf AND lq -> ls"), Ok(("", expected)));
        let input = "123 -> x
456 -> y
x AND y -> d
x OR y -> e
x LSHIFT 2 -> f
y RSHIFT 2 -> g
NOT x -> h
NOT y -> i";
        let parsed: Vec<Instruction> = problem.parse_input(input)?;
        let input: HashMap<WireName, Operation> = parsed
            .into_iter()
            .map(|instruction| (instruction.wire, instruction.operation))
            .collect();
        let mut cache = HashMap::new();

        assert_eq!(eval_wire(&"d".to_string(), &input, &mut cache)?, 72);
        assert_eq!(eval_wire(&"e".to_string(), &input, &mut cache)?, 507);
        assert_eq!(eval_wire(&"f".to_string(), &input, &mut cache)?, 492);
        assert_eq!(eval_wire(&"g".to_string(), &input, &mut cache)?, 114);
        assert_eq!(eval_wire(&"h".to_string(), &input, &mut cache)?, 65412);
        assert_eq!(eval_wire(&"i".to_string(), &input, &mut cache)?, 65079);
        assert_eq!(eval_wire(&"x".to_string(), &input, &mut cache)?, 123);
        assert_eq!(eval_wire(&"y".to_string(), &input, &mut cache)?, 456);
        Ok(())
    }
}
