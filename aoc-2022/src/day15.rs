use std::collections::HashSet;

use super::AOC2022;
use anyhow::{Context, Result};
use aoc_runner::point2d::{manhattan_distance, Point2D};
use aoc_runner::{Day, ParseInput, Part, Solution};
use nom::{
    bytes::complete::tag,
    character::complete::newline,
    combinator::{all_consuming, map, opt},
    multi::separated_list1,
    sequence::{preceded, separated_pair, terminated},
    IResult,
};

#[derive(PartialEq, Debug)]
pub struct LogLine {
    sensor: Point2D<i64>,
    closest_beacon: Point2D<i64>,
}

fn parse_point(input: &str) -> IResult<&str, Point2D<i64>> {
    let parse_x = |input| preceded(tag("x="), nom::character::complete::i64)(input);
    let parse_y = |input| preceded(tag("y="), nom::character::complete::i64)(input);
    Ok(map(
        separated_pair(parse_x, tag(", "), parse_y),
        |(x, y)| Point2D { x, y },
    )(input)?)
}

impl LogLine {
    fn parse(input: &str) -> IResult<&str, LogLine> {
        let parse_sensor = |input| preceded(tag("Sensor at "), parse_point)(input);
        let parse_beacon = |input| preceded(tag("closest beacon is at "), parse_point)(input);
        map(
            separated_pair(parse_sensor, tag(": "), parse_beacon),
            |(sensor, beacon)| LogLine {
                sensor,
                closest_beacon: beacon,
            },
        )(input)
    }
}

impl ParseInput<'_, { Day::Day15 }> for AOC2022<{ Day::Day15 }> {
    type Parsed = Vec<LogLine>;

    fn parse_input(&self, input: &'_ str) -> Result<Self::Parsed> {
        let parse_lines = |input| separated_list1(newline, LogLine::parse)(input);
        let (_, lines) = all_consuming(terminated(parse_lines, opt(newline)))(input)
            .map_err(|e| e.to_owned())?;
        Ok(lines)
    }
}

fn pos_can_contain_beacon(
    pos: &Point2D<i64>,
    sensor: &Point2D<i64>,
    beacon: &Point2D<i64>,
) -> bool {
    manhattan_distance(sensor, pos) > manhattan_distance(sensor, beacon)
}

fn cannot_contain_beacon_count(input: &[LogLine], y: i64) -> Result<usize> {
    let x_values: Vec<i64> = input
        .iter()
        .map(|line| [line.sensor.x, line.closest_beacon.x])
        .flatten()
        .collect();
    let min_x = x_values.iter().min().context("Failed to find x values.")?;
    let max_x = x_values.iter().max().context("Failed to find x values.")?;
    let beacon_locations: HashSet<Point2D<i64>> =
        input.iter().map(|line| line.closest_beacon).collect();

    let beacon_possible = |pos: Point2D<i64>| {
        if beacon_locations.contains(&pos) {
            return true;
        }
        for line in input {
            if !pos_can_contain_beacon(&pos, &line.sensor, &line.closest_beacon) {
                return false;
            }
        }
        true
    };

    Ok((*min_x - 1_000_000..max_x + 1_000_000)
        .map(|x| !beacon_possible(Point2D { x, y }))
        .filter(|x| *x)
        .count())
}

impl Solution<'_, { Day::Day15 }, { Part::One }> for AOC2022<{ Day::Day15 }> {
    type Input = Vec<LogLine>;
    type Output = usize;

    fn solve(&self, input: &Self::Input) -> Result<Self::Output> {
        cannot_contain_beacon_count(&input, 2_000_000)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line() -> Result<()> {
        assert_eq!(
            LogLine::parse(
                "Sensor at x=2832148, y=322979: closest beacon is at x=3015667, y=-141020"
            )?,
            (
                "",
                LogLine {
                    sensor: Point2D {
                        x: 2832148,
                        y: 322979
                    },
                    closest_beacon: Point2D {
                        x: 3015667,
                        y: -141020
                    }
                }
            )
        );
        Ok(())
    }

    #[test]
    fn test() -> Result<()> {
        let input = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";
        let problem = super::AOC2022::<{ Day::Day15 }>;
        let parsed = problem.parse_input(input)?;
        assert_eq!(cannot_contain_beacon_count(&parsed, 10)?, 26);
        Ok(())
    }
}
