#![allow(incomplete_features)]
#![feature(adt_const_params)]
#![feature(specialization)]
#![feature(generic_const_exprs)]

use anyhow::Result;
use num_enum::{IntoPrimitive, TryFromPrimitive};
use std::fmt::Display;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use thiserror::Error;
use pretty_assertions::assert_eq;

pub mod point2d;

#[derive(
    PartialEq,
    Eq,
    EnumIter,
    PartialOrd,
    Ord,
    TryFromPrimitive,
    IntoPrimitive,
    Debug,
    Clone,
    Copy,
    Hash,
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

#[derive(Error, Debug)]
pub enum AOCError {
    #[error("Day: {day:?}, Part: {part:?} failed. Actual: {actual} Expected: {expected}")]
    SolutionMismatch {
        day: Day,
        part: Part,
        actual: String,
        expected: String,
    },
}

pub trait Solution<'a, const DAY: Day, const PART: Part> {
    type Input;
    type Output: Display + Eq + std::fmt::Debug + PartialEq;

    fn solve(&'a self, input: &Self::Input) -> Result<Self::Output>;
}

pub trait ParseInput<'a, const DAY: Day> {
    type Parsed;

    fn parse_input(&'a self, input: &'a str) -> Result<Self::Parsed>;
}

pub trait SolutionRunner<'a, const DAY: Day> {
    fn run(&'a self, input: &'a str) -> Result<()>;
}

pub trait PartOneVerifier<'a, const DAY: Day, T> {
    fn test_part1(&'a self, input: &'a str, expected: T) -> Result<()>;
}

pub trait PartTwoVerifier<'a, const DAY: Day, T> {
    fn test_part2(&'a self, input: &'a str, expected: T) -> Result<()>;
}

impl<'a, T: 'a, const DAY: Day> SolutionRunner<'a, DAY> for T
where
    T: ParseInput<'a, DAY>
        + Solution<'a, DAY, { Part::One }, Input = <Self as ParseInput<'a, DAY>>::Parsed>
        + Solution<'a, DAY, { Part::Two }, Input = <Self as ParseInput<'a, DAY>>::Parsed>,
{
    fn run(&'a self, input: &'a str) -> Result<()> {
        let parsed_input = <Self as ParseInput<DAY>>::parse_input(self, input)?;
        let part1_output = <Self as Solution<'a, DAY, { Part::One }>>::solve(self, &parsed_input)?;
        let part2_output = <Self as Solution<'a, DAY, { Part::Two }>>::solve(self, &parsed_input)?;
        println!("Part One: {}", part1_output);
        println!("Part Two: {}", part2_output);
        Ok(())
    }
}

impl<'a, T, const DAY: Day> SolutionRunner<'a, DAY> for T
where
    T: ParseInput<'a, DAY>
        + Solution<'a, DAY, { Part::One }, Input = <Self as ParseInput<'a, DAY>>::Parsed>,
{
    default fn run(&'a self, input: &'a str) -> Result<()> {
        let parsed_input = <Self as ParseInput<DAY>>::parse_input(self, input)?;
        let part1_output = <Self as Solution<'a, DAY, { Part::One }>>::solve(self, &parsed_input)?;

        println!("Part One: {}", part1_output);
        Ok(())
    }
}

impl<'a, T, const DAY: Day, U> PartOneVerifier<'a, DAY, U> for T
where
    T: ParseInput<'a, DAY>
        + Solution<'a, DAY, { Part::One }, Input = <Self as ParseInput<'a, DAY>>::Parsed, Output = U>,
    U: is_type::Is<Type = T::Output> + std::fmt::Debug + std::cmp::PartialEq + std::fmt::Display,
{
    fn test_part1(&'a self, input: &'a str, expected: U) -> Result<()> {
        let parsed_input = <Self as ParseInput<DAY>>::parse_input(self, input)?;
        let output = <Self as Solution<'a, DAY, { Part::One }>>::solve(self, &parsed_input)?;
        assert_eq!(output, expected);
        Ok(())
    }
}

impl<'a, T, const DAY: Day, U> PartTwoVerifier<'a, DAY, U> for T
where
    T: ParseInput<'a, DAY>
        + Solution<'a, DAY, { Part::Two }, Input = <Self as ParseInput<'a, DAY>>::Parsed, Output = U>,
    U: is_type::Is<Type = T::Output> + std::fmt::Debug + std::cmp::PartialEq + std::fmt::Display,
{
    fn test_part2(&'a self, input: &'a str, expected: U) -> Result<()> {
        let input = <Self as ParseInput<DAY>>::parse_input(self, input)?;
        let output = <Self as Solution<'a, DAY, { Part::Two }>>::solve(self, &input)?;
        assert_eq!(output, expected);
        Ok(())
    }
}

pub fn run_solutions(solver: &dyn Fn(&Day) -> Result<()>) {
    if let Some(day) = std::env::args().nth(1) {
        let day_num = day.parse::<u8>().expect("unable to parse day");
        let day = Day::try_from(day_num).expect("unable to parse day");
        eprintln!("Running day: {}", day_num);
        match solver(&day) {
            Ok(_) => (),
            Err(e) => eprintln!("Error: {}", e),
        }
    } else {
        for day in Day::iter() {
            println!("Solving AOC 2021 Day: {:?}", day);
            match solver(&day) {
                Ok(_) => (),
                Err(e) => eprintln!("Error: {}", e),
            }
        }
    }
}
