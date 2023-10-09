use std::collections::{HashMap, HashSet};

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
        if node.x <= self.height - 1 {
            result.push(Square {
                x: node.x + 1,
                y: node.y,
            });
        }
        if node.y <= self.width - 1 {
            result.push(Square {
                x: node.x,
                y: node.y + 1,
            });
        }
        result
    }
}

impl ParseInput<'_, { Day::Day12 }> for AOC2022<{ Day::Day12 }> {
    type Parsed = HeightMap;

    fn parse_input(&self, input: &'_ str) -> Result<Self::Parsed> {
        eprintln!("{input}");
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

        let width = grid.len();
        let height = grid[0].len();

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
        let mut distances = HashMap::new();
        let mut unvisited = HashSet::new();
        for x in 0..input.width {
            for y in 0..input.height {
                distances.insert(Square { x, y }, i32::max_value());
                unvisited.insert(Square { x, y });
            }
        }

        distances.insert(input.start, 0);
        let mut curr;
        while unvisited.contains(&input.end) {
            curr = *unvisited
                .iter()
                .min_by_key(|point| distances[point])
                .unwrap();
            for neighbor in &input.get_neighbors(&curr) {
                eprintln!("Curr: {curr}, Neighbor: {neighbor}");
                let diff = input.height(&curr) - input.height(neighbor);
                if diff.abs() <= 1 {
                    distances.insert(neighbor.clone(), distances[&curr] + 1);
                }
            }
            unvisited.remove(&curr);
        }
        Ok(distances[&input.end])
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
