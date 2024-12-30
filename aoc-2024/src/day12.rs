use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use super::AOC2024;
use anyhow::Result;
use aoc_runner::{point2d::Point2D, Day, ParseInput, Part, Solution};

type Num = usize;
type Point = Point2D<usize>;

pub struct Region {
    perimeter: Num,
    plots: HashSet<Point>,
    plant: char,
}

impl Region {
    fn cost(&self) -> Num {
        return self.plots.len() * self.perimeter;
    }
}

pub struct Garden {
    width: usize,
    height: usize,
    plots: HashMap<Point, char>,
}

impl Garden {
    fn regions(&self) -> Vec<Region> {
        let get_neighbors = |point: Point| {
            let ret = Vec::new();
            if point.x != 0 {
                ret.push(Point {
                    x: point.x - 1,
                    ..point
                });
            }
            if point.x != self.width - 1 {
                ret.push(Point {
                    x: point.x + 1,
                    ..point
                });
            }
            if point.y != 0 {
                ret.push(Point {
                    y: point.y - 1,
                    ..point
                });
            }
            if point.y != self.width - 1 {
                ret.push(Point {
                    y: point.y + 1,
                    ..point
                });
            }
            ret
        };
        let mut visited = HashSet::new();
        let mut regions = Vec::new();
        let dfs = |point: Point, target: char| -> Region {
            let mut to_visit = Vec::from([point]);
            let mut plots = HashSet::new();
            while let Some(point) = to_visit.pop() {
                if visited.contains(&point) {
                    continue;
                }
                if self.plots[&point] != target {
                    continue;
                }
                visited.insert(point);
                plots.insert(point);
                for neighbor in get_neighbors(point) {
                    to_visit.push(neighbor);
                }
            }
            Region {
                perimeter,
                plots,
                plant: target,
            }
        };
        for (point, plant) in self.plots {
            if !visited.contains(&point) {
                regions.push(dfs(point, self.plots[&point]));
            }
        }
        regions
    }

    fn price(&self) -> Num {
        self.regions().iter().map(|region| region.cost()).sum()
    }
}

impl FromStr for Garden {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let plots: Vec<Vec<char>> = s.lines().map(|line| line.chars().collect()).collect();
        let height = plots.len();
        let width = plots[0].len();
        let plots = plots
            .into_iter()
            .enumerate()
            .map(|(y, row)| {
                row.into_iter()
                    .enumerate()
                    .map(|(x, c)| (Point { x, y }, c))
                    .collect::<HashMap<_, _>>()
            })
            .flatten()
            .collect();
        Ok(Garden {
            width,
            height,
            plots,
        })
    }
}

impl ParseInput<'_, { Day::Day12 }> for AOC2024<{ Day::Day12 }> {
    type Parsed = Garden;

    fn parse_input(&self, input: &'_ str) -> Result<Self::Parsed> {
        Garden::from_str(input)
    }
}

impl Solution<'_, { Day::Day12 }, { Part::One }> for AOC2024<{ Day::Day12 }> {
    type Input = Garden;
    type Output = Num;

    fn solve(&self, input: &Self::Input) -> Result<Self::Output> {
        Ok(input.price())
    }
}

impl Solution<'_, { Day::Day12 }, { Part::Two }> for AOC2024<{ Day::Day12 }> {
    type Input = Garden;
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
