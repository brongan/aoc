use super::AOC2022;
use anyhow::Context;
use anyhow::Result;
use aoc_runner::{Day, ParseInput, Part, Solution};

impl ParseInput<'_, { Day::Day15 }> for AOC2022<{ Day::Day15 }> {
    type Parsed = Vec<Vec<u32>>;

    fn parse_input(&self, input: &'_ str) -> Result<Self::Parsed> {}
}
impl Solution<'_, { Day::Day15 }, { Part::One }> for AOC2022<{ Day::Day15 }> {
    type Input = Vec<Vec<u32>>;
    type Output = u32;

    fn solve(&self, input: &Self::Input) -> Result<Self::Output> {}
}
