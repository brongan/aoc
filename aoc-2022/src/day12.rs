use std::cmp::Ordering;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

use super::AOC2022;
use aoc_runner::point2d::Point2D;
use aoc_runner::{Day, ParseInput, Part, Solution};

use anyhow::Context;
use anyhow::Result;

type Square = Point2D<usize>;

struct HeightMap {
    grid: Vec<Vec<u32>>,
    start: Square,
    end: Square,
    width: usize,
    height: usize,
}

#[derive(Eq, PartialEq)]
struct CloudNode {
    pos: Square,
    dist: u32,
}

impl Ord for CloudNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.dist.cmp(&self.dist)
    }
}

impl PartialOrd for CloudNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl HeightMap {
    fn all_nodes(&self) -> Vec<CloudNode> {
        self.grid
            .iter()
            .enumerate(|(x, row)| {
                row.iter().enumerate(|(y, h)| CloudNode {
                    pos: Square { x, y },
                    dist: u32::max_value(),
                })
            })
            .collect()
    }

    fn height(&self, node: Square) -> u32 {
        self.grid[node.x][node.y]
    }

    fn get_neighbors(&self, node: Square) -> Vec<Square> {
        let mut result = Vec::new();
        if node.x != 0 && node.y != 0 {
            result.push(Square {
                x: node.x - 1,
                y: node.y - 1,
            });
        }
        if node.x != 0 && node.y != self.width {
            result.push(Square {
                x: node.x - 1,
                y: node.y + 1,
            });
        }
        if node.x != self.height && node.y != 0 {
            result.push(Square {
                x: node.x + 1,
                y: node.y - 1,
            });
        }
        if node.x != self.height && node.y != self.width {
            result.push(Square {
                x: node.x + 1,
                y: node.y + 1,
            });
        }
        result
    }
}

impl ParseInput<'_, { Day::Day12 }> for AOC2022<{ Day::Day12 }> {
    type Parsed = HeightMap;

    fn parse_input(&self, input: &'_ str) -> Result<Self::Parsed> {
        let mut start = Square { x: 0, y: 0 };
        let mut end = start;
        let mut grid = Vec::new();

        for (x, line) in input.lines().enumerate() {
            let mut row = Vec::new();
            for (y, c) in line.chars().enumerate() {
                match c {
                    'S' => {
                        start = Square { x, y };
                        row.push(0);
                    }
                    'E' => {
                        end = Square { x, y };
                        row.push(25);
                    }
                    _ => {
                        row.push(c as u32 - 'a' as u32);
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
    type Output = usize;

    fn solve(&self, input: &Self::Input) -> Result<Self::Output> {
        let mut distances = HashMap::from([(input.start, 0)]);
        let mut cloud: Vec<CloudNode> = input.all_nodes();
        distances[&input.start] = 0;

        while !cloud.is_empty() {
            let curr: CloudNode = std::cmp::min(cloud);
            distances[curr.pos] = curr.dist;
            for neighbor in input.get_neighbors(curr) {
                if (input.height(curr) as i64 - input.height(neighbor) as i64).abs() <= 1 {
                    t
                }
            }
        }
        Ok(distances[input.end])
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
