use super::AOC2025;
use anyhow::Result;
use aoc_runner::{Day, ParseInput, Part, Solution};
use std::str::FromStr;

impl ParseInput<'_, { Day::Day1 }> for AOC2025<{ Day::Day1 }> {
    type Parsed = Vec<i32>;

    fn parse_input(&self, input: &'_ str) -> Result<Self::Parsed> {
        Ok(input
            .trim()
            .lines()
            .map(|line| {
                let dir = line.chars().next().unwrap();
                let num = i32::from_str(&line[1..]).unwrap();
                match dir {
                    'L' => -num,
                    'R' => num,
                    c => panic!("Unexpected dir: {c}"),
                }
            })
            .collect())
    }
}

impl Solution<'_, { Day::Day1 }, { Part::One }> for AOC2025<{ Day::Day1 }> {
    type Input = Vec<i32>;
    type Output = u64;

    fn solve(&self, input: &Self::Input) -> Result<Self::Output> {
        let mut dial = 50;
        let mut count = 0;

        for num in input {
            dial += num;
            dial %= 100;
            if dial == 0 {
                count += 1;
            }
        }

        Ok(count)
    }
}

impl Solution<'_, { Day::Day1 }, { Part::Two }> for AOC2025<{ Day::Day1 }> {
    type Input = Vec<i32>;
    type Output = i32;

    fn solve(&self, input: &Self::Input) -> Result<Self::Output> {
        let mut dial = 50;
        let mut count = 0;

        for num in input {
            for _ in 0..num.abs() {
                if *num > 0 {
                    dial += 1;
                } else {
                    dial -= 1;
                }
                dial %= 100;
                if dial == 0 {
                    count += 1;
                }
            }
        }

        Ok(count)
    }
}
