use super::AOC2023;
use anyhow::Result;
use aoc_runner::{Day, ParseInput, Part, Solution};
use counter::Counter;
use nom::{
    bytes::complete::take,
    character::complete::{digit1, multispace0, newline},
    combinator::{all_consuming, map, map_res},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};
use std::cmp::Ordering;

type Num = i32;

#[derive(PartialOrd, Ord, PartialEq, Eq, Debug, Hash, Clone)]
pub enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jacker,
    Queen,
    King,
    Ace,
}

impl Card {
    fn part_2_value(&self) -> i32 {
        match self {
            Card::Two => 2,
            Card::Three => 3,
            Card::Four => 4,
            Card::Five => 5,
            Card::Six => 6,
            Card::Seven => 7,
            Card::Eight => 8,
            Card::Nine => 9,
            Card::Ten => 10,
            Card::Jacker => 1,
            Card::Queen => 11,
            Card::King => 12,
            Card::Ace => 13,
        }
    }

    fn compare(&self, other: &Self, part: Part) -> Ordering {
        match part {
            Part::One => self.cmp(other),
            Part::Two => self.part_2_value().cmp(&other.part_2_value()),
        }
    }
}

#[derive(PartialOrd, Ord, PartialEq, Eq, Debug, Clone)]
enum Type {
    HighCard,
    Pair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(PartialEq, Eq, Debug, Clone)]
struct Hand([Card; 5]);

fn get_type_from_counts(counts: &Counter<&Card>) -> Type {
    let most_common = counts.most_common();
    if most_common[0].1 == 5 {
        return Type::FiveOfAKind;
    }
    if most_common[0].1 == 4 {
        return Type::FourOfAKind;
    }
    if most_common[0].1 == 3 {
        if most_common[1].1 == 2 {
            return Type::FullHouse;
        }
        return Type::ThreeOfAKind;
    }
    if most_common[0].1 == 2 {
        if most_common[1].1 == 2 {
            return Type::TwoPair;
        }
        return Type::Pair;
    }
    return Type::HighCard;
}

fn score_hand_1(hand: &Hand) -> Type {
    let counts = hand.0.iter().collect::<Counter<_>>();
    get_type_from_counts(&counts)
}

fn score_hand_2(hand: &Hand) -> Type {
    let mut counts = hand.0.iter().collect::<Counter<_>>();
    let joker_count = *counts.get(&Card::Jacker).unwrap_or(&0);
    if let Some(other) = counts
        .most_common()
        .iter()
        .filter(|(card, _)| **card != Card::Jacker)
        .map(|(card, _)| card)
        .next()
    {
        *counts.get_mut(other).unwrap() += joker_count;
        counts.insert(&Card::Jacker, 0);
    }

    get_type_from_counts(&counts)
}

impl Hand {
    fn cmp(&self, other: &Self, part: Part) -> Ordering {
        let ordering = match part {
            Part::One => score_hand_1(self).cmp(&score_hand_1(other)),
            Part::Two => score_hand_2(self).cmp(&score_hand_2(other)),
        };
        match ordering {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => {
                for i in 0..6 {
                    match self.0[i].compare(&other.0[i], part.clone()) {
                        Ordering::Less => return Ordering::Less,
                        Ordering::Greater => return Ordering::Greater,
                        Ordering::Equal => (),
                    }
                }
                Ordering::Equal
            }
        }
    }
}

#[derive(PartialEq, Eq, Clone)]
pub struct Play {
    hand: Hand,
    bid: Num,
}

fn parse_card(input: u8) -> Card {
    match input as char {
        'A' => Card::Ace,
        'K' => Card::King,
        'Q' => Card::Queen,
        'J' => Card::Jacker,
        'T' => Card::Ten,
        '9' => Card::Nine,
        '8' => Card::Eight,
        '7' => Card::Seven,
        '6' => Card::Six,
        '5' => Card::Five,
        '4' => Card::Four,
        '3' => Card::Three,
        '2' => Card::Two,
        _ => panic!("illegal char"),
    }
}

fn parse_hand(input: &str) -> IResult<&str, Hand> {
    let (input, slice): (&str, &str) = take(5usize)(input)?;
    let bytes = slice.as_bytes();
    let hand = [
        parse_card(bytes[0]),
        parse_card(bytes[1]),
        parse_card(bytes[2]),
        parse_card(bytes[3]),
        parse_card(bytes[4]),
    ];
    Ok((input, Hand(hand)))
}

fn parse_num(input: &str) -> IResult<&str, Num> {
    map_res(digit1, |num: &str| Num::from_str_radix(num, 10))(input)
}

fn parse_play(input: &str) -> IResult<&str, Play> {
    map(
        separated_pair(parse_hand, multispace0, parse_num),
        |(hand, bid)| Play { hand, bid },
    )(input)
}

fn parse_game(input: &str) -> IResult<&str, Vec<Play>> {
    separated_list1(newline, parse_play)(input)
}

impl ParseInput<'_, { Day::Day7 }> for AOC2023<{ Day::Day7 }> {
    type Parsed = Vec<Play>;

    fn parse_input(&self, input: &'_ str) -> Result<Self::Parsed> {
        let (_, game) = all_consuming(parse_game)(input.trim()).map_err(|e| e.to_owned())?;
        Ok(game)
    }
}

fn total_winnings(plays: &[Play], part: Part) -> i32 {
    let mut input: Vec<Play> = plays.to_vec();
    input.sort_by(|a, b| a.hand.cmp(&b.hand, part.clone()));
    input
        .iter()
        .enumerate()
        .map(|(i, play)| (i + 1) as i32 * play.bid)
        .sum()
}

impl Solution<'_, { Day::Day7 }, { Part::One }> for AOC2023<{ Day::Day7 }> {
    type Input = Vec<Play>;
    type Output = i32;

    fn solve(&self, input: &Self::Input) -> Result<Self::Output> {
        Ok(total_winnings(input, Part::One))
    }
}

impl Solution<'_, { Day::Day7 }, { Part::Two }> for AOC2023<{ Day::Day7 }> {
    type Input = Vec<Play>;
    type Output = i32;

    fn solve(&self, input: &Self::Input) -> Result<Self::Output> {
        Ok(total_winnings(input, Part::Two))
    }
}

#[cfg(test)]
mod tests {
    use aoc_runner::{PartOneVerifier, PartTwoVerifier};

    use super::*;

    #[test]
    fn test() -> Result<()> {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        let problem = super::AOC2023::<{ Day::Day7 }>;
        problem.test_part1(input, 6440)?;
        problem.test_part2(input, 5905)
    }
}
