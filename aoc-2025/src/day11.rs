use super::AOC2025;
use anyhow::Result;
use aoc_runner::{Day, ParseInput, Part, Solution};

pub struct Device {
    output: String,
    inputs: Vec<String>,
}

type IR = Vec<Device>;
type Num = usize;

impl ParseInput<'_, { Day::Day11 }> for AOC2025<{ Day::Day11 }> {
    type Parsed = IR;

    fn parse_input(&self, input: &'_ str) -> Result<Self::Parsed> {
        todo!();
    }
}

impl Solution<'_, { Day::Day11 }, { Part::One }> for AOC2025<{ Day::Day11 }> {
    type Input = IR;
    type Output = Num;

    fn solve(&self, input: &Self::Input) -> Result<Self::Output> {
        todo!();
    }
}
