use super::AOC2015;

use aoc_runner::point2d::Point2D;
use aoc_runner::{Day, ParseInput, Part, Solution};

use anyhow::anyhow;
use anyhow::Result;
use std::collections::HashSet;

impl ParseInput<'_, { Day::Day3 }> for AOC2015<{ Day::Day3 }> {
    type Parsed = String;

    fn parse_input(&self, input: &'_ str) -> Result<Self::Parsed> {
        Ok(input.to_string())
    }
}

impl Solution<'_, { Day::Day3 }, { Part::One }> for AOC2015<{ Day::Day3 }> {
    type Input = String;
    type Output = usize;

    fn solve(&self, input: &Self::Input) -> Result<Self::Output> {
        let mut x = 0;
        let mut y = 0;
        let mut visited = std::collections::HashSet::from([(0, 0)]);
        for char in input.chars() {
            match char {
                '^' => y += 1,
                'v' => y -= 1,
                '>' => x += 1,
                '<' => x -= 1,
                _ => return Err(anyhow!("Invalid character")),
            }
            visited.insert((x, y));
        }
        Ok(visited.len())
    }
}

impl Solution<'_, { Day::Day3 }, { Part::Two }> for AOC2015<{ Day::Day3 }> {
    type Input = String;
    type Output = usize;

    fn solve(&self, input: &Self::Input) -> Result<Self::Output> {
        let mut santa = Point2D { x: 0, y: 0 };
        let mut robot = santa;
        let mut visited = HashSet::from([santa]);
        for (i, char) in input.chars().enumerate() {
            match (i % 2, char) {
                (0, '^') => santa.y += 1,
                (1, '^') => robot.y += 1,
                (0, 'v') => santa.y -= 1,
                (1, 'v') => robot.y -= 1,
                (0, '>') => santa.x += 1,
                (1, '>') => robot.x += 1,
                (0, '<') => santa.x -= 1,
                (1, '<') => robot.x -= 1,
                _ => return Err(anyhow!("Invalid character")),
            }
            visited.insert(santa);
            visited.insert(robot);
        }
        Ok(visited.len())
    }
}
