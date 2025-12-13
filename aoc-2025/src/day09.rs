use super::AOC2025;
use anyhow::Result;
use aoc_runner::{Day, ParseInput, Part, Solution, point2d::Point2D};

type IR = Vec<Point2D<i32>>;
type Num = usize;

impl ParseInput<'_, { Day::Day9 }> for AOC2025<{ Day::Day9 }> {
    type Parsed = IR;

    fn parse_input(&self, input: &'_ str) -> Result<Self::Parsed> {
        todo!()
    }
}

impl Solution<'_, { Day::Day9 }, { Part::One }> for AOC2025<{ Day::Day9 }> {
    type Input = IR;
    type Output = Num;

    fn solve(&self, input: &Self::Input) -> Result<Self::Output> {
        todo!()
    }
}
