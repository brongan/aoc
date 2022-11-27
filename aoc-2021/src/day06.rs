use super::AOC2021;
use aoc_runner::{Day, ParseInput, Part, Solution};

impl ParseInput<'_, { Day::Day6 }> for AOC2021<{ Day::Day6 }> {
    type Parsed = [u64; 9];

    fn parse_input(&self, input: &'_ str) -> Self::Parsed {
        let mut counts = [0u64; 9];
        for num in input
            .trim()
            .split(',')
            .map(|num| num.parse::<usize>().expect("Failed to parse number."))
        {
            counts[num] += 1;
        }
        counts
    }
}

fn solve(mut counts: [u64; 9], num_iterations: u32) -> u64 {
    for _ in 0..num_iterations {
        let breeders = counts[0];
        for age_bucket in 0..counts.len() - 1 {
            counts[age_bucket] = counts[age_bucket + 1];
        }
        counts[6] += breeders;
        counts[8] = breeders;
    }
    counts.iter().sum()
}

impl Solution<'_, { Day::Day6 }, { Part::One }> for AOC2021<{ Day::Day6 }> {
    type Input = [u64; 9];
    type Output = u64;

    fn solve(&self, input: &Self::Input) -> Self::Output {
        solve(*input, 80)
    }
}

impl Solution<'_, { Day::Day6 }, { Part::Two }> for AOC2021<{ Day::Day6 }> {
    type Input = [u64; 9];
    type Output = u64;

    fn solve(&self, input: &Self::Input) -> Self::Output {
        solve(*input, 256)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_runner::PartOneVerifier;
    use aoc_runner::PartTwoVerifier;

    #[test]
    fn test() -> Result<(), String> {
        let problem = super::AOC2021::<{ Day::Day6 }>;
        problem.test_part1("3,4,3,1,2", 5934)?;
        problem.test_part2("3,4,3,1,2", 26984457539)
    }
}
