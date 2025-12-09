use super::AOC2025;
use anyhow::Result;
use aoc_runner::{Day, ParseInput, Part, Solution};

#[derive(Copy, Clone)]
enum Operator {
    Multiply,
    Add,
}

pub struct Problem {
    operands: Vec<Num>,
    operator: Operator,
}

impl Problem {
    fn solve(&self) -> Num {
        match self.operator {
            Operator::Multiply => self.operands.iter().product(),
            Operator::Add => self.operands.iter().sum(),
        }
    }
}

type IR = Vec<Problem>;
type Num = u64;

impl ParseInput<'_, { Day::Day6 }> for AOC2025<{ Day::Day6 }> {
    type Parsed = IR;

    fn parse_input(&self, input: &'_ str) -> Result<Self::Parsed> {
        let input: Vec<&str> = input.trim().lines().collect();
        let (operators, operands) = input.split_last().unwrap();
        let operands: Vec<Vec<Num>> = operands
            .iter()
            .map(|line| {
                line.split_whitespace()
                    .map(|num| Num::from_str_radix(num, 10).unwrap())
                    .collect()
            })
            .collect();
        let operators: Vec<Operator> = operators
            .split_whitespace()
            .map(|c| match c {
                "*" => Operator::Multiply,
                "+" => Operator::Add,
                _ => unreachable!("owo"),
            })
            .collect();
        Ok((0..(operators.len()))
            .map(|i| {
                let operator = operators[i];
                let operands = operands.iter().map(|operands| operands[i]).collect();
                Problem { operands, operator }
            })
            .collect())
    }
}

impl Solution<'_, { Day::Day6 }, { Part::One }> for AOC2025<{ Day::Day6 }> {
    type Input = IR;
    type Output = Num;

    fn solve(&self, input: &Self::Input) -> Result<Self::Output> {
        Ok(input.iter().map(|problem| problem.solve()).sum())
    }
}
