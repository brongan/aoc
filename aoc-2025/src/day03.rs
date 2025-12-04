use super::AOC2025;
use anyhow::Result;
use aoc_runner::{Day, ParseInput, Part, Solution};

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

impl Solution<'_, { Day::Day3 }, { Part::Two }> for AOC2025<{ Day::Day3 }> {
    type Input = Vec<Vec<u64>>;
    type Output = u64;

    fn solve(&self, input: &Self::Input) -> Result<Self::Output> {
        let joltage = |line: &[u64]| -> u64 {
            line.array_windows::<12>()
                .map(|window| window.iter().fold(0, |acc, num| acc * 10 + num))
                .max()
                .unwrap()
        };
        Ok(input.iter().map(|line| joltage(line)).sum())
    }
}
