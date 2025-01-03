use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use super::AOC2024;
use anyhow::Result;
use aoc_runner::{point2d::Point2D, Day, ParseInput, Part, Solution};

type Num = usize;
type Point = Point2D<i32>;

pub struct Region {
    perimeter: Num,
    sides: Num,
    plots: HashSet<Point>,
    _plant: char,
}

impl Region {
    fn cost(&self) -> Num {
        return self.plots.len() * self.perimeter;
    }

    fn cost2(&self) -> Num {
        return self.plots.len() * self.sides;
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
            vec![
                Point {
                    x: point.x - 1,
                    ..point
                },
                Point {
                    x: point.x + 1,
                    ..point
                },
                Point {
                    y: point.y - 1,
                    ..point
                },
                Point {
                    y: point.y + 1,
                    ..point
                },
            ]
        };
        let inbounds = |Point2D { x, y }: Point| {
            x >= 0 && x < self.width as i32 && y >= 0 && y < self.height as i32
        };
        let mut visited = HashSet::new();
        let mut regions = Vec::new();
        let dfs = |point: Point, target: char, visited: &mut HashSet<Point>| -> Region {
            let mut perimeter = 0;
            let mut to_visit = Vec::from([point]);
            let mut plots = HashSet::new();
            while let Some(point) = to_visit.pop() {
                if visited.contains(&point) {
                    continue;
                }
                visited.insert(point);
                plots.insert(point);
                for neighbor in get_neighbors(point) {
                    if inbounds(neighbor) {
                        if self.plots[&neighbor] != target {
                            perimeter += 1;
                        } else {
                            to_visit.push(neighbor);
                        }
                    } else {
                        perimeter += 1;
                    }
                }
            }
            Region {
                perimeter,
                sides,
                plots,
                _plant: target,
            }
        };
        for point in self.plots.keys() {
            if !visited.contains(point) {
                regions.push(dfs(*point, self.plots[&point], &mut visited));
            }
        }
        regions
    }

    fn price(&self) -> Num {
        self.regions().iter().map(|region| region.cost()).sum()
    }

    fn price2(&self) -> Num {
        self.regions().iter().map(|region| region.cost2()).sum()
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
                    .map(|(x, c)| {
                        (
                            Point {
                                x: x as i32,
                                y: y as i32,
                            },
                            c,
                        )
                    })
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
