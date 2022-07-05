use std::str::FromStr;

use super::AOC2021;
use crate::aoc::{Day, ParseInput, Part, Solution};

#[derive(PartialEq)]
pub enum SyntaxScore {
    Corrupt(char),
    Incomplete(Vec<char>),
}

impl ParseInput<'_, { Day::Day10 }> for AOC2021<{ Day::Day10 }> {
    type Parsed = Vec<SyntaxScore>;

    fn parse_input(&self, input: &'_ str) -> Self::Parsed {
        input
            .split('\n')
            .map(SyntaxScore::from_str)
            .map(|x| x.expect("Failed to parse line"))
            .collect()
    }
}

impl FromStr for SyntaxScore {
    type Err = std::string::ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut stack = Vec::new();
        for c in s.chars() {
            match c {
                '(' => stack.push(')'),
                '[' => stack.push(']'),
                '{' => stack.push('}'),
                '<' => stack.push('>'),
                _ => {
                    if stack.pop() != Some(c) {
                        return Ok(SyntaxScore::Corrupt(c));
                    }
                }
            }
        }
        stack.reverse();
        Ok(SyntaxScore::Incomplete(stack))
    }
}

fn score_line_completion(string: &Vec<char>) -> usize {
    string
        .iter()
        .map(|c| match c {
            ')' => 1,
            ']' => 2,
            '}' => 3,
            '>' => 4,
            _ => panic!("what ze fuk 2"),
        })
        .fold(0, |total, val| (5 * total) + val)
}

fn score_corruption_char(c: &char) -> usize {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => panic!("what ze fuck"),
    }
}

impl SyntaxScore {
    fn score(&self) -> usize {
        match self {
            Self::Corrupt(c) => score_corruption_char(c),
            Self::Incomplete(string) => score_line_completion(string),
        }
    }

    fn is_corrupt(&self) -> bool {
        matches!(self, Self::Corrupt(_))
    }

    fn is_incomplete(&self) -> bool {
        matches!(self, Self::Incomplete(_))
    }
}

impl Solution<'_, { Day::Day10 }, { Part::One }> for AOC2021<{ Day::Day10 }> {
    type Input = Vec<SyntaxScore>;
    type Output = usize;

    fn solve(&self, input: &Self::Input) -> Self::Output {
        input
            .clone()
            .into_iter()
            .filter(|s| s.is_corrupt())
            .map(|s| s.score())
            .sum()
    }
}

impl Solution<'_, { Day::Day10 }, { Part::Two }> for AOC2021<{ Day::Day10 }> {
    type Input = Vec<SyntaxScore>;
    type Output = usize;

    fn solve(&self, input: &Self::Input) -> Self::Output {
        let mut completion_scores: Vec<usize> = input
            .clone()
            .into_iter()
            .filter(|s| s.is_incomplete())
            .map(|s| s.score())
            .collect();
        completion_scores.sort_unstable();
        completion_scores[completion_scores.len() / 2]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aoc::PartOneVerifier;
    use crate::aoc::PartTwoVerifier;

    #[test]
    fn test() -> Result<(), String> {
        let input = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";
        let problem = super::AOC2021::<{ Day::Day10 }>;
        (&&&problem).test_part1(input, 26397)?;
        (&&&problem).test_part2(input, 288957)
    }
}
