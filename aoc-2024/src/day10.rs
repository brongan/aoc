use std::collections::HashSet;

use super::AOC2024;
use anyhow::{Context, Result};
use aoc_runner::{point2d::Point2D, Day, ParseInput, Part, Solution};

type Num = u32;
type IR = Vec<Vec<Num>>;
type Point = Point2D<i32>;

impl ParseInput<'_, { Day::Day10 }> for AOC2024<{ Day::Day10 }> {
    type Parsed = IR;

    fn parse_input(&self, input: &'_ str) -> Result<Self::Parsed> {
        input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|digit| digit.to_digit(10).context("Expected Digit"))
                    .collect()
            })
            .collect()
    }
}

fn reachable_peaks(x: i32, y: i32, num: u32, input: &IR) -> HashSet<Point> {
    let height = input.len() as i32;
    let width = input[0].len() as i32;

    if x < 0 || x >= width || y < 0 || y >= height || input[y as usize][x as usize] != num {
        return HashSet::new();
    }
    if num == 9 {
        return HashSet::from([Point { x, y }]);
    }
    let mut ret = HashSet::new();
    for (dx, dy) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
        let new_x = x + dx;
        let new_y = y + dy;
        ret.extend(reachable_peaks(new_x, new_y, num + 1, input));
    }
    ret
}

fn trailhead_score(x: i32, y: i32, num: u32, input: &IR) -> u32 {
    let height = input.len() as i32;
    let width = input[0].len() as i32;

    if x < 0 || x >= width || y < 0 || y >= height || input[y as usize][x as usize] != num {
        return 0;
    }
    if num == 9 {
        return 1;
    }
    let mut ret = 0;
    for (dx, dy) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
        let new_x = x + dx;
        let new_y = y + dy;
        ret += trailhead_score(new_x, new_y, num + 1, input);
    }
    ret
}
impl Solution<'_, { Day::Day10 }, { Part::One }> for AOC2024<{ Day::Day10 }> {
    type Input = IR;
    type Output = usize;

    fn solve(&self, input: &Self::Input) -> Result<Self::Output> {
        let height = input.len();
        let width = input[0].len();
        Ok((0..width)
            .map(|x| (0..height).map(move |y| reachable_peaks(x as i32, y as i32, 0, &input).len()))
            .flatten()
            .sum())
    }
}

impl Solution<'_, { Day::Day10 }, { Part::Two }> for AOC2024<{ Day::Day10 }> {
    type Input = IR;
    type Output = Num;

    fn solve(&self, input: &Self::Input) -> Result<Self::Output> {
        let height = input.len();
        let width = input[0].len();
        Ok((0..width)
            .map(|x| (0..height).map(move |y| trailhead_score(x as i32, y as i32, 0, &input)))
            .flatten()
            .sum())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_runner::PartOneVerifier;
    use aoc_runner::PartTwoVerifier;

    #[test]
    fn test_baby() -> Result<()> {
        let problem = super::AOC2024::<{ Day::Day10 }>;
        problem.test_part1(
            "0123
1234
8765
9876",
            1,
        )?;
        Ok(())
    }

    #[test]
    fn test_example() -> Result<()> {
        let problem = super::AOC2024::<{ Day::Day10 }>;
        let input = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";
        problem.parse_input(input)?;
        problem.test_part1(input, 36)?;
        problem.test_part2(input, 81)?;

        Ok(())
    }
}
