use anyhow::Context;
use itertools::Itertools;
use std::str::FromStr;

use super::AOC2025;
use anyhow::Result;
use aoc_runner::point2d::{Point2D, manhattan_area_inclusive};
use aoc_runner::{Day, ParseInput, Part, Solution};

type IR = Vec<Point2D<i64>>;

impl ParseInput<'_, { Day::Day9 }> for AOC2025<{ Day::Day9 }> {
    type Parsed = IR;

    fn parse_input(&self, input: &'_ str) -> Result<Self::Parsed> {
        input
            .trim()
            .lines()
            .map(Point2D::<i64>::from_str)
            .map(|res| res.map_err(anyhow::Error::new))
            .collect()
    }
}

impl Solution<'_, { Day::Day9 }, { Part::One }> for AOC2025<{ Day::Day9 }> {
    type Input = IR;
    type Output = i64;

    fn solve(&self, input: &Self::Input) -> Result<Self::Output> {
        input
            .iter()
            .combinations(2)
            .map(|pair| manhattan_area_inclusive(pair[0], pair[1]))
            .max()
            .context("nonempty input")
    }
}
