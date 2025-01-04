use super::AOC2021;
use crate::aoc::{Day, ParseInput, Part, Solution};
use std::ops::Range;

impl ParseInput<'_, { Day::Day7 }> for AOC2021<{ Day::Day7 }> {
    type Parsed = Vec<usize>;

    fn parse_input(&self, input: &'_ str) -> Self::Parsed {
        input
            .trim()
            .split(',')
            .map(|num| num.parse::<usize>().expect("Failed to parse number"))
            .collect()
    }
}

impl Solution<'_, { Day::Day7 }, { Part::One }> for AOC2021<{ Day::Day7 }> {
    type Input = Vec<usize>;
    type Output = usize;

    fn solve(&self, input: &Self::Input) -> Self::Output {
        fn median(list: &mut Vec<usize>) -> usize {
            list.sort_unstable();
            list[list.len() / 2]
        }
        let median = median(&mut input.clone());
        input
            .iter()
            .map(|elem| usize::abs_diff(*elem, median))
            .sum()
    }
}

impl Solution<'_, { Day::Day7 }, { Part::Two }> for AOC2021<{ Day::Day7 }> {
    type Input = Vec<usize>;
    type Output = usize;

    fn solve(&self, input: &Self::Input) -> Self::Output {
        fn part2_fuel_cost(list: &[usize], value: usize) -> usize {
            list.iter()
                .map(|elem| {
                    let n = usize::abs_diff(*elem, value);
                    n * (n + 1) / 2
                })
                .sum()
        }
        let possible_range: Range<usize> =
            (*input.iter().min().expect("has min"))..(*input.iter().max().unwrap());
        possible_range
            .into_iter()
            .map(|crab| part2_fuel_cost(&input, crab))
            .min()
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aoc::PartOneVerifier;
    use crate::aoc::PartTwoVerifier;

    #[test]
    fn test() -> Result<(), String> {
        let problem = super::AOC2021::<{ Day::Day7 }>;
        (&&&problem).test_part1("16,1,2,0,4,2,7,1,2,14", 37)?;
        (&&&problem).test_part2("16,1,2,0,4,2,7,1,2,14", 168)
    }
}
