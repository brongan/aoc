use super::AOC2024;
use anyhow::anyhow;
use anyhow::Result;
use aoc_runner::{Day, ParseInput, Part, Solution};
use itertools::Itertools;
use std::num::ParseIntError;
use std::str::FromStr;

type Num = i32;
type Report = Vec<Num>;

impl ParseInput<'_, { Day::Day2 }> for AOC2024<{ Day::Day2 }> {
    type Parsed = Vec<Report>;

    fn parse_input(&self, input: &'_ str) -> Result<Self::Parsed> {
        input
            .lines()
            .map(|line| {
                line.split_whitespace()
                    .map(Num::from_str)
                    .collect::<Result<Vec<_>, ParseIntError>>()
                    .map_err(|_| anyhow!("Bad int."))
            })
            .collect::<Result<Vec<_>>>()
    }
}

fn safe_report(report: &[Num]) -> bool {
    let increasing = report
        .iter()
        .tuple_windows()
        .all(|(l, r)| r > l && r.abs_diff(*l) <= 3);
    let decreasing = report
        .iter()
        .tuple_windows()
        .all(|(l, r)| l > r && l.abs_diff(*r) <= 3);
    increasing || decreasing
}

fn safe_dampened_report(report: &[Num]) -> bool {
    if safe_report(report) {
        return true;
    }
    for i in 0..report.len() {
        let mut report = report.to_owned();
        report.remove(i);
        if safe_report(&report) {
            return true;
        }
    }
    false
}

impl Solution<'_, { Day::Day2 }, { Part::One }> for AOC2024<{ Day::Day2 }> {
    type Input = Vec<Report>;
    type Output = usize;

    fn solve(&self, input: &Self::Input) -> Result<Self::Output> {
        Ok(input
            .into_iter()
            .map(|report| safe_report(report))
            .filter(|b| *b)
            .count())
    }
}

impl Solution<'_, { Day::Day2 }, { Part::Two }> for AOC2024<{ Day::Day2 }> {
    type Input = Vec<Report>;
    type Output = usize;

    fn solve(&self, input: &Self::Input) -> Result<Self::Output> {
        Ok(input
            .into_iter()
            .map(|report| safe_dampened_report(report))
            .filter(|b| *b)
            .count())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        todo!()
    }
}
