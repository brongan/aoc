use std::{cmp::Ordering, str::FromStr};

use super::AOC2024;
use anyhow::Result;
use aoc_runner::{
    point2d::{recognize_point2d, Point2D},
    Day, ParseInput, Part, Solution,
};
use counter::Counter;
use nom::{
    bytes::complete::tag,
    character::complete::{multispace1, newline},
    combinator::{map, map_res},
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    IResult,
};

type Num = i64;
type Point = Point2D<Num>;

#[derive(Debug, Clone)]
pub struct Robot {
    p: Point,
    v: Point,
}

#[derive(Clone)]
pub struct Lobby {
    robots: Vec<Robot>,
    width: Num,
    height: Num,
}

impl Lobby {
    fn step(&mut self) {
        self.robots = self
            .robots
            .iter()
            .map(|robot| robot.step(self.width, self.height))
            .collect();
    }

    fn safety_factor(&self) -> usize {
        let counts = self
            .robots
            .iter()
            .map(|robot| find_quadrant(&robot.p, self.width, self.height))
            .flatten()
            .collect::<Counter<Quadrant>>();
        counts.values().product()
    }

    fn print(&self) -> std::fmt::Result {
        let counts: Counter<Point> = self.robots.iter().map(|r| r.p).collect();
        for y in (0..self.height).rev() {
            for x in 0..self.width {
                let c = match counts.get(&Point { x, y }) {
                    None => String::from("."),
                    Some(n) => format!("{n}"),
                };
                print!("{c}");
            }
            println!("");
        }
        Ok(())
    }
}

impl Robot {
    fn step(&self, width: i64, height: i64) -> Robot {
        let Robot { p, v } = &self;
        let mut x = (p.x + v.x) % width;
        if x < 0 {
            x = width + x;
        }
        let mut y = (p.y + v.y) % height;
        if y < 0 {
            y = height + y;
        }
        Robot {
            p: Point { x, y },
            ..*self
        }
    }
}

fn parse_p(input: &str) -> IResult<&str, Point> {
    map_res(preceded(tag("p="), recognize_point2d), Point2D::from_str)(input)
}

fn parse_v(input: &str) -> IResult<&str, Point> {
    map_res(preceded(tag("v="), recognize_point2d), Point2D::from_str)(input)
}

// p=32,46 v=96,-70
fn parse_robot(input: &str) -> IResult<&str, Robot> {
    map(separated_pair(parse_p, multispace1, parse_v), |(p, v)| {
        Robot { p, v }
    })(input)
}

impl ParseInput<'_, { Day::Day14 }> for AOC2024<{ Day::Day14 }> {
    type Parsed = Vec<Robot>;

    fn parse_input(&self, input: &'_ str) -> Result<Self::Parsed> {
        let (_, robots) = separated_list1(newline, parse_robot)(input).map_err(|e| e.to_owned())?;
        Ok(robots)
    }
}

#[derive(Eq, PartialEq, Hash, Debug)]
enum Quadrant {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

fn find_quadrant(p: &Point, width: i64, height: i64) -> Option<Quadrant> {
    let Point { x, y } = p.to_owned();
    match (x.cmp(&(width / 2)), y.cmp(&(height / 2))) {
        (_, Ordering::Equal) => None,
        (Ordering::Equal, _) => None,
        (Ordering::Less, Ordering::Less) => Some(Quadrant::BottomLeft),
        (Ordering::Less, Ordering::Greater) => Some(Quadrant::TopLeft),
        (Ordering::Greater, Ordering::Less) => Some(Quadrant::TopRight),
        (Ordering::Greater, Ordering::Greater) => Some(Quadrant::BottomRight),
    }
}

fn part_1(mut lobby: Lobby, steps: usize) -> usize {
    for _i in 0..steps {
        lobby.step();
    }
    lobby.safety_factor()
}

fn part_2(mut lobby: Lobby) -> usize {
    let mut seconds = 0;
    while !lobby
        .robots
        .iter()
        .map(|r| r.p)
        .collect::<Counter<_>>()
        .values()
        .all(|count| *count < 2)
    {
        lobby.step();
        seconds += 1;
    }
    lobby.print().unwrap();
    seconds
}

impl Solution<'_, { Day::Day14 }, { Part::One }> for AOC2024<{ Day::Day14 }> {
    type Input = Vec<Robot>;
    type Output = usize;

    fn solve(&self, input: &Self::Input) -> Result<Self::Output> {
        let lobby: Lobby = Lobby {
            robots: input.to_owned(),
            width: 101,
            height: 103,
        };
        Ok(part_1(lobby, 100))
    }
}

impl Solution<'_, { Day::Day14 }, { Part::Two }> for AOC2024<{ Day::Day14 }> {
    type Input = Vec<Robot>;
    type Output = usize;

    fn solve(&self, input: &Self::Input) -> Result<Self::Output> {
        let lobby: Lobby = Lobby {
            robots: input.to_owned(),
            width: 101,
            height: 103,
        };
        Ok(part_2(lobby))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() -> Result<()> {
        let problem = super::AOC2024::<{ Day::Day14 }>;
        let input = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";
        let lobby = Lobby {
            robots: problem.parse_input(input)?,
            width: 11,
            height: 7,
        };
        assert_eq!(part_1(lobby, 100), 12);
        Ok(())
    }
}
