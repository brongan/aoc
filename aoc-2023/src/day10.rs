use super::AOC2023;
use anyhow::{anyhow, Context, Result};
use aoc_runner::{point2d::Point2D, Day, ParseInput, Part, Solution};
use colored::Colorize;
use std::collections::HashSet;
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};

#[derive(Clone, Copy, PartialEq, Eq, Debug, Display)]
pub enum Tile {
    Pipe,
    Hyphen,
    L,
    J,
    Seven,
    F,
    Dot,
    Start,
}

fn to_tile(c: char) -> Result<Tile> {
    match c {
        '|' => Ok(Tile::Pipe),
        '-' => Ok(Tile::Hyphen),
        'L' => Ok(Tile::L),
        'J' => Ok(Tile::J),
        '7' => Ok(Tile::Seven),
        'F' => Ok(Tile::F),
        '.' => Ok(Tile::Dot),
        'S' => Ok(Tile::Start),
        _ => Err(anyhow!("Invalid tile: {c}")),
    }
}

type Map = Vec<Vec<Tile>>;
type Coord = Point2D<i32>;

fn parse_map(input: &str) -> Result<Map> {
    input
        .lines()
        .map(|line| line.chars().map(|c| to_tile(c)).collect())
        .rev()
        .collect()
}

impl ParseInput<'_, { Day::Day10 }> for AOC2023<{ Day::Day10 }> {
    type Parsed = String;

    fn parse_input(&self, input: &'_ str) -> Result<Self::Parsed> {
        Ok(input.to_owned())
    }
}

fn find_start(map: &Map) -> Result<Coord> {
    map.iter()
        .enumerate()
        .map(|(y, row)| row.iter().enumerate().map(move |(x, tile)| (x, y, *tile)))
        .flatten()
        .find(|(_, _, tile)| *tile == Tile::Start)
        .map(|(x, y, _)| Coord {
            x: x as i32,
            y: y as i32,
        })
        .context("bs")
}

#[derive(Debug, EnumIter, PartialEq, Clone, Copy, Display)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

fn walk_path(start: Coord, mut direction: Direction, map: &Map) -> Option<Vec<Coord>> {
    let mut path = vec![start];
    let mut curr = start;

    loop {
        // Step to next point
        match direction {
            Direction::Up => curr.y += 1,
            Direction::Down => curr.y -= 1,
            Direction::Right => curr.x += 1,
            Direction::Left => curr.x -= 1,
        }
        path.push(curr);

        // Test if the new point is in bounds.
        if curr.y >= map.len() as i32 || curr.y < 0 || curr.x < 0 || curr.x >= map[0].len() as i32 {
            return None;
        }
        let tile = map[curr.y as usize][curr.x as usize];

        // Test if this is a valid transition given the tiles and update direction.
        direction = match (direction, tile) {
            (Direction::Up, Tile::F) => Direction::Right,
            (Direction::Up, Tile::Seven) => Direction::Left,
            (Direction::Up, Tile::Pipe) => Direction::Up,
            (Direction::Right, Tile::Seven) => Direction::Down,
            (Direction::Right, Tile::J) => Direction::Up,
            (Direction::Right, Tile::Hyphen) => Direction::Right,
            (Direction::Down, Tile::L) => Direction::Right,
            (Direction::Down, Tile::J) => Direction::Left,
            (Direction::Down, Tile::Pipe) => Direction::Down,
            (Direction::Left, Tile::L) => Direction::Up,
            (Direction::Left, Tile::F) => Direction::Down,
            (Direction::Left, Tile::Hyphen) => Direction::Left,
            (_, Tile::Start) => direction,
            (_, _) => {
                return None;
            }
        };

        if curr == start {
            return Some(path);
        }
    }
}

fn find_path(start: Coord, map: &Map) -> Result<Vec<Coord>> {
    for direction in Direction::iter() {
        if let Some(path) = walk_path(start, direction, &map) {
            return Ok(path);
        }
    }
    Err(anyhow!("Failed to find loop."))
}

impl Solution<'_, { Day::Day10 }, { Part::One }> for AOC2023<{ Day::Day10 }> {
    type Input = String;
    type Output = usize;

    fn solve(&self, input: &Self::Input) -> Result<Self::Output> {
        let map = parse_map(input)?;
        Ok(find_path(find_start(&map)?, &map)?.len() / 2)
    }
}

impl Solution<'_, { Day::Day10 }, { Part::Two }> for AOC2023<{ Day::Day10 }> {
    type Input = String;
    type Output = i32;

    fn solve(&self, input: &Self::Input) -> Result<Self::Output> {
        let map = parse_map(input)?;
        let loop_points: HashSet<Coord> = find_path(find_start(&map)?, &map)?.into_iter().collect();
        let mut enclosed_points: HashSet<Coord> = HashSet::new();

        let mut result = 0;
        for x in 0..map[0].len() {
            let mut num_intersections = 0;
            for y in 0..map.len() {
                let tile = map[y][x];
                let x = x as i32;
                let y = y as i32;
                let point = Coord { x, y };
                if loop_points.contains(&point) {
                    if tile != Tile::F && tile != Tile::L && tile != Tile::Pipe {
                        num_intersections += 1;
                    }
                } else if num_intersections % 2 == 1 {
                    enclosed_points.insert(point);
                    result += 1;
                }
            }
        }

        for x in 0..map[0].len() {
            for y in (0..map.len()).rev() {
                let tile = match map[y][x] {
                    Tile::Pipe => "|",
                    Tile::Hyphen => "-",
                    Tile::L => "L",
                    Tile::J => "J",
                    Tile::Seven => "7",
                    Tile::F => "F",
                    Tile::Dot => ".",
                    Tile::Start => "S",
                };
                let x = x as i32;
                let y = y as i32;
                let point = Coord { x, y };
                if enclosed_points.contains(&point) {
                    print!("{}", tile.bright_red());
                } else if loop_points.contains(&point) {
                    print!("{}", tile.bright_green());
                } else {
                    print!("{tile}");
                }
            }
            print!("\n");
        }

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use aoc_runner::{PartOneVerifier, PartTwoVerifier};

    use super::*;

    #[test]
    fn test() -> Result<()> {
        let input = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";

        let problem = super::AOC2023::<{ Day::Day10 }>;
        problem.test_part1(input, 8)?;
        let input = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";
        problem.test_part2(input, 4)?;
        let input = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";
        problem.test_part2(input, 10)
    }
}
