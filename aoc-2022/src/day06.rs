use super::AOC2022;
use aoc_runner::{Day, ParseInput, Part, Solution};

use anyhow::Result;
use counter::Counter;

impl ParseInput<'_, { Day::Day6 }> for AOC2022<{ Day::Day6 }> {
    type Parsed = String;

    fn parse_input(&self, input: &'_ str) -> Result<Self::Parsed> {
        Ok(input.to_owned())
    }
}

fn first_unique_window(s: &str, size: usize) -> Result<usize> {
    let input = s.as_bytes();
    let mut counter = input[..size].iter().copied().collect::<Counter<u8>>();
    if counter.len() == size {
        return Ok(size);
    }
    for (i, c) in input[size..].iter().enumerate() {
        counter[&input[i]] -= 1;
        counter[c] += 1;
        counter.retain(|_, &mut v| v > 0);
        if counter.len() == size {
            eprintln!("{}: {:?}", i, counter);
            return Ok(1 + i + size);
        }
    }
    Err(anyhow::anyhow!("No unique window found."))
}

impl Solution<'_, { Day::Day6 }, { Part::One }> for AOC2022<{ Day::Day6 }> {
    type Input = String;
    type Output = usize;

    fn solve(&self, input: &Self::Input) -> Result<Self::Output> {
        first_unique_window(input, 4)
    }
}

impl Solution<'_, { Day::Day6 }, { Part::Two }> for AOC2022<{ Day::Day6 }> {
    type Input = String;
    type Output = usize;

    fn solve(&self, input: &Self::Input) -> Result<Self::Output> {
        first_unique_window(input, 14)
    }
}
