use super::AOC2024;
use anyhow::Result;
use aoc_runner::{Day, ParseInput, Part, Solution};
use nom::{
    bytes::complete::tag,
    character::complete::{digit1, newline},
    combinator::{map, map_res},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

type Num = i64;

#[derive(Debug)]
pub struct Equation {
    test_value: Num,
    operands: Vec<Num>,
}

fn calibrate(left: Num, right: Num, nums: &[Num], sum: Num) -> bool {
    if nums.is_empty() {
        return left + right == sum || left * right == sum;
    }
    let left1 = left + right;
    let left2 = left * right;
    let right = nums[0];
    let nums = &nums[1..];
    return calibrate(left1, right, &nums, sum) || calibrate(left2, right, &nums, sum);
}

fn calibrate2(left: Num, right: Num, nums: &[Num], sum: Num) -> bool {
    if nums.is_empty() {
        return left + right == sum
            || left * right == sum
            || Num::from_str_radix(&(format!("{left}{right}")), 10).unwrap() == sum;
    }
    let left1 = left + right;
    let left2 = left * right;
    let left3 = Num::from_str_radix(&(format!("{left}{right}")), 10).unwrap();
    let right = nums[0];
    let nums = &nums[1..];
    return calibrate2(left1, right, &nums, sum)
        || calibrate2(left2, right, &nums, sum)
        || calibrate2(left3, right, &nums, sum);
}

impl Equation {
    fn calibrate(&self) -> bool {
        calibrate(
            self.operands[0],
            self.operands[1],
            &self.operands[2..],
            self.test_value,
        )
    }
    fn calibrate2(&self) -> bool {
        calibrate2(
            self.operands[0],
            self.operands[1],
            &self.operands[2..],
            self.test_value,
        )
    }
}

type IR = Vec<Equation>;

fn parse_element(input: &str) -> IResult<&str, Num> {
    map_res(digit1, |num| Num::from_str_radix(num, 10))(input)
}

fn parse_operands(input: &str) -> IResult<&str, Vec<Num>> {
    separated_list1(tag(" "), parse_element)(input)
}

fn parse_equation(input: &str) -> IResult<&str, Equation> {
    map(
        separated_pair(parse_element, tag(": "), parse_operands),
        |(test_value, operands)| Equation {
            test_value,
            operands,
        },
    )(input)
}

impl ParseInput<'_, { Day::Day7 }> for AOC2024<{ Day::Day7 }> {
    type Parsed = IR;

    fn parse_input(&self, input: &'_ str) -> Result<Self::Parsed> {
        let (_, ret) = separated_list1(newline, parse_equation)(input).map_err(|e| e.to_owned())?;
        Ok(ret)
    }
}

impl Solution<'_, { Day::Day7 }, { Part::One }> for AOC2024<{ Day::Day7 }> {
    type Input = IR;
    type Output = Num;

    fn solve(&self, input: &Self::Input) -> Result<Self::Output> {
        Ok(input
            .iter()
            .filter(|equation| equation.calibrate())
            .map(|equation| equation.test_value)
            .sum())
    }
}

impl Solution<'_, { Day::Day7 }, { Part::Two }> for AOC2024<{ Day::Day7 }> {
    type Input = IR;
    type Output = Num;

    fn solve(&self, input: &Self::Input) -> Result<Self::Output> {
        Ok(input
            .iter()
            .filter(|equation| equation.calibrate2())
            .map(|equation| equation.test_value)
            .sum())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(calibrate(10, 19, &[], 190), true);
        assert_eq!(calibrate(81, 40, &[27], 3267), true);
    }
}
