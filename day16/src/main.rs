use std::{fs::read_to_string, str::FromStr};

struct BitIterator {
    buf: Vec<u8>,
    remainder: u8,
}

impl Iterator for BitIterator {
    type Item = u8; // single bit
    fn next(&mut self) -> Option<Self::Item> {
        
    }
}

fn from_hex(c: char) -> u8 {
    if c.is_digit() {
        c.to_digit(10).unwrap()
    } else {
     match c {   
        'A' => 10,
        'B' => 11,
        'C' => 12,
        'D' => 13,
        'E' => 14,
        'F' => 15
    }
}

struct Packet {
    version: char,
    type_id: char,
}

impl FromStr for Packet {
    type Err = std::string::ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s: Vec<char> = s.chars().collect();
        let version = s[0];
        let type_id = s[1];
    }
}

fn part1(input: &str) {
    match header {
        0b100 => Literal(),
        0b110 => PacketVersion(),
    }
}

fn main() {
    let input = read_to_string("input").expect("bad file").trim();
    println!("Part 1: {}", part1(input));
}
