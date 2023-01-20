use super::AOC2021;
use anyhow::Context;
use anyhow::Result;
use aoc_runner::{Day, ParseInput, Part, Solution};

pub struct Entry {
    dir: String,
    dist: u32,
}

impl ParseInput<'_, { Day::Day2 }> for AOC2021<{ Day::Day2 }> {
    type Parsed = Vec<Entry>;

    fn parse_input(&self, input: &'_ str) -> Result<Self::Parsed> {
        input
            .lines()
            .map(|s| {
                let mut words = s.split_whitespace().take(2);
                Ok(Entry {
                    dir: words
                        .next()
                        .context("Missing first word on line")?
                        .to_string(),
                    dist: words
                        .next()
                        .context("Missing second word on line")?
                        .parse::<u32>()
                        .context("Failed to parse input")?,
                })
            })
            .collect()
    }
}

impl Solution<'_, { Day::Day2 }, { Part::One }> for AOC2021<{ Day::Day2 }> {
    type Input = Vec<Entry>;
    type Output = u32;

    fn solve(&self, input: &Self::Input) -> Result<Self::Output> {
        let mut position = 0;
        let mut depth = 0;
        for entry in input {
            if "forward" == entry.dir {
                position += entry.dist;
            } else if "down" == entry.dir {
                depth += entry.dist;
            } else if "up" == entry.dir {
                depth -= entry.dist;
            }
        }
        eprintln!(
            "Position: {}, Depth: {}, Value: {}",
            position,
            depth,
            position * depth
        );
        Ok(position * depth)
    }
}

impl Solution<'_, { Day::Day2 }, { Part::Two }> for AOC2021<{ Day::Day2 }> {
    type Input = Vec<Entry>;
    type Output = u32;

    fn solve(&self, input: &Self::Input) -> Result<Self::Output> {
        let mut position = 0;
        let mut depth = 0;
        let mut aim = 0;

        for entry in input {
            if "forward" == entry.dir {
                position += entry.dist;
                depth += aim * entry.dist;
            } else if "down" == entry.dir {
                aim += entry.dist;
            } else if "up" == entry.dir {
                aim -= entry.dist;
            }
        }

        eprintln!(
            "Position: {}, Depth: {}, Aim: {}, Value: {}",
            position,
            depth,
            aim,
            position * depth
        );
        Ok(position * depth)
    }
}
