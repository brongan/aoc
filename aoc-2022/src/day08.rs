use std::rc::Rc;

use super::AOC2022;
use aoc_runner::{Day, ParseInput, Part, Solution};

use anyhow::{Context, Result};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

type Tree = u8;
type Forest = Vec<Vec<Tree>>;
type ForestView = Rc<Vec<Vec<u8>>>;

impl ParseInput<'_, { Day::Day8 }> for AOC2022<{ Day::Day8 }> {
    type Parsed = ForestView;

    fn parse_input(&self, input: &'_ str) -> Result<Self::Parsed> {
        Ok(Rc::new(
            input
                .split('\n')
                .map(|line| -> Result<Vec<u8>, anyhow::Error> {
                    line.split('\n')
                        .collect::<String>()
                        .chars()
                        .map(|c| c.to_digit(10).context("Invalid digit").map(|c| c as u8))
                        .collect()
                })
                .collect::<Result<Forest>>()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn is_visible(input: &ForestView, x: usize, y: usize) -> bool {
    let tree = input[x][y];
    let trees_between = |direction| -> Box<dyn Iterator<Item = u8>> {
        match direction {
            Direction::Up => Box::new((0..y).map(move |y| input[x][y])),
            Direction::Down => Box::new((y..input[x].len()).map(move |y| input[x][y])),
            Direction::Left => Box::new((0..x).map(move |x| input[x][y])),
            Direction::Right => Box::new((x..input.len()).map(move |x| input[x][y])),
        }
    };

    println!("is_visible");

    for direction in Direction::iter() {
        for other in trees_between(direction) {
            println!("Tree: {}, other: {}", tree, other);
            if other >= tree {
                break;
            }
        }
        println!("({},{}) is visible!", x, y);
        return true;
    }
    false
}

impl Solution<'_, { Day::Day8 }, { Part::One }> for AOC2022<{ Day::Day8 }> {
    type Input = ForestView;
    type Output = usize;

    fn solve(&self, input: &Self::Input) -> Result<Self::Output> {
        let mut visibile = vec![vec![false; input[0].len()]; input.len()];
        for (x, row) in input.iter().enumerate() {
            for (y, _) in row.iter().enumerate() {
                println!("fuck this");
                visibile[x][y] = is_visible(input, x, y);
            }
        }
        Ok(visibile.iter().flatten().filter(|&&v| v).count())
    }
}

/*
impl Solution<'_, { Day::Day8 }, { Part::Two }> for AOC2022<{ Day::Day8 }> {
    type Input = Input;
    type Output = u32;

    fn solve(&self, input: &Self::Input) -> Result<Self::Output> {
        Ok(sorted(input.iter().map(|elf| elf.iter().sum::<u32>()))
            .rev()
            .take(3)
            .sum())
    }
}
*/

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
        let problem = super::AOC2022::<{ Day::Day7 }>;
        problem.test_part1(input, 21)
    }
}
