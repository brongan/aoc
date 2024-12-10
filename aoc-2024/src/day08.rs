use std::collections::{HashMap, HashSet};

use super::AOC2024;
use anyhow::Result;
use aoc_runner::{point2d::Point2D, Day, ParseInput, Part, Solution};
use itertools::Itertools;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Tile {
    Empty,
    Antenna(char),
}

fn to_tile(c: char) -> Tile {
    match c {
        '.' => Tile::Empty,
        c => Tile::Antenna(c),
    }
}

fn parse_map(input: &str) -> Map {
    input
        .lines()
        .map(|line| line.chars().map(to_tile).collect())
        .rev()
        .collect()
}

type Map = Vec<Vec<Tile>>;
type Coord = Point2D<i32>;
type Num = usize;

impl ParseInput<'_, { Day::Day8 }> for AOC2024<{ Day::Day8 }> {
    type Parsed = Map;

    fn parse_input(&self, input: &'_ str) -> Result<Self::Parsed> {
        Ok(parse_map(input))
    }
}

fn inbounds(map: &Map, coord: Coord) -> bool {
    coord.x >= 0
        && (coord.x as usize) < map.len()
        && coord.y >= 0
        && (coord.y as usize) < map[0].len()
}

fn antennae(map: &Map) -> HashMap<char, Vec<Coord>> {
    let mut antennae: HashMap<char, Vec<Coord>> = HashMap::new();
    for x in 0..map.len() {
        for y in 0..map[0].len() {
            if let Tile::Antenna(c) = map[y][x] {
                if antennae.get(&c) == None {
                    antennae.insert(c, Vec::new());
                }
                antennae.get_mut(&c).unwrap().push(Point2D {
                    x: x as i32,
                    y: y as i32,
                });
            }
        }
    }
    antennae
}
fn compute_antinodes(map: &Map) -> HashSet<Coord> {
    let antennae = antennae(map);
    let mut ret = HashSet::new();
    for (_frequency, nodes) in antennae {
        for combo in nodes.iter().combinations(2) {
            let node1 = combo[0];
            let node2 = combo[1];
            let diff = *node2 - *node1;
            let candidate1 = *node2 + diff;
            let candidate2 = *node1 - diff;
            if inbounds(map, candidate1) {
                ret.insert(candidate1);
            }
            if inbounds(map, candidate2) {
                ret.insert(candidate2);
            }
        }
    }
    ret
}

fn compute_harmonic_antinodes(map: &Map) -> HashSet<Coord> {
    let antennae = antennae(map);
    let mut ret = HashSet::new();
    for (_frequency, nodes) in antennae {
        for combo in nodes.iter().combinations(2) {
            let node1 = combo[0];
            let node2 = combo[1];
            ret.insert(*node1);
            ret.insert(*node2);
            let diff = *node2 - *node1;
            let mut candidate1 = *node2 + diff;
            let mut candidate2 = *node1 - diff;
            while inbounds(map, candidate1) {
                ret.insert(candidate1);
                candidate1 = candidate1 + diff;
            }
            while inbounds(map, candidate2) {
                ret.insert(candidate2);
                candidate2 = candidate2 - diff;
            }
        }
    }
    ret
}

impl Solution<'_, { Day::Day8 }, { Part::One }> for AOC2024<{ Day::Day8 }> {
    type Input = Map;
    type Output = Num;

    fn solve(&self, input: &Self::Input) -> Result<Self::Output> {
        Ok(compute_antinodes(input).len())
    }
}

impl Solution<'_, { Day::Day8 }, { Part::Two }> for AOC2024<{ Day::Day8 }> {
    type Input = Map;
    type Output = Num;

    fn solve(&self, input: &Self::Input) -> Result<Self::Output> {
        Ok(compute_harmonic_antinodes(input).len())
    }
}
