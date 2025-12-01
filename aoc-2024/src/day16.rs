use std::collections::{HashMap, HashSet, VecDeque};

use super::AOC2024;
use anyhow::Result;
use aoc_runner::{point2d::Point2D, Day, ParseInput, Part, Solution};

type Point = Point2D<usize>;
type IR = Vec<Vec<char>>;
type Num = u64;

enum Direction {
    North,
    East,
    South,
    West,
}

impl ParseInput<'_, { Day::Day16 }> for AOC2024<{ Day::Day16 }> {
    type Parsed = IR;

    fn parse_input(&self, input: &'_ str) -> Result<Self::Parsed> {
        Ok(input
            .lines()
            .rev()
            .map(|line| line.chars().collect())
            .collect())
    }
}

fn find_start(map: &Vec<Vec<char>>) -> Point {
    map.iter()
        .enumerate()
        .flat_map(|(y, row)| row.iter().enumerate().map(move |(x, tile)| (x, y, *tile)))
        .find(|(_, _, tile)| *tile == 'S')
        .map(|(x, y, _)| Point { x, y })
        .unwrap()
}

impl Solution<'_, { Day::Day16 }, { Part::One }> for AOC2024<{ Day::Day16 }> {
    type Input = IR;
    type Output = Num;

    fn solve(&self, input: &Self::Input) -> Result<Self::Output> {
        let start = find_start(&input);
        // BFS
        // cost(point, direction) = min(old_cost,
        //  choice:
        //  turn left,
        //  turn right,
        //  walk straight,
        //  turn twice
        let visited: HashMap<(Point, Direction), Num> = HashMap::new();
        let to_visit = VecDeque::new();
    }
}

impl Solution<'_, { Day::Day16 }, { Part::Two }> for AOC2024<{ Day::Day16 }> {
    type Input = IR;
    type Output = Num;

    fn solve(&self, input: &Self::Input) -> Result<Self::Output> {
        todo!()
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
