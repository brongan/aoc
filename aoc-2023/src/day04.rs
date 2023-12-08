use super::AOC2023;
use anyhow::Result;
use aoc_runner::{Day, ParseInput, Part, Solution};
use nom::{
    bytes::complete::tag,
    character::complete::{digit1, multispace0, multispace1, newline},
    combinator::{map, map_res},
    multi::separated_list1,
    sequence::{preceded, separated_pair, terminated, tuple},
    IResult,
};

type Num = u32;

pub struct ScratchCard {
    winning: Vec<Num>,
    numbers: Vec<Num>,
}

impl ScratchCard {
    fn num_winning(&self) -> usize {
        self.numbers
            .iter()
            .filter(|num| self.winning.contains(num))
            .count()
    }

    fn score(&self) -> Num {
        match self.num_winning() {
            0 => 0,
            n => {
                let base: Num = 2;
                base.pow(n as Num - 1)
            }
        }
    }
}

fn parse_vec(input: &str) -> IResult<&str, Vec<Num>> {
    let parse_num = |input| map_res(digit1, |num: &str| Num::from_str_radix(num, 10))(input);
    separated_list1(multispace1, parse_num)(input)
}

fn parse_id(input: &str) -> IResult<&str, usize> {
    preceded(
        tag("Card"),
        preceded(
            multispace0,
            terminated(map_res(digit1, |num: &str| num.parse()), tag(":")),
        ),
    )(input)
}

fn parse_scratchcard(input: &str) -> IResult<&str, ScratchCard> {
    let (input, _id): (&str, usize) = parse_id(input)?;
    map(
        preceded(
            multispace0,
            separated_pair(
                parse_vec,
                tuple((multispace0, tag("|"), multispace0)),
                parse_vec,
            ),
        ),
        |(winning, numbers)| ScratchCard { winning, numbers },
    )(input)
}

impl ParseInput<'_, { Day::Day4 }> for AOC2023<{ Day::Day4 }> {
    type Parsed = Vec<ScratchCard>;

    fn parse_input(&self, input: &'_ str) -> Result<Self::Parsed> {
        let (_, ret) =
            separated_list1(newline, parse_scratchcard)(input).map_err(|e| e.to_owned())?;
        Ok(ret)
    }
}

impl Solution<'_, { Day::Day4 }, { Part::One }> for AOC2023<{ Day::Day4 }> {
    type Input = Vec<ScratchCard>;
    type Output = u32;

    fn solve(&self, input: &Self::Input) -> Result<Self::Output> {
        Ok(input.iter().map(|scratchcard| scratchcard.score()).sum())
    }
}

impl Solution<'_, { Day::Day4 }, { Part::Two }> for AOC2023<{ Day::Day4 }> {
    type Input = Vec<ScratchCard>;
    type Output = u32;

    fn solve(&self, input: &Self::Input) -> Result<Self::Output> {
        let mut card_counts: Vec<u32> = input.iter().map(|_| 1).collect();
        for (i, card) in input.iter().enumerate() {
            let win_count = card.num_winning();
            for j in i + 1..(i + win_count + 1) {
                card_counts[j] += card_counts[i];
            }
        }
        Ok(card_counts.iter().sum())
    }
}
