#![feature(adt_const_params)]

mod day01;
mod day02;

use anyhow::Context;
use anyhow::Result;
use anyhow::anyhow;
use aoc_runner::Day;
use aoc_runner::SolutionRunner;
use aoc_runner::run_solutions;
use std::fs::read_to_string;

pub struct AOC2025<const DAY: Day>;

fn input(day: &Day) -> Result<String> {
    let day: u8 = (*day).into();
    let path = format!("aoc-2025/input/day{day}.txt");
    read_to_string(&path).context(path)
}

fn solve(day: &Day) -> Result<()> {
    let input = input(day)?;
    match day {
        Day::Day1 => AOC2025::<{ Day::Day1 }>.run(&input),
        Day::Day2 => AOC2025::<{ Day::Day2 }>.run(&input),
        _ => Err(anyhow!("Day not implemented yet")),
    }
}

fn main() {
    run_solutions(&solve);
}
