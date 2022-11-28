use super::AOC2021;
use anyhow::Context;
use anyhow::anyhow;
use anyhow::Result;
use aoc_runner::{Day, ParseInput, Part, Solution};
use std::ops::Range;

impl ParseInput<'_, { Day::Day7 }> for AOC2021<{ Day::Day7 }> {
    type Parsed = Vec<usize>;

    fn parse_input(&self, input: &'_ str) -> Result<Self::Parsed> {
        input
            .trim()
            .split(',')
            .map(|num| num.parse::<usize>().context("Failed to parse number"))
            .collect()
    }
}

impl Solution<'_, { Day::Day7 }, { Part::One }> for AOC2021<{ Day::Day7 }> {
    type Input = Vec<usize>;
    type Output = usize;

    fn solve(&self, input: &Self::Input) -> Result<Self::Output> {
        fn median(list: &mut Vec<usize>) -> usize {
            list.sort_unstable();
            list[list.len() / 2]
        }
        let median = median(&mut input.clone());
        Ok(input
            .iter()
            .map(|elem| usize::abs_diff(*elem, median))
            .sum())
    }
}

impl Solution<'_, { Day::Day7 }, { Part::Two }> for AOC2021<{ Day::Day7 }> {
    type Input = Vec<usize>;
    type Output = usize;

    fn solve(&self, input: &Self::Input) -> Result<Self::Output> {
        fn part2_fuel_cost(list: &[usize], value: usize) -> usize {
            list.iter()
                .map(|elem| {
                    let n = usize::abs_diff(*elem, value);
                    n * (n + 1) / 2
                })
                .sum()
        }
        let possible_range: Range<usize> =
            (*input.iter().min().ok_or_else(|| anyhow!("Has a min"))?)
                ..(*input.iter().max().ok_or_else(|| anyhow!("Has a max"))?);
        possible_range
            .into_iter()
            .map(|crab| part2_fuel_cost(input, crab))
            .min()
            .context("Failed to find min")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_runner::PartOneVerifier;
    use aoc_runner::PartTwoVerifier;

    #[test]
    fn test() -> Result<()> {
        let problem = super::AOC2021::<{ Day::Day7 }>;
        problem.test_part1("16,1,2,0,4,2,7,1,2,14", 37)?;
        problem.test_part2("16,1,2,0,4,2,7,1,2,14", 168)
    }
}
