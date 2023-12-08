use super::AOC2023;
use anyhow::Result;
use aoc_runner::{Day, ParseInput, Part, Solution};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, space1},
    combinator::{all_consuming, map, map_res, recognize},
    multi::{many1, separated_list1},
    sequence::{preceded, separated_pair},
    IResult,
};

enum Color {
    Red,
    Green,
    Blue,
}

fn parse_color(input: &str) -> IResult<&str, Color> {
    let red = |input| map(tag("red"), |_| Color::Red)(input);
    let green = |input| map(tag("green"), |_| Color::Green)(input);
    let blue = |input| map(tag("blue"), |_| Color::Blue)(input);
    alt((red, green, blue))(input)
}

struct Subset {
    red: u32,
    green: u32,
    blue: u32,
}

fn parse_num(input: &str) -> IResult<&str, usize> {
    map_res(recognize(digit1), str::parse)(input)
}

fn parse_subset(input: &str) -> IResult<&str, Subset> {
    let mut ret = Subset {
        red: 0,
        green: 0,
        blue: 0,
    };
    let parse_cube = |input| separated_pair(parse_num, space1, parse_color)(input);
    let (input, pairs): (&str, Vec<(_, _)>) = many1(parse_cube)(input)?;
    for pair in &pairs {
        let (count, color): (usize, Color) = pairs;
        *(match color {
            Color::Red => &mut ret.red,
            Color::Blue => &mut ret.blue,
            Color::Green => &mut ret.green,
        }) = count;
    }
    Ok((input, ret))
}

struct Game {
    id: usize,
    subsets: Vec<Subset>,
}

fn parse_game(input: &str) -> IResult<&str, Game> {
    let parse_subsets = |input| separated_list1(tag("; "), parse_subset)(input);
    let (input, id) = preceded(tag("Game "), parse_num)(input)?;
    let (input, subsets) = preceded(tag(": "), all_consuming(parse_subsets))(input)?;
    Ok((input, Game { id, subsets }))
}

impl ParseInput<'_, { Day::Day2 }> for AOC2023<{ Day::Day2 }> {
    type Parsed = Vec<Game>;

    fn parse_input(&self, input: &'_ str) -> Result<Self::Parsed> {}
}

fn is_possible(game: &[Subset], bag: &Subset) -> bool {
    for subset in game {
        if subset.red > bag.red {
            return false;
        }
        if subset.blue > bag.blue {
            return false;
        }
        if subset.green > bag.green {
            return false;
        }
    }
    true
}

impl Solution<'_, { Day::Day2 }, { Part::One }> for AOC2023<{ Day::Day2 }> {
    type Input = Vec<Game>;
    type Output = usize;

    fn solve(&self, input: &Self::Input) -> Result<Self::Output> {
        let bag = Subset {
            red: 12,
            green: 13,
            blue: 14,
        };
        Ok(input
            .iter()
            .filter(|game| is_possible(&game.subsets, &bag))
            .map(|game| game.id)
            .sum())
    }
}
