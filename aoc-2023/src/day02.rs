use super::AOC2023;
use anyhow::Result;
use aoc_runner::{Day, ParseInput, Part, Solution};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, space1},
    combinator::{all_consuming, map, map_res, recognize},
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    IResult,
};
use std::cmp::max;

#[derive(Debug, PartialEq, PartialOrd)]
pub enum Color {
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

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct Subset {
    red: usize,
    green: usize,
    blue: usize,
}

impl Subset {
    fn power(&self) -> usize {
        self.red * self.green * self.blue
    }
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
    let (input, pairs): (&str, Vec<(_, _)>) = separated_list1(tag(", "), parse_cube)(input)?;
    for pairs in &pairs {
        let (count, color): &(usize, Color) = pairs;
        *(match color {
            Color::Red => &mut ret.red,
            Color::Blue => &mut ret.blue,
            Color::Green => &mut ret.green,
        }) = *count;
    }
    Ok((input, ret))
}

#[derive(Debug, PartialEq, PartialOrd)]
pub struct Game {
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

    fn parse_input(&self, input: &'_ str) -> Result<Self::Parsed> {
        input
            .lines()
            .map(|line| {
                let (_, ret) = parse_game(line).map_err(|e| e.to_owned())?;
                Ok(ret)
            })
            .collect()
    }
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

fn smallest_possible(subsets: &[Subset]) -> Subset {
    let mut smallest = subsets[0].clone();
    for subset in subsets {
        smallest.red = max(smallest.red, subset.red);
        smallest.blue = max(smallest.blue, subset.blue);
        smallest.green = max(smallest.green, subset.green);
    }
    smallest
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

impl Solution<'_, { Day::Day2 }, { Part::Two }> for AOC2023<{ Day::Day2 }> {
    type Input = Vec<Game>;
    type Output = usize;

    fn solve(&self, input: &Self::Input) -> Result<Self::Output> {
        Ok(input
            .iter()
            .map(|game| smallest_possible(&game.subsets))
            .map(|subset| subset.power())
            .sum())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_runner::PartOneVerifier;
    use aoc_runner::PartTwoVerifier;

    #[test]
    fn test_parse_color() -> Result<()> {
        assert_eq!(parse_color("green")?, ("", Color::Green));
        assert_eq!(parse_color("red")?, ("", Color::Red));
        assert_eq!(parse_color("blue")?, ("", Color::Blue));
        Ok(())
    }

    #[test]
    fn test_parse_subset() -> Result<()> {
        assert_eq!(
            parse_subset("1 green, 6 red, 4 blue")?,
            (
                "",
                Subset {
                    red: 6,
                    blue: 4,
                    green: 1
                }
            )
        );
        Ok(())
    }

    #[test]
    fn test_parse_game() -> Result<()> {
        assert_eq!(parse_game("Game 1: 1 green, 6 red, 4 blue; 2 blue, 6 green, 7 red; 3 red, 4 blue, 6 green; 3 green; 3 blue, 2 green, 1 red")?, 
                   ("", Game {id: 1,subsets: [Subset { red: 6, green: 1, blue: 4}, Subset { red: 7, blue: 2, green: 6}, Subset {red: 3, blue: 4, green: 6}, Subset {red: 0, blue: 0, green: 3}, Subset {red: 1, blue: 3, green: 2}].to_vec()}));
        Ok(())
    }

    #[test]
    fn test() -> Result<()> {
        let problem = super::AOC2023::<{ Day::Day2 }>;
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        problem.test_part1(input, 8)?;
        problem.test_part2(input, 2286)
    }
}
