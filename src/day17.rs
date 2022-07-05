use super::AOC2021;
use crate::aoc::{Day, ParseInput, Part, Solution};
use crate::point2d::Point2D;
use std::cmp::{max, min, Ordering};
use std::str::FromStr;

#[derive(Copy, Clone)]
struct Velocity {
    dx: i32,
    dy: i32,
}

#[derive(Clone, Copy)]
pub struct TargetArea {
    bottom_left: Point2D<i32>,
    top_right: Point2D<i32>,
}

impl FromStr for TargetArea {
    type Err = std::string::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut ranges = s
            .trim_start_matches("target area: x=")
            .split(", y=")
            .map(|range| {
                range
                    .split_once("..")
                    .map(|(l, r)| (l.parse().unwrap(), r.parse().unwrap()))
            });

        let (x1, x2) = ranges.next().unwrap().unwrap();
        let (y1, y2) = ranges.next().unwrap().unwrap();

        Ok(TargetArea {
            bottom_left: Point2D {
                x: min(x1, x2),
                y: min(y1, y2),
            },
            top_right: Point2D {
                x: max(x1, x2),
                y: max(y1, y2),
            },
        })
    }
}

impl ParseInput<'_, { Day::Day17 }> for AOC2021<{ Day::Day17 }> {
    type Parsed = TargetArea;

    fn parse_input(&self, input: &'_ str) -> Self::Parsed {
        TargetArea::from_str(input).expect("failed to parse target area")
    }
}

enum ProbeStatus {
    Before,
    Hit,
    Miss,
}

fn hit_or_miss(p: &Point2D<i32>, v: &Velocity, target: &TargetArea) -> ProbeStatus {
    if target.bottom_left.x <= p.x
        && p.x <= target.top_right.x
        && target.bottom_left.y <= p.y
        && p.y <= target.top_right.y
    {
        return ProbeStatus::Hit;
    }

    if p.x > target.top_right.x {
        return ProbeStatus::Miss;
    }

    if p.y < target.bottom_left.y && v.dy <= 0 {
        return ProbeStatus::Miss;
    }

    ProbeStatus::Before
}

fn trajectory_height(mut p: Point2D<i32>, mut v: Velocity, t: &TargetArea) -> Option<i32> {
    let mut max_height = p.y;
    loop {
        p.x += v.dx;
        p.y += v.dy;
        max_height = max(max_height, p.y);
        v.dx -= match v.dx.cmp(&0) {
            Ordering::Greater => 1,
            Ordering::Equal => 0,
            Ordering::Less => -1,
        };
        v.dy -= 1;
        match hit_or_miss(&p, &v, t) {
            ProbeStatus::Before => (),
            ProbeStatus::Hit => return Some(max_height),
            ProbeStatus::Miss => return None,
        }
    }
}

impl Solution<'_, { Day::Day17 }, { Part::One }> for AOC2021<{ Day::Day17 }> {
    type Input = TargetArea;
    type Output = i32;

    fn solve(&self, input: &Self::Input) -> Self::Output {
        let p = Point2D { x: 0, y: 0 };
        (0..100)
            .map(|dx: i32| {
                (0..1000)
                    .filter_map(move |dy: i32| trajectory_height(p, Velocity { dx, dy }, &input))
            })
            .flatten()
            .max()
            .unwrap()
    }
}

impl Solution<'_, { Day::Day17 }, { Part::Two }> for AOC2021<{ Day::Day17 }> {
    type Input = TargetArea;
    type Output = usize;

    fn solve(&self, input: &Self::Input) -> Self::Output {
        let p = Point2D { x: 0, y: 0 };
        (0..input.top_right.x + 1)
            .map(|dx: i32| {
                (input.bottom_left.y..400)
                    .filter_map(move |dy: i32| trajectory_height(p, Velocity { dx, dy }, &input))
            })
            .flatten()
            .count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aoc::PartOneVerifier;
    use crate::aoc::PartTwoVerifier;

    #[test]
    fn test() -> Result<(), String> {
        let input = "target area: x=20..30, y=-10..-5";
        let problem = super::AOC2021::<{ Day::Day17 }>;
        (&&&problem).test_part1(input, 45)?;
        (&&&problem).test_part2(input, 112)
    }
}
