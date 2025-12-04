use super::AOC2025;
use anyhow::Result;
use aoc_runner::{Day, ParseInput, Part, Solution};
use rayon::prelude::*;

impl ParseInput<'_, { Day::Day3 }> for AOC2025<{ Day::Day3 }> {
    type Parsed = Vec<Vec<u64>>;

    fn parse_input(&self, input: &'_ str) -> Result<Self::Parsed> {
        Ok(input
            .trim()
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).unwrap() as u64)
                    .collect()
            })
            .collect())
    }
}

impl Solution<'_, { Day::Day3 }, { Part::One }> for AOC2025<{ Day::Day3 }> {
    type Input = Vec<Vec<u64>>;
    type Output = u64;

    fn solve(&self, input: &Self::Input) -> Result<Self::Output> {
        let joltage = |line: &[u64]| -> u64 {
            let (_last, most) = line.split_last().unwrap();
            let (index, max) = most
                .iter()
                .enumerate()
                .max_by(|(i, a), (j, b)| a.cmp(b).then(i.cmp(j).reverse()))
                .unwrap();
            let next = &line[index + 1..].iter().max().unwrap();
            10 * max + *next
        };
        Ok(input.iter().map(|line| joltage(line)).sum())
    }
}

fn joltage2(slice: &[u64], count: u32) -> u64 {
    if count == 0 || slice.len() == 0 {
        return 0;
    }
    let (index, max) = &slice[..slice.len() - count as usize + 1]
        .iter()
        .enumerate()
        .max_by(|(i, a), (j, b)| a.cmp(b).then(i.cmp(j).reverse()))
        .unwrap();
    let val = *max * 10_u64.pow(count - 1);
    val + joltage2(&slice[index + 1..], count - 1)
}

impl Solution<'_, { Day::Day3 }, { Part::Two }> for AOC2025<{ Day::Day3 }> {
    type Input = Vec<Vec<u64>>;
    type Output = u64;

    fn solve(&self, input: &Self::Input) -> Result<Self::Output> {
        Ok(input.par_iter().map(|line| joltage2(line, 12)).sum())
    }
}
