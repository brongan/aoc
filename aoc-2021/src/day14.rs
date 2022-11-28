use super::AOC2021;
use anyhow::Result;
use aoc_runner::{Day, ParseInput, Part, Solution};
use std::collections::HashMap;

use counter::Counter;

type Pair = (char, char);

pub struct Input {
    template: Vec<char>,
    rules: HashMap<Pair, char>,
}

impl ParseInput<'_, { Day::Day14 }> for AOC2021<{ Day::Day14 }> {
    type Parsed = Input;

    fn parse_input(&self, input: &'_ str) -> Result<Self::Parsed> {
        let input = input.trim().split_once("\n\n").expect("invalid input");
        let template = input.0.trim().chars().collect();
        let rules: HashMap<(char, char), char> = input
            .1
            .trim()
            .split('\n')
            .map(|line| {
                let terms = line.split_once(" -> ").expect("invalid line");
                let mut pair_input = terms.0.chars();
                (
                    (
                        pair_input.next().expect("invalid rule"),
                        pair_input.next().expect("invalid rule"),
                    ),
                    terms.1.chars().next().expect("invalid rule"),
                )
            })
            .collect();
        Ok(Input { template, rules })
    }
}

fn run_polymerization(
    template: &[char],
    rules: &HashMap<(char, char), char>,
    steps: usize,
) -> usize {
    let mut pair_counts: Counter<Pair> =
        Counter::init(template.windows(2).map(|window| (window[0], window[1])));
    for _ in 0..steps {
        let mut new_pair_counts: Counter<Pair> = Counter::new();
        for ((a, c), count) in pair_counts.iter() {
            let b = rules[&(*a, *c)];
            new_pair_counts[&(*a, b)] += count;
            new_pair_counts[&(b, *c)] += count;
        }
        pair_counts = new_pair_counts;
    }

    let mut char_counts: Counter<char> = Counter::new();
    for ((a, b), count) in pair_counts.iter() {
        char_counts[a] += count;
        char_counts[b] += count;
    }
    for (_c, count) in char_counts.iter_mut() {
        *count /= 2;
    }
    return char_counts.values().max().unwrap() - char_counts.values().min().unwrap() + 1;
}

impl Solution<'_, { Day::Day14 }, { Part::One }> for AOC2021<{ Day::Day14 }> {
    type Input = Input;
    type Output = usize;

    fn solve(&self, input: &Self::Input) -> Result<Self::Output> {
        Ok(run_polymerization(&input.template, &input.rules, 10))
    }
}

impl Solution<'_, { Day::Day14 }, { Part::Two }> for AOC2021<{ Day::Day14 }> {
    type Input = Input;
    type Output = usize;

    fn solve(&self, input: &Self::Input) -> Result<Self::Output> {
        Ok(run_polymerization(&input.template, &input.rules, 40))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_runner::PartOneVerifier;
    use aoc_runner::PartTwoVerifier;

    #[test]
    fn test() -> Result<()> {
        let input = "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";
        let problem = super::AOC2021::<{ Day::Day14 }>;
        problem.test_part1(input, 1588)?;
        problem.test_part2(input, 2188189693529)
    }
}
