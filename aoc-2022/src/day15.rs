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
use std::cmp::{max, min};

type Point = Point2D<i64>;

#[derive(PartialEq, Debug)]
pub struct LogLine {
    sensor: Point,
    beacon: Point,
}

fn parse_point(input: &str) -> IResult<&str, Point> {
    let parse_x = |input| preceded(tag("x="), nom::character::complete::i64)(input);
    let parse_y = |input| preceded(tag("y="), nom::character::complete::i64)(input);
    Ok(map(
        separated_pair(parse_x, tag(", "), parse_y),
        |(x, y)| Point { x, y },
    )(input)?)
}

impl LogLine {
    fn parse(input: &str) -> IResult<&str, LogLine> {
        let parse_sensor = |input| preceded(tag("Sensor at "), parse_point)(input);
        let parse_beacon = |input| preceded(tag("closest beacon is at "), parse_point)(input);
        map(
            separated_pair(parse_sensor, tag(": "), parse_beacon),
            |(sensor, beacon)| LogLine { sensor, beacon },
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

fn get_boundaries(input: &[LogLine]) -> (i64, i64, i64, i64) {
    let mut min_x = 0;
    let mut max_x = 0;
    let mut min_y = 0;
    let mut max_y = 0;

    for line in input {
        let dist = manhattan_distance(&line.sensor, &line.beacon);
        min_x = min(min_x, min(line.beacon.x - dist, line.sensor.x - dist));
        max_x = max(max_x, max(line.beacon.x + dist, line.sensor.x + dist));
        min_y = min(min_y, min(line.beacon.y - dist, line.sensor.y - dist));
        max_y = max(max_y, max(line.beacon.y + dist, line.sensor.y + dist));
    }
    (min_x, max_x, min_y, max_y)
}

#[derive(PartialEq)]
enum State {
    Beacon,
    Pound,
    Dot,
}

fn beacon_possibility(point: &Point, input: &[LogLine]) -> State {
    for line in input {
        if *point == line.beacon {
            return State::Beacon;
        }
    }
    for line in input {
        if manhattan_distance(&point, &line.sensor)
            <= manhattan_distance(&line.beacon, &line.sensor)
        {
            return State::Pound;
        }
    }
    State::Dot
}

fn cannot_contain_beacon_count(input: &[LogLine], y: i64) -> Result<usize> {
    let (min_x, max_x, _min_y, _max_y) = get_boundaries(&input);

    Ok((min_x..max_x + 1)
        .map(|x| beacon_possibility(&Point { x, y }, &input))
        .filter(|x| *x == State::Pound)
        .count())
}

impl Solution<'_, { Day::Day15 }, { Part::One }> for AOC2022<{ Day::Day15 }> {
    type Input = Vec<LogLine>;
    type Output = usize;

    fn solve(&self, input: &Self::Input) -> Result<Self::Output> {
        cannot_contain_beacon_count(&input, 2_000_000)
    }
}

fn tuning_frequency(point: &Point) -> i64 {
    point.x * 4000000 + point.y
}

fn can_contain_brute(input: &[LogLine], bottom_right: &Point) -> Option<Point> {
    for x in 0..bottom_right.x {
        for y in 0..bottom_right.y {
            let point = Point { x, y };
            if beacon_possibility(&point, input) == State::Dot {
                return Some(point);
            }
        }
    }
    None
}

struct Interval {
    start: i64,
    end: i64,
}

fn can_contain(input: &[LogLine], bottom_right: &Point) -> Option<Point> {
    // for each edge line, consider the parallel edge line with 1 away
    //
}

fn find_frequency(input: &[LogLine], bottom_right: &Point) -> Result<i64> {
    can_contain_brute(input, &bottom_right)
        .map(|p| tuning_frequency(&p))
        .context("Did not find point.")
}

impl Solution<'_, { Day::Day15 }, { Part::Two }> for AOC2022<{ Day::Day15 }> {
    type Input = Vec<LogLine>;
    type Output = i64;

    fn solve(&self, input: &Self::Input) -> Result<Self::Output> {
        find_frequency(
            input,
            &Point {
                x: 4000000,
                y: 4000000,
            },
        )
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
                    sensor: Point {
                        x: 2832148,
                        y: 322979
                    },
                    beacon: Point {
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
        assert_eq!(
            can_contain_brute(&parsed, &Point { x: 20, y: 20 }),
            Some(Point { x: 14, y: 11 })
        );
        assert_eq!(find_frequency(&parsed, &Point { x: 20, y: 20 })?, 56000011);
        Ok(())
    }
}
