use super::AOC2023;
use anyhow::{Context, Result};
use aoc_runner::{point2d::Point2D, Day, ParseInput, Part, Solution};
use std::collections::HashMap;

fn numbers_near_symbols(&self) -> Vec<i32> {
    self.numbers
        .iter()
        .filter(|(p, v)| {
            if self.symbols.contains_key(&Point { x: p.x - 1, y: p.y }) {
                return true;
            }
            if self.symbols.contains_key(&Point { x: p.x + 1, y: p.y }) {
                return true;
            }
            if self.symbols.contains_key(&Point { x: p.x, y: p.y - 1 }) {
                return true;
            }
            if self.symbols.contains_key(&Point { x: p.x, y: p.y + 1 }) {
                return true;
            }
            false
        })
        .map(|(p, v)| v)
        .collect()
}

impl ParseInput<'_, { Day::Day3 }> for AOC2023<{ Day::Day3 }> {
    type Parsed = Vec<Vec<char>>;

    fn parse_input(&self, input: &'_ str) -> Result<Self::Parsed> {
        Ok(input.lines().map(|line| line.as_bytes().clone()).collect())
    }
}

impl Solution<'_, { Day::Day3 }, { Part::One }> for AOC2023<{ Day::Day3 }> {
    type Input = Vec<Vec<char>>;
    type Output = u32;

    fn solve(&self, input: &Self::Input) -> Result<Self::Output> {
        let symbols: HashMap<Point2D<i32>, char>;
        for (y, row) in input.iter().enumerate() {
            for (x, c) in row.iter().enumerate() {
                let point = Point { x, y };
                match *c as char {
                    '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | '.' => (),
                    _ => {
                        symbols.insert(point, *c as char);
                    }
                }
            }
        }

        for (y, row) in input.iter().enumerate() {
            let mut x = 0;
            while x < row.len() {
                
            }
        }
    }

impl Solution<'_, { Day::Day3 }, { Part::Two }> for AOC2023<{ Day::Day3 }> {
    type Input = Schematic;
    type Output = u32;

    fn solve(&self, input: &Self::Input) -> Result<Self::Output> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_color() -> Result<()> {
        assert_eq!("32...".to_string().parse(), Ok(32u32));
        Ok(())
    }
}
