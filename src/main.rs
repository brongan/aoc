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
//mod day16;
//mod day17;
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

use std::fs::read_to_string;
use strum::IntoEnumIterator;
use std::collections::BTreeMap;
use std::convert::TryFrom;

use crate::aoc::{Day, SolutionRunner};

pub struct AdventOfCode2021<const DAY: Day>;

fn input(day: &Day) -> String {
    let day: u8 = (*day).into();
    read_to_string(format!("input/day{:02}.txt", day)).expect("yo input file where you at")
}

fn main() {
    let mut solvers: BTreeMap<Day, Box<dyn Fn(&str) -> ()>> = BTreeMap::new();
    solvers.insert(Day::One, Box::new(|input| run!(AdventOfCode2021::<{ Day::One }>, input)));
    solvers.insert(Day::Two, Box::new(|input| run!(AdventOfCode2021::<{ Day::Two }>, input)));
    solvers.insert(Day::Three, Box::new(|input| run!(AdventOfCode2021::<{ Day::Three }>, input)));
    solvers.insert(Day::Four, Box::new(|input| run!(AdventOfCode2021::<{ Day::Four }>, input)));
    solvers.insert(Day::Five, Box::new(|input| run!(AdventOfCode2021::<{ Day::Five }>, input)));
    solvers.insert(Day::Six, Box::new(|input| run!(AdventOfCode2021::<{ Day::Six }>, input)));
    solvers.insert(Day::Seven, Box::new(|input| run!(AdventOfCode2021::<{ Day::Seven }>, input)));
    solvers.insert(Day::Eight, Box::new(|input| run!(AdventOfCode2021::<{ Day::Eight }>, input)));
    solvers.insert(Day::Nine, Box::new(|input| run!(AdventOfCode2021::<{ Day::Nine }>, input)));
    solvers.insert(Day::Ten, Box::new(|input| run!(AdventOfCode2021::<{ Day::Ten }>, input)));
    solvers.insert(Day::Eleven, Box::new(|input| run!(AdventOfCode2021::<{ Day::Eleven }>, input)));
    solvers.insert(Day::Twelve, Box::new(|input| run!(AdventOfCode2021::<{ Day::Twelve }>, input)));
    solvers.insert(Day::Thirteen, Box::new(|input| run!(AdventOfCode2021::<{ Day::Thirteen }>, input)));
    solvers.insert(Day::Fourteen, Box::new(|input| run!(AdventOfCode2021::<{ Day::Fourteen }>, input)));
    solvers.insert(Day::Fifteen, Box::new(|input| run!(AdventOfCode2021::<{ Day::Fifteen }>, input)));
    //solvers.insert(Day::Sixteen, Box::new(|input| run!(AdventOfCode2021::<{ Day::Sixteen }>, input)));

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
