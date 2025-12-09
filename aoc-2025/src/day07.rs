use std::collections::HashSet;

use super::AOC2025;
use anyhow::Result;
use aoc_runner::{Day, ParseInput, Part, Solution};

type IR = Vec<Vec<char>>;
type Num = usize;

impl ParseInput<'_, { Day::Day7 }> for AOC2025<{ Day::Day7 }> {
    type Parsed = IR;

    fn parse_input(&self, input: &'_ str) -> Result<Self::Parsed> {
        Ok(input.lines().map(|line| line.chars().collect()).collect())
    }
}

impl Solution<'_, { Day::Day7 }, { Part::One }> for AOC2025<{ Day::Day7 }> {
    type Input = IR;
    type Output = Num;

    fn solve(&self, input: &Self::Input) -> Result<Self::Output> {
        let manifold_index = input[0].iter().position(|x| *x == 'S').unwrap();
        let mut tachyon = HashSet::from([manifold_index]);
        for row in &input[1..] {
            let mut next = HashSet::new();
            for (i, c) in row.iter().enumerate() {
                match (c, tachyon.contains(&i)) {
                    ('^', true) => {
                        next.insert(i - 1);
                        next.insert(i + 1);
                    }
                    ('.', true) => {
                        next.insert(i);
                    }
                    ('^', false) => (),
                    ('.', false) => (),
                    _ => unreachable!("bakana"),
                }
            }
            tachyon = next;
        }
        Ok(tachyon.len())
    }
}
