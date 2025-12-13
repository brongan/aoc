use super::AOC2025;
use anyhow::Result;
use aoc_runner::{Day, ParseInput, Part, Solution};

type Schematic = Vec<i32>;

pub struct Machine {
    indicator: Vec<bool>,
    wiring: Vec<Schematic>,
}
type IR = Vec<Machine>;
type Num = usize;

impl ParseInput<'_, { Day::Day10 }> for AOC2025<{ Day::Day10 }> {
    type Parsed = IR;

    fn parse_input(&self, input: &'_ str) -> Result<Self::Parsed> {
        todo!();
    }
}

impl Solution<'_, { Day::Day10 }, { Part::One }> for AOC2025<{ Day::Day10 }> {
    type Input = IR;
    type Output = Num;

    fn solve(&self, input: &Self::Input) -> Result<Self::Output> {
        todo!();
    }
}
