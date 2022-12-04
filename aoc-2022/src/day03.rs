use super::AOC2022;
use aoc_runner::{Day, ParseInput, Part, Solution};

use anyhow::Context;
use anyhow::Result;
use std::collections::HashSet;

type Ingredient = char;

pub struct Rucksack {
    left_compartment: HashSet<Ingredient>,
    right_compartment: HashSet<Ingredient>,
}

impl std::fmt::Debug for Rucksack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Rucksack").field("contents", &self.union()).finish()
    }
}

impl Rucksack {
    fn intersection(&self) -> Result<Ingredient> {
        self.left_compartment
            .intersection(&self.right_compartment)
            .next()
            .copied()
            .context("No intersection")
    }

    fn union(&self) -> HashSet<Ingredient> {
        self.left_compartment
            .union(&self.right_compartment)
            .copied()
            .collect()
    }
}

fn priority(ingredient: char) -> Result<u32> {
    if ingredient.is_ascii_uppercase() {
        Ok(27 + ingredient as u32 - 'A' as u32)
    } else if ingredient.is_ascii_lowercase() {
        Ok(1 + ingredient as u32 - 'a' as u32)
    } else {
        Err(anyhow::anyhow!("Invalid ingredient: {}", ingredient))
    }
}

#[test]
fn test_priority() -> Result<()> {
    assert_eq!(priority('A')?, 27);
    assert_eq!(priority('Z')?, 52);
    assert_eq!(priority('a')?, 1);
    assert_eq!(priority('z')?, 26);
    assert_eq!(priority('p')?, 16);
    assert_eq!(priority('P')?, 42);
    assert_eq!(priority('L')?, 38);
    assert_eq!(priority('v')?, 22);
    assert_eq!(priority('t')?, 20);
    Ok(())
}

impl ParseInput<'_, { Day::Day3 }> for AOC2022<{ Day::Day3 }> {
    type Parsed = Vec<Rucksack>;

    fn parse_input(&self, input: &'_ str) -> Result<Self::Parsed> {
        input
            .lines()
            .map(|line| {
                let split = line.split_at(line.len() / 2);
                Ok(Rucksack {
                    left_compartment: split.0.chars().collect(),
                    right_compartment: split.1.chars().collect(),
                })
            })
            .collect()
    }
}

impl Solution<'_, { Day::Day3 }, { Part::One }> for AOC2022<{ Day::Day3 }> {
    type Input = Vec<Rucksack>;
    type Output = u32;

    fn solve(&self, input: &Self::Input) -> Result<Self::Output> {
        input
            .iter()
            .map(|rucksack| -> Result<u32> { priority(rucksack.intersection()?) })
            .sum::<Result<u32>>()
    }
}

impl Solution<'_, { Day::Day3 }, { Part::Two }> for AOC2022<{ Day::Day3 }> {
    type Input = Vec<Rucksack>;
    type Output = u32;

    fn solve(&self, input: &Self::Input) -> Result<Self::Output> {
        input
            .chunks(3)
            .enumerate()
            .map(|(i, rucksacks)| -> Result<u32> {
                priority(
                    *rucksacks[0]
                        .union()
                        .intersection(&rucksacks[1].union())
                        .copied()
                        .collect::<HashSet<_>>()
                        .intersection(&rucksacks[2].union())
                        .next()
                        .context(format!("no intersection between elves in group {}: {:?}", i, rucksacks))?,
                )
            })
            .sum()
    }
}
