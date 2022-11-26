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
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
//mod day18;
//mod day19;
//mod day20;
//mod day21;
//mod day22;
//mod day23;
//mod day24;
//mod day25;

use aoc_runner::{run, Day, SolutionRunner};
use std::convert::TryFrom;
use std::fs::read_to_string;
use strum::IntoEnumIterator;

pub struct AOC2021<const DAY: Day>;

fn input(day: &Day) -> String {
    let day: u8 = (*day).into();
    read_to_string(format!("aoc-2021/input/day{:02}.txt", day)).expect("yo input file where you at")
}

fn solve(day: &Day) {
    match day {
        Day::Day1 => run!(AOC2021::<{ Day::Day1 }>, &input(day)),
        Day::Day2 => run!(AOC2021::<{ Day::Day2 }>, &input(day)),
        Day::Day3 => run!(AOC2021::<{ Day::Day3 }>, &input(day)),
        Day::Day4 => run!(AOC2021::<{ Day::Day4 }>, &input(day)),
        Day::Day5 => run!(AOC2021::<{ Day::Day5 }>, &input(day)),
        Day::Day6 => run!(AOC2021::<{ Day::Day6 }>, &input(day)),
        Day::Day7 => run!(AOC2021::<{ Day::Day7 }>, &input(day)),
        Day::Day8 => run!(AOC2021::<{ Day::Day8 }>, &input(day)),
        Day::Day9 => run!(AOC2021::<{ Day::Day9 }>, &input(day)),
        Day::Day10 => run!(AOC2021::<{ Day::Day10 }>, &input(day)),
        Day::Day11 => run!(AOC2021::<{ Day::Day11 }>, &input(day)),
        Day::Day12 => run!(AOC2021::<{ Day::Day12 }>, &input(day)),
        Day::Day13 => run!(AOC2021::<{ Day::Day13 }>, &input(day)),
        Day::Day14 => run!(AOC2021::<{ Day::Day14 }>, &input(day)),
        Day::Day15 => run!(AOC2021::<{ Day::Day15 }>, &input(day)),
        Day::Day16 => run!(AOC2021::<{ Day::Day16 }>, &input(day)),
        Day::Day17 => run!(AOC2021::<{ Day::Day17 }>, &input(day)),
        _ => panic!("Day not implemented yet"),
    };
}

fn main() {
    if let Some(day) = std::env::args().nth(1) {
        let day_num = day.parse::<u8>().expect("unable to parse day");
        let day = Day::try_from(day_num).expect("unable to parse day");
        eprintln!("Running day: {}", day_num);
        solve(&day);
    } else {
        for day in Day::iter() {
            println!("Solving AOC 2021 Day: {:?}", day);
            solve(&day);
        }
    }
}
