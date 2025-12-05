use std::{cmp::max, range::Range};

use super::AOC2025;
use anyhow::Result;
use aoc_runner::{Day, ParseInput, Part, Solution};

pub struct Input {
    ranges: Vec<Range<u64>>,
    ids: Vec<u64>,
}

type IR = Input;

impl ParseInput<'_, { Day::Day5 }> for AOC2025<{ Day::Day5 }> {
    type Parsed = IR;

    fn parse_input(&self, input: &'_ str) -> Result<Self::Parsed> {
        let mut input = input.split("\n\n");
        let ranges = input.next().unwrap();
        let ids = input.next().unwrap();
        let ranges = ranges
            .lines()
            .map(|line| {
                let mut range = line.split('-');
                let start = u64::from_str_radix(range.next().unwrap(), 10).unwrap();
                let end = u64::from_str_radix(range.next().unwrap(), 10).unwrap() + 1;
                Range { start, end }
            })
            .collect();

        let ids = ids
            .lines()
            .map(|id| u64::from_str_radix(id, 10).unwrap())
            .collect();

        Ok(Input { ranges, ids })
    }
}

impl Solution<'_, { Day::Day5 }, { Part::One }> for AOC2025<{ Day::Day5 }> {
    type Input = IR;
    type Output = usize;

    fn solve(&self, input: &Self::Input) -> Result<Self::Output> {
        Ok(input
            .ids
            .iter()
            .filter(|id| input.ranges.iter().any(|range| range.contains(id)))
            .count())
    }
}

fn merge_intervals(mut intervals: Vec<Range<u64>>) -> Vec<Range<u64>> {
    intervals.sort_by_key(|range| range.start);
    let mut ret = Vec::from([intervals[0]]);
    for interval in &intervals[1..] {
        let last = ret.last_mut().unwrap();
        if interval.start <= last.end {
            last.end = max(last.end, interval.end);
        } else {
            ret.push(*interval);
        }
    }
    ret
}

impl Solution<'_, { Day::Day5 }, { Part::Two }> for AOC2025<{ Day::Day5 }> {
    type Input = IR;
    type Output = u64;

    fn solve(&self, input: &Self::Input) -> Result<Self::Output> {
        Ok(merge_intervals(input.ranges.to_owned())
            .iter()
            .map(|range| range.end - range.start)
            .sum())
    }
}
