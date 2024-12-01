use super::AOC2024;
use anyhow::Result;
use aoc_runner::{Day, ParseInput, Part, Solution};
use counter::Counter;
use std::{num::ParseIntError, str::FromStr};

type Num = i64;
type Pair = (i64, i64);

impl ParseInput<'_, { Day::Day1 }> for AOC2024<{ Day::Day1 }> {
    type Parsed = Vec<Pair>;

    fn parse_input(&self, input: &'_ str) -> Result<Self::Parsed> {
        Ok(input
            .split_whitespace()
            .map(Num::from_str)
            .collect::<Result<Vec<_>, ParseIntError>>()?
            .chunks(2)
            .map(|chunk| (chunk[0], chunk[1]))
            .collect())
    }
}

// manhattan ish
fn list_distance(mut list1: Vec<Num>, mut list2: Vec<Num>) -> u64 {
    list1.sort();
    list2.sort();
    list1
        .into_iter()
        .zip(list2.into_iter())
        .map(|(l, r)| l.abs_diff(r))
        .sum()
}

fn list_similarity(list1: &[Num], list2: &[Num]) -> usize {
    let list2_counts = list2.iter().collect::<Counter<_>>();
    list1
        .iter()
        .map(|n| *n as usize * list2_counts.get(n).unwrap_or(&0))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(
            list_distance(vec![3, 4, 2, 1, 3, 3], vec![4, 3, 5, 3, 9, 3]),
            11
        );
    }
}

impl Solution<'_, { Day::Day1 }, { Part::One }> for AOC2024<{ Day::Day1 }> {
    type Input = Vec<Pair>;
    type Output = u64;

    fn solve(&self, input: &Self::Input) -> Result<Self::Output> {
        let list1 = input.iter().map(|(l, _r)| l.to_owned()).collect();
        let list2 = input.iter().map(|(_l, r)| r.to_owned()).collect();

        Ok(list_distance(list1, list2))
    }
}

impl Solution<'_, { Day::Day1 }, { Part::Two }> for AOC2024<{ Day::Day1 }> {
    type Input = Vec<Pair>;
    type Output = usize;

    fn solve(&self, input: &Self::Input) -> Result<Self::Output> {
        let list1: Vec<i64> = input.iter().map(|(l, _r)| l.to_owned()).collect();
        let list2: Vec<i64> = input.iter().map(|(_l, r)| r.to_owned()).collect();

        Ok(list_similarity(&list1, &list2))
    }
}
