use super::AOC2025;
use anyhow::Result;
use aoc_runner::{Day, ParseInput, Part, Solution};

type Shape = Vec<Vec<bool>>;
pub struct Region {
    length: u32,
    width: u32,
    quantities: Vec<u32>,
}

pub struct IR {
    trees: Vec<Shape>,
    regions: Vec<Region>,
}

type Num = usize;

impl ParseInput<'_, { Day::Day12 }> for AOC2025<{ Day::Day12 }> {
    type Parsed = IR;

    fn parse_input(&self, input: &'_ str) -> Result<Self::Parsed> {
        todo!();
    }
}

impl Solution<'_, { Day::Day12 }, { Part::One }> for AOC2025<{ Day::Day12 }> {
    type Input = IR;
    type Output = Num;

    fn solve(&self, input: &Self::Input) -> Result<Self::Output> {
        todo!();
    }
}
