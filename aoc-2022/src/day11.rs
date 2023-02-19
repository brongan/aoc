use super::AOC2022;
use aoc_runner::{Day, ParseInput, Part, Solution};
use counter::Counter;
use std::{collections::VecDeque, str::FromStr};

use anyhow::anyhow;
use anyhow::Result;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_till, take_until},
    character::complete::{digit1, newline, space0},
    combinator::{map, map_res, value},
    multi::separated_list1,
    sequence::{delimited, pair, preceded, terminated, tuple},
    IResult,
};

#[derive(Clone, Debug, PartialEq)]
enum Operand {
    Number(u64),
    Old,
}

impl Operand {
    fn parse(input: &str) -> IResult<&str, Self> {
        alt((
            map_res(digit1, |s: &str| s.parse().map(|n| Operand::Number(n))),
            value(Operand::Old, tag("old")),
        ))(input)
    }
}

#[derive(Clone, Debug, PartialEq)]
enum Operator {
    Add,
    Multiply,
}

impl Operator {
    fn parse(input: &str) -> IResult<&str, Self> {
        alt((
            value(Operator::Add, tag("+")),
            value(Operator::Multiply, tag("*")),
        ))(input)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Operation {
    left: Operand,
    operator: Operator,
    right: Operand,
}

impl Operation {
    fn parse(input: &str) -> IResult<&str, Self> {
        map(
            preceded(
                pair(space0, tag("Operation: new = ")),
                tuple((
                    Operand::parse,
                    delimited(space0, Operator::parse, space0),
                    Operand::parse,
                )),
            ),
            |(left, operator, right)| Operation {
                left,
                operator,
                right,
            },
        )(input)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Test {
    divisible_by: u64,
    truthy_index: usize,
    falsy_index: usize,
}

fn parse_line<'a, T: FromStr>(input: &'a str) -> IResult<&'a str, T> {
    let (input, _) = take_till(|c: char| c.is_digit(10))(input)?;
    map_res(digit1, |n: &str| n.parse::<T>())(input)
}

impl Test {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (input, divisible_by) = parse_line(input)?;
        let (input, truthy_index) = parse_line(input)?;
        let (input, falsy_index) = parse_line(input)?;

        Ok((
            input,
            Test {
                divisible_by,
                truthy_index,
                falsy_index,
            },
        ))
    }
}

fn parse_starting_items(input: &str) -> IResult<&str, VecDeque<u64>> {
    map(
        preceded(
            pair(space0, tag("Starting items: ")),
            separated_list1(tag(", "), map_res(digit1, |n: &str| n.parse::<u64>())),
        ),
        |v| VecDeque::from(v),
    )(input)
}

#[derive(Clone, Debug)]
pub struct Monkey {
    items: VecDeque<u64>,
    operation: Operation,
    test: Test,
}

impl Monkey {
    fn parse(input: &str) -> IResult<&str, Monkey> {
        let (input, _) = terminated(take_until("\n"), newline)(input)?;
        let (input, starting_items) = terminated(parse_starting_items, newline)(input)?;
        let (input, operation) = terminated(Operation::parse, newline)(input)?;
        let (input, test) = terminated(Test::parse, newline)(input)?;

        Ok((
            input,
            Monkey {
                items: starting_items,
                operation,
                test,
            },
        ))
    }

    fn inspect(&self, item: u64) -> u64 {
        let left = match self.operation.left {
            Operand::Number(n) => n,
            Operand::Old => item,
        };
        let right = match self.operation.right {
            Operand::Number(n) => n,
            Operand::Old => item,
        };

        match self.operation.operator {
            Operator::Add => left + right,
            Operator::Multiply => left * right,
        }
    }

    fn eval_test(&self, item: u64) -> bool {
        item % self.test.divisible_by == 0
    }
}

impl ParseInput<'_, { Day::Day11 }> for AOC2022<{ Day::Day11 }> {
    type Parsed = Vec<Monkey>;

    fn parse_input(&self, input: &'_ str) -> Result<Self::Parsed> {
        let (_, monkeys) =
            separated_list1(newline, Monkey::parse)(input).map_err(|e| e.to_owned())?;
        Ok(monkeys)
    }
}

fn do_round<F>(input: &mut [Monkey], counter: &mut Counter<usize, usize>, worry_update: F)
where
    F: Fn(u64) -> u64,
{
    for i in 0..input.len() {
        counter[&i] += input[i].items.len();
        for
            loop {
                if input[i].items.len() == 0 {
                    break;
                }

                let item = input[i].items.pop_front().unwrap();
                let item = input[i].inspect(item);
                let item = worry_update(item);
                if input[i].eval_test(item) {
                    let j = input[i].test.truthy_index;
                    input
                        .get_mut(j)
                        .ok_or_else(|| anyhow!("Not enough monkeys: j={}, #={}.", j, num_monkeys))?
                        .items
                        .push_back(item);
                } else {
                    let j = input[i].test.falsy_index;
                    input
                        .get_mut(j)
                        .ok_or_else(|| anyhow!("Not enough monkeys: j={}, #={}.", j, num_monkeys))?
                        .items
                        .push_back(item);
                }
            }
    }
}


fn monkey_business<F>(mut input: Vec<Monkey>, num_rounds: usize, worry_update: F) -> Result<usize>
where
    F: Fn(u64) -> u64,
{
    let mut counter: Counter<usize, usize> = Counter::new();
    let num_monkeys = input.len();
    for _round in 0..num_rounds {
    }

    Ok(counter
        .k_most_common_ordered(2)
        .iter()
        .map(|(_i, count)| *count)
        .product())
}

impl Solution<'_, { Day::Day11 }, { Part::One }> for AOC2022<{ Day::Day11 }> {
    type Input = Vec<Monkey>;
    type Output = usize;

    fn solve(&self, input: &Self::Input) -> Result<Self::Output> {
        monkey_business(input.clone(), 20, |i: u64| i / 3)
    }
}

impl Solution<'_, { Day::Day11 }, { Part::Two }> for AOC2022<{ Day::Day11 }> {
    type Input = Vec<Monkey>;
    type Output = usize;

    fn solve(&self, input: &Self::Input) -> Result<Self::Output> {
        let divisor_product: u64 = input.iter().map(|m| m.test.divisible_by).product();
        monkey_business(input.clone(), 1000, |i: u64| i % divisor_product)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_runner::PartOneVerifier;

    #[test]
    fn test_parsing() {
        let starting_items = "  Starting items: 79, 9";
        assert_eq!(
            parse_starting_items(starting_items),
            Ok(("", VecDeque::from([79, 9])))
        );

        assert_eq!(Operand::parse("old"), Ok(("", Operand::Old)));
        assert_eq!(Operand::parse("19"), Ok(("", Operand::Number(19))));
        assert_eq!(Operator::parse("+"), Ok(("", Operator::Add)));
        assert_eq!(Operator::parse("*"), Ok(("", Operator::Multiply)));

        let operation = "  Operation: new = old * 19";
        assert_eq!(
            Operation::parse(operation),
            Ok((
                "",
                Operation {
                    left: Operand::Old,
                    operator: Operator::Multiply,
                    right: Operand::Number(19)
                }
            ))
        );

        assert_eq!(parse_line("    If true: throw to monkey 2",), Ok(("", 2)));

        let test = "  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3";
        assert_eq!(
            Test::parse(test),
            Ok((
                "",
                Test {
                    divisible_by: 23,
                    truthy_index: 2,
                    falsy_index: 3
                }
            ))
        );
    }

    #[test]
    fn test() -> Result<()> {
        let problem = super::AOC2022::<{ Day::Day11 }>;
        let input = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1\n";
        let parsed = problem.parse_input(input)?;
        assert_eq!(parsed.len(), 4);
        problem.test_part1(input, 10605)
    }
}
