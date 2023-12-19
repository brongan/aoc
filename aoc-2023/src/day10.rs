use super::AOC2023;
use anyhow::{anyhow, Context, Result};
use aoc_runner::{point2d::Point2D, Day, ParseInput, Part, Solution};
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

impl ParseInput<'_, { Day::Day10 }> for AOC2023<{ Day::Day10 }> {
    type Parsed = Map;

    fn parse_input(&self, input: &'_ str) -> Result<Self::Parsed> {
        input
            .lines()
            .map(|line| line.chars().map(|c| to_tile(c)).collect())
            .rev()
            .collect()
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

fn walk_path(start: Coord, mut direction: Direction, map: &Map) -> Option<i32> {
    let mut steps = 0;
    let mut curr = start;

    loop {
        // Step to next point
        match direction {
            Direction::Up => curr.y += 1,
            Direction::Down => curr.y -= 1,
            Direction::Right => curr.x += 1,
            Direction::Left => curr.x -= 1,
        }
        steps += 1;

        // Test if the new point is in bounds.
        if curr.y > map.len() as i32 || curr.y < 0 || curr.x < 0 || curr.x > map[0].len() as i32 {
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
            (_, Tile::Start) => Direction::Up,
            (_, _) => {
                return None;
            }
        };

        if curr == start {
            return Some(steps);
        }
    }
}

fn walk_paths(start: Coord, map: &Map) -> Result<i32> {
    for direction in Direction::iter() {
        if let Some(dist) = walk_path(start, direction, &map) {
            return Ok(dist / 2);
        }
    }
    return Err(anyhow!("Failed to find loop."));
}

impl Solution<'_, { Day::Day10 }, { Part::One }> for AOC2023<{ Day::Day10 }> {
    type Input = Map;
    type Output = i32;

    fn solve(&self, input: &Self::Input) -> Result<Self::Output> {
        walk_paths(find_start(&input)?, &input)
    }
}

#[cfg(test)]
mod tests {
    use aoc_runner::PartOneVerifier;

    use super::*;

    #[test]
    fn test() -> Result<()> {
        let input = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";

        let problem = super::AOC2023::<{ Day::Day10 }>;
        problem.test_part1(input, 8)
    }
}
