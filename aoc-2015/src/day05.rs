use std::collections::HashMap;

use super::AOC2015;

use aoc_runner::{Day, ParseInput, Part, Solution};

use anyhow::Result;

impl ParseInput<'_, { Day::Day5 }> for AOC2015<{ Day::Day5 }> {
    type Parsed = Vec<String>;

    fn parse_input(&self, input: &'_ str) -> Result<Self::Parsed> {
        Ok(input.lines().map(|l| l.to_string()).collect())
    }
}

fn is_vowel(c: char) -> bool {
    matches!(c, 'a' | 'e' | 'i' | 'o' | 'u')
}

fn condition1(string: &str) -> bool {
    string.matches(is_vowel).count() >= 3
}

fn condition2(string: &str) -> bool {
    string.as_bytes().windows(2).any(|w| w[0] == w[1])
}

fn condition3(string: &str) -> bool {
    string.contains("ab") || string.contains("cd") || string.contains("pq") || string.contains("xy")
}

impl Solution<'_, { Day::Day5 }, { Part::One }> for AOC2015<{ Day::Day5 }> {
    type Input = Vec<String>;
    type Output = usize;

    fn solve(&self, input: &Self::Input) -> Result<Self::Output> {
        Ok(input
            .iter()
            .filter(|string| condition1(string) && condition2(string) && !condition3(string))
            .count())
    }
}

fn condition4(string: &str) -> bool {
}

fn condition5(string: &str) -> bool {
    string.as_bytes().windows(3).any(|w| w[0] == w[2])
}

impl Solution<'_, { Day::Day5 }, { Part::Two }> for AOC2015<{ Day::Day5 }> {
    type Input = Vec<String>;
    type Output = usize;

    fn solve(&self, input: &Self::Input) -> Result<Self::Output> {
        Ok(input.iter().filter(|string| condition4(string) && condition5(string)).count())
    }
}
