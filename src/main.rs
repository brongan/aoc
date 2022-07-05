#![feature(iter_advance_by)]
#![feature(specialization)]
#![feature(adt_const_params)]
#![allow(incomplete_features)]

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
mod point2d;

#[macro_use]
mod aoc;

use std::collections::BTreeMap;
use std::convert::TryFrom;
use std::fs::read_to_string;
use strum::IntoEnumIterator;

use crate::aoc::{Day, SolutionRunner};

pub struct AOC2021<const DAY: Day>;

fn input(day: &Day) -> String {
    let day: u8 = (*day).into();
    read_to_string(format!("input/day{:02}.txt", day)).expect("yo input file where you at")
}

fn main() {
    let mut solvers: BTreeMap<Day, Box<dyn Fn(&str) -> ()>> = BTreeMap::new();
    solvers.insert(
        Day::Day1,
        Box::new(|input| run!(AOC2021::<{ Day::Day1 }>, input)),
    );
    solvers.insert(
        Day::Day2,
        Box::new(|input| run!(AOC2021::<{ Day::Day2 }>, input)),
    );
    solvers.insert(
        Day::Day3,
        Box::new(|input| run!(AOC2021::<{ Day::Day3 }>, input)),
    );
    solvers.insert(
        Day::Day4,
        Box::new(|input| run!(AOC2021::<{ Day::Day4 }>, input)),
    );
    solvers.insert(
        Day::Day5,
        Box::new(|input| run!(AOC2021::<{ Day::Day5 }>, input)),
    );
    solvers.insert(
        Day::Day6,
        Box::new(|input| run!(AOC2021::<{ Day::Day6 }>, input)),
    );
    solvers.insert(
        Day::Day7,
        Box::new(|input| run!(AOC2021::<{ Day::Day7 }>, input)),
    );
    solvers.insert(
        Day::Day8,
        Box::new(|input| run!(AOC2021::<{ Day::Day8 }>, input)),
    );
    solvers.insert(
        Day::Day9,
        Box::new(|input| run!(AOC2021::<{ Day::Day9 }>, input)),
    );
    solvers.insert(
        Day::Day10,
        Box::new(|input| run!(AOC2021::<{ Day::Day10 }>, input)),
    );
    solvers.insert(
        Day::Day11,
        Box::new(|input| run!(AOC2021::<{ Day::Day11 }>, input)),
    );
    solvers.insert(
        Day::Day12,
        Box::new(|input| run!(AOC2021::<{ Day::Day12 }>, input)),
    );
    solvers.insert(
        Day::Day13,
        Box::new(|input| run!(AOC2021::<{ Day::Day13 }>, input)),
    );
    solvers.insert(
        Day::Day14,
        Box::new(|input| run!(AOC2021::<{ Day::Day14 }>, input)),
    );
    solvers.insert(
        Day::Day15,
        Box::new(|input| run!(AOC2021::<{ Day::Day15 }>, input)),
    );
    solvers.insert(
        Day::Day16,
        Box::new(|input| run!(AOC2021::<{ Day::Day16 }>, input)),
    );
    solvers.insert(
        Day::Day17,
        Box::new(|input| run!(AOC2021::<{ Day::Day17 }>, input)),
    );

    if let Some(day) = std::env::args().skip(1).next() {
        let day_num = day.parse::<u8>().expect("unable to parse day");
        let day = Day::try_from(day_num).expect("unable to parse day");
        eprintln!("Running day: {}", day_num);
        let solver = solvers.get(&day).expect("day not implemented");
        solver(&input(&day));
    } else {
        for day in Day::iter() {
            if let Some(solver) = solvers.get(&day).take() {
                println!("Solving AOC 2021 Day: {:?}", day);
                solver(&input(&day));
            }
        }
    }
}
