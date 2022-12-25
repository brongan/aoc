use std::str::FromStr;

use super::AOC2022;
use aoc_runner::{Day, ParseInput, Part, Solution};

use anyhow::Context;
use anyhow::Result;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::anychar;
use nom::character::complete::digit1;
use nom::character::complete::multispace0;
use nom::combinator::map_res;
use nom::multi::many0;
use nom::multi::separated_list1;
use nom::sequence::delimited;
use nom::sequence::tuple;
use nom::IResult;

type Crate = char;
type Stack = Vec<Crate>;

pub struct Input {
    initial_state: Vec<Stack>,
    instructions: Vec<Instruction>,
}

pub struct Instruction {
    num_crates: usize,
    from: usize,
    to: usize,
}

fn parse_crate(s: &str) -> IResult<&str, Option<Crate>> {
    delimited(tag("["), anychar, tag("]"))(s).map(|(s, c)| (s, Some(c)))
}

fn air(s: &str) -> IResult<&str, Option<Crate>> {
    tag("   ")(s).map(|(s, _)| (s, None))
}

fn try_parse_crate(s: &str) -> IResult<&str, Option<Crate>> {
    alt((air, parse_crate))(s)
}

fn parse_crate_row(s: &str) -> IResult<&str, Vec<Option<Crate>>> {
    separated_list1(tag(" "), try_parse_crate)(s)
}

fn parse_stack_labels(s: &str) -> IResult<&str, Vec<usize>> {
    many0(delimited(multispace0, parse_number, multispace0))(s)
}

fn parse_state(s: &str) -> Result<Vec<Stack>> {
    let mut lines = s.lines().rev();
    match parse_stack_labels(lines.next().context("Missing stack label line")?) {
        Ok((_, stack_labels)) => {
            let mut state = vec![Vec::new(); stack_labels.len()];
            for line in lines {
                if let Ok((_, crates)) = parse_crate_row(line) {
                    for (j, c) in crates.into_iter().enumerate() {
                        if let Some(c) = c {
                            state[j].push(c);
                        }
                    }
                } else {
                    return Err(anyhow::anyhow!("Failed to parse line: {}", line));
                }
            }
            Ok(state)
        }
        Err(e) => Err(anyhow::anyhow!("Failed to parse stack labels: {}", e)),
    }
}

fn parse_number<T>(s: &str) -> IResult<&str, T>
where
    T: FromStr,
{
    map_res(digit1, |s: &str| s.parse())(s)
}

fn parse_instruction(s: &str) -> IResult<&str, Instruction> {
    // move 6 from 5 to 7
    let (s, (_, num_crates, _, from, _, to)) = tuple((
        tag("move "),
        parse_number,
        tag(" from "),
        parse_number,
        tag(" to "),
        parse_number,
    ))(s)?;
    Ok((
        s,
        Instruction {
            num_crates,
            from,
            to,
        },
    ))
}

fn parse_instructions(s: &str) -> IResult<&str, Vec<Instruction>> {
    separated_list1(tag("\n"), parse_instruction)(s)
}

fn eval_instruction1(state: &mut [Stack], instruction: &Instruction) {
    for _ in 0..instruction.num_crates {
        let c = state[instruction.from - 1].pop().unwrap();
        state[instruction.to - 1].push(c);
    }
}

fn eval_instruction2(state: &mut [Stack], instruction: &Instruction) {
    let from = &mut state[instruction.from - 1];
    let mut crates = from.split_off(from.len() - instruction.num_crates);
    let to = &mut state[instruction.to - 1];
    to.append(&mut crates);
}

impl ParseInput<'_, { Day::Day5 }> for AOC2022<{ Day::Day5 }> {
    type Parsed = Input;

    fn parse_input(&self, input: &'_ str) -> Result<Self::Parsed> {
        let input = input.split_once("\n\n").context("Failed to split input")?;
        let initial_state = parse_state(input.0)?;
        match parse_instructions(input.1) {
            Ok((_, instructions)) => Ok(Input {
                initial_state,
                instructions,
            }),
            Err(e) => Err(anyhow::anyhow!("Failed to parse instructions: {}", e)),
        }
    }
}

fn solve(input: &Input, eval_instruction: fn(&mut [Stack], &Instruction)) -> Result<String> {
    let mut state = input.initial_state.clone();
    for instruction in &input.instructions {
        eval_instruction(&mut state, instruction);
    }
    state
        .iter()
        .map(|stack| stack.last().context("Empty stack of crates"))
        .collect()
}

impl Solution<'_, { Day::Day5 }, { Part::One }> for AOC2022<{ Day::Day5 }> {
    type Input = Input;
    type Output = String;

    fn solve(&self, input: &Self::Input) -> Result<Self::Output> {
        solve(input, eval_instruction1)
    }
}

impl Solution<'_, { Day::Day5 }, { Part::Two }> for AOC2022<{ Day::Day5 }> {
    type Input = Input;
    type Output = String;

    fn solve(&self, input: &Self::Input) -> Result<Self::Output> {
        solve(input, eval_instruction2)
    }
}
