use super::AdventOfCode2021;
use crate::aoc::ParseInput;
use crate::aoc::{Day, Part, Solution};
use num_enum::IntoPrimitive;
use std::collections::HashMap;
use std::str::FromStr;

#[derive(IntoPrimitive, Clone, Copy)]
#[repr(u8)]
enum Digit {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

enum Segment {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
}

enum Display {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

impl Display {
    fn to_segments(&self) -> Vec<Segment> {
        match self {
            Zero => vec![
                Segment::A,
                Segment::B,
                Segment::C,
                Segment::E,
                Segment::F,
                Segment::G,
            ],
            One => vec![Segment::C, Segment::F],
            Two => vec![Segment::A, Segment::C, Segment::D, Segment::E, Segment::G],
            Three => vec![Segment::A, Segment::C, Segment::D, Segment::F, Segment::G],
            Four => vec![Segment::B, Segment::C, Segment::D, Segment::F],
            Five => vec![Segment::A, Segment::B, Segment::D, Segment::F, Segment::G],
            Six => vec![
                Segment::A,
                Segment::B,
                Segment::D,
                Segment::E,
                Segment::F,
                Segment::G,
            ],
            Seven => vec![Segment::A, Segment::C, Segment::F],
            Eight => vec![
                Segment::A,
                Segment::B,
                Segment::C,
                Segment::D,
                Segment::E,
                Segment::F,
                Segment::G,
            ],
            Nine => vec![
                Segment::A,
                Segment::B,
                Segment::C,
                Segment::D,
                Segment::F,
                Segment::G,
            ],
        }
    }
}

pub struct Entry {
    signal: Vec<Vec<char>>,
    output: Vec<Vec<char>>,
}

impl FromStr for Entry {
    type Err = std::string::ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (signal, output) = s.split_once('|').expect("Failed to split line");
        let signal = signal
            .split(' ')
            .map(|word| word.chars().collect())
            .collect();
        let output = output
            .split(' ')
            .map(|word| word.chars().collect())
            .collect();
        Ok(Entry { signal, output })
    }
}

impl ParseInput<'_, { Day::Eight }> for AdventOfCode2021<{ Day::Eight }> {
    type Parsed = Vec<Entry>;

    fn parse_input(&self, input: &'_ str) -> Self::Parsed {
        input
            .lines()
            .map(Entry::from_str)
            .map(|t| t.unwrap())
            .collect()
    }
}

fn calculate_assignment(signal: &Vec<Vec<char>>) -> HashMap<Vec<char>, Digit> {
    let ret: HashMap<Vec<char>, Digit> = HashMap::new();

    ret
}

fn decode_output(
    assignment: HashMap<Vec<char>, Digit>,
    encoded_output: &Vec<Vec<char>>,
) -> Vec<Digit> {
    let mut ret: Vec<Digit> = vec![];
    for word in encoded_output {
        ret.push(assignment[word]);
    }
    ret
}

impl Solution<'_, { Day::Eight }, { Part::One }> for AdventOfCode2021<{ Day::Eight }> {
    type Input = Vec<Entry>;
    type Output = usize;

    fn solve(&self, input: &Self::Input) -> Self::Output {
        input
            .iter()
            .map(|entry| {
                let assignment = calculate_assignment(&entry.signal);
                let decoded_output = decode_output(assignment, &entry.output);
                decoded_output
                    .iter()
                    .map(|digit| match digit {
                        Digit::One | Digit::Four | Digit::Seven | Digit::Eight => 1,
                        _ => 0,
                    })
                    .sum::<usize>()
            })
            .sum()
    }
}
