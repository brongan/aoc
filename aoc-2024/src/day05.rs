use std::collections::{HashMap, HashSet};

use super::AOC2024;
use anyhow::Result;
use aoc_runner::{Day, ParseInput, Part, Solution};
use nom::{
    bytes::complete::tag,
    character::complete::{digit1, newline},
    combinator::{map, map_res},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

#[derive(Debug)]
struct Rule {
    before: Num,
    after: Num,
}

#[derive(Debug)]
pub struct Manual {
    rules: Vec<Rule>,
    pages: Vec<Vec<Num>>,
}

type IR = Manual;
type Num = i32;

fn parse_element(input: &str) -> IResult<&str, Num> {
    map_res(digit1, |num| Num::from_str_radix(num, 10))(input)
}

fn parse_rule(input: &str) -> IResult<&str, Rule> {
    map(
        separated_pair(parse_element, tag("|"), parse_element),
        |(before, after)| Rule { before, after },
    )(input)
}

fn parse_rules(input: &str) -> IResult<&str, Vec<Rule>> {
    separated_list1(newline, parse_rule)(input)
}

fn parse_pages(input: &str) -> IResult<&str, Vec<Vec<Num>>> {
    let parse_page = |input| separated_list1(tag(","), parse_element)(input);
    separated_list1(newline, parse_page)(input)
}

fn parse_manual(input: &str) -> IResult<&str, Manual> {
    map(
        separated_pair(parse_rules, tag("\n\n"), parse_pages),
        |(rules, pages)| Manual { rules, pages },
    )(input)
}

impl ParseInput<'_, { Day::Day5 }> for AOC2024<{ Day::Day5 }> {
    type Parsed = IR;

    fn parse_input(&self, input: &'_ str) -> Result<Self::Parsed> {
        let (_, manual) = parse_manual(input).map_err(|e| e.to_owned())?;
        Ok(manual)
    }
}

fn is_valid(page: &[Num], rules: &[Rule]) -> bool {
    let mut seen = HashSet::new();
    for num in page {
        for rule in rules {
            if *num == rule.before && seen.contains(&rule.after) {
                return false;
            }
        }
        seen.insert(num);
    }
    true
}

fn fix_inversion(page: &mut Vec<Num>, rules: &[Rule]) -> bool {
    let mut seen = HashMap::new();
    for i in 0..page.len() {
        let num = page[i];
        for rule in rules {
            if num == rule.before && seen.contains_key(&rule.after) {
                page.swap(i, seen[&rule.after]);
                return true;
            }
        }
        seen.insert(num, i);
    }
    false
}

fn make_valid(page: &[Num], rules: &[Rule]) -> Vec<Num> {
    let mut page = page.to_owned();

    while fix_inversion(&mut page, rules) {}

    return page;
}

impl Solution<'_, { Day::Day5 }, { Part::One }> for AOC2024<{ Day::Day5 }> {
    type Input = IR;
    type Output = Num;

    fn solve(&self, input: &Self::Input) -> Result<Self::Output> {
        Ok(input
            .pages
            .iter()
            .filter(|page| is_valid(&page, &input.rules))
            .map(|page| page[page.len() / 2])
            .sum())
    }
}

impl Solution<'_, { Day::Day5 }, { Part::Two }> for AOC2024<{ Day::Day5 }> {
    type Input = IR;
    type Output = Num;

    fn solve(&self, input: &Self::Input) -> Result<Self::Output> {
        Ok(input
            .pages
            .iter()
            .filter(|page| !is_valid(&page, &input.rules))
            .map(|page| make_valid(&page, &input.rules))
            .map(|page| page[page.len() / 2])
            .sum())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        todo!()
    }
}
