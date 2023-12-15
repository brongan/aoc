use super::AOC2023;
use anyhow::Result;
use aoc_runner::{Day, ParseInput, Part, Solution};
use nom::{
    bytes::complete::tag,
    character::complete::{digit1, multispace0, newline},
    combinator::{all_consuming, map, map_res},
    multi::separated_list0,
    sequence::{preceded, terminated, tuple},
    IResult,
};

type Num = i64;

#[derive(Debug)]
pub struct Race {
    time: Num,
    distance: Num,
}

impl Race {
    fn num_ways(&self) -> usize {
        (0..self.time)
            .map(|i| (self.time - i) * i)
            .filter(|distance| *distance >= self.distance)
            .count()
    }
}

fn parse_num(input: &str) -> IResult<&str, Num> {
    map_res(digit1, |num: &str| Num::from_str_radix(num, 10))(input)
}

fn time_prefix(input: &str) -> IResult<&str, (&str, &str)> {
    tuple((tag("Time:"), multispace0))(input)
}

fn distance_prefix(input: &str) -> IResult<&str, (&str, &str)> {
    tuple((tag("Distance:"), multispace0))(input)
}

fn parse_nums(input: &str) -> IResult<&str, Vec<Num>> {
    separated_list0(multispace0, parse_num)(input)
}

fn parse_races(input: &str) -> IResult<&str, Vec<Race>> {
    let parse_times = |input| terminated(preceded(time_prefix, parse_nums), newline)(input);
    let parse_distances = |input| terminated(preceded(distance_prefix, parse_nums), newline)(input);
    map(
        tuple((parse_times, parse_distances)),
        |(times, distances)| {
            times
                .iter()
                .zip(distances.into_iter())
                .map(|(&time, distance)| Race { time, distance })
                .collect()
        },
    )(input)
}

impl ParseInput<'_, { Day::Day6 }> for AOC2023<{ Day::Day6 }> {
    type Parsed = String;

    fn parse_input(&self, input: &'_ str) -> Result<Self::Parsed> {
        Ok(input.to_string())
    }
}

impl Solution<'_, { Day::Day6 }, { Part::One }> for AOC2023<{ Day::Day6 }> {
    type Input = String;
    type Output = usize;

    fn solve(&self, input: &Self::Input) -> Result<Self::Output> {
        let (_, races) = all_consuming(parse_races)(input).map_err(|e| e.to_owned())?;
        Ok(races.iter().map(|race| race.num_ways()).product())
    }
}

impl Solution<'_, { Day::Day6 }, { Part::Two }> for AOC2023<{ Day::Day6 }> {
    type Input = String;
    type Output = usize;

    fn solve(&self, input: &Self::Input) -> Result<Self::Output> {
        let mut input = input.to_owned();
        input.retain(|c| !c.is_whitespace() || c == '\n');
        let (_, races) = all_consuming(parse_races)(&input).map_err(|e| e.to_owned())?;
        Ok(races[0].num_ways())
    }
}
