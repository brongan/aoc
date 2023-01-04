use std::collections::HashSet;
use std::str::FromStr;

use super::AOC2022;
use aoc_runner::point2d::Point2D;
use aoc_runner::{Day, ParseInput, Part, Solution};

use anyhow::{Context, Result};
use nom::character::complete::{alpha0, digit0, newline, space1};
use nom::combinator::{map, map_res};
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use nom::IResult;

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl FromStr for Direction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "U" => Ok(Direction::Up),
            "D" => Ok(Direction::Down),
            "L" => Ok(Direction::Left),
            "R" => Ok(Direction::Right),
            _ => Err(anyhow::anyhow!("Invalid direction: {s}")),
        }
    }
}

pub struct Instruction {
    direction: Direction,
    distance: i32,
}

type State = Vec<Point2D<i32>>;

fn parse_instructions(s: &str) -> IResult<&str, Vec<Instruction>> {
    let parse_direction = map_res(alpha0, Direction::from_str);
    let parse_distance = map_res(digit0, |s: &str| s.parse());
    let parse_instruction = map(
        separated_pair(parse_direction, space1, parse_distance),
        |(direction, distance)| Instruction {
            direction,
            distance,
        },
    );
    separated_list1(newline, parse_instruction)(s)
}

impl ParseInput<'_, { Day::Day9 }> for AOC2022<{ Day::Day9 }> {
    type Parsed = Vec<Instruction>;

    fn parse_input(&self, input: &'_ str) -> Result<Self::Parsed> {
        let (_, instructions) = parse_instructions(input).map_err(|e| e.to_owned())?;
        Ok(instructions)
    }
}

fn simulate_instruction(
    rope: &mut State,
    visited: &mut HashSet<Point2D<i32>>,
    instruction: &Instruction,
) -> Result<()> {
    let head = &mut rope.get_mut(0).context("Empty rope.")?;
    match instruction.direction {
        Direction::Up => head.y += instruction.distance,
        Direction::Down => head.y -= instruction.distance,
        Direction::Left => head.x -= instruction.distance,
        Direction::Right => head.x += instruction.distance,
    }

    let rope = &mut rope.as_mut_slice();
    for i in 1..rope.len() {
        let (first, second) = rope.split_at_mut(i);
        let lead = first[0];
        let mut follow = second[0];

        loop {
            let delta_x = lead.x - follow.x;
            let delta_y = lead.y - follow.y;

            if delta_x.abs() <= 1 && delta_y.abs() <= 1 {
                break;
            }

            if delta_x.abs() > 1 && delta_y == 0 {
                follow.x += delta_x.signum();
            } else if delta_y.abs() > 1 && delta_x == 0 {
                follow.y += delta_y.signum();
            } else {
                follow.x += delta_x.signum();
                follow.y += delta_y.signum();
            }
            visited.insert(follow);
        }
    }
    Ok(())
}

fn simulate_instructions(mut rope: State, instructions: &[Instruction]) -> Result<usize> {
    let mut visited = HashSet::from([Point2D::new(0, 0)]);
    for instruction in instructions {
        simulate_instruction(&mut rope, &mut visited, instruction)?;
    }
    Ok(visited.len())
}

impl Solution<'_, { Day::Day9 }, { Part::One }> for AOC2022<{ Day::Day9 }> {
    type Input = Vec<Instruction>;
    type Output = usize;

    fn solve(&self, input: &Self::Input) -> Result<Self::Output> {
        simulate_instructions(vec![Point2D::new(0, 0); 2], input)
    }
}

/*
impl Solution<'_, { Day::Day9 }, { Part::Two }> for AOC2022<{ Day::Day9 }> {
    type Input = Vec<Vec<u32>>;
    type Output = u32;

    fn solve(&self, input: &Self::Input) -> Result<Self::Output> {
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
        let input = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";
        let problem = super::AOC2022::<{ Day::Day9 }>;
        problem.test_part1(input, 13)
    }
}
