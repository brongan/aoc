use super::AOC2015;
use aoc_runner::{Day, ParseInput, Part, Solution};

use anyhow::Result;
use anyhow::anyhow;

impl ParseInput<'_, { Day::Day1 }> for AOC2015<{ Day::Day1 }> {
    type Parsed = String;

    fn parse_input(&self, input: &'_ str) -> Result<Self::Parsed> {
        Ok(input.to_string())
    }
}

impl Solution<'_, { Day::Day1 }, { Part::One }> for AOC2015<{ Day::Day1 }> {
    type Input = String;
    type Output = i32;

    fn solve(&self, input: &Self::Input) -> Result<Self::Output> {
        Ok(input.chars().map(|c| match c { '(' => 1, ')' => -1, _ => panic!("not a parenthesis")}).sum::<i32>())
    }
}

impl Solution<'_, { Day::Day1 }, { Part::Two }> for AOC2015<{ Day::Day1 }> {
    type Input = String;
    type Output = usize;

    fn solve(&self, input: &Self::Input) -> Result<Self::Output> {
        let mut pos = 0;
        for (i, char) in input.chars().enumerate() {
            match char {
                '(' => pos += 1,
                ')' => pos -= 1,
                _ => panic!("not a parenthesis"),
            }
            if pos < 0 {
                return Ok(i + 1);
            }
        }
        Err(anyhow!("never went to the basement"))
    }
}
