use nom::IResult;
use nom::bytes::complete::take_while_m_n;
use nom::combinator::map_res;
use nom::multi::many1;

use super::AOC2021;
use crate::aoc::{Day, Part, Solution, ParseInput};
use std::iter::Iterator;
use std::str::FromStr;

#[derive(Clone)]
pub struct BitIterator {
    buf: Vec<u8>,
    index: usize,
    bit_index: u8,
    num_remaining: usize,
}

fn from_hex(input: &str) -> Result<u8, std::num::ParseIntError> {
  u8::from_str_radix(input, 16)
}

fn is_hex_digit(c: char) -> bool {
  c.is_digit(16)
}

fn hex2(input: &str) -> IResult<&str, u8> {
    map_res(
    take_while_m_n(2, 2, is_hex_digit),
    from_hex
  )(input)
}

fn hex_to_array(input: &str) -> IResult<&str, Vec<u8>> {
    many1(hex2)(input)
}

impl FromStr for BitIterator {
    type Err = std::string::ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_remainder, buf) = hex_to_array(s).expect("failed to parse hex");
        let len = buf.len();
        Ok(BitIterator {
            buf,
            index: 0,
            bit_index: 7,
            num_remaining: len * 8,
        })
    }
}

impl Iterator for BitIterator {
    type Item = bool; // single bit
    fn next(&mut self) -> Option<Self::Item> {
        if self.num_remaining == 0 || self.index >= self.buf.len() {
            return None;
        }
        let ret = (self.buf[self.index] & (0b1 << self.bit_index)) != 0;
        if self.bit_index == 0 {
            self.index += 1;
            self.bit_index = 7
        } else {
            self.bit_index -= 1;
        }
        self.num_remaining -= 1;
        Some(ret)
    }
}

impl BitIterator {
    fn read(&mut self, bits: u8) -> Option<u8> {
        let mut ret = 0;
        for i in 0..bits {
            let bit = if self.next()? { 0b1 } else { 0b0 };
            ret |= bit << (bits - i - 1);
        }
        Some(ret)
    }

    fn read2(&mut self, bits: u8) -> Option<u16> {
        let mut ret = 0;
        for i in 0..bits {
            let bit = if self.next()? { 0b1 } else { 0b0 };
            ret |= bit << (bits - i - 1);
        }
        Some(ret)
    }
}

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum Packet {
    Op(OperatorPacket),
    Lit(LiteralPacket),
}

#[allow(dead_code)]
#[derive(Default, Debug, PartialEq)]
pub struct OperatorPacket {
    version: u8,
    type_id: u8,
    length_type_id: bool,
    sub_packets: Vec<Packet>,
}

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub struct LiteralPacket {
    version: u8,
    type_id: u8,
    data: Vec<u8>,
}

impl Packet {
    fn from_iterator(b: &mut BitIterator) -> Result<Self, String> {
        let version = b.read(3).ok_or("Failed to read version")?;
        let type_id = b.read(3).ok_or("Failed to read type_id")?;
        if let 4 = type_id {
            let mut nibbles = Vec::new();
            let mut prefix = true;
            while prefix {
                prefix = b.next().ok_or("Failed to read prefix")?;
                nibbles.push(b.read(4).ok_or("Failed to read prefix")?);
            }
            if nibbles.len() % 2 == 1 {
                nibbles.push(0);
            }
            let data = nibbles.chunks(2).map(|c| { (c[0] << 4) | c[1]}).collect();
            Ok(Packet::Lit(LiteralPacket {
                version,
                type_id,
                data,
            }))
        } else {
            let length_type_id: bool = b.next().ok_or("Failed to read length_type_id")?;
            let mut sub_packets: Vec<Packet> = Vec::new();
            if !length_type_id {
                let total_bit_length = b.read2(15).ok_or("Failed to read total_bit_length")?;
                let curr_bits_remaining = b.num_remaining;
                while curr_bits_remaining - b.num_remaining > total_bit_length.into() {
                    sub_packets.push(Packet::from_iterator(b)?);
                }
            } else {
                let additional_sub_packets = b.read2(11).ok_or("Failed to read additional_sub_packets")?;
                for _ in 0..additional_sub_packets {
                    sub_packets.push(Packet::from_iterator(b)?);
                }
            }
            Ok(Packet::Op(OperatorPacket {
                version,
                type_id,
                length_type_id,
                sub_packets,
            }))
        }
    }
}

fn get_version_numbers(packet: &Packet) -> Vec<u8> {
    match packet {
        Packet::Op(op_packet) => {
            let mut ret: Vec<u8> = op_packet
            .sub_packets
            .iter()
            .map(|p| get_version_numbers(p))
            .flatten()
            .collect();
            ret.push(op_packet.version);
            ret
        },
        Packet::Lit(lit_packet) => vec![lit_packet.version],
    }
}

impl ParseInput<'_, { Day::Day16 }> for AOC2021<{ Day::Day16 }> {
    type Parsed = Packet;

    fn parse_input(&self, input: &'_ str) -> Self::Parsed {
        Packet::from_iterator(&mut BitIterator::from_str(&input).expect("failed to parse hex")).expect("failed to parse packet")
    }
}

impl Solution<'_, { Day::Day16 }, { Part::One }> for AOC2021<{ Day::Day16 }> {
    type Input = Packet;
    type Output = u64;

    fn solve(&self, input: &Self::Input) -> Self::Output {
        eprintln!("Packet:\n{:?}",input);
        get_version_numbers(&input).iter().map(|v| *v as u64).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aoc::PartOneVerifier;

    #[test]
    fn test_hex_parsing() {
          assert_eq!(hex_to_array("2F14DF"), Ok(("", vec![47,20,223])));
    }

    #[test]
    fn test_from_hex() {
        assert_eq!(from_hex("0"), Ok(0));
        assert_eq!(from_hex("1"), Ok(1));
        assert_eq!(from_hex("9"), Ok(9));
        assert_eq!(from_hex("a"), Ok(10));
        assert_eq!(from_hex("f"), Ok(15));
        assert_eq!(from_hex("A"), Ok(10));
        assert_eq!(from_hex("F"), Ok(15));
    }

    #[test]
    fn test_hex_color() {
        assert_eq!(hex2("2f"), Ok(("", 47)));
        assert_eq!(hex2("14"), Ok(("", 20)));
        assert_eq!(hex2("DF"), Ok(("", 223)));
        assert_eq!(hex2("28"), Ok(("", 40)));
    }

    #[test]
    fn test_hex_binary0() {
        assert_eq!(format!("{:08b}", 40), "00101000");

        let (rem, output) = hex2("05").unwrap();
        assert!(rem.is_empty());
        assert_eq!(format!("{:08b}", output), "00000101");

        let (rem, val) = hex_to_array("D2FE28").expect("is ok");
        assert!(rem.is_empty());
        assert_eq!(val.iter().map(|v| format!("{:08b}", v)).collect::<String>(), "110100101111111000101000");
    }

    #[test]
    fn test_hex_binary1() {
        let (rem, val) = hex_to_array("D2FE28").expect("is ok");
        assert!(rem.is_empty());
        assert_eq!(val.iter().map(|v| format!("{:08b}", v)).collect::<String>(), "110100101111111000101000");
    }

    #[test]
    fn test_hex_binary2() {
        let (rem, val) = hex_to_array("38006F45291200").expect("is ok");
        assert!(rem.is_empty());
        assert_eq!(val.iter().map(|v| format!("{:08b}", v)).collect::<String>(), "00111000000000000110111101000101001010010001001000000000");
    }

    #[test]
    fn test_hex_binary3() {
        let (rem, val) = hex_to_array("EE00D40C823060").expect("is ok");
        assert!(rem.is_empty());
        assert_eq!(val.iter().map(|v| format!("{:08b}", v)).collect::<String>(), "11101110000000001101010000001100100000100011000001100000");
    }

    #[test]
    fn test_literal_packet() {
        let packet = Packet::from_iterator(&mut BitIterator::from_str("D2FE28").expect("bit iterator")).expect("is a packet");
        assert_eq!(packet, Packet::Lit(LiteralPacket {version: 6, type_id: 4, data: vec![7, 229]}));
    }

    #[test]
    fn test() -> Result<(), String> {
        let problem = super::AOC2021::<{ Day::Day16 }>;
        let input = "620080001611562C8802118E34";
        (&&&problem).test_part1(input, 12)?;
        let input = "8A004A801A8002F478";
        (&&&problem).test_part1(input, 16)?;
        let input = "C0015000016115A2E0802F182340";
        (&&&problem).test_part1(input, 23)?;
        let input = "A0016C880162017C3686B18A3D4780";
        (&&&problem).test_part1(input, 31)
    }
}
