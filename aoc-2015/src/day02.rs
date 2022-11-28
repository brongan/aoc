use std::cmp::min;

use super::AOC2015;
use aoc_runner::{Day, ParseInput, Part, Solution};

use anyhow::Result;
use anyhow::anyhow;

pub struct Box {
    l: u32,
    w: u32,
    h: u32,
}

impl ParseInput<'_, { Day::Day2 }> for AOC2015<{ Day::Day2 }> {
    type Parsed = Vec<Box>;

    fn parse_input(&self, input: &'_ str) -> Result<Self::Parsed> {
        input
            .lines()
            .map(|line| {
                let mut dims = line.split('x');
                let l = dims.next().ok_or_else(|| anyhow!("Missing length"))?.parse()?;
                let w = dims.next().ok_or_else(|| anyhow!("Missing width"))?.parse()?;
                let h = dims.next().ok_or_else(|| anyhow!("Missing height"))?.parse()?;
                Ok(Box { l, w, h })
            })
            .collect()
    }
}

impl Solution<'_, { Day::Day2 }, { Part::One }> for AOC2015<{ Day::Day2 }> {
    type Input = Vec<Box>;
    type Output = u32;

    fn solve(&self, input: &Self::Input) -> Result<Self::Output> {
        Ok(input.iter().map(|dims| {
            let lw = dims.l * dims.w;
            let wh = dims.w * dims.h;
            let hl = dims.h * dims.l;
            2 * lw + 2 * wh + 2 * hl + min(min(lw, wh), hl)
        }).sum())
    }
}

impl Solution<'_, { Day::Day2 }, { Part::Two }> for AOC2015<{ Day::Day2 }> {
    type Input = Vec<Box>;
    type Output = u32;

    fn solve(&self, input: &Self::Input) -> Result<Self::Output> {
        Ok(input.iter().map(|dims| {
            let perim = 2 * min(min(dims.l + dims.h, dims.h + dims.w), dims.l + dims.w);
            let bow = dims.l * dims.w * dims.h;
            perim + bow
        }).sum())
    }
}
