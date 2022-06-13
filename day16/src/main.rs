#![feature(try_trait_v2)]
use std::{fs::read_to_string, str::FromStr};

struct BitIterator {
    buf: Vec<u8>,
    index: usize,
    bit_index: u8,
}

impl FromStr for BitIterator {
    type Err = std::string::ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let buf = s.chars().map(|c| from_hex(c)).collect();
        Ok(BitIterator {
            buf,
            index: 0,
            bit_index: 7,
        })
    }
}

impl Iterator for BitIterator {
    type Item = bool; // single bit
    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.buf.len() {
            return None;
        }
        let ret = (self.buf[self.index] & (0b1 << self.bit_index)) != 0;
        if self.bit_index == 0 {
            self.index += 1;
            self.bit_index = 7
        } else {
            self.bit_index -= 1;
        }
        Some(ret)
    }
}

fn from_hex(c: char) -> u8 {
    if c.is_digit(10) {
        c.to_digit(10).unwrap().try_into().unwrap()
    } else {
        match c {
            'A' => 10,
            'B' => 11,
            'C' => 12,
            'D' => 13,
            'E' => 14,
            'F' => 15,
            _ => panic!("Unicode is not real"),
        }
    }
}

enum Packet {
    Op(OperatorPacket),
    Lit(LiteralPacket),
}

#[derive(Default)]
struct OperatorPacket {
    version: u16,
    type_id: u16,
    length_type_id: bool,
    sub_packets: Vec<Packet>,
}

struct LiteralPacket {
    version: u16,
    type_id: u16,
    data: Vec<u16>,
}

impl Packet {
    fn from_bit_iterator(b: &mut impl Iterator<Item = bool>) -> Option<Self> {
        let version = from_bit_iter(b, 3);
        let type_id = from_bit_iter(b, 3);
        match type_id {
            4 => {
                let mut data = Vec::new();
                let mut prefix = true;
                while prefix {
                    prefix = b.take(1).next().unwrap();
                    data.push(from_bit_iter(b, 4));
                }
                Some(Packet::Lit(LiteralPacket {
                    version,
                    type_id,
                    data,
                }))
            }
            _ => {
                let length_type_id: bool = b.take(1).next().unwrap();
                let mut sub_packets: Vec<Packet> = Vec::new();
                if !length_type_id {
                    let total_bit_length = from_bit_iter(b, 15);
                    let mut bool_slice = b.take(total_bit_length as usize);
                    while let Some(packet) = Packet::from_bit_iterator(&mut bool_slice) {
                        sub_packets.push(packet);
                    }
                } else {
                    let additional_sub_packets = from_bit_iter(b, 11);
                    for _ in 0..additional_sub_packets {
                        sub_packets.push(Packet::from_bit_iterator(b).unwrap());
                    }
                }
                Some(Packet::Op(OperatorPacket {
                    version,
                    type_id,
                    length_type_id,
                    sub_packets,
                }))
            }
        }
    }
}

fn from_bit_iter(b: &mut dyn Iterator<Item = bool>, bits: usize) -> u16 {
    let mut ret = 0;
    for _ in 0..bits - 1 {
        ret |= if b.next().unwrap() { 0b1 } else { 0b0 };
        ret <<= 1
    }
    ret |= if b.next().unwrap() { 0b1 } else { 0b0 };
    ret
}

fn get_version_numbers(packet: &Packet) -> Vec<u16> {
    match packet {
        Packet::Op(op_packet) => op_packet
            .sub_packets
            .iter()
            .map(|p| get_version_numbers(p))
            .flatten()
            .collect(),
        Packet::Lit(lit_packet) => vec![lit_packet.version],
    }
}

fn part1(mut b: Box<dyn Iterator<Item = bool>>) -> u64 {
    let packet = Packet::from_bit_iterator(&mut b).expect("Failed to parse outer packet");
    get_version_numbers(&packet).iter().map(|v| *v as u64).sum()
}

fn main() {
    let input = BitIterator::from_str(read_to_string("input").expect("bad file").trim())
        .expect("Failed to parse input");
    println!("Part 1: {}", part1(Box::new(input)));
}
