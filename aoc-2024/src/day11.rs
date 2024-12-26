use super::AOC2024;
use anyhow::Result;
use aoc_runner::{Day, ParseInput, Part, Solution};
use counter::Counter;

type Num = u64;
type IR = Vec<String>;

impl ParseInput<'_, { Day::Day11 }> for AOC2024<{ Day::Day11 }> {
    type Parsed = IR;

    fn parse_input(&self, input: &'_ str) -> Result<Self::Parsed> {
        Ok(input.split_whitespace().map(|s| s.to_owned()).collect())
    }
}

fn eval_rules(stones: &Counter<String>) -> Counter<String> {
    let mut ret = Counter::new();
    for (stone, count) in stones {
        if *stone == "0" {
            ret[&String::from("1")] += count;
        } else if stone.len() % 2 == 0 {
            let (l, r) = stone.split_at(stone.len() / 2);
            let clean = |s| Num::from_str_radix(s, 10).unwrap().to_string();
            ret[&clean(l).to_string()] += count;
            ret[&clean(r).to_string()] += count;
        } else {
            let num = Num::from_str_radix(stone, 10).unwrap() * 2024;
            ret[&num.to_string()] += count;
        }
    }
    ret
}

fn solve(input: &IR, steps: usize) -> usize {
    let mut stones: Counter<String> = input.iter().map(|s| s.to_owned()).collect();
    for _ in 0..steps {
        stones = eval_rules(&stones);
    }
    stones.iter().map(|(_, c)| c).sum()
}

impl Solution<'_, { Day::Day11 }, { Part::One }> for AOC2024<{ Day::Day11 }> {
    type Input = IR;
    type Output = usize;

    fn solve(&self, input: &Self::Input) -> Result<Self::Output> {
        Ok(solve(input, 25))
    }
}

impl Solution<'_, { Day::Day11 }, { Part::Two }> for AOC2024<{ Day::Day11 }> {
    type Input = IR;
    type Output = usize;

    fn solve(&self, input: &Self::Input) -> Result<Self::Output> {
        Ok(solve(input, 75))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_runner::PartOneVerifier;

    #[test]
    fn test_example() -> Result<()> {
        let problem = super::AOC2024::<{ Day::Day11 }>;
        let input = "125 17";
        problem.test_part1(input, 55312)?;

        Ok(())
    }
}
