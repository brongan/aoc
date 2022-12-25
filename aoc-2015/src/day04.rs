use super::AOC2015;

use aoc_runner::{Day, ParseInput, Part, Solution};

use anyhow::Result;

impl ParseInput<'_, { Day::Day4 }> for AOC2015<{ Day::Day4 }> {
    type Parsed = String;

    fn parse_input(&self, input: &'_ str) -> Result<Self::Parsed> {
        Ok(input.to_string())
    }
}

impl Solution<'_, { Day::Day4 }, { Part::One }> for AOC2015<{ Day::Day4 }> {
    type Input = String;
    type Output = usize;

    fn solve(&self, input: &Self::Input) -> Result<Self::Output> {
        let mut i = 0;
        loop {
            let hash = md5::compute(format!("{input}{i}"));
            if hash[0] == 0 && hash[1] == 0 && hash[2] < 16 {
                return Ok(i);
            }
            i += 1;
        }
    }
}

impl Solution<'_, { Day::Day4 }, { Part::Two }> for AOC2015<{ Day::Day4 }> {
    type Input = String;
    type Output = usize;

    fn solve(&self, input: &Self::Input) -> Result<Self::Output> {
        let mut i = 0;
        loop {
            let hash = md5::compute(format!("{input}{i}"));
            if hash[0] == 0 && hash[1] == 0 && hash[2] == 0 {
                return Ok(i);
            }
            i += 1;
        }
    }
}
