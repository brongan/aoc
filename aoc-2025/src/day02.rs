use super::AOC2025;
use anyhow::Result;
use aoc_runner::{Day, ParseInput, Part, Solution};
use itertools::Itertools;
use std::str::FromStr;

impl ParseInput<'_, { Day::Day2 }> for AOC2025<{ Day::Day2 }> {
    type Parsed = Vec<(u64, u64)>;

    fn parse_input(&self, input: &'_ str) -> Result<Self::Parsed> {
        Ok(input
            .trim()
            .split(",")
            .map(|range| {
                let nums: Vec<&str> = range.split("-").collect();
                let low = u64::from_str(nums[0]).unwrap();
                let hi = u64::from_str(nums[1]).unwrap();
                (low, hi)
            })
            .collect())
    }
}

impl Solution<'_, { Day::Day2 }, { Part::One }> for AOC2025<{ Day::Day2 }> {
    type Input = Vec<(u64, u64)>;
    type Output = u64;

    fn solve(&self, input: &Self::Input) -> Result<Self::Output> {
        let invalid = |num: u64| -> bool {
            let str = format!("{num}");
            let len = str.len();
            if len % 2 == 1 {
                return false;
            }
            &str[0..(len / 2)] == &str[(len / 2)..]
        };
        Ok(input
            .iter()
            .map(|range| (range.0..=range.1).into_iter().filter(|num| invalid(*num)))
            .flatten()
            .sum())
    }
}

impl Solution<'_, { Day::Day2 }, { Part::Two }> for AOC2025<{ Day::Day2 }> {
    type Input = Vec<(u64, u64)>;
    type Output = u64;

    fn solve(&self, input: &Self::Input) -> Result<Self::Output> {
        let invalid = |num: u64| -> bool {
            let str = format!("{num}");
            let strlen = str.len();
            for len in 1..strlen {
                if strlen % len != 0 {
                    continue;
                }
                if str.as_bytes().chunks(len).all_equal() {
                    return true;
                }
            }
            false
        };
        Ok(input
            .iter()
            .map(|range| (range.0..=range.1).into_iter().filter(|num| invalid(*num)))
            .flatten()
            .sum())
    }
}
