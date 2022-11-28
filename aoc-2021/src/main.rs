#![allow(incomplete_features)]
#![feature(adt_const_params)]
#![feature(iter_advance_by)]
#![feature(specialization)]
#![feature(map_first_last)]

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;

use anyhow::anyhow;
use anyhow::Result;
use aoc_runner::run_solutions;
use aoc_runner::Day;
use aoc_runner::SolutionRunner;
use std::fs::read_to_string;

pub struct AOC2021<const DAY: Day>;

fn input(day: &Day) -> Result<String> {
    let day: u8 = (*day).into();
    match read_to_string(format!("aoc-2021/input/day{:02}.txt", day)) {
        Ok(s) => Ok(s),
        Err(e) => Err(e.into()),
    }
}

fn solve(day: &Day) -> Result<()> {
    let input = input(day)?;
    match day {
        Day::Day1 => AOC2021::<{ Day::Day1 }>.run(&input),
        Day::Day2 => AOC2021::<{ Day::Day2 }>.run(&input),
        Day::Day3 => AOC2021::<{ Day::Day3 }>.run(&input),
        Day::Day4 => AOC2021::<{ Day::Day4 }>.run(&input),
        Day::Day5 => AOC2021::<{ Day::Day5 }>.run(&input),
        Day::Day6 => AOC2021::<{ Day::Day6 }>.run(&input),
        Day::Day7 => AOC2021::<{ Day::Day7 }>.run(&input),
        Day::Day8 => AOC2021::<{ Day::Day8 }>.run(&input),
        Day::Day9 => AOC2021::<{ Day::Day9 }>.run(&input),
        Day::Day10 => AOC2021::<{ Day::Day10 }>.run(&input),
        Day::Day11 => AOC2021::<{ Day::Day11 }>.run(&input),
        Day::Day12 => AOC2021::<{ Day::Day12 }>.run(&input),
        Day::Day13 => AOC2021::<{ Day::Day13 }>.run(&input),
        Day::Day14 => AOC2021::<{ Day::Day14 }>.run(&input),
        Day::Day15 => AOC2021::<{ Day::Day15 }>.run(&input),
        Day::Day16 => AOC2021::<{ Day::Day16 }>.run(&input),
        Day::Day17 => AOC2021::<{ Day::Day17 }>.run(&input),
        _ => Err(anyhow!("Day not implemented yet")),
    }
}

fn main() {
    run_solutions(&solve);
}
