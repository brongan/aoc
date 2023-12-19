use super::AOC2023;
use anyhow::{Context, Result};
use aoc_runner::{Day, ParseInput, Part, Solution};
use itertools::Itertools;

type Num = i64;
type History = Vec<Num>;

fn parse_line(line: &str) -> Result<History> {
    line.split_whitespace()
        .into_iter()
        .map(|num| num.parse::<Num>().map_err(anyhow::Error::msg))
        .collect()
}

impl ParseInput<'_, { Day::Day9 }> for AOC2023<{ Day::Day9 }> {
    type Parsed = Vec<History>;

    fn parse_input(&self, input: &'_ str) -> Result<Self::Parsed> {
        input.lines().map(|line| parse_line(line)).collect()
    }
}

fn predict_next_value(history: &[Num]) -> Result<Num> {
    let mut prediction: Num = *history
        .last()
        .context("Cannot predict with empty history.")?;
    let mut curr: Vec<Num> = history.iter().map(|num| num.to_owned()).collect_vec();
    while curr.iter().any(|num| *num != 0) {
        curr = curr.iter().tuple_windows().map(|(l, r)| r - l).collect();
        prediction += curr.last().context("extrapolation failed.")?;
    }
    Ok(prediction)
}

fn predict_previous_value(history: &[Num]) -> Result<Num> {
    let mut prediction: Num = *history
        .first()
        .context("Cannot predict with empty history.")?;
    let mut curr: Vec<Num> = history.iter().map(|num| num.to_owned()).collect_vec();
    while curr.iter().any(|num| *num != 0) {
        curr = curr.iter().tuple_windows().map(|(l, r)| l - r).collect();
        prediction += curr.first().context("extrapolation failed.")?;
    }
    Ok(prediction)
}

impl Solution<'_, { Day::Day9 }, { Part::One }> for AOC2023<{ Day::Day9 }> {
    type Input = Vec<History>;
    type Output = Num;

    fn solve(&self, input: &Self::Input) -> Result<Self::Output> {
        input
            .iter()
            .map(|history| predict_next_value(history))
            .sum()
    }
}

impl Solution<'_, { Day::Day9 }, { Part::Two }> for AOC2023<{ Day::Day9 }> {
    type Input = Vec<History>;
    type Output = Num;

    fn solve(&self, input: &Self::Input) -> Result<Self::Output> {
        input
            .iter()
            .map(|history| predict_previous_value(history))
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use aoc_runner::PartOneVerifier;

    use super::*;

    #[test]
    fn test_parse_input() -> Result<()> {
        let input = "0   3   6   9  12  15
1   3   6  10  15  21
10  13  16  21  30  45";
        let expected = vec![
            vec![0, 3, 6, 9, 12, 15],
            vec![1, 3, 6, 10, 15, 21],
            vec![10, 13, 16, 21, 30, 45],
        ];

        let problem = super::AOC2023::<{ Day::Day9 }>;

        assert_eq!(problem.parse_input(input)?, expected);
        Ok(())
    }

    #[test]
    fn test() -> Result<()> {
        let input = "0   3   6   9  12  15
1   3   6  10  15  21
10  13  16  21  30  45";

        let problem = super::AOC2023::<{ Day::Day9 }>;
        problem.test_part1(input, 114)
    }
}
