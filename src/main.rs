#![feature(iter_advance_by)]
#![feature(specialization)]
#![feature(exclusive_range_pattern)]
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
mod day18;
//mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;
mod point2d;

#[macro_use]
mod aoc;

use std::{fs::read_to_string, process::exit};

use crate::aoc::{Day, SolutionRunner};

pub struct AdventOfCode2021<const DAY: Day>;

fn main() {
    let day_to_solver: Vec<Box<dyn Fn(&str) -> ()>> = vec![
        Box::new(|input| run!(AdventOfCode2021::<{ Day::One }>, input)),
        Box::new(|input| run!(AdventOfCode2021::<{ Day::Two }>, input)),
        Box::new(|input| run!(AdventOfCode2021::<{ Day::Three }>, input)),
        Box::new(|input| run!(AdventOfCode2021::<{ Day::Four }>, input)),
        Box::new(|input| run!(AdventOfCode2021::<{ Day::Five }>, input)),
        Box::new(|input| run!(AdventOfCode2021::<{ Day::Six }>, input)),
        Box::new(|input| run!(AdventOfCode2021::<{ Day::Seven }>, input)),
        Box::new(|input| run!(AdventOfCode2021::<{ Day::Eight }>, input)),
        Box::new(|input| run!(AdventOfCode2021::<{ Day::Nine }>, input)),
        Box::new(|input| run!(AdventOfCode2021::<{ Day::Ten }>, input)),
        Box::new(|input| run!(AdventOfCode2021::<{ Day::Eleven }>, input)),
        Box::new(|input| run!(AdventOfCode2021::<{ Day::Twelve }>, input)),
        Box::new(|input| run!(AdventOfCode2021::<{ Day::Thirteen }>, input)),
        Box::new(|input| run!(AdventOfCode2021::<{ Day::Fourteen }>, input)),
        Box::new(|input| run!(AdventOfCode2021::<{ Day::Fifteen }>, input)),
    ];
    if let Some(day) = std::env::args().skip(1).next() {
        let day = day.parse::<usize>().expect("unable to parse day");
        let input =
            read_to_string(format!("input/day{:02}.txt", 1)).expect("yo input file where you at");
        match day_to_solver.get(day - 1) {
            Some(solver) => solver(&input),
            None => {
                eprintln!("Invalid day input");
                exit(-1)
            }
        }
    } else {
        for i in 0..day_to_solver.len() {
            let solver = &day_to_solver[i];
            let day = i + 1;
            println!("Solving AOC 2021 Day: {}", day);
            let input = read_to_string(format!("input/day{:02}.txt", day))
                .expect("yo input file where you at");
            solver(&input);
        }
    }
}
