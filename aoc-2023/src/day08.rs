use super::AOC2023;
use anyhow::Result;
use aoc_runner::{Day, ParseInput, Part, Solution};
use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    character::complete::{multispace0, newline},
    combinator::{all_consuming, map},
    multi::{many1, separated_list0},
    sequence::{delimited, separated_pair},
    IResult,
};
use std::collections::HashMap;

pub enum Instruction {
    Left,
    Right,
}
type Node = [char; 3];

pub struct MapElement {
    val: Node,
    left: Node,
    right: Node,
}

fn parse_element(input: &str) -> IResult<&str, Node> {
    map(take(3usize), |s: &str| {
        let s = s.as_bytes();
        [s[0] as char, s[1] as char, s[2] as char]
    })(input)
}

fn parse_pair(input: &str) -> IResult<&str, (Node, Node)> {
    delimited(
        tag("("),
        separated_pair(parse_element, tag(", "), parse_element),
        tag(")"),
    )(input)
}

fn parse_node(input: &str) -> IResult<&str, MapElement> {
    map(
        separated_pair(
            parse_element,
            delimited(multispace0, tag("="), multispace0),
            parse_pair,
        ),
        |(val, choice)| MapElement {
            val,
            left: choice.0,
            right: choice.1,
        },
    )(input)
}

pub struct Puzzle {
    instructions: Vec<Instruction>,
    mapping: HashMap<Node, (Node, Node)>,
}

fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
    let left = |input| map(tag("L"), |_| Instruction::Left)(input);
    let right = |input| map(tag("R"), |_| Instruction::Right)(input);
    alt((left, right))(input)
}

fn parse_instructions(input: &str) -> IResult<&str, Vec<Instruction>> {
    many1(parse_instruction)(input)
}

fn parse_puzzle(input: &str) -> IResult<&str, Puzzle> {
    map(
        separated_pair(
            parse_instructions,
            tag("\n\n"),
            separated_list0(newline, parse_node),
        ),
        |(instructions, nodes)| Puzzle {
            instructions,
            mapping: nodes
                .iter()
                .map(|node| {
                    (
                        node.val.to_owned(),
                        (node.left.to_owned(), node.right.to_owned()),
                    )
                })
                .collect(),
        },
    )(input)
}

impl ParseInput<'_, { Day::Day8 }> for AOC2023<{ Day::Day8 }> {
    type Parsed = Puzzle;

    fn parse_input(&self, input: &'_ str) -> Result<Self::Parsed> {
        let (_, game) = all_consuming(parse_puzzle)(input.trim()).map_err(|e| e.to_owned())?;
        Ok(game)
    }
}

impl Solution<'_, { Day::Day8 }, { Part::One }> for AOC2023<{ Day::Day8 }> {
    type Input = Puzzle;
    type Output = usize;

    fn solve(&self, input: &Self::Input) -> Result<Self::Output> {
        let mut steps = 0;
        let mut curr: Node = ['A', 'A', 'A'];
        while curr != ['Z', 'Z', 'Z'] {
            curr = match input.instructions[steps % input.instructions.len()] {
                Instruction::Left => input.mapping[&curr].0,
                Instruction::Right => input.mapping[&curr].1,
            };
            steps += 1;
        }
        Ok(steps)
    }
}

fn count_until_z(
    node: &Node,
    instructions: &[Instruction],
    mapping: &HashMap<Node, (Node, Node)>,
) -> usize {
    let mut node = *node;
    let mut steps = 0;
    while node[2] != 'Z' {
        node = match instructions[steps % instructions.len()] {
            Instruction::Left => mapping[&node].0,
            Instruction::Right => mapping[&node].1,
        };
        steps += 1;
    }
    steps
}

impl Solution<'_, { Day::Day8 }, { Part::Two }> for AOC2023<{ Day::Day8 }> {
    type Input = Puzzle;
    type Output = usize;

    fn solve(&self, input: &Self::Input) -> Result<Self::Output> {
        Ok(input
            .mapping
            .keys()
            .filter(|s| s[2] == 'A')
            .map(|node| count_until_z(node, &input.instructions, &input.mapping))
            .fold(1, num::integer::lcm))
    }
}

#[cfg(test)]
mod tests {
    use aoc_runner::PartOneVerifier;
    use aoc_runner::PartTwoVerifier;

    use super::*;

    #[test]
    fn test() -> Result<()> {
        let input = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";
        let problem = super::AOC2023::<{ Day::Day8 }>;
        problem.test_part1(input, 2)?;
        let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
        problem.test_part2(input, 6)
    }
}
