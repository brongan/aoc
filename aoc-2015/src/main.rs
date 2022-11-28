#![allow(incomplete_features)]
#![feature(adt_const_params)]
#![feature(iter_advance_by)]
#![feature(specialization)]
#![feature(map_first_last)]

mod day01;
mod day02;

use anyhow::Context;
use anyhow::anyhow;
use anyhow::Result;
use aoc_runner::run_solutions;
use aoc_runner::Day;
use aoc_runner::SolutionRunner;
use std::fs::read_to_string;

pub struct AOC2015<const DAY: Day>;

fn input(day: &Day) -> Result<String> {
    let day: u8 = (*day).into();
    let path = format!("aoc-2015/input/day{}.txt", day);
    read_to_string(&path).context(path)
}

fn solve(day: &Day) -> Result<()> {
    let input = input(day)?;
    match day {
        Day::Day1 => AOC2015::<{ Day::Day1 }>.run(&input),
        Day::Day2 => AOC2015::<{ Day::Day2 }>.run(&input),
        _ => Err(anyhow!("Day not implemented yet")),
    }
}

fn main() {
    run_solutions(&solve);
}

