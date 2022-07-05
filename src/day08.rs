use super::AOC2021;
use crate::aoc::{Day, ParseInput, Part, Solution};
use std::str::FromStr;

#[allow(dead_code)]
pub struct Entry {
    signal: Vec<String>,
    output: Vec<String>,
}

impl ParseInput<'_, { Day::Day8 }> for AOC2021<{ Day::Day8 }> {
    type Parsed = Vec<Entry>;

    fn parse_input(&self, input: &'_ str) -> Self::Parsed {
        input
            .lines()
            .map(Entry::from_str)
            .map(|r| r.expect("failed to parse entry"))
            .collect()
    }
}

impl FromStr for Entry {
    type Err = std::string::ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (signal, output) = s.split_once('|').expect("Failed to split line");
        let signal = signal
            .split(' ')
            .map(|word| word.chars().collect())
            .collect();
        let output = output
            .split(' ')
            .map(|word| word.chars().collect())
            .collect();
        Ok(Entry { signal, output })
    }
}

impl Solution<'_, { Day::Day8 }, { Part::One }> for AOC2021<{ Day::Day8 }> {
    type Input = Vec<Entry>;
    type Output = usize;

    fn solve(&self, input: &Self::Input) -> Self::Output {
        input.len()
    }
}
