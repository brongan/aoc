#![allow(incomplete_features)]
#![feature(adt_const_params)]
#![feature(iter_advance_by)]
#![feature(specialization)]

mod day01;
mod day04;
mod day05;

use anyhow::anyhow;
use anyhow::Context;
use anyhow::Result;
use aoc_runner::run_solutions;
use aoc_runner::Day;
use aoc_runner::SolutionRunner;
use std::fs::read_to_string;

pub struct AOC2023<const DAY: Day>;

fn input(day: &Day) -> Result<String> {
    let day: u8 = (*day).into();
    let path = format!("aoc-2023/input/day{day}.txt");
    read_to_string(&path).context(path)
}

fn solve(day: &Day) -> Result<()> {
    let input = input(day)?;
    match day {
        Day::Day1 => AOC2023::<{ Day::Day1 }>.run(&input),
        Day::Day4 => AOC2023::<{ Day::Day4 }>.run(&input),
        Day::Day5 => AOC2023::<{ Day::Day5 }>.run(&input),
        _ => Err(anyhow!("Day not implemented yet")),
    }
}

fn main() {
    run_solutions(&solve);
}
