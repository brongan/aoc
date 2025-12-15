use rayon::prelude::*;
use std::collections::VecDeque;

use super::AOC2025;
use anyhow::Result;
use aoc_runner::{Day, ParseInput, Part, Solution};
use nom::{
    IResult,
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, newline},
    combinator::{all_consuming, map},
    multi::{many0, separated_list0},
    sequence::{delimited, separated_pair},
};

type Schematic = Vec<usize>;

pub struct Machine {
    indicator: Vec<bool>,
    wiring: Vec<Schematic>,
    joltage: Schematic,
}

fn update_state(schematic: &Schematic, mut state: Vec<bool>) -> Vec<bool> {
    for index in schematic {
        state[*index] = !state[*index]
    }
    state
}

impl Machine {
    pub fn num_presses(&self) -> u64 {
        let mut queue = VecDeque::from([(vec![false; self.indicator.len()], 1)]);
        loop {
            let (state, score) = queue.pop_front().unwrap();
            for schematic in &self.wiring {
                let new = update_state(&schematic, state.clone());
                if new == self.indicator {
                    return score;
                }
                queue.push_back((new, score + 1));
            }
        }
    }
}

fn parse_usize(input: &str) -> IResult<&str, usize> {
    map(digit1, |s: &str| s.parse::<usize>().unwrap())(input)
}

fn parse_indicator(input: &str) -> IResult<&str, Vec<bool>> {
    let parse_bool = |input| alt((map(tag("#"), |_| true), map(tag("."), |_| false)))(input);
    delimited(tag("["), many0(parse_bool), tag("]"))(input)
}

fn parse_schematic(input: &str) -> IResult<&str, Schematic> {
    alt((
        delimited(tag("{"), separated_list0(tag(","), parse_usize), tag("}")),
        delimited(tag("("), separated_list0(tag(","), parse_usize), tag(")")),
    ))(input)
}

fn parse_machine(input: &str) -> IResult<&str, Machine> {
    let parse_wiring = |input| separated_list0(tag(" "), parse_schematic)(input);
    map(
        separated_pair(parse_indicator, tag(" "), parse_wiring),
        |(indicator, mut wiring)| {
            let joltage = wiring.pop().unwrap();
            Machine {
                indicator,
                wiring,
                joltage,
            }
        },
    )(input)
}

fn parse_input(input: &str) -> IResult<&str, Vec<Machine>> {
    separated_list0(newline, parse_machine)(input)
}

type IR = Vec<Machine>;

impl ParseInput<'_, { Day::Day10 }> for AOC2025<{ Day::Day10 }> {
    type Parsed = IR;

    fn parse_input(&self, input: &'_ str) -> Result<Self::Parsed> {
        let (_, input) = all_consuming(parse_input)(input.trim()).map_err(|e| e.to_owned())?;
        Ok(input)
    }
}

impl Solution<'_, { Day::Day10 }, { Part::One }> for AOC2025<{ Day::Day10 }> {
    type Input = IR;
    type Output = u64;

    fn solve(&self, input: &Self::Input) -> Result<Self::Output> {
        Ok(input.par_iter().map(Machine::num_presses).sum())
    }
}
