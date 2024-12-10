#![allow(incomplete_features)]
#![feature(adt_const_params)]
#![feature(iter_advance_by)]
#![feature(specialization)]

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;

use anyhow::anyhow;
use anyhow::Context;
use anyhow::Result;
use aoc_runner::run_solutions;
use aoc_runner::Day;
use aoc_runner::SolutionRunner;
use std::fs::read_to_string;

pub struct AOC2024<const DAY: Day>;

fn input(day: &Day) -> Result<String> {
    let day: u8 = (*day).into();
    let path = format!("aoc-2024/input/day{day}.txt");
    read_to_string(&path).context(path)
}

fn solve(day: &Day) -> Result<()> {
    let input = input(day)?;
    match day {
        Day::Day1 => AOC2024::<{ Day::Day1 }>.run(&input),
        Day::Day2 => AOC2024::<{ Day::Day2 }>.run(&input),
        Day::Day3 => AOC2024::<{ Day::Day3 }>.run(&input),
        Day::Day4 => AOC2024::<{ Day::Day4 }>.run(&input),
        Day::Day5 => AOC2024::<{ Day::Day5 }>.run(&input),
        Day::Day6 => AOC2024::<{ Day::Day6 }>.run(&input),
        Day::Day7 => AOC2024::<{ Day::Day7 }>.run(&input),
        Day::Day8 => AOC2024::<{ Day::Day8 }>.run(&input),
        _ => Err(anyhow!("Day not implemented yet")),
    }
}

fn main() {
    run_solutions(&solve);
}
