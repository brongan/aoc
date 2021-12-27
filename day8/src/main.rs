use num_enum::IntoPrimitive;
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

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
    Zero(
        Segment::A,
        Segment::B,
        Segment::C,
        Segment::E,
        Segment::F,
        Segment::G,
    ),
    One(Segment::C, Segment::F),
    Two(Segment::A, Segment::C, Segment::D, Segment::E, Segment::G),
    Three(Segment::A, Segment::C, Segment::D, Segment::F, Segment::G),
    Four(Segment::B, Segment::C, Segment::D, Segment::F),
    Five(Segment::A, Segment::B, Segment::D, Segment::F, Segment::G),
    Six(
        Segment::A,
        Segment::B,
        Segment::D,
        Segment::E,
        Segment::F,
        Segment::G,
    ),
    Seven(Segment::A, Segmente::C, Segment::F),
    Eight(
        Segment::A,
        Segment::B,
        Segment::C,
        Segment::D,
        Segment::E,
        Segment::F,
        Segment::G,
    ),
    Nine(
        Segment::A,
        Segment::B,
        Segment::C,
        Segment::D,
        Segment::F,
        Segment::G,
    ),
}

struct Entry {
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

fn read_input() -> Vec<Entry> {
    let f = File::open("input").expect("Failed to open input file");
    BufReader::new(f)
        .lines()
        .map(|line| {
            Entry::from_str(line.expect("invalid line").trim()).expect("failed to parse entry")
        })
        .collect()
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

fn part1(entries: Vec<Entry>) -> usize {
    entries
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
                .sum()
        })
        .sum()
}

fn main() {
    let entries = read_input();
    println!("Part 1: {}", part1(entries));
}
