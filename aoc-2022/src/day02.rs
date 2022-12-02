use super::AOC2022;
use aoc_runner::{Day, ParseInput, Part, Solution};

use anyhow::anyhow;
use anyhow::Context;
use anyhow::Result;

pub enum Choice {
    Rock,
    Paper,
    Scissors,
}

pub enum Strategy {
    X,
    Y,
    Z,
}

pub struct Game {
    enemy: Choice,
    strategy: Strategy,
}

impl Game {
    fn score(&self, choice: Choice) -> u32 {
        let mut score = match choice {
            Choice::Rock => 1,
            Choice::Paper => 2,
            Choice::Scissors => 3,
        };
        score += match (&self.enemy, choice) {
            (Choice::Rock, Choice::Paper) => 6,
            (Choice::Rock, Choice::Scissors) => 0,
            (Choice::Paper, Choice::Rock) => 0,
            (Choice::Paper, Choice::Scissors) => 6,
            (Choice::Scissors, Choice::Rock) => 6,
            (Choice::Scissors, Choice::Paper) => 0,
            _ => 3,
        };
        score
    }

    fn part1(&self) -> u32 {
        self.score(match self.strategy {
            Strategy::X => Choice::Rock,
            Strategy::Y => Choice::Paper,
            Strategy::Z => Choice::Scissors,
        })
    }

    fn part2(&self) -> u32 {
        self.score(match (&self.enemy, &self.strategy) {
            (Choice::Rock, Strategy::X) => Choice::Scissors,
            (Choice::Rock, Strategy::Y) => Choice::Rock,
            (Choice::Rock, Strategy::Z) => Choice::Paper,
            (Choice::Paper, Strategy::X) => Choice::Rock,
            (Choice::Paper, Strategy::Y) => Choice::Paper,
            (Choice::Paper, Strategy::Z) => Choice::Scissors,
            (Choice::Scissors, Strategy::X) => Choice::Paper,
            (Choice::Scissors, Strategy::Y) => Choice::Scissors,
            (Choice::Scissors, Strategy::Z) => Choice::Rock,
        })
    }
}

impl ParseInput<'_, { Day::Day2 }> for AOC2022<{ Day::Day2 }> {
    type Parsed = Vec<Game>;

    fn parse_input(&self, input: &'_ str) -> Result<Self::Parsed> {
        input
            .lines()
            .map(|line| {
                line.split_once(' ')
                    .map(|(l, r)| {
                        let enemy = match l {
                            "A" => Ok(Choice::Rock),
                            "B" => Ok(Choice::Paper),
                            "C" => Ok(Choice::Scissors),
                            _ => Err(anyhow!("Invalid enemy choice")),
                        }?;
                        let suggestion = match r {
                            "X" => Ok(Strategy::X),
                            "Y" => Ok(Strategy::Y),
                            "Z" => Ok(Strategy::Z),
                            _ => Err(anyhow!("Invalid suggestion choice")),
                        }?;
                        Ok(Game {
                            enemy,
                            strategy: suggestion,
                        })
                    })
                    .context("Failed to split line")?
            })
            .collect::<Result<Self::Parsed>>()
    }
}

impl Solution<'_, { Day::Day2 }, { Part::One }> for AOC2022<{ Day::Day2 }> {
    type Input = Vec<Game>;
    type Output = u32;

    fn solve(&self, input: &Self::Input) -> Result<Self::Output> {
        Ok(input.iter().map(|game| game.part1()).sum())
    }
}

impl Solution<'_, { Day::Day2 }, { Part::Two }> for AOC2022<{ Day::Day2 }> {
    type Input = Vec<Game>;
    type Output = u32;

    fn solve(&self, input: &Self::Input) -> Result<Self::Output> {
        Ok(input.iter().map(|game| game.part2()).sum())
    }
}
