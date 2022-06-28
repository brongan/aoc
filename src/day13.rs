use super::AdventOfCode2021;
use crate::aoc::ParseInput;
use crate::aoc::{Day, Part, Solution};
use itertools::Itertools;
use std::{collections::HashSet, str::FromStr};

use crate::point2d::Point2D;

enum FoldInstruction {
    X(usize),
    Y(usize),
}

impl FromStr for FoldInstruction {
    type Err = std::string::ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut equation = s.chars();
        equation.advance_by(11).expect("equation too short");
        let equation = equation
            .as_str()
            .split_once('=')
            .expect("Failed to split equation");
        Ok(match equation.0 {
            "x" => FoldInstruction::X(
                equation
                    .1
                    .parse()
                    .expect("failed to parse fold equation line"),
            ),
            "y" => FoldInstruction::Y(
                equation
                    .1
                    .parse()
                    .expect("failed to parse fold equation line"),
            ),
            _ => panic!("bad equation"),
        })
    }
}

type Point = Point2D<usize>;
type Points = Vec<Point>;

pub struct Manual {
    points: Points,
    instructions: Vec<FoldInstruction>,
}

impl ParseInput<'_, { Day::Thirteen }> for AdventOfCode2021<{ Day::Thirteen }> {
    type Parsed = Manual;

    fn parse_input(&self, input: &'_ str) -> Self::Parsed {
        let input = input.trim().split_once("\n\n").expect("invalid input");
        let points: Vec<Point> = input
            .0
            .lines()
            .map(|line| Point2D::from_str(line.trim()).expect("Failed to parse point"))
            .collect();

        let instructions = input
            .1
            .lines()
            .map(|line| {
                FoldInstruction::from_str(line.trim()).expect("Failed to parse instruction")
            })
            .collect();
        Manual {
            points,
            instructions,
        }
    }
}

fn fold(points: &mut Points, instruction: &FoldInstruction) {
    for mut point in points {
        match *instruction {
            FoldInstruction::X(x) => {
                if point.x > x {
                    debug_assert!(2 * x + 1 >= point.x, "x = {}, point.x = {}", x, point.x);
                    point.x = 2 * x + 1 - point.x
                }
            }
            FoldInstruction::Y(y) => {
                if point.y > y {
                    debug_assert!(2 * y + 1 >= point.y, "y = {}, point.y = {}", y, point.y);
                    point.y = 2 * y + 1 - point.y
                }
            }
        }
    }
}

impl Solution<'_, { Day::Thirteen }, { Part::One }> for AdventOfCode2021<{ Day::Thirteen }> {
    type Input = Manual;
    type Output = usize;

    fn solve(&self, input: &Self::Input) -> Self::Output {
        let mut points = input.points.clone();
        fold(
            &mut points,
            input
                .instructions
                .first()
                .expect("Need at least one instruction"),
        );
        HashSet::<Point>::from_iter(points.into_iter()).len()
    }
}

fn print_paper(points: &Points) -> String {
    let max_x = points
        .iter()
        .map(|p| p.x)
        .max()
        .expect("need a point after folding");

    let max_y = points
        .iter()
        .map(|p| p.y)
        .max()
        .expect("need a point after folding");
    let mut output: Vec<Vec<char>> = vec![vec!['.'; max_y + 1]; max_x + 1];
    for point in points {
        output[point.x][point.y] = '#';
    }
    format!("\n{}", output
        .into_iter()
        .map(|row| row.into_iter().collect::<String>())
        .join("\n"))
}

impl Solution<'_, { Day::Thirteen }, { Part::Two }> for AdventOfCode2021<{ Day::Thirteen }> {
    type Input = Manual;
    type Output = String;

    fn solve(&self, input: &Self::Input) -> Self::Output {
        let mut points = input.points.clone();
        input
            .instructions
            .iter()
            .for_each(|instruction| fold(&mut points, instruction));
        print_paper(&points)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aoc::PartOneVerifier;

    #[test]
    fn test() {
        let input = "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5";
        let problem = super::AdventOfCode2021::<{ Day::Thirteen }>;
        (&&&problem).test_part1(input, 17);
    }
}
