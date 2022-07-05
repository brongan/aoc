use num_enum::{IntoPrimitive, TryFromPrimitive};
use std::fmt::Display;
use strum_macros::EnumIter;

#[derive(
    PartialEq, Eq, EnumIter, PartialOrd, Ord, TryFromPrimitive, IntoPrimitive, Debug, Clone, Copy,
)]
#[repr(u8)]
pub enum Day {
    Day1 = 1,
    Day2,
    Day3,
    Day4,
    Day5,
    Day6,
    Day7,
    Day8,
    Day9,
    Day10,
    Day11,
    Day12,
    Day13,
    Day14,
    Day15,
    Day16,
    Day17,
    Day18,
    Day19,
    Day20,
    Day21,
    Day22,
    Day23,
    Day24,
    Day25,
}

#[derive(PartialEq, Eq, Debug)]
pub enum Part {
    One,
    Two,
}

pub trait Solution<'a, const DAY: Day, const PART: Part> {
    type Input;
    type Output: Display + Eq + std::fmt::Debug + PartialEq;

    fn solve(&'a self, input: &Self::Input) -> Self::Output;
}

pub trait ParseInput<'a, const DAY: Day> {
    type Parsed;

    fn parse_input(&'a self, input: &'a str) -> Self::Parsed;
}

pub trait SolutionRunner<'a, const DAY: Day> {
    fn run(&'a self, input: &'a str);
}

pub trait PartOneVerifier<'a, const DAY: Day, T> {
    fn test_part1(&'a self, input: &'a str, expected: T) -> Result<(), String>;
}

pub trait PartTwoVerifier<'a, const DAY: Day, T> {
    fn test_part2(&'a self, input: &'a str, expected: T) -> Result<(), String>;
}

impl<'a, T: 'a, const DAY: Day> SolutionRunner<'a, DAY> for T
where
    T: ParseInput<'a, DAY>
        + Solution<'a, DAY, { Part::One }, Input = <Self as ParseInput<'a, DAY>>::Parsed>
        + Solution<'a, DAY, { Part::Two }, Input = <Self as ParseInput<'a, DAY>>::Parsed>,
{
    fn run(&'a self, input: &'a str) {
        let parsed_input = <Self as ParseInput<DAY>>::parse_input(&self, input.trim());
        let part1_output = <Self as Solution<'a, DAY, { Part::One }>>::solve(&self, &parsed_input);
        let part2_output = <Self as Solution<'a, DAY, { Part::Two }>>::solve(&self, &parsed_input);
        println!("Part One: {}", part1_output);
        println!("Part Two: {}", part2_output);
    }
}

impl<'a, T, const DAY: Day> SolutionRunner<'a, DAY> for T
where
    T: ParseInput<'a, DAY>
        + Solution<'a, DAY, { Part::One }, Input = <Self as ParseInput<'a, DAY>>::Parsed>,
{
    default fn run(&'a self, input: &'a str) {
        let parsed_input = <Self as ParseInput<DAY>>::parse_input(&self, input.trim());
        let part1_output = <Self as Solution<'a, DAY, { Part::One }>>::solve(&self, &parsed_input);

        println!("Part One: {}", part1_output);
    }
}

impl<'a, T, const DAY: Day, U> PartOneVerifier<'a, DAY, U> for T
where
    T: ParseInput<'a, DAY>
        + Solution<'a, DAY, { Part::One }, Input = <Self as ParseInput<'a, DAY>>::Parsed, Output = U>,
    U: is_type::Is<Type = T::Output> + std::fmt::Debug + std::cmp::PartialEq + std::fmt::Display,
{
    fn test_part1(&'a self, input: &'a str, expected: U) -> Result<(), String> {
        let parsed_input = <Self as ParseInput<DAY>>::parse_input(&self, input.trim());
        let output = <Self as Solution<'a, DAY, { Part::One }>>::solve(&self, &parsed_input);
        if output == expected {
            Ok(())
        } else {
            Err(format!(
                "Day: {:?}, Part: {:?} failed. Expected: {} Found: {}",
                DAY,
                Part::One,
                expected,
                output
            ))
        }
    }
}

impl<'a, T, const DAY: Day, U> PartTwoVerifier<'a, DAY, U> for T
where
    T: ParseInput<'a, DAY>
        + Solution<'a, DAY, { Part::Two }, Input = <Self as ParseInput<'a, DAY>>::Parsed, Output = U>,
    U: is_type::Is<Type = T::Output> + std::fmt::Debug + std::cmp::PartialEq + std::fmt::Display,
{
    fn test_part2(&'a self, input: &'a str, expected: U) -> Result<(), String> {
        let input = <Self as ParseInput<DAY>>::parse_input(&self, input.trim());
        let output = <Self as Solution<'a, DAY, { Part::Two }>>::solve(&self, &input);
        if output == expected {
            Ok(())
        } else {
            Err(format!(
                "Day: {:?}, Part: {:?} failed. Expected: {} Found: {}",
                DAY,
                Part::Two,
                expected,
                output
            ))
        }
    }
}

#[allow(unused_macros)]
macro_rules! run {
    ($day: expr, $input: expr) => {{
        let problem = $day;
        (&&&problem).run($input)
    }};
}
