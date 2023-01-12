use std::{rc::Rc, str::FromStr};

use super::AOC2022;
use aoc_runner::{Day, ParseInput, Part, Solution};

use anyhow::{Context, Result};
use nom::{
    character::complete::{anychar, newline},
    combinator::map_res,
    multi::{many1, separated_list1},
    Finish, IResult,
};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

type Tree = u8;
type Forest = Vec<Vec<Tree>>;
#[derive(Debug)]
pub struct ForestView(Rc<Forest>);

fn parse_tree(s: &str) -> IResult<&str, Tree> {
    map_res(anychar, |c| {
        c.to_digit(10)
            .map(|n| n as Tree)
            .context("Failed to parse digit.")
    })(s)
}

fn parse_forest(s: &str) -> IResult<&str, Forest> {
    separated_list1(newline, many1(parse_tree))(s)
}

impl FromStr for ForestView {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, forest) = parse_forest(s).map_err(|e| e.to_owned()).finish()?;
        Ok(ForestView(Rc::new(forest)))
    }
}

impl ParseInput<'_, { Day::Day8 }> for AOC2022<{ Day::Day8 }> {
    type Parsed = ForestView;

    fn parse_input(&self, input: &'_ str) -> Result<Self::Parsed> {
        input.parse()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Visibility {
    visible: bool,
    distance: usize,
}

impl ForestView {
    fn visibility(&self, x: usize, y: usize, direction: Direction) -> Visibility {
        let forest = &self.0;
        let tree = forest[x][y];
        let trees_between = |direction| -> Box<dyn Iterator<Item = u8>> {
            match direction {
                Direction::Up => Box::new((0..y).rev().map(move |y| forest[x][y])),
                Direction::Down => Box::new(((y + 1)..forest[x].len()).map(move |y| forest[x][y])),
                Direction::Left => Box::new((0..x).rev().map(move |x| forest[x][y])),
                Direction::Right => Box::new(((x + 1)..forest.len()).map(move |x| forest[x][y])),
            }
        };

        let mut distance = 0;
        for other in trees_between(direction) {
            distance += 1;
            if other >= tree {
                return Visibility {
                    visible: false,
                    distance,
                };
            }
        }
        Visibility {
            visible: true,
            distance,
        }
    }

    fn is_visible(&self, x: usize, y: usize) -> bool {
        Direction::iter()
            .map(|dir| self.visibility(x, y, dir).visible)
            .any(|v| v)
    }

    fn iterator(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        (0..self.0.len()).flat_map(move |x| (0..self.0[x].len()).map(move |y| (x, y)))
    }

    fn tree_score(&self, x: usize, y: usize) -> usize {
        Direction::iter()
            .map(|dir| self.visibility(x, y, dir).distance)
            .product()
    }

    fn scenic_score(&self) -> Result<usize> {
        self.iterator()
            .map(|(x, y)| self.tree_score(x, y))
            .max()
            .context("No trees in the forest!")
    }
}

impl Solution<'_, { Day::Day8 }, { Part::One }> for AOC2022<{ Day::Day8 }> {
    type Input = ForestView;
    type Output = usize;

    fn solve(&self, input: &Self::Input) -> Result<Self::Output> {
        Ok(input
            .iterator()
            .map(|(x, y)| input.is_visible(x, y))
            .filter(|&b| b)
            .count())
    }
}

impl Solution<'_, { Day::Day8 }, { Part::Two }> for AOC2022<{ Day::Day8 }> {
    type Input = ForestView;
    type Output = usize;

    fn solve(&self, input: &Self::Input) -> Result<Self::Output> {
        input.scenic_score()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_runner::PartOneVerifier;

    #[test]
    fn test_parsing() {}

    #[test]
    fn test() -> Result<()> {
        let input = "30373
25512
65332
33549
35390";
        let problem = super::AOC2022::<{ Day::Day8 }>;
        problem.test_part1("1", 1)?;
        problem.test_part1("12\n34", 4)?;
        problem.test_part1(input, 21)?;
        let parsed = problem.parse_input(input)?;
        assert_eq!(parsed.0[3][2], 5, "{parsed:?}");
        assert_eq!(parsed.tree_score(3, 2), 8);
        Ok(())
    }
}
