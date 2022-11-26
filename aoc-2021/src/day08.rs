use enumset::{EnumSetType, enum_set};

use super::AOC2021;
use aoc_runner::{Day, ParseInput, Part, Solution};
use std::{str::FromStr, collections::HashMap};

#[allow(dead_code)]
pub struct Entry {
    signal: Vec<String>,
    output: Vec<String>,
}

impl ParseInput<'_, { Day::Day8 }> for AOC2021<{ Day::Day8 }> {
    type Parsed = Vec<Entry>;

    fn parse_input(&self, input: &'_ str) -> Self::Parsed {
        input
            .trim()
            .lines()
            .map(Entry::from_str)
            .map(|r| r.expect("failed to parse entry"))
            .collect()
    }
}

impl FromStr for Entry {
    type Err = std::string::ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (signal, output) = s.split_once('|').expect("Failed to split line");
        let signal = signal
            .split(' ')
            .map(|word| word.chars().collect())
            .collect();
        let output = output
            .split(' ')
            .map(|word| word.chars().collect())
            .collect();
        Ok(Entry { signal, output })
    }
}

impl Solution<'_, { Day::Day8 }, { Part::One }> for AOC2021<{ Day::Day8 }> {
    type Input = Vec<Entry>;
    type Output = usize;

    fn solve(&self, input: &Self::Input) -> Self::Output {
        input
            .into_iter()
            .map(|entry| {
                entry
                    .output
                    .iter()
                    .filter(|digit| {
                        digit.len() == 2 || digit.len() == 4 || digit.len() == 3 || digit.len() == 7
                    })
                    .count()
            })
            .sum()
    }
}

enum Digit {
    Digit0,
    Digit1,
    Digit2,
    Digit3,
    Digit4,
    Digit5,
    Digit6,
    Digit7,
    Digit8,
    Digit9,
}
#[derive(EnumSetType, Debug)]
enum Segment {
    Top,
    TopLeft,
    TopRight,
    Middle,
    BottomLeft,
    BottomRight,
    Bottom,
}

impl Solution<'_, { Day::Day8 }, { Part::Two }> for AOC2021<{ Day::Day8 }> {
    type Input = Vec<Entry>;
    type Output = usize;

    fn solve(&self, input: &Self::Input) -> Self::Output {
        // len() == 2 => 1
        // len() == 4 => 4
        // len() == 3 => 7
        // len() == 7 => 8
        // Map from letter to Segment(s)
        // Map from Segments to Digit
        let mut segment_to_digit = HashMap::new();
        segment_to_digit.insert(enum_set!(Segment::TopLeft | Segment::Top | Segment::TopRight | Segment::BottomLeft | Segment::BottomRight | Segment::Bottom), Digit::Digit0);
        segment_to_digit.insert(enum_set!(Segment::TopRight | Segment::BottomRight), Digit::Digit1);
        segment_to_digit.insert(enum_set!(Segment::Top| Segment::TopRight | Segment::Middle | Segment::BottomLeft | Segment::Bottom), Digit::Digit2);
        segment_to_digit.insert(enum_set!(Segment::Top | Segment::TopRight | Segment::Middle | Segment::BottomRight | Segment::Bottom), Digit::Digit3);
        segment_to_digit.insert(enum_set!(Segment::TopLeft | Segment::TopRight | Segment::Middle | Segment::BottomRight), Digit::Digit4);
        segment_to_digit.insert(enum_set!(Segment::TopLeft | Segment::Top | Segment::Middle | Segment::BottomRight | Segment::Bottom), Digit::Digit5);
        segment_to_digit.insert(enum_set!(Segment::TopLeft | Segment::Top | Segment::Middle | Segment::BottomLeft | Segment::BottomRight | Segment::Bottom), Digit::Digit6);
        segment_to_digit.insert(enum_set!(Segment::Top | Segment::TopRight | Segment::BottomRight), Digit::Digit7);
        segment_to_digit.insert(enum_set!(Segment::TopLeft | Segment::Top | Segment::TopRight | Segment::Middle | Segment::BottomLeft | Segment::BottomRight | Segment::Bottom), Digit::Digit8);
        segment_to_digit.insert(enum_set!(Segment::TopLeft | Segment::Top | Segment::TopRight | Segment::Middle |  Segment::BottomRight | Segment::Bottom), Digit::Digit9);

        input
            .into_iter()
            .map(|entry| {
                entry
                    .output
                    .iter()
                    .filter(|digit| {
                        digit.len() == 2 || digit.len() == 4 || digit.len() == 3 || digit.len() == 7
                    })
                    .count()
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_runner::PartOneVerifier;
    use aoc_runner::PartTwoVerifier;

    #[test]
    fn test() -> Result<(), String> {
        let input = "
be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb |fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec |fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef |cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega |efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga |gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf |gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf |cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd |ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg |gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc |fgae cfgab fg bagce";
        let problem = super::AOC2021::<{ Day::Day8 }>;
        (&&&problem).test_part1(input, 26)?;
        (&&&problem).test_part2(input, 61229)
    }
}
