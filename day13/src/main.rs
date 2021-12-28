#![feature(iter_advance_by)]
use std::fmt::Write as FmtWrite;
use std::{collections::HashSet, fs::read_to_string, str::FromStr};

#[derive(Clone, Copy, Default, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Point2D<T> {
    x: T,
    y: T,
}

impl FromStr for Point2D<usize> {
    type Err = std::string::ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split = s.split_once(',').expect("Failed to find comma");
        Ok(Point2D {
            x: split
                .0
                .parse::<usize>()
                .expect("Failed to parse coordinate"),
            y: split
                .1
                .parse::<usize>()
                .expect("Failed to parse coordinate"),
        })
    }
}

enum FoldInstruction {
    X(usize),
    Y(usize),
}

impl FromStr for FoldInstruction {
    type Err = std::string::ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut equation = s.chars();
        equation.advance_by(11).expect("equation too short");
        let equation = equation
            .as_str()
            .split_once('=')
            .expect("Failed to split equation");
        Ok(match equation.0 {
            "x" => FoldInstruction::X(
                equation
                    .1
                    .parse::<usize>()
                    .expect("failed to parse fold equation line"),
            ),
            "y" => FoldInstruction::Y(
                equation
                    .1
                    .parse::<usize>()
                    .expect("failed to parse fold equation line"),
            ),
            _ => panic!("bad equation"),
        })
    }
}

fn parse_input(input: &str) -> (Vec<Point2D<usize>>, Vec<FoldInstruction>) {
    let input = input.trim().split_once("\n\n").expect("invalid input");
    let points: Vec<Point2D<usize>> = input
        .0
        .lines()
        .map(|line| Point2D::from_str(line.trim()).expect("Failed to parse point"))
        .collect();

    let instructions = input
        .1
        .lines()
        .map(|line| FoldInstruction::from_str(line.trim()).expect("Failed to parse instruction"))
        .collect();
    (points, instructions)
}

fn fold(points: &mut Vec<Point2D<usize>>, instruction: &FoldInstruction) {
    for mut point in points {
        match instruction {
            FoldInstruction::X(x) => {
                if point.x > *x {
                    point.x = 2 * *x - point.x
                }
            }
            FoldInstruction::Y(y) => {
                if point.y > *y {
                    point.y = 2 * *y - point.y
                }
            }
        }
    }
}

fn part1(mut points: Vec<Point2D<usize>>, instructions: &[FoldInstruction]) -> usize {
    fold(
        &mut points,
        instructions.first().expect("Need at least one instruction"),
    );
    HashSet::<Point2D<usize>>::from_iter(points).len()
}

fn part2(mut points: Vec<Point2D<usize>>, instructions: &[FoldInstruction]) -> String {
    instructions
        .iter()
        .for_each(|instruction| fold(&mut points, instruction));
    let max_x = points
        .iter()
        .map(|p| p.x)
        .max()
        .expect("need a point after folding");
    let max_y = points
        .iter()
        .map(|p| p.y)
        .max()
        .expect("need a point after folding");
    let mut output: Vec<Vec<char>> = vec![vec![' '; max_y + 1]; max_x + 1];
    for point in points {
        output[point.x][point.y] = '#';
    }

    let mut ret: String = String::new();
    for row in output.iter().rev() {
        writeln!(&mut ret, "{}", row.iter().collect::<String>()).unwrap();
    }
    ret
}

fn main() {
    let input = read_to_string("input").expect("failed to open input");
    let (points, instructions) = parse_input(&input);
    println!("Part 1: {}", part1(points.clone(), &instructions));
    println!("Part 1: \n{}", part2(points, &instructions));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let input = "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5";
        let (points, instructions) = parse_input(input);
        assert_eq!(part1(points, &instructions), 17);
    }
}
