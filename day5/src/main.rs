use std::cmp::{max, min};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Range;
use std::str::FromStr;
use thiserror::Error;

#[derive(PartialEq, Clone, Debug)]
enum Axis {
    Horizontal,
    Vertical,
}

#[derive(Clone, Debug)]
struct Fissure {
    range: Range<usize>,
    axis: Axis,
    static_coord: usize,
}

impl Fissure {
    fn validate(x1: usize, x2: usize, y1: usize, y2: usize) -> Option<Self> {
        match ((x1 == x2), (y1 == y2)) {
            (true, _) => Some(Fissure {
                range: min(y1, y2)..max(y1, y2),
                axis: Axis::Vertical,
                static_coord: x1,
            }),
            (false, true) => Some(Fissure {
                range: min(x1, x2)..max(x1, x2),
                axis: Axis::Horizontal,
                static_coord: y1,
            }),
            (_, _) => None,
        }
    }
}

#[derive(Error, Debug)]
pub enum FissureError {
    #[error("Failed to Parse")]
    ParseInt(#[from] std::num::ParseIntError),
    #[error("Failed to split line into coordinates")]
    ParseCoords,
    #[error("Diagonal")]
    Diagonal,
}

impl FromStr for Fissure {
    type Err = FissureError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (p1, p2) = s.split_once(" -> ").ok_or(FissureError::ParseCoords)?;
        let p1 = p1.split_once(',').ok_or(FissureError::ParseCoords)?;
        let p2 = p2.split_once(',').ok_or(FissureError::ParseCoords)?;
        let x1 = p1.0.parse()?;
        let x2 = p2.0.parse()?;
        let y1 = p1.1.parse()?;
        let y2 = p2.1.parse()?;
        Fissure::validate(x1, x2, y1, y2).ok_or(FissureError::Diagonal)
    }
}

fn part1(fissures: &[Fissure]) -> u32 {
    let mut vent_counts = [[0u32; 1000]; 1000];
    for fissure in fissures.to_vec().iter() {
        if fissure.axis == Axis::Vertical {
            for y in fissure.range.clone() {
                vent_counts[fissure.static_coord][y] += 1
            }
        } else {
            for x in fissure.range.clone() {
                vent_counts[x][fissure.static_coord] += 1
            }
        }
    }
    vent_counts
        .iter()
        .flat_map(|x| x.iter())
        .filter(|x| **x >= 2)
        .sum()
}

fn main() {
    let f = File::open("input").expect("Failed to open input");
    let f = BufReader::new(f);
    let fissures: Vec<Fissure> = f
        .lines()
        .flatten()
        .map(|line| Fissure::from_str(line.as_str()))
        .filter_map(|f| f.ok())
        .collect();
    println!("Part 1: {}", part1(&fissures));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pt_2() {
        let input: Vec<Fissure> = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2"
            .lines()
            .map(Fissure::from_str)
            .filter_map(|f| f.ok())
            .collect();
        assert_eq!(part1(&input), 5);
    }
}
