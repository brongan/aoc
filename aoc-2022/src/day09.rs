use std::collections::HashSet;

use super::AOC2022;
use aoc_runner::point2d::Point2D;
use aoc_runner::{Day, ParseInput, Part, Solution};

use anyhow::Result;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{newline, space1};
use nom::combinator::{map, value};
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use nom::IResult;

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn parse(input: &str) -> IResult<&str, Self> {
        alt((
            value(Direction::Up, tag("U")),
            value(Direction::Down, tag("D")),
            value(Direction::Left, tag("L")),
            value(Direction::Right, tag("R")),
        ))(input)
    }
}

pub struct Motion {
    direction: Direction,
    distance: i32,
}

impl Motion {
    fn parse(input: &str) -> IResult<&str, Self> {
        map(
            separated_pair(Direction::parse, space1, nom::character::complete::i32),
            |(direction, distance)| Self {
                direction,
                distance,
            },
        )(input)
    }
}

type Point = Point2D<i32>;
type State = Vec<Point>;
type Instruction = Vec<Motion>;

impl ParseInput<'_, { Day::Day9 }> for AOC2022<{ Day::Day9 }> {
    type Parsed = Instruction;

    fn parse_input(&self, input: &'_ str) -> Result<Self::Parsed> {
        let (_, motions) =
            separated_list1(newline, Motion::parse)(input).map_err(|e| e.to_owned())?;
        Ok(motions)
    }
}

fn pull(lead: &Point, follow: &mut Point, visited: &mut HashSet<Point>, update: bool) {
    loop {
        let delta = *lead - *follow;

        if delta.x.abs() <= 1 && delta.y.abs() <= 1 {
            break;
        }

        if delta.x.abs() > 1 && delta.y == 0 {
            follow.x += delta.x.signum();
        } else if delta.y.abs() > 1 && delta.x == 0 {
            follow.y += delta.y.signum();
        } else {
            follow.x += delta.x.signum();
            follow.y += delta.y.signum();
        }

        if update {
            eprintln!("The tail visited: {follow}");
            visited.insert(*follow);
        }
    }
}

fn follow_motion(motion: &Motion, rope: &mut State, visited: &mut HashSet<Point>) {
    let head = rope.first_mut().unwrap();
    match motion.direction {
        Direction::Up => head.y += motion.distance,
        Direction::Down => head.y -= motion.distance,
        Direction::Left => head.x -= motion.distance,
        Direction::Right => head.x += motion.distance,
    }

    let rope_len = rope.len();
    for i in 1..rope_len {
        let (left, right) = rope.split_at_mut(i);
        let lead = left.last().unwrap();
        let follow = right.first_mut().unwrap();
        pull(lead, follow, visited, i + 1 == rope_len);
    }
}

fn follow_motions(mut rope: State, instructions: &[Motion]) -> usize {
    let mut visited = HashSet::from([Point2D::new(0, 0)]);
    for instruction in instructions {
        follow_motion(instruction, &mut rope, &mut visited);
    }
    eprintln!("Visited: {visited:?}");
    visited.len()
}

impl Solution<'_, { Day::Day9 }, { Part::One }> for AOC2022<{ Day::Day9 }> {
    type Input = Instruction;
    type Output = usize;

    fn solve(&self, input: &Self::Input) -> Result<Self::Output> {
        Ok(follow_motions(vec![Point2D::new(0, 0); 2], input))
    }
}

impl Solution<'_, { Day::Day9 }, { Part::Two }> for AOC2022<{ Day::Day9 }> {
    type Input = Instruction;
    type Output = usize;

    fn solve(&self, input: &Self::Input) -> Result<Self::Output> {
        Ok(follow_motions(vec![Point2D::new(0, 0); 10], input))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_runner::PartOneVerifier;
    use aoc_runner::PartTwoVerifier;

    #[test]
    fn test_parsing() {}

    #[test]
    fn test() -> Result<()> {
        let input = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";
        let problem = super::AOC2022::<{ Day::Day9 }>;
        problem.test_part1(input, 13)?;
        problem.test_part2(input, 1)?;

        let input2 = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";
        problem.test_part2(input2, 36)
    }
}
