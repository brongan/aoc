use std::collections::HashMap;

use super::AOC2024;
use anyhow::Result;
use aoc_runner::{point2d::Point2D, Day, ParseInput, Part, Solution};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, newline, one_of},
    combinator::map,
    multi::{many1, separated_list1},
    sequence::separated_pair,
    IResult,
};

type Point = Point2D<usize>;

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
enum Tile {
    Robot,
    Box,
    Empty,
    Wall,
}

fn parse_tile(input: &str) -> IResult<&str, Tile> {
    let robot = |input| map(char('@'), |_| Tile::Robot)(input);
    let parse_box = |input| map(char('O'), |_| Tile::Box)(input);
    let empty = |input| map(char('.'), |_| Tile::Empty)(input);
    let wall = |input| map(char('#'), |_| Tile::Wall)(input);

    alt((robot, parse_box, empty, wall))(input)
}

#[derive(Eq, PartialEq, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Warehouse {
    tiles: HashMap<Point, Tile>,
}

impl Warehouse {
    fn push(&mut self, direction: Direction) {
        let robot = self.find_robot();
        let get_next = |point: Point| match direction {
            Direction::Up => Point {
                y: point.y - 1,
                ..point
            },
            Direction::Down => Point {
                y: point.y + 1,
                ..point
            },
            Direction::Left => Point {
                x: point.x - 1,
                ..point
            },
            Direction::Right => Point {
                x: point.x + 1,
                ..point
            },
        };

        let linear_scan = |mut point: Point| {
            while self.tiles[&point] == Tile::Box {
                point = get_next(point);
            }
            point
        };
        let next = get_next(robot);
        match self.tiles[&next] {
            Tile::Wall => (),
            Tile::Empty => {
                self.tiles.insert(robot, Tile::Empty);
                self.tiles.insert(next, Tile::Robot);
            }
            Tile::Robot => panic!("why robot?"),
            Tile::Box => {
                let scan_result = linear_scan(next);
                match self.tiles[&scan_result] {
                    Tile::Wall => (),
                    Tile::Empty => {
                        self.tiles.insert(robot, Tile::Empty);
                        self.tiles.insert(next, Tile::Robot);
                        self.tiles.insert(scan_result, Tile::Box);
                    }
                    x => panic!("Nani tf?: {x:?}"),
                }
            }
        }
    }

    fn find_robot(&self) -> Point {
        for (point, tile) in &self.tiles {
            if *tile == Tile::Robot {
                return *point;
            }
        }
        panic!("no robot??");
    }

    fn score(&self) -> usize {
        self.tiles
            .iter()
            .filter(|(_point, tile)| **tile == Tile::Box)
            .map(|(point, _tile)| point.y * 100 + point.x)
            .sum()
    }
}

pub struct IR {
    warehouse: Vec<Vec<Tile>>,
    directions: Vec<Vec<Direction>>,
}

fn parse_warehouse(input: &str) -> IResult<&str, Vec<Vec<Tile>>> {
    separated_list1(newline, many1(parse_tile))(input)
}

fn parse_moves(input: &str) -> IResult<&str, Vec<Vec<Direction>>> {
    let parse_move = |input| {
        map(one_of("><^v"), |c| match c {
            '>' => Direction::Right,
            '<' => Direction::Left,
            'v' => Direction::Down,
            '^' => Direction::Up,
            _ => panic!("no"),
        })(input)
    };
    separated_list1(newline, many1(parse_move))(input)
}

impl ParseInput<'_, { Day::Day15 }> for AOC2024<{ Day::Day15 }> {
    type Parsed = IR;

    fn parse_input(&self, input: &'_ str) -> Result<Self::Parsed> {
        let (_, ir) = map(
            separated_pair(parse_warehouse, tag("\n\n"), parse_moves),
            |(warehouse, moves)| IR {
                warehouse,
                directions: moves,
            },
        )(input)
        .map_err(|e| e.to_owned())?;
        Ok(ir)
    }
}

impl Solution<'_, { Day::Day15 }, { Part::One }> for AOC2024<{ Day::Day15 }> {
    type Input = IR;
    type Output = usize;

    fn solve(&self, input: &Self::Input) -> Result<Self::Output> {
        let mut warehouse = Warehouse {
            tiles: input
                .warehouse
                .iter()
                .enumerate()
                .map(move |(y, row)| {
                    row.iter()
                        .enumerate()
                        .map(move |(x, tile)| (Point { x, y }, *tile))
                })
                .flatten()
                .collect(),
        };
        for row in &input.directions {
            for movement in row {
                warehouse.push(*movement);
            }
        }
        Ok(warehouse.score())
    }
}

impl Solution<'_, { Day::Day15 }, { Part::Two }> for AOC2024<{ Day::Day15 }> {
    type Input = IR;
    type Output = usize;

    fn solve(&self, input: &Self::Input) -> Result<Self::Output> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() -> Result<()> {
        let input = "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########";

        let (rem, _warehouse) = parse_warehouse(&input)?;
        assert_eq!(rem, "");

        let input = "<^^>>>vv<v>>v<<";
        let (rem, _moves) = parse_moves(&input)?;
        assert_eq!(rem, "");
        Ok(())
    }
}
