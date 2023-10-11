use std::collections::HashMap;

use super::AOC2022;
use aoc_runner::point2d::Point2D;
use aoc_runner::{Day, ParseInput, Part, Solution};

use anyhow::Result;

type Square = Point2D<usize>;

pub struct HeightMap {
    grid: Vec<Vec<i32>>,
    start: Square,
    end: Square,
    width: usize,
    height: usize,
}

impl HeightMap {
    fn height(&self, node: &Square) -> i32 {
        if node.x >= self.width || node.y >= self.height {
            panic!("{} is not a valid point.", node)
        }
        self.grid[node.y][node.x]
    }

    fn get_neighbors(&self, node: &Square) -> Vec<Square> {
        let mut result = Vec::new();
        if node.x > 0 {
            result.push(Square {
                x: node.x - 1,
                y: node.y,
            });
        }
        if node.y > 0 {
            result.push(Square {
                x: node.x,
                y: node.y - 1,
            });
        }
        if node.x < self.width - 1 {
            result.push(Square {
                x: node.x + 1,
                y: node.y,
            });
        }
        if node.y < self.height - 1 {
            result.push(Square {
                x: node.x,
                y: node.y + 1,
            });
        }
        result
            .into_iter()
            .filter(|neighbor| self.height(neighbor) - self.height(&node) <= 1)
            .collect()
    }

    fn djikstra(&self, start: &Square) -> HashMap<Square, i32> {
        let mut distances = HashMap::new();
        let mut cloud = HashMap::from([(*start, 0)]);
        while !cloud.is_empty() {
            let next = cloud.iter().min_by_key(|entry| entry.1).unwrap();
            let (curr, dist) = (*next.0, *next.1);
            distances.insert(curr, dist);
            for neighbor in &self.get_neighbors(&curr) {
                if !distances.contains_key(neighbor) && !cloud.contains_key(neighbor) {
                    cloud.insert(neighbor.clone(), distances[&curr] + 1);
                }
            }
            cloud.remove(&curr);
        }
        distances
    }
}

impl ParseInput<'_, { Day::Day12 }> for AOC2022<{ Day::Day12 }> {
    type Parsed = HeightMap;

    fn parse_input(&self, input: &'_ str) -> Result<Self::Parsed> {
        let mut start = Square { x: 0, y: 0 };
        let mut end = start;
        let mut grid = Vec::new();

        for (y, line) in input.lines().enumerate() {
            let mut row = Vec::new();
            for (x, c) in line.chars().enumerate() {
                match c {
                    'S' => {
                        start = Square { x, y };
                        row.push(-1);
                    }
                    'E' => {
                        end = Square { x, y };
                        row.push(26);
                    }
                    _ => {
                        row.push(c as i32 - 'a' as i32);
                    }
                }
            }
            grid.push(row);
        }

        let height = grid.len();
        let width = grid[0].len();

        Ok(HeightMap {
            grid,
            start,
            end,
            width,
            height,
        })
    }
}

impl Solution<'_, { Day::Day12 }, { Part::One }> for AOC2022<{ Day::Day12 }> {
    type Input = HeightMap;
    type Output = i32;

    fn solve(&self, input: &Self::Input) -> Result<Self::Output> {
        let distances = input.djikstra(&input.start);
        Ok(distances[&input.end])
    }
}

impl Solution<'_, { Day::Day12 }, { Part::Two }> for AOC2022<{ Day::Day12 }> {
    type Input = HeightMap;
    type Output = i32;

    fn solve(&self, input: &Self::Input) -> Result<Self::Output> {
        let mut starts: Vec<Square> = Vec::new();
        for (y, row) in input.grid.iter().enumerate() {
            for (x, height) in row.iter().enumerate() {
                if *height == 0 {
                    starts.push(Square { x, y });
                }
            }
        }
        let result = starts
            .iter()
            .map(|start| {
                let distances = input.djikstra(&start);
                match distances.get(&input.end) {
                    Some(x) => Some(*x),
                    None => None,
                }
            })
            .flatten()
            .min()
            .unwrap();
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_runner::PartOneVerifier;

    #[test]
    fn test() -> Result<()> {
        let input = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";
        let problem = super::AOC2022::<{ Day::Day12 }>;
        problem.test_part1(input, 31)
    }
}
