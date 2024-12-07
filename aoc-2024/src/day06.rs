use std::collections::HashSet;

use super::AOC2024;
use anyhow::Result;
use aoc_runner::{point2d::Point2D, Day, ParseInput, Part, Solution};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Tile {
    Empty,
    Obstacle,
    Guard,
}

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn turn(self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

fn to_tile(c: char) -> Tile {
    match c {
        '.' => Tile::Empty,
        '#' => Tile::Obstacle,
        '^' => Tile::Guard,
        _ => panic!("Invalid tile: {c}"),
    }
}

type Map = Vec<Vec<Tile>>;
type Coord = Point2D<i32>;
type Num = usize;

fn parse_map(input: &str) -> Map {
    input
        .lines()
        .map(|line| line.chars().map(to_tile).collect())
        .rev()
        .collect()
}

impl ParseInput<'_, { Day::Day6 }> for AOC2024<{ Day::Day6 }> {
    type Parsed = Map;

    fn parse_input(&self, input: &'_ str) -> Result<Self::Parsed> {
        Ok(parse_map(input))
    }
}

fn find_start(map: &Map) -> Coord {
    map.iter()
        .enumerate()
        .flat_map(|(y, row)| row.iter().enumerate().map(move |(x, tile)| (x, y, *tile)))
        .find(|(_, _, tile)| *tile == Tile::Guard)
        .map(|(x, y, _)| Coord {
            x: x as i32,
            y: y as i32,
        })
        .unwrap()
}

fn step(mut point: Coord, direction: Direction) -> Coord {
    match direction {
        Direction::Up => point.y += 1,
        Direction::Right => point.x += 1,
        Direction::Down => point.y -= 1,
        Direction::Left => point.x -= 1,
    }
    point
}

fn inbounds(map: &Map, coord: Coord) -> bool {
    coord.x >= 0
        && (coord.x as usize) < map.len()
        && coord.y >= 0
        && (coord.y as usize) < map[0].len()
}

fn guard_visits(map: &Map, mut curr: Coord, mut direction: Direction) -> HashSet<Coord> {
    let mut visited = HashSet::new();

    while inbounds(map, curr) {
        visited.insert(curr);
        let next = step(curr, direction);
        if inbounds(map, next) && map[next.y as usize][next.x as usize] == Tile::Obstacle {
            direction = direction.turn();
        } else {
            curr = next;
        }
    }

    visited
}

impl Solution<'_, { Day::Day6 }, { Part::One }> for AOC2024<{ Day::Day6 }> {
    type Input = Map;
    type Output = Num;

    fn solve(&self, map: &Self::Input) -> Result<Self::Output> {
        Ok(guard_visits(map, find_start(map), Direction::Up).len())
    }
}

impl Solution<'_, { Day::Day6 }, { Part::Two }> for AOC2024<{ Day::Day6 }> {
    type Input = Map;
    type Output = Num;

    fn solve(&self, map: &Self::Input) -> Result<Self::Output> {
        let visited = guard_visits(map, find_start(map), Direction::Up);
        // Return every naturally visited space where that participates in a rectangle with 3 other
        // obstacles. Turning logic is sus.
        todo!()
    }
}
