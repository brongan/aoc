use super::AOC2021;
use anyhow::Result;
use aoc_runner::{Day, ParseInput, Part, Solution};
use itertools::Itertools;
use std::{collections::HashSet, str::FromStr};

use aoc_runner::point2d::Point2D;

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

impl ParseInput<'_, { Day::Day13 }> for AOC2021<{ Day::Day13 }> {
    type Parsed = Manual;

    fn parse_input(&self, input: &'_ str) -> Result<Self::Parsed> {
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
        Ok(Manual {
            points,
            instructions,
        })
    }
}

fn fold(points: &mut Points, instruction: &FoldInstruction) {
    for mut point in points {
        match *instruction {
            FoldInstruction::X(x) => {
                if point.x > x {
                    point.x = 2 * x - point.x
                }
            }
            FoldInstruction::Y(y) => {
                if point.y > y {
                    point.y = 2 * y - point.y
                }
            }
        }
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
    let mut output: Vec<Vec<char>> = vec![vec!['.'; max_x + 1]; max_y + 1];
    for point in points {
        output[point.y][point.x] = '#';
    }
    format!(
        "\n{}",
        output
            .into_iter()
            .map(|row| row.into_iter().collect::<String>())
            .join("\n")
    )
}

impl Solution<'_, { Day::Day13 }, { Part::One }> for AOC2021<{ Day::Day13 }> {
    type Input = Manual;
    type Output = usize;

    fn solve(&self, input: &Self::Input) -> Result<Self::Output> {
        let mut points = input.points.clone();
        fold(
            &mut points,
            input
                .instructions
                .first()
                .expect("Need at least one instruction"),
        );

        Ok(HashSet::<Point>::from_iter(points.into_iter()).len())
    }
}

impl Solution<'_, { Day::Day13 }, { Part::Two }> for AOC2021<{ Day::Day13 }> {
    type Input = Manual;
    type Output = String;

    fn solve(&self, input: &Self::Input) -> Result<Self::Output> {
        let mut points = input.points.clone();
        input
            .instructions
            .iter()
            .for_each(|instruction| fold(&mut points, instruction));
        Ok(print_paper(&points))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_runner::PartOneVerifier;
    use aoc_runner::PartTwoVerifier;

    const EXAMPLE_INPUT: &str = "
6,10
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

    const EXAMPLE_PART2_RESULT: &str = "
#####
#...#
#...#
#...#
#####";

    #[test]
    fn test_example() -> Result<()> {
        let problem = super::AOC2021::<{ Day::Day13 }>;
        let mut parsed = problem.parse_input(EXAMPLE_INPUT)?;
        let expected_paper = "
...#..#..#.
....#......
...........
#..........
...#....#.#
...........
...........
...........
...........
...........
.#....#.##.
....#......
......#...#
#..........
#.#........";
        assert_eq!(print_paper(&parsed.points).trim(), expected_paper.trim());

        let expected_after_fold = "
#.##..#..#.
#...#......
......#...#
#...#......
.#.#..#.###";
        fold(&mut parsed.points, &parsed.instructions[0]);
        eprintln!("Actual After Fold 1:\n{}", print_paper(&parsed.points));
        assert_eq!(
            print_paper(&parsed.points).trim(),
            expected_after_fold.trim()
        );

        fold(&mut parsed.points, &parsed.instructions[1]);
        eprintln!("Actual After Fold 2:\n{}", print_paper(&parsed.points));
        assert_eq!(
            print_paper(&parsed.points).trim(),
            EXAMPLE_PART2_RESULT.trim().to_owned()
        );
        Ok(())
    }

    #[test]
    fn test() -> Result<()> {
        let problem = super::AOC2021::<{ Day::Day13 }>;

        problem.test_part1(EXAMPLE_INPUT, 17)?;
        problem.test_part2(EXAMPLE_INPUT, format!("\n{}", EXAMPLE_PART2_RESULT.trim()))
    }
}
