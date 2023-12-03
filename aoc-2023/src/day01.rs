use super::AOC2023;
use anyhow::{Context, Result};
use aoc_runner::{Day, ParseInput, Part, Solution};
use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    combinator::{map, map_res},
    IResult,
};

fn parse_word(input: &str) -> IResult<&str, u8> {
    let one = |input| map(tag("one"), |_| 1)(input);
    let two = |input| map(tag("two"), |_| 2)(input);
    let three = |input| map(tag("three"), |_| 3)(input);
    let four = |input| map(tag("four"), |_| 4)(input);
    let five = |input| map(tag("five"), |_| 5)(input);
    let six = |input| map(tag("six"), |_| 6)(input);
    let seven = |input| map(tag("seven"), |_| 7)(input);
    let eight = |input| map(tag("eight"), |_| 8)(input);
    let nine = |input| map(tag("nine"), |_| 9)(input);
    alt((one, two, three, four, five, six, seven, eight, nine))(input)
}

fn parse_digit(input: &str) -> IResult<&str, u8> {
    map_res(take(1usize), |s: &str| s.parse())(input)
}

fn parse_number(input: &str) -> IResult<&str, u8> {
    alt((parse_word, parse_digit))(input)
}

fn parse_line(input: &str) -> Vec<u8> {
    let mut result = Vec::new();
    for i in 0..input.len() {
        let slice = &input[i..input.len()];
        match parse_number(slice) {
            Ok((_, num)) => result.push(num),
            _ => (),
        }
    }
    result
}

fn parse_calibration_value(input: &str) -> Result<u32> {
    let digits = parse_line(input);
    eprintln!("{input}: {digits:?}");
    Ok(*digits.first().context("no digits.")? as u32 * 10 + *digits.last().unwrap() as u32)
}

impl ParseInput<'_, { Day::Day1 }> for AOC2023<{ Day::Day1 }> {
    type Parsed = Vec<String>;

    fn parse_input(&self, input: &'_ str) -> Result<Self::Parsed> {
        Ok(input.lines().map(|s| s.to_owned()).collect())
    }
}

impl Solution<'_, { Day::Day1 }, { Part::One }> for AOC2023<{ Day::Day1 }> {
    type Input = Vec<String>;
    type Output = u32;

    fn solve(&self, input: &Self::Input) -> Result<Self::Output> {
        Ok(input
            .iter()
            .map(|line| {
                line.chars()
                    .filter(|c| match c {
                        '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => true,
                        _ => false,
                    })
                    .collect::<String>()
            })
            .map(|digits| {
                let chars = digits.as_bytes();
                let l = *chars.first().unwrap() as char;
                let r = *chars.last().unwrap() as char;
                format!("{l}{r}").parse::<u32>().unwrap()
            })
            .sum())
    }
}

impl Solution<'_, { Day::Day1 }, { Part::Two }> for AOC2023<{ Day::Day1 }> {
    type Input = Vec<String>;
    type Output = u32;

    fn solve(&self, input: &Self::Input) -> Result<Self::Output> {
        input.iter().map(|line| parse_calibration_value(line)).sum()
    }
}
