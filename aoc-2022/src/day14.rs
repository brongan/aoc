use super::AOC2022;
use anyhow::{Context, Result};
use aoc_runner::{
    point2d::{recognize_point2d, Point2D},
    Day, ParseInput, Part, Solution,
};
use nom::{
    bytes::complete::tag,
    character::complete::{multispace0, newline},
    combinator::{all_consuming, map_res, opt},
    multi::separated_list1,
    sequence::{delimited, terminated},
    IResult,
};
use std::{
    cmp::{max, min},
    collections::HashMap,
    str::FromStr,
};

#[derive(Debug, Eq, PartialEq)]
enum Unit {
    Air,
    Sand,
    Rock,
}

type Line = Vec<Point2D<usize>>;

fn parse_line(input: &str) -> IResult<&str, Line> {
    separated_list1(
        delimited(multispace0, tag("->"), multispace0),
        map_res(recognize_point2d, Point2D::from_str),
    )(input)
}

impl ParseInput<'_, { Day::Day14 }> for AOC2022<{ Day::Day14 }> {
    type Parsed = Vec<Line>;

    fn parse_input(&self, input: &'_ str) -> Result<Self::Parsed> {
        let parse_lines = |input| separated_list1(newline, parse_line)(input);
        let (_, lines) = all_consuming(terminated(parse_lines, opt(newline)))(input)
            .map_err(|e| e.to_owned())?;
        Ok(lines)
    }
}

fn create_world(input: &[Line]) -> HashMap<Point2D<usize>, Unit> {
    let mut world: HashMap<Point2D<usize>, Unit> = HashMap::new();
    for line in input {
        for edge in line.windows(2) {
            let point1 = edge[0];
            let point2 = edge[1];
            if point1.x == point2.x {
                let l = min(point1.y, point2.y);
                let r = max(point1.y, point2.y);
                for y in l..r + 1 {
                    world.insert(Point2D { x: point1.x, y }, Unit::Rock);
                }
            } else {
                let l = min(point1.x, point2.x);
                let r = max(point1.x, point2.x);

                for x in l..r + 1 {
                    world.insert(Point2D { x, y: point1.y }, Unit::Rock);
                }
            }
        }
    }
    world
}

impl Solution<'_, { Day::Day14 }, { Part::One }> for AOC2022<{ Day::Day14 }> {
    type Input = Vec<Line>;
    type Output = u64;

    fn solve(&self, input: &Self::Input) -> Result<Self::Output> {
        let mut world = create_world(&input);
        let max_y = *world
            .keys()
            .map(|Point2D { x: _x, y }| y)
            .max()
            .context("No rocks.")?;
        let mut num_sand = 0;
        loop {
            let mut curr = Point2D { x: 500, y: 0 };
            num_sand += 1;
            loop {
                if curr.y >= max_y {
                    return Ok(num_sand - 1);
                }

                println!("{curr}");
                let down = Point2D {
                    x: curr.x,
                    y: curr.y + 1,
                };
                let left = Point2D {
                    x: curr.x - 1,
                    y: curr.y + 1,
                };
                let right = Point2D {
                    x: curr.x + 1,
                    y: curr.y + 1,
                };
                curr = if world.get(&down).unwrap_or(&Unit::Air) == &Unit::Air {
                    world.insert(curr, Unit::Air);
                    down
                } else if world.get(&left).unwrap_or(&Unit::Air) == &Unit::Air {
                    world.insert(curr, Unit::Air);
                    left
                } else if world.get(&right).unwrap_or(&Unit::Air) == &Unit::Air {
                    world.insert(curr, Unit::Air);
                    right
                } else {
                    world.insert(curr, Unit::Sand);
                    break;
                };
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_runner::PartOneVerifier;

    #[test]
    fn test_parse_line() -> Result<()> {
        let expected1 = Vec::from([
            Point2D { x: 498, y: 4 },
            Point2D { x: 498, y: 6 },
            Point2D { x: 496, y: 6 },
        ]);
        assert_eq!(parse_line("498,4 -> 498,6 -> 496,6")?, ("", expected1));
        Ok(())
    }

    #[test]
    fn test() -> Result<()> {
        let problem = super::AOC2022::<{ Day::Day14 }>;
        let expected1 = Vec::from([
            Point2D { x: 498, y: 4 },
            Point2D { x: 498, y: 6 },
            Point2D { x: 496, y: 6 },
        ]);
        assert_eq!(
            parse_line("498,4 -> 498,6 -> 496,6")?,
            ("", expected1.clone())
        );
        let input = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";
        let parsed = problem.parse_input(&input)?;
        assert_eq!(
            parsed,
            Vec::from([
                expected1,
                Vec::from([
                    Point2D { x: 503, y: 4 },
                    Point2D { x: 502, y: 4 },
                    Point2D { x: 502, y: 9 },
                    Point2D { x: 494, y: 9 },
                ])
            ])
        );
        problem.test_part1(&input, 24)?;
        Ok(())
    }
}
