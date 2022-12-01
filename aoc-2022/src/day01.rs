use super::AOC2022;
use aoc_runner::{Day, ParseInput, Part, Solution};

use anyhow::Context;
use anyhow::Result;
use itertools::sorted;

impl ParseInput<'_, { Day::Day1 }> for AOC2022<{ Day::Day1 }> {
    type Parsed = Vec<Vec<u32>>;

    fn parse_input(&self, input: &'_ str) -> Result<Self::Parsed> {
        input
            .split("\n\n")
            .map(|elf| {
                elf.split('\n')
                    .map(|calories| calories.parse().context("Failed to parse line"))
                    .collect()
            })
            .collect::<Result<Self::Parsed>>()
    }
}
impl Solution<'_, { Day::Day1 }, { Part::One }> for AOC2022<{ Day::Day1 }> {
    type Input = Vec<Vec<u32>>;
    type Output = u32;

    fn solve(&self, input: &Self::Input) -> Result<Self::Output> {
        input
            .iter()
            .map(|elf| elf.iter().sum())
            .max()
            .context("no elves")
    }
}

impl Solution<'_, { Day::Day1 }, { Part::Two }> for AOC2022<{ Day::Day1 }> {
    type Input = Vec<Vec<u32>>;
    type Output = u32;

    fn solve(&self, input: &Self::Input) -> Result<Self::Output> {
        Ok(sorted(input.iter().map(|elf| elf.iter().sum::<u32>()))
            .rev()
            .take(3)
            .sum())
    }
}
