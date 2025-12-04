use std::collections::VecDeque;

use super::AOC2025;
use anyhow::Result;
use aoc_runner::{Day, ParseInput, Part, Solution};

type IR = Vec<Vec<char>>;
type Num = u32;

impl ParseInput<'_, { Day::Day4 }> for AOC2025<{ Day::Day4 }> {
    type Parsed = IR;

    fn parse_input(&self, input: &'_ str) -> Result<Self::Parsed> {
        Ok(input
            .trim()
            .lines()
            .map(|line| line.chars().collect())
            .collect())
    }
}

impl Solution<'_, { Day::Day4 }, { Part::One }> for AOC2025<{ Day::Day4 }> {
    type Input = IR;
    type Output = Num;

    fn solve(&self, input: &Self::Input) -> Result<Self::Output> {
        type Point = (usize, usize);
        let size = input.len();
        let get_neighbors = |pos: Point| -> Vec<Point> {
            let mut ret: Vec<Point> = Vec::with_capacity(4);
            for (dx, dy) in [
                (0i32, 1i32),
                (1i32, 0i32),
                (0i32, -1i32),
                (-1i32, 0i32),
                (1i32, 1i32),
                (1i32, -1i32),
                (-1i32, 1i32),
                (-1i32, -1i32),
            ] {
                let x = pos.0 as i32 + dx;
                let y = pos.1 as i32 + dy;
                if x >= 0 && x < size as i32 && y >= 0 && y < size as i32 {
                    ret.push((x as usize, y as usize));
                }
            }
            ret
        };
        let mut count = 0;
        for (i, row) in input.iter().enumerate() {
            for (j, val) in row.iter().enumerate() {
                if *val != '@' {
                    continue;
                }
                if get_neighbors((i, j))
                    .iter()
                    .filter(|(i, j)| input[*i][*j] == '@')
                    .count()
                    < 4
                {
                    count += 1;
                }
            }
        }
        Ok(count)
    }
}

impl Solution<'_, { Day::Day4 }, { Part::Two }> for AOC2025<{ Day::Day4 }> {
    type Input = IR;
    type Output = usize;

    fn solve(&self, input: &Self::Input) -> Result<Self::Output> {
        let mut input = input.to_owned();
        type Point = (usize, usize);
        let size = input.len();
        let get_neighbors = |pos: Point| -> Vec<Point> {
            let mut ret: Vec<Point> = Vec::with_capacity(4);
            for (dx, dy) in [
                (0i32, 1i32),
                (1i32, 0i32),
                (0i32, -1i32),
                (-1i32, 0i32),
                (1i32, 1i32),
                (1i32, -1i32),
                (-1i32, 1i32),
                (-1i32, -1i32),
            ] {
                let x = pos.0 as i32 + dx;
                let y = pos.1 as i32 + dy;
                if x >= 0 && x < size as i32 && y >= 0 && y < size as i32 {
                    ret.push((x as usize, y as usize));
                }
            }
            ret
        };

        let mut queue = VecDeque::new();

        for i in 0..input.len() {
            for j in 0..input.len() {
                queue.push_back((i, j));
            }
        }

        while let Some((i, j)) = queue.pop_back() {
            if input[i][j] != '@' {
                continue;
            }
            if get_neighbors((i, j))
                .iter()
                .filter(|(i, j)| input[*i][*j] == '@')
                .count()
                < 4
            {
                input[i][j] = 'x';
                for neighbor in get_neighbors((i, j)) {
                    queue.push_back(neighbor);
                }
            }
        }

        Ok(input
            .iter()
            .map(|line| line.iter().filter(|val| **val == 'x'))
            .flatten()
            .count())
    }
}
