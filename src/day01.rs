use super::AOC2021;
use crate::aoc::{Day, ParseInput, Part, Solution};

use std::iter::zip;

impl ParseInput<'_, { Day::Day1 }> for AOC2021<{ Day::Day1 }> {
    type Parsed = Vec<u32>;

    fn parse_input(&self, input: &'_ str) -> Self::Parsed {
        input
            .lines()
            .into_iter()
            .map(|s| s.parse::<u32>().expect("Failed to parse input"))
            .collect()
    }
}

impl Solution<'_, { Day::Day1 }, { Part::One }> for AOC2021<{ Day::Day1 }> {
    type Input = Vec<u32>;
    type Output = usize;

    fn solve(&self, input: &Self::Input) -> Self::Output {
        1 + zip(input.iter(), input[1..].iter())
            .filter(|(first, second)| second > first && **first != 0)
            .count()
    }
}

impl Solution<'_, { Day::Day1 }, { Part::Two }> for AOC2021<{ Day::Day1 }> {
    type Input = Vec<u32>;
    type Output = usize;

    fn solve(&self, input: &Self::Input) -> Self::Output {
        let mut count: Self::Output = 1;
        let mut old_sum: u32 = input[0..3].iter().sum();
        let mut new_sum;
        for (i, num) in input[3..].iter().enumerate() {
            new_sum = old_sum + num - input[i];
            if new_sum > old_sum {
                count += 1
            }
            old_sum = new_sum
        }
        count
    }
}
