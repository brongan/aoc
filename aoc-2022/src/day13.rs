use std::cmp::Ordering;

use super::AOC2022;
use anyhow::Result;
use aoc_runner::{Day, ParseInput, Part, Solution};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, newline},
    combinator::{all_consuming, map, map_res},
    multi::{count, separated_list0, separated_list1},
    sequence::{delimited, separated_pair},
    IResult,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Packet {
    List(Vec<Packet>),
    Value(i32),
}

fn parse_elements(input: &str) -> IResult<&str, Vec<Packet>> {
    separated_list0(tag(","), Packet::parse)(input)
}

impl Packet {
    fn parse(input: &str) -> IResult<&str, Self> {
        let parse_list = |input| delimited(tag("["), parse_elements, tag("]"))(input);
        alt((
            map_res(digit1, |s: &str| s.parse().map(|n| Packet::Value(n))),
            map(parse_list, |packets| Packet::List(packets)),
        ))(input)
    }
}

fn compare_lists(l: &[Packet], r: &[Packet]) -> Ordering {
    for (a, b) in l.iter().zip(r.iter()) {
        match a.cmp(b) {
            Ordering::Less => return Ordering::Less,
            Ordering::Equal => (),
            Ordering::Greater => return Ordering::Greater,
        }
    }
    l.len().cmp(&r.len())
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Packet::Value(l), Packet::Value(r)) => l.cmp(r),
            (Packet::List(l), Packet::List(r)) => compare_lists(l, r),
            (Packet::Value(l), Packet::List(r)) => {
                let l = Packet::List(Vec::from([Packet::Value(*l)]));
                let r = Packet::List(r.clone());
                l.cmp(&r)
            }
            (Packet::List(l), Packet::Value(r)) => {
                let l = Packet::List(l.clone());
                let r = Packet::List(Vec::from([Packet::Value(*r)]));
                l.cmp(&r)
            }
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct PacketPair(Packet, Packet);

impl PacketPair {
    fn ordered(&self) -> bool {
        self.0 < self.1
    }
}

impl PacketPair {
    fn parse(input: &str) -> IResult<&str, Self> {
        map(
            separated_pair(Packet::parse, newline, Packet::parse),
            |(l, r)| PacketPair(l, r),
        )(input)
    }
}

impl ParseInput<'_, { Day::Day13 }> for AOC2022<{ Day::Day13 }> {
    type Parsed = Vec<PacketPair>;

    fn parse_input(&self, input: &'_ str) -> Result<Self::Parsed> {
        let parse_packet_pairs =
            |input| separated_list1(count(newline, 2), PacketPair::parse)(input);
        let (_, packet_pairs) = parse_packet_pairs(input).map_err(|e| e.to_owned()).unwrap();
        Ok(packet_pairs)
    }
}
impl Solution<'_, { Day::Day13 }, { Part::One }> for AOC2022<{ Day::Day13 }> {
    type Input = Vec<PacketPair>;
    type Output = usize;

    fn solve(&self, input: &Self::Input) -> Result<Self::Output> {
        Ok(input
            .iter()
            .enumerate()
            .filter(|(_, pair)| pair.ordered())
            .map(|(i, _)| i)
            .sum())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_packet() -> Result<()> {
        assert_eq!(
            Packet::parse("[1,1,3,1,1]")?,
            (
                "",
                Packet::List(
                    [
                        Packet::Value(1),
                        Packet::Value(1),
                        Packet::Value(3),
                        Packet::Value(1),
                        Packet::Value(1)
                    ]
                    .to_vec()
                )
            )
        );
        assert_eq!(Packet::parse("[]")?, ("", Packet::List([].to_vec())));
        Ok(())
    }

    #[test]
    fn test_parse_pair() -> Result<()> {
        assert_eq!(
            PacketPair::parse(
                "[]
[3]"
            )?,
            (
                "",
                PacketPair(
                    Packet::List([].to_vec()),
                    Packet::List([Packet::Value(3)].to_vec()),
                )
            )
        );
        Ok(())
    }

    #[test]
    fn test() -> Result<()> {
        let problem = super::AOC2022::<{ Day::Day13 }>;
        /*
                let input = "[1,1,3,1,1]
        [1,1,5,1,1]

        [[1],[2,3,4]]
        [[1],4]

        [9]
        [[8,7,6]]

        [[4,4],4,4]
        [[4,4],4,4,4]

        [7,7,7,7]
        [7,7,7]

        []
        [3]

        [[[]]]
        [[]]

        [1,[2,[3,[4,[5,6,7]]]],8,9]
        [1,[2,[3,[4,[5,6,0]]]],8,9]";
                let _ = problem.parse_input(input)?;*/
        Ok(())
    }
}
